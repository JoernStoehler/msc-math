#!/usr/bin/env python3
"""Small schema and interpretation tests for the multi-view adapter."""

import json
import importlib.util
import math
from pathlib import Path
import unittest

import numpy as np

import shape_quality


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("atlas_adapter", HERE / "atlas.py")
ATLAS = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(ATLAS)


class AtlasSmokeTests(unittest.TestCase):
    def test_panel_and_report_are_target_free_and_stratified(self):
        panel = HERE / "artifacts/panel/factor-shapes.jsonl"
        report = HERE / "artifacts/atlas/report.json"
        rows = [json.loads(line) for line in panel.read_text().splitlines() if line.strip()]
        payload = json.loads(report.read_text())
        self.assertEqual(len(rows), 544)
        self.assertEqual(payload["rows_validated"], len(rows))
        self.assertEqual(len(payload["populations"]), 8)
        self.assertIn("target prediction", " ".join(payload["interpretation"]["prohibited"]))
        self.assertEqual(payload["rank_uncertainty"]["pilot_selection_confirmation"], "deferred")
        self.assertEqual(payload["structural_product_classification"]["status"], "deferred")

    def test_view_tables_are_present_and_have_rows(self):
        expected = (
            "within-population.tsv",
            "between-population.tsv",
            "nearest-cross-population.tsv",
            "combinatorial-occupancy.tsv",
            "acceptance-cost.tsv",
            "sample-size-saturation.tsv",
            "feature-spectrum.tsv",
            "feature-range-overlap.tsv",
            "feature-law-confounding.tsv",
        )
        for name in expected:
            with self.subTest(name=name):
                lines = (HERE / "artifacts/atlas" / name).read_text().splitlines()
                self.assertGreaterEqual(len(lines), 2)

    def test_covariance_anisotropy_is_rotation_invariant(self):
        vertices = np.array([[0.0, 0.0], [2.0, 0.0], [2.4, 0.7], [1.2, 2.0], [-0.2, 1.1]])
        theta = 0.371
        rotation = np.array([[math.cos(theta), -math.sin(theta)], [math.sin(theta), math.cos(theta)]])
        left = type("ShapeStub", (), {"vertices": vertices})()
        right = type("ShapeStub", (), {"vertices": vertices @ rotation.T})()
        self.assertAlmostEqual(ATLAS.invariant_features(left)[1], ATLAS.invariant_features(right)[1], places=12)

    def test_grid_distance_calibration(self):
        support = np.sin(2.0 * np.pi * np.arange(64) / 64.0) + 0.2 * np.cos(5.0 * 2.0 * np.pi * np.arange(64) / 64.0)
        self.assertLess(ATLAS.l2(support, np.roll(support, 7)), 1e-12)
        polygon = np.array([[0.0, 0.0], [2.0, 0.0], [2.6, 0.8], [1.3, 2.0], [-0.2, 1.0]])
        theta = 0.1234
        rotation = np.array([[math.cos(theta), -math.sin(theta)], [math.sin(theta), math.cos(theta)]])
        left = shape_quality.standardize_row({"schema": shape_quality.SCHEMA, "sample_id": "left", "law": "calibration", "side_count": 5, "vertices_ccw": polygon.tolist()}, 64, 1024, 1)
        right = shape_quality.standardize_row({"schema": shape_quality.SCHEMA, "sample_id": "right", "law": "calibration", "side_count": 5, "vertices_ccw": (polygon @ rotation.T).tolist()}, 64, 1024, 1)
        continuous, _, _, _ = shape_quality.shape_rotation_metrics(left, right)
        grid = ATLAS.l2(left.support, right.support)
        self.assertLess(abs(grid - continuous), 0.02)

    def test_source_contract_binds_producer_and_analyzer(self):
        provenance = json.loads((HERE / "artifacts/panel/provenance.json").read_text())
        report = json.loads((HERE / "artifacts/atlas/report.json").read_text())
        producer = provenance["producer"]
        self.assertEqual(len(producer["executable_sha256"]), 64)
        self.assertEqual(producer["source_revision"], "fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e")
        self.assertEqual(set(producer["source_blobs"]), {"Cargo.lock", "experiments/sys-datascience/methods/generator-zoo-smoke/main.rs", "experiments/sys-landscape/Cargo.toml"})
        self.assertEqual(provenance["analyzer"], report["implementation_hashes"])
        self.assertEqual(report["provenance_artifact"], "artifacts/panel/provenance.json")
        linkage = report["source_exact_validation_witness"]
        self.assertEqual(linkage["source"], "../generator-zoo-smoke/artifacts/factor-shapes.jsonl")
        self.assertTrue((HERE / "artifacts/atlas/source-exact-validation-witness/linkage.json").is_file())
        self.assertFalse((HERE / "artifacts/atlas/exact-subset").exists())
        within_header = (HERE / "artifacts/atlas/within-population.tsv").read_text().splitlines()[0]
        between_header = (HERE / "artifacts/atlas/between-population.tsv").read_text().splitlines()[0]
        spectrum_header = (HERE / "artifacts/atlas/feature-spectrum.tsv").read_text().splitlines()[0]
        self.assertIn("positive_gram_spectrum_participation_ratio", within_header)
        self.assertNotIn("effective_dimension", within_header)
        self.assertIn("raw_feature_centroid_separation", between_header)
        self.assertIn("raw_feature_covariance_spectrum_participation_ratio", spectrum_header)


if __name__ == "__main__":
    unittest.main()
