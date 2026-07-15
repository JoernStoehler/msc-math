#!/usr/bin/env python3
import csv
import hashlib
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

    def test_artifact_provenance_and_stratum_findings(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        self.assertEqual(report["schema"], curves.SCHEMA)
        self.assertEqual(report["rows"]["pilot"], 545)
        self.assertEqual(report["rows"]["confirmation"], 1090)
        self.assertEqual(report["protocol"]["pilot_seed"], 20260716)
        self.assertEqual(report["protocol"]["confirmation_seeds"], [20260717, 20260718])
        self.assertEqual(report["provenance"]["analyzer_sha256"], hashlib.sha256(Path(__file__).with_name("rare_hit_curves.py").read_bytes()).hexdigest())
        with (HERE / "artifacts/analysis/stratum-findings.tsv").open() as handle:
            findings = list(csv.DictReader(handle, delimiter="\t"))
        self.assertEqual(len(findings), 207)
        self.assertTrue(any(row["classification"] == "pilot-artifact-no-confirmation-hit" for row in findings))
        self.assertTrue(any(row["classification"] == "replicates-both-confirmation-seeds" for row in findings))


if __name__ == "__main__":
    unittest.main()
