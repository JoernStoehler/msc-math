# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.9,<4",
#   "numpy>=1.26,<3",
# ]
# ///
"""Validate and summarize quotient-basis polls of an optimizer endpoint population."""

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


def read_json(path: Path):
    return json.loads(path.read_text())


def read_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def write_csv(path: Path, rows: list[dict]) -> None:
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(
            stream, fieldnames=list(rows[0]), lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def state_key(state_id: str) -> str:
    return state_id.split("--", 1)[0]


def analyze(raw: Path) -> tuple[list[dict], list[dict], dict, list[dict]]:
    summary = read_json(raw / "summary.json")
    states = read_jsonl(raw / "states.jsonl")
    polls = read_jsonl(raw / "poll-directions.jsonl")
    radius_rows = read_jsonl(raw / "radius-summaries.jsonl")
    assert not summary["smoke"]
    assert summary["state_count"] == len(states)
    assert summary["poll_row_count"] == len(polls)
    assert len(radius_rows) == len(states) * len(summary["radii"])
    assert len({row["state_id"] for row in states}) == len(states)
    assert all(row["control_role"] == "held_out_optimizer_endpoint" for row in states)
    assert all(row["facet_count"] == 10 for row in states)

    state_by_id = {row["state_id"]: row for row in states}
    by_state = defaultdict(list)
    for row in radius_rows:
        by_state[row["state_id"]].append(row)
    for state_id, rows in by_state.items():
        assert len(rows) == len(summary["radii"])
        expected = 2 * state_by_id[state_id]["quotient_dimension"]
        assert all(row["expected_direction_count"] == expected for row in rows)

    state_rows = []
    for state_id, rows in sorted(by_state.items()):
        rows = sorted(rows, key=lambda row: row["relative_radius"], reverse=True)
        no_positive = [
            row["invalid_direction_count"] == 0
            and row["improving_direction_count"] == 0
            for row in rows
        ]
        same_incidence = [
            row["combinatorial_change_count"] == 0 for row in rows
        ]
        state_rows.append(
            {
                "state_id": state_id,
                "base_sys": state_by_id[state_id]["recomputed_sys"],
                "no_positive_at_all_radii": all(no_positive),
                "clean_same_incidence_no_positive_at_all_radii": all(
                    passed and unchanged
                    for passed, unchanged in zip(no_positive, same_incidence)
                ),
                "no_positive_radius_count": sum(no_positive),
                "maximum_delta_sys": max(row["max_delta_sys"] for row in rows),
                "maximum_delta_sys_per_step": max(
                    row["max_delta_sys_per_step"] for row in rows
                ),
                "total_improving_directions": sum(
                    row["improving_direction_count"] for row in rows
                ),
                "total_invalid_directions": sum(
                    row["invalid_direction_count"] for row in rows
                ),
                "total_combinatorial_changes": sum(
                    row["combinatorial_change_count"] for row in rows
                ),
            }
        )

    aggregate = []
    for radius in sorted(summary["radii"], reverse=True):
        rows = [row for row in radius_rows if row["relative_radius"] == radius]
        maxima = np.asarray([row["max_delta_sys"] for row in rows], dtype=float)
        slopes = np.asarray(
            [row["max_delta_sys_per_step"] for row in rows], dtype=float
        )
        aggregate.append(
            {
                "relative_radius": radius,
                "states": len(rows),
                "no_positive_count": sum(
                    row["invalid_direction_count"] == 0
                    and row["improving_direction_count"] == 0
                    for row in rows
                ),
                "positive_observed_count": sum(
                    row["improving_direction_count"] > 0 for row in rows
                ),
                "incomplete_count": sum(
                    row["invalid_direction_count"] > 0 for row in rows
                ),
                "incidence_change_count": sum(
                    row["combinatorial_change_count"] > 0 for row in rows
                ),
                "median_maximum_delta_sys": float(np.median(maxima)),
                "q10_maximum_delta_sys": float(np.quantile(maxima, 0.1)),
                "q90_maximum_delta_sys": float(np.quantile(maxima, 0.9)),
                "median_maximum_slope": float(np.median(slopes)),
                "q10_maximum_slope": float(np.quantile(slopes, 0.1)),
                "q90_maximum_slope": float(np.quantile(slopes, 0.9)),
            }
        )
    return state_rows, aggregate, summary, radius_rows


def optimizer_metadata(dataset: Path | None) -> dict[str, dict]:
    if dataset is None:
        return {}
    runs = [
        row
        for row in read_jsonl(dataset / "runs.jsonl")
        if row["algorithm_id"] == "history-baseline"
    ]
    evaluations = {
        row["evaluation_id"]: row
        for row in read_jsonl(dataset / "evaluations.jsonl")
    }
    result = {}
    uncertainty_fields = (
        "geometry_indeterminate_count",
        "vertex_indeterminate_count",
        "bounded_near_singular_vertex_count",
        "ambiguous_vertex_incidence_count",
        "facet_intersection_indeterminate_count",
        "omega_indeterminate_count",
    )
    for run in runs:
        evaluation = evaluations[run["best_evaluation_id"]]
        result[run["start_id"]] = {
            "charged_calls": run["charged_calls"],
            "charged_compute_ms": run["charged_compute_ms"],
            "stop_reason": run["stop_reason"],
            "geometry_predicate_uncertainties": sum(
                evaluation[field] for field in uncertainty_fields
            ),
        }
    return result


def plot_aggregate(aggregate: list[dict], output: Path) -> None:
    rows = sorted(aggregate, key=lambda row: row["relative_radius"])
    radii = [row["relative_radius"] for row in rows]
    figure, axes = plt.subplots(2, 1, figsize=(7.2, 6.8), sharex=True)
    for axis, key, low, high, label in (
        (
            axes[0],
            "median_maximum_delta_sys",
            "q10_maximum_delta_sys",
            "q90_maximum_delta_sys",
            "best observed change in sys",
        ),
        (
            axes[1],
            "median_maximum_slope",
            "q10_maximum_slope",
            "q90_maximum_slope",
            "best observed change / step norm",
        ),
    ):
        median = np.asarray([row[key] for row in rows])
        q10 = np.asarray([row[low] for row in rows])
        q90 = np.asarray([row[high] for row in rows])
        axis.plot(radii, median, marker="o")
        axis.fill_between(radii, q10, q90, alpha=0.2, label="10–90% across states")
        axis.axhline(0.0, color="black", linewidth=0.8)
        axis.set_ylabel(label)
        axis.grid(alpha=0.25)
        axis.legend()
    axes[0].set_yscale("symlog", linthresh=1e-8)
    axes[1].set_xlabel("relative radius (step norm / state norm)")
    axes[1].set_xscale("log")
    figure.tight_layout()
    for suffix in ("png", "pdf"):
        figure.savefig(output / f"endpoint-poll-by-radius.{suffix}", dpi=200)
    plt.close(figure)


def plot_state_slopes(radius_rows: list[dict], output: Path) -> None:
    grouped = defaultdict(list)
    for row in radius_rows:
        grouped[row["state_id"]].append(row)
    figure, axes = plt.subplots(4, 4, figsize=(10.5, 9.0), sharex=True)
    for axis, (state_id, rows) in zip(axes.flat, sorted(grouped.items())):
        rows = sorted(rows, key=lambda row: row["relative_radius"])
        axis.plot(
            [row["relative_radius"] for row in rows],
            [row["max_delta_sys_per_step"] for row in rows],
            marker="o",
        )
        axis.axhline(0.0, color="black", linewidth=0.7)
        axis.set_xscale("log")
        axis.grid(alpha=0.2)
        axis.set_title(state_key(state_id), fontsize=8)
    figure.supxlabel("relative radius")
    figure.supylabel("best observed change / step norm")
    figure.tight_layout()
    for suffix in ("png", "pdf"):
        figure.savefig(output / f"endpoint-slope-by-state.{suffix}", dpi=200)
    plt.close(figure)


def comparison_rows(
    states: list[dict],
    baseline_states: list[dict],
    metadata: dict[str, dict],
    baseline_metadata: dict[str, dict],
) -> list[dict]:
    current = {state_key(row["state_id"]): row for row in states}
    baseline = {state_key(row["state_id"]): row for row in baseline_states}
    if set(current) != set(baseline):
        raise ValueError("current and baseline endpoint panels do not match")
    rows = []
    for start_id in sorted(current):
        now = current[start_id]
        before = baseline[start_id]
        row = {
            "start_id": start_id,
            "baseline_sys": before["base_sys"],
            "current_sys": now["base_sys"],
            "sys_gain": now["base_sys"] - before["base_sys"],
            "baseline_explicit_improvement": not before["no_positive_at_all_radii"],
            "current_explicit_improvement": not now["no_positive_at_all_radii"],
            "baseline_maximum_slope": before["maximum_delta_sys_per_step"],
            "current_maximum_slope": now["maximum_delta_sys_per_step"],
            "current_maximum_single_step_gain": now["maximum_delta_sys"],
        }
        for prefix, source in (
            ("baseline", baseline_metadata),
            ("current", metadata),
        ):
            if start_id in source:
                row[f"{prefix}_charged_calls"] = source[start_id]["charged_calls"]
                row[f"{prefix}_stop_reason"] = source[start_id]["stop_reason"]
        rows.append(row)
    return rows


def write_report(
    output: Path,
    raw: Path,
    states: list[dict],
    aggregate: list[dict],
    summary: dict,
    radius_rows: list[dict],
    metadata: dict[str, dict],
    comparisons: list[dict],
    followup_report: Path | None,
) -> None:
    passing = sum(row["no_positive_at_all_radii"] for row in states)
    clean_passing = sum(
        row["clean_same_incidence_no_positive_at_all_radii"] for row in states
    )
    lines = [
        "# Optimizer endpoint diagnostic",
        "",
        f"The signed quotient-basis diagnostic tested {len(states)} population-stratified "
        f"`history-baseline` endpoints from an F=10 optimizer run. Each state "
        f"was probed in both signs of all 25 quotient-basis axes at relative radii "
        + ", ".join(f"`{radius:g}`" for radius in summary["radii"])
        + ". Every probe recomputed full `sys`.",
        "",
        f"**{len(states) - passing}/{len(states)} endpoints have an explicit improving "
        f"basis direction, while {passing}/{len(states)} have no observed improvement "
        f"at all three radii.** Of the latter, {clean_passing} also keep the same "
        "facet-incidence signature in every probe. This is a finite necessary-condition "
        "check, not a local-maximality proof.",
        "",
        "| relative radius | positive observed | no positive observed | invalid | states with incidence changes | median best change | 10–90% | median best slope |",
        "|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for row in aggregate:
        lines.append(
            f"| {row['relative_radius']:.0e} | {row['positive_observed_count']} | "
            f"{row['no_positive_count']} | {row['incomplete_count']} | "
            f"{row['incidence_change_count']} | "
            f"{row['median_maximum_delta_sys']:.6g} | "
            f"{row['q10_maximum_delta_sys']:.6g}–{row['q90_maximum_delta_sys']:.6g} | "
            f"{row['median_maximum_slope']:.6g} |"
        )
    lines += [
        "",
        "A state fails this check whenever any tested direction has raw positive "
        "change. Invalid probes make the corresponding radius inconclusive. An "
        "incidence change is reported separately: it leaves the full-sys comparison "
        "valid, but means that the finite move crossed into a different combinatorial "
        "cell. `state-summary.csv` records every endpoint.",
        "",
        "The diagnostic removes the tangent span of translations, scaling, and the "
        "identity-component linear symplectic action. Its signed orthonormal basis "
        "is positive spanning but not dense on the quotient sphere. Passing cannot "
        "exclude ascent between basis axes, below the smallest radius, or through "
        "branch behavior not resolved by finite probing. Failing does establish an "
        "explicit finite improving move under the current full evaluator.",
    ]
    by_state = defaultdict(list)
    for row in radius_rows:
        by_state[state_key(row["state_id"])].append(row)
    no_positive = [
        state_key(row["state_id"])
        for row in states
        if row["no_positive_at_all_radii"]
    ]
    constant_negative = 0
    trending_to_zero = 0
    for start_id in no_positive:
        rows = sorted(by_state[start_id], key=lambda row: row["relative_radius"])
        small = abs(rows[0]["max_delta_sys_per_step"])
        large = abs(rows[-1]["max_delta_sys_per_step"])
        ratio = small / large if large > 0 else float("inf")
        if 0.5 <= ratio <= 2.0:
            constant_negative += 1
        elif ratio < 0.5:
            trending_to_zero += 1
    lines += [
        "",
        "## What the radius dependence says",
        "",
        "The raw changes are small partly because the steps are small; the normalized "
        "slopes are the relevant scale. For a smooth strict local maximum one expects "
        "the best symmetric-poll change to be of order \\(r^2\\), hence change divided "
        "by step norm tends to zero from below. At a sharp nonsmooth maximum it can be "
        "of order \\(-r\\), so the normalized slope tends to a negative constant. A "
        "saddle or other nonstationary point has a positive first-order direction.",
        "",
        f"Among the {len(no_positive)} endpoints with no tested ascent, "
        f"{constant_negative} have smallest-radius and largest-radius negative slopes "
        "within a factor of two; this is compatible with a sharp ridge or corner. "
        f"{trending_to_zero} instead shrink by more than a factor of two toward zero; "
        "this is compatible with smoother second-order behavior. Three radii and a "
        "basis poll do not determine a convergence law. The per-state curves are in "
        "`endpoint-slope-by-state.png`.",
    ]
    if metadata:
        selected_metadata = [
            metadata[state_key(row["state_id"])]
            for row in states
            if state_key(row["state_id"]) in metadata
        ]
        uncertainty_count = sum(
            row["geometry_predicate_uncertainties"] > 0
            for row in selected_metadata
        )
        improving_ids = {
            state_key(row["state_id"])
            for row in states
            if not row["no_positive_at_all_radii"]
        }
        incidence_ids = {
            state_key(row["state_id"])
            for row in states
            if row["total_combinatorial_changes"] > 0
        }
        overlap_count = len(incidence_ids & improving_ids)
        overlap_label = "state" if overlap_count == 1 else "states"
        lines += [
            "",
            "## Incidence changes and numerical predicates",
            "",
            f"The source optimizer evaluation had no f64 geometry-predicate "
            f"uncertainty at "
            f"{len(selected_metadata) - uncertainty_count}/{len(selected_metadata)} "
            "endpoints. Incidence changes occurred only at the largest tested radius; "
            "all probes at the two smaller radii kept the base incidence signature. "
            f"The {len(incidence_ids)} incidence-changing states and "
            f"{len(improving_ids)} states with tested ascent overlap in only "
            f"{overlap_count} {overlap_label}, so the equal-looking counts "
            "do not describe one common failure set. "
            "This pattern is evidence for ordinary finite combinatorial wall crossings, "
            "not incidence flicker at numerical scale. It does not rule out a genuinely "
            "short or nearly redundant facet; testing facet deletion would be a "
            "different, dimension-changing perturbation.",
        ]
    if comparisons:
        old_positive = sum(row["baseline_explicit_improvement"] for row in comparisons)
        new_positive = sum(row["current_explicit_improvement"] for row in comparisons)
        changed = [row for row in comparisons if abs(row["sys_gain"]) > 1e-9]
        fixed = [
            row
            for row in comparisons
            if row["baseline_explicit_improvement"]
            and not row["current_explicit_improvement"]
        ]
        remaining = [
            row for row in comparisons if row["current_explicit_improvement"]
        ]
        lines += [
            "",
            "## Same starts with a larger optimizer budget",
            "",
            f"On the matched 16-start panel, the larger-budget run changed "
            f"{len(changed)}/16 endpoint objective values and reduced explicit "
            f"basis-poll failures from {old_positive}/16 to {new_positive}/16. "
            f"It removed the observed ascent at {len(fixed)} of the original "
            f"{old_positive} failing endpoints. Most runs stopped because the "
            "optimizer returned no proposals or reached its minimum internal distance, "
            "so the population curve plateaus rather than following one common "
            "logarithmic or power-law convergence curve.",
            "",
        ]
        if remaining:
            lines += [
                "| remaining endpoint | sys gain from larger budget | best tested one-step gain | current best slope |",
                "|---|---:|---:|---:|",
            ]
            for row in remaining:
                lines.append(
                    f"| {row['start_id']} | {row['sys_gain']:.6g} | "
                    f"{row['current_maximum_single_step_gain']:.6g} | "
                    f"{row['current_maximum_slope']:.6g} |"
                )
            lines += [
                "",
                "The first remaining positive slope is a real unresolved direction "
                "under this evaluator, although much smaller than before. The other is "
                "about four orders of magnitude smaller and changes `sys` by only about "
                "five billionths at the smallest tested radius; it should not be used "
                "as evidence of material non-convergence without a numerical-scale "
                "repeat. A slope alone gives no upper bound on the objective gap to an "
                "unknown local maximum.",
            ]
    lines += [
        "",
        f"Raw evidence: `{raw}`. The endpoint selection was fixed by the optimizer "
        "comparison before these poll outcomes were computed.",
    ]
    if followup_report:
        lines += [
            "",
            "A same-start larger-compute follow-up and its repeated endpoint polls are "
            f"reported in `{followup_report}`.",
        ]
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--optimizer-dataset", type=Path)
    parser.add_argument("--baseline-raw", type=Path)
    parser.add_argument("--baseline-optimizer-dataset", type=Path)
    parser.add_argument("--followup-report", type=Path)
    parser.add_argument("--overwrite", action="store_true")
    args = parser.parse_args()
    if args.out.exists() and not args.overwrite:
        raise ValueError(f"output already exists: {args.out}")
    args.out.mkdir(parents=True, exist_ok=args.overwrite)
    states, aggregate, summary, radius_rows = analyze(args.raw)
    metadata = optimizer_metadata(args.optimizer_dataset)
    comparisons = []
    if args.baseline_raw:
        baseline_states, _, _, _ = analyze(args.baseline_raw)
        baseline_metadata = optimizer_metadata(args.baseline_optimizer_dataset)
        comparisons = comparison_rows(
            states, baseline_states, metadata, baseline_metadata
        )
    write_csv(args.out / "state-summary.csv", states)
    write_csv(args.out / "radius-summary.csv", aggregate)
    if comparisons:
        write_csv(args.out / "compute-depth-comparison.csv", comparisons)
    plot_aggregate(aggregate, args.out)
    plot_state_slopes(radius_rows, args.out)
    write_report(
        args.out,
        args.raw,
        states,
        aggregate,
        summary,
        radius_rows,
        metadata,
        comparisons,
        args.followup_report,
    )
    print(f"analyzed {len(states)} optimizer endpoints")


if __name__ == "__main__":
    main()
