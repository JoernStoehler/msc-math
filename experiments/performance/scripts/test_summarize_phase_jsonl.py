import unittest

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


if __name__ == "__main__":
    unittest.main()
