"""Run: python3 -m unittest discover -s scripts -p test_subtree_usage.py"""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("usage", Path(__file__).with_name("subtree-usage.py"))
usage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(usage)


class UsageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)
        self.start = "2026-09-16T10:00:00Z"
        self.now = usage.date("2026-09-16T11:00:00Z")

    def log(self, ident, parent=None, events=(), folder="sessions"):
        path = self.home / folder / f"rollout-{ident}.jsonl"
        path.parent.mkdir(parents=True, exist_ok=True)
        meta = {"id": ident, "timestamp": self.start}
        if parent:
            meta["source"] = {"subagent": {"thread_spawn": {"parent_thread_id": parent}}}
        records = [{"type": "session_meta", "payload": meta}]
        records += list(events)
        path.write_text("".join(json.dumps(e) + "\n" for e in records))
        return path, meta

    def context(self, model):
        return {"type": "turn_context", "payload": {"model": model}}

    def count(self, at, total, last=None):
        def tokens(values):
            return dict(zip(usage.KEYS, values))
        return {"type": "event_msg", "timestamp": f"2026-09-16T{at}Z", "payload": {
            "type": "token_count", "info": {"total_token_usage": tokens(total),
            "last_token_usage": tokens(last or total)}}}

    def test_price_cache_reasoning_and_long_context(self):
        self.assertAlmostEqual(usage.price("gpt-6-astra", (1000, 800, 100, 0)), .0078)
        self.assertAlmostEqual(usage.price("gpt-6-astra", (300000, 200000, 1000, 0)), 2.475)
        self.assertAlmostEqual(usage.price("gpt-6-astra", (1000, 800, 100, 100)), .00805)
        self.assertIsNone(usage.price("unknown", (1, 0, 1, 0)))

    def test_lineage_and_archive_dedup(self):
        self.log("root")
        self.log("child", "root")
        self.log("grandchild", "child")
        self.log("other")
        self.log("root", folder="archived_sessions")
        sessions = usage.inventory(self.home)
        self.assertEqual(len(sessions), 4)
        self.assertEqual(usage.descendants(sessions, "root"), {"root", "child", "grandchild"})

    def test_dedup_switches_and_windows(self):
        first = self.count("10:10:00", (1000, 800, 100, 0))
        events = [self.context("gpt-6-astra"), first, first,
                  self.context("gpt-5.6-luna"),
                  self.count("10:40:00", (2000, 1600, 200, 0), (1000, 800, 100, 0)),
                  self.count("10:58:00", (3000, 2400, 300, 0), (1000, 800, 100, 0))]
        path, meta = self.log("root", events=events)
        warnings = set()
        parsed = usage.collect(path, meta, self.now, warnings)
        self.assertEqual(len(parsed), 3)
        self.assertEqual([e[1] for e in parsed], ["gpt-6-astra", "gpt-5.6-luna", "gpt-5.6-luna"])
        self.assertFalse(warnings)
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            code = usage.report(usage.inventory(self.home), "root", self.now)
        self.assertEqual(code, 0)
        lines = out.getvalue().splitlines()
        self.assertEqual(next(x for x in lines if x.startswith("last 30m")).split()[2], "400")
        self.assertEqual(next(x for x in lines if x.startswith("last 5m")).split()[2], "200")

    def test_unknown_price_is_not_silent(self):
        self.log("root", events=[self.context("new-model"), self.count("10:58:00", (10, 0, 1, 0))])
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            code = usage.report(usage.inventory(self.home), "root", self.now)
        self.assertEqual(code, 2)
        self.assertIn("Unpriced model: new-model", out.getvalue())
        self.assertIn("0.00+?", out.getvalue())

    def test_mismatch_and_partial_log_warn(self):
        path, meta = self.log("root", events=[self.context("gpt-6-astra"),
            self.count("10:10:00", (1000, 0, 100, 0)),
            self.count("10:40:00", (3000, 0, 300, 0), (1000, 0, 100, 0))])
        with path.open("a") as f:
            f.write('{"type":"token_count",')
        warnings = set()
        usage.collect(path, meta, self.now, warnings)
        self.assertEqual(len(warnings), 2)

    def test_inherited_past_and_future_excluded(self):
        path, meta = self.log("root", events=[self.context("gpt-6-astra"),
            self.count("09:00:00", (100, 0, 10, 0)),
            self.count("10:58:00", (200, 0, 20, 0), (100, 0, 10, 0)),
            self.count("12:00:00", (300, 0, 30, 0), (100, 0, 10, 0))])
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertFalse(warnings)

    def native(self, owner, response, at="10:40:00", counts=(100, 90, 10, 0)):
        return {"type": "token_usage_record", "timestamp": f"2026-09-16T{at}Z",
                "payload": {"thread_id": owner, "response_id": response,
                            "usage": dict(zip(usage.KEYS, counts))}}

    def settings(self, owner):
        return {"type": "event_msg", "payload": {"type": "thread_settings_applied",
                "thread_id": owner, "thread_settings": {"model": "gpt-6-astra"}}}

    def test_ordinary_fork_is_independent_of_worker_tree(self):
        self.log("root")
        path, _ = self.log("fork")
        meta = {"id": "fork", "timestamp": self.start, "forked_from_id": "root", "source": "cli"}
        path.write_text(json.dumps({"type": "session_meta", "payload": meta}) + "\n")
        self.log("worker", "root")
        sessions = usage.inventory(self.home)
        self.assertEqual(usage.descendants(sessions, "root"), {"root", "worker"})
        self.assertIsNone(usage.parent(meta))

    def test_copied_native_records_are_owned_and_response_deduplicated(self):
        own = self.native("fork", "new")
        path, meta = self.log("fork", events=[self.settings("source"),
            self.native("source", "old"), self.settings("fork"),
            self.count("10:30:00", (100, 90, 10, 0)), own, own,
            self.count("10:40:00", (200, 180, 20, 0), (100, 90, 10, 0))])
        meta["forked_from_id"] = "source"
        warnings = set()
        events = usage.collect(path, meta, self.now, warnings)
        self.assertEqual(len(events), 1)
        self.assertEqual(events[0][2:6], (10, 90, 10, 0))
        self.assertFalse(warnings)

    def test_copied_legacy_prefix_uses_owned_settings_boundary(self):
        inherited = self.count("10:10:00", (100, 90, 10, 0))
        path, meta = self.log("fork", events=[self.context("gpt-6-astra"), inherited,
            self.settings("fork"), inherited,
            self.count("10:40:00", (200, 180, 20, 0), (100, 90, 10, 0))])
        meta["forked_from_id"] = "source"
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertFalse(warnings)

    def test_ambiguous_legacy_fork_is_excluded_with_reason(self):
        path, meta = self.log("fork", events=[self.count("10:10:00", (100, 90, 10, 0))])
        meta["forked_from_id"] = "source"
        warnings = set()
        self.assertEqual(usage.collect(path, meta, self.now, warnings), [])
        self.assertTrue(any("ownership boundary unavailable" in w for w in warnings))

    def test_referenced_fork_ignores_seed_counter_and_counts_new_response(self):
        path, meta = self.log("fork", events=[self.settings("fork"),
            self.count("10:10:00", (100, 90, 10, 0)), self.native("fork", "new")])
        meta.update(forked_from_id="source", history_base={"thread_id": "source", "ordinal": 50})
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertFalse(warnings)

    def test_required_fields_never_default_to_zero(self):
        for field in usage.KEYS[:3]:
            for side in ("total_token_usage", "last_token_usage"):
                count = self.count("10:10:00", (100, 90, 10, 0))
                del count["payload"]["info"][side][field]
                path, meta = self.log("root", events=[count])
                warnings = set()
                self.assertEqual(usage.collect(path, meta, self.now, warnings), [])
                self.assertTrue(any("missing accounting field" in w for w in warnings))

    def test_malformed_numbers_warn_instead_of_crashing(self):
        for value in (None, True, 1.5, "not-a-number", -1):
            record = self.native("root", "bad")
            record["payload"]["usage"]["input_tokens"] = value
            path, meta = self.log("root", events=[record])
            warnings = set()
            self.assertEqual(usage.collect(path, meta, self.now, warnings), [])
            self.assertTrue(warnings)

    def test_invalid_owned_record_does_not_recharge_cumulative_mirror(self):
        bad = self.native("root", "same")
        del bad["payload"]["usage"]["input_tokens"]
        path, meta = self.log("root", events=[self.settings("root"), bad,
            self.count("10:40:00", (100, 90, 10, 0)), self.native("root", "same")])
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertTrue(warnings)

    def test_unknown_optional_cache_writes_remain_unknown(self):
        record = self.native("root", "new")
        del record["payload"]["usage"]["cache_write_input_tokens"]
        path, meta = self.log("root", events=[self.context("gpt-6-astra"), record])
        warnings = set()
        events = usage.collect(path, meta, self.now, warnings)
        self.assertIsNone(events[0][2])
        self.assertIsNone(events[0][5])
        self.assertIsNone(events[0][6])
        self.assertTrue(any("cache-write count absent" in w for w in warnings))

    def test_conflicting_duplicate_response_is_visible(self):
        path, meta = self.log("root", events=[self.context("gpt-6-astra"),
            self.native("root", "same"), self.native("root", "same", counts=(200, 90, 10, 0))])
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertTrue(warnings)

    def test_resume_replays_response_without_recharging_it(self):
        first = self.native("root", "first")
        path, meta = self.log("root", events=[self.settings("root"), first,
            self.count("10:40:00", (100, 90, 10, 0)), self.settings("root"), first,
            self.native("root", "second", "10:50:00")])
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 2)
        self.assertFalse(warnings)

    def test_legacy_to_native_transition_preserves_earlier_usage(self):
        path, meta = self.log("root", events=[self.settings("root"),
            self.count("10:10:00", (100, 90, 10, 0)),
            self.native("root", "new"),
            self.count("10:40:00", (200, 180, 20, 0), (100, 90, 10, 0))])
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 2)
        self.assertFalse(warnings)

    def test_legacy_subagent_ordinal_boundary_excludes_copied_prefix(self):
        inherited = self.count("10:10:00", (100, 90, 10, 0))
        inherited["ordinal"] = 5
        owned = self.count("10:40:00", (200, 180, 20, 0), (100, 90, 10, 0))
        owned["ordinal"] = 10
        path, meta = self.log("child", "source", events=[self.context("gpt-6-astra"), inherited, owned])
        meta.update(forked_from_id="source", subagent_history_start_ordinal=10)
        warnings = set()
        self.assertEqual(len(usage.collect(path, meta, self.now, warnings)), 1)
        self.assertFalse(warnings)


if __name__ == "__main__":
    unittest.main()
