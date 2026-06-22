import unittest
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import compare


class F64CapacityCompareTest(unittest.TestCase):
    def test_limited_claim_passes_when_expectations_match(self):
        cases = [
            {
                "case_id": "product_tie",
                "source_id": "row-1",
                "reason": "expected product tie",
                "claim_scope": "limited",
                "expect": {
                    "output_epistemics.minimizing_sigma_set_status": "undecided",
                    "near_minimizing_sigma_count": {"min": 2},
                    "output_epistemics.reasons": {"contains": "tie"},
                },
            }
        ]
        scan_rows = {
            "row-1": {
                "source_id": "row-1",
                "near_minimizing_sigma_count": 4,
                "output_epistemics": {
                    "minimizing_sigma_set_status": "undecided",
                    "reasons": ["tie"],
                },
            }
        }

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "met")
        self.assertEqual(rows[0]["claim_scope"], "limited")
        self.assertEqual(rows[0]["failures"], [])

    def test_unexpected_value_fails(self):
        cases = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "full",
                "expect": {
                    "output_epistemics.minimizing_sigma_set_status": "decided",
                    "near_minimizing_sigma_count": {"equals": 1},
                },
            }
        ]
        scan_rows = {
            "row-1": {
                "source_id": "row-1",
                "near_minimizing_sigma_count": 4,
                "output_epistemics": {
                    "minimizing_sigma_set_status": "undecided",
                },
            }
        }

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "failed")
        self.assertEqual(len(rows[0]["failures"]), 2)

    def test_unexpected_scan_row_fails(self):
        cases = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "full",
                "expect": {"source_id": "row-1"},
            }
        ]
        scan_rows = {
            "row-1": {"source_id": "row-1", "output_epistemics": {}},
            "row-2": {"source_id": "row-2", "output_epistemics": {}},
        }

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "met")
        self.assertEqual(rows[1]["expectation_status"], "failed")
        self.assertEqual(rows[1]["failures"], ["unexpected scan row"])

    def test_unknown_operator_fails(self):
        cases = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "full",
                "expect": {"near_minimizing_sigma_count": {"equal": 1}},
            }
        ]
        scan_rows = {
            "row-1": {
                "source_id": "row-1",
                "near_minimizing_sigma_count": 1,
                "output_epistemics": {},
            },
        }

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "failed")
        self.assertIn("unknown expectation operator", rows[0]["failures"][0])

    def test_missing_field_fails_even_when_expected_value_is_null(self):
        cases = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "full",
                "expect": {"present_null": None, "missing_null": None},
            }
        ]
        scan_rows = {"row-1": {"source_id": "row-1", "present_null": None}}

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "failed")
        self.assertEqual(rows[0]["failures"], ["missing_null: missing field"])

    def test_invalid_claim_scope_fails(self):
        cases = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "global",
                "expect": {"source_id": "row-1"},
            }
        ]
        scan_rows = {"row-1": {"source_id": "row-1", "output_epistemics": {}}}

        rows = compare.compare_cases(cases, scan_rows)

        self.assertEqual(rows[0]["expectation_status"], "failed")
        self.assertIn("invalid claim_scope", rows[0]["failures"][0])

    def test_write_outputs_writes_comparison_jsonl(self):
        rows = [
            {
                "case_id": "clean",
                "source_id": "row-1",
                "reason": "clean row",
                "claim_scope": "full",
                "expectation_status": "met",
                "failures": [],
                "observed": {
                    "capacity_value_status": "decided",
                    "capacity_label_status": "label_agrees",
                    "minimizing_sigma_set_status": "decided",
                    "near_minimizing_sigma_count": 1,
                    "fallback_recommended": False,
                    "rel_action_error": 0.0,
                    "support_kind": "stored_label",
                    "exact_audit_status": "not_requested",
                },
            }
        ]
        with self.subTest("write output"):
            import tempfile

            with tempfile.TemporaryDirectory() as tmp:
                out_dir = Path(tmp)
                rows_path = compare.write_outputs(out_dir, rows)
                written = rows_path.read_text(encoding="utf-8").strip()

        self.assertIn('"expectation_status": "met"', written)
        self.assertIn('"support_kind": "stored_label"', written)


if __name__ == "__main__":
    unittest.main()
