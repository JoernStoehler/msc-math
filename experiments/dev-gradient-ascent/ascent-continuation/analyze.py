# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.9,<4",
#   "numpy>=1.26,<3",
# ]
# ///
"""Analyze repeated validated ascent paths from frozen optimizer endpoints."""

from __future__ import annotations

import argparse
import csv
import json
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np


LABELS = {
    "random_F10_s0_34--history-baseline--c000128": "one-second endpoint",
    "random_F10_s0_34--history-baseline--c000640": "later endpoint",
    "random_F10_s0_44--history-baseline--c000640": "axis-negative endpoint",
    "positive_control_hko2024": "HKO control",
}
COLORS = {
    "gap-window-0.1": "#0072B2",
    "gap-window-1.0": "#D55E00",
    "winning-branch-gradient": "#009E73",
}


def read_json(path: Path):
    return json.loads(path.read_text())


def read_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def write_csv(path: Path, rows: list[dict]) -> None:
    if not rows:
        return
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def group(rows, key):
    result = defaultdict(list)
    for row in rows:
        result[row[key]].append(row)
    return result


def family_comparison(candidates: list[dict], steps: list[dict]) -> list[dict]:
    selected_steps = {(row["state_id"], row["accepted_step"] - 1) for row in steps}
    by_step = defaultdict(list)
    for row in candidates:
        key = (row["state_id"], row["accepted_step"])
        if key in selected_steps and not row["fallback_scan"]:
            by_step[key].append(row)
    rows = []
    for state_id in LABELS:
        state_steps = [values for (sid, _), values in by_step.items() if sid == state_id]
        for family in COLORS:
            family_rows = [
                max(
                    (row for row in values if row["family"] == family),
                    key=lambda row: row["delta_sys"] if row["delta_sys"] is not None else -np.inf,
                    default=None,
                )
                for values in state_steps
            ]
            family_rows = [row for row in family_rows if row is not None]
            if not family_rows:
                continue
            rows.append(
                {
                    "state": LABELS[state_id],
                    "family": family,
                    "accepted_step_opportunities": len(family_rows),
                    "positive_best_radius_count": sum(
                        row["delta_sys"] is not None and row["delta_sys"] > 0
                        for row in family_rows
                    ),
                    "selected_count": sum(row["selected"] for row in family_rows),
                    "median_best_delta": float(
                        np.median([row["delta_sys"] for row in family_rows])
                    ),
                    "maximum_best_delta": max(row["delta_sys"] for row in family_rows),
                }
            )
    return rows


def compute_rows(summary: dict, candidates: list[dict]) -> list[dict]:
    by_state = group(candidates, "state_id")
    summaries = {row["state_id"]: row for row in summary["states"]}
    rows = []
    for state_id, state_candidates in by_state.items():
        evaluator_seconds = sum(row["total_ms"] for row in state_candidates) / 1000
        extension_seconds = 0.0
        search_seconds = 0.0
        derivative_seconds = 0.0
        solve_seconds = 0.0
        gradient_seconds = 0.0
        for row in state_candidates:
            phase = row["proposal_fields"].get("phase_ms", {})
            extension_seconds += phase.get("branch_extension_enumeration", 0) / 1000
            search_seconds += phase.get("candidate_window_search", 0) / 1000
            derivative_seconds += phase.get("branch_derivative", 0) / 1000
            solve_seconds += phase.get("model_solve", 0) / 1000
            gradient_seconds += (
                phase.get("branch_derivative_and_direction", 0)
                + phase.get("quotient_slice", 0)
            ) / 1000
        rows.append(
            {
                "state": LABELS[state_id],
                "full_sys_evaluations": summaries[state_id]["full_sys_evaluations"],
                "evaluator_seconds": evaluator_seconds,
                "transition_extension_seconds": extension_seconds,
                "candidate_search_seconds": search_seconds,
                "branch_derivative_seconds": derivative_seconds,
                "model_solve_seconds": solve_seconds,
                "gradient_direction_seconds": gradient_seconds,
            }
        )
    return rows


def plot_paths(out: Path, summary: dict, steps: list[dict]) -> None:
    by_state = group(steps, "state_id")
    initial = {row["state_id"]: row["recomputed_initial_sys"] for row in summary["states"]}
    fig, (left, right) = plt.subplots(1, 2, figsize=(10.5, 4.2))
    for state_id, rows in by_state.items():
        rows = sorted(rows, key=lambda row: row["accepted_step"])
        x = [0.0] + [row["cumulative_normalized_path_length"] for row in rows]
        gain = [0.0] + [row["cumulative_gain"] for row in rows]
        sys = [initial[state_id]] + [row["sys_after"] for row in rows]
        left.plot(x, gain, marker="o", label=LABELS[state_id])
        right.plot(x, sys, marker="o", label=LABELS[state_id])
    left.set_xlabel("cumulative relative path length")
    left.set_ylabel("cumulative sys gain")
    right.set_xlabel("cumulative relative path length")
    right.set_ylabel("sys")
    left.grid(alpha=0.25)
    right.grid(alpha=0.25)
    left.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "gain-and-sys-vs-path.png", dpi=180)
    plt.close(fig)


def plot_slopes(out: Path, steps: list[dict]) -> None:
    fig, ax = plt.subplots(figsize=(6.8, 4.3))
    for state_id, rows in group(steps, "state_id").items():
        rows = sorted(rows, key=lambda row: row["accepted_step"])
        ax.plot(
            [row["accepted_step"] for row in rows],
            [row["slope"] for row in rows],
            marker="o",
            label=LABELS[state_id],
        )
    ax.set_yscale("log")
    ax.set_xlabel("accepted step")
    ax.set_ylabel("validated gain / Euclidean step length")
    ax.grid(alpha=0.25)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "validated-slope-by-step.png", dpi=180)
    plt.close(fig)


def plot_candidates(out: Path, candidates: list[dict]) -> None:
    states = list(LABELS)
    fig, axes = plt.subplots(2, 2, figsize=(10.5, 7.8), sharex=True)
    for ax, state_id in zip(axes.flat, states, strict=True):
        rows = [row for row in candidates if row["state_id"] == state_id]
        for family, color in COLORS.items():
            family_rows = [
                row
                for row in rows
                if row["family"] == family and row["delta_sys"] is not None
            ]
            if not family_rows:
                continue
            ax.scatter(
                [row["normalized_radius"] for row in family_rows],
                [row["delta_sys"] for row in family_rows],
                s=12,
                alpha=0.32,
                color=color,
                label=family,
            )
            selected = [row for row in family_rows if row["selected"]]
            ax.scatter(
                [row["normalized_radius"] for row in selected],
                [row["delta_sys"] for row in selected],
                s=50,
                marker="*",
                color=color,
                edgecolor="black",
                linewidth=0.35,
            )
        basis = [
            row
            for row in rows
            if row["fallback_scan"] and row["delta_sys"] is not None
        ]
        if basis:
            ax.scatter(
                [row["normalized_radius"] for row in basis],
                [row["delta_sys"] for row in basis],
                s=12,
                alpha=0.35,
                color="#777777",
                label="signed basis",
            )
        ax.axhline(0, color="black", linewidth=0.7)
        ax.set_xscale("log")
        ax.set_title(LABELS[state_id])
        ax.grid(alpha=0.2)
    axes[1, 0].set_xlabel("relative proposal distance")
    axes[1, 1].set_xlabel("relative proposal distance")
    axes[0, 0].set_ylabel("validated sys change")
    axes[1, 0].set_ylabel("validated sys change")
    handles, labels = axes[0, 0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="upper center", ncol=3, fontsize=8)
    fig.tight_layout(rect=(0, 0, 1, 0.95))
    fig.savefig(out / "candidate-gain-by-distance.png", dpi=180)
    plt.close(fig)


def plot_compute(out: Path, rows: list[dict]) -> None:
    fields = [
        ("transition_extension_seconds", "transition extension"),
        ("candidate_search_seconds", "candidate search"),
        ("branch_derivative_seconds", "branch derivatives"),
        ("model_solve_seconds", "model solve"),
        ("gradient_direction_seconds", "single-branch direction"),
        ("evaluator_seconds", "full sys evaluation"),
    ]
    fig, ax = plt.subplots(figsize=(8.5, 4.3))
    x = np.arange(len(rows))
    bottom = np.zeros(len(rows))
    for field, label in fields:
        values = np.asarray([row[field] for row in rows])
        ax.bar(x, values, bottom=bottom, label=label)
        bottom += values
    ax.set_xticks(x, [row["state"] for row in rows], rotation=18, ha="right")
    ax.set_ylabel("traced CPU seconds")
    ax.set_title("Original exploratory implementation (redundant model rebuilds)")
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "measured-compute-breakdown.png", dpi=180)
    plt.close(fig)


def build_report(
    summary: dict,
    candidates: list[dict],
    steps: list[dict],
    family_rows: list[dict],
    compute: list[dict],
) -> str:
    states = {row["state_id"]: row for row in summary["states"]}
    by_state = group(steps, "state_id")
    axis_id = "random_F10_s0_44--history-baseline--c000640"
    axis_steps = sorted(by_state[axis_id], key=lambda row: row["accepted_step"])
    x = np.asarray([row["cumulative_normalized_path_length"] for row in axis_steps])
    y = np.asarray([row["slope"] for row in axis_steps])
    slope_fit, intercept_fit = np.polyfit(x, y, 1)
    zero_fit = -intercept_fit / slope_fit if slope_fit < 0 else np.inf
    evaluator_seconds = sum(row["evaluator_seconds"] for row in compute)
    extension_seconds = sum(row["transition_extension_seconds"] for row in compute)
    family_lookup = {(row["state"], row["family"]): row for row in family_rows}
    gap_by_step = defaultdict(lambda: defaultdict(list))
    for row in candidates:
        if row["family"].startswith("gap-window-"):
            gap_by_step[(row["state_id"], row["accepted_step"])][row["family"]].append(
                row
            )
    gap_differences = []
    for families in gap_by_step.values():
        if set(families) == {"gap-window-0.1", "gap-window-1.0"}:
            narrow = max(row["delta_sys"] for row in families["gap-window-0.1"])
            wide = max(row["delta_sys"] for row in families["gap-window-1.0"])
            gap_differences.append(abs(narrow - wide))

    def result_line(state_id):
        row = states[state_id]
        return (
            f"- **{LABELS[state_id]}:** {row['accepted_steps']} accepted moves, "
            f"relative path length `{row['cumulative_normalized_path_length']:.5g}`, "
            f"gain `{row['cumulative_gain']:.6g}`, final tested slope "
            f"`{row['final_step_slope']:.6g}`; stop reason `{row['stop_reason']}`."
        )

    report = f"""# Repeated ascent continuation: four-state exploratory result

## Answer

The two endpoints previously described as nearly converged are **not near a
local maximum in the tested finite-step direction family**.

The strongest counterexample is the endpoint whose 50 signed coordinate
directions were all negative. The finite-step multi-branch model took ten
successive relative-distance `1e-3` moves. Its validated slope changed only
from `{axis_steps[0]['slope']:.6g}` to `{axis_steps[-1]['slope']:.6g}` while
`sys` rose by `{states[axis_id]['cumulative_gain']:.6g}` over relative path
length `{states[axis_id]['cumulative_normalized_path_length']:.5g}`. Thus the
old axis poll missed a sustained oblique ascent path.

This run did **not** locate the eventual local maximum: all three generic paths
hit the ten-step diagnostic cap while still improving. It provides lower
bounds on remaining useful path and gain, not an estimate of the full distance
to a local maximum.

![Cumulative gain and sys versus path](gain-and-sys-vs-path.png)

## Fixed controls and outcomes

{result_line("random_F10_s0_34--history-baseline--c000128")}
{result_line("random_F10_s0_34--history-baseline--c000640")}
{result_line(axis_id)}
- **HKO control:** no accepted move. Both finite-step models declined to emit a
  positive-prediction move. None of the five current-branch proposals or 50
  signed transverse coordinate directions at relative distance `1e-5`
  improved the fully recomputed scalar.

The positive and negative controls therefore behaved as required. HKO remains
a numerical control here; its theorem packet, not this run, establishes local
maximality.

![Validated slopes](validated-slope-by-step.png)

## What the path shape says

For the axis-negative endpoint, consecutive directions differ by only
`{axis_steps[1]['turning_angle_radians']:.4g}` radians initially and
`{axis_steps[-1]['turning_angle_radians']:.4g}` radians at step ten. The path
is becoming straighter, not oscillating among unrelated numerical directions.
Only two of its ten accepted moves change the fully recomputed minimizing
sigma.

A linear fit of slope against traveled relative distance would cross zero near
relative path length `{zero_fit:.3g}`, about `{zero_fit / x[-1]:.1f}` times
the observed path. That extrapolation is deliberately **not** used as a
convergence estimate: ten local points do not justify assuming the same
curvature or branch structure over that distance. Its useful implication is
only that the observed slope decay gives no evidence of imminent convergence.

The other two paths turn much more (`0.14`--`1.31` radians between successive
directions) and sometimes need `1e-5` or `3e-5` moves before a later `1e-3`
move becomes profitable. Their finite-step geometry is less regular, but their
final tested slopes remain positive.

## Why the earlier endpoint poll failed

The earlier poll tested the positive and negative directions of one
orthonormal coordinate basis in the 25-dimensional symmetry-transverse space.
A positive linear combination can improve a nonsmooth function even when
every individual coordinate direction decreases it. The multi-branch
max--min solve searches such combinations. The axis-negative endpoint is now a
direct observed example: every old basis direction had slope at most about
`-0.0417`, while the first combined direction had validated slope
`{axis_steps[0]['slope']:.4g}`.

This does not show that arbitrary random directions would work. It shows that
the branch-informed combined direction is materially more informative than
the coordinate poll.

![All proposed gains by distance](candidate-gain-by-distance.png)

## Model and distance comparison

Across the 30 accepted moves, candidate windows `0.1` and `1.0` were each
selected 15 times, but that split is numerical tie-breaking rather than a
meaningful contest: the maximum difference between their best validated gains
at one state was only `{max(gap_differences):.3g}` (median
`{np.median(gap_differences):.3g}`). Thus widening this particular window from
`0.1` to `1.0` added no practical value on these paths.

The single-minimizing-branch gradient was selected zero times. On the
axis-negative path, the best-radius single-branch proposal was positive on
`{family_lookup[("axis-negative endpoint", "winning-branch-gradient")]["positive_best_radius_count"]}`
of 10 steps, but it never beat both multi-branch proposals.

The largest tested radius, `1e-3`, was selected for all ten axis-negative
moves, but only 11 of the other 20 moves. Smaller radii there are not merely a
convergence schedule: they sometimes cross into a point from which a large
profitable move becomes available.

This is a diagnostic portfolio, not a tuned optimizer. It evaluates all
families and radii before selecting a move, so its evaluation count should not
be compared directly with an online optimizer that chooses one proposal.

## Compute and implementation result

The run used 509 full `sys` evaluations and took `{summary['wall_seconds']:.1f}`
CPU/wall seconds. Instrumented full evaluations account for only
`{evaluator_seconds:.1f}` seconds for the 505 candidate points (the four
initial-state timings were not retained). Redundantly rebuilding transition-extended
branch models at every radius accounts for `{extension_seconds:.1f}` traced
seconds and is the main cost.

![Measured compute breakdown](measured-compute-breakdown.png)

The producer has now been changed to build each candidate-window model once at
an accepted state and solve it at all five radii. The refactor compiles but was
only rerun in the 4.49-second debug mode, not on the four-state packet, because
repeating a nine-minute
exploratory run only to confirm a performance refactor was not worth the
compute. From the traced components, a roughly fourfold runtime reduction is a
reasonable prediction, not a measurement.

## What remains open

- The paths must be continued beyond ten moves before estimating their eventual
  endpoint, remaining gain, or compute needed to reach it.
- No richer random or branch-gradient direction cover was run after a generic
  model stop, because none of the three generic paths stopped.
- This run records minimizing-sigma changes but not rounded incidence
  signatures. It therefore does not answer whether removing near-redundant dual
  vertices stabilizes incidence or changes optimizer behavior.
- One path from each endpoint does not measure start-point variability or
  compare full optimizers statistically.
- Failure to find a move would remain a restricted diagnostic result, not a
  local-maximality certificate.
"""
    return report


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("raw", type=Path)
    parser.add_argument("out", type=Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    summary = read_json(args.raw / "summary.json")
    candidates = read_jsonl(args.raw / "candidates.jsonl")
    steps = read_jsonl(args.raw / "steps.jsonl")
    assert sum(row["full_sys_evaluations"] for row in summary["states"]) == (
        len(candidates) + len(summary["states"])
    )
    assert all(row["delta_sys"] > 0 for row in steps)
    assert all(row["accepted_steps"] == 10 for row in summary["states"][:3])
    assert summary["states"][3]["accepted_steps"] == 0

    family_rows = family_comparison(candidates, steps)
    compute = compute_rows(summary, candidates)
    write_csv(args.out / "family-comparison.csv", family_rows)
    write_csv(args.out / "compute-breakdown.csv", compute)
    plot_paths(args.out, summary, steps)
    plot_slopes(args.out, steps)
    plot_candidates(args.out, candidates)
    plot_compute(args.out, compute)
    (args.out / "REPORT.md").write_text(
        build_report(summary, candidates, steps, family_rows, compute)
    )
    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "source": str(args.raw),
                "state_count": len(summary["states"]),
                "candidate_count": len(candidates),
                "accepted_step_count": len(steps),
                "family_comparison": family_rows,
                "compute_breakdown": compute,
            },
            indent=2,
        )
        + "\n"
    )


if __name__ == "__main__":
    main()
