#!/usr/bin/env python3
"""Small schema and interpretation tests for the multi-view adapter."""

import json
from pathlib import Path
import subprocess
import sys
import unittest


HERE = Path(__file__).resolve().parent


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


if __name__ == "__main__":
    unittest.main()
