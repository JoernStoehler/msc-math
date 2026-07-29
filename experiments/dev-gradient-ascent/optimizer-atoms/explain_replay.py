# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Produce a question-oriented report from a retained predictor replay."""

# ruff: noqa: ISC004

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

DIRECT = "named_branch_kkt_at_target"
AFFINE = "affine_named_branches_at_anchor"
TARGET_WINNER = "target_winner_oracle"
TARGET_ALL = "target_effective_all_oracle"
ANCHOR_ALL = "anchor_transition_feasible_all"
WINDOWS = [
    "anchor_action_window_0.010000",
    "anchor_action_window_0.100000",
    "anchor_action_window_0.300000",
    "anchor_action_window_1.000000",
]
SELECTOR_LABELS = {
    "anchor_winner": "anchor winner",
    WINDOWS[0]: "1% action window",
    WINDOWS[1]: "10% action window",
    WINDOWS[2]: "30% action window",
    WINDOWS[3]: "100% action window",
    ANCHOR_ALL: "all retained anchor germs",
    TARGET_WINNER: "target winner",
    TARGET_ALL: "all retained target germs",
}
BIN_EDGES = np.asarray([0.0, 1e-4, 1e-3, 1e-2, 0.05, 0.10, 0.20, np.inf])
BIN_LABELS = [
    "[0, 1e-4)",
    "[1e-4, 1e-3)",
    "[1e-3, 1e-2)",
    "[0.01, 0.05)",
    "[0.05, 0.10)",
    "[0.10, 0.20)",
    "[0.20, 0.434]",
]


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def finite(value) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(value)


def fmt(value, digits: int = 4) -> str:
    if value is None or not finite(value):
        return "—"
    return f"{value:.{digits}g}"


def bin_index(distance: float) -> int | None:
    index = int(np.searchsorted(BIN_EDGES, distance, side="right") - 1)
    return index if 0 <= index < len(BIN_LABELS) else None


def percentile(values, q: float) -> float | None:
    values = [value for value in values if finite(value)]
    return float(np.quantile(values, q)) if values else None


def cluster_bootstrap_mean(
    rows: list[dict], field: str, *, repeats: int = 2000, seed: int = 20260727
) -> tuple[float | None, float | None]:
    by_start: dict[str, list[float]] = defaultdict(list)
    for row in rows:
        value = row.get(field)
        if finite(value):
            by_start[row["start_id"]].append(float(value))
    starts = sorted(by_start)
    if not starts:
        return None, None
    rng = np.random.default_rng(seed)
    samples = []
    for _ in range(repeats):
        selected = rng.choice(starts, size=len(starts), replace=True)
        values = [value for start in selected for value in by_start[start]]
        samples.append(np.mean(values))
    return float(np.quantile(samples, 0.025)), float(np.quantile(samples, 0.975))


def write_csv(path: Path, rows: list[dict]) -> None:
    if not rows:
        raise ValueError(f"cannot write empty table: {path}")
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def coverage_rows(rows: list[dict]) -> list[dict]:
    result = []
    selectors = ["anchor_winner", *WINDOWS, ANCHOR_ALL]
    for selector in selectors:
        selected = [
            row
            for row in rows
            if row["selector"] == selector
            and row["value_model"] == DIRECT
            and row.get("target_winner_covered") is not None
        ]
        for index, label in enumerate(BIN_LABELS):
            group = [
                row
                for row in selected
                if bin_index(row["normalized_distance"]) == index
            ]
            values = [float(row["target_winner_covered"]) for row in group]
            low, high = cluster_bootstrap_mean(
                [
                    {**row, "coverage": float(row["target_winner_covered"])}
                    for row in group
                ],
                "coverage",
                seed=20260727 + index,
            )
            result.append(
                {
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "distance_bin": label,
                    "distance_bin_index": index,
                    "targets": len(group),
                    "starts": len({row["start_id"] for row in group}),
                    "coverage": float(np.mean(values)) if values else None,
                    "cluster_bootstrap_low": low,
                    "cluster_bootstrap_high": high,
                    "median_candidates": percentile(
                        [row["candidate_count"] for row in group], 0.5
                    ),
                }
            )
    return result


def mechanism_rows(rows: list[dict]) -> list[dict]:
    definitions = [
        (
            "candidate omission: 10% set, target branch values",
            WINDOWS[1],
            DIRECT,
        ),
        (
            "same-branch affine error: target winner only",
            TARGET_WINNER,
            AFFINE,
        ),
        (
            "combined affine error: 10% set",
            WINDOWS[1],
            AFFINE,
        ),
        (
            "combined affine error: all retained anchor germs",
            ANCHOR_ALL,
            AFFINE,
        ),
    ]
    result = []
    for mechanism, selector, value_model in definitions:
        selected = [
            row
            for row in rows
            if row["selector"] == selector and row["value_model"] == value_model
        ]
        for index, label in enumerate(BIN_LABELS):
            group = [
                row
                for row in selected
                if bin_index(row["normalized_distance"]) == index
            ]
            errors = [
                abs(row["prediction_error"])
                for row in group
                if finite(row.get("prediction_error"))
            ]
            result.append(
                {
                    "mechanism": mechanism,
                    "selector": selector,
                    "value_model": value_model,
                    "distance_bin": label,
                    "distance_bin_index": index,
                    "targets": len(group),
                    "usable_predictions": len(errors),
                    "prediction_rate": len(errors) / len(group) if group else None,
                    "median_abs_error": percentile(errors, 0.5),
                    "q90_abs_error": percentile(errors, 0.9),
                    "q99_abs_error": percentile(errors, 0.99),
                    "max_abs_error": max(errors, default=None),
                }
            )
    return result


def miss_impact_rows(rows: list[dict]) -> list[dict]:
    selected = [
        row
        for row in rows
        if row["selector"] == WINDOWS[1]
        and row["value_model"] == DIRECT
        and row.get("target_winner_covered") is not None
    ]
    result = []
    for index, label in enumerate(BIN_LABELS):
        group = [
            row for row in selected if bin_index(row["normalized_distance"]) == index
        ]
        misses = [row for row in group if not row["target_winner_covered"]]
        miss_errors = [
            abs(row["prediction_error"])
            for row in misses
            if finite(row.get("prediction_error"))
        ]
        result.append(
            {
                "distance_bin": label,
                "distance_bin_index": index,
                "targets": len(group),
                "identity_misses": len(misses),
                "identity_miss_fraction": len(misses) / len(group) if group else None,
                "unusable_predictions": sum(
                    not finite(row.get("prediction_error")) for row in group
                ),
                "unusable_fraction": (
                    sum(not finite(row.get("prediction_error")) for row in group)
                    / len(group)
                    if group
                    else None
                ),
                "fraction_abs_error_gt_1e_6": (
                    sum(
                        finite(row.get("prediction_error"))
                        and abs(row["prediction_error"]) > 1e-6
                        for row in group
                    )
                    / len(group)
                    if group
                    else None
                ),
                "fraction_abs_error_gt_1e_3": (
                    sum(
                        finite(row.get("prediction_error"))
                        and abs(row["prediction_error"]) > 1e-3
                        for row in group
                    )
                    / len(group)
                    if group
                    else None
                ),
                "fraction_abs_error_gt_1e_2": (
                    sum(
                        finite(row.get("prediction_error"))
                        and abs(row["prediction_error"]) > 1e-2
                        for row in group
                    )
                    / len(group)
                    if group
                    else None
                ),
                "median_abs_error_given_identity_miss": percentile(miss_errors, 0.5),
                "q90_abs_error_given_identity_miss": percentile(miss_errors, 0.9),
                "max_abs_error_given_identity_miss": max(miss_errors, default=None),
            }
        )
    return result


def scale_summary(rows: list[dict]) -> list[dict]:
    result = []
    definitions = [
        (WINDOWS[1], DIRECT),
        (TARGET_WINNER, AFFINE),
        (WINDOWS[1], AFFINE),
        (ANCHOR_ALL, AFFINE),
    ]
    for selector, value_model in definitions:
        for scale in sorted({row["distance_scale"] for row in rows}):
            group = [
                row
                for row in rows
                if row["selector"] == selector
                and row["value_model"] == value_model
                and row["distance_scale"] == scale
            ]
            errors = [
                abs(row["prediction_error"])
                for row in group
                if finite(row.get("prediction_error"))
            ]
            coverage = [
                float(row["target_winner_covered"])
                for row in group
                if row.get("target_winner_covered") is not None
            ]
            result.append(
                {
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "value_model": value_model,
                    "distance_scale": scale,
                    "targets": len(group),
                    "prediction_rate": len(errors) / len(group) if group else None,
                    "winner_coverage": float(np.mean(coverage)) if coverage else None,
                    "median_candidates": percentile(
                        [row["candidate_count"] for row in group], 0.5
                    ),
                    "median_represented": percentile(
                        [row["represented_branch_count"] for row in group], 0.5
                    ),
                    "median_abs_error": percentile(errors, 0.5),
                    "q90_abs_error": percentile(errors, 0.9),
                    "max_abs_error": max(errors, default=None),
                }
            )
    return result


def cost_rows(atoms: list[dict], pairs: list[dict]) -> list[dict]:
    pair_lookup = {
        (row["pair_id"], row["distance_scale"]): row
        for row in pairs
        if finite(row.get("full_evaluation_ms"))
    }
    result = []
    selectors = ["anchor_winner", *WINDOWS, ANCHOR_ALL, TARGET_ALL]
    for scale in sorted({row["distance_scale"] for row in atoms}):
        for selector in selectors:
            group = [
                row
                for row in atoms
                if row["distance_scale"] == scale
                and row["selector"] == selector
                and row["value_model"] == DIRECT
                and (row["pair_id"], row["distance_scale"]) in pair_lookup
            ]
            measured = [
                sum(
                    float(row.get(field, 0.0))
                    for field in (
                        "geometry_ms",
                        "volume_ms",
                        "named_branch_ms",
                        "model_ms",
                    )
                )
                for row in group
            ]
            ratios = [
                cost
                / pair_lookup[(row["pair_id"], row["distance_scale"])][
                    "full_evaluation_ms"
                ]
                for row, cost in zip(group, measured)
            ]
            covered = [
                float(row["target_winner_covered"])
                for row in group
                if row.get("target_winner_covered") is not None
            ]
            result.append(
                {
                    "distance_scale": scale,
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "targets": len(group),
                    "median_candidates": percentile(
                        [row["candidate_count"] for row in group], 0.5
                    ),
                    "winner_coverage": float(np.mean(covered)) if covered else None,
                    "median_measured_ms": percentile(measured, 0.5),
                    "median_fraction_of_full_evaluation": percentile(ratios, 0.5),
                    "q90_fraction_of_full_evaluation": percentile(ratios, 0.9),
                }
            )
    return result


def pooled_cost_row(atoms: list[dict], pairs: list[dict], selector: str) -> dict:
    pair_lookup = {
        (row["pair_id"], row["distance_scale"]): row
        for row in pairs
        if finite(row.get("full_evaluation_ms"))
    }
    group = [
        row
        for row in atoms
        if row["selector"] == selector
        and row["value_model"] == DIRECT
        and (row["pair_id"], row["distance_scale"]) in pair_lookup
    ]
    measured = [
        sum(
            float(row.get(field, 0.0))
            for field in ("geometry_ms", "volume_ms", "named_branch_ms", "model_ms")
        )
        for row in group
    ]
    ratios = [
        cost
        / pair_lookup[(row["pair_id"], row["distance_scale"])]["full_evaluation_ms"]
        for row, cost in zip(group, measured)
    ]
    covered = [
        float(row["target_winner_covered"])
        for row in group
        if row.get("target_winner_covered") is not None
    ]
    return {
        "targets": len(group),
        "median_candidates": percentile([row["candidate_count"] for row in group], 0.5),
        "winner_coverage": float(np.mean(covered)) if covered else None,
        "median_measured_ms": percentile(measured, 0.5),
        "median_fraction_of_full_evaluation": percentile(ratios, 0.5),
    }


def failure_examples(rows: list[dict]) -> list[dict]:
    keep = [
        row
        for row in rows
        if row["value_model"] == AFFINE
        and row["selector"] in {TARGET_WINNER, WINDOWS[1], ANCHOR_ALL}
        and finite(row.get("prediction_error"))
    ]
    result = []
    for selector in (TARGET_WINNER, WINDOWS[1], ANCHOR_ALL):
        selected = sorted(
            (row for row in keep if row["selector"] == selector),
            key=lambda row: abs(row["prediction_error"]),
            reverse=True,
        )[:5]
        for rank, row in enumerate(selected, 1):
            result.append(
                {
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "rank": rank,
                    "pair_id": row["pair_id"],
                    "algorithm_id": row["algorithm_id"],
                    "trajectory_phase": row["trajectory_phase"],
                    "distance_scale": row["distance_scale"],
                    "normalized_distance": row["normalized_distance"],
                    "candidate_count": row["candidate_count"],
                    "represented_branch_count": row["represented_branch_count"],
                    "target_winner_in_named_set": row["target_winner_covered"],
                    "target_sys": row["target_sys"],
                    "predicted_target_sys": row["predicted_target_sys"],
                    "signed_error_actual_minus_prediction": row["prediction_error"],
                    "absolute_error": abs(row["prediction_error"]),
                }
            )
    return result


def plot_distance_population(pairs: list[dict], path: Path) -> None:
    usable = [row for row in pairs if finite(row.get("target_sys"))]
    algorithms = sorted({row["algorithm_id"] for row in usable})
    figure, axes = plt.subplots(
        len(algorithms), 1, figsize=(9.0, 2.3 * len(algorithms)), sharex=True
    )
    if len(algorithms) == 1:
        axes = [axes]
    plot_edges = np.geomspace(1e-7, 0.5, 35)
    for axis, algorithm in zip(axes, algorithms):
        group = [row for row in usable if row["algorithm_id"] == algorithm]
        for phase in ("early", "middle", "late"):
            values = [
                max(row["normalized_distance"], 1e-7)
                for row in group
                if row["trajectory_phase"] == phase
            ]
            axis.hist(
                values, bins=plot_edges, alpha=0.55, label=f"{phase} (n={len(values)})"
            )
        axis.set_xscale("log")
        axis.set_ylabel("targets")
        axis.set_title(algorithm)
        axis.grid(alpha=0.2)
        axis.legend(fontsize=8)
    axes[-1].set_xlabel(r"ambient normalized distance $\|a_1-a_0\|_2/\|a_0\|_2$")
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_coverage(rows: list[dict], path: Path) -> None:
    figure, axis = plt.subplots(figsize=(10.5, 5.6))
    selectors = [
        "anchor_winner",
        WINDOWS[0],
        WINDOWS[1],
        WINDOWS[2],
        WINDOWS[3],
        ANCHOR_ALL,
    ]
    x = np.arange(len(BIN_LABELS))
    for selector in selectors:
        group = sorted(
            (row for row in rows if row["selector"] == selector),
            key=lambda row: row["distance_bin_index"],
        )
        y = np.asarray([row["coverage"] for row in group], dtype=float)
        low = np.asarray([row["cluster_bootstrap_low"] for row in group], dtype=float)
        high = np.asarray([row["cluster_bootstrap_high"] for row in group], dtype=float)
        axis.plot(x, y, marker="o", label=SELECTOR_LABELS[selector])
        axis.fill_between(x, low, high, alpha=0.09)
    counts = [
        next(
            row["targets"]
            for row in rows
            if row["selector"] == WINDOWS[1] and row["distance_bin_index"] == index
        )
        for index in range(len(BIN_LABELS))
    ]
    axis.set_xticks(
        x, [f"{label}\nn={count}" for label, count in zip(BIN_LABELS, counts)]
    )
    axis.set_ylim(-0.02, 1.02)
    axis.set_ylabel("fraction containing target full-sys winner")
    axis.set_xlabel("ambient normalized distance bin")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8, ncol=2)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_coverage_strata(atoms: list[dict], path: Path) -> None:
    selected = [
        row
        for row in atoms
        if row["selector"] == WINDOWS[1]
        and row["value_model"] == DIRECT
        and row.get("target_winner_covered") is not None
    ]
    algorithms = sorted({row["algorithm_id"] for row in selected})
    figure, axes = plt.subplots(
        len(algorithms), 1, figsize=(10.5, 2.8 * len(algorithms)), sharex=True
    )
    if len(algorithms) == 1:
        axes = [axes]
    x = np.arange(len(BIN_LABELS))
    for axis, algorithm in zip(axes, algorithms):
        for phase in ("early", "middle", "late"):
            values = []
            denominators = []
            for index in range(len(BIN_LABELS)):
                group = [
                    row
                    for row in selected
                    if row["algorithm_id"] == algorithm
                    and row["trajectory_phase"] == phase
                    and bin_index(row["normalized_distance"]) == index
                ]
                values.append(
                    np.mean([row["target_winner_covered"] for row in group])
                    if group
                    else np.nan
                )
                denominators.append(len(group))
            axis.plot(x, values, marker="o", label=phase)
            for position, value, count in zip(x, values, denominators):
                if count and finite(value):
                    axis.annotate(
                        str(count),
                        (position, value),
                        xytext=(2, 3),
                        textcoords="offset points",
                        fontsize=6,
                    )
        axis.set_ylim(-0.03, 1.03)
        axis.set_ylabel("winner coverage")
        axis.set_title(algorithm + " (labels are target counts)")
        axis.grid(alpha=0.2)
        axis.legend(fontsize=8)
    axes[-1].set_xticks(x, BIN_LABELS)
    axes[-1].set_xlabel("ambient normalized distance bin")
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_miss_impact(rows: list[dict], path: Path) -> None:
    x = np.arange(len(BIN_LABELS))
    series = [
        ("different winning sigma", "identity_miss_fraction"),
        ("no usable direct prediction", "unusable_fraction"),
        ("absolute sys error > 1e-3", "fraction_abs_error_gt_1e_3"),
        ("absolute sys error > 1e-2", "fraction_abs_error_gt_1e_2"),
    ]
    figure, axis = plt.subplots(figsize=(10.5, 5.4))
    for label, field in series:
        axis.plot(x, [row[field] for row in rows], marker="o", label=label)
    axis.set_ylim(-0.02, 1.02)
    axis.set_xticks(
        x,
        [
            f"{row['distance_bin']}\nn={row['targets']}"
            for row in sorted(rows, key=lambda row: row["distance_bin_index"])
        ],
    )
    axis.set_xlabel("ambient normalized distance bin")
    axis.set_ylabel("fraction of 10%-window target rows")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_mechanisms(rows: list[dict], path: Path) -> None:
    figure, axis = plt.subplots(figsize=(10.5, 5.6))
    x = np.arange(len(BIN_LABELS))
    for mechanism in dict.fromkeys(row["mechanism"] for row in rows):
        group = sorted(
            (row for row in rows if row["mechanism"] == mechanism),
            key=lambda row: row["distance_bin_index"],
        )
        axis.plot(
            x,
            [row["q90_abs_error"] for row in group],
            marker="o",
            label=mechanism,
        )
    axis.set_yscale("symlog", linthresh=1e-6)
    axis.set_xticks(x, BIN_LABELS)
    axis.set_xlabel("ambient normalized distance bin")
    axis.set_ylabel("90th percentile absolute sys error")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_target_winner_affine(rows: list[dict], path: Path) -> None:
    selected = [
        row
        for row in rows
        if row["selector"] == TARGET_WINNER and row["value_model"] == AFFINE
    ]
    x = np.arange(len(BIN_LABELS))
    rates = []
    q90 = []
    counts = []
    for index in range(len(BIN_LABELS)):
        group = [
            row for row in selected if bin_index(row["normalized_distance"]) == index
        ]
        errors = [
            abs(row["prediction_error"])
            for row in group
            if finite(row.get("prediction_error"))
        ]
        rates.append(len(errors) / len(group) if group else np.nan)
        q90.append(percentile(errors, 0.9))
        counts.append(len(group))
    figure, left = plt.subplots(figsize=(10.5, 5.2))
    right = left.twinx()
    left.plot(x, rates, color="tab:blue", marker="o", label="affine model exists")
    right.plot(x, q90, color="tab:red", marker="s", label="q90 error when it exists")
    left.set_ylim(-0.02, 1.02)
    right.set_yscale("symlog", linthresh=1e-6)
    left.set_xticks(
        x, [f"{label}\nn={count}" for label, count in zip(BIN_LABELS, counts)]
    )
    left.set_xlabel("ambient normalized distance bin")
    left.set_ylabel(
        "fraction of target winners represented by affine builder at anchor",
        color="tab:blue",
    )
    right.set_ylabel("90th percentile same-branch affine error", color="tab:red")
    left.grid(alpha=0.2)
    lines = left.lines + right.lines
    left.legend(lines, [line.get_label() for line in lines], fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_tail(rows: list[dict], path: Path) -> None:
    definitions = [
        ("10% set, target values", WINDOWS[1], DIRECT),
        ("target winner, affine", TARGET_WINNER, AFFINE),
        ("10% set, affine", WINDOWS[1], AFFINE),
        ("all retained anchor germs, affine", ANCHOR_ALL, AFFINE),
    ]
    figure, axis = plt.subplots(figsize=(8.4, 5.4))
    for label, selector, model in definitions:
        errors = sorted(
            abs(row["prediction_error"])
            for row in rows
            if row["selector"] == selector
            and row["value_model"] == model
            and finite(row.get("prediction_error"))
        )
        x = np.maximum(np.asarray(errors, dtype=float), 1e-12)
        survival = 1.0 - np.arange(1, len(x) + 1) / (len(x) + 1)
        axis.step(x, survival, where="post", label=f"{label} (n={len(x)})")
    axis.set_xscale("log")
    axis.set_yscale("log")
    axis.set_xlabel("absolute sys prediction error")
    axis.set_ylabel("empirical fraction with larger error")
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_cost_coverage(rows: list[dict], path: Path) -> None:
    selectors = [*WINDOWS, ANCHOR_ALL]
    figure, axis = plt.subplots(figsize=(8.6, 5.4))
    for scale in sorted({row["distance_scale"] for row in rows}):
        group = [
            row
            for row in rows
            if row["distance_scale"] == scale and row["selector"] in selectors
        ]
        group.sort(key=lambda row: selectors.index(row["selector"]))
        axis.plot(
            [row["median_fraction_of_full_evaluation"] for row in group],
            [row["winner_coverage"] for row in group],
            marker="o",
            label=f"replay scale {scale:g}",
        )
        for row in group:
            axis.annotate(
                SELECTOR_LABELS[row["selector"]].split()[0],
                (
                    row["median_fraction_of_full_evaluation"],
                    row["winner_coverage"],
                ),
                xytext=(3, 3),
                textcoords="offset points",
                fontsize=7,
            )
    axis.set_xlabel("median predictor cost / paired full-sys evaluation cost")
    axis.set_ylabel("fraction containing target full-sys winner")
    axis.set_xlim(left=0)
    axis.set_ylim(-0.02, 1.02)
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def report(
    path: Path,
    dataset: Path,
    pairs: list[dict],
    atoms: list[dict],
    coverage: list[dict],
    miss_impact: list[dict],
    mechanisms: list[dict],
    scales: list[dict],
    costs: list[dict],
) -> None:
    usable_pairs = [row for row in pairs if finite(row.get("target_sys"))]
    control = [
        row
        for row in atoms
        if row["selector"] == TARGET_ALL
        and row["value_model"] == DIRECT
        and finite(row.get("prediction_error"))
    ]
    covered_direct = [
        row
        for row in atoms
        if row["value_model"] == DIRECT
        and row.get("target_winner_covered") is True
        and finite(row.get("prediction_error"))
    ]
    winner_affine = [
        row
        for row in atoms
        if row["selector"] == TARGET_WINNER and row["value_model"] == AFFINE
    ]
    winner_affine_usable = [
        row for row in winner_affine if finite(row.get("prediction_error"))
    ]
    status_counts = Counter(row["target_status"] for row in pairs)

    lines = [
        "# What the predictor replay establishes",
        "",
        "## Short answer",
        "",
        (
            "Directly reevaluating a named candidate set at the target has no "
            "additional value error once that set contains the full-sys winning "
            "sigma. The recorded discrepancy was 0 for all "
            f"{len(covered_direct)} covered selector-target rows. The useful "
            "questions are therefore (1) whether a stale set contains that winner, "
            "(2) whether the winner is even admissible at the anchor where an "
            "affine model is built, and (3) how badly finite-distance affine "
            "models extrapolate represented branches."
        ),
        "",
        "The present dataset answers those questions descriptively on selected "
        "optimizer proposals. It does not identify the particular sigma responsible "
        "for an affine minimum, so it cannot fully separate branch curvature from "
        "frozen-domain selection in multi-branch failures.",
        "",
        "## Population and distance",
        "",
        (
            f"Source: `{dataset}`. The producer selected three evenly spaced rounds "
            "from each of 48 runs: 16 starts for each of safeguarded gradient, "
            "the affine gap optimizer, and branch history. This gave 144 anchor/"
            "recorded-proposal pairs. Each direction was replayed at 0.5, 1, and "
            f"2 times its recorded length, giving {len(pairs)} targets."
        ),
        "",
        (
            f"Usable targets: {len(usable_pairs)}. Status counts: "
            + ", ".join(
                f"`{key}` {value}" for key, value in sorted(status_counts.items())
            )
            + "."
        ),
        "",
        "Distance means the ambient relative Euclidean displacement",
        "",
        r"\[d(a_0,a_1)=\frac{\lVert a_1-a_0\rVert_2}{\lVert a_0\rVert_2}.\]",
        "",
        "It is not quotient distance. Late optimizer proposals make the population "
        "strongly concentrated near zero; the histogram below exposes that rather "
        "than treating the three replay multipliers as absolute distances.",
        "",
        "![Distance population](distance-population.png)",
        "",
        "## How candidate sets become stale",
        "",
        "At an anchor, the candidate-universe search enumerates transition-feasible "
        "sigma and performs an unrestricted f64 KKT solve. Sigma whose raw solve "
        "fails do not become germs; the full evaluator's anchor winner is inserted "
        "as a witness if necessary. For relative window `w`,",
        "",
        r"\[C_w(a_0)=\{\sigma:A_\sigma(a_0)\leq "
        r"A_{\min,\mathrm{germ}}(a_0)(1+w)\}.\]",
        "",
        "There is no beta-sign cutoff in this selection. For the direct target "
        "predictor, every named sigma is reevaluated at the target and target "
        "transition and beta-admissibility predicates are then applied. Indeterminate "
        "f64 beta decisions use the rational-arithmetic singleton fallback:",
        "",
        r"\[\widetilde{\operatorname{sys}}_{C_w}(a_1)="
        r"\min_{\substack{\sigma\in C_w(a_0)\\"
        r"\mathrm{transition}_\sigma(a_1)\\"
        r"\mathrm{beta\text{-}admissible}_\sigma(a_1)}}"
        r"\operatorname{sys}_\sigma(a_1).\]",
        "",
        "For each target, coverage is the indicator that the named set constructed "
        "at the anchor contains the sigma selected by full sys at the target. Curves "
        "show raw bin means; shaded bands are 95% start-cluster bootstrap intervals "
        "over the 16 starting polytopes. Rows sharing a start are therefore not "
        "treated as independent.",
        "",
        "![Winner coverage versus distance](coverage-vs-distance.png)",
        "",
        "Winning-sigma identity is not monotone in pooled distance. The pool mixes "
        "optimizer families and trajectory phases: for example, very short late gap "
        "proposals change the winning sigma often, while equally short safeguarded-"
        "gradient proposals do not. The stratified plot exposes this confounding; "
        "point labels are target counts and many cells are sparse.",
        "",
        "![Coverage stratified by optimizer and phase](coverage-vs-distance-strata.png)",
        "",
        "Increasing the anchor action window raises coverage mainly for moves above "
        "0.05 and then saturates. Even the complete retained anchor-germ set can miss "
        "target winners. The rows cannot tell whether such a winner was "
        "transition-infeasible at the anchor, had a failed raw anchor solve, or was "
        "omitted for another reason.",
        "",
        "| distance bin | targets | anchor winner | 10% window | 30% window | 100% window | all retained anchor germs |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for index, label in enumerate(BIN_LABELS):
        rows_by_selector = {
            row["selector"]: row
            for row in coverage
            if row["distance_bin_index"] == index
        }
        lines.append(
            "| {label} | {n} | {winner} | {w10} | {w30} | {w100} | {all_anchor} |".format(
                label=label,
                n=rows_by_selector[WINDOWS[1]]["targets"],
                winner=fmt(rows_by_selector["anchor_winner"]["coverage"]),
                w10=fmt(rows_by_selector[WINDOWS[1]]["coverage"]),
                w30=fmt(rows_by_selector[WINDOWS[2]]["coverage"]),
                w100=fmt(rows_by_selector[WINDOWS[3]]["coverage"]),
                all_anchor=fmt(rows_by_selector[ANCHOR_ALL]["coverage"]),
            )
        )
    lines.extend(
        [
            "",
            "A changed winning sigma often has little value effect at very short "
            "distance in this sample. This is consistent with nearly tied branches, "
            "but branch gaps were not recorded. The next plot distinguishes identity "
            "changes from value error for the 10% candidate set reevaluated at the "
            "target.",
            "",
            "![Impact of candidate misses](candidate-miss-impact-vs-distance.png)",
            "",
            "| distance bin | identity miss | unusable | sys error > 1e-3 | sys error > 1e-2 | median error given miss | 90% error given miss |",
            "|---|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for row in miss_impact:
        lines.append(
            "| {distance} | {miss} | {unusable} | {e3} | {e2} | {median} | {q90} |".format(
                distance=row["distance_bin"],
                miss=fmt(row["identity_miss_fraction"]),
                unusable=fmt(row["unusable_fraction"]),
                e3=fmt(row["fraction_abs_error_gt_1e_3"]),
                e2=fmt(row["fraction_abs_error_gt_1e_2"]),
                median=fmt(row["median_abs_error_given_identity_miss"]),
                q90=fmt(row["q90_abs_error_given_identity_miss"]),
            )
        )
    lines.extend(
        [
            "",
            "Below 1e-4 relative distance, the target winner identity differs in a "
            "substantial fraction of rows, but no row has sys error above 1e-3. "
            "Material errors concentrate at larger distances, especially above "
            "0.1, although the pooled middle-distance bins remain nonmonotone "
            "because optimizer and trajectory phase are confounded with distance.",
            "",
            "## Which approximation causes which error",
            "",
            "The controls form a chain:",
            "",
            "1. **Target branch values over an anchor-selected set:** only candidate "
            "omission remains. If the winner is covered, the error is zero.",
            "2. **Affine model of the target-winning sigma alone:** wrong-branch "
            "selection is impossible. Missing predictions mean the affine builder "
            "could not represent that sigma at the anchor; the current rows do not "
            "record whether this was anchor inadmissibility or another model-build "
            "failure. Remaining error is finite-distance error for that one branch.",
            "3. **Affine minimum over an anchor-selected set:** combines candidate "
            "omission, anchor-domain freezing, per-branch affine error, and selecting "
            "the smallest of many extrapolation errors.",
            "",
            "![Error mechanisms versus distance](error-mechanisms-vs-distance.png)",
            "",
            "![Target-winner representability and error](target-winner-affine-vs-distance.png)",
            "",
            (
                f"Only {len(winner_affine_usable)}/{len(winner_affine)} target-winner "
                "rows had an affine model at the anchor. This is consistent with "
                "common admissibility/domain change, but the absent build-failure "
                "reason prevents assigning every missing row to that mechanism."
            ),
            "",
            "The same-branch control also has a substantial finite-distance tail. "
            "Thus affine failure is not solely wrong-branch selection. With many "
            "branches, however, the tail becomes much worse because the minimum "
            "can select one extreme extrapolation.",
            "",
            "| replay scale | model | usable | winner coverage | median error | 90% error | maximum error |",
            "|---:|---|---:|---:|---:|---:|---:|",
        ]
    )
    for row in scales:
        model = f"{row['selector_label']}; " + (
            "target branch values" if row["value_model"] == DIRECT else "affine"
        )
        lines.append(
            "| {scale:g} | {model} | {usable} | {coverage} | {median} | {q90} | {maximum} |".format(
                scale=row["distance_scale"],
                model=model,
                usable=fmt(row["prediction_rate"]),
                coverage=fmt(row["winner_coverage"]),
                median=fmt(row["median_abs_error"]),
                q90=fmt(row["q90_abs_error"]),
                maximum=fmt(row["max_abs_error"]),
            )
        )
    lines.extend(
        [
            "",
            "The survival plot shows why medians were misleading:",
            "",
            "![Prediction-error tails](error-tail-survival.png)",
            "",
            "## Measured cost of larger candidate sets",
            "",
            "The direct predictor timings include reconstructing target geometry and "
            "volume plus the named KKT solves. They are compared with the paired "
            "full-sys target evaluation. An optimizer that reuses target geometry "
            "would have a different cost ratio, so these are implementation-specific "
            "measurements rather than a lower bound.",
            "",
            "![Candidate coverage versus measured cost](cost-coverage-by-scale.png)",
            "",
            "| candidate set | median candidates | median ms | median fraction of full sys | pooled winner coverage |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for selector in ["anchor_winner", *WINDOWS, ANCHOR_ALL, TARGET_ALL]:
        pooled = pooled_cost_row(atoms, pairs, selector)
        lines.append(
            "| {label} | {candidates} | {ms} | {ratio} | {coverage} |".format(
                label=SELECTOR_LABELS[selector],
                candidates=fmt(pooled["median_candidates"]),
                ms=fmt(pooled["median_measured_ms"]),
                ratio=fmt(pooled["median_fraction_of_full_evaluation"]),
                coverage=fmt(pooled["winner_coverage"]),
            )
        )
    lines.extend(
        [
            "",
            "## Sanity controls",
            "",
            (
                f"Reevaluating all retained target germs reproduced full target sys "
                f"on all {len(control)} usable controls; maximum "
                f"recorded discrepancy was "
                f"{max((abs(row['prediction_error']) for row in control), default=math.nan):.3g}."
            ),
            "",
            "The target-germ universe inserts the full evaluator's target winner as "
            "a witness if the raw germ search omitted it. This is therefore a plumbing "
            "control for named-branch reevaluation, not independent evidence that the "
            "target germ search finds every winner.",
            "",
            "Whenever any directly reevaluated named set contained the target winner, "
            f"the maximum discrepancy across {len(covered_direct)} rows was "
            f"{max((abs(row['prediction_error']) for row in covered_direct), default=math.nan):.3g}.",
            "",
            "These controls establish that the direct named-branch evaluator and "
            "coverage label agree with full sys on this dataset. They do not establish "
            "population generalization.",
            "",
            "## What this dataset cannot answer",
            "",
            "- It does not store candidate membership or the sigma selected by the "
            "affine minimum. Therefore a catastrophic multi-branch error cannot be "
            "assigned afterward to a named branch.",
            "- It does not store the anchor transition or raw-solve status of the "
            "future target winner. Therefore misses by the retained anchor-germ set "
            "cannot be attributed to transition changes alone.",
            "- It does not store per-branch target values, beta margins, transition "
            "margins, KKT residuals, or active constraints. Therefore branch "
            "curvature and frozen admissibility cannot be fully separated.",
            "- Its distance is ambient, not symmetry-quotiented.",
            "- It contains selected directions from three optimizers and only 16 "
            "starting polytopes. It is not an iid sample of directions or starts.",
            "- The three replay lengths lie on the same selected direction and are "
            "correlated. The plots are descriptive and cluster by start where an "
            "interval is shown.",
            "- Predictor accuracy is not optimizer performance. Acceptance, recovery, "
            "and long-run sys improvement require on-policy comparison.",
            "",
            "## Files",
            "",
            "- `coverage-by-distance.csv`: denominators, coverage, clustered intervals, "
            "and candidate counts.",
            "- `candidate-miss-impact-by-distance.csv`: identity misses, unusable "
            "predictions, and value-error thresholds.",
            "- `error-mechanisms-by-distance.csv`: usable counts and error quantiles.",
            "- `summary-by-replay-scale.csv`: numeric rows behind the scale summary.",
            "- `cost-coverage-by-scale.csv`: measured candidate-set cost relative to "
            "the paired full-sys evaluation.",
            "- `largest-affine-failures.csv`: selected only by absolute error, five per "
            "reported selector.",
            "- `analysis.json`: input identity and claim boundary.",
            "",
            "Regenerate with:",
            "",
            "```bash",
            "uv run --script experiments/dev-gradient-ascent/optimizer-atoms/explain_replay.py \\",
            f"  --dataset {dataset} \\",
            "  --out /tmp/predictor-replay-evidence",
            "```",
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
    atoms_path = args.dataset / "atoms.jsonl"
    pairs_path = args.dataset / "pairs.jsonl"
    atoms = read_jsonl(atoms_path)
    pairs = read_jsonl(pairs_path)
    if not atoms or not pairs:
        raise ValueError("replay dataset is empty")
    if atoms[0].get("schema_version", 1) >= 2:
        raise ValueError(
            "schema-v2 replay data requires diagnose_replay.py; "
            "explain_replay.py retains the schema-v1 report"
        )
    args.out.mkdir(parents=True)

    coverage = coverage_rows(atoms)
    mechanisms = mechanism_rows(atoms)
    miss_impact = miss_impact_rows(atoms)
    scales = scale_summary(atoms)
    costs = cost_rows(atoms, pairs)
    failures = failure_examples(atoms)
    write_csv(args.out / "coverage-by-distance.csv", coverage)
    write_csv(args.out / "candidate-miss-impact-by-distance.csv", miss_impact)
    write_csv(args.out / "error-mechanisms-by-distance.csv", mechanisms)
    write_csv(args.out / "summary-by-replay-scale.csv", scales)
    write_csv(args.out / "cost-coverage-by-scale.csv", costs)
    write_csv(args.out / "largest-affine-failures.csv", failures)
    plot_distance_population(pairs, args.out / "distance-population.png")
    plot_coverage(coverage, args.out / "coverage-vs-distance.png")
    plot_coverage_strata(atoms, args.out / "coverage-vs-distance-strata.png")
    plot_miss_impact(miss_impact, args.out / "candidate-miss-impact-vs-distance.png")
    plot_mechanisms(mechanisms, args.out / "error-mechanisms-vs-distance.png")
    plot_target_winner_affine(atoms, args.out / "target-winner-affine-vs-distance.png")
    plot_tail(atoms, args.out / "error-tail-survival.png")
    plot_cost_coverage(costs, args.out / "cost-coverage-by-scale.png")
    report(
        args.out / "REPORT.md",
        args.dataset,
        pairs,
        atoms,
        coverage,
        miss_impact,
        mechanisms,
        scales,
        costs,
    )

    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "schema_version": 1,
                "dataset": str(args.dataset),
                "atoms_sha256": hashlib.sha256(atoms_path.read_bytes()).hexdigest(),
                "pairs_sha256": hashlib.sha256(pairs_path.read_bytes()).hexdigest(),
                "atom_rows": len(atoms),
                "pair_rows": len(pairs),
                "distance_definition": "l2(a1-a0)/l2(a0), ambient coordinates",
                "distance_bins": BIN_LABELS,
                "bootstrap": {
                    "unit": "start_id",
                    "repeats": 2000,
                    "seed_base": 20260727,
                },
                "claim_boundary": (
                    "Selected on-trajectory directions from 16 development starts "
                    "and three optimizer families; predictor mechanisms, not full "
                    "optimizer performance or population generalization."
                ),
            },
            indent=2,
        )
        + "\n"
    )
    print(f"wrote question-oriented replay report from {len(atoms)} atom rows")


if __name__ == "__main__":
    main()
