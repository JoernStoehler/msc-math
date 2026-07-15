#!/usr/bin/env python3

import importlib.util
import subprocess
import sys
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("produce.py")
SPEC = importlib.util.spec_from_file_location("equivalence_produce", MODULE_PATH)
produce = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(produce)


class EquivalenceMatrixTests(unittest.TestCase):
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

    def test_anti_symplectic_capacity_status_remains_pending(self):
        row = next(row for row in produce.rows() if row["row_id"] == "antiunitary-antisymplectic-endpoint")
        self.assertIn("proof_pending", row["proof_status"])
        self.assertIn("do not collapse", row["collapse_scope"])
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


if __name__ == "__main__":
    unittest.main()
