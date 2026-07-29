# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Explain on-trajectory optimizer steps using the shared trace tables."""

from __future__ import annotations

import argparse
import csv
import json
import math
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def finite(value):
    return value is not None and isinstance(value, (int, float)) and math.isfinite(value)


def nested_number(row: dict, *keys: str):
    value = row
    for key in keys:
        if not isinstance(value, dict) or key not in value:
            return None
        value = value[key]
    return float(value) if finite(value) else None


def trace_rows(dataset: Path) -> list[dict]:
    runs = {row["run_id"]: row for row in read_jsonl(dataset / "runs.jsonl")}
    evaluations = {
        row["evaluation_id"]: row for row in read_jsonl(dataset / "evaluations.jsonl")
    }
    proposals = {
        row["proposal_id"]: row for row in read_jsonl(dataset / "proposals.jsonl")
    }
    records = []
    for round_row in read_jsonl(dataset / "rounds.jsonl"):
        run = runs[round_row["run_id"]]
        phase = (
            "early"
            if round_row["round_index"] < max(1, run["rounds"] / 3)
            else "middle"
            if round_row["round_index"] < max(2, 2 * run["rounds"] / 3)
            else "late"
        )
        selected_ids = {
            selected["proposal_id"] for selected in round_row.get("selected", [])
        }
        proposal_count = max(len(round_row["proposal_ids"]), 1)
        for proposal_id in round_row["proposal_ids"]:
            proposal = proposals[proposal_id]
            target = evaluations[proposal["evaluation_id"]]
            fields = proposal.get("algorithm_fields") or {}
            outcome = round_row.get("algorithm_fields") or {}
            predicted_delta = nested_number(fields, "predicted_delta")
            if predicted_delta is None:
                predicted_delta = nested_number(outcome, "predicted_delta")
            observed_delta = nested_number(outcome, "observed_delta")
            if observed_delta is None and finite(target.get("sys")):
                observed_delta = float(target["sys"]) - float(round_row["best_sys_before"])
            target_winner = target.get("winning_sigma")
            predicted_winner = fields.get("predicted_winning_sigma")
            candidate_covered = outcome.get("target_winner_in_candidate_set")
            distance = proposal.get("normalized_displacement_l2")
            round_compute = (
                float(round_row.get("charged_compute_ms_after", 0.0))
                - float(round_row.get("charged_compute_ms_before", 0.0))
            )
            if round_compute <= 0.0:
                round_compute = (
                    float(round_row.get("ask_ms", 0.0))
                    + float(round_row.get("tell_ms", 0.0))
                    + float(target.get("total_ms", 0.0))
                )
            baseline_id = proposal.get("baseline_evaluation_id")
            baseline_sys = (
                evaluations[baseline_id].get("sys")
                if baseline_id in evaluations
                else round_row["best_sys_before"]
            )
            validated_delta = (
                float(target["sys"]) - float(baseline_sys)
                if finite(target.get("sys")) and finite(baseline_sys)
                else None
            )
            records.append(
                {
                    "run_id": run["run_id"],
                    "start_id": run["start_id"],
                    "algorithm_id": run["algorithm_id"],
                    "round_index": round_row["round_index"],
                    "phase": phase,
                    "distance": distance,
                    "predicted_delta": predicted_delta,
                    "observed_delta": observed_delta,
                    "prediction_error": (
                        observed_delta - predicted_delta
                        if finite(observed_delta) and finite(predicted_delta)
                        else None
                    ),
                    "sign_correct": (
                        (observed_delta > 0) == (predicted_delta > 0)
                        if finite(observed_delta) and finite(predicted_delta)
                        else None
                    ),
                    "candidate_covered": candidate_covered,
                    "winner_prediction_correct": (
                        predicted_winner == target_winner
                        if predicted_winner is not None and target_winner is not None
                        else None
                    ),
                    "candidate_count": fields.get(
                        "candidate_count", fields.get("candidate_branch_count")
                    ),
                    "selected": proposal_id in selected_ids,
                    "validated_delta": validated_delta,
                    "round_best_gain": float(round_row["best_sys_after"])
                    - float(round_row["best_sys_before"]),
                    "proposal_compute_ms": round_compute / proposal_count,
                    "cumulative_compute_ms": float(
                        round_row.get("charged_compute_ms_after", 0.0)
                    ),
                    "geometry_ms": float(target.get("geometry_ms", 0.0)),
                    "volume_ms": float(target.get("volume_ms", 0.0)),
                    "capacity_ms": float(target.get("capacity_ms", 0.0)),
                    "ask_ms": float(round_row.get("ask_ms", 0.0)),
                    "tell_ms": float(round_row.get("tell_ms", 0.0)),
                }
            )
    return records


def statistic(values, operation):
    values = np.array([value for value in values if finite(value)], dtype=float)
    return float(operation(values)) if len(values) else None


def summarize(records: list[dict]) -> list[dict]:
    groups = defaultdict(list)
    for row in records:
        groups[(row["algorithm_id"], row["phase"])].append(row)
    summaries = []
    for (algorithm, phase), rows in sorted(groups.items()):
        errors = [abs(row["prediction_error"]) for row in rows if finite(row["prediction_error"])]
        summaries.append(
            {
                "algorithm_id": algorithm,
                "phase": phase,
                "steps": len(rows),
                "starts": len({row["start_id"] for row in rows}),
                "median_distance": statistic(
                    [row["distance"] for row in rows], np.median
                ),
                "median_abs_prediction_error": statistic(errors, np.median),
                "q90_abs_prediction_error": statistic(errors, lambda x: np.quantile(x, 0.9)),
                "sign_accuracy": statistic(
                    [
                        float(row["sign_correct"])
                        for row in rows
                        if row["sign_correct"] is not None
                    ],
                    np.mean,
                ),
                "candidate_coverage": statistic(
                    [
                        float(row["candidate_covered"])
                        for row in rows
                        if row["candidate_covered"] is not None
                    ],
                    np.mean,
                ),
                "selection_rate": statistic(
                    [float(row["selected"]) for row in rows], np.mean
                ),
                "median_validated_gain": statistic(
                    [row["validated_delta"] for row in rows], np.median
                ),
                "median_proposal_compute_ms": statistic(
                    [row["proposal_compute_ms"] for row in rows], np.median
                ),
                "median_candidate_count": statistic(
                    [row["candidate_count"] for row in rows], np.median
                ),
            }
        )
    return summaries


def write_csv(path: Path, rows: list[dict]) -> None:
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def plot_error(records: list[dict], path: Path) -> None:
    figure, axis = plt.subplots(figsize=(7.2, 4.8))
    for algorithm in sorted({row["algorithm_id"] for row in records}):
        group = [
            row
            for row in records
            if row["algorithm_id"] == algorithm
            and finite(row["distance"])
            and finite(row["prediction_error"])
        ]
        if group:
            axis.scatter(
                [row["distance"] for row in group],
                [abs(row["prediction_error"]) for row in group],
                s=9,
                alpha=0.35,
                label=algorithm,
            )
    axis.set_xscale("log")
    axis.set_yscale("symlog", linthresh=1e-10)
    axis.set_xlabel("realized normalized distance")
    axis.set_ylabel("absolute prediction error")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=7)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_cost(records: list[dict], path: Path) -> None:
    labels = sorted({row["algorithm_id"] for row in records})
    components = ("ask_ms", "geometry_ms", "volume_ms", "capacity_ms", "tell_ms")
    values = {
        component: [
            statistic(
                [row[component] for row in records if row["algorithm_id"] == label],
                np.median,
            )
            or 0.0
            for label in labels
        ]
        for component in components
    }
    figure, axis = plt.subplots(figsize=(max(7.0, len(labels) * 0.9), 4.8))
    bottom = np.zeros(len(labels))
    for component in components:
        axis.bar(labels, values[component], bottom=bottom, label=component)
        bottom += np.array(values[component])
    axis.set_ylabel("median milliseconds per recorded proposal")
    axis.tick_params(axis="x", rotation=35)
    axis.legend(fontsize=7)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    if args.out.exists():
        raise ValueError(f"output already exists: {args.out}")
    records = trace_rows(args.dataset)
    if not records:
        raise ValueError("dataset contains no proposal rounds")
    summaries = summarize(records)
    args.out.mkdir(parents=True)
    write_csv(args.out / "atom-summary.csv", summaries)
    write_csv(args.out / "atom-rows.csv", records)
    plot_error(records, args.out / "prediction-error-vs-distance.png")
    plot_cost(records, args.out / "step-cost-breakdown.png")
    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "dataset": str(args.dataset),
                "rows": len(records),
                "algorithms": sorted({row["algorithm_id"] for row in records}),
                "claim_boundary": (
                    "On-trajectory diagnostics explain recorded proposals; "
                    "they do not establish complete-optimizer superiority."
                ),
            },
            indent=2,
        )
        + "\n"
    )
    print(f"analyzed {len(records)} on-trajectory proposals")


if __name__ == "__main__":
    main()
