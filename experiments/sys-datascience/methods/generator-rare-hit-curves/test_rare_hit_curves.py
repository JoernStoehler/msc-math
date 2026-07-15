#!/usr/bin/env python3
import csv
import json
import tempfile
import unittest
from pathlib import Path

import numpy as np

import rare_hit_curves as curves


HERE = Path(__file__).resolve().parent


class RareHitCurveTests(unittest.TestCase):
    def test_rotation_quotient_and_view_contract(self):
        angles = np.linspace(0.0, 2.0 * np.pi, 6, endpoint=False)
        polygon = np.column_stack((np.cos(angles), np.sin(angles)))
        left = curves.support_vector(polygon)
        right = curves.support_vector(np.roll(polygon, 2, axis=0))
        self.assertLess(curves.rotation_distance(left, right), 1e-12)
        row = {"vertices_ccw": polygon.tolist()}
        views = curves.row_views(row)
        self.assertEqual(set(curves.VIEW_NAMES), set(views) - {"support", "angular_gap_cv"})
        self.assertTrue(np.isfinite(views["central_symmetry_residual"]))

    def test_synthetic_probability_and_censoring_controls(self):
        rows = curves.synthetic_controls()
        p01 = [row for row in rows if row["control"] == "bernoulli_p=0.1"]
        p001 = [row for row in rows if row["control"] == "bernoulli_p=0.01"]
        self.assertLess(abs(np.mean([row["hit_rate"] for row in p01]) - 0.1), 0.01)
        self.assertLess(abs(np.mean([row["hit_rate"] for row in p001]) - 0.01), 0.005)
        self.assertTrue(all(row["censored"] for row in rows if row["control"] in {"duplicate-stream", "no-hit-censoring"}))
        original = next(row for row in rows if row["control"] == "order-original")
        permuted = [row for row in rows if row["control"] == "order-permutation"]
        self.assertEqual(original["first_hit_index"], 100)
        self.assertLess(np.mean([row["first_hit_index"] for row in permuted]), 100)

    def test_target_field_scan_and_pilot_freeze(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "bad.jsonl"
            path.write_text(json.dumps({"schema": curves.ROW_SCHEMA, "sample_id": "x", "side_count": 3, "vertices_ccw": [[0, 0], [1, 0], [0, 1]], "sys": 2, "attempt": 0}) + "\n")
            with self.assertRaises(ValueError):
                curves.load_rows([path])
        polygon = [[0, 0], [1, 0], [1, 1], [0, 1]]
        pilot = [{"schema": curves.ROW_SCHEMA, "sample_id": "pilot", "population": "a", "seed": 1, "side_count": 4, "vertices_ccw": polygon, "attempt": 0}]
        # make_regions is intentionally pure with respect to confirmation rows;
        # a single-row pilot is accepted and produces deterministic thresholds.
        regions = curves.make_regions([dict(row, _views=curves.row_views(row)) for row in pilot])
        self.assertTrue(regions)

    def test_missing_duplicate_and_order_controls_fail_closed(self):
        base = [{"seed": 20260717, "population": "p", "side_count": 3, "row_index": index} for index in range(2)]
        self.assertEqual([row["row_index"] for row in curves.grouped_streams(base)[(20260717, "p", 3)]], [0, 1])
        with self.assertRaises(ValueError):
            curves.grouped_streams(base + [{"seed": 20260717, "population": "p", "side_count": 3, "row_index": 1}])
        with self.assertRaises(ValueError):
            curves.grouped_streams(list(reversed(base)))
        low, high = curves.wilson(0, 2)
        self.assertEqual(low, 0.0)
        self.assertGreater(high, 0.5)  # two-stream censoring interval remains intentionally wide
        pilot = [{"seed": 20260716, "population": "p", "side_count": 3}]
        confirmation = [{"seed": 20260717, "population": "p", "side_count": 3}]
        with self.assertRaises(ValueError):
            curves.validate_completeness(pilot, confirmation, {})

    def test_artifact_provenance_and_stratum_findings(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        self.assertEqual(report["schema"], curves.SCHEMA)
        self.assertEqual(report["rows"]["pilot"], 545)
        self.assertEqual(report["rows"]["confirmation"], 1090)
        self.assertEqual(report["protocol"]["pilot_seed"], 20260716)
        self.assertEqual(report["protocol"]["confirmation_seeds"], [20260717, 20260718])
        self.assertFalse(report["provenance"]["source_dirty"])
        self.assertEqual(set(report["provenance"]["source_file_hashes"]), {"analyzer", "tests", "readme"})
        self.assertNotIn("validation_ms", report["provenance"])
        self.assertEqual(len(report["producer_reports"]), 6)
        self.assertEqual(len(report["rows"]["pilot_inputs"] + report["rows"]["confirmation_inputs"]), 6)
        with (HERE / "artifacts/analysis/stratum-findings.tsv").open() as handle:
            findings = list(csv.DictReader(handle, delimiter="\t"))
        self.assertEqual(len(findings), 207)
        self.assertTrue(any(row["classification"] == "not-reobserved-both-confirmation-seeds-right-censored" for row in findings))
        self.assertTrue(any(row["classification"] == "replicates-both-confirmation-seeds" for row in findings))
        with (HERE / "artifacts/analysis/hit-curves.tsv").open() as handle:
            curve_fields = next(csv.reader(handle, delimiter="\t"))
            curve_rows = list(csv.DictReader(handle, fieldnames=curve_fields, delimiter="\t"))
        self.assertNotIn("seed", curve_fields)
        self.assertEqual(set(curve_fields) & {"wilson_low", "wilson_high"}, {"wilson_low", "wilson_high"})
        self.assertTrue(all(row["stream_count"] == "2" for row in curve_rows))


if __name__ == "__main__":
    unittest.main()
