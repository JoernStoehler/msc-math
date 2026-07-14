import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("bridge", HERE / "bridge.py")
bridge = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(bridge)


def row(**changes):
    value = {
        "schema": bridge.SCHEMA,
        "sample_id": "sample/a",
        "law": "law",
        "law_version": "v1",
        "seed": 1,
        "row_index": 0,
        "attempt": 0,
        "attempts": 1,
        "rejections": 0,
        "parameter": "p",
        "pair_bucket": "3x3",
        "facet_count": 6,
        "accepted": True,
        "validation_status": "survived",
        "rejection_reason": "target backend disabled",
        "pairing_id": None,
        "factor_q_area": 1.0,
        "factor_p_area": 1.0,
        "factor_q_support_cv": 0.1,
        "factor_p_support_cv": 0.2,
        "factor_q_gap_cv": 0.3,
        "factor_p_gap_cv": 0.4,
        "factor_q_isoperimetric_ratio": 0.5,
        "factor_p_isoperimetric_ratio": 0.6,
        "volume": 1.0,
        "capacity": None,
        "sys": None,
        "iterations": None,
        "generation_ms": 1.0,
        "validation_ms": 2.0,
        "target_ms": 0.0,
    }
    value.update(changes)
    return value


class BridgeTests(unittest.TestCase):
    def write(self, directory, name, rows):
        path = Path(directory) / name
        with path.open("w", encoding="utf-8") as handle:
            for item in rows:
                handle.write(json.dumps(item) + "\n")
        return path

    def test_target_status_and_join(self):
        with tempfile.TemporaryDirectory() as directory:
            path = self.write(directory, "rows.jsonl", [row(), row(sample_id="sample/b", row_index=1, validation_status="runtime_cap", rejection_reason="above predeclared facet-count cap", facet_count=12)])
            provenance, prepared, report = bridge.build_sidecars([("pilot", path)])
        self.assertEqual(report["evaluated_sys_rows"], 0)
        self.assertEqual(prepared[0]["target_status"], "not_requested")
        self.assertEqual(prepared[1]["target_status"], "skipped_runtime_cap")
        self.assertEqual(prepared[0]["provenance_join"], "pilot:sample/a")
        self.assertEqual(provenance[0]["source_sha256"], report["source_files"][0]["sha256"])

    def test_evaluated_target_is_retained(self):
        with tempfile.TemporaryDirectory() as directory:
            path = self.write(directory, "rows.jsonl", [row(sys=0.5, capacity=1.0, iterations=3, target_ms=4.0, rejection_reason=None)])
            _, prepared, report = bridge.build_sidecars([("pilot", path)])
        self.assertEqual(report["evaluated_sys_rows"], 1)
        self.assertEqual(prepared[0]["sys"], 0.5)
        self.assertEqual(prepared[0]["censored_fields"], [])

    def test_target_failure_is_distinct_from_runtime_censoring(self):
        with tempfile.TemporaryDirectory() as directory:
            path = self.write(directory, "rows.jsonl", [row(validation_status="target_failed", rejection_reason="target failed after evaluation")])
            _, prepared, report = bridge.build_sidecars([("pilot", path)])
        self.assertEqual(report["target_status_counts"], {"failed": 1})
        self.assertEqual(prepared[0]["target_status"], "failed")

    def test_duplicate_and_schema_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            first = self.write(directory, "first.jsonl", [row()])
            duplicate = self.write(directory, "duplicate.jsonl", [row()])
            with self.assertRaises(bridge.BridgeError):
                bridge.build_sidecars([("a", first), ("b", duplicate)])
            incompatible = self.write(directory, "bad.jsonl", [row(schema="alternative-generator-smoke-row-v1")])
            with self.assertRaises(bridge.BridgeError):
                bridge.build_sidecars([("a", incompatible)])

    def test_nonfinite_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = self.write(directory, "rows.jsonl", [row(volume=float("nan"))])
            with self.assertRaises(bridge.BridgeError):
                bridge.build_sidecars([("a", path)])


if __name__ == "__main__":
    unittest.main()
