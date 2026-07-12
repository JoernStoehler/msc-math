import unittest
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import scan_to_events


class ScanToEventsTest(unittest.TestCase):
    def test_capacity_observation_uses_fresh_reference_route_label(self):
        case = {
            "case_id": "generated",
            "source_id": "row-1",
        }
        row = {
            "source_id": "row-1",
            "family": "generated_random_f64",
            "f64_sigma": [0, 1],
            "f64_capacity": 2.0,
            "audit_capacity_label": 2.25,
            "exact_audit_status": "reference_route_capacity_success",
        }

        events = scan_to_events.events_for_case(case, row)
        capacity = next(row for row in events if row.get("variable") == "capacity")

        self.assertEqual(
            capacity["input_pair_kind"],
            "f64_input_with_fresh_reference_route_label",
        )
        self.assertEqual(capacity["exact_geometry_validation_status"], "accepted")
        self.assertEqual(capacity["f64"], 2.0)
        self.assertEqual(
            capacity["comparison_label_kind"],
            "fresh_reference_route_capacity_label",
        )
        self.assertEqual(capacity["comparison_label_f64"], 2.25)
        self.assertAlmostEqual(capacity["comparison_label_abs_difference"], 0.25)
        self.assertNotIn("oracle_kind", capacity)
        self.assertNotIn("exact", capacity)
        self.assertNotIn("abs_error", capacity)

    def test_retained_label_is_context_label_not_oracle(self):
        context = scan_to_events.context_fields(
            {"case_id": "retained", "source_id": "row-1"},
            {
                "source_id": "row-1",
                "family": "random",
                "f64_sigma": [0, 1],
                "audit_capacity_label": 3.0,
                "exact_audit_status": "not_requested",
            },
        )

        self.assertEqual(context["input_pair_kind"], "f64_input_with_stored_capacity_label")
        self.assertEqual(context["exact_geometry_validation_status"], "not_requested")
        events = scan_to_events.events_for_case(
            {"case_id": "retained", "source_id": "row-1"},
            {
                "source_id": "row-1",
                "family": "random",
                "f64_sigma": [0, 1],
                "f64_capacity": 2.0,
                "audit_capacity_label": 3.0,
                "exact_audit_status": "not_requested",
            },
        )
        capacity = next(row for row in events if row.get("variable") == "capacity")
        self.assertNotIn("oracle_kind", capacity)
        self.assertNotIn("abs_error", capacity)
        self.assertEqual(capacity["comparison_label_kind"], "stored_artifact_label")
        self.assertEqual(capacity["comparison_label_f64"], 3.0)
        self.assertEqual(capacity["comparison_label_abs_difference"], 1.0)

    def test_missing_manifest_row_emits_missing_context(self):
        events = scan_to_events.build_events(
            [{"case_id": "missing", "source_id": "absent"}],
            {},
        )

        self.assertTrue(
            any(row.get("status") == "missing_scan_row" for row in events),
            events,
        )
        self.assertTrue(scan_to_events.has_missing_scan_rows(events))
        run_finished = next(row for row in events if row.get("event") == "run_finished")
        self.assertEqual(run_finished["status"], "incomplete")
        self.assertEqual(run_finished["missing_contexts"], 1)

    def test_emits_indeterminacy_and_preprocessing_diagnostics(self):
        events = scan_to_events.events_for_case(
            {"case_id": "edge", "source_id": "row-1"},
            {
                "source_id": "row-1",
                "family": "edge_product",
                "f64_sigma": [],
                "f64_capacity": None,
                "audit_capacity_label": None,
                "exact_audit_status": "not_requested",
                "vertex_indeterminate_count": 2,
                "omega_indeterminate_count": 3,
                "facet_intersection_indeterminate_count": 4,
                "indeterminate_f64_count": 5,
                "removed_facet_count": 1,
                "near_redundant_facet_removal_delta_bound": 1e-8,
                "capacity_ratio_upper_bound": 1.0001,
            },
        )

        by_variable = {
            row["variable"]: row
            for row in events
            if row.get("event") == "observation"
        }

        self.assertEqual(by_variable["capacity"]["status"], "missing_f64_capacity")
        self.assertEqual(by_variable["vertex_indeterminate_count"]["f64"], 2.0)
        self.assertEqual(by_variable["omega_indeterminate_count"]["f64"], 3.0)
        self.assertEqual(by_variable["facet_intersection_indeterminate_count"]["f64"], 4.0)
        self.assertEqual(by_variable["indeterminate_f64_count"]["f64"], 5.0)
        self.assertEqual(by_variable["removed_facet_count"]["f64"], 1.0)
        self.assertAlmostEqual(
            by_variable["near_redundant_facet_removal_delta_bound"]["f64"],
            1e-8,
        )
        self.assertAlmostEqual(by_variable["capacity_ratio_upper_bound"]["f64"], 1.0001)

    def test_exact_geometry_rejection_is_separate_from_capacity_comparison(self):
        events = scan_to_events.events_for_case(
            {"case_id": "rejected", "source_id": "row-1"},
            {
                "source_id": "row-1",
                "family": "edge",
                "f64_capacity": None,
                "audit_capacity_label": None,
                "exact_audit_status": "exact_validation_rejected",
            },
        )

        capacity = next(row for row in events if row.get("variable") == "capacity")
        self.assertEqual(capacity["exact_geometry_validation_status"], "rejected")
        self.assertEqual(capacity["input_pair_kind"], "f64_input_without_capacity_label")
        self.assertNotIn("comparison_label_kind", capacity)
        self.assertNotIn("oracle_kind", capacity)


if __name__ == "__main__":
    unittest.main()
