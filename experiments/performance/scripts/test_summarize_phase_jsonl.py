import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import summarize_phase_jsonl as phase_summary


class PhaseJsonlSummaryTests(unittest.TestCase):
    def test_pct_complete_ignores_samples_with_error_rows(self):
        rows = [
            {
                "target": "hk2017-pruned-f64",
                "facet_count": 5,
                "sample": 0,
                "phase": "accepted_fixture_acquisition",
                "elapsed_ms": 2.0,
                "status": "ok",
            },
            {
                "target": "hk2017-pruned-f64",
                "facet_count": 5,
                "sample": 0,
                "phase": "solve_candidates",
                "elapsed_ms": 3.0,
                "status": "ok",
            },
            {
                "target": "hk2017-pruned-f64",
                "facet_count": 5,
                "sample": 1,
                "phase": "accepted_fixture_acquisition",
                "elapsed_ms": 5.0,
                "status": "ok",
            },
            {
                "target": "hk2017-pruned-f64",
                "facet_count": 5,
                "sample": 1,
                "phase": "solve_candidates",
                "elapsed_ms": 7.0,
                "status": "error",
                "error": "fixture failure",
            },
        ]

        summary = phase_summary.summarize(rows)
        by_phase = {row["phase"]: row for row in summary}

        accepted = by_phase["accepted_fixture_acquisition"]
        self.assertAlmostEqual(accepted["pct_of_sample_total"], 70.0)
        self.assertAlmostEqual(accepted["pct_of_completed_sample_total"], 40.0)

        solve = by_phase["solve_candidates"]
        self.assertAlmostEqual(solve["pct_of_sample_total"], 60.0)
        self.assertAlmostEqual(solve["pct_of_completed_sample_total"], 60.0)
        self.assertEqual(solve["error_events"], 1)

    def test_optional_family_and_method_split_groups(self):
        rows = [
            {
                "target": "f64-capacity-e2e",
                "family": "random_product",
                "method": "strict",
                "facet_count": 10,
                "sample": 0,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 1.0,
                "status": "ok",
                "sigma_count": 0,
                "outcome": "not_run",
                "capacity_ran": False,
            },
            {
                "target": "f64-capacity-e2e",
                "family": "random_product",
                "method": "lp",
                "facet_count": 10,
                "sample": 0,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 7.0,
                "status": "ok",
                "sigma_count": 100,
                "outcome": "success",
                "capacity_ran": True,
            },
        ]

        summary = phase_summary.summarize(rows)
        self.assertEqual(len(summary), 2)
        by_method = {row["method"]: row for row in summary}
        self.assertEqual(by_method["strict"]["ok_mean_ms"], 1.0)
        self.assertEqual(by_method["lp"]["ok_mean_ms"], 7.0)
        self.assertEqual(by_method["lp"]["sigma_count_mean"], 100.0)
        self.assertEqual(by_method["strict"]["capacity_not_run_events"], 1)
        self.assertEqual(by_method["lp"]["capacity_run_events"], 1)

    def test_capacity_bundle_mean_ignores_no_capacity_fallback_rows(self):
        rows = [
            {
                "target": "f64-capacity-e2e",
                "family": "random_product",
                "method": "strict",
                "facet_count": 10,
                "sample": 0,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 1.0,
                "status": "ok",
                "outcome": "not_run",
                "capacity_ran": False,
                "validation_bundle_time_ms": 1.0,
                "capacity_bundle_time_ms": None,
                "sigma_count": 0,
                "admissible_f64_count": 0,
            },
            {
                "target": "f64-capacity-e2e",
                "family": "random_product",
                "method": "strict",
                "facet_count": 10,
                "sample": 1,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 13.0,
                "status": "ok",
                "outcome": "success",
                "capacity_ran": True,
                "validation_bundle_time_ms": 3.0,
                "capacity_bundle_time_ms": 10.0,
                "capacity_candidate_solve_ms": 9.0,
                "capacity_candidate_kkt_solve_ms": 8.0,
                "capacity_candidate_non_kkt_ms": 1.0,
                "sigma_count": 42,
                "admissible_f64_count": 5,
            },
        ]

        [summary] = phase_summary.summarize(rows)

        self.assertEqual(summary["capacity_run_events"], 1)
        self.assertEqual(summary["capacity_not_run_events"], 1)
        self.assertAlmostEqual(summary["validation_bundle_time_ms_mean"], 2.0)
        self.assertAlmostEqual(summary["capacity_bundle_time_ms_mean"], 10.0)
        self.assertAlmostEqual(summary["capacity_candidate_solve_ms_mean"], 9.0)
        self.assertAlmostEqual(summary["capacity_candidate_kkt_solve_ms_mean"], 8.0)
        self.assertAlmostEqual(summary["capacity_candidate_non_kkt_ms_mean"], 1.0)
        self.assertAlmostEqual(summary["capacity_sigma_count_mean"], 42.0)
        self.assertAlmostEqual(summary["admissible_f64_mean"], 5.0)

    def test_legacy_f64_capacity_field_names_are_still_summarized(self):
        rows = [
            {
                "target": "f64-capacity-e2e",
                "family": "hko2024_f64",
                "method": "lp",
                "facet_count": 8,
                "sample": 0,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 15.0,
                "status": "ok",
                "outcome": "success",
                "validation_time_ms": 4.0,
                "f64_time_ms": 11.0,
                "iterations": 17,
            }
        ]

        [summary] = phase_summary.summarize(rows)

        self.assertEqual(summary["capacity_run_events"], 1)
        self.assertAlmostEqual(summary["validation_bundle_time_ms_mean"], 4.0)
        self.assertAlmostEqual(summary["capacity_bundle_time_ms_mean"], 11.0)
        self.assertAlmostEqual(summary["capacity_sigma_count_mean"], 17.0)

    def test_csv_writer_includes_kkt_split_columns(self):
        rows = [
            {
                "target": "f64-capacity-e2e",
                "family": "random_product",
                "method": "lp_origin_vertex_product_billiard_or_hk",
                "facet_count": 12,
                "sample": 0,
                "phase": "f64_capacity_e2e",
                "elapsed_ms": 10.0,
                "status": "ok",
                "outcome": "success",
                "capacity_ran": True,
                "capacity_candidate_solve_ms": 9.0,
                "capacity_candidate_kkt_solve_ms": 8.0,
                "capacity_candidate_non_kkt_ms": 1.0,
            }
        ]
        summary = phase_summary.summarize(rows)
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "summary.csv"
            phase_summary.write_csv(path, summary)
            text = path.read_text()
        self.assertIn("capacity_candidate_kkt_solve_ms_mean", text)
        self.assertIn("capacity_candidate_non_kkt_ms_mean", text)


if __name__ == "__main__":
    unittest.main()
