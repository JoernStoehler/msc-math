# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.9,<4",
# ]
# ///
"""Analyze recovery from known perturbations of the HKO local maximum."""

from __future__ import annotations

import argparse
import csv
import json
from collections import defaultdict
from pathlib import Path
from statistics import median

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def read_json(path: Path):
    return json.loads(path.read_text())


def read_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def write_csv(path: Path, rows: list[dict]) -> None:
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def label(direction: str) -> str:
    return {
        "sentinel_slice_basis_column_0": "slice-basis sentinel",
        "sentinel_projected_rotated_pentagon_tangent": "pentagon-tangent sentinel",
        "random_000": "random 000",
        "random_001": "random 001",
        "hko_control": "HKO control",
    }[direction]


def plot_summary(out: Path, rows: list[dict]) -> None:
    fig, axes = plt.subplots(1, 3, figsize=(13.2, 4.2), sharex=True)
    for direction in sorted({row["direction_class"] for row in rows}):
        group = sorted(
            (row for row in rows if row["direction_class"] == direction),
            key=lambda row: row["source_distance"],
        )
        x = [row["source_distance"] for row in group]
        axes[0].plot(x, [row["recovered_gap_fraction"] for row in group], "o-", label=label(direction))
        axes[1].plot(x, [row["reference_distance_contraction"] for row in group], "o-")
        axes[2].plot(
            x,
            [row["reference_sys"] - row["final_sys"] for row in group],
            "o-",
        )
    for ax in axes:
        ax.set_xscale("log")
        ax.set_xlabel("source distance from HKO")
        ax.grid(alpha=0.25)
    axes[0].axhline(1.0, color="black", linewidth=0.7)
    axes[1].axhline(1.0, color="black", linewidth=0.7)
    axes[0].set_ylabel("fraction of known sys gap recovered")
    axes[1].set_ylabel("fraction of HKO distance removed")
    axes[2].set_yscale("log")
    axes[2].set_ylabel("remaining sys gap to HKO")
    axes[0].legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "recovery-by-source-distance.png", dpi=180)
    plt.close(fig)


def plot_paths(out: Path, summaries: list[dict], steps: list[dict]) -> None:
    summary = {row["state_id"]: row for row in summaries}
    grouped = defaultdict(list)
    for row in steps:
        grouped[row["state_id"]].append(row)
    fig, ax = plt.subplots(figsize=(8.2, 5.0))
    for state_id, state_steps in grouped.items():
        meta = summary[state_id]
        if meta["source_distance"] == 0:
            continue
        state_steps.sort(key=lambda row: row["accepted_step"])
        x = [1] + [row["full_sys_evaluations_so_far"] for row in state_steps]
        y = [1.0] + [
            row["distance_to_reference"] / meta["initial_reference_distance"]
            for row in state_steps
        ]
        ax.plot(
            x,
            y,
            marker="o",
            alpha=0.75,
            label=f"{label(meta['direction_class'])}, d={meta['source_distance']:.0e}",
        )
    ax.set_yscale("log")
    ax.set_xlabel("cumulative full sys evaluations")
    ax.set_ylabel("remaining Euclidean distance / initial distance")
    ax.grid(alpha=0.25)
    ax.legend(fontsize=7, ncol=2)
    fig.tight_layout()
    fig.savefig(out / "reference-distance-by-compute.png", dpi=180)
    plt.close(fig)


def method_rows(states: list[dict], candidates: list[dict]) -> list[dict]:
    metadata = {row["state_id"]: row for row in states}
    grouped = defaultdict(list)
    for row in candidates:
        state = metadata[row["state_id"]]
        if state["source_distance"] > 0 and row["delta_sys"] is not None:
            grouped[(row["state_id"], row["family"])].append(row)
    result = []
    for (state_id, family), rows in grouped.items():
        state = metadata[state_id]
        best = max(rows, key=lambda row: row["delta_sys"])
        predicted = best["proposal_fields"].get("predicted_delta")
        result.append(
            {
                "state_id": state_id,
                "direction_class": state["direction_class"],
                "source_distance": state["source_distance"],
                "family": family,
                "best_validated_gain": best["delta_sys"],
                "known_initial_gap": state["initial_reference_gap"],
                "gap_recovery_fraction": (
                    best["delta_sys"] / state["initial_reference_gap"]
                ),
                "predicted_gain": predicted,
                "prediction_error_over_known_gap": (
                    None
                    if predicted is None
                    else (predicted - best["delta_sys"])
                    / state["initial_reference_gap"]
                ),
                "best_normalized_radius": best["normalized_radius"],
                "best_normalized_distance": best["normalized_distance"],
            }
        )
    return result


def plot_method_recovery(out: Path, rows: list[dict]) -> None:
    colors = {
        "gap-window-0.1": "#0072B2",
        "gap-window-1.0": "#D55E00",
        "winning-branch-gradient": "#009E73",
    }
    fig, ax = plt.subplots(figsize=(7.2, 4.7))
    distances = sorted({row["source_distance"] for row in rows})
    for family, color in colors.items():
        values = {
            distance: [
                row["gap_recovery_fraction"]
                for row in rows
                if row["family"] == family and row["source_distance"] == distance
            ]
            for distance in distances
        }
        lower = [min(values[distance]) for distance in distances]
        center = [median(values[distance]) for distance in distances]
        upper = [max(values[distance]) for distance in distances]
        ax.fill_between(distances, lower, upper, color=color, alpha=0.14)
        ax.plot(distances, center, "o-", color=color, label=family)
    ax.axhline(1.0, color="black", linewidth=0.7)
    ax.axhline(0.0, color="black", linewidth=0.5)
    ax.set_xscale("log")
    ax.set_xlabel("source distance from HKO")
    ax.set_ylabel("best validated gain / known sys gap")
    ax.grid(alpha=0.25)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "method-recovery-by-distance.png", dpi=180)
    plt.close(fig)


def plot_prediction_error(out: Path, rows: list[dict]) -> None:
    colors = {
        "sentinel_slice_basis_column_0": "#0072B2",
        "sentinel_projected_rotated_pentagon_tangent": "#D55E00",
        "random_000": "#009E73",
        "random_001": "#CC79A7",
    }
    model_rows = [row for row in rows if row["family"] == "gap-window-0.1"]
    fig, ax = plt.subplots(figsize=(7.2, 4.7))
    for direction, color in colors.items():
        group = sorted(
            (row for row in model_rows if row["direction_class"] == direction),
            key=lambda row: row["source_distance"],
        )
        ax.plot(
            [row["source_distance"] for row in group],
            [row["prediction_error_over_known_gap"] for row in group],
            "o-",
            color=color,
            label=label(direction),
        )
    ax.axhline(0.0, color="black", linewidth=0.7)
    ax.set_xscale("log")
    ax.set_xlabel("source distance from HKO")
    ax.set_ylabel("(predicted gain - validated gain) / known sys gap")
    ax.grid(alpha=0.25)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "prediction-error-by-distance.png", dpi=180)
    plt.close(fig)


def compute_rows(candidates: list[dict]) -> list[dict]:
    result = []
    for family in ["gap-window-0.1", "gap-window-1.0", "winning-branch-gradient"]:
        rows = [
            row
            for row in candidates
            if row["state_id"] != "hko-control" and row["family"] == family
        ]
        build_seconds = 0.0
        extension_seconds = 0.0
        search_seconds = 0.0
        solve_seconds = 0.0
        for row in rows:
            phases = row["proposal_fields"].get("phase_ms") or {}
            shared = phases.get("shared_model_build")
            if shared:
                build_seconds += shared["total"] / 1000
                extension_seconds += shared["branch_extension_enumeration"] / 1000
                search_seconds += shared["candidate_window_search"] / 1000
            solve_seconds += (phases.get("model_solve") or 0.0) / 1000
        result.append(
            {
                "family": family,
                "validated_candidate_evaluations": len(rows),
                "model_build_seconds": build_seconds,
                "extension_enumeration_seconds": extension_seconds,
                "candidate_search_seconds": search_seconds,
                "model_solve_seconds": solve_seconds,
                "full_evaluator_seconds": sum(row["total_ms"] for row in rows) / 1000,
            }
        )
    return result


def plot_compute(out: Path, rows: list[dict]) -> None:
    fig, ax = plt.subplots(figsize=(7.5, 4.6))
    fields = [
        ("extension_enumeration_seconds", "branch-extension enumeration"),
        ("candidate_search_seconds", "candidate-window search"),
        ("model_solve_seconds", "local model solve"),
        ("full_evaluator_seconds", "validated candidate evaluation"),
    ]
    x = list(range(len(rows)))
    bottoms = [0.0] * len(rows)
    for field, label_name in fields:
        values = [row[field] for row in rows]
        ax.bar(x, values, bottom=bottoms, label=label_name)
        bottoms = [left + right for left, right in zip(bottoms, values, strict=True)]
    ax.set_xticks(x, [row["family"] for row in rows])
    ax.set_ylabel("traced seconds across 16 perturbations")
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(out / "compute-by-method.png", dpi=180)
    plt.close(fig)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("raw", type=Path)
    parser.add_argument("out", type=Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    summary = read_json(args.raw / "summary.json")
    steps = read_jsonl(args.raw / "steps.jsonl")
    candidates = read_jsonl(args.raw / "candidates.jsonl")
    states = summary["states"]
    control = next(row for row in states if row["direction_class"] == "hko_control")
    rows = [row for row in states if row["source_distance"] > 0]
    write_csv(args.out / "state-recovery.csv", rows)
    plot_summary(args.out, rows)
    plot_paths(args.out, states, steps)
    methods = method_rows(states, candidates)
    compute = compute_rows(candidates)
    write_csv(args.out / "method-comparison.csv", methods)
    write_csv(args.out / "compute-by-method.csv", compute)
    plot_method_recovery(args.out, methods)
    plot_prediction_error(args.out, methods)
    plot_compute(args.out, compute)

    missed = [row for row in rows if row["accepted_steps"] == 0]
    weak = [
        row
        for row in rows
        if row["recovered_gap_fraction"] is not None
        and row["recovered_gap_fraction"] < 0.5
    ]
    by_distance = defaultdict(list)
    for row in rows:
        by_distance[row["source_distance"]].append(row)
    distance_lines = []
    for distance, group in sorted(by_distance.items()):
        recovery = [row["recovered_gap_fraction"] for row in group]
        contraction = [row["reference_distance_contraction"] for row in group]
        distance_lines.append(
            f"- distance `{distance:.0e}`: recovered sys-gap fraction "
            f"`{min(recovery):.3g}`–`{max(recovery):.3g}`; removed distance fraction "
            f"`{min(contraction):.3g}`–`{max(contraction):.3g}`."
        )
    method_lookup = defaultdict(dict)
    for row in methods:
        method_lookup[row["state_id"]][row["family"]] = row
    narrow_wide_differences = [
        values["gap-window-1.0"]["gap_recovery_fraction"]
        - values["gap-window-0.1"]["gap_recovery_fraction"]
        for values in method_lookup.values()
    ]
    gradients = [
        values["winning-branch-gradient"]["gap_recovery_fraction"]
        for values in method_lookup.values()
    ]
    narrow_errors = [
        row["prediction_error_over_known_gap"]
        for row in methods
        if row["family"] == "gap-window-0.1"
    ]
    compute_lookup = {row["family"]: row for row in compute}
    duplicated_wide_seconds = (
        compute_lookup["gap-window-1.0"]["model_build_seconds"]
        + compute_lookup["gap-window-1.0"]["full_evaluator_seconds"]
    )
    full_evaluations = sum(row["full_sys_evaluations"] for row in states)
    residual_gaps_at_far_distance = [
        row["reference_sys"] - row["final_sys"]
        for row in rows
        if row["source_distance"] == 1.0e-1
    ]
    report = f"""# HKO perturbation calibration

## Direct answer

The continuation diagnostic was tested on `{len(rows)}` known perturbations of
the proved HKO local maximum: four fixed directions at four controlled
Euclidean distances. HKO itself is the false-positive control.

The HKO control accepted `{control['accepted_steps']}` moves and changed `sys`
by `{control['cumulative_gain']:.3g}`. The perturbation panel contains
`{len(missed)}` complete misses and `{len(weak)}` states that recovered less
than half of their known `sys` gap in one move.

The quantities below distinguish three statements that the earlier endpoint
screen conflated: a validated improvement exists, the tested method can find
it, and the method removes a substantial fraction of the known gap and
distance.

![Recovery by source distance](recovery-by-source-distance.png)

## Distance dependence

{chr(10).join(distance_lines)}

![Distance to HKO versus full evaluations](reference-distance-by-compute.png)

At distance `1e-1`, the single move leaves a `sys` gap of
`{min(residual_gaps_at_far_distance):.3g}`–`{max(residual_gaps_at_far_distance):.3g}`.
Those four residual states are the highest-value population for testing
additional moves; repeating near-HKO moves for all 16 states was
computationally wasteful.

## Which proposal machinery mattered

The two multi-branch models recover essentially the same gap. Across all 16
states, changing the candidate window from `0.1` to `1.0` changes the recovered
gap fraction by `{min(narrow_wide_differences):.3g}` to
`{max(narrow_wide_differences):.3g}`. The wider model therefore adds no
scientifically meaningful recovery on this panel.

The current minimizing-branch gradient is qualitatively worse: its best tested
move recovers between `{min(gradients):.3g}` and `{max(gradients):.3g}` of the
known gap, and is negative on the rotated-pentagon tangent. This is the
expected ridge failure: the differentiated branch ceases to control the
minimum after moving.

![Recovery by method](method-recovery-by-distance.png)

For the `0.1` branch model, prediction error on the best tested move ranges
from `{min(narrow_errors):.3g}` to `{max(narrow_errors):.3g}` of the known
gap. Its magnitude grows with source distance and depends strongly on
direction, but the validated gain remains positive in every case.

![Prediction error by distance](prediction-error-by-distance.png)

## Measured cost

The completed run took `{summary['wall_seconds']:.1f}` wall seconds and
`{full_evaluations}` full `sys` evaluations. Every perturbation used ten:
one initial evaluation and three distances for each of the two branch models
and the current-branch gradient. HKO used 54 because no model move improved and
the signed-basis fallback ran.

For the 16 perturbations, each branch-window model independently spent about
`{compute_lookup['gap-window-0.1']['model_build_seconds']:.1f}` seconds building
its branches. Keeping the empirically redundant `1.0` window accounts for
`{duplicated_wide_seconds:.1f}` directly traced seconds, before counting its
share of untraced HKO model construction. Branch-extension enumeration, not
the local max-min solve or the final validated evaluator calls, is the measured
hotspot.

![Compute by method](compute-by-method.png)

An earlier attempt allowed three accepted moves for every perturbation. It was
stopped without a final summary after `706.58` CPU seconds and `11:47` wall
time. The partial rows were not retained as evidence. The failure mechanism
was visible before termination: after a first move had nearly reached HKO, the
program rebuilt both full branch windows at the original source-distance
schedule to obtain gains below `1e-7`. The one-step panel above is the corrected
measurement.

## Evidence boundary

These four directions were selected before this run to expose a slice-basis
direction, a structured shallow direction, and two random directions. They are
a development calibration set, not an estimate of the probability of missing
an arbitrary improving direction. The remaining retained random rays have not
been used here and can serve as a held-out panel after the proposal rules and
distance schedule are frozen.

An accepted move is fully re-evaluated and is strong evidence of ascent for
that state. A stop is only a false-negative observation relative to the known
HKO gap; it is not evidence of local maximality. A population likelihood for a
stop requires the held-out panel. Thus this run substantially raises confidence
that the multi-branch diagnostic distinguishes HKO from these controlled
perturbations, but it does not assign a miss probability to an arbitrary
optimizer endpoint.
"""
    (args.out / "REPORT.md").write_text(report)
    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "state_count": len(rows),
                "hko_control_accepted_steps": control["accepted_steps"],
                "complete_miss_count": len(missed),
                "under_half_gap_recovery_count": len(weak),
                "minimum_gap_recovery_fraction": min(
                    row["recovered_gap_fraction"] for row in rows
                ),
                "minimum_reference_distance_contraction": min(
                    row["reference_distance_contraction"] for row in rows
                ),
                "wide_minus_narrow_recovery_range": [
                    min(narrow_wide_differences),
                    max(narrow_wide_differences),
                ],
                "current_branch_gradient_recovery_range": [
                    min(gradients),
                    max(gradients),
                ],
                "full_sys_evaluations": full_evaluations,
                "run_wall_seconds": summary["wall_seconds"],
                "claim_boundary": (
                    "Development directions only; no population miss-rate estimate."
                ),
            },
            indent=2,
        )
        + "\n"
    )


if __name__ == "__main__":
    main()
