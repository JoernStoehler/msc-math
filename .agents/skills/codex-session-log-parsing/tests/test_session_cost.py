import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "scripts" / "session_cost.py"
SPEC = importlib.util.spec_from_file_location("session_cost", SCRIPT)
session_cost = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = session_cost
SPEC.loader.exec_module(session_cost)


def row(timestamp, payload_type, **payload):
    return {"timestamp": timestamp, "type": "event_msg", "payload": {"type": payload_type, **payload}}


def meta(timestamp, session_id, parent_id=None, agent_path=None, model=None):
    payload = {"id": session_id, "source": "cli"}
    if model:
        payload["model"] = model
    if parent_id:
        payload["source"] = {"subagent": {"thread_spawn": {"parent_thread_id": parent_id, "agent_path": agent_path}}}
    return {"timestamp": timestamp, "type": "session_meta", "payload": payload}


def token(timestamp, input_tokens, cached_input_tokens, output_tokens):
    return row(timestamp, "token_count", info={"total_token_usage": {"input_tokens": input_tokens, "cached_input_tokens": cached_input_tokens, "output_tokens": output_tokens}})


def write_rollout(root, name, rows):
    path = Path(root) / f"rollout-{name}.jsonl"
    path.write_text("".join(json.dumps(item) + "\n" for item in rows), encoding="utf-8")
    return path


class SessionCostTests(unittest.TestCase):
    def report(self, root, target="root", since=None):
        return session_cost.build_report(target, [Path(root)], None, (10.0, 1.0, 60.0), since)

    def test_cached_input_is_charged_at_cached_rate_and_counters_are_not_summed(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), token("2026-01-01T00:00:01Z", 100, 80, 10), token("2026-01-01T00:00:02Z", 150, 120, 15)])
            report = self.report(directory)
        total = report["total"]
        self.assertEqual((total["input_tokens"], total["cached_input_tokens"], total["output_tokens"]), (150, 120, 15))
        self.assertEqual((total["uncached_input_tokens"], total["cost_usd"]), (30, 0.00132))

    def test_since_uses_last_counter_at_or_before_boundary(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), token("2026-01-01T00:00:01Z", 100, 80, 10), token("2026-01-01T00:00:03Z", 160, 120, 20)])
            report = self.report(directory, since=session_cost.parse_timestamp("2026-01-01T00:00:02Z"))
        self.assertEqual((report["total"]["input_tokens"], report["total"]["cached_input_tokens"], report["total"]["output_tokens"]), (60, 40, 10))

    def test_detected_model_uses_its_dated_rate_card_without_an_override(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root", model="gpt-5.6-luna"), token("2026-01-01T00:00:01Z", 100, 80, 10)])
            report = session_cost.build_report("root", [Path(directory)], None, None, None)
        self.assertEqual(report["sessions"][0]["model"], "gpt-5.6-luna")
        self.assertEqual(report["sessions"][0]["cost_usd"], 0.000176)

    def test_recursive_descendants_are_included(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), token("2026-01-01T00:00:01Z", 10, 0, 1)])
            write_rollout(directory, "child", [meta("2026-01-01T00:00:00Z", "child", "root", "/root/child"), token("2026-01-01T00:00:01Z", 20, 0, 2)])
            write_rollout(directory, "grandchild", [meta("2026-01-01T00:00:00Z", "grandchild", "child", "/root/child/grandchild"), token("2026-01-01T00:00:01Z", 30, 0, 3)])
            report = self.report(directory)
        self.assertEqual({item["session_id"] for item in report["sessions"]}, {"root", "child", "grandchild"})

    def test_inherited_parent_metadata_does_not_replace_child_identity(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), token("2026-01-01T00:00:01Z", 10, 0, 1)])
            write_rollout(
                directory,
                "child",
                [
                    meta("2026-01-01T00:00:00Z", "child", "root", "/root/child"),
                    meta("2026-01-01T00:00:00Z", "root"),
                    token("2026-01-01T00:00:01Z", 20, 0, 2),
                ],
            )
            report = self.report(directory)
        self.assertEqual([item["session_id"] for item in report["sessions"]], ["root", "child"])
        self.assertEqual(report["sessions"][1]["parent_id"], "root")

    def test_counter_reset_is_charged_as_new_segment_and_reported(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), token("2026-01-01T00:00:01Z", 100, 50, 10), token("2026-01-01T00:00:02Z", 20, 5, 2)])
            report = self.report(directory)
        self.assertEqual((report["total"]["input_tokens"], report["total"]["cached_input_tokens"], report["total"]["output_tokens"]), (120, 55, 12))
        self.assertTrue(any("counter reset/nonmonotone" in warning for warning in report["warnings"]))

    def test_checkpoints_use_interaction_kind_not_message_body(self):
        with tempfile.TemporaryDirectory() as directory:
            write_rollout(directory, "root", [meta("2026-01-01T00:00:00Z", "root"), row("2026-01-01T00:00:01Z", "user_message", message="private request"), token("2026-01-01T00:00:02Z", 10, 2, 1), row("2026-01-01T00:00:03Z", "task_complete", last_agent_message="private answer"), token("2026-01-01T00:00:04Z", 20, 4, 2)])
            report = self.report(directory)
        self.assertEqual([item["label"] for item in report["checkpoints"]], ["user_message", "task_complete"])
        self.assertNotIn("private", json.dumps(report))


if __name__ == "__main__":
    unittest.main()
