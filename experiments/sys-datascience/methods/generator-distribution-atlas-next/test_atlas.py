#!/usr/bin/env python3
"""Small schema and interpretation tests for the multi-view adapter."""

import json
import importlib.util
import math
from pathlib import Path
import unittest

import numpy as np


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

    def test_source_contract_binds_producer_and_analyzer(self):
        provenance = json.loads((HERE / "artifacts/panel/provenance.json").read_text())
        report = json.loads((HERE / "artifacts/atlas/report.json").read_text())
        producer = provenance["producer"]
        self.assertEqual(len(producer["executable_sha256"]), 64)
        self.assertEqual(producer["source_revision"], "fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e")
        self.assertEqual(set(producer["source_blobs"]), {"Cargo.lock", "experiments/sys-datascience/methods/generator-zoo-smoke/main.rs", "experiments/sys-landscape/Cargo.toml"})
        self.assertEqual(provenance["analyzer"], report["implementation_hashes"])
        self.assertEqual(report["provenance_artifact"], "artifacts/panel/provenance.json")
        self.assertTrue((HERE / "artifacts/atlas/source-exact-validation-witness/linkage.json").is_file())
        self.assertFalse((HERE / "artifacts/atlas/exact-subset").exists())


if __name__ == "__main__":
    unittest.main()
