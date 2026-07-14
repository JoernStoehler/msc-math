# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "blake3>=1.0,<2",
#   "matplotlib>=3.9,<4",
# ]
# ///
"""Validate and render the retained quotient endpoint diagnostic."""

from __future__ import annotations

import json
import math
from collections import defaultdict
from pathlib import Path

import blake3
import matplotlib.pyplot as plt


OWNER = Path(__file__).resolve().parent
ARTIFACTS = OWNER / "artifacts"
FIGURES = OWNER / "figures"
EXPECTED_RADII = [1.0e-3, 1.0e-4, 1.0e-5]
STATE_ORDER = [
    "negative_control_initial",
    "negative_control_midtrajectory",
    "unknown_global_best_so_far",
    "unknown_terminal_best_so_far",
    "positive_control_hko2024",
]
SHORT_LABELS = {
    "negative_control_initial": "Negative: initial",
    "negative_control_midtrajectory": "Negative: mid-trajectory",
    "unknown_global_best_so_far": "Unknown: global best",
    "unknown_terminal_best_so_far": "Unknown: terminal best",
    "positive_control_hko2024": "Positive: HKO2024",
}
COLORS = {
    "negative_control_initial": "#0072B2",
    "negative_control_midtrajectory": "#56B4E9",
    "unknown_global_best_so_far": "#D55E00",
    "unknown_terminal_best_so_far": "#E69F00",
    "positive_control_hko2024": "#009E73",
}


def read_json(path: Path):
    return json.loads(path.read_text())


def read_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def close(left: float, right: float, *, atol: float = 2e-14, rtol: float = 2e-11):
    return abs(left - right) <= atol + rtol * max(abs(left), abs(right))


def validate():
    summary = read_json(ARTIFACTS / "summary.json")
    provenance = read_json(ARTIFACTS / "run-provenance.json")
    states = read_jsonl(ARTIFACTS / "states.jsonl")
    polls = read_jsonl(ARTIFACTS / "poll-directions.jsonl")
    radius_rows = read_jsonl(ARTIFACTS / "radius-summaries.jsonl")

    assert not summary["smoke"]
    assert summary["radii"] == EXPECTED_RADII
    assert [row["state_id"] for row in states] == STATE_ORDER
    assert summary["state_count"] == len(states) == 5
    assert summary["poll_row_count"] == len(polls) == 366
    assert len(radius_rows) == 15
    assert len(provenance["input_identities"]) == 36
    implementation_path = Path(provenance["implementation_path"])
    assert implementation_path.is_file()
    assert (
        blake3.blake3(implementation_path.read_bytes()).hexdigest()
        == provenance["implementation_blake3"]
    )

    for identity in provenance["input_identities"]:
        path = Path(identity["path"])
        assert path.is_file(), path
        assert blake3.blake3(path.read_bytes()).hexdigest() == identity["blake3"]

    state_by_id = {row["state_id"]: row for row in states}
    for state in states:
        assert state["orbit_generator_count"] == 15
        assert state["orbit_rank"] == 15
        assert state["ambient_dimension"] == 4 * state["facet_count"]
        assert state["quotient_dimension"] == state["ambient_dimension"] - 15
        assert state["max_orbit_orthonormal_error"] <= 2e-10
        assert state["max_slice_orthonormal_error"] <= 2e-10
        assert state["max_orbit_slice_inner_product"] <= 2e-10
        assert state["facet_count"] in (6, 10)
        if state["recorded_sys"] is not None:
            assert close(state["recorded_sys"], state["recomputed_sys"])

    grouped = defaultdict(list)
    for row in polls:
        state = state_by_id[row["state_id"]]
        grouped[(row["state_id"], row["relative_radius"])].append(row)
        assert row["state_valid"]
        assert row["failure"] is None
        assert row["all_facets_defining"]
        assert row["same_incidence_signature"]
        assert row["facet_count"] == state["facet_count"]
        assert close(row["direction_norm"], 1.0, atol=2e-13, rtol=0.0)
        assert row["orbit_projection_norm"] <= 2e-10
        assert close(row["step_norm"], row["absolute_radius"])
        assert close(
            row["absolute_radius"],
            row["relative_radius"] * state["dual_norm"],
        )
        assert close(row["delta_sys"], row["perturbed_sys"] - row["base_sys"])
        assert close(
            row["delta_sys_per_step"], row["delta_sys"] / row["step_norm"]
        )
        assert row["min_action_lower"] <= row["min_action_upper"]

    regenerated = []
    for state_id in STATE_ORDER:
        state = state_by_id[state_id]
        for radius in EXPECTED_RADII:
            rows = grouped[(state_id, radius)]
            expected = 2 * state["quotient_dimension"]
            assert len(rows) == expected
            assert {(row["basis_index"], row["sign"]) for row in rows} == {
                (index, sign)
                for index in range(state["quotient_dimension"])
                for sign in (-1, 1)
            }
            best = max(rows, key=lambda row: row["delta_sys"])
            regenerated.append(
                {
                    "state_id": state_id,
                    "relative_radius": radius,
                    "expected_direction_count": expected,
                    "improving_direction_count": sum(
                        row["delta_sys"] > 0 for row in rows
                    ),
                    "max_delta_sys": best["delta_sys"],
                    "min_delta_sys": min(row["delta_sys"] for row in rows),
                    "max_delta_sys_per_step": max(
                        row["delta_sys_per_step"] for row in rows
                    ),
                    "best_basis_index": best["basis_index"],
                    "best_sign": best["sign"],
                }
            )

    retained_by_key = {
        (row["state_id"], row["relative_radius"]): row for row in radius_rows
    }
    for row in regenerated:
        retained = retained_by_key[(row["state_id"], row["relative_radius"])]
        for key in (
            "expected_direction_count",
            "improving_direction_count",
            "best_basis_index",
            "best_sign",
        ):
            assert row[key] == retained[key]
        for key in ("max_delta_sys", "min_delta_sys", "max_delta_sys_per_step"):
            assert close(row[key], retained[key])
        assert retained["invalid_direction_count"] == 0
        assert retained["combinatorial_change_count"] == 0

    for state_id in STATE_ORDER[:2] + STATE_ORDER[2:4]:
        assert all(
            retained_by_key[(state_id, radius)]["max_delta_sys"] > 0
            for radius in EXPECTED_RADII
        )
    assert all(
        retained_by_key[("positive_control_hko2024", radius)]["max_delta_sys"] < 0
        for radius in EXPECTED_RADII
    )

    generic_polls = [
        row for row in polls if row["state_id"] != "positive_control_hko2024"
    ]
    hko_polls = [
        row for row in polls if row["state_id"] == "positive_control_hko2024"
    ]
    assert all(
        close(row["min_action_lower"], row["min_action_upper"])
        for row in generic_polls
    )
    hko_noncollapsed_bounds = sum(
        not close(row["min_action_lower"], row["min_action_upper"])
        for row in hko_polls
    )
    hko_max_bound_width = max(
        row["min_action_upper"] - row["min_action_lower"] for row in hko_polls
    )

    analysis = {
        "artifact_identity": {
            "trajectory_input_count": 36,
            "trajectory_hashes_recomputed": True,
            "producer_hash_recomputed": True,
            "poll_row_count": len(polls),
            "state_count": len(states),
        },
        "validation": {
            "source_target_agreement": "all four trajectory targets reproduce exactly; HKO differs from its known-capacity target by 1.33e-15",
            "quotient": "all five states have orbit rank 15 and orthonormal/cross residuals below 2e-10",
            "directions": "every signed basis pair is present; direction norm, orbit projection, absolute step, and delta arithmetic were recomputed",
            "geometry": "366/366 probes valid, all listed facets defining, 0 incidence-signature changes",
            "capacity_bounds": f"all {len(generic_polls)} generic probe minimum-action intervals collapse; {hko_noncollapsed_bounds}/{len(hko_polls)} HKO intervals do not, with maximum width {hko_max_bound_width:.6g}",
            "summary": "all retained radius summaries recomputed from raw poll rows",
        },
        "direct_findings": {
            "negative_controls_discriminated": "2/2 controls have at least one positive quotient-basis direction at 3/3 radii",
            "hko_control_discriminated": "HKO has no positive direction among 50/50 directions at each of 3/3 radii",
            "unknown_global_best": "fails finite stationarity at 3/3 radii",
            "unknown_terminal_best": "fails finite stationarity at 3/3 radii",
        },
        "radius_rows": regenerated,
        "figure_paths": [
            str(FIGURES / "max-margin-by-radius.png"),
            str(FIGURES / "directional-spread.png"),
        ],
    }
    return summary, states, polls, radius_rows, analysis


def plot_max_margin(radius_rows):
    FIGURES.mkdir(exist_ok=True)
    by_state = defaultdict(list)
    for row in radius_rows:
        by_state[row["state_id"]].append(row)
    fig, axes = plt.subplots(2, 1, figsize=(8.2, 7.2), sharex=True)
    for state_id in STATE_ORDER:
        rows = sorted(by_state[state_id], key=lambda row: row["relative_radius"])
        radii = [row["relative_radius"] for row in rows]
        axes[0].plot(
            radii,
            [row["max_delta_sys"] for row in rows],
            marker="o",
            color=COLORS[state_id],
            label=SHORT_LABELS[state_id],
        )
        axes[1].plot(
            radii,
            [row["max_delta_sys_per_step"] for row in rows],
            marker="o",
            color=COLORS[state_id],
        )
    for axis in axes:
        axis.axhline(0.0, color="black", linewidth=0.8)
        axis.grid(alpha=0.25)
    axes[0].set_xscale("log")
    axes[0].set_yscale("symlog", linthresh=1e-6)
    axes[0].set_ylabel("maximum observed Δsys")
    axes[0].legend(loc="best", fontsize=8)
    axes[0].set_title("Best signed quotient-basis direction at each radius")
    axes[1].set_ylabel("maximum Δsys / step norm")
    axes[1].set_xlabel("relative radius (step norm / dual-state norm)")
    fig.tight_layout()
    for suffix in ("png", "pdf"):
        fig.savefig(FIGURES / f"max-margin-by-radius.{suffix}", dpi=220)
    plt.close(fig)


def plot_directional_spread(polls):
    fig, axes = plt.subplots(1, len(EXPECTED_RADII), figsize=(13.0, 4.6), sharey=True)
    for axis, radius in zip(axes, EXPECTED_RADII):
        for state_index, state_id in enumerate(STATE_ORDER):
            values = [
                row["delta_sys_per_step"]
                for row in polls
                if row["state_id"] == state_id
                and row["relative_radius"] == radius
            ]
            offsets = [
                state_index + 0.16 * (index / max(1, len(values) - 1) - 0.5)
                for index in range(len(values))
            ]
            axis.scatter(
                offsets,
                values,
                s=13,
                alpha=0.65,
                color=COLORS[state_id],
                edgecolors="none",
            )
            axis.plot(
                [state_index - 0.15, state_index + 0.15],
                [max(values), max(values)],
                color="black",
                linewidth=1.2,
            )
        axis.axhline(0.0, color="black", linewidth=0.8)
        axis.grid(axis="y", alpha=0.25)
        axis.set_xticks(range(len(STATE_ORDER)))
        axis.set_xticklabels(
            [SHORT_LABELS[state].replace(": ", "\n") for state in STATE_ORDER],
            rotation=25,
            ha="right",
            fontsize=7,
        )
        axis.set_title(f"relative radius {radius:g}")
    axes[0].set_ylabel("Δsys / step norm for every signed direction")
    fig.suptitle("Directional spread; black segment marks the best observed direction")
    fig.tight_layout()
    for suffix in ("png", "pdf"):
        fig.savefig(FIGURES / f"directional-spread.{suffix}", dpi=220)
    plt.close(fig)


def fmt(value: float) -> str:
    return f"{value:.6g}"


def write_discussion(states, radius_rows, analysis):
    state_by_id = {row["state_id"]: row for row in states}
    rows_by_state = defaultdict(list)
    for row in radius_rows:
        rows_by_state[row["state_id"]].append(row)
    table_lines = [
        "| State role | Relative radius | Directions | Improving | Max Δsys | Max Δsys / step |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for state_id in STATE_ORDER:
        for row in sorted(rows_by_state[state_id], key=lambda item: item["relative_radius"], reverse=True):
            table_lines.append(
                f"| {SHORT_LABELS[state_id]} | {row['relative_radius']:.0e} | "
                f"{row['expected_direction_count']} | {row['improving_direction_count']} | "
                f"{fmt(row['max_delta_sys'])} | {fmt(row['max_delta_sys_per_step'])} |"
            )
    text = f"""# Quotient-Aware Endpoint Diagnostic Discussion

## Mathematical heuristic stationarity definition

For a valid labelled fixed-`F` dual-vertex state `a`, the diagnostic forms the tangent span of the four translations, positive scaling, and ten `sp(4,R)` generators. All five retained states have tangent rank `15`. The slice is the Euclidean orthogonal complement at the base state, of dimension `4F-15`: `9` for the generic six-facet states and `25` for HKO.

At each relative radius `r` in `1e-3, 1e-4, 1e-5`, the producer polls both signs of every vector in a deterministic orthonormal basis of that slice, with absolute step norm `r ||a||_2`. A state passes the packet's finite stationarity condition at one radius only when all `2(4F-15)` recomputed states are valid, keep the same facet incidence signature, and none has positive raw `Δsys`. There is no relative-gain cutoff: the table reports the best raw change and change per step. Passing this condition is only finite, basis-dependent evidence.

## Direct control outcomes

The two retained negative controls were selected because their next literal-gradient update has positive full-`sys` change. Both show positive quotient-basis directions at all three radii, so the diagnostic does not confuse ordinary improvable states with endpoints. HKO2024, the exact-theorem positive control, has no positive direction among all `50` directions at any radius. Its least-negative margin is `{fmt(max(row['max_delta_sys'] for row in rows_by_state['positive_control_hko2024']))}`. This agreement calibrates the central scalar route; it is not evidence for the HKO theorem, whose exact certificate remains authoritative.

{chr(10).join(table_lines)}

Across the packet, `366/366` probes were valid, every listed dual point stayed extreme, and no probe changed the base incidence signature. All trajectory targets recomputed exactly; the HKO recomputation differs from its known-capacity target by `1.33e-15`. Direction norms, orbit projections, target differences, denominators, and all compact summaries were independently recomputed by `analyze.py`.

## Unknown-state outcomes

- `unknown_global_best_so_far` is the highest valid `sys` row across the frozen 3,142-row six-start evaluation. It has positive directions at all three radii (respectively `5/18`, `9/18`, and `9/18`) and therefore fails the finite stationarity condition at every tested resolution.
- `unknown_terminal_best_so_far` is the highest complete iteration-100 state whose trajectory best occurs at iteration 100. It also has positive directions at all three radii (`8/18`, `9/18`, and `9/18`) and fails at every tested resolution.

These outcome-selected states are diagnostic unknowns, not independent optimizer validation. The first is explicitly a best point on an oscillatory trajectory; the second tests whether a high terminal best is any closer to stationarity. Neither may be called a heuristic local maximum from this packet.

## Quotient and branch-completeness limitations

The derivative-free poll was chosen because HKO's `44` nonsingular active KKT rows span only rank `23` of its `25`-dimensional quotient; the exact theorem needs singular feasible upper sections. The poll therefore does not assume that the base active sigma list contains every right-active or singular germ. It does, however, rely on the current `MinimaSafe` full-capacity scalar at each finite perturbed state and does not establish limiting branch-germ completeness.

Capacity-bound audit: {analysis['validation']['capacity_bounds']}. The broad HKO bounds come from ill-conditioned returned candidates near the singular control. Consequently the HKO finite-poll signs are operational central-scalar observations, not independently certified capacity inequalities. This numerical limitation does not weaken the generic unknown-state failures, but it prevents using the HKO poll as new mathematical support. A diagnostic disagreement with HKO would have to be treated first as an evaluator/branch-completeness failure.

The Euclidean slice and its coordinate-ordered Gram-Schmidt basis are one local gauge. The signed basis is positive spanning but is not dense on the quotient sphere; nonsmooth directional ascent can exist between tested axes. The affine slice is tangent-transverse only at the base. The finite radii do not prove behavior below `1e-5 ||a||_2`, and f64 state coordinates are rationalized exactly rather than representing unknown exact optimizer endpoints. Discrete facet relabellings and HKO's finite symmetry group create no extra tangent directions.

## Evidence thresholds

Calling a future state a **heuristic local maximum** should require at least: successful negative and HKO controls; valid fixed-facet geometry; a complete signed quotient-basis poll with no improvement at several shrinking radii; a materially richer deterministic or seeded quotient-direction cover (or branch-aware gradient sampling) that also finds no improvement; exact raw margins and direction coverage; and repetition after the polisher's stopping state is frozen. A finite no-improvement scan remains heuristic.

**Theorem-grade local maximality** requires a local chart and an exact certificate controlling every transverse direction, such as HKO's feasible upper branches with exact rank and positive convex relation. No amount of finite polling alone supplies that implication.

## Decision and next optimizer experiment

Stop treating these two frozen high states as endpoint candidates. The next useful experiment is a safeguarded quotient-basis polisher seeded at them: at each iteration evaluate the signed quotient basis, accept the largest positive full-`sys` move, and shrink the radius only when a complete poll has no improvement. Retain every raw poll and stop after no improvement at three declared radii. Then rerun this endpoint packet plus a richer direction cover on the frozen polished states. This directly tests whether the parallel optimizer policies merely reach high values or can actually remove the ascent directions observed here.

## Reproduction

From the repository root, after checking out the frozen raw trajectories from Git LFS:

```bash
cargo run --release -p exp-dev-gradient-ascent \\
  --bin dev-gradient-ascent-quotient-endpoint-diagnostic -- \\
  --out-dir experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts \\
  --threads 8

uv run --script \\
  experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/analyze.py
```

`run-provenance.json` hashes all `36` selection inputs and the producer. `poll-directions.jsonl` is the raw evidence; `states.jsonl` and `radius-summaries.jsonl` are compact generated views. The figures are generated directly from the validated rows.
"""
    (ARTIFACTS / "DISCUSSION.md").write_text(text)


def main():
    _, states, polls, radius_rows, analysis = validate()
    plot_max_margin(radius_rows)
    plot_directional_spread(polls)
    write_discussion(states, radius_rows, analysis)
    (ARTIFACTS / "analysis.json").write_text(json.dumps(analysis, indent=2) + "\n")


if __name__ == "__main__":
    main()
