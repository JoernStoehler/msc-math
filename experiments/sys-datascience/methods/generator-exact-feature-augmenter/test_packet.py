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
            orientation_path=root / "orientation" / "rows.jsonl"
            orientation_path.parent.mkdir()
            orientation_path.write_text("".join(json.dumps(row, sort_keys=True) + "\n" for row in cls.rows[:5]))
            for mode in ("source", "replay"):
                command=[str(binary), "--out-dir", str(root/mode), "--rows-per-law", "1", "--only-family", "factorial", "--identity-scope", "test-disposable"]
                if mode == "replay": command.append("--geometry-sidecar")
                subprocess.run(command, check=True, stdout=subprocess.DEVNULL)
            src,_=augment.load_rows(root/"source"/"smoke-rows.jsonl"); replay,_=augment.load_rows(root/"replay"/"smoke-rows.jsonl")
            cls.input_evidence = {
                "orientation": orientation_path,
                "tangential-source": root / "source" / "smoke-rows.jsonl",
                "tangential-replay": root / "replay" / "smoke-rows.jsonl",
            }
            cls.disposable_features = cls.features[:5] + [augment.feature_row(s,"tangential",g) for s,g in augment._join_tangential(src,replay)]
        else:
            cls.input_evidence = None
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

        rows = copy.deepcopy(self.features)
        rows[0]["vertex_covariance"]["nu1"] = None
        with self.assertRaisesRegex(augment.AnalysisError, "required covariance feature nu1"):
            analyzer.validate(rows, require_complete=False)

        rows = copy.deepcopy(self.features)
        allowed = next(row for row in rows if row["strict_cycle"] is not None)
        allowed["strict_cycle"]["strict_cycle_count"] = True
        with self.assertRaisesRegex(augment.AnalysisError, "strict-cycle metadata malformed"):
            analyzer.validate(rows, require_complete=False)

        rows = copy.deepcopy(self.features)
        allowed = next(row for row in rows if row["strict_cycle"] is not None)
        allowed["strict_cycle"]["strict_cycle_count"] += 1
        with self.assertRaisesRegex(augment.AnalysisError, "strict-cycle metadata inconsistent"):
            analyzer.validate(rows, require_complete=False)

    def test_altered_feature_manifest_and_revision_hash_guards(self):
        if self.input_evidence is None:
            self.skipTest("release producer binary is unavailable")
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory); feature_path=directory/"features.jsonl"; manifest_path=directory/"augment-report.json"
            payload=(json.dumps(self.features[0])+"\n").encode(); feature_path.write_bytes(payload)
            revision=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()
            inputs=[]
            for role,path in self.input_evidence.items():
                payload_input=path.read_bytes()
                schema="generator-orientation-smoke-row-v2" if role=="orientation" else "alternative-generator-smoke-row-v2"
                inputs.append({"role":role,"path":str(path),"sha256":hashlib.sha256(payload_input).hexdigest(),"rows":len(payload_input.splitlines()),"schema":schema})
            manifest={"schema":"generator-exact-feature-augmenter-report-v2","rows":1,"feature_output":{"path":str(feature_path),"sha256":hashlib.sha256(payload).hexdigest(),"rows":1,"schema":analyzer.FEATURE_SCHEMA},"provenance":{"source_revision":revision,"source_dirty":False,"inputs":inputs}}
            manifest_path.write_text(json.dumps(manifest))
            analyzer.verify_manifest(feature_path,manifest_path,expected_revision=revision,allow_external_disposable=True)
            feature_path.write_text(json.dumps(self.features[1])+"\n")
            with self.assertRaisesRegex(augment.AnalysisError,"feature output"):
                analyzer.verify_manifest(feature_path,manifest_path,expected_revision=revision,allow_external_disposable=True)
            feature_path.write_bytes(payload)
            with self.assertRaisesRegex(augment.AnalysisError,"revision mismatch"):
                analyzer.verify_manifest(feature_path,manifest_path,expected_revision="0"*40,allow_external_disposable=True)
        rows = copy.deepcopy(self.features)
        rows[0]["vertex_covariance"]["status"] = "mystery"
        with self.assertRaisesRegex(augment.AnalysisError, "must be eligible"):
            analyzer.validate(rows, require_complete=False)

    def test_manifest_input_evidence_is_explicit_and_fail_closed(self):
        if self.input_evidence is None:
            self.skipTest("release producer binary is unavailable")
        def records():
            out=[]
            for role,path in self.input_evidence.items():
                payload=path.read_bytes()
                schema="generator-orientation-smoke-row-v2" if role=="orientation" else "alternative-generator-smoke-row-v2"
                out.append({"role":role,"path":str(path),"sha256":hashlib.sha256(payload).hexdigest(),"rows":len(payload.splitlines()),"schema":schema})
            return out
        with self.assertRaisesRegex(augment.AnalysisError, "exactly three"):
            analyzer._classify_inputs([], "disposable")
        valid=records()
        missing_role=copy.deepcopy(valid); missing_role[0].pop("role")
        with self.assertRaisesRegex(augment.AnalysisError, "invalid role"):
            analyzer._classify_inputs(missing_role, "disposable")
        duplicate_role=copy.deepcopy(valid); duplicate_role[1]["role"]="orientation"
        with self.assertRaisesRegex(augment.AnalysisError, "duplicate manifest input role"):
            analyzer._classify_inputs(duplicate_role, "disposable")
        wrong_role=copy.deepcopy(valid); wrong_role[0]["role"]="tangential-source"
        with self.assertRaisesRegex(augment.AnalysisError, "schema does not match role"):
            analyzer._classify_inputs(wrong_role, "disposable")
        wrong_path=copy.deepcopy(valid); wrong_path[0]["path"]=str(Path(self.smoke_dir.name) / "does-not-exist.jsonl")
        with self.assertRaisesRegex(augment.AnalysisError, "cannot load manifest input"):
            analyzer._classify_inputs(wrong_path, "disposable")
        with tempfile.TemporaryDirectory() as directory:
            short=Path(directory) / "orientation.jsonl"
            short.write_text("".join(json.dumps(row, sort_keys=True) + "\n" for row in self.rows[:4]))
            wrong_count=copy.deepcopy(valid); wrong_count[0]["path"]=str(short); wrong_count[0]["sha256"]=hashlib.sha256(short.read_bytes()).hexdigest(); wrong_count[0]["rows"]=4
            with self.assertRaisesRegex(augment.AnalysisError, "expected 5 rows"):
                analyzer._classify_inputs(wrong_count, "disposable")

    def test_retained_producer_at_head_is_a_valid_ancestor(self):
        feature_path=HERE / "artifacts/full-panels/features.jsonl"
        orientation_path=Path(__file__).parents[2] / "methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"
        source_path=Path(__file__).parents[2] / "methods/generator-tangential-matchability/artifacts/full-64/smoke-rows.jsonl"
        replay_path=HERE / "artifacts/full-panels/tangential-replay/smoke-rows.jsonl"
        revision=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()
        inputs=[]
        for role,path in (("orientation",orientation_path),("tangential-source",source_path),("tangential-replay",replay_path)):
            payload=path.read_bytes(); schema="generator-orientation-smoke-row-v2" if role=="orientation" else "alternative-generator-smoke-row-v2"
            inputs.append({"role":role,"path":str(path),"sha256":hashlib.sha256(payload).hexdigest(),"rows":len(payload.splitlines()),"schema":schema})
        feature_payload=feature_path.read_bytes(); manifest={"schema":"generator-exact-feature-augmenter-report-v2","rows":len(feature_payload.splitlines()),"feature_output":{"path":str(feature_path),"sha256":hashlib.sha256(feature_payload).hexdigest(),"rows":len(feature_payload.splitlines()),"schema":analyzer.FEATURE_SCHEMA},"provenance":{"source_revision":revision,"source_dirty":False,"inputs":inputs}}
        with tempfile.TemporaryDirectory(dir=analyzer.REPO_ROOT) as directory:
            report_path=Path(directory) / "augment-report.json"; report_path.write_text(json.dumps(manifest))
            _,_,_,audit=analyzer.verify_manifest(feature_path,report_path,expected_revision=revision,design="retained")
            self.assertEqual(audit["producer_revision"],revision)
            self.assertEqual(set(audit["retained_inputs"]),{"orientation","tangential-source","tangential-replay"})

    def test_retained_producer_survives_a_descendant_test_commit(self):
        feature_path=HERE / "artifacts/full-panels/features.jsonl"
        paths=(
            ("orientation",Path(__file__).parents[2] / "methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"),
            ("tangential-source",Path(__file__).parents[2] / "methods/generator-tangential-matchability/artifacts/full-64/smoke-rows.jsonl"),
            ("tangential-replay",HERE / "artifacts/full-panels/tangential-replay/smoke-rows.jsonl"),
        )
        producer=subprocess.check_output(["git","rev-parse","HEAD~1"],text=True).strip()
        current=subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()
        inputs=[]
        for role,path in paths:
            payload=path.read_bytes(); schema="generator-orientation-smoke-row-v2" if role=="orientation" else "alternative-generator-smoke-row-v2"
            inputs.append({"role":role,"path":str(path),"sha256":hashlib.sha256(payload).hexdigest(),"rows":len(payload.splitlines()),"schema":schema})
        feature_payload=feature_path.read_bytes(); manifest={"schema":"generator-exact-feature-augmenter-report-v2","rows":len(feature_payload.splitlines()),"feature_output":{"path":str(feature_path),"sha256":hashlib.sha256(feature_payload).hexdigest(),"rows":len(feature_payload.splitlines()),"schema":analyzer.FEATURE_SCHEMA},"provenance":{"source_revision":producer,"source_dirty":False,"inputs":inputs}}
        with tempfile.TemporaryDirectory(dir=analyzer.REPO_ROOT) as directory:
            report_path=Path(directory) / "augment-report.json"; report_path.write_text(json.dumps(manifest))
            _,_,_,audit=analyzer.verify_manifest(feature_path,report_path,expected_revision=current,design="retained")
            self.assertEqual(audit["producer_revision"],producer)

    def test_retained_producer_code_change_is_rejected(self):
        history=subprocess.check_output(["git","log","--format=%H","--",str(HERE / "augment.py")],text=True).splitlines()
        self.assertGreaterEqual(len(history),2)
        self.assertFalse(analyzer._git_diff_clean(history[-1], []))

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
