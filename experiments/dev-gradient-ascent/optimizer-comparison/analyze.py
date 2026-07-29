# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Validate and analyze an optimizer-runs trajectory dataset."""

from __future__ import annotations

import argparse
import bisect
import csv
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

SCHEMA_VERSION = 1
BOOTSTRAP_REPLICATES = 4000
BOOTSTRAP_SEED = 2026072303
ALGORITHM_STATE_FIELDS = ("algorithm_state_before", "algorithm_state_after")


def display_algorithm(algorithm_id: str) -> str:
    return {
        "cma-s1e-1-l8": "CMA-ES",
        "directional-above-8e-2": "history + transition prediction",
        "gap-w1e-1-adaptive-d1e-1": "finite-gap affine model",
        "history-baseline": "branch history",
        "literal-eta1e-2": "literal branch gradient",
        "pattern-r3e-2": "coordinate direct search",
        "safeguarded-adaptive-d1e-1": "safeguarded branch gradient",
    }.get(algorithm_id, algorithm_id)


def load_json(path: Path) -> Any:
    with path.open() as stream:
        return json.load(stream)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows = []
    with path.open() as stream:
        for line_number, line in enumerate(stream, 1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as error:
                raise ValueError(f"{path}:{line_number}: {error}") from error
            if row.get("schema_version") != SCHEMA_VERSION:
                raise ValueError(
                    f"{path}:{line_number}: unsupported schema version "
                    f"{row.get('schema_version')!r}"
                )
            rows.append(row)
    return rows


def unique_by(rows: Iterable[dict[str, Any]], key: str) -> dict[str, dict[str, Any]]:
    result: dict[str, dict[str, Any]] = {}
    for row in rows:
        value = row[key]
        if value in result:
            raise ValueError(f"duplicate {key} {value!r}")
        result[value] = row
    return result


def assert_close(left: float, right: float, context: str) -> None:
    tolerance = 1e-12 * max(1.0, abs(left), abs(right))
    if abs(left - right) > tolerance:
        raise ValueError(f"{context}: {left} != {right}")


def state_evaluation_ids(
    state: dict[str, Any],
    run_id: str,
    evaluation_by_id: dict[str, dict[str, Any]],
) -> list[str]:
    kind = state.get("kind")
    if kind == "evaluated_point":
        ids = [state.get("evaluation_id")]
    elif kind == "evaluated_population":
        ids = state.get("evaluation_ids")
        if not isinstance(ids, list) or not ids:
            raise ValueError(f"{run_id}: evaluated population is empty or malformed")
        if len(ids) != len(set(ids)):
            raise ValueError(f"{run_id}: duplicate evaluated population member")
    elif kind in {"unevaluated_model_or_distribution", "no_single_current_state"}:
        return []
    else:
        raise ValueError(f"{run_id}: unknown algorithm state kind {kind!r}")
    for evaluation_id in ids:
        evaluation = evaluation_by_id.get(evaluation_id)
        if evaluation is None or evaluation["run_id"] != run_id:
            raise ValueError(f"{run_id}: algorithm state references absent evaluation")
        if not evaluation["usable_by_optimizer"] or evaluation["sys"] is None:
            raise ValueError(f"{run_id}: algorithm state references unusable evaluation")
    return ids


def validate_packet(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    rounds: list[dict[str, Any]],
    proposals: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
) -> dict[str, Any]:
    expected_run_ids = {row["run_id"] for row in plan["runs"]}
    run_by_id = unique_by(runs, "run_id")
    if set(run_by_id) != expected_run_ids:
        raise ValueError(
            "completion coverage mismatch: "
            f"missing={sorted(expected_run_ids - set(run_by_id))}, "
            f"unexpected={sorted(set(run_by_id) - expected_run_ids)}"
        )
    proposal_by_id = unique_by(proposals, "proposal_id")
    evaluation_by_id = unique_by(evaluations, "evaluation_id")
    round_by_id = unique_by(rounds, "round_id")
    grouped_evaluations: dict[str, list[dict[str, Any]]] = defaultdict(list)
    grouped_proposals: dict[str, list[dict[str, Any]]] = defaultdict(list)
    grouped_rounds: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in evaluations:
        grouped_evaluations[row["run_id"]].append(row)
    for row in proposals:
        grouped_proposals[row["run_id"]].append(row)
    for row in rounds:
        grouped_rounds[row["run_id"]].append(row)
    diagnostics = {
        "run_count": len(runs),
        "evaluation_count": len(evaluations),
        "proposal_count": len(proposals),
        "round_count": len(rounds),
    }
    initial_sys_by_start: dict[str, list[float]] = defaultdict(list)
    for run_id in sorted(expected_run_ids):
        run = run_by_id[run_id]
        run_evaluations = grouped_evaluations[run_id]
        run_proposals = grouped_proposals[run_id]
        run_rounds = sorted(grouped_rounds[run_id], key=lambda row: row["round_index"])
        if len(run_evaluations) != 1 + len(run_proposals):
            raise ValueError(
                f"{run_id}: {len(run_evaluations)} evaluations but "
                f"{len(run_proposals)} proposals"
            )
        initials = [row for row in run_evaluations if row["role"] == "initial"]
        if len(initials) != 1:
            raise ValueError(f"{run_id}: expected one initial evaluation")
        initial = initials[0]
        if initial["evaluation_id"] != run["initial_evaluation_id"]:
            raise ValueError(f"{run_id}: initial evaluation id mismatch")
        if not initial["usable_by_optimizer"] or initial["sys"] is None:
            raise ValueError(f"{run_id}: unusable initial evaluation")
        assert_close(initial["sys"], run["initial_sys"], f"{run_id}: initial sys")
        initial_sys_by_start[run["start_id"]].append(initial["sys"])
        charged = sorted(
            [row for row in run_evaluations if row["charged"]],
            key=lambda row: row["logical_call"],
        )
        expected_calls = list(range(1, len(charged) + 1))
        actual_calls = [row["logical_call"] for row in charged]
        if actual_calls != expected_calls:
            raise ValueError(
                f"{run_id}: charged logical calls {actual_calls} != {expected_calls}"
            )
        if run["charged_calls"] != len(charged):
            raise ValueError(f"{run_id}: charged call count mismatch")
        if run["stop_reason"] == "budget_exhausted" and len(charged) != run["budget"]:
            raise ValueError(f"{run_id}: budget stop without exhausting budget")
        if [row["round_index"] for row in run_rounds] != list(range(len(run_rounds))):
            raise ValueError(f"{run_id}: noncontiguous round indices")
        if len(run_rounds) != run["rounds"]:
            raise ValueError(f"{run_id}: round count mismatch")
        best_id = initial["evaluation_id"]
        best_sys = initial["sys"]
        calls_before = int(run["charge_initial"])
        previous_algorithm_state = None
        round_state_presence = [
            tuple(field in round_row for field in ALGORITHM_STATE_FIELDS)
            for round_row in run_rounds
        ]
        if any(any(presence) for presence in round_state_presence) and not all(
            all(presence) for presence in round_state_presence
        ):
            raise ValueError(
                f"{run_id}: incomplete current-state fields in round trace"
            )
        has_final_algorithm_state = "final_algorithm_state" in run
        has_round_algorithm_state = (
            has_final_algorithm_state
            if not run_rounds
            else all(all(presence) for presence in round_state_presence)
        )
        if has_round_algorithm_state != has_final_algorithm_state:
            raise ValueError(
                f"{run_id}: current-state fields are incomplete across run trace"
            )
        for round_row in run_rounds:
            if round_row["charged_calls_before"] != calls_before:
                raise ValueError(f"{run_id}: round call boundary mismatch")
            if round_row["best_evaluation_id_before"] != best_id:
                raise ValueError(f"{run_id}: round best-before id mismatch")
            assert_close(
                round_row["best_sys_before"], best_sys, f"{run_id}: round best before"
            )
            proposal_ids = round_row["proposal_ids"]
            if len(proposal_ids) != len(set(proposal_ids)):
                raise ValueError(f"{run_id}: duplicate proposal in round")
            for proposal_id in proposal_ids:
                proposal = proposal_by_id.get(proposal_id)
                if proposal is None or proposal["round_id"] != round_row["round_id"]:
                    raise ValueError(f"{run_id}: broken proposal-round link")
                linked = [
                    row
                    for row in run_evaluations
                    if row["proposal_id"] == proposal_id
                ]
                if len(linked) != 1:
                    raise ValueError(
                        f"{run_id}: proposal {proposal_id} has {len(linked)} evaluations"
                    )
                evaluation = linked[0]
                if proposal["evaluation_id"] != evaluation["evaluation_id"]:
                    raise ValueError(f"{run_id}: proposal evaluation id mismatch")
                if (
                    evaluation["usable_by_optimizer"]
                    and evaluation["sys"] is not None
                    and evaluation["sys"] > best_sys
                ):
                    best_id = evaluation["evaluation_id"]
                    best_sys = evaluation["sys"]
            selected_ids = {row["proposal_id"] for row in round_row["selected"]}
            if not selected_ids.issubset(proposal_ids):
                raise ValueError(f"{run_id}: selected proposal absent from round")
            if has_round_algorithm_state:
                selected_evaluation_ids = {
                    proposal_by_id[proposal_id]["evaluation_id"]
                    for proposal_id in selected_ids
                }
                state_before = round_row["algorithm_state_before"]
                state_after = round_row["algorithm_state_after"]
                before_ids = state_evaluation_ids(
                    state_before, run_id, evaluation_by_id
                )
                after_ids = state_evaluation_ids(
                    state_after, run_id, evaluation_by_id
                )
                if previous_algorithm_state is None:
                    if (
                        state_before["kind"] == "evaluated_point"
                        and before_ids != [initial["evaluation_id"]]
                    ):
                        raise ValueError(
                            f"{run_id}: initial current state is not the start"
                        )
                elif state_before != previous_algorithm_state:
                    raise ValueError(f"{run_id}: discontinuous algorithm state")
                if state_after != state_before:
                    changed_ids = set(after_ids) - set(before_ids)
                    if not changed_ids.issubset(selected_evaluation_ids):
                        raise ValueError(
                            f"{run_id}: current state changed to an unselected evaluation"
                        )
                previous_algorithm_state = state_after
            calls_before += len(proposal_ids)
            if round_row["charged_calls_after"] != calls_before:
                raise ValueError(f"{run_id}: round call-after mismatch")
            if round_row["best_evaluation_id_after"] != best_id:
                raise ValueError(f"{run_id}: round best-after id mismatch")
            assert_close(
                round_row["best_sys_after"], best_sys, f"{run_id}: round best after"
            )
        if calls_before != run["charged_calls"]:
            raise ValueError(f"{run_id}: final call boundary mismatch")
        if best_id != run["best_evaluation_id"]:
            raise ValueError(f"{run_id}: final best id mismatch")
        assert_close(best_sys, run["best_sys"], f"{run_id}: final best sys")
        if has_final_algorithm_state:
            state_evaluation_ids(
                run["final_algorithm_state"], run_id, evaluation_by_id
            )
            if (
                previous_algorithm_state is not None
                and run["final_algorithm_state"] != previous_algorithm_state
            ):
                raise ValueError(f"{run_id}: final algorithm state mismatch")
        expected_physical = sum(row["cache_status"] == "miss" for row in run_evaluations)
        if expected_physical != run["physical_evaluations"]:
            raise ValueError(f"{run_id}: physical evaluation count mismatch")
        expected_statuses = {
            "invalid_evaluations": "invalid",
            "indeterminate_evaluations": "indeterminate_geometry",
            "exact_fallback_evaluations": "exact_fallback",
        }
        for field, status in expected_statuses.items():
            if sum(row["status"] == status for row in run_evaluations) != run[field]:
                raise ValueError(f"{run_id}: {field} mismatch")
        for proposal in run_proposals:
            if proposal["round_id"] not in round_by_id:
                raise ValueError(f"{run_id}: proposal references absent round")
        for evaluation in run_evaluations:
            if evaluation["proposal_id"] is not None and evaluation["proposal_id"] not in proposal_by_id:
                raise ValueError(f"{run_id}: evaluation references absent proposal")
    source_by_start = {row["name"]: row["source_sys"] for row in plan["starts"]}
    source_differences = []
    for start_id, values in initial_sys_by_start.items():
        if max(values) != min(values):
            raise ValueError(f"{start_id}: initial evaluator is not deterministic across runs")
        source_sys = source_by_start[start_id]
        if source_sys is not None:
            source_differences.append((values[0], source_sys))
    diagnostics["source_reference_comparison"] = {
        "count": len(source_differences),
        "max_absolute_difference": max(
            (abs(observed - reference) for observed, reference in source_differences),
            default=0.0,
        ),
        "max_relative_difference": max(
            (
                abs(observed - reference) / max(abs(reference), 1e-300)
                for observed, reference in source_differences
            ),
            default=0.0,
        ),
        "relative_difference_over_1e-8_count": sum(
            abs(observed - reference) / max(abs(reference), 1e-300) > 1e-8
            for observed, reference in source_differences
        ),
    }
    return diagnostics


def trajectory_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
) -> tuple[list[dict[str, Any]], dict[str, list[dict[str, Any]]]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in evaluations:
        grouped[row["run_id"]].append(row)
    per_run: dict[str, list[dict[str, Any]]] = {}
    long_rows: list[dict[str, Any]] = []
    for run in runs:
        rows = grouped[run["run_id"]]
        initial = next(row for row in rows if row["role"] == "initial")
        charged = sorted(
            [row for row in rows if row["charged"]], key=lambda row: row["logical_call"]
        )
        best_sys = initial["sys"]
        best_evaluation_id = initial["evaluation_id"]
        cumulative_evaluator_ms = 0.0
        trajectory = [
            {
                "run_id": run["run_id"],
                "start_id": run["start_id"],
                "algorithm_id": run["algorithm_id"],
                "call": 0,
                "cumulative_evaluator_ms": 0.0,
                "best_sys": best_sys,
                "best_evaluation_id": best_evaluation_id,
                "evaluation_id": initial["evaluation_id"],
                "evaluation_status": initial["status"],
            }
        ]
        for evaluation in charged:
            cumulative_evaluator_ms += evaluation["total_ms"]
            if (
                evaluation["usable_by_optimizer"]
                and evaluation["sys"] is not None
                and evaluation["sys"] > best_sys
            ):
                best_sys = evaluation["sys"]
                best_evaluation_id = evaluation["evaluation_id"]
            trajectory.append(
                {
                    "run_id": run["run_id"],
                    "start_id": run["start_id"],
                    "algorithm_id": run["algorithm_id"],
                    "call": evaluation["logical_call"],
                    "cumulative_evaluator_ms": cumulative_evaluator_ms,
                    "best_sys": best_sys,
                    "best_evaluation_id": best_evaluation_id,
                    "evaluation_id": evaluation["evaluation_id"],
                    "evaluation_status": evaluation["status"],
                }
            )
        per_run[run["run_id"]] = trajectory
        long_rows.extend(trajectory)
    return long_rows, per_run


def percentile(values: Iterable[float], q: float) -> float:
    return float(np.quantile(np.asarray(list(values), dtype=float), q))


def bootstrap_interval(
    values: Iterable[float], statistic=np.median
) -> tuple[float, float]:
    array = np.asarray(list(values), dtype=float)
    if len(array) == 1:
        return float(array[0]), float(array[0])
    rng = np.random.default_rng(BOOTSTRAP_SEED + len(array))
    indices = rng.integers(0, len(array), size=(BOOTSTRAP_REPLICATES, len(array)))
    samples = statistic(array[indices], axis=1)
    return float(np.quantile(samples, 0.025)), float(np.quantile(samples, 0.975))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise ValueError(f"refusing to write empty table {path}")
    fieldnames = list(dict.fromkeys(key for row in rows for key in row))
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def algorithm_configuration_rows(plan: dict[str, Any]) -> list[dict[str, Any]]:
    def flatten(prefix: str, value: Any, output: dict[str, Any]) -> None:
        if isinstance(value, dict):
            for key, child in value.items():
                flatten(f"{prefix}.{key}" if prefix else key, child, output)
        elif isinstance(value, list):
            output[prefix] = json.dumps(value, separators=(",", ":"))
        else:
            output[prefix] = value

    rows = []
    for algorithm in plan["algorithms"]:
        row: dict[str, Any] = {}
        flatten("", algorithm, row)
        row["configuration_json"] = json.dumps(
            algorithm, sort_keys=True, separators=(",", ":")
        )
        rows.append(row)
    return rows


def call_curve_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    by_algorithm: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        by_algorithm[run["algorithm_id"]].append(run)
    checkpoints = sorted(set([0, *plan["checkpoints"], plan["budget"]]))
    output = []
    for algorithm_id, algorithm_runs in sorted(by_algorithm.items()):
        for call in checkpoints:
            values = []
            for run in algorithm_runs:
                eligible = [
                    row for row in per_run[run["run_id"]] if row["call"] <= call
                ]
                values.append(eligible[-1]["best_sys"])
            low, high = bootstrap_interval(values)
            output.append(
                {
                    "algorithm_id": algorithm_id,
                    "call": call,
                    "n": len(values),
                    "mean_best_sys": float(np.mean(values)),
                    "median_best_sys": float(np.median(values)),
                    "q10_best_sys": percentile(values, 0.10),
                    "q90_best_sys": percentile(values, 0.90),
                    "median_bootstrap_low": low,
                    "median_bootstrap_high": high,
                }
            )
    return output


def facet_call_curve_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    by_algorithm_facet: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(
        list
    )
    for run in runs:
        by_algorithm_facet[
            (run["algorithm_id"], facet_by_start[run["start_id"]])
        ].append(run)
    checkpoints = sorted(set([0, *plan["checkpoints"], plan["budget"]]))
    output = []
    for (algorithm_id, facet_count), algorithm_runs in sorted(
        by_algorithm_facet.items()
    ):
        for call in checkpoints:
            values = []
            for run in algorithm_runs:
                eligible = [
                    row for row in per_run[run["run_id"]] if row["call"] <= call
                ]
                values.append(eligible[-1]["best_sys"])
            low, high = bootstrap_interval(values)
            output.append(
                {
                    "facet_count": facet_count,
                    "algorithm_id": algorithm_id,
                    "call": call,
                    "n": len(values),
                    "mean_best_sys": float(np.mean(values)),
                    "median_best_sys": float(np.median(values)),
                    "q10_best_sys": percentile(values, 0.10),
                    "q90_best_sys": percentile(values, 0.90),
                    "median_bootstrap_low": low,
                    "median_bootstrap_high": high,
                }
            )
    return output


def full_call_curve_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    by_algorithm: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        by_algorithm[run["algorithm_id"]].append(run)
    output = []
    for algorithm_id, algorithm_runs in sorted(by_algorithm.items()):
        for call in range(plan["budget"] + 1):
            values = []
            for run in algorithm_runs:
                eligible = [
                    row for row in per_run[run["run_id"]] if row["call"] <= call
                ]
                values.append(eligible[-1]["best_sys"])
            output.append(
                {
                    "algorithm_id": algorithm_id,
                    "call": call,
                    "n": len(values),
                    "median_best_sys": float(np.median(values)),
                    "q10_best_sys": percentile(values, 0.10),
                    "q90_best_sys": percentile(values, 0.90),
                }
            )
    return output


def threshold_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    thresholds = [0.6, 0.7, 0.75, 0.8]
    calls = sorted(set([*plan["checkpoints"], plan["budget"]]))
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        grouped[run["algorithm_id"]].append(run)
    output = []
    for algorithm_id, algorithm_runs in sorted(grouped.items()):
        for call in calls:
            values = []
            for run in algorithm_runs:
                eligible = [
                    row for row in per_run[run["run_id"]] if row["call"] <= call
                ]
                values.append(eligible[-1]["best_sys"])
            for threshold in thresholds:
                output.append(
                    {
                        "algorithm_id": algorithm_id,
                        "call": call,
                        "threshold": threshold,
                        "n": len(values),
                        "reach_fraction": sum(
                            value >= threshold for value in values
                        )
                        / len(values),
                    }
                )
    return output


def iid_crossover_rows(full_calls: list[dict[str, Any]]) -> list[dict[str, Any]]:
    curves: dict[str, dict[int, float]] = defaultdict(dict)
    for row in full_calls:
        curves[row["algorithm_id"]][row["call"]] = row["median_best_sys"]
    iid_id = next(
        (algorithm_id for algorithm_id in curves if algorithm_id.startswith("iid-")),
        None,
    )
    if iid_id is None:
        return []
    output = []
    for algorithm_id in sorted(curves):
        if algorithm_id == iid_id:
            continue
        winning_calls = [
            call
            for call, value in curves[algorithm_id].items()
            if value > curves[iid_id][call]
        ]
        final_call = max(curves[algorithm_id])
        output.append(
            {
                "algorithm_id": algorithm_id,
                "iid_algorithm_id": iid_id,
                "first_call_median_exceeds_iid": (
                    None if not winning_calls else min(winning_calls)
                ),
                "final_median_minus_iid": curves[algorithm_id][final_call]
                - curves[iid_id][final_call],
            }
        )
    return output


def compute_curve_rows(
    runs: list[dict[str, Any]], per_run: dict[str, list[dict[str, Any]]]
) -> list[dict[str, Any]]:
    by_algorithm: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        by_algorithm[run["algorithm_id"]].append(run)
    terminal_costs = [
        per_run[run["run_id"]][-1]["cumulative_evaluator_ms"] for run in runs
    ]
    common_maximum = max(terminal_costs)
    grid = np.linspace(0.0, common_maximum, 101)
    output = []
    for algorithm_id, algorithm_runs in sorted(by_algorithm.items()):
        for compute_ms in grid:
            values = []
            carried = 0
            for run in algorithm_runs:
                trace = per_run[run["run_id"]]
                eligible = [
                    row
                    for row in trace
                    if row["cumulative_evaluator_ms"] <= compute_ms
                ]
                values.append(eligible[-1]["best_sys"])
                carried += trace[-1]["cumulative_evaluator_ms"] < compute_ms
            output.append(
                {
                    "algorithm_id": algorithm_id,
                    "evaluator_ms": float(compute_ms),
                    "n": len(values),
                    "terminal_carry_forward_fraction": carried / len(values),
                    "median_best_sys": float(np.median(values)),
                    "q10_best_sys": percentile(values, 0.10),
                    "q90_best_sys": percentile(values, 0.90),
                }
            )
    return output


def measured_compute_curve_rows(
    runs: list[dict[str, Any]],
    rounds: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    evaluations_by_run: dict[str, dict[int, dict[str, Any]]] = defaultdict(dict)
    for evaluation in evaluations:
        if evaluation["charged"]:
            evaluations_by_run[evaluation["run_id"]][evaluation["logical_call"]] = (
                evaluation
            )
    rounds_by_run: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for round_row in rounds:
        rounds_by_run[round_row["run_id"]].append(round_row)
    traces: dict[str, list[tuple[float, float]]] = {}
    for run in runs:
        cumulative = 0.0
        trace = [(0.0, run["initial_sys"])]
        for round_row in sorted(
            rounds_by_run[run["run_id"]], key=lambda row: row["round_index"]
        ):
            evaluation_ms = sum(
                evaluations_by_run[run["run_id"]][logical_call]["total_ms"]
                for logical_call in range(
                    round_row["charged_calls_before"] + 1,
                    round_row["charged_calls_after"] + 1,
                )
            )
            cumulative += (
                evaluation_ms + round_row["ask_ms"] + round_row["tell_ms"]
            )
            trace.append((cumulative, round_row["best_sys_after"]))
        traces[run["run_id"]] = trace
    common_maximum = max(trace[-1][0] for trace in traces.values())
    grid = np.linspace(0.0, common_maximum, 101)
    by_algorithm: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        by_algorithm[run["algorithm_id"]].append(run)
    output = []
    for algorithm_id, algorithm_runs in sorted(by_algorithm.items()):
        for compute_ms in grid:
            values = []
            carried = 0
            for run in algorithm_runs:
                trace = traces[run["run_id"]]
                eligible = [
                    best_sys
                    for cumulative, best_sys in trace
                    if cumulative <= compute_ms
                ]
                values.append(eligible[-1])
                carried += trace[-1][0] < compute_ms
            output.append(
                {
                    "algorithm_id": algorithm_id,
                    "measured_compute_ms": float(compute_ms),
                    "n": len(values),
                    "terminal_carry_forward_fraction": carried / len(values),
                    "median_best_sys": float(np.median(values)),
                    "q10_best_sys": percentile(values, 0.10),
                    "q90_best_sys": percentile(values, 0.90),
                }
            )
    return output


def final_rows(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        grouped[run["algorithm_id"]].append(run)
    output = []
    for algorithm_id, algorithm_runs in sorted(grouped.items()):
        finals = [row["best_sys"] for row in algorithm_runs]
        improvements = [row["best_sys"] - row["initial_sys"] for row in algorithm_runs]
        low, high = bootstrap_interval(finals)
        output.append(
            {
                "algorithm_id": algorithm_id,
                "n": len(finals),
                "mean_final_sys": float(np.mean(finals)),
                "median_final_sys": float(np.median(finals)),
                "q10_final_sys": percentile(finals, 0.10),
                "q90_final_sys": percentile(finals, 0.90),
                "median_final_bootstrap_low": low,
                "median_final_bootstrap_high": high,
                "median_improvement": float(np.median(improvements)),
                "invalid_rate": sum(row["invalid_evaluations"] for row in algorithm_runs)
                / sum(1 + row["charged_calls"] for row in algorithm_runs),
                "indeterminate_rate": sum(
                    row["indeterminate_evaluations"] for row in algorithm_runs
                )
                / sum(1 + row["charged_calls"] for row in algorithm_runs),
                "exact_fallback_rate": sum(
                    row["exact_fallback_evaluations"] for row in algorithm_runs
                )
                / sum(1 + row["charged_calls"] for row in algorithm_runs),
                "median_wall_ms": float(
                    np.median([row["wall_ms"] for row in algorithm_runs])
                ),
            }
        )
    return output


def run_stop_rows(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        grouped[run["algorithm_id"]].append(run)
    output = []
    for algorithm_id, algorithm_runs in sorted(grouped.items()):
        reasons = Counter(run["stop_reason"] for run in algorithm_runs)
        output.append(
            {
                "algorithm_id": algorithm_id,
                "n": len(algorithm_runs),
                "median_charged_calls": float(
                    np.median([run["charged_calls"] for run in algorithm_runs])
                ),
                "q10_charged_calls": percentile(
                    [run["charged_calls"] for run in algorithm_runs], 0.10
                ),
                "q90_charged_calls": percentile(
                    [run["charged_calls"] for run in algorithm_runs], 0.90
                ),
                "maximum_best_sys": max(
                    run["best_sys"] for run in algorithm_runs
                ),
                "reached_sys_1_count": sum(
                    run["best_sys"] >= 1.0 for run in algorithm_runs
                ),
                "stop_reason_counts": "; ".join(
                    f"{reason}={count}" for reason, count in sorted(reasons.items())
                ),
            }
        )
    return output


def facet_final_rows(
    plan: dict[str, Any], runs: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    grouped: dict[tuple[int, str], list[dict[str, Any]]] = defaultdict(list)
    for run in runs:
        grouped[(facet_by_start[run["start_id"]], run["algorithm_id"])].append(run)
    output = []
    for (facet_count, algorithm_id), algorithm_runs in sorted(grouped.items()):
        finals = [row["best_sys"] for row in algorithm_runs]
        improvements = [
            row["best_sys"] - row["initial_sys"] for row in algorithm_runs
        ]
        low, high = bootstrap_interval(finals)
        output.append(
            {
                "facet_count": facet_count,
                "algorithm_id": algorithm_id,
                "n": len(finals),
                "mean_final_sys": float(np.mean(finals)),
                "median_final_sys": float(np.median(finals)),
                "q10_final_sys": percentile(finals, 0.10),
                "q90_final_sys": percentile(finals, 0.90),
                "median_bootstrap_low": low,
                "median_bootstrap_high": high,
                "median_improvement": float(np.median(improvements)),
            }
        )
    return output


def facet_balanced_regret_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    algorithms = sorted({row["algorithm_id"] for row in runs})
    starts = sorted({row["start_id"] for row in runs})
    run_by_algorithm_start = {
        (row["algorithm_id"], row["start_id"]): row for row in runs
    }
    calls = sorted(set([*plan["checkpoints"], plan["budget"]]))
    output = []
    for call in calls:
        values = {}
        initial = {}
        for start in starts:
            for algorithm in algorithms:
                run = run_by_algorithm_start[(algorithm, start)]
                eligible = [
                    row for row in per_run[run["run_id"]] if row["call"] <= call
                ]
                values[(algorithm, start)] = eligible[-1]["best_sys"]
                initial[start] = run["initial_sys"]
        oracle = {
            start: max(values[(algorithm, start)] for algorithm in algorithms)
            for start in starts
        }
        for algorithm in algorithms:
            normalized = []
            regrets = []
            per_facet = defaultdict(list)
            for start in starts:
                gain = values[(algorithm, start)] - initial[start]
                available = oracle[start] - initial[start]
                score = 1.0 if available <= 1.0e-15 else gain / available
                normalized.append(score)
                regrets.append(oracle[start] - values[(algorithm, start)])
                per_facet[facet_by_start[start]].append(score)
            output.append(
                {
                    "algorithm_id": algorithm,
                    "call": call,
                    "n": len(starts),
                    "mean_fraction_of_observed_available_gain": float(
                        np.mean(normalized)
                    ),
                    "median_fraction_of_observed_available_gain": float(
                        np.median(normalized)
                    ),
                    "mean_oracle_regret": float(np.mean(regrets)),
                    "maximum_facet_mean_score_gap": float(
                        max(np.mean(values) for values in per_facet.values())
                        - min(np.mean(values) for values in per_facet.values())
                    ),
                    "facet_mean_scores_json": json.dumps(
                        {
                            facet: float(np.mean(values))
                            for facet, values in sorted(per_facet.items())
                        },
                        sort_keys=True,
                    ),
                }
            )
    return output


def portfolio_screen_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    algorithms = sorted({row["algorithm_id"] for row in runs})
    starts = sorted({row["start_id"] for row in runs})
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    run_by_algorithm_start = {
        (row["algorithm_id"], row["start_id"]): row for row in runs
    }

    def value(algorithm: str, start: str, call: int) -> float:
        run = run_by_algorithm_start[(algorithm, start)]
        eligible = [
            row for row in per_run[run["run_id"]] if row["call"] <= call
        ]
        return eligible[-1]["best_sys"]

    oracle = {
        start: max(value(algorithm, start, plan["budget"]) for algorithm in algorithms)
        for start in starts
    }
    allocation_step = min(plan["checkpoints"], default=plan["budget"])
    allocations = range(allocation_step, plan["budget"], allocation_step)
    start_groups: list[tuple[object, list[str]]] = [("all", starts)]
    for facet_count in sorted(set(facet_by_start.values())):
        start_groups.append(
            (
                facet_count,
                [start for start in starts if facet_by_start[start] == facet_count],
            )
        )
    output = []
    for left_index, left in enumerate(algorithms):
        for right in algorithms[left_index + 1 :]:
            for left_calls in allocations:
                right_calls = plan["budget"] - left_calls
                for facet_count, group_starts in start_groups:
                    portfolio = [
                        max(
                            value(left, start, left_calls),
                            value(right, start, right_calls),
                        )
                        for start in group_starts
                    ]
                    regrets = [
                        oracle[start] - result
                        for start, result in zip(group_starts, portfolio)
                    ]
                    output.append(
                        {
                            "facet_count": facet_count,
                            "left_algorithm_id": left,
                            "left_calls": left_calls,
                            "right_algorithm_id": right,
                            "right_calls": right_calls,
                            "total_calls": plan["budget"],
                            "n": len(group_starts),
                            "mean_oracle_regret": float(np.mean(regrets)),
                            "median_oracle_regret": float(np.median(regrets)),
                            "q90_oracle_regret": percentile(regrets, 0.90),
                            "maximum_oracle_regret": max(regrets),
                            "within_0_01_of_oracle_fraction": sum(
                                regret <= 0.01 for regret in regrets
                            )
                            / len(regrets),
                            "median_portfolio_sys": float(np.median(portfolio)),
                        }
                    )
    return output


def paired_rows(runs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_algorithm_start = {
        (row["algorithm_id"], row["start_id"]): row for row in runs
    }
    algorithms = sorted({row["algorithm_id"] for row in runs})
    starts = sorted({row["start_id"] for row in runs})
    output = []
    for left_index, left in enumerate(algorithms):
        for right in algorithms[left_index + 1 :]:
            differences = [
                by_algorithm_start[(left, start)]["best_sys"]
                - by_algorithm_start[(right, start)]["best_sys"]
                for start in starts
            ]
            low, high = bootstrap_interval(differences)
            output.append(
                {
                    "left_algorithm_id": left,
                    "right_algorithm_id": right,
                    "n": len(differences),
                    "mean_left_minus_right": float(np.mean(differences)),
                    "median_left_minus_right": float(np.median(differences)),
                    "median_bootstrap_low": low,
                    "median_bootstrap_high": high,
                    "left_win_fraction": sum(value > 0.0 for value in differences)
                    / len(differences),
                    "tie_fraction": sum(value == 0.0 for value in differences)
                    / len(differences),
                }
            )
    return output


def paired_checkpoint_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    runs_by_algorithm_start = {
        (row["algorithm_id"], row["start_id"]): row for row in runs
    }
    algorithms = sorted({row["algorithm_id"] for row in runs})
    all_starts = sorted({row["start_id"] for row in runs})
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    facet_groups: list[tuple[object, list[str]]] = [("all", all_starts)]
    for facet_count in sorted(set(facet_by_start.values())):
        facet_groups.append(
            (
                facet_count,
                [
                    start
                    for start in all_starts
                    if facet_by_start[start] == facet_count
                ],
            )
        )
    calls = sorted(set([*plan["checkpoints"], plan["budget"]]))
    output = []
    for facet_count, starts in facet_groups:
        for call in calls:
            values = {}
            for algorithm_id in algorithms:
                for start_id in starts:
                    run = runs_by_algorithm_start[(algorithm_id, start_id)]
                    eligible = [
                        row
                        for row in per_run[run["run_id"]]
                        if row["call"] <= call
                    ]
                    values[(algorithm_id, start_id)] = eligible[-1]["best_sys"]
            for left_index, left in enumerate(algorithms):
                for right in algorithms[left_index + 1 :]:
                    differences = [
                        values[(left, start)] - values[(right, start)]
                        for start in starts
                    ]
                    low, high = bootstrap_interval(differences)
                    output.append(
                        {
                            "facet_count": facet_count,
                            "call": call,
                            "left_algorithm_id": left,
                            "right_algorithm_id": right,
                            "n": len(differences),
                            "mean_left_minus_right": float(np.mean(differences)),
                            "median_left_minus_right": float(np.median(differences)),
                            "median_bootstrap_low": low,
                            "median_bootstrap_high": high,
                            "left_win_fraction": sum(
                                value > 0.0 for value in differences
                            )
                            / len(differences),
                            "tie_fraction": sum(
                                value == 0.0 for value in differences
                            )
                            / len(differences),
                        }
                    )
    return output


def proposal_behavior_rows(
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    rounds: list[dict[str, Any]],
    proposals: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    run_by_id = {row["run_id"]: row for row in runs}
    round_by_id = {row["round_id"]: row for row in rounds}
    evaluation_by_id = {row["evaluation_id"]: row for row in evaluations}
    evaluation_by_proposal = {
        row["proposal_id"]: row
        for row in evaluations
        if row["proposal_id"] is not None
    }
    selected = {
        item["proposal_id"]: item["weight"]
        for round_row in rounds
        for item in round_row["selected"]
    }
    checkpoint_bounds = sorted(set([*plan["checkpoints"], plan["budget"]]))

    def phase(call: int) -> str:
        index = bisect.bisect_left(checkpoint_bounds, call)
        lower = 1 if index == 0 else checkpoint_bounds[index - 1] + 1
        upper = checkpoint_bounds[index]
        return f"{lower:06}_to_{upper:06}"

    long_rows = []
    for proposal in proposals:
        evaluation = evaluation_by_proposal[proposal["proposal_id"]]
        round_row = round_by_id[proposal["round_id"]]
        run = run_by_id[proposal["run_id"]]
        baseline = (
            evaluation_by_id.get(proposal["baseline_evaluation_id"])
            if proposal["baseline_evaluation_id"] is not None
            else None
        )
        distance = proposal["normalized_displacement_l2"]
        delta_baseline = (
            None
            if baseline is None
            or baseline["sys"] is None
            or evaluation["sys"] is None
            else evaluation["sys"] - baseline["sys"]
        )
        delta_best_before = (
            None
            if evaluation["sys"] is None
            else evaluation["sys"] - round_row["best_sys_before"]
        )
        long_rows.append(
            {
                "run_id": proposal["run_id"],
                "start_id": run["start_id"],
                "algorithm_id": run["algorithm_id"],
                "round_id": proposal["round_id"],
                "proposal_id": proposal["proposal_id"],
                "logical_call": evaluation["logical_call"],
                "phase": phase(evaluation["logical_call"]),
                "status": evaluation["status"],
                "usable": evaluation["usable_by_optimizer"],
                "selected": proposal["proposal_id"] in selected,
                "selection_weight": selected.get(proposal["proposal_id"]),
                "normalized_displacement_l2": distance,
                "sys": evaluation["sys"],
                "best_sys_before": round_row["best_sys_before"],
                "delta_vs_best_before": delta_best_before,
                "baseline_evaluation_id": proposal["baseline_evaluation_id"],
                "delta_vs_baseline": delta_baseline,
                "local_slope": (
                    None
                    if delta_baseline is None or distance in (None, 0.0)
                    else delta_baseline / distance
                ),
                "algorithm_fields": json.dumps(
                    proposal["algorithm_fields"], sort_keys=True
                ),
            }
        )
    grouped: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in long_rows:
        grouped[(row["algorithm_id"], row["phase"])].append(row)
    summary = []
    for (algorithm_id, phase_name), values in sorted(grouped.items()):
        distances = [
            row["normalized_displacement_l2"]
            for row in values
            if row["normalized_displacement_l2"] is not None
        ]
        global_deltas = [
            row["delta_vs_best_before"]
            for row in values
            if row["delta_vs_best_before"] is not None
        ]
        local_deltas = [
            row["delta_vs_baseline"]
            for row in values
            if row["delta_vs_baseline"] is not None
        ]
        local_slopes = [
            row["local_slope"] for row in values if row["local_slope"] is not None
        ]
        summary.append(
            {
                "algorithm_id": algorithm_id,
                "phase": phase_name,
                "proposal_count": len(values),
                "usable_fraction": sum(row["usable"] for row in values) / len(values),
                "selected_fraction": sum(row["selected"] for row in values)
                / len(values),
                "global_improvement_fraction": sum(
                    row["delta_vs_best_before"] is not None
                    and row["delta_vs_best_before"] > 0.0
                    for row in values
                )
                / len(values),
                "median_normalized_distance": (
                    None if not distances else float(np.median(distances))
                ),
                "q90_normalized_distance": (
                    None if not distances else percentile(distances, 0.90)
                ),
                "median_delta_vs_best_before": (
                    None if not global_deltas else float(np.median(global_deltas))
                ),
                "median_delta_vs_baseline": (
                    None if not local_deltas else float(np.median(local_deltas))
                ),
                "median_local_slope": (
                    None if not local_slopes else float(np.median(local_slopes))
                ),
            }
        )
    round_output = []
    for round_row in rounds:
        run = run_by_id[round_row["run_id"]]
        proposal_evaluations = [
            evaluation_by_proposal[proposal_id]
            for proposal_id in round_row["proposal_ids"]
        ]
        evaluator_ms = sum(row["total_ms"] for row in proposal_evaluations)
        call_count = (
            round_row["charged_calls_after"] - round_row["charged_calls_before"]
        )
        round_output.append(
            {
                "run_id": round_row["run_id"],
                "start_id": run["start_id"],
                "algorithm_id": run["algorithm_id"],
                "round_id": round_row["round_id"],
                "round_index": round_row["round_index"],
                "phase": phase(round_row["charged_calls_after"]),
                "charged_calls_before": round_row["charged_calls_before"],
                "charged_calls_after": round_row["charged_calls_after"],
                "call_count": call_count,
                "best_sys_before": round_row["best_sys_before"],
                "best_sys_after": round_row["best_sys_after"],
                "delta_best_sys": round_row["best_sys_after"]
                - round_row["best_sys_before"],
                "delta_best_per_call": (
                    round_row["best_sys_after"] - round_row["best_sys_before"]
                )
                / call_count,
                "evaluator_ms": evaluator_ms,
                "delta_best_per_evaluator_ms": (
                    round_row["best_sys_after"] - round_row["best_sys_before"]
                )
                / evaluator_ms,
                "ask_ms": round_row["ask_ms"],
                "tell_ms": round_row["tell_ms"],
                "selected_count": len(round_row["selected"]),
                "algorithm_fields": json.dumps(
                    round_row["algorithm_fields"], sort_keys=True
                ),
            }
        )
    return long_rows, summary, round_output


def profile_rows(
    runs: list[dict[str, Any]],
    rounds: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    algorithm_by_run = {row["run_id"]: row["algorithm_id"] for row in runs}
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for evaluation in evaluations:
        grouped[algorithm_by_run[evaluation["run_id"]]].append(evaluation)
    output = []
    for algorithm_id, rows in sorted(grouped.items()):
        physical = [row for row in rows if row["cache_status"] == "miss"]
        algorithm_runs = [row for row in runs if row["algorithm_id"] == algorithm_id]
        algorithm_rounds = [
            row
            for row in rounds
            if algorithm_by_run[row["run_id"]] == algorithm_id
        ]
        total = sum(row["total_ms"] for row in physical)
        geometry = sum(row["geometry_ms"] for row in physical)
        volume = sum(row["volume_ms"] for row in physical)
        capacity = sum(row["capacity_ms"] for row in physical)
        ask = sum(row["ask_ms"] for row in algorithm_rounds)
        tell = sum(row["tell_ms"] for row in algorithm_rounds)
        run_wall = sum(row["wall_ms"] for row in algorithm_runs)
        output.append(
            {
                "algorithm_id": algorithm_id,
                "physical_evaluations": len(physical),
                "median_evaluation_ms": float(
                    np.median([row["total_ms"] for row in physical])
                ),
                "q90_evaluation_ms": percentile(
                    [row["total_ms"] for row in physical], 0.90
                ),
                "total_evaluator_ms": total,
                "geometry_ms": geometry,
                "volume_ms": volume,
                "capacity_ms": capacity,
                "ask_ms": ask,
                "tell_ms": tell,
                "run_wall_ms": run_wall,
                "other_runner_ms": run_wall - total - ask - tell,
                "unattributed_ms": total - geometry - volume - capacity,
                "geometry_fraction": geometry / total,
                "volume_fraction": volume / total,
                "capacity_fraction": capacity / total,
                "evaluator_fraction_of_run_wall": total / run_wall,
                "ask_tell_fraction_of_run_wall": (ask + tell) / run_wall,
            }
        )
    return output


def select_checkpoints(
    artifact_dir: Path,
    plan: dict[str, Any],
    runs: list[dict[str, Any]],
    evaluations: list[dict[str, Any]],
    per_run: dict[str, list[dict[str, Any]]],
) -> dict[str, Any]:
    evaluation_by_id = unique_by(evaluations, "evaluation_id")
    finals_by_start: dict[str, list[float]] = defaultdict(list)
    for run in runs:
        finals_by_start[run["start_id"]].append(run["best_sys"])
    ranked_starts = sorted(
        finals_by_start,
        key=lambda start_id: (
            float(np.median(finals_by_start[start_id])),
            start_id,
        ),
    )
    requested_count = plan.get("probe_start_count")
    selected_count = len(ranked_starts) if requested_count is None else requested_count
    facet_by_start = {row["name"]: row["facet_count"] for row in plan["starts"]}
    facet_counts = sorted({facet_by_start[start] for start in ranked_starts})
    if selected_count == len(ranked_starts):
        selected_start_ids = ranked_starts
        selection_strategy = "all_starts"
    else:
        if selected_count % len(facet_counts) != 0:
            raise ValueError(
                "probe_start_count must be divisible by the number of facet counts"
            )
        per_facet = selected_count // len(facet_counts)
        selected_start_ids = []
        for facet_count in facet_counts:
            candidates = [
                start
                for start in ranked_starts
                if facet_by_start[start] == facet_count
            ]
            if per_facet > len(candidates):
                raise ValueError(
                    f"probe_start_count requests {per_facet} F={facet_count} "
                    f"starts but only {len(candidates)} exist"
                )
            indices = np.rint(
                np.linspace(0, len(candidates) - 1, per_facet)
            ).astype(int)
            if len(set(indices.tolist())) != per_facet:
                raise ValueError(
                    "facet-stratified checkpoint selection repeated an index"
                )
            selected_start_ids.extend(candidates[index] for index in indices)
        selection_strategy = (
            "even_quantiles_of_median_final_sys_within_each_facet_count"
        )
    selected_start_set = set(selected_start_ids)
    checkpoints = []
    for run in sorted(runs, key=lambda row: row["run_id"]):
        if run["start_id"] not in selected_start_set:
            continue
        for call in plan["checkpoints"]:
            eligible = [
                row for row in per_run[run["run_id"]] if row["call"] <= call
            ]
            state = eligible[-1]
            evaluation = evaluation_by_id[state["best_evaluation_id"]]
            checkpoints.append(
                {
                    "checkpoint_id": f"{run['run_id']}--c{call:06}",
                    "run_id": run["run_id"],
                    "algorithm_id": run["algorithm_id"],
                    "checkpoint_call": call,
                    "evaluation_id": evaluation["evaluation_id"],
                    "base_sys": evaluation["sys"],
                    "dual_flat": evaluation["dual_flat"],
                }
            )
    return {
        "schema_version": SCHEMA_VERSION,
        "source_artifact_dir": str(artifact_dir.resolve()),
        "radii": [0.0003, 0.003, 0.03, 0.3],
        "random_direction_count": 4,
        "selection_strategy": selection_strategy,
        "population_start_count": len(ranked_starts),
        "population_start_count_by_facet": {
            facet_count: sum(
                facet_by_start[start] == facet_count for start in ranked_starts
            )
            for facet_count in facet_counts
        },
        "selected_start_ids": selected_start_ids,
        "checkpoints": checkpoints,
    }


def plot_curves(
    artifact_dir: Path,
    call_rows: list[dict[str, Any]],
    compute_rows: list[dict[str, Any]],
    measured_compute_rows: list[dict[str, Any]],
    runs: list[dict[str, Any]],
) -> None:
    figures = artifact_dir / "figures"
    figures.mkdir(exist_ok=True)
    algorithms = sorted({row["algorithm_id"] for row in call_rows})
    color_map = plt.get_cmap("tab10")
    for x_name, rows, filename, x_label in [
        ("call", call_rows, "best-sys-by-call.png", "charged objective calls"),
        (
            "evaluator_ms",
            compute_rows,
            "best-sys-by-evaluator-time.png",
            "measured evaluator time (ms)",
        ),
        (
            "measured_compute_ms",
            measured_compute_rows,
            "best-sys-by-measured-compute.png",
            "measured evaluator + optimizer time (ms)",
        ),
    ]:
        figure, axis = plt.subplots(figsize=(10, 6))
        for index, algorithm_id in enumerate(algorithms):
            subset = [row for row in rows if row["algorithm_id"] == algorithm_id]
            x = np.asarray([row[x_name] for row in subset])
            median = np.asarray([row["median_best_sys"] for row in subset])
            low = np.asarray([row["q10_best_sys"] for row in subset])
            high = np.asarray([row["q90_best_sys"] for row in subset])
            color = color_map(index % 10)
            axis.plot(x, median, label=display_algorithm(algorithm_id), color=color)
            axis.fill_between(x, low, high, color=color, alpha=0.12)
        axis.set_xlabel(x_label)
        axis.set_ylabel("best recorded evaluator value")
        axis.grid(alpha=0.25)
        axis.legend(fontsize=7, ncol=2)
        figure.tight_layout()
        figure.savefig(figures / filename, dpi=160)
        plt.close(figure)
    grouped: dict[str, list[float]] = defaultdict(list)
    for run in runs:
        grouped[run["algorithm_id"]].append(run["best_sys"])
    figure, axis = plt.subplots(figsize=(max(9, len(grouped) * 0.9), 6))
    labels = sorted(grouped)
    axis.boxplot(
        [grouped[label] for label in labels],
        tick_labels=[display_algorithm(label) for label in labels],
        showfliers=True,
    )
    axis.set_ylabel("final best evaluator value")
    axis.tick_params(axis="x", labelrotation=55)
    axis.grid(axis="y", alpha=0.25)
    figure.tight_layout()
    figure.savefig(figures / "final-best-sys.png", dpi=160)
    plt.close(figure)


def plot_facet_call_curves(
    artifact_dir: Path, facet_rows: list[dict[str, Any]]
) -> None:
    figures = artifact_dir / "figures"
    figures.mkdir(exist_ok=True)
    facet_counts = sorted({row["facet_count"] for row in facet_rows})
    algorithms = sorted({row["algorithm_id"] for row in facet_rows})
    figure, axes = plt.subplots(
        len(facet_counts),
        1,
        figsize=(10, 4.2 * len(facet_counts)),
        squeeze=False,
        sharex=True,
    )
    color_map = plt.get_cmap("tab20")
    for row_index, facet_count in enumerate(facet_counts):
        axis = axes[row_index, 0]
        for algorithm_index, algorithm_id in enumerate(algorithms):
            subset = [
                row
                for row in facet_rows
                if row["facet_count"] == facet_count
                and row["algorithm_id"] == algorithm_id
            ]
            axis.plot(
                [row["call"] for row in subset],
                [row["median_best_sys"] for row in subset],
                label=display_algorithm(algorithm_id),
                color=color_map(algorithm_index % 20),
            )
        axis.set_title(f"F={facet_count}")
        axis.set_ylabel("median best evaluator value")
        axis.grid(alpha=0.25)
    axes[-1, 0].set_xlabel("charged objective calls")
    axes[0, 0].legend(fontsize=6, ncol=3)
    figure.tight_layout()
    figure.savefig(figures / "best-sys-by-call-and-facet.png", dpi=180)
    plt.close(figure)


def summary_markdown(
    diagnostics: dict[str, Any],
    final: list[dict[str, Any]],
    run_stops: list[dict[str, Any]],
    profile: list[dict[str, Any]],
    checkpoint_curves: list[dict[str, Any]],
    facet_final: list[dict[str, Any]],
    portfolios: list[dict[str, Any]],
    mode: str,
) -> str:
    ranked = sorted(final, key=lambda row: row["median_final_sys"], reverse=True)
    lines = [
        "# Optimizer trajectory analysis",
        "",
        (
            f"Strict validation passed for {diagnostics['run_count']} runs, "
            f"{diagnostics['evaluation_count']} evaluations, "
            f"{diagnostics['proposal_count']} proposals, and "
            f"{diagnostics['round_count']} rounds."
        ),
        "",
        (
            "`sys` in the tables and filenames below is the recorded field of "
            "this dataset's configured evaluator. It is not, by this analysis "
            "alone, a certified value of the mathematical systolic ratio."
        ),
        "",
        "Facet-count-specific results are primary. The pooled table below is "
        "descriptive only: it must not override a reversal or failure at any "
        "individual facet count.",
        "",
        "## Best median by facet count",
        "",
        "| facets | best algorithm | median final sys | n |",
        "|---:|---|---:|---:|",
    ]
    for facet_count in sorted({row["facet_count"] for row in facet_final}):
        best = max(
            (row for row in facet_final if row["facet_count"] == facet_count),
            key=lambda row: row["median_final_sys"],
        )
        lines.append(
            f"| {facet_count} | {best['algorithm_id']} | "
            f"{best['median_final_sys']:.6g} | {best['n']} |"
        )
    lines += [
        "",
        "## Pooled final outcomes (secondary)",
        "",
        "| algorithm | n | median final sys | 10–90% | invalid | exact fallback |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for row in ranked:
        lines.append(
            f"| {row['algorithm_id']} | {row['n']} | "
            f"{row['median_final_sys']:.6g} | "
            f"{row['q10_final_sys']:.6g}–{row['q90_final_sys']:.6g} | "
            f"{row['invalid_rate']:.1%} | {row['exact_fallback_rate']:.1%} |"
        )
    lines += [
        "",
        "The 10–90% interval is the across-start distribution, not uncertainty "
        "about its median. `final-summary.csv` also contains bootstrap intervals.",
        "",
        "## Best median by budget",
        "",
        "| charged calls | best algorithm | median best sys |",
        "|---:|---|---:|",
    ]
    for call in sorted(
        {row["call"] for row in checkpoint_curves if row["call"] != 0}
    ):
        best = max(
            (row for row in checkpoint_curves if row["call"] == call),
            key=lambda row: row["median_best_sys"],
        )
        lines.append(
            f"| {call} | {best['algorithm_id']} | {best['median_best_sys']:.6g} |"
        )
    if portfolios:
        if mode == "final":
            portfolio_heading = (
                "## Post-hoc two-run portfolio on the held-out outcomes"
            )
            portfolio_note = (
                "This allocation was selected after seeing these held-out outcomes. "
                "It is descriptive only and would need a new independent population "
                "before it could be treated as a confirmed optimizer choice."
            )
        else:
            portfolio_heading = (
                "## Best derived two-run portfolio at the full budget"
            )
            portfolio_note = (
                "This is a tuning-data screen assembled from independent stored "
                "trajectories. It proposes configurations for confirmation; it is "
                "not held-out evidence."
            )
        lines += [
            "",
            portfolio_heading,
            "",
            portfolio_note,
            "",
            "| facets | allocation | mean regret to observed oracle | worst regret | within 0.01 |",
            "|---:|---|---:|---:|---:|",
        ]
        for facet_count in [
            "all",
            *sorted({row["facet_count"] for row in facet_final}),
        ]:
            best = min(
                (
                    row
                    for row in portfolios
                    if str(row["facet_count"]) == str(facet_count)
                ),
                key=lambda row: (
                    row["mean_oracle_regret"],
                    row["maximum_oracle_regret"],
                ),
            )
            lines.append(
                f"| {facet_count} | {best['left_calls']}×{best['left_algorithm_id']} "
                f"+ {best['right_calls']}×{best['right_algorithm_id']} | "
                f"{best['mean_oracle_regret']:.6g} | "
                f"{best['maximum_oracle_regret']:.6g} | "
                f"{best['within_0_01_of_oracle_fraction']:.1%} |"
            )
    lines += [
        "",
        "## Evaluator profile",
        "",
        "| algorithm | median eval ms | geometry | volume | capacity | evaluator / run wall | ask+tell / run wall |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for row in sorted(profile, key=lambda item: item["algorithm_id"]):
        lines.append(
            f"| {row['algorithm_id']} | {row['median_evaluation_ms']:.4g} | "
            f"{row['geometry_fraction']:.1%} | {row['volume_fraction']:.1%} | "
            f"{row['capacity_fraction']:.1%} | "
            f"{row['evaluator_fraction_of_run_wall']:.1%} | "
                f"{row['ask_tell_fraction_of_run_wall']:.1%} |"
        )
    lines += [
        "",
        "## Termination and observed maxima",
        "",
        "| algorithm | median charged calls | 10–90% calls | maximum best sys | runs reaching sys >= 1 | stop reasons |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for row in sorted(
        run_stops,
        key=lambda item: item["maximum_best_sys"],
        reverse=True,
    ):
        lines.append(
            f"| {row['algorithm_id']} | {row['median_charged_calls']:.4g} | "
            f"{row['q10_charged_calls']:.4g}–{row['q90_charged_calls']:.4g} | "
            f"{row['maximum_best_sys']:.9g} | {row['reached_sys_1_count']} | "
            f"{row['stop_reason_counts']} |"
        )
    companion_outputs = [
        "`best-sys-by-call.png`",
        "`best-sys-by-measured-compute.png`",
        "`best-sys-by-call-and-facet.png`",
    ]
    if len(final) > 1:
        companion_outputs.append("`paired-checkpoint-comparisons.csv`")
    companion_outputs.append("`checkpoint-selection.json`")
    lines += [
        "",
        "See "
        + ", ".join(companion_outputs)
        + " for the trajectory curves and independent-probe input.",
        "",
    ]
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument(
        "--mode",
        required=True,
        choices=("component", "tuning", "final", "development"),
    )
    args = parser.parse_args()
    dataset_dir = args.dataset
    output_dir = args.out
    if output_dir.exists():
        raise ValueError(f"output already exists: {output_dir}")
    output_dir.mkdir(parents=True)
    plan = load_json(dataset_dir / "resolved-plan.json")
    required_role = {"tuning": "tuning", "final": "held_out"}.get(args.mode)
    if required_role is not None and plan.get("dataset_role") != required_role:
        raise ValueError(
            f"{args.mode} analysis requires dataset_role={required_role!r}, "
            f"got {plan.get('dataset_role')!r}"
        )
    runs = load_jsonl(dataset_dir / "runs.jsonl")
    rounds = load_jsonl(dataset_dir / "rounds.jsonl")
    proposals = load_jsonl(dataset_dir / "proposals.jsonl")
    evaluations = load_jsonl(dataset_dir / "evaluations.jsonl")
    input_names = [
        "resolved-plan.json",
        "run-provenance.json",
        "runs.jsonl",
        "rounds.jsonl",
        "proposals.jsonl",
        "evaluations.jsonl",
    ]
    analysis_provenance = {
        "schema_version": SCHEMA_VERSION,
        "analysis_mode": args.mode,
        "analyzer": str(Path(__file__).resolve()),
        "analyzer_sha256": sha256_file(Path(__file__)),
        "input_sha256": {
            name: sha256_file(dataset_dir / name) for name in input_names
        },
    }
    diagnostics = validate_packet(plan, runs, rounds, proposals, evaluations)
    long_trajectory, per_run = trajectory_rows(plan, runs, evaluations)
    call_checkpoints = call_curve_rows(plan, runs, per_run)
    facet_call_checkpoints = facet_call_curve_rows(plan, runs, per_run)
    full_calls = full_call_curve_rows(plan, runs, per_run)
    compute = compute_curve_rows(runs, per_run)
    measured_compute = measured_compute_curve_rows(runs, rounds, evaluations)
    final = final_rows(runs)
    run_stops = run_stop_rows(runs)
    facet_final = facet_final_rows(plan, runs)
    facet_balanced_regret = facet_balanced_regret_rows(plan, runs, per_run)
    portfolios = portfolio_screen_rows(plan, runs, per_run)
    paired = paired_rows(runs)
    paired_checkpoints = paired_checkpoint_rows(plan, runs, per_run)
    thresholds = threshold_rows(plan, runs, per_run)
    iid_crossovers = iid_crossover_rows(full_calls)
    _proposal_long, proposal_summary, _round_behavior = proposal_behavior_rows(
        plan, runs, rounds, proposals, evaluations
    )
    profile = profile_rows(runs, rounds, evaluations)
    configurations = algorithm_configuration_rows(plan)
    checkpoint_selection = select_checkpoints(
        dataset_dir, plan, runs, evaluations, per_run
    )
    write_csv(output_dir / "curves-at-checkpoints.csv", call_checkpoints)
    write_csv(
        output_dir / "curves-at-checkpoints-by-facet.csv",
        facet_call_checkpoints,
    )
    write_csv(output_dir / "curves-by-call.csv", full_calls)
    write_csv(output_dir / "curves-by-evaluator-time.csv", compute)
    write_csv(output_dir / "curves-by-measured-compute.csv", measured_compute)
    write_csv(output_dir / "final-summary.csv", final)
    write_csv(output_dir / "run-stop-summary.csv", run_stops)
    write_csv(output_dir / "final-summary-by-facet.csv", facet_final)
    write_csv(
        output_dir / "facet-balanced-configuration-regret.csv",
        facet_balanced_regret,
    )
    if portfolios:
        write_csv(output_dir / "portfolio-screen.csv", portfolios)
    if paired:
        write_csv(output_dir / "paired-final-comparisons.csv", paired)
    if paired_checkpoints:
        write_csv(
            output_dir / "paired-checkpoint-comparisons.csv",
            paired_checkpoints,
        )
    write_csv(output_dir / "threshold-probabilities.csv", thresholds)
    if iid_crossovers:
        write_csv(output_dir / "iid-crossover.csv", iid_crossovers)
    write_csv(output_dir / "proposal-behavior-summary.csv", proposal_summary)
    write_csv(output_dir / "evaluator-profile.csv", profile)
    write_csv(output_dir / "algorithm-configurations.csv", configurations)
    with (output_dir / "checkpoint-selection.json").open("w") as stream:
        json.dump(checkpoint_selection, stream, indent=2)
        stream.write("\n")
    analysis = {
        "schema_version": SCHEMA_VERSION,
        "analysis_mode": args.mode,
        "dataset": str(dataset_dir.resolve()),
        "validation": diagnostics,
        "best_median_final_algorithm": max(
            final, key=lambda row: row["median_final_sys"]
        )["algorithm_id"],
        "common_compute_curve_maximum_ms": compute[-1]["evaluator_ms"],
        "checkpoint_count": len(checkpoint_selection["checkpoints"]),
    }
    with (output_dir / "analysis.json").open("w") as stream:
        json.dump(analysis, stream, indent=2)
        stream.write("\n")
    with (output_dir / "analysis-provenance.json").open("w") as stream:
        json.dump(analysis_provenance, stream, indent=2)
        stream.write("\n")
    plot_curves(output_dir, full_calls, compute, measured_compute, runs)
    plot_facet_call_curves(output_dir, facet_call_checkpoints)
    (output_dir / "SUMMARY.md").write_text(
        summary_markdown(
            diagnostics,
            final,
            run_stops,
            profile,
            call_checkpoints,
            facet_final,
            portfolios,
            args.mode,
        )
    )
    print(
        f"validated and analyzed {diagnostics['run_count']} runs; "
        f"wrote {len(checkpoint_selection['checkpoints'])} checkpoint selections"
    )


if __name__ == "__main__":
    main()
