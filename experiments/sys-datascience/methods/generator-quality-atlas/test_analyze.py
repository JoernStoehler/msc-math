#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

import importlib.util
import json
import math
from pathlib import Path
import sys
import tempfile
import unittest

import numpy as np


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("quality_atlas_analyze", HERE / "analyze.py")
ANALYZE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = ANALYZE
SPEC.loader.exec_module(ANALYZE)


def row(sample_id, law, vertices):
    return {
        "schema": ANALYZE.SCHEMA,
        "sample_id": sample_id,
        "law": law,
        "side_count": len(vertices),
        "vertices_ccw": np.asarray(vertices).tolist(),
    }


def standardized(payload, support_grid=256, steiner_grid=4096):
    return ANALYZE.standardize_row(payload, support_grid, steiner_grid, 1)


class GeometryTests(unittest.TestCase):
    def test_population_label_separates_knob_settings(self):
        payload = row("one", "family", ANALYZE.regular_polygon(5))
        payload["population"] = "family[knob=2]"
        self.assertEqual(standardized(payload).law, "family[knob=2]")

    def test_rotation_scale_translation_are_quotiented(self):
        polygon = ANALYZE.transform_polygon(
            ANALYZE.regular_polygon(5),
            stretch=0.31,
            shear=0.12,
            rotation=0.0,
            scale=1.0,
            translation=(0.0, 0.0),
        )
        transformed = ANALYZE.transform_polygon(
            polygon,
            stretch=0.0,
            shear=0.0,
            rotation=37 * 2.0 * math.pi / 256.0,
            scale=4.7,
            translation=(13.0, -8.0),
        )
        left = standardized(row("left", "baseline", polygon))
        right = standardized(row("right", "baseline", transformed))
        l2, linf, _, _ = ANALYZE.shape_rotation_metrics(left, right)
        self.assertLess(l2, 2e-9)
        self.assertLess(linf, 5e-9)

    def test_arbitrary_rotation_is_continuously_quotiented(self):
        polygon = ANALYZE.transform_polygon(
            ANALYZE.regular_polygon(5), 0.31, 0.12, 0.0, 1.0, (0.0, 0.0)
        )
        transformed = ANALYZE.transform_polygon(
            polygon, 0.0, 0.0, 0.3712345, 2.3, (-4.0, 7.0)
        )
        left = standardized(row("left", "baseline", polygon))
        right = standardized(row("right", "baseline", transformed))
        l2, linf, _, _ = ANALYZE.shape_rotation_metrics(left, right)
        self.assertLess(l2, 2e-9)
        self.assertLess(linf, 5e-9)

    def test_exact_polygon_steiner_center_matches_dense_support_integral(self):
        polygon = np.array([[0.0, 0.0], [2.0, 0.0], [2.7, 0.8], [1.4, 2.3], [-0.3, 1.1]])
        exact = ANALYZE.polygon_steiner_center(polygon)
        numerical = ANALYZE.numerical_steiner_center(polygon, 1 << 18)
        self.assertLess(float(np.linalg.norm(exact - numerical)), 2e-8)

    def test_reflection_is_not_silently_quotiented(self):
        polygon = np.array([[0.0, 0.0], [2.0, 0.0], [2.7, 0.8], [1.4, 2.3], [-0.3, 1.1]])
        reflected = polygon.copy()
        reflected[:, 0] *= -1.0
        reflected = reflected[::-1]
        left = standardized(row("left", "baseline", polygon))
        right = standardized(row("right", "candidate", reflected))
        l2, _, _, _ = ANALYZE.shape_rotation_metrics(left, right)
        self.assertGreater(l2, 1e-3)

    def test_perturbation_is_detected(self):
        polygon = ANALYZE.regular_polygon(6)
        perturbed = ANALYZE.transform_polygon(
            polygon, 0.25, 0.1, 0.0, 1.0, (0.0, 0.0)
        )
        left = standardized(row("left", "baseline", polygon))
        right = standardized(row("right", "candidate", perturbed))
        l2, _, _, _ = ANALYZE.shape_rotation_metrics(left, right)
        self.assertGreater(l2, 0.02)

    def test_invalid_polygon_orders_and_concavity_fail(self):
        square = np.array([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]])
        with self.assertRaisesRegex(ValueError, "strictly convex, cyclic, and CCW"):
            standardized(row("clockwise", "baseline", square[::-1]))
        concave = np.array([[0.0, 0.0], [2.0, 0.0], [1.0, 0.4], [2.0, 2.0], [0.0, 2.0]])
        with self.assertRaisesRegex(ValueError, "strictly convex, cyclic, and CCW"):
            standardized(row("concave", "baseline", concave))


class AtlasTests(unittest.TestCase):
    def test_synthetic_narrow_and_broad_laws_are_discriminated(self):
        shapes = [standardized(payload) for payload in ANALYZE.synthetic_rows()]
        report = ANALYZE.build_atlas(shapes, "baseline", 0.9, 1e-9)
        self.assertEqual(report["side_counts"], [5, 6])
        for stratum in report["strata"]:
            laws = {law["law"]: law for law in stratum["laws"]}
            self.assertEqual(laws["baseline"]["within_l2"]["duplicate_pair_fraction"], 1.0)
            self.assertGreater(
                laws["broad"]["within_l2"]["pairwise_mean"],
                5 * laws["narrow"]["within_l2"]["pairwise_mean"],
            )
            self.assertGreater(
                laws["broad"]["baseline_comparison"]["energy_like_l2_v_statistic"],
                laws["narrow"]["baseline_comparison"]["energy_like_l2_v_statistic"],
            )
            self.assertGreater(
                laws["broad"]["baseline_comparison"]["centroid_distance_baseline_medoid_gauge"],
                5 * laws["narrow"]["baseline_comparison"]["centroid_distance_baseline_medoid_gauge"],
            )
            self.assertEqual(
                laws["broad"]["baseline_comparison"]["outside_baseline_central_body_fraction"],
                1.0,
            )

    def test_missing_baseline_and_small_denominator_are_visible(self):
        payloads = [
            row(f"candidate-{i}", "candidate", ANALYZE.regular_polygon(4))
            for i in range(2)
        ]
        report = ANALYZE.build_atlas([standardized(item) for item in payloads], "baseline", 0.9, 1e-9)
        law = report["strata"][0]["laws"][0]
        self.assertFalse(law["baseline_comparison"]["baseline_available"])
        self.assertEqual(law["sample_status"], "small-sample")
        self.assertTrue(report["issues"])

    def test_regeneration_is_byte_deterministic(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            fixture = root / "fixture.jsonl"
            ANALYZE.write_jsonl(ANALYZE.synthetic_rows(), fixture)
            shapes = ANALYZE.load_shapes(fixture, 64, 1024)
            report = ANALYZE.build_atlas(shapes, "baseline", 0.9, 1e-9)
            config = {
                "input": "fixture.jsonl",
                "baseline_law": "baseline",
                "support_grid": 64,
                "steiner_grid": 1024,
                "central_fraction": 0.9,
                "duplicate_tolerance": 1e-9,
            }
            out = root / "out"
            ANALYZE.write_outputs(report, out, config)
            first = {path.name: path.read_bytes() for path in out.iterdir()}
            ANALYZE.write_outputs(report, out, config)
            second = {path.name: path.read_bytes() for path in out.iterdir()}
            self.assertEqual(first, second)
            parsed = json.loads((out / "report.json").read_text())
            self.assertTrue(parsed["no_combined_quality_score"])


if __name__ == "__main__":
    unittest.main()
