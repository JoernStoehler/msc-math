#!/usr/bin/env python3

import copy
import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("produce.py")
SPEC = importlib.util.spec_from_file_location("equivalence_produce", MODULE_PATH)
produce = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(produce)


class EquivalenceMatrixTests(unittest.TestCase):
    def documents(self):
        payloads = produce.artifact_payloads()
        return (
            json.loads(payloads["matrix.json"]),
            json.loads(payloads["witnesses.json"]),
        )

    def provenance_fixture(self):
        payloads = produce.artifact_payloads()
        revision = produce.run_git("rev-parse", "HEAD")
        tree = produce.run_git("rev-parse", "HEAD^{tree}")
        return produce.provenance_document(payloads, "fixture command", revision, tree)

    def test_every_row_has_a_passing_deterministic_witness(self):
        matrix_rows = produce.rows()
        witnesses = produce.witness_results()
        produce.validate_matrix(matrix_rows, witnesses)
        self.assertEqual(len(matrix_rows), 16)
        self.assertEqual(len(witnesses), 16)

    def test_required_levels_and_negative_controls_are_present(self):
        matrix_rows = produce.rows()
        levels = {row["level"] for row in matrix_rows}
        self.assertEqual(levels, produce.LEVELS - {"full_law"})
        self.assertGreaterEqual(sum(row["level"] == "not_equivalent" for row in matrix_rows), 5)

    def test_marginal_is_not_promoted_to_full_law(self):
        by_id = {row["row_id"]: row for row in produce.rows()}
        self.assertEqual(by_id["angles-iid-dirichlet1-marginal"]["level"], "component_marginal")
        self.assertEqual(by_id["iid-support-baseline-vs-equal-support-dirichlet1"]["level"], "not_equivalent")
        self.assertEqual(by_id["iid-support-baseline-vs-equal-support-dirichlet1"]["expected"]["law_parameters"], "nonzero")

    def test_anti_symplectic_theorem_and_executable_boundaries_are_explicit(self):
        row = next(row for row in produce.rows() if row["row_id"] == "antiunitary-antisymplectic-endpoint")
        self.assertIn("ehz_and_sys_invariance_theorem_proved", row["proof_status"])
        self.assertIn("no_dedicated_capacity_regression", row["proof_status"])
        self.assertNotIn("proof_pending", row["proof_status"])
        self.assertIn("exact or certified", row["collapse_scope"])
        self.assertIn("near-floating matrices remain non-theorem", row["collapse_scope"])
        self.assertIn("retain signed omega features", row["collapse_scope"])
        self.assertIn("reversed directed/facet-word semantics", row["collapse_scope"])
        self.assertIn("c_EHZ/sys were not evaluated", row["executable_control"])
        required_sources = {
            "papers/hk2017/EHZ-polytopes.tex",
            "thesis/02-preliminaries-ehz-capacity.tex",
            "formal/hk2017-qp-core.tex",
        }
        self.assertTrue(required_sources <= set(produce.SOURCE_PATHS))
        for source in required_sources:
            self.assertTrue(any(citation.startswith(source) for citation in row["proof_source"]))
        witness = next(witness for witness in produce.witness_results() if witness["row_id"] == row["row_id"])
        self.assertEqual(witness["evidence"]["ehz_sys_status"], "analytic_theorem_source_backed_no_dedicated_capacity_regression")
        self.assertEqual(row["expected"]["signed_symplectic_features"], "nonzero")
        self.assertEqual(row["expected"]["absolute_symplectic_features"], "zero")

    def test_payload_generation_is_byte_deterministic(self):
        self.assertEqual(produce.artifact_payloads(), produce.artifact_payloads())

    def test_every_view_has_an_explicit_comparison_semantics(self):
        self.assertEqual(set(produce.VIEWS), set(produce.VIEW_DEFINITIONS))

    def test_optimized_python_is_rejected_fail_closed(self):
        result = subprocess.run(
            [sys.executable, "-O", str(MODULE_PATH), "--check"],
            text=True,
            capture_output=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("optimized Python disables required equivalence checks", result.stderr)

    def test_matrix_rejects_incomplete_or_extra_row_schema_and_empty_collapse(self):
        matrix, witnesses = self.documents()
        mutations = []
        missing = copy.deepcopy(matrix)
        missing["rows"][0].pop("arithmetic")
        mutations.append(missing)
        extra = copy.deepcopy(matrix)
        extra["rows"][0]["junk"] = "unexpected"
        mutations.append(extra)
        empty_collapse = copy.deepcopy(matrix)
        empty_collapse["rows"][0]["collapse_scope"] = ""
        mutations.append(empty_collapse)
        missing_view = copy.deepcopy(matrix)
        missing_view["rows"][0]["expected"].pop(produce.VIEWS[0])
        mutations.append(missing_view)
        false_complete = copy.deepcopy(matrix)
        false_complete["complete"] = False
        mutations.append(false_complete)
        extra_top_level = copy.deepcopy(matrix)
        extra_top_level["junk"] = True
        mutations.append(extra_top_level)
        for index, malformed in enumerate(mutations):
            with self.subTest(malformed=index):
                with self.assertRaises(produce.PacketValidationError):
                    produce.validate_matrix_documents(malformed, witnesses)

        malformed_witnesses = copy.deepcopy(witnesses)
        malformed_witnesses["complete"] = False
        with self.assertRaises(produce.PacketValidationError):
            produce.validate_matrix_documents(matrix, malformed_witnesses)

    def test_provenance_rejects_false_complete_and_source_or_artifact_path_drift(self):
        valid = self.provenance_fixture()
        produce.validate_provenance_schema(valid)
        mutations = []
        false_complete = copy.deepcopy(valid)
        false_complete["complete"] = False
        mutations.append(false_complete)
        empty_sources = copy.deepcopy(valid)
        empty_sources["source_inputs"] = []
        mutations.append(empty_sources)
        extra_source = copy.deepcopy(valid)
        extra_source["source_inputs"].append(copy.deepcopy(extra_source["source_inputs"][0]))
        mutations.append(extra_source)
        omitted_artifact = copy.deepcopy(valid)
        omitted_artifact["artifacts"].pop("matrix.tsv")
        mutations.append(omitted_artifact)
        extra_artifact = copy.deepcopy(valid)
        extra_artifact["artifacts"]["junk.json"] = {"sha256": "0" * 64, "bytes": 0}
        mutations.append(extra_artifact)
        for index, malformed in enumerate(mutations):
            with self.subTest(malformed=index):
                with self.assertRaises(produce.PacketValidationError):
                    produce.validate_provenance_schema(malformed)

    def test_provenance_rejects_wrong_revision_tree_and_byte_count(self):
        valid = self.provenance_fixture()
        with self.assertRaises(produce.PacketValidationError):
            produce.validate_revision_tree(valid["source_revision"], "0" * 40)
        data = b"exact bytes"
        wrong = {"sha256": produce.sha256(data), "bytes": len(data) + 1}
        with self.assertRaises(produce.PacketValidationError):
            produce.validate_byte_record(wrong, data, "malformed bytes")
        wrong_hash = {"sha256": produce.sha256(b"wrong bytes"), "bytes": len(data)}
        with self.assertRaises(produce.PacketValidationError):
            produce.validate_byte_record(wrong_hash, data, "wrong byte content")

    def test_output_tree_rejects_junk_and_missing_paths(self):
        with tempfile.TemporaryDirectory() as raw:
            out = Path(raw)
            for name in produce.OUTPUT_PATHS:
                (out / name).write_bytes(b"")
            produce.validate_output_tree(out)
            (out / "junk.txt").write_text("junk")
            with self.assertRaises(produce.PacketValidationError):
                produce.validate_output_tree(out)
            (out / "junk.txt").unlink()
            (out / "matrix.tsv").unlink()
            with self.assertRaises(produce.PacketValidationError):
                produce.validate_output_tree(out)


if __name__ == "__main__":
    unittest.main()
