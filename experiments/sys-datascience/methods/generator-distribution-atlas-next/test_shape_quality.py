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
SPEC = importlib.util.spec_from_file_location("quality_atlas_shape_quality", HERE / "shape_quality.py")
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
    def test_bounded_selection_is_hash_ranked_order_invariant_and_prefix_stable(self):
        payloads = []
        for population in ("baseline", "candidate"):
            for index in range(7):
                vertices = ANALYZE.transform_polygon(
                    ANALYZE.regular_polygon(5),
                    stretch=0.03 * (index + (population == "candidate")),
                    shear=0.01 * index,
                    rotation=0.07 * index,
                    scale=1.0,
                    translation=(0.0, 0.0),
                )
                payload = row(f"{population}/sample-{index}", "shared-law", vertices)
                payload["population"] = population
                payload["provenance"] = f"mutable note {index}"
                payloads.append(payload)
        shapes = [standardized(payload, 32, 64) for payload in payloads]

        selected_forward, summary_forward = ANALYZE.select_bounded_shapes(shapes, 3)
        selected_reverse, summary_reverse = ANALYZE.select_bounded_shapes(
            list(reversed(shapes)), 3
        )
        self.assertEqual(
            [shape.sample_id for shape in selected_forward],
            [shape.sample_id for shape in selected_reverse],
        )
        self.assertEqual(summary_forward, summary_reverse)

        selected_four, _ = ANALYZE.select_bounded_shapes(shapes, 4)
        for population in ("baseline", "candidate"):
            ids_three = {
                shape.sample_id for shape in selected_forward if shape.law == population
            }
            ids_four = {
                shape.sample_id for shape in selected_four if shape.law == population
            }
            self.assertEqual(len(ids_three), 3)
            self.assertTrue(ids_three < ids_four)
            observed = [shape for shape in shapes if shape.law == population]
            expected = {
                shape.sample_id
                for shape in sorted(observed, key=ANALYZE.bounded_selection_key)[:3]
            }
            self.assertEqual(ids_three, expected)

        report_forward = ANALYZE.build_atlas(shapes, "baseline", 0.9, 1e-9, 3)
        report_reverse = ANALYZE.build_atlas(
            list(reversed(shapes)), "baseline", 0.9, 1e-9, 3
        )
        self.assertEqual(report_forward, report_reverse)
        self.assertEqual(report_forward["rows_observed"], 14)
        self.assertEqual(report_forward["rows_used_for_geometry"], 6)
        self.assertEqual(report_forward["rows_excluded_from_geometry"], 8)
        self.assertIn("not a random", report_forward["bounded_analysis"]["interpretation"])
        laws = {law["law"]: law for law in report_forward["strata"][0]["laws"]}
        self.assertEqual(laws["candidate"]["observed_count"], 7)
        self.assertEqual(laws["candidate"]["used_count"], 3)
        self.assertEqual(laws["candidate"]["excluded_count"], 4)
        self.assertEqual(laws["candidate"]["sample_status"], "small-sample")
        self.assertEqual(
            report_forward["accepted_row_side_count_allocation"]["by_population"]
            ["candidate"]["accepted_shape_row_counts_by_side_count"]["5"],
            7,
        )

    def test_bounded_selection_rejects_nonpositive_cap(self):
        shape = standardized(row("one", "baseline", ANALYZE.regular_polygon(5)), 32, 64)
        for cap in (0, -1):
            with self.subTest(cap=cap), self.assertRaisesRegex(ValueError, "positive"):
                ANALYZE.select_bounded_shapes([shape], cap)

    def test_side_count_tv_is_explicitly_an_accepted_row_allocation_diagnostic(self):
        payloads = []
        allocations = {
            "baseline": [4, 4, 5, 5],
            "candidate": [4, 4, 4, 5],
        }
        for population, side_counts in allocations.items():
            for index, side_count in enumerate(side_counts):
                payload = row(
                    f"{population}-{index}", "shared-law", ANALYZE.regular_polygon(side_count)
                )
                payload["population"] = population
                payloads.append(payload)
        report = ANALYZE.build_atlas(
            [standardized(payload, 32, 64) for payload in payloads],
            "baseline",
            0.9,
            1e-9,
        )
        diagnostic = report["accepted_row_side_count_allocation"]
        self.assertIn("not an estimate", diagnostic["interpretation"])
        candidate = diagnostic["by_population"]["candidate"]
        self.assertEqual(
            candidate["accepted_shape_row_counts_by_side_count"], {"4": 3, "5": 1}
        )
        self.assertAlmostEqual(
            candidate["accepted_row_side_count_allocation_tv_from_baseline"], 0.25
        )
        self.assertNotIn("combinatorial_side_count_breadth", report)

    def test_synthetic_narrow_and_broad_laws_are_discriminated(self):
        shapes = [standardized(payload) for payload in ANALYZE.synthetic_rows()]
        report = ANALYZE.build_atlas(shapes, "baseline", 0.9, 1e-9)
        self.assertEqual(report["side_counts"], [5, 6])
        self.assertFalse(report["bounded_analysis"]["enabled"])
        self.assertEqual(report["rows_observed"], report["rows_used_for_geometry"])
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
            self.assertNotIn(b"\r\n", first["atlas.tsv"])
            self.assertTrue(first["atlas.tsv"].endswith(b"\n"))
            ANALYZE.write_outputs(report, out, config)
            second = {path.name: path.read_bytes() for path in out.iterdir()}
            self.assertEqual(first, second)
            parsed = json.loads((out / "report.json").read_text())
            self.assertTrue(parsed["no_combined_quality_score"])


if __name__ == "__main__":
    unittest.main()
