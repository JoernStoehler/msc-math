# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Analyze matched predictor-atom replays at saved optimizer states."""

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


def finite(value) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(value)


def statistic(values, operation):
    values = np.asarray([value for value in values if finite(value)], dtype=float)
    return float(operation(values)) if len(values) else None


def selector_label(selector: str) -> str:
    if selector.startswith("anchor_action_window_"):
        return "action window " + selector.removeprefix("anchor_action_window_")
    return {
        "anchor_winner": "anchor winner",
        "anchor_transition_feasible_all": "all anchor-feasible",
        "target_winner_oracle": "target winner control",
        "target_effective_all_oracle": "all target-feasible control",
    }.get(selector, selector)


def value_label(value_model: str) -> str:
    return {
        "named_branch_kkt_at_target": "branches reevaluated at target",
        "affine_named_branches_at_anchor": "affine anchor model",
    }.get(value_model, value_model)


def summarize(rows: list[dict]) -> list[dict]:
    groups = defaultdict(list)
    for row in rows:
        groups[
            (
                row["algorithm_id"],
                row["trajectory_phase"],
                row["distance_scale"],
                row["selector"],
                row["value_model"],
                row["domain_model"],
            )
        ].append(row)
    summaries = []
    for key, group in sorted(groups.items()):
        algorithm, phase, scale, selector, value_model, domain_model = key
        errors = [
            abs(row["prediction_error"])
            for row in group
            if finite(row.get("prediction_error"))
        ]
        compute = [
            sum(
                float(row.get(field, 0.0))
                for field in ("geometry_ms", "volume_ms", "named_branch_ms", "model_ms")
            )
            for row in group
        ]
        summaries.append(
            {
                "algorithm_id": algorithm,
                "trajectory_phase": phase,
                "distance_scale": scale,
                "selector": selector,
                "value_model": value_model,
                "domain_model": domain_model,
                "pairs": len(group),
                "predictions": len(errors),
                "prediction_rate": len(errors) / len(group),
                "accepted_rate": statistic(
                    [float(row["accepted_by_optimizer"]) for row in group], np.mean
                ),
                "median_normalized_distance": statistic(
                    [row["normalized_distance"] for row in group], np.median
                ),
                "median_candidate_count": statistic(
                    [row["candidate_count"] for row in group], np.median
                ),
                "median_represented_branch_count": statistic(
                    [row["represented_branch_count"] for row in group], np.median
                ),
                "winner_coverage": statistic(
                    [
                        float(row["target_winner_covered"])
                        for row in group
                        if row.get("target_winner_covered") is not None
                    ],
                    np.mean,
                ),
                "median_abs_error": statistic(errors, np.median),
                "q90_abs_error": statistic(errors, lambda x: np.quantile(x, 0.9)),
                "fraction_usable_and_error_le_1e_3": sum(
                    finite(row.get("prediction_error"))
                    and abs(row["prediction_error"]) <= 1e-3
                    for row in group
                )
                / len(group),
                "fraction_usable_and_error_le_1e_2": sum(
                    finite(row.get("prediction_error"))
                    and abs(row["prediction_error"]) <= 1e-2
                    for row in group
                )
                / len(group),
                "mean_signed_error_actual_minus_prediction": statistic(
                    [row["prediction_error"] for row in group], np.mean
                ),
                "sign_accuracy": statistic(
                    [
                        float(row["sign_correct"])
                        for row in group
                        if row.get("sign_correct") is not None
                    ],
                    np.mean,
                ),
                "median_compute_ms": statistic(compute, np.median),
            }
        )
    return summaries


def aggregate(rows: list[dict]) -> list[dict]:
    groups = defaultdict(list)
    for row in rows:
        groups[(row["distance_scale"], row["selector"], row["value_model"])].append(row)
    result = []
    for (scale, selector, value_model), group in sorted(groups.items()):
        errors = [
            abs(row["prediction_error"])
            for row in group
            if finite(row.get("prediction_error"))
        ]
        compute = [
            sum(
                float(row.get(field, 0.0))
                for field in ("geometry_ms", "volume_ms", "named_branch_ms", "model_ms")
            )
            for row in group
        ]
        result.append(
            {
                "distance_scale": scale,
                "selector": selector,
                "value_model": value_model,
                "pairs": len(group),
                "prediction_rate": len(errors) / len(group),
                "accepted_rate": statistic(
                    [float(row["accepted_by_optimizer"]) for row in group], np.mean
                ),
                "median_distance": statistic(
                    [row["normalized_distance"] for row in group], np.median
                ),
                "median_candidates": statistic(
                    [row["candidate_count"] for row in group], np.median
                ),
                "winner_coverage": statistic(
                    [
                        float(row["target_winner_covered"])
                        for row in group
                        if row.get("target_winner_covered") is not None
                    ],
                    np.mean,
                ),
                "median_abs_error": statistic(errors, np.median),
                "q90_abs_error": statistic(errors, lambda x: np.quantile(x, 0.9)),
                "fraction_usable_and_error_le_1e_3": sum(
                    finite(row.get("prediction_error"))
                    and abs(row["prediction_error"]) <= 1e-3
                    for row in group
                )
                / len(group),
                "fraction_usable_and_error_le_1e_2": sum(
                    finite(row.get("prediction_error"))
                    and abs(row["prediction_error"]) <= 1e-2
                    for row in group
                )
                / len(group),
                "sign_accuracy": statistic(
                    [
                        float(row["sign_correct"])
                        for row in group
                        if row.get("sign_correct") is not None
                    ],
                    np.mean,
                ),
                "median_compute_ms": statistic(compute, np.median),
            }
        )
    return result


def write_csv(path: Path, rows: list[dict]) -> None:
    if not rows:
        raise ValueError(f"cannot write empty table: {path}")
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def plot_error_by_distance(rows: list[dict], path: Path) -> None:
    keep = {
        "anchor_winner",
        "anchor_action_window_0.100000",
        "anchor_transition_feasible_all",
        "target_winner_oracle",
    }
    figure, axes = plt.subplots(1, 2, figsize=(12.0, 4.6), sharex=True, sharey=True)
    for axis, value_model in zip(
        axes, ("named_branch_kkt_at_target", "affine_named_branches_at_anchor")
    ):
        for selector in sorted(keep):
            group = [
                row
                for row in rows
                if row["selector"] == selector
                and row["value_model"] == value_model
                and finite(row.get("normalized_distance"))
                and finite(row.get("prediction_error"))
            ]
            if not group:
                continue
            axis.scatter(
                [row["normalized_distance"] for row in group],
                [abs(row["prediction_error"]) for row in group],
                s=10,
                alpha=0.32,
                label=selector_label(selector),
            )
        axis.set_xscale("log")
        axis.set_yscale("symlog", linthresh=1e-10)
        axis.set_title(value_label(value_model))
        axis.set_xlabel("normalized distance from saved state")
        axis.grid(alpha=0.2)
    axes[0].set_ylabel("absolute error in predicted sys")
    axes[1].legend(fontsize=7)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_window_tradeoff(rows: list[dict], path: Path) -> None:
    group = [
        row
        for row in rows
        if row["selector"].startswith("anchor_action_window_")
        and row["value_model"] == "named_branch_kkt_at_target"
        and finite(row.get("prediction_error"))
    ]
    figure, axis = plt.subplots(figsize=(7.2, 4.8))
    for scale in sorted({row["distance_scale"] for row in group}):
        scaled = [row for row in group if row["distance_scale"] == scale]
        by_selector = defaultdict(list)
        for row in scaled:
            by_selector[row["selector"]].append(row)
        points = []
        for selector, selected in by_selector.items():
            points.append(
                (
                    statistic([row["candidate_count"] for row in selected], np.median),
                    statistic(
                        [abs(row["prediction_error"]) for row in selected],
                        lambda x: np.quantile(x, 0.9),
                    ),
                    selector,
                    statistic(
                        [float(row["target_winner_covered"]) for row in selected],
                        np.mean,
                    ),
                )
            )
        points.sort()
        axis.plot(
            [point[0] for point in points],
            [point[1] for point in points],
            marker="o",
            label=f"distance scale {scale:g}",
        )
        for candidates, error, selector, coverage in points:
            window = float(selector.removeprefix("anchor_action_window_"))
            axis.annotate(
                f"{window:g}; {coverage:.0%}",
                (candidates, error),
                xytext=(3, 3),
                textcoords="offset points",
                fontsize=7,
            )
    axis.set_yscale("symlog", linthresh=1e-10)
    axis.set_xlabel("median candidate count (label: window; winner coverage)")
    axis.set_ylabel("90th percentile absolute error in sys")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_cost_error(rows: list[dict], path: Path) -> None:
    aggregate_rows = aggregate(rows)
    figure, axis = plt.subplots(figsize=(8.0, 5.2))
    markers = {
        "named_branch_kkt_at_target": "o",
        "affine_named_branches_at_anchor": "x",
    }
    for value_model, marker in markers.items():
        group = [
            row
            for row in aggregate_rows
            if row["value_model"] == value_model
            and finite(row["median_compute_ms"])
            and finite(row["q90_abs_error"])
        ]
        axis.scatter(
            [row["median_compute_ms"] for row in group],
            [row["q90_abs_error"] for row in group],
            marker=marker,
            alpha=0.7,
            label=value_label(value_model),
        )
    axis.set_xscale("symlog", linthresh=1e-3)
    axis.set_yscale("symlog", linthresh=1e-10)
    axis.set_xlabel("median measured predictor compute (ms)")
    axis.set_ylabel("90th percentile absolute error in sys")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def fmt(value, digits=4) -> str:
    if value is None or not finite(value):
        return "—"
    return f"{value:.{digits}g}"


def write_summary(path: Path, rows: list[dict], aggregate_rows: list[dict]) -> None:
    controls = [
        row
        for row in rows
        if row["selector"] == "target_effective_all_oracle"
        and row["value_model"] == "named_branch_kkt_at_target"
        and finite(row.get("prediction_error"))
    ]
    max_control_error = max(
        (abs(row["prediction_error"]) for row in controls), default=math.inf
    )
    if max_control_error > 1e-9:
        raise ValueError(
            "target-effective-all direct reevaluation failed to reproduce target sys: "
            f"maximum error {max_control_error}"
        )
    lines = [
        "# Predictor replay analysis",
        "",
        (
            f"The direct target reevaluation control reproduced `sys` on all "
            f"{len(controls)} usable targets (maximum absolute discrepancy "
            f"{max_control_error:.3g})."
        ),
        "",
        "The table reports pooled development data. Rows remain paired by saved "
        "state in `atoms.jsonl`; this summary is not a complete-optimizer comparison.",
        "",
        "| distance scale | candidate set | branch values | median candidates | usable | winner coverage | median error | 90% error | within 0.01 | sign accuracy | median ms |",
        "|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for row in aggregate_rows:
        if row["selector"] == "target_effective_all_oracle":
            continue
        lines.append(
            "| {distance_scale:g} | {selector} | {value_model} | {candidates} | "
            "{usable} | {coverage} | {median_error} | {q90_error} | {within} | "
            "{sign_accuracy} | {cost} |".format(
                distance_scale=row["distance_scale"],
                selector=selector_label(row["selector"]),
                value_model=value_label(row["value_model"]),
                candidates=fmt(row["median_candidates"]),
                usable=fmt(row["prediction_rate"]),
                coverage=fmt(row["winner_coverage"]),
                median_error=fmt(row["median_abs_error"]),
                q90_error=fmt(row["q90_abs_error"]),
                within=fmt(row["fraction_usable_and_error_le_1e_2"]),
                sign_accuracy=fmt(row["sign_accuracy"]),
                cost=fmt(row["median_compute_ms"]),
            )
        )
    lines.extend(
        [
            "",
            "Interpret the controls in order:",
            "",
            "- Direct target reevaluation with an anchor-selected set isolates candidate-set staleness.",
            "- Replacing those target reevaluations by anchor affine models adds branch-value and constant-domain approximation.",
            "- The target-winner control removes candidate selection error for one branch.",
            "- Errors are reported in `sys` units; optimizer value still depends on realized improvement per measured compute.",
            "",
            "See `error-vs-distance.png`, `window-tradeoff.png`, and `cost-vs-error.png`.",
        ]
    )
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    if args.out.exists():
        raise ValueError(f"output already exists: {args.out}")
    rows = read_jsonl(args.dataset / "atoms.jsonl")
    if not rows:
        raise ValueError("replay dataset contains no atom rows")
    summaries = summarize(rows)
    aggregate_rows = aggregate(rows)
    args.out.mkdir(parents=True)
    write_csv(args.out / "summary-by-algorithm.csv", summaries)
    write_csv(args.out / "summary-pooled.csv", aggregate_rows)
    plot_error_by_distance(rows, args.out / "error-vs-distance.png")
    plot_window_tradeoff(rows, args.out / "window-tradeoff.png")
    plot_cost_error(rows, args.out / "cost-vs-error.png")
    write_summary(args.out / "SUMMARY.md", rows, aggregate_rows)
    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "dataset": str(args.dataset),
                "atom_rows": len(rows),
                "algorithms": sorted({row["algorithm_id"] for row in rows}),
                "distance_scales": sorted({row["distance_scale"] for row in rows}),
                "claim_boundary": (
                    "Matched replay diagnoses predictor components at saved states; "
                    "it does not establish full-trajectory performance."
                ),
            },
            indent=2,
        )
        + "\n"
    )
    print(f"analyzed {len(rows)} predictor-atom rows")


if __name__ == "__main__":
    main()
