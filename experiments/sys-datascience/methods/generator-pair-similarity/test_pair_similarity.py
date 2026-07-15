#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

import importlib.util
import math
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import numpy as np

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("pair_similarity", HERE / "pair_similarity.py")
M = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = M
SPEC.loader.exec_module(M)


class PairSimilarityTests(unittest.TestCase):
    def test_representation_ladder_preserves_order_diagnostic_then_removes_it(self):
        base = M.regular_polygon(5, 0.17)
        shifted = base[[2, 3, 4, 0, 1]]
        self.assertGreater(M.raw_coordinate_distances(base, shifted)["l2"], 1.0)
        self.assertLess(M.canonical_cyclic_l2(base, shifted), 1e-14)
        self.assertLess(M.exact_assignment_l2(base, shifted), 1e-14)
        self.assertLess(M.cyclic_dihedral_vertex_l2(base, shifted), 1e-14)
        self.assertLess(
            M.permitted_permutation_assignment_l2(
                base, shifted, [range(5), [3, 4, 0, 1, 2]]
            ),
            1e-14,
        )

    def test_planar_cyclic_grid_quotient_and_reflection_switch(self):
        base = M.synthetic_planar_cases()["base"]
        moved = M.synthetic_planar_cases()["translated_scaled_grid_rotated"]
        off_grid = M.synthetic_planar_cases()["off_grid_rotated"]
        reflected = M.synthetic_planar_cases()["reflected"]
        aligned = M.planar_support_distance(base, moved)
        self.assertEqual(aligned["rotation_group"], f"C_{M.SUPPORT_ANGLES}")
        self.assertEqual(aligned["support_grid"], M.SUPPORT_ANGLES)
        self.assertLess(aligned["support_rms"], 1e-10)
        self.assertGreater(M.planar_support_distance(base, off_grid)["support_rms"], 1e-4)
        self.assertGreater(M.planar_support_distance(base, reflected)["support_rms"], 1e-3)
        reflected_distance = M.planar_support_distance(base, reflected, allow_reflection=True)
        self.assertEqual(reflected_distance["rotation_group"], f"D_{M.SUPPORT_ANGLES}")
        self.assertLess(reflected_distance["support_rms"], 1e-10)

    def test_4d_gram_controls_and_bounded_disposition(self):
        cases = M.synthetic_4d_cases()
        self.assertLess(M.euclidean_gram_distance(cases["base"], cases["orthogonal_not_symplectic"]), 1e-12)
        self.assertLess(M.symplectic_gram_quotient(cases["base"], cases["facet_permuted"]), 1e-12)
        self.assertLess(M.symplectic_gram_quotient(cases["base"], cases["symplectic"]), 1e-12)
        self.assertGreater(M.symplectic_gram_quotient(cases["base"], cases["orthogonal_not_symplectic"]), 1e-3)
        over_cap = np.column_stack((M.regular_polygon(9), M.regular_polygon(9)))
        self.assertIsNone(M.symplectic_gram_quotient(over_cap, over_cap))

    def test_incidence_and_variable_facet_distance(self):
        incidence = [[1, 0, 1], [0, 1, 1], [1, 1, 0]]
        self.assertEqual(M.incidence_isomorphism_hamming(incidence, [[1, 1, 0], [1, 0, 1], [0, 1, 1]]), 0.0)
        self.assertGreater(M.normalized_vertex_cloud_hausdorff(M.regular_polygon(4), M.regular_polygon(5)), 0.01)

    def test_adversarial_finite_triangle_suite_and_smoke_schema(self):
        calibration = M.calibration()
        self.assertTrue(calibration["claimed_controls_pass"])
        self.assertTrue(all(value["passed"] for value in calibration["symmetry"].values()))
        self.assertTrue(
            all(row["passed"] for row in calibration["equivalence_regression_matrix"])
        )
        rows = []
        for population, polygon in (("regular", M.regular_polygon(4)), ("deformed", M.regular_polygon(4) @ [[1.4, 0.2], [0, 0.8]])):
            rows.append({"schema": "factor-shape-row-v1", "sample_id": population, "population": population, "vertices_ccw": polygon.tolist()})
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "fixture.jsonl"
            path.write_text("\n".join(__import__("json").dumps(row) for row in rows) + "\n")
            report = M.smoke_report(M.load_factor_shapes(path), 1)
        self.assertEqual(report["selected_row_count"], 2)
        self.assertEqual(report["comparison_table"]["support_rms"]["pair_count"], 1)

    def test_real_smoke_regeneration_is_byte_deterministic(self):
        input_path = (
            HERE.parent / "generator-zoo-smoke" / "artifacts" / "factor-shapes.jsonl"
        )
        self.assertTrue(input_path.is_file(), "real generator-zoo fixture is required")
        self.assertFalse(
            input_path.read_text(encoding="utf-8").startswith("version https://git-lfs"),
            "hydrate the LFS fixture before running this reproducibility test",
        )
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            first, second = root / "first", root / "second"
            command = [
                sys.executable,
                str(HERE / "pair_similarity.py"),
                "--input",
                str(input_path),
                "--per-population",
                "2",
            ]
            for output in (first, second):
                subprocess.run(
                    [*command, "--out-dir", str(output)],
                    check=True,
                    capture_output=True,
                    text=True,
                )
            for name in ("report.json", "comparison.tsv"):
                self.assertEqual((first / name).read_bytes(), (second / name).read_bytes())


if __name__ == "__main__":
    unittest.main()
