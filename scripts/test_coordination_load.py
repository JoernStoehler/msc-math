"""Run: python3 -m unittest discover -s scripts -p test_coordination_load.py"""
from datetime import timedelta
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("load", Path(__file__).with_name("coordination-load.py"))
load = importlib.util.module_from_spec(spec)
spec.loader.exec_module(load)


class LoadTests(unittest.TestCase):
    def setUp(self):
        self.now = load.date("2026-09-30T17:00:00Z")

    def test_header_decoding_and_missing_auth(self):
        self.assertEqual(load.auth_header({"OTEL_EXPORTER_OTLP_LOGS_HEADERS":
                                          "Authorization=Basic%20dummy%3D,other=value"}), "Basic dummy=")
        with self.assertRaises(ValueError):
            load.auth_header({})

    def test_server_and_client_projection_exclude_content(self):
        class Response:
            def __enter__(self):
                return self

            def __exit__(self, *args):
                pass

            def read(self):
                return json.dumps({"hits": [{"conversation_id": "root", "prompt": "private",
                                             "arguments": "private", "_timestamp": 1,
                                             "duration_ms": "15", "reasoning_token_count": 2,
                                             "tool_token_count": "110"}]}).encode()

        with patch.object(load.urllib.request, "urlopen", return_value=Response()) as opened:
            rows, errors = load.query("http://127.0.0.1:5080", "private-auth", {"root"},
                                     self.now-timedelta(minutes=15), self.now)
        request = opened.call_args.args[0]
        sql = json.loads(request.data)["query"]["sql"]
        for forbidden in ("prompt", "arguments", "content", "output,", "body"):
            self.assertNotIn(forbidden, sql)
        for field in ("duration_ms", "reasoning_token_count", "tool_token_count"):
            self.assertIn(field, sql)
        self.assertEqual(rows, [{"conversation_id": "root", "_timestamp": 1,
                                "duration_ms": "15", "reasoning_token_count": 2,
                                "tool_token_count": "110"}])
        self.assertEqual(errors, [])

    def test_completed_only_counters_and_missing_not_zero(self):
        row = {"event_name": "codex.sse_event", "event_kind": "response.completed",
               "input_token_count": "100", "cached_token_count": 90, "output_token_count": "10"}
        totals = load.token_totals([row, {**row, "event_kind": "response.delta"}])
        self.assertEqual(totals["observed_completions"], 1)
        self.assertEqual(totals["input_token_count"], 100)
        self.assertEqual(totals["output_token_count"], 10)
        self.assertIsNone(totals["cache_write_token_count"])
        self.assertIsNone(load.token_totals([])["input_token_count"])
        self.assertEqual(load.token_totals([{**row, "cached_token_count": 101}])[
            "invalid_or_missing_token_records"], 1)

    def test_raw_poll_and_parsed_usage_count_once_without_false_error(self):
        raw = {"event_name": "codex.sse_event", "event_kind": "response.completed",
               "duration_ms": "15"}
        usage = {"event_name": "codex.sse_event", "event_kind": "response.completed",
                 "input_token_count": "100", "cached_token_count": 90,
                 "output_token_count": "10", "cache_write_token_count": 0}
        totals = load.token_totals([raw, usage])
        self.assertEqual(totals["observed_completions"], 1)
        self.assertEqual(totals["raw_transport_completion_events"], 1)
        self.assertEqual(totals["valid_token_records"], 1)
        self.assertEqual(totals["invalid_or_missing_token_records"], 0)
        self.assertEqual(totals["input_token_count"], 100)
        self.assertEqual(totals["cache_write_token_count"], 0)

    def test_raw_only_completion_keeps_usage_unknown(self):
        raw = {"event_name": "codex.sse_event", "event_kind": "response.completed",
               "duration_ms": "15", "_timestamp": int(self.now.timestamp()*1e6)}
        thread = load.thread_snapshot("root", [raw], self.now)
        totals = thread["tokens"]
        self.assertEqual(totals["observed_completions"], 0)
        self.assertEqual(totals["raw_transport_completion_events"], 1)
        self.assertEqual(totals["valid_token_records"], 0)
        self.assertEqual(totals["invalid_or_missing_token_records"], 0)
        for field in ("input_token_count", "cached_token_count", "output_token_count",
                      "cache_write_token_count"):
            self.assertIsNone(totals[field])
        self.assertIsNone(thread["latest_completed_at"])

    def test_duration_does_not_hide_partial_or_malformed_usage(self):
        base = {"event_name": "codex.sse_event", "event_kind": "response.completed",
                "duration_ms": "15"}
        for field in load.USAGE_COUNTER_FIELDS:
            for value in (None, "bad", 2):
                with self.subTest(field=field, value=value):
                    totals = load.token_totals([{**base, field: value}])
                    self.assertEqual(totals["raw_transport_completion_events"], 0)
                    self.assertEqual(totals["observed_completions"], 1)
                    self.assertEqual(totals["invalid_or_missing_token_records"], 1)
        malformed = {**base, "input_token_count": "bad", "cached_token_count": 90,
                     "output_token_count": 10}
        self.assertEqual(load.token_totals([malformed])["invalid_or_missing_token_records"], 1)

    def test_unmarked_counterless_completion_remains_invalid(self):
        totals = load.token_totals([{"event_name": "codex.sse_event",
                                     "event_kind": "response.completed"}])
        self.assertEqual(totals["raw_transport_completion_events"], 0)
        self.assertEqual(totals["observed_completions"], 1)
        self.assertEqual(totals["invalid_or_missing_token_records"], 1)
        self.assertIsNone(totals["input_token_count"])

    def test_fallback_excludes_inheritance_and_deduplicates(self):
        def count(when, total):
            return {"timestamp": when, "type": "event_msg", "payload": {"type": "token_count", "info": {
                "total_token_usage": {"input_tokens": total, "cached_input_tokens": total*9//10,
                                      "output_tokens": total//10},
                "last_token_usage": {"input_tokens": 100, "cached_input_tokens": 90,
                                     "output_tokens": 10}}}}
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)/"log.jsonl"
            current = count("2026-09-30T16:59:00Z", 200)
            path.write_text("\n".join(json.dumps(r) for r in [
                count("2026-09-30T16:40:00Z", 100), current, current,
                {"type": "response_item", "payload": {"prompt": "token_count"}}]))
            result, errors = load.local_tokens(path, {"timestamp": "2026-09-30T16:50:00Z"},
                                              self.now-timedelta(minutes=30), self.now)
        self.assertEqual(result["observed_completions"], 1)
        self.assertEqual(result["input_token_count"], 100)
        self.assertIsNone(result["cache_write_token_count"])
        self.assertFalse(errors)

    def test_silence_and_nested_failures_are_event_counts(self):
        stamp = int(self.now.timestamp()*1e6)
        rows = [{"_timestamp": stamp-10000000, "event_name": "codex.tool_result",
                 "tool_name": tool, "success": "false"} for tool in ("exec", "apply_patch")]
        result = load.thread_snapshot("root", rows, self.now)
        self.assertEqual(result["observed_unsuccessful_events"], 2)
        self.assertEqual(result["native_event_silence_seconds"], 10)
        self.assertEqual(result["largest_observed_event_gap_seconds"], 0)
        self.assertIsNone(load.thread_snapshot("root", [], self.now)["native_event_silence_seconds"])

    def test_coordination_is_metadata_and_semantic_load_unknown(self):
        result = load.coordination({"tasks": [{"id": "t", "status": "running", "owner": None,
                                               "outcome": "irrelevant prose"}],
                                    "decisions": [{"id": "gate", "status": "pending"},
                                                  {"id": "closed", "status": "resolved"}]})
        self.assertEqual(result["unresolved_decisions"], [{"id": "gate", "status": "pending"}])
        self.assertIsNone(result["semantic_queue_or_pending_review_count"])
        self.assertNotIn("outcome", result["running_assignments"][0])

    def test_portfolio_includes_independent_and_released_threads(self):
        a = "01a0f33e-8cf0-77b0-9d80-7ee6fad1d6fc"
        b = "01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a"
        ids = load.portfolio_ids({"tasks": [{"owner": a}, {"owner": None, "related_threads": [b]},
                                            {"owner": "/root/worker"}]})
        self.assertEqual(ids, {a, b})

    def test_relative_review_signals_have_evidence_and_limitations(self):
        def response(ident, inp, cached, seconds):
            return {"conversation_id": ident, "_timestamp": int(self.now.timestamp()*1e6)-seconds*1000000,
                    "event_name": "codex.sse_event", "event_kind": "response.completed",
                    "input_token_count": inp, "cached_token_count": cached, "output_token_count": 1}
        rows = [response("root", 1000, 990, s) for s in (100, 90, 80)]
        rows += [response("root", 1000, 900, 10), response("a", 100, 90, 20), response("b", 200, 190, 20)]
        threads = [load.thread_snapshot(i, [r for r in rows if r["conversation_id"] == i], self.now)
                   for i in ("root", "a", "b")]
        for thread in threads:
            thread["scope_role"] = "root" if thread["thread_id"] == "root" else "root_descendant"
        signals = load.review_signals("root", threads, rows, {"running_assignments": []}, self.now)
        self.assertEqual({s["signal"] for s in signals}, {
            "coordinator_context_larger_than_recent_workers", "coordinator_non_cached_input_spike"})
        self.assertTrue(all("evidence" in s and "limitation" in s for s in signals))
        for thread in threads[1:]:
            thread["scope_role"] = "other_scoped_thread"
        independent_signals = load.review_signals("root", threads, rows, {}, self.now)
        self.assertEqual({s["signal"] for s in independent_signals}, {"coordinator_non_cached_input_spike"})
        no_root = load.review_signals("absent", threads, rows, {"status": "unknown"}, self.now)
        self.assertEqual(no_root, [])

    def test_release_after_completion_does_not_infer_ownership_problem(self):
        row = {"conversation_id": "root", "_timestamp": int(self.now.timestamp()*1e6)-60000000,
               "event_name": "codex.sse_event", "event_kind": "response.completed",
               "input_token_count": 100, "cached_token_count": 90, "output_token_count": 1}
        thread = load.thread_snapshot("root", [row], self.now)
        state = {"running_assignments": [], "updated_at": self.now.isoformat()}
        self.assertEqual(load.review_signals("root", [thread], [row], state, self.now), [])

    def test_latest_tool_event_cannot_make_stale_completion_recent(self):
        stamp = int(self.now.timestamp()*1e6)
        def response(ident, inp, seconds):
            return {"conversation_id": ident, "_timestamp": stamp-seconds*1000000,
                    "event_name": "codex.sse_event", "event_kind": "response.completed",
                    "input_token_count": inp, "cached_token_count": inp-10, "output_token_count": 1}
        def signals(rows):
            threads = [load.thread_snapshot(i, [r for r in rows if r["conversation_id"] == i], self.now)
                       for i in ("root", "a", "b")]
            for thread in threads:
                thread["scope_role"] = "root" if thread["thread_id"] == "root" else "root_descendant"
            return load.review_signals("root", threads, rows, {}, self.now)
        fresh = [response("root", 1000, 10), response("a", 100, 10), response("b", 100, 10)]
        self.assertEqual(len(signals(fresh)), 1)
        for stale_thread in ("root", "a"):
            rows = [dict(r, _timestamp=stamp-300000000) if r["conversation_id"] == stale_thread else r
                    for r in fresh]
            rows += [{"conversation_id": stale_thread, "_timestamp": stamp-1000000,
                      "event_name": "codex.tool_result", "success": "true", "tool_name": "exec"}]
            self.assertEqual(signals(rows), [])

    def test_fallback_missing_essential_counters_are_unknown(self):
        counts = {"input_tokens": 100, "cached_input_tokens": 90, "output_tokens": 10}
        for field in counts:
            for side in ("total_token_usage", "last_token_usage"):
                with self.subTest(field=field, side=side), tempfile.TemporaryDirectory() as temp:
                    info = {"total_token_usage": counts.copy(), "last_token_usage": counts.copy()}
                    del info[side][field]
                    path = Path(temp)/"log.jsonl"
                    path.write_text(json.dumps({"timestamp": "2026-09-30T16:59:00Z", "type": "event_msg",
                                                "payload": {"type": "token_count", "info": info}}))
                    result, warnings = load.local_tokens(path, {"timestamp": "2026-09-30T16:50:00Z"},
                                                         self.now-timedelta(minutes=15), self.now)
                    self.assertEqual(result["valid_token_records"], 0)
                    self.assertIsNone(result["input_token_count"])
                    self.assertEqual(result["invalid_or_missing_token_records"], 1)
                    self.assertTrue(warnings)

    def test_brief_exposes_separate_fallback_during_native_outage(self):
        ident = "01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a"
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)/"log.jsonl"
            counts = {"input_tokens": 100, "cached_input_tokens": 90, "output_tokens": 10}
            path.write_text(json.dumps({"timestamp": "2026-09-30T16:59:00Z", "type": "event_msg", "payload": {
                "type": "token_count", "info": {"total_token_usage": counts, "last_token_usage": counts}}}))
            state = Path(temp)/"state.json"
            state.write_text(json.dumps({"tasks": [{"id": "t", "status": "running", "owner": ident}]}))
            sessions = {ident: (path, {"timestamp": "2026-09-30T16:50:00Z"})}
            out = io.StringIO()
            with patch.object(load, "inventory", return_value=(sessions, lambda s, r: {r})), \
                    patch.object(load, "auth_header", return_value="dummy"), \
                    patch.object(load, "query", side_effect=OSError("private diagnostic")), \
                    patch.object(load.sys, "argv", ["load", ident, "--state", str(state), "--brief",
                                                   "--now", self.now.isoformat()]), contextlib.redirect_stdout(out):
                status = load.main()
            report = json.loads(out.getvalue())
        self.assertEqual(status, 2)
        self.assertEqual(report["native_status"], "unavailable")
        self.assertIsNone(report["root_native"]["input_token_count"])
        self.assertEqual(report["root_fallback"]["input_token_count"], 100)
        self.assertEqual(report["root_fallback"]["threads_checked"], 1)
        self.assertIsNone(report["root_fallback"]["cache_write_token_count"])
        self.assertEqual(report["workers_fallback"]["threads_checked"], 0)
        self.assertNotIn("private diagnostic", out.getvalue())

    def test_brief_keeps_independent_session_tree_out_of_workers(self):
        root, worker, other, other_worker = [f"00000000-0000-0000-0000-{i:012d}" for i in range(1, 5)]
        sessions = {ident: (Path("unused"), {}) for ident in (root, worker, other, other_worker)}
        trees = {root: {root, worker}, other: {other, other_worker}}
        rows = [{"conversation_id": ident, "_timestamp": int(self.now.timestamp()*1e6),
                 "event_name": "codex.sse_event", "event_kind": "response.completed",
                 "input_token_count": amount, "cached_token_count": 0, "output_token_count": 1}
                for ident, amount in zip((root, worker, other, other_worker), (100, 200, 300, 400))]
        rows.append({"conversation_id": root, "_timestamp": int(self.now.timestamp()*1e6),
                     "event_name": "codex.sse_event", "event_kind": "response.completed",
                     "duration_ms": "15"})
        with tempfile.TemporaryDirectory() as temp:
            state = Path(temp)/"state.json"
            state.write_text(json.dumps({"tasks": [{"id": "other", "status": "running", "owner": other}]}))
            out = io.StringIO()
            with patch.object(load, "inventory", return_value=(sessions, lambda s, r: set(trees.get(r, {r})))), \
                    patch.object(load, "auth_header", return_value="dummy"), \
                    patch.object(load, "query", return_value=(rows, [])), \
                    patch.object(load.sys, "argv", ["load", root, "--state", str(state), "--brief",
                                                   "--now", self.now.isoformat()]), contextlib.redirect_stdout(out):
                status = load.main()
            report = json.loads(out.getvalue())
        self.assertEqual(status, 0)
        self.assertEqual(report["root_native"]["input_token_count"], 100)
        self.assertEqual(report["root_native"]["completed_responses"], 1)
        self.assertEqual(report["root_native"]["raw_transport_completion_events"], 1)
        self.assertEqual(report["workers_native"]["raw_transport_completion_events"], 0)
        self.assertEqual(report["errors"], [])
        self.assertEqual(report["workers_native"]["input_token_count"], 200)
        self.assertEqual(report["other_scoped_threads_native"]["input_token_count"], 700)
        self.assertEqual(report["scope_sources"]["root_locally_discovered_thread_ids"], sorted([root, worker]))

    def test_row_limit_and_partial_results_are_explicit(self):
        class Response:
            def __enter__(self):
                return self

            def __exit__(self, *args):
                pass

            def read(self):
                return json.dumps({"hits": [{"_timestamp": 1}]*3, "is_partial": True}).encode()

        with patch.object(load, "LIMIT", 2), patch.object(load.urllib.request, "urlopen", return_value=Response()):
            rows, errors = load.query("http://127.0.0.1:5080", "dummy", {"root"}, self.now, self.now)
        self.assertEqual(len(rows), 2)
        self.assertEqual(len(errors), 2)

    def test_fallback_uses_owned_response_ids_across_copied_fork(self):
        counts = {"input_tokens": 100, "cached_input_tokens": 90,
                  "output_tokens": 10, "cache_write_input_tokens": 0}
        def native(owner, response):
            return {"type": "token_usage_record", "timestamp": "2026-09-30T16:59:00Z",
                    "payload": {"thread_id": owner, "response_id": response, "usage": counts}}
        own = native("child", "new")
        inherited_mirror = {"type": "event_msg", "timestamp": "2026-09-30T16:58:00Z",
                            "payload": {"type": "token_count", "info": {
                                "total_token_usage": counts, "last_token_usage": counts}}}
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)/"fork.jsonl"
            path.write_text("\n".join(json.dumps(r) for r in [
                native("parent", "old"), inherited_mirror, own, own]))
            result, errors = load.local_tokens(path, {"id": "child", "forked_from_id": "parent",
                "timestamp": "2026-09-30T16:50:00Z"}, self.now-timedelta(minutes=15), self.now)
        self.assertEqual(result["observed_completions"], 1)
        self.assertEqual(result["input_token_count"], 100)
        self.assertFalse(errors)

    def test_inventory_preserves_fork_ownership_metadata_without_worker_edge(self):
        with tempfile.TemporaryDirectory() as temp:
            home = Path(temp)
            folder = home/"sessions"
            folder.mkdir()
            for ident, extra in (("root", {}), ("fork", {"forked_from_id": "root",
                    "history_base": {"thread_id": "root", "ordinal": 5}})):
                (folder/f"rollout-{ident}.jsonl").write_text(json.dumps({"type": "session_meta",
                    "payload": {"id": ident, "timestamp": "2026-09-30T16:50:00Z", **extra}}))
            sessions, descendants = load.inventory(home)
        self.assertEqual(descendants(sessions, "root"), {"root"})
        self.assertEqual(sessions["fork"][1]["forked_from_id"], "root")
        self.assertIn("history_base", sessions["fork"][1])


if __name__ == "__main__":
    unittest.main()
