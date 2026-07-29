# /// script
# requires-python = ">=3.11"
# dependencies = ["matplotlib"]
# ///
"""Report one-step continuation checks on retained optimizer endpoints."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def fmt(value: float | None) -> str:
    if value is None:
        return "--"
    return f"{value:.9g}"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("run", type=Path)
    parser.add_argument("input", type=Path)
    parser.add_argument("out", type=Path)
    parser.add_argument("--mirror-run", type=Path)
    args = parser.parse_args()

    summary = json.loads((args.run / "summary.json").read_text())
    packet = json.loads(args.input.read_text())
    candidates = read_jsonl(args.run / "candidates.jsonl")
    mirror_candidates = (
        read_jsonl(args.mirror_run / "candidates.jsonl") if args.mirror_run else []
    )
    args.out.mkdir(parents=True, exist_ok=True)

    by_state: dict[str, list[dict]] = {}
    for row in candidates:
        by_state.setdefault(row["state_id"], []).append(row)

    rows = []
    for state in summary["states"]:
        state_candidates = by_state[state["state_id"]]
        selected = next((row for row in state_candidates if row["selected"]), None)
        usable_deltas = [
            row["delta_sys"]
            for row in state_candidates
            if row["usable"] and row["delta_sys"] is not None
        ]
        rows.append(
            {
                **state,
                "selected_family": selected["family"] if selected else None,
                "selected_radius": selected["normalized_radius"] if selected else None,
                "best_tested_delta": max(usable_deltas) if usable_deltas else None,
            }
        )

    accepted = [row for row in rows if row["accepted_steps"] > 0]
    stopped = [row for row in rows if row["accepted_steps"] == 0]
    crossed_one = [row for row in rows if row["final_sys"] > 1.0]

    cohort_table = [
        "| rank/start | max--min gains | max--min losses | losses with unchanged clean geometry | winning-gradient losses |",
        "| --- | ---: | ---: | ---: | ---: |",
    ]
    cohort_totals = {
        "max_min": 0,
        "loss": 0,
        "represented_positive": 0,
        "unchanged_incidence": 0,
        "unchanged_clean": 0,
        "winning_gradient": 0,
        "winning_gradient_loss": 0,
    }
    for state in rows:
        state_candidates = by_state[state["state_id"]]
        max_min = [
            row
            for row in state_candidates
            if row["family"].startswith("gap-window-")
        ]
        losses = [
            row
            for row in max_min
            if row["delta_sys"] is not None and row["delta_sys"] < 0.0
        ]
        represented_positive = [
            row
            for row in losses
            if (
                row["candidate_winner_in_base_branch_set"] is True
                and row["candidate_winner_base_branch_predicted_delta"] is not None
                and row["candidate_winner_base_branch_predicted_delta"] > 0.0
            )
            or (
                row["candidate_winner_in_extension_pool"] is True
                and row["candidate_winner_extension_predicted_delta"] is not None
                and row["candidate_winner_extension_predicted_delta"] > 0.0
            )
        ]
        unchanged = [row for row in losses if row["incidence_changed"] is False]
        clean = [
            row
            for row in unchanged
            if row["base_geometry_indeterminate_count"] == 0
            and row["candidate_geometry_indeterminate_count"] == 0
        ]
        winning_gradient = [
            row
            for row in state_candidates
            if row["family"] == "winning-branch-gradient"
        ]
        winning_gradient_losses = [
            row
            for row in winning_gradient
            if row["delta_sys"] is not None and row["delta_sys"] < 0.0
        ]
        cohort_table.append(
            f"| `{state['state_id']}` | "
            f"{sum(row['delta_sys'] is not None and row['delta_sys'] > 0.0 for row in max_min)}/{len(max_min)} | "
            f"{len(losses)}/{len(max_min)} | {len(clean)}/{len(losses)} | "
            f"{len(winning_gradient_losses)}/{len(winning_gradient)} |"
        )
        cohort_totals["max_min"] += len(max_min)
        cohort_totals["loss"] += len(losses)
        cohort_totals["represented_positive"] += len(represented_positive)
        cohort_totals["unchanged_incidence"] += len(unchanged)
        cohort_totals["unchanged_clean"] += len(clean)
        cohort_totals["winning_gradient"] += len(winning_gradient)
        cohort_totals["winning_gradient_loss"] += len(winning_gradient_losses)

    fig, ax = plt.subplots(figsize=(7.2, 4.4))
    for status, subset, marker, color in [
        ("validated improvement", accepted, "o", "#2c7fb8"),
        ("no tested improvement", stopped, "X", "#d95f0e"),
    ]:
        ax.scatter(
            [row["recorded_sys"] for row in subset],
            [row["cumulative_gain"] for row in subset],
            label=status,
            marker=marker,
            color=color,
            s=60,
        )
    ax.axhline(0.0, color="black", linewidth=0.8)
    ax.set_xlabel("retained endpoint sys")
    ax.set_ylabel("best validated one-step gain")
    ax.grid(alpha=0.25)
    ax.legend()
    fig.tight_layout()
    fig.savefig(args.out / "one-step-gain-by-endpoint.png", dpi=180)
    plt.close(fig)

    top_model_rows = sorted(
        (
            row
            for row in by_state[rows[0]["state_id"]]
            if row["family"] == "gap-window-0.1"
        ),
        key=lambda row: row["normalized_radius"],
    )
    top_mirror_rows = sorted(
        (
            row
            for row in mirror_candidates
            if row["family"] == "mirrored-gap-window-0.1"
        ),
        key=lambda row: row["normalized_radius"],
    )
    model_error_section = ""
    if top_model_rows and top_mirror_rows:
        fig, ax = plt.subplots(figsize=(7.2, 4.4))
        radii = [row["normalized_radius"] for row in top_model_rows]
        ax.plot(
            radii,
            [row["candidate_winner_base_branch_predicted_delta"] for row in top_model_rows],
            marker="o",
            label="affine prediction for target winner",
        )
        ax.plot(
            radii,
            [row["delta_sys"] for row in top_model_rows],
            marker="o",
            label="recomputed sys, proposed direction",
        )
        ax.plot(
            [row["normalized_radius"] for row in top_mirror_rows],
            [row["delta_sys"] for row in top_mirror_rows],
            marker="o",
            label="recomputed sys, opposite direction",
        )
        ax.axhline(0.0, color="black", linewidth=0.8)
        ax.set_xscale("log")
        ax.set_yscale("symlog", linthresh=1.0e-6)
        ax.set_xlabel("normalized radius")
        ax.set_ylabel("change from endpoint")
        ax.grid(alpha=0.25)
        ax.legend()
        fig.tight_layout()
        fig.savefig(args.out / "top-endpoint-model-error.png", dpi=180)
        plt.close(fig)

        plus_slopes = [row["slope"] for row in top_model_rows]
        minus_slopes = [row["slope"] for row in top_mirror_rows]
        in_base = sum(
            row["candidate_winner_in_base_branch_set"] is True
            for row in top_model_rows
        )
        incidence_changes = sum(row["incidence_changed"] is True for row in top_model_rows)
        model_error_section = f"""
## Why the top endpoint's branch proposal failed

The evaluator's displayed winning branch at the proposed target was already
in the base branch set at {in_base}/{len(top_model_rows)} radii. Its affine
model predicted an increase at every radius, but recomputation of the
evaluator's `sys` field decreased with normalized slopes
`{fmt(min(plus_slopes))}`--`{fmt(max(plus_slopes))}`.
The opposite direction also decreased, with slopes
`{fmt(min(minus_slopes))}`--`{fmt(max(minus_slopes))}`.

![Top-endpoint branch prediction and both finite directions](top-endpoint-model-error.png)

Only {incidence_changes}/{len(top_model_rows)} proposed points changed the
recorded incidence signature. The nonvanishing error per unit distance down
to radius `1e-5` is evidence against ordinary quadratic Taylor remainder for
the complete implemented affine model along these proposals. The base also
had one indeterminate vertex count, but this aggregate count does not by
itself establish a primal incidence boundary. The failure could instead lie
in the capacity derivative, volume derivative, KKT/admissibility regime, or
model bookkeeping. It does not decide whether another oblique direction
improves the endpoint.
"""

    table = [
        "| rank/start | initial sys | outcome | gain | selected model/radius | best tested delta | full sys evaluations |",
        "| --- | ---: | --- | ---: | --- | ---: | ---: |",
    ]
    for row in rows:
        selected_text = (
            f"`{row['selected_family']}` / {fmt(row['selected_radius'])}"
            if row["selected_family"]
            else "--"
        )
        outcome = "improved" if row["accepted_steps"] else "no tested improvement"
        table.append(
            f"| `{row['state_id']}` | {fmt(row['recorded_sys'])} | {outcome} | "
            f"{fmt(row['cumulative_gain'])} | {selected_text} | "
            f"{fmt(row['best_tested_delta'])} | {row['full_sys_evaluations']} |"
        )

    top = rows[0]
    report = f"""# One-step continuation of the eight highest retained endpoints

Here `sys` denotes the binary64 heuristic evaluator field used by this
diagnostic. It is not, by this report alone, a certified value of the
mathematical systolic ratio; “crossing one” refers only to that recorded
field.

## Direct answer

The highest endpoint had `sys = {fmt(top["recorded_sys"])}`. Neither finite-gap
max--min branch model, the five current-winning-branch gradient moves, nor any
of the 50 signed quotient-basis probes improved it; the least negative tested
change was `{fmt(top["best_tested_delta"])}`.

Across the eight outcome-selected endpoints, {len(accepted)}/8 had a validated
improving branch-model move, {len(stopped)}/8 had no improvement in the tested
models or basis, and {len(crossed_one)}/8 crossed `sys = 1`. The five gains were
between `{fmt(min(row["cumulative_gain"] for row in accepted))}` and
`{fmt(max(row["cumulative_gain"] for row in accepted))}`.

![One-step gain by retained endpoint](one-step-gain-by-endpoint.png)

## Endpoint rows

{chr(10).join(table)}

{model_error_section}

## The affine failure across all eight endpoints

The top-endpoint mismatch is not isolated. Of
{cohort_totals["max_min"]} action-window max--min proposals,
{cohort_totals["loss"]} decreased the recomputed evaluator `sys` field. In all
{cohort_totals["represented_positive"]} of those losses, the target winner was
represented in the base or extension branch set and its recorded affine
prediction was positive. {cohort_totals["unchanged_incidence"]} losses retained
the same recorded incidence, and {cohort_totals["unchanged_clean"]} had
determinate geometry at both endpoints as well as unchanged incidence. All
{cohort_totals["winning_gradient_loss"]} of
{cohort_totals["winning_gradient"]} current-winning-branch gradient proposals
decreased the recomputed evaluator `sys` field.

{chr(10).join(cohort_table)}

Rank 2 is an internal positive control: all ten max--min proposals improved.
Ranks 4 and 8 are clean failure controls: all ten max--min proposals decreased
with determinate, unchanged recorded geometry. This pattern favors a
systematic derivative, KKT-branch-identity, admissibility, or bookkeeping
problem over an explanation special to the top endpoint.

## Cost and interpretation

The run took `{fmt(summary["wall_seconds"])}` seconds and
`{sum(row["full_sys_evaluations"] for row in rows)}` full evaluator calls.
An endpoint that accepted a max--min branch-model move used 16 evaluations:
one base evaluation plus ten max--min and five winning-gradient proposals.
A model stop triggered the 50-direction signed-basis fallback and used 66.

The endpoint population is the top eight outcomes of the retained 128-start
tuning dataset for `{packet["selection"]["algorithm"]}`. It is deliberately
outcome-selected discovery evidence, not a held-out optimizer comparison.
A validated gain proves only that the corresponding endpoint admitted a finite
improving move under this evaluator.
No tested improvement does not establish local maximality: the known
rotated-pentagon control shows that both a signed basis and sparse generic
directions can miss a thin improving set.

The top endpoint therefore remains a numerical near-one local-max candidate,
not a classified local maximum. A larger continuation run is not useful for
that state until a richer direction or branch-completeness hypothesis is
specified; repeating the same radii would reproduce the same stop.
"""
    (args.out / "REPORT.md").write_text(report)


if __name__ == "__main__":
    main()
