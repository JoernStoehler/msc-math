#!/usr/bin/env python3
import copy
import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("augment", HERE / "augment.py")
augment = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(augment)
sys.modules["augment"] = augment


ORIENTATION = Path(__file__).parents[2] / "methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl"


class ExactFeaturePacketTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.rows = augment.load_rows(ORIENTATION)
        cls.features = [augment.feature_row(row, "orientation") for row in cls.rows]

    def test_exact_join_and_strict_cycle_scope(self):
        self.assertEqual(len(self.features), 40)
        cycles = [r for r in self.features if r["strict_cycle"] is not None]
        self.assertEqual(len(cycles), 2)
        self.assertTrue(all(r["source_id"].endswith("map=identity") for r in cycles))
        self.assertTrue(all(r["strict_cycle"]["strict_sign_cell"] for r in cycles))
        self.assertTrue(all(r["strict_cycle"]["strict_cycle_feasible"] for r in cycles))
        self.assertTrue(all(r["geometry_validation_status"] == "validated" for r in self.features))

    def test_forbidden_cycle_metadata_is_rejected(self):
        rows = copy.deepcopy(self.features)
        forbidden = next(row for row in rows if not row["source_id"].endswith("map=identity"))
        forbidden["strict_cycle"] = {"strict_sign_cell": True, "strict_cycle_feasible": True}
        with self.assertRaisesRegex(augment.AnalysisError, "forbidden row"):
            # Importing the analyzer this way keeps this test runnable without
            # installing the packet as a Python package.
            spec = importlib.util.spec_from_file_location("analyze", HERE / "analyze.py")
            analyzer = importlib.util.module_from_spec(spec)
            assert spec and spec.loader
            spec.loader.exec_module(analyzer)
            analyzer.validate(rows, require_complete=False)

    def test_wrong_primal_join_is_rejected(self):
        row = copy.deepcopy(self.rows[0])
        row["reconstructed_primal_vertices_rational"][0][0] = "0/1"
        with self.assertRaisesRegex(augment.AnalysisError, "exact primal/dual join"):
            augment.feature_row(row, "orientation")

    def test_incomplete_variant_grid_is_rejected(self):
        spec = importlib.util.spec_from_file_location("analyze", HERE / "analyze.py")
        analyzer = importlib.util.module_from_spec(spec)
        assert spec and spec.loader
        spec.loader.exec_module(analyzer)
        with self.assertRaisesRegex(augment.AnalysisError, "incomplete orientation"):
            analyzer.validate(self.features[:-1])

    def test_target_field_is_rejected(self):
        row = copy.deepcopy(self.rows[0])
        row["sys"] = 0.25
        with self.assertRaisesRegex(augment.AnalysisError, "target field"):
            augment.feature_row(row, "orientation")

    def test_duplicate_and_truncated_feature_inputs_are_rejected(self):
        spec = importlib.util.spec_from_file_location("analyze", HERE / "analyze.py")
        analyzer = importlib.util.module_from_spec(spec)
        assert spec and spec.loader
        spec.loader.exec_module(analyzer)
        duplicate = [copy.deepcopy(self.features[0]), copy.deepcopy(self.features[0])]
        with self.assertRaisesRegex(augment.AnalysisError, "duplicate"):
            analyzer.validate(duplicate, require_complete=False)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "truncated.jsonl"
            path.write_text(json.dumps(self.features[0]))
            with self.assertRaisesRegex(augment.AnalysisError, "no final newline"):
                augment.load_rows(path)

    def test_tangential_sidecar_identity_join_is_checked(self):
        source = copy.deepcopy(self.rows[0])
        source.update({
            "schema": augment.TANGENTIAL_SCHEMA, "accepted": True, "validation_status": "survived",
            "law": "factorial-baseline", "pair_bucket": "3x3", "sample_id": "tangential-sample",
            "pairing_id": "tangential-pair", "volume": source["exact_volume_as_f64"],
            "geometry_dual_vertices_rational": source["transformed_dual_vertices_rational"],
            "geometry_primal_vertices_rational": source["reconstructed_primal_vertices_rational"],
            "geometry_vertex_facet_incidence": [[i in facets for i in range(len(source["transformed_dual_vertices_rational"]))] for facets in source["labeled_incidence_signature"]],
            "geometry_volume": source["exact_volume_as_f64"], "geometry_source_sample_id": "wrong-sample",
            "geometry_source_pairing_id": "tangential-pair",
        })
        with self.assertRaisesRegex(augment.AnalysisError, "sidecar identity"):
            augment.feature_row(source, "tangential")


if __name__ == "__main__":
    unittest.main()
