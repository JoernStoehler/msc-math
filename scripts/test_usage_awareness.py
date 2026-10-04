"""Accounting and delivery behavior without model calls or live configuration."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from datetime import timedelta

spec = importlib.util.spec_from_file_location("awareness", Path(__file__).with_name("usage-awareness.py"))
awareness = importlib.util.module_from_spec(spec)
spec.loader.exec_module(awareness)


class AwarenessTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)/"home"
        self.state = Path(self.temp.name)/"state"
        self.now = awareness.USAGE.date("2026-10-03T21:00:00Z")
        self.root, self.child, self.fork = [f"00000000-0000-0000-0000-{i:012d}" for i in (1, 2, 3)]
        self.log(self.root, 100, 90, 10)
        self.log(self.child, 200, 100, 30, parent=self.root)
        self.log(self.fork, 1000, 0, 100, fork=self.root)
        self.event = {"hook_event_name": "PostToolUse", "cwd": str(awareness.ROOT),
                      "session_id": self.root, "tool_input": {"secret": "private-content"},
                      "tool_response": {"secret": "private-content"}}

    def log(self, ident, inp, cached, out, parent=None, fork=None):
        meta = {"id": ident, "timestamp": "2026-10-03T20:00:00Z"}
        if parent:
            meta["parent_thread_id"] = parent
        if fork:
            meta["forked_from_id"] = fork
        row = {"type": "token_usage_record", "timestamp": "2026-10-03T20:59:00Z",
               "payload": {"thread_id": ident, "response_id": f"response-{ident}",
                           "usage": {"input_tokens": inp, "cached_input_tokens": cached,
                                     "output_tokens": out, "cache_write_input_tokens": 0}}}
        path = self.home/"sessions"/f"rollout-{ident}.jsonl"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"type": "session_meta", "payload": meta})+"\n"+json.dumps(row)+"\n")
        return path

    def test_snapshot_is_count_up_and_forks_are_independent(self):
        report = awareness.snapshot(self.home, self.child, self.now)
        self.assertEqual(report["root"], self.root)
        self.assertEqual(report["tree_usage_units"], 150)
        self.assertEqual(report["own_usage_units"], 130)
        self.assertEqual(len(report["threads"]), 2)
        self.assertEqual(report["status"], "recorded")
        fork = awareness.snapshot(self.home, self.fork, self.now)
        self.assertEqual(fork["root"], self.fork)
        self.assertEqual(fork["tree_usage_units"], 1100)

    def test_first_report_delivers_context_and_throttles_without_scanning(self):
        result = awareness.hook(self.home, self.event, self.state, 300, self.now)
        self.assertEqual(result["hookSpecificOutput"]["hookEventName"], "PostToolUse")
        text = result["hookSpecificOutput"]["additionalContext"]
        self.assertIn("shared tree 150", text)
        self.assertIn("noncached input + output", text)
        with patch.object(awareness, "snapshot", side_effect=AssertionError("unnecessary scan")):
            self.assertIsNone(awareness.hook(self.home, self.event, self.state, 300,
                                            self.now+timedelta(seconds=299)))
        saved = (self.state/f"{self.root}.json").read_text()
        self.assertNotIn("private-content", saved)
        self.assertNotIn("tool_input", saved)
        self.assertNotIn("private-content", text)

    def test_later_tool_result_reports_delta(self):
        awareness.hook(self.home, self.event, self.state, 300, self.now)
        self.log(self.child, 300, 100, 40, parent=self.root)
        result = awareness.hook(self.home, self.event, self.state, 300,
                                self.now+timedelta(seconds=300))
        self.assertIn("shared tree 260", result["hookSpecificOutput"]["additionalContext"])
        self.assertIn("+110 since", result["hookSpecificOutput"]["additionalContext"])

    def test_unchanged_report_is_silent_and_does_not_reset_usage(self):
        awareness.hook(self.home, self.event, self.state, 300, self.now)
        self.assertIsNone(awareness.hook(self.home, self.event, self.state, 300,
                                        self.now+timedelta(seconds=300)))
        self.assertEqual(json.loads((self.state/f"{self.root}.json").read_text())["tree_usage_units"], 150)

    def test_worker_receives_own_and_shared_usage(self):
        result = awareness.hook(self.home, {**self.event, "agent_id": self.child}, self.state, 300, self.now)
        self.assertIn("your thread 130", result["hookSpecificOutput"]["additionalContext"])
        self.assertTrue((self.state/f"{self.child}.json").exists())

    def test_different_checkout_is_not_selected(self):
        with patch.object(awareness, "snapshot", side_effect=AssertionError("wrong scope")):
            self.assertIsNone(awareness.hook(self.home, {**self.event, "cwd": "/tmp"}, self.state, 300, self.now))
        self.assertFalse(self.state.exists())

    def test_missing_ancestry_is_named_and_repeated_errors_are_suppressed(self):
        missing = "00000000-0000-0000-0000-000000000099"
        event = {**self.event, "session_id": missing}
        result = awareness.hook(self.home, event, self.state, 300, self.now)
        self.assertIn(missing+": thread metadata absent", result["hookSpecificOutput"]["additionalContext"])
        self.assertIsNone(awareness.hook(self.home, event, self.state, 300,
                                        self.now+timedelta(seconds=300)))

    def test_accounting_error_labels_subtotal_and_valid_rows_survive(self):
        path = self.home/"sessions"/f"rollout-{self.child}.jsonl"
        with path.open("a") as f:
            f.write(json.dumps({"type": "token_usage_record", "timestamp": "2026-10-03T20:59:00Z",
                "payload": {"thread_id": self.child, "response_id": "bad", "usage": {}}})+"\n")
        report = awareness.snapshot(self.home, self.root, self.now)
        self.assertEqual(report["status"], "partial")
        self.assertEqual(report["tree_usage_units"], 150)
        self.assertIn("observed subtotal", awareness.message(report))
        self.assertIn(self.child, awareness.message(report))

    def test_counter_decrease_invalidates_delta(self):
        report = awareness.snapshot(self.home, self.root, self.now)
        self.assertIn("prior checkpoint no longer comparable", awareness.message(report,
                       {"root": self.root, "tree_usage_units": 200}))

    def test_partial_checkpoint_cannot_create_a_complete_delta(self):
        report = awareness.snapshot(self.home, self.root, self.now)
        self.assertIn("change unavailable across accounting errors", awareness.message(report,
                       {"root": self.root, "tree_usage_units": 100, "errors": ["missing record"]}))

    def test_invalid_checkpoint_is_replaced_without_exposing_its_body(self):
        self.state.mkdir()
        (self.state/f"{self.root}.json").write_text('["private-stale-data"]')
        output = awareness.hook(self.home, self.event, self.state, 300, self.now)
        self.assertIn("shared tree 150", output["hookSpecificOutput"]["additionalContext"])
        self.assertNotIn("private-stale-data", (self.state/f"{self.root}.json").read_text())


if __name__ == "__main__":
    unittest.main()
