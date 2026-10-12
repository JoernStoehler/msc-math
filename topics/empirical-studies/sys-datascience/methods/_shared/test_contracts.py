#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "matplotlib", "scipy", "scikit-learn"]
# ///
"""Small consumer-boundary regressions; no retained inputs or producers."""

import importlib.util
from pathlib import Path
import tempfile
import unittest

from relocation import REPO_ROOT, resolve_repo_path

from random_only import (
    ACTIVE_INVARIANT_NUMERIC_FEATURES,
    active_invariant_numeric_feature_names,
    first_height_range,
    write_jsonl,
)

METHODS = Path(__file__).resolve().parents[1]


def load_method(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), METHODS / name / "analyze.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class RelocationTests(unittest.TestCase):
    def test_frozen_label_resolves_to_new_tree_without_changing_label(self):
        label = "experiments/sys-datascience/methods/generator-orientation-target-pilot/design.json"
        target = resolve_repo_path(label)
        self.assertEqual(target, REPO_ROOT / "topics/empirical-studies/sys-datascience/methods/generator-orientation-target-pilot/design.json")
        self.assertTrue(target.is_file())
        self.assertTrue(label.startswith("experiments/"))

    def test_unmoved_repository_and_external_paths_stay_put(self):
        self.assertEqual(resolve_repo_path("Cargo.lock"), REPO_ROOT / "Cargo.lock")
        self.assertEqual(resolve_repo_path("/tmp/external-fixture.json"), Path("/tmp/external-fixture.json"))


class FeatureCoverageTests(unittest.TestCase):
    def rows(self, count):
        return [{key: 1.0 for key in ACTIVE_INVARIANT_NUMERIC_FEATURES} for _ in range(count)]

    def test_small_fixture_does_not_round_98_percent_down(self):
        rows = self.rows(3)
        del rows[0]["vertex_count"]
        with self.assertRaisesRegex(SystemExit, "vertex_count"):
            active_invariant_numeric_feature_names(rows)

    def test_exact_98_percent_is_eligible(self):
        rows = self.rows(50)
        del rows[0]["vertex_count"]
        self.assertIn("vertex_count", active_invariant_numeric_feature_names(rows))

    def test_ordering_failure_stops_whole_matrix(self):
        rows = self.rows(2)
        rows[0]["ridge_symp_area_ordering_failure_count"] = 1
        with self.assertRaisesRegex(SystemExit, "complete two-face ordering"):
            active_invariant_numeric_feature_names(rows)


class MetadataTests(unittest.TestCase):
    def test_height_label_shared_without_changing_consumer_semantics(self):
        cases = [([], "missing"), ([{"sample_h_min": 0.8, "sample_h_max": 1.2}], "0.8:1.2"),
                 ([{"sample_h_min": 0.8, "sample_h_max": 1.2},
                   {"sample_h_min": 0.6, "sample_h_max": 1.4}], "multi:0.6:1.4,0.8:1.2")]
        for name in ["prediction-ranking", "projection-structure", "tail-rule-mining"]:
            self.assertIs(load_method(name).first_height_range, first_height_range)
        for rows, expected in cases:
            self.assertEqual(first_height_range(rows), expected)


class BoundedMarginalTests(unittest.TestCase):
    def test_targets_at_or_above_one_and_nonfinite_rows_are_not_clipped_into_fit(self):
        broad = load_method("sys-distribution-broad-scan")
        with tempfile.TemporaryDirectory() as directory:
            tables = Path(directory)
            write_jsonl(tables / "polytope-provenance-table.jsonl", [
                {"poly_id": "fixture", "dataset": "random_sample"}])
            for value in [0.0, 1.0, 1.1, float("nan"), float("inf")]:
                write_jsonl(tables / "polytope-table.jsonl", [
                    {"poly_id": "fixture", "capacity_source": "random_sample", "facet_count": 5, "sys": value}])
                with self.subTest(value=value), self.assertRaisesRegex(SystemExit, "finite 0 < sys < 1"):
                    broad.load_groups(tables)
            write_jsonl(tables / "polytope-table.jsonl", [
                {"poly_id": "fixture", "capacity_source": "random_sample", "facet_count": 5, "sys": 0.75}])
            self.assertEqual(broad.load_groups(tables)["generic:F5"].tolist(), [0.75])


if __name__ == "__main__":
    unittest.main()
