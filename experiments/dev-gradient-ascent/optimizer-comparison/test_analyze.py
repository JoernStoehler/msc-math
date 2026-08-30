"""Regression tests for legacy and current-state optimizer packets."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path


HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("optimizer_comparison_analyze", HERE / "analyze.py")
assert SPEC and SPEC.loader
ANALYZE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ANALYZE)


def packet(*, current_state: bool) -> tuple[dict, list[dict], list[dict], list[dict], list[dict]]:
    run_id = "run"
    round_id = f"{run_id}--r00000"
    proposal_id = f"{round_id}--p000"
    initial_id = f"{run_id}--e000000"
    proposal_evaluation_id = f"{run_id}--e000001"
    plan = {
        "runs": [{"run_id": run_id}],
        "starts": [{"name": "start", "source_sys": None}],
    }
    run = {
        "schema_version": 1,
        "run_id": run_id,
        "start_id": "start",
        "initial_evaluation_id": initial_id,
        "initial_sys": 0.5,
        "best_evaluation_id": proposal_evaluation_id,
        "best_sys": 0.8,
        "charge_initial": False,
        "budget": 1,
        "charged_calls": 1,
        "rounds": 1,
        "stop_reason": "budget_exhausted",
        "physical_evaluations": 2,
        "invalid_evaluations": 0,
        "indeterminate_evaluations": 0,
        "exact_fallback_evaluations": 0,
    }
    if current_state:
        run["final_algorithm_state"] = {
            "kind": "evaluated_point",
            "evaluation_id": proposal_evaluation_id,
        }
    evaluation_template = {
        "schema_version": 1,
        "run_id": run_id,
        "usable_by_optimizer": True,
        "sys": 0.5,
        "cache_status": "miss",
        "status": "ok",
    }
    evaluations = [
        {
            **evaluation_template,
            "evaluation_id": initial_id,
            "proposal_id": None,
            "role": "initial",
            "logical_call": 0,
            "charged": False,
        },
        {
            **evaluation_template,
            "evaluation_id": proposal_evaluation_id,
            "proposal_id": proposal_id,
            "role": "proposal",
            "logical_call": 1,
            "charged": True,
            "sys": 0.8,
        },
    ]
    proposals = [
        {
            "schema_version": 1,
            "run_id": run_id,
            "round_id": round_id,
            "proposal_id": proposal_id,
            "evaluation_id": proposal_evaluation_id,
        }
    ]
    round_row = {
        "schema_version": 1,
        "run_id": run_id,
        "round_id": round_id,
        "round_index": 0,
        "charged_calls_before": 0,
        "charged_calls_after": 1,
        "best_evaluation_id_before": initial_id,
        "best_evaluation_id_after": proposal_evaluation_id,
        "best_sys_before": 0.5,
        "best_sys_after": 0.8,
        "proposal_ids": [proposal_id],
        "selected": [{"proposal_id": proposal_id, "weight": 1.0}],
    }
    if current_state:
        round_row.update(
            algorithm_state_before={
                "kind": "evaluated_point",
                "evaluation_id": initial_id,
            },
            algorithm_state_after={
                "kind": "evaluated_point",
                "evaluation_id": proposal_evaluation_id,
            },
        )
    return plan, [run], [round_row], proposals, evaluations


class ValidatePacketTests(unittest.TestCase):
    def test_legacy_round_without_current_state_fields(self) -> None:
        packet_rows = packet(current_state=False)
        diagnostics = ANALYZE.validate_packet(*packet_rows)
        self.assertEqual(diagnostics["run_count"], 1)

    def test_current_state_round_is_validated(self) -> None:
        packet_rows = packet(current_state=True)
        diagnostics = ANALYZE.validate_packet(*packet_rows)
        self.assertEqual(diagnostics["round_count"], 1)

    def test_partial_current_state_is_rejected(self) -> None:
        plan, runs, rounds, proposals, evaluations = packet(current_state=False)
        runs[0]["final_algorithm_state"] = {
            "kind": "no_single_current_state"
        }
        with self.assertRaisesRegex(ValueError, "current-state fields"):
            ANALYZE.validate_packet(plan, runs, rounds, proposals, evaluations)


class MeasuredComputeCurveTests(unittest.TestCase):
    def test_cutoff_and_unrounded_terminal_compute_are_retained(self) -> None:
        runs = [
            {
                "run_id": "run",
                "algorithm_id": "algorithm",
                "initial_sys": 0.5,
                "best_sys": 0.8,
                "compute_budget_ms": 1000.0,
                # The last ask returned no proposals and therefore has no round.
                "charged_compute_ms": 1030.0,
            }
        ]
        rounds = [
            {
                "run_id": "run",
                "round_index": 0,
                "charged_calls_before": 0,
                "charged_calls_after": 1,
                "ask_ms": 15.0,
                "tell_ms": 5.0,
                "best_sys_after": 0.8,
            }
        ]
        evaluations = [
            {
                "run_id": "run",
                "charged": True,
                "logical_call": 1,
                "total_ms": 990.0,
            }
        ]

        rows = ANALYZE.measured_compute_curve_rows(runs, rounds, evaluations)
        by_time = {row["measured_compute_ms"]: row for row in rows}

        self.assertIn(1000.0, by_time)
        self.assertEqual(by_time[1000.0]["median_best_sys"], 0.5)
        self.assertEqual(by_time[1030.0]["median_best_sys"], 0.8)
        self.assertEqual(by_time[1030.0]["terminal_carry_forward_fraction"], 0.0)


if __name__ == "__main__":
    unittest.main()
