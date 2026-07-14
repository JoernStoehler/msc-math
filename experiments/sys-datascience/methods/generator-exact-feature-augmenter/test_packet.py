#!/usr/bin/env python3
import copy
import importlib.util
import json
import sys
import subprocess
import hashlib
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
        cls.smoke_dir = tempfile.TemporaryDirectory()
        root = Path(cls.smoke_dir.name)
        binary = Path(__file__).parents[4] / "target/release/sys-datascience-alternative-generator-smoke"
        if binary.exists():
            for mode in ("source", "replay"):
                command=[str(binary), "--out-dir", str(root/mode), "--rows-per-law", "1", "--only-family", "factorial", "--identity-scope", "test-disposable"]
                if mode == "replay": command.append("--geometry-sidecar")
                subprocess.run(command, check=True, stdout=subprocess.DEVNULL)
            src,_=augment.load_rows(root/"source"/"smoke-rows.jsonl"); replay,_=augment.load_rows(root/"replay"/"smoke-rows.jsonl")
            cls.disposable_features = cls.features[:5] + [augment.feature_row(s,"tangential",g) for s,g in augment._join_tangential(src,replay)]
        else:
            cls.disposable_features = None

    @classmethod
    def tearDownClass(cls):
        cls.smoke_dir.cleanup()

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

        rows = copy.deepcopy(self.features)
        allowed = next(row for row in rows if row["map_variant"] == "identity" and row["bucket"] == "3x3")
        allowed["strict_cycle"]["strict_cycle_count"] = -1
        with self.assertRaisesRegex(augment.AnalysisError, "strict-cycle"):
            analyzer.validate(rows, require_complete=False)

        rows = copy.deepcopy(self.features)
        allowed = next(row for row in rows if row["map_variant"] == "identity" and row["bucket"] == "3x3")
        allowed["strict_cycle"]["strict_signs"][0][0] = 2
        with self.assertRaisesRegex(augment.AnalysisError, "strict-cycle"):
            analyzer.validate(rows, require_complete=False)

        rows = copy.deepcopy(self.features)
        so4 = next(row for row in rows if row["map_variant"] == "so4-haar")
        so4["vertex_covariance"]["status"] = "ordinary_condition_exceeds_limit"
        with self.assertRaisesRegex(augment.AnalysisError, "covariance"):
            analyzer.validate(rows)

    def test_grid_and_target_guards(self):
        with self.assertRaisesRegex(augment.AnalysisError, "incomplete orientation"):
            analyzer.validate(self.features[:-1])
        row = copy.deepcopy(self.rows[0]); row["iteration"] = 2
        with self.assertRaisesRegex(augment.AnalysisError, "target field"):
            augment.feature_row(row, "orientation")
        with self.assertRaisesRegex(augment.AnalysisError, "orientation bucket set"):
            analyzer.validate_design(self.features, "disposable")

        if self.disposable_features is not None:
            with self.assertRaisesRegex(augment.AnalysisError, "tangential bucket set"):
                analyzer.validate_design(self.disposable_features[:5], "disposable")
            missing_pair=self.disposable_features[:-4]
            with self.assertRaisesRegex(augment.AnalysisError, "incomplete tangential|expected 1 tangential pairs"):
                analyzer.validate_design(missing_pair, "disposable")
            duplicate=copy.deepcopy(self.disposable_features); duplicate.append(copy.deepcopy(duplicate[0])); duplicate[-1]["source_id"]="new-source-id"
            with self.assertRaisesRegex(augment.AnalysisError, "exactly 17|duplicate orientation"):
                analyzer.validate_design(duplicate, "disposable")

    def test_required_feature_and_covariance_fail_closed(self):
        rows = copy.deepcopy(self.features)
        rows[0]["euclidean_ridge_area_mean"] = None
        with self.assertRaisesRegex(augment.AnalysisError, "required feature"):
            analyzer.validate(rows, require_complete=False)

    def test_altered_feature_manifest_and_revision_hash_guards(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory); feature_path=directory/"features.jsonl"; manifest_path=directory/"augment-report.json"
            payload=(json.dumps(self.features[0])+"\n").encode(); feature_path.write_bytes(payload)
            revision=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()
            manifest={"schema":"generator-exact-feature-augmenter-report-v2","feature_output":{"path":str(feature_path),"sha256":hashlib.sha256(payload).hexdigest(),"rows":1,"schema":analyzer.FEATURE_SCHEMA},"provenance":{"source_revision":revision,"source_dirty":False,"inputs":[]}}
            manifest_path.write_text(json.dumps(manifest))
            analyzer.verify_manifest(feature_path,manifest_path,expected_revision=revision)
            feature_path.write_text(json.dumps(self.features[1])+"\n")
            with self.assertRaisesRegex(augment.AnalysisError,"feature output"):
                analyzer.verify_manifest(feature_path,manifest_path,expected_revision=revision)
            feature_path.write_bytes(payload)
            with self.assertRaisesRegex(augment.AnalysisError,"revision mismatch"):
                analyzer.verify_manifest(feature_path,manifest_path,expected_revision="0"*40)
        rows = copy.deepcopy(self.features)
        rows[0]["vertex_covariance"]["status"] = "mystery"
        with self.assertRaisesRegex(augment.AnalysisError, "must be eligible"):
            analyzer.validate(rows, require_complete=False)

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

    def test_replay_incidence_and_simplex_substitution_are_rejected(self):
        source = copy.deepcopy(self.rows[0])
        source.update({"schema": augment.TANGENTIAL_SCHEMA, "accepted": True, "validation_status": "survived", "law": "factorial-baseline", "pair_bucket": "3x3", "sample_id": "sample", "pairing_id": "pair", "volume": source["exact_volume_as_f64"]})
        incidence = [[i in facets for i in range(len(source["transformed_dual_vertices_rational"]))] for facets in source["labeled_incidence_signature"]]
        geometry = {"schema": augment.TANGENTIAL_SCHEMA, "geometry_dual_vertices_rational": source["transformed_dual_vertices_rational"], "geometry_primal_vertices_rational": source["reconstructed_primal_vertices_rational"], "geometry_vertex_facet_incidence": incidence, "geometry_source_sample_id": "sample", "geometry_source_pairing_id": "pair", "geometry_volume": source["volume"]}
        geometry["geometry_vertex_facet_incidence"][0][0] = 1
        with self.assertRaisesRegex(augment.AnalysisError, "literal bool"):
            augment.feature_row(source, "tangential", geometry)
        geometry["geometry_vertex_facet_incidence"][0][0] = True
        geometry["geometry_primal_vertices_rational"] = geometry["geometry_primal_vertices_rational"][:5]
        with self.assertRaisesRegex(augment.AnalysisError, "incidence dimensions|vertex count"):
            augment.feature_row(source, "tangential", geometry)

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
