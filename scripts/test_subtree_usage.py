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


if __name__ == "__main__":
    unittest.main()
