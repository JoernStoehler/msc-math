#!/usr/bin/env python3
import csv
import hashlib
import json
from pathlib import Path
import subprocess
import unittest

import numpy as np

import atlas
import shape_quality


HERE = Path(__file__).resolve().parent


class ConfirmationTests(unittest.TestCase):
    def test_rows_and_seeds(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        self.assertEqual(report["rows_validated"], 1635)
        self.assertEqual(report["master_seeds"], [20260716, 20260717, 20260718])
        self.assertTrue(report["repository"]["tracked_clean"])

    def test_named_overlap_and_reversal_are_visible(self):
        with (HERE / "artifacts/analysis/joint-effects.tsv").open() as handle:
            rows = list(csv.DictReader(handle, delimiter="\t"))
        overlap = [row for row in rows if row["contrast"] == "baseline_vs_alpha1_nearest_cross_overlap" and row["effect"] == "bidirectional_substantial"]
        self.assertEqual({row["side_count"] for row in overlap}, {"3", "4", "6"})
        self.assertTrue(all(row["pass_rate"] == "1.0" for row in overlap))
        with (HERE / "artifacts/analysis/rank-stability.tsv").open() as handle:
            stability = list(csv.DictReader(handle, delimiter="\t"))
        self.assertTrue(any(row["order_reversal"] == "True" for row in stability if row["master_seed"] != "joint"))

    def test_scale_and_boundary_contracts(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        config = report["configuration"]
        self.assertIn("scale-sensitive", config["raw_feature_boundary"])
        self.assertIn("not intrinsic", config["positive_gram_boundary"])
        self.assertIn("declared-grid", config["distance_contract"])
        self.assertEqual(report["producer_provenance"]["source_revision"], "fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e")

    def test_lineage_and_implementation_hashes(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        repo = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], cwd=HERE, text=True).strip())
        source_revision = report["repository"]["revision"]
        lineage = subprocess.run(["git", "merge-base", "--is-ancestor", source_revision, "HEAD"], cwd=repo)
        self.assertEqual(lineage.returncode, 0)
        expected = {
            "analyze_py_sha256": hashlib.sha256((HERE / "analyze.py").read_bytes()).hexdigest(),
            "atlas_py_sha256": hashlib.sha256((HERE / "atlas.py").read_bytes()).hexdigest(),
            "shape_quality_py_sha256": hashlib.sha256((HERE / "shape_quality.py").read_bytes()).hexdigest(),
        }
        self.assertEqual(report["implementation_hashes"], expected)

    def test_executable_path_is_packet_relative_and_matches_hash(self):
        report = json.loads((HERE / "artifacts/analysis/report.json").read_text())
        producer = report["producer_provenance"]
        self.assertIn("packet directory", producer["executable_path_base"])
        executable = (HERE / producer["executable_path_at_capture"]).resolve()
        self.assertTrue(executable.is_file())
        self.assertEqual(hashlib.sha256(executable.read_bytes()).hexdigest(), producer["executable_sha256"])

    def test_local_grid_metric_and_geometry_copy(self):
        support = np.sin(2.0 * np.pi * np.arange(64) / 64.0)
        self.assertLess(atlas.l2(support, np.roll(support, 11)), 1e-12)
        polygon = [[0.0, 0.0], [2.0, 0.0], [2.5, 0.8], [1.2, 1.9], [-0.2, 1.0]]
        shape = shape_quality.standardize_row({"schema": shape_quality.SCHEMA, "sample_id": "test", "law": "test", "side_count": 5, "vertices_ccw": polygon}, 64, 1024, 1)
        self.assertEqual(shape.side_count, 5)
        self.assertTrue(np.all(np.isfinite(shape.support)))

    def test_saturation_side_coverage_and_narrow_broad_comparison(self):
        with (HERE / "artifacts/analysis/saturation.tsv").open() as handle:
            rows = list(csv.DictReader(handle, delimiter="\t"))
        keys = {(row["master_seed"], row["side_count"], row["population"], row["requested_n"]) for row in rows}
        self.assertEqual(len(rows), 216)  # 3 seeds * 3 sides * 6 laws * 4 prefixes
        self.assertEqual(len(keys), len(rows))
        self.assertEqual({row["side_count"] for row in rows}, {"3", "4", "6"})
        with (HERE / "artifacts/analysis/joint-effects.tsv").open() as handle:
            joint = list(csv.DictReader(handle, delimiter="\t"))
        comparison = [row for row in joint if row["contrast"] == "saturation_narrow_vs_broad" and row["effect"] == "narrow_stabilizes_earlier"]
        self.assertEqual({row["side_count"] for row in comparison}, {"3", "4", "6"})
        self.assertTrue(all(row["seeds"] == "3" for row in comparison))
        self.assertTrue(all("unavailable_seeds" in row for row in comparison))


if __name__ == "__main__":
    unittest.main()
