#!/usr/bin/env python3
import copy
import importlib.util
import json
import sys
import subprocess
import tempfile
import unittest
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("augment", HERE / "augment.py")
augment = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(augment)
sys.modules["augment"] = augment
ASPEC = importlib.util.spec_from_file_location("analyze", HERE / "analyze.py")
analyzer = importlib.util.module_from_spec(ASPEC)
assert ASPEC and ASPEC.loader
ASPEC.loader.exec_module(analyzer)

ORIENTATION = Path(__file__).parents[2] / "methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"


class ExactFeaturePacketTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.rows, _ = augment.load_rows(ORIENTATION)
        cls.features = [augment.feature_row(row, "orientation") for row in cls.rows]

    def test_exact_join_and_strict_cycle_scope(self):
        self.assertEqual(len(self.features), 40)
        cycles = [r for r in self.features if r["strict_cycle"] is not None]
        self.assertEqual(len(cycles), 2)
        self.assertTrue(all(r["strict_cycle"]["strict_sign_cell"] for r in cycles))
        self.assertTrue(all(r["geometry_validation_status"] == "validated" for r in self.features))

    def test_unflagged_exact_incidence_equality_is_rejected(self):
        row = copy.deepcopy(self.rows[0])
        row["labeled_incidence_signature"][0].pop()
        with self.assertRaisesRegex(augment.AnalysisError, "unflagged equality"):
            augment.feature_row(row, "orientation")

    def test_analytic_area_and_decomposition(self):
        q_square = [[Fraction(0), Fraction(0), Fraction(0), Fraction(0)], [Fraction(1), 0, 0, 0], [Fraction(1), 1, 0, 0], [Fraction(0), 1, 0, 0]]
        qp_square = [[Fraction(0), 0, 0, 0], [Fraction(1), 0, 0, 0], [Fraction(1), 0, 1, 0], [Fraction(0), 0, 1, 0]]
        self.assertAlmostEqual(augment._euclidean_area(q_square), 1.0)
        self.assertAlmostEqual(augment._euclidean_area(qp_square), 1.0)
        symp = abs(float(sum(augment.omega(qp_square[i], qp_square[(i + 1) % 4]) for i in range(4))) / 2)
        self.assertAlmostEqual(symp, 1.0)
        self.assertAlmostEqual(symp - augment._euclidean_area(qp_square) * 1.0, 0.0)

    def test_diagonal_covariance_rho_fixture(self):
        variances = [1.0, 4.0, 9.0, 16.0]
        vertices = []
        for i, variance in enumerate(variances):
            for sign in (-1.0, 1.0):
                v = [Fraction(0)] * 4
                v[i] = Fraction(sign * 2.0 * variance**0.5)
                vertices.append(v)
        diag = augment._covariance(vertices, 8)
        self.assertEqual(diag["status"], "eligible")
        self.assertAlmostEqual(diag["nu1"], 3.0, places=10)
        self.assertAlmostEqual(diag["nu2"], 8.0, places=10)
        self.assertAlmostEqual(diag["rho"], 8.0 / 3.0, places=10)
        self.assertAlmostEqual(diag["condition"], 16.0, places=10)

    def test_covariance_condition_limit_is_ineligible(self):
        variances = [1.0, 1.0, 1.0, 2.0e11]
        vertices = []
        for i, variance in enumerate(variances):
            for sign in (-1.0, 1.0):
                v = [Fraction(0)] * 4
                v[i] = Fraction(sign * 2.0 * variance**0.5)
                vertices.append(v)
        self.assertEqual(augment._covariance(vertices, 8)["status"], "ordinary_condition_exceeds_limit")

    def test_face_order_failure_is_visible(self):
        incidence = [[True, True, True] for _ in range(4)]
        faces = augment._two_faces(incidence)
        self.assertEqual(len(faces), 3)
        self.assertIsNone(augment._ordered_face(faces[0], incidence))

    def test_forbidden_cycle_metadata_and_u2_perturbation_are_rejected(self):
        rows = copy.deepcopy(self.features)
        forbidden = next(row for row in rows if row["map_variant"] != "identity")
        forbidden["strict_cycle"] = {"strict_sign_cell": True}
        with self.assertRaisesRegex(augment.AnalysisError, "forbidden"):
            analyzer.validate(rows, require_complete=False)
        rows = copy.deepcopy(self.features)
        u2 = next(row for row in rows if row["map_variant"] == "u2-haar")
        u2["symplectic_ridge_area_mean"] += 1.0
        with self.assertRaisesRegex(augment.AnalysisError, "orientation u2-haar"):
            analyzer.validate(rows)

    def test_grid_and_target_guards(self):
        with self.assertRaisesRegex(augment.AnalysisError, "incomplete orientation"):
            analyzer.validate(self.features[:-1])
        row = copy.deepcopy(self.rows[0]); row["iteration"] = 2
        with self.assertRaisesRegex(augment.AnalysisError, "target field"):
            augment.feature_row(row, "orientation")

    def test_duplicate_and_truncated_inputs(self):
        duplicate = [copy.deepcopy(self.features[0]), copy.deepcopy(self.features[0])]
        with self.assertRaisesRegex(augment.AnalysisError, "duplicate"):
            analyzer.validate(duplicate, require_complete=False)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "truncated.jsonl"
            path.write_text(json.dumps(self.features[0]))
            with self.assertRaisesRegex(augment.AnalysisError, "no final newline"):
                augment.load_rows(path)

    def test_tangential_authoritative_replay_mismatch(self):
        source = copy.deepcopy(self.rows[0]); replay = copy.deepcopy(source)
        source.update({"schema": augment.TANGENTIAL_SCHEMA, "accepted": True, "validation_status": "survived", "law": "factorial-baseline", "pair_bucket": "3x3", "sample_id": "sample", "pairing_id": "pair", "volume": source["exact_volume_as_f64"]})
        replay.update(source)
        replay["factor_q_area"] = 999.0
        replay["geometry_dual_vertices_rational"] = source["transformed_dual_vertices_rational"]
        with self.assertRaisesRegex(augment.AnalysisError, "contract mismatch"):
            augment._join_tangential([source], [replay])

    def test_producer_ordinary_rows_omit_geometry_keys(self):
        binary = Path(__file__).parents[4] / "target/release/sys-datascience-alternative-generator-smoke"
        if not binary.exists():
            self.skipTest("release producer binary is unavailable")
        with tempfile.TemporaryDirectory() as directory:
            subprocess.run([str(binary), "--out-dir", directory, "--rows-per-law", "1", "--only-family", "factorial", "--identity-scope", "serialization-test"], check=True, stdout=subprocess.DEVNULL)
            row = json.loads((Path(directory) / "smoke-rows.jsonl").read_text().splitlines()[0])
            self.assertFalse(any(key.startswith("geometry_") for key in row))

    def test_incomplete_tangential_grid(self):
        feature = copy.deepcopy(self.features[0])
        feature.update({"source_kind": "tangential", "law": "factorial-baseline", "bucket": "3x3", "base_id": "pair", "source_pairing_id": "pair", "source_id": "sample"})
        with self.assertRaisesRegex(augment.AnalysisError, "incomplete tangential"):
            analyzer.validate([feature])


if __name__ == "__main__":
    unittest.main()
