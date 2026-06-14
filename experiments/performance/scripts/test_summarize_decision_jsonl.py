import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import summarize_decision_jsonl as decision_summary


class DecisionSummaryTests(unittest.TestCase):
    def test_summarizes_pairwise_decision_rows(self):
        rows = [
            {
                "target": "f64-decision-compare",
                "mode": "smoke",
                "family": "random",
                "decision": "origin_in_interior",
                "left_method": "strict_origin_predicate",
                "right_method": "lp_origin",
                "left_time_ms": 0.2,
                "right_time_ms": 0.1,
                "left_true_count": 1,
                "left_false_count": 0,
                "left_indeterminate_count": 2,
                "left_error_count": 0,
                "right_true_count": 3,
                "right_false_count": 0,
                "right_indeterminate_count": 0,
                "right_error_count": 0,
                "agreement_count": 1,
                "disagreement_count": 2,
                "left_indeterminate_right_decisive_count": 2,
                "left_decisive_right_indeterminate_count": 0,
            },
            {
                "target": "f64-decision-compare",
                "mode": "smoke",
                "family": "random_product",
                "decision": "origin_in_interior",
                "left_method": "strict_origin_predicate",
                "right_method": "lp_origin",
                "left_time_ms": 0.4,
                "right_time_ms": 0.2,
                "left_true_count": 0,
                "left_false_count": 0,
                "left_indeterminate_count": 4,
                "left_error_count": 0,
                "right_true_count": 4,
                "right_false_count": 0,
                "right_indeterminate_count": 0,
                "right_error_count": 0,
                "agreement_count": 0,
                "disagreement_count": 4,
                "left_indeterminate_right_decisive_count": 4,
                "left_decisive_right_indeterminate_count": 0,
            },
        ]

        [summary] = decision_summary.summarize(rows)

        self.assertEqual(summary["rows"], 2)
        self.assertEqual(summary["families"], "random:1|random_product:1")
        self.assertAlmostEqual(summary["left_time_ms_mean"], 0.3)
        self.assertAlmostEqual(summary["right_time_ms_mean"], 0.15)
        self.assertEqual(summary["left_true_total"], 1)
        self.assertEqual(summary["left_indeterminate_total"], 6)
        self.assertEqual(summary["right_true_total"], 7)
        self.assertEqual(summary["disagreement_total"], 6)

    def test_summarizes_single_method_rows(self):
        [summary] = decision_summary.summarize(
            [
                {
                    "target": "f64-decision-compare",
                    "mode": "smoke",
                    "family": "hko2024_f64",
                    "decision": "omega_sign",
                    "left_method": "f64_omega",
                    "right_method": None,
                    "left_time_ms": 0.3,
                    "left_true_count": 4,
                    "left_false_count": 5,
                    "left_indeterminate_count": 1,
                    "left_error_count": 0,
                }
            ]
        )

        self.assertEqual(summary["right_method"], "")
        self.assertEqual(summary["right_time_ms_mean"], None)
        self.assertEqual(summary["right_true_total"], None)


if __name__ == "__main__":
    unittest.main()
