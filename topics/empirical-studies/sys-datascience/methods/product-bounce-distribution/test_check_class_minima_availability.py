#!/usr/bin/env python3
"""Contract regressions for the retained A3-null availability checker."""
import importlib.util
import io
from contextlib import redirect_stderr
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("check_class_minima_availability.py")
SPEC = importlib.util.spec_from_file_location("availability_checker", MODULE_PATH)
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


def row(*, enumerated=2, solved=0, inadmissible=2, failures=0):
    return {
        "name": "fixture",
        "enumerated_sigma_counts_by_bounces": {"2": 0, "3": enumerated},
        "candidate_orbit_counts_by_bounces": {"2": 0, "3": solved},
        "f64_inadmissible_sigma_counts_by_bounces": {
            "2": 0,
            "3": inadmissible,
        },
        "f64_numerical_failure_counts_by_bounces": {"2": 0, "3": failures},
        "exact_admissible_f64_rejected_counts_by_bounces": {"2": 0, "3": 0},
    }


class AvailabilityContractTest(unittest.TestCase):
    def test_missing_advisory_input_does_not_block_semantic_check(self):
        with tempfile.TemporaryDirectory() as directory:
            missing = Path(directory) / "class-minima.jsonl"
            stderr = io.StringIO()
            with redirect_stderr(stderr):
                CHECKER.check_advisory_input_identity(missing, "unused")
            self.assertIn("byte identity was not", stderr.getvalue())

    def test_classifies_fully_accounted_solver_rejections(self):
        self.assertEqual(
            CHECKER.classify_a3_null_row(row()),
            "all_three_bounce_sigmas_f64_inadmissible",
        )

    def test_rejects_unaccounted_transition_filtered_sigma(self):
        with self.assertRaisesRegex(AssertionError, "incomplete f64 accounting"):
            CHECKER.classify_a3_null_row(row(enumerated=3))

    def test_does_not_call_numerical_failure_inadmissible(self):
        with self.assertRaisesRegex(AssertionError, "unexpected A3-null row"):
            CHECKER.classify_a3_null_row(
                row(enumerated=2, inadmissible=1, failures=1)
            )


if __name__ == "__main__":
    unittest.main()
