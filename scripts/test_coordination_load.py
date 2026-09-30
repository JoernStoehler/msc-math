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
                                             "arguments": "private", "_timestamp": 1}]}).encode()

        with patch.object(load.urllib.request, "urlopen", return_value=Response()) as opened:
            rows, errors = load.query("http://127.0.0.1:5080", "private-auth", {"root"},
                                     self.now-timedelta(minutes=15), self.now)
        request = opened.call_args.args[0]
        sql = json.loads(request.data)["query"]["sql"]
        for forbidden in ("prompt", "arguments", "content", "output,", "body"):
            self.assertNotIn(forbidden, sql)
        self.assertEqual(rows, [{"conversation_id": "root", "_timestamp": 1}])
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

    def test_fallback_excludes_inheritance_and_deduplicates(self):
        def count(when, total):
            return {"timestamp": when, "type": "event_msg", "payload": {"type": "token_count", "info": {
                "total_token_usage": {"input_tokens": total, "cached_input_tokens": total-10,
                                      "output_tokens": 10},
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
        signals = load.review_signals("root", threads, rows, {"running_assignments": []}, self.now)
        self.assertEqual({s["signal"] for s in signals}, {
            "coordinator_context_larger_than_recent_workers", "coordinator_non_cached_input_spike"})
        self.assertTrue(all("evidence" in s and "limitation" in s for s in signals))
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


if __name__ == "__main__":
    unittest.main()
