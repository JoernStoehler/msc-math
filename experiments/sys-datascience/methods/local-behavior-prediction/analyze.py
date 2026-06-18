# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""Analyze prepared local-behavior prediction tables."""

from __future__ import annotations

import argparse
import csv
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

import matplotlib.pyplot as plt
import numpy as np


def add_experiments_dir_to_path() -> None:
    path = Path(__file__).resolve()
    for parent in path.parents:
        if parent.name == "experiments":
            sys.path.insert(0, str(parent))
            return


add_experiments_dir_to_path()
from figure_config import FIGSIZE_DUAL, FIGSIZE_SINGLE, SCATTER_SIZE, setup  # noqa: E402


STATUS_COLORS = {
    "same_min_branch_set": "#2a9d8f",
    "target_min_subset_of_base_min_branch_set": "#4c956c",
    "target_min_partly_in_base_min_branch_set": "#90be6d",
    "target_min_in_base_near_active": "#457b9d",
    "target_min_in_base_candidate_window": "#f4a261",
    "target_min_partly_in_base_candidate_window": "#e9c46a",
    "target_min_missing_from_base_candidate_window": "#e76f51",
    "no_target_min_branch": "#6c757d",
}


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def read_csv(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    with path.open("r", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    for row in rows:
        for key, value in list(row.items()):
            if value == "":
                row[key] = None
                continue
            try:
                row[key] = float(value)
            except ValueError:
                pass
    return rows


def save_figure(fig, path: Path) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(path)
    plt.close(fig)
    return path


def finite(value: Any) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(float(value))


def plot_branch_stability(summary: list[dict[str, Any]], out_dir: Path) -> Path:
    rows = [row for row in summary if finite(row.get("radius"))]
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    rows_by_family: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        rows_by_family[str(row.get("direction_family", "all"))].append(row)
    family_styles = {
        "gradient": "-",
        "random": "--",
    }
    for field, label, color in [
        ("min_branch_equal_fraction", "same min set", "#2a9d8f"),
        ("min_branch_intersect_fraction", "intersects", "#457b9d"),
        ("target_min_in_base_candidate_fraction", "target in base window", "#f4a261"),
        ("target_min_in_base_near_fraction", "target in base near-active", "#e76f51"),
    ]:
        for family, family_rows in sorted(rows_by_family.items()):
            xs = [row["radius"] for row in family_rows if finite(row.get(field))]
            ys = [row[field] for row in family_rows if finite(row.get(field))]
            if not xs:
                continue
            order = np.argsort(xs)
            ax.plot(
                np.array(xs)[order],
                np.array(ys)[order],
                marker="o",
                linestyle=family_styles.get(family, ":"),
                label=f"{label} ({family})",
                color=color,
            )
    ax.set_xscale("log")
    ax.set_ylim(-0.05, 1.05)
    ax.set_xlabel(r"radius $\|a-a_0\|$")
    ax.set_ylabel("fraction of successful samples")
    ax.set_title("Branch stability by radius and direction family")
    ax.legend(loc="best")
    return save_figure(fig, out_dir / "branch-stability-by-radius.png")


def plot_branch_variation(rows: list[dict[str, Any]], out_dir: Path) -> Path:
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    xs: list[float] = []
    ys: list[float] = []
    colors: list[str] = []
    for row in rows:
        radius = row.get("radius")
        value = row.get("relative_abs_delta_sys_sigma")
        if not finite(radius) or not finite(value):
            continue
        xs.append(max(float(radius), 1.0e-300))
        ys.append(max(float(value), 1.0e-300))
        if row.get("base_is_min_action_branch") and row.get("target_is_min_action_branch"):
            colors.append("#2a9d8f")
        elif row.get("target_is_min_action_branch"):
            colors.append("#e76f51")
        elif row.get("base_is_min_action_branch"):
            colors.append("#457b9d")
        else:
            colors.append("#adb5bd")
    if xs:
        ax.scatter(xs, ys, s=SCATTER_SIZE, c=colors, alpha=0.75, linewidths=0)
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel(r"radius $\|a-a_0\|$")
    ax.set_ylabel(r"relative $|\Delta sys_\sigma|$")
    ax.set_title(r"Branch-function variation")
    return save_figure(fig, out_dir / "sys-branch-variation-vs-radius.png")


def plot_gradient_prediction(rows: list[dict[str, Any]], out_dir: Path) -> Path:
    points = [
        (row.get("branch_predicted_delta_sys"), row.get("observed_delta_sys"))
        for row in rows
        if finite(row.get("branch_predicted_delta_sys")) and finite(row.get("observed_delta_sys"))
    ]
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    if points:
        xs = np.array([point[0] for point in points], dtype=float)
        ys = np.array([point[1] for point in points], dtype=float)
        ax.scatter(xs, ys, s=SCATTER_SIZE, alpha=0.65, linewidths=0)
        low = float(min(xs.min(), ys.min()))
        high = float(max(xs.max(), ys.max()))
        ax.plot([low, high], [low, high], color="black", linewidth=1.0, alpha=0.7)
    ax.axhline(0.0, color="black", linewidth=0.7, alpha=0.4)
    ax.axvline(0.0, color="black", linewidth=0.7, alpha=0.4)
    ax.set_xlabel(r"branch-gradient predicted $\Delta sys$")
    ax.set_ylabel(r"observed $\Delta sys$")
    ax.set_title("Gradient predictions against observed change")
    return save_figure(fig, out_dir / "gradient-prediction-vs-observed.png")


def plot_target_status(rows: list[dict[str, Any]], out_dir: Path) -> Path:
    grouped: dict[float, Counter[str]] = defaultdict(Counter)
    for row in rows:
        if finite(row.get("radius")):
            grouped[float(row["radius"])][row["target_branch_status_at_base"]] += 1
    radii = sorted(grouped)
    statuses = [status for status in STATUS_COLORS if any(grouped[radius][status] for radius in radii)]
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    bottom = np.zeros(len(radii), dtype=float)
    totals = np.array([sum(grouped[radius].values()) for radius in radii], dtype=float)
    xs = np.arange(len(radii))
    for status in statuses:
        values = np.array(
            [grouped[radius][status] / totals[index] if totals[index] else 0.0 for index, radius in enumerate(radii)]
        )
        ax.bar(xs, values, bottom=bottom, label=status.replace("_", " "), color=STATUS_COLORS[status])
        bottom += values
    ax.set_xticks(xs)
    ax.set_xticklabels([f"{radius:.0e}" for radius in radii], rotation=30, ha="right")
    ax.set_ylim(0.0, 1.0)
    ax.set_xlabel(r"radius $\|a-a_0\|$")
    ax.set_ylabel("fraction of successful samples")
    ax.set_title(r"Target branch status at $a_0$")
    ax.legend(loc="center left", bbox_to_anchor=(1.02, 0.5))
    return save_figure(fig, out_dir / "target-branch-status-at-base.png")


def correlation(rows: list[dict[str, Any]], x_field: str, y_field: str) -> float | None:
    points = [
        (row.get(x_field), row.get(y_field))
        for row in rows
        if finite(row.get(x_field)) and finite(row.get(y_field))
    ]
    if len(points) < 2:
        return None
    xs = np.array([point[0] for point in points], dtype=float)
    ys = np.array([point[1] for point in points], dtype=float)
    return float(np.corrcoef(xs, ys)[0, 1])


def write_report(
    path: Path,
    pairs: list[dict[str, Any]],
    branch_variation: list[dict[str, Any]],
    gradient_projections: list[dict[str, Any]],
    figures: list[Path],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    status_counts = Counter(row["target_branch_status_at_base"] for row in pairs)
    radii = sorted({row["radius"] for row in pairs if finite(row.get("radius"))})
    gradient_corr = correlation(gradient_projections, "branch_predicted_delta_sys", "observed_delta_sys")
    median_variation = None
    values = [
        row["relative_abs_delta_sys_sigma"]
        for row in branch_variation
        if finite(row.get("relative_abs_delta_sys_sigma"))
    ]
    if values:
        median_variation = float(np.quantile(values, 0.5))

    lines = [
        "# Local Behavior Prediction Report",
        "",
        "Status: exploratory method artifact, not proof evidence.",
        "",
        "## Data",
        "",
        f"- successful pair rows: {len(pairs)}",
        f"- branch variation rows: {len(branch_variation)}",
        f"- gradient projection rows: {len(gradient_projections)}",
        f"- radii: {', '.join(f'{radius:.6g}' for radius in radii)}",
        "",
        "## Observations",
        "",
        f"- Target branch status counts: {dict(status_counts)}.",
        f"- Median relative branch-function variation: {median_variation}.",
        f"- Correlation between branch-gradient predicted and observed `Delta sys`: {gradient_corr}.",
        "",
        "## Figures",
        "",
    ]
    captions = {
        "branch-stability-by-radius.png": "Fractions of samples whose target minimizing branch data is already visible at `a0`, grouped by radius and direction family.",
        "sys-branch-variation-vs-radius.png": "Per-branch relative `sys_sigma` variation grows with radius; color marks whether the branch is minimizing at either endpoint.",
        "gradient-prediction-vs-observed.png": "Branch-gradient predictions are compared with recomputed `sys` changes. The diagonal is exact first-order agreement at finite radius.",
        "target-branch-status-at-base.png": "Stacked branch-status classes show when target minimizing branches leave the base action window.",
    }
    for figure in figures:
        lines.extend(
            [
                f"![{figure.name}]({figure.name})",
                "",
                captions.get(figure.name, ""),
                "",
            ]
        )
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Analyze prepared local-behavior prediction tables.")
    parser.add_argument("prepared_dir", type=Path)
    parser.add_argument("--out-dir", type=Path)
    args = parser.parse_args()

    prepared_dir = args.prepared_dir.resolve()
    out_dir = args.out_dir.resolve() if args.out_dir else prepared_dir / "local-behavior-prediction"
    setup()
    pairs = read_jsonl(prepared_dir / "local-behavior-pairs.jsonl")
    branch_variation = read_jsonl(prepared_dir / "local-behavior-branch-variation.jsonl")
    gradient_projections = read_jsonl(prepared_dir / "local-behavior-gradient-projections.jsonl")
    summary = read_csv(prepared_dir / "local-behavior-radius-summary.csv")

    figures = [
        plot_branch_stability(summary, out_dir),
        plot_branch_variation(branch_variation, out_dir),
        plot_gradient_prediction(gradient_projections, out_dir),
        plot_target_status(pairs, out_dir),
    ]
    report = out_dir / "report.md"
    write_report(report, pairs, branch_variation, gradient_projections, figures)
    for path in figures + [report]:
        print(path)


if __name__ == "__main__":
    main()
