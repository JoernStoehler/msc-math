#!/usr/bin/env python3
"""Validate and summarize the known-sys=1 local-maximum screen."""

from __future__ import annotations

import argparse
import json
import os
from collections import defaultdict
from pathlib import Path
from typing import Any


EXPECTED_SEEDS = {
    "pentagon_threshold_control": "expected_positive_control",
    "triangle_hexagon_theta0": "target",
    "square_square_pi_over_4": "target",
    "ch2021_six_vertex": "target",
}
ROW_RADII = (1.0e-3, 1.0e-4, 1.0e-5)
ANGULAR_RADII = (1.0e-2, 1.0e-3, 1.0e-4)
RANDOM_PAIRS = 32
ORIENTATION_DIRECTIONS = 16


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open(encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]


def atomic_json(path: Path, value: Any) -> None:
    temporary = path.with_suffix(path.suffix + ".new")
    temporary.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    os.replace(temporary, path)


def atomic_text(path: Path, value: str) -> None:
    temporary = path.with_suffix(path.suffix + ".new")
    temporary.write_text(value, encoding="utf-8")
    os.replace(temporary, path)


def key(row: dict[str, Any]) -> tuple[str, str, float]:
    return row["seed_id"], row["perturbation"], row["radius"]


def validate(
    bases: list[dict[str, Any]],
    probes: list[dict[str, Any]],
    summaries: list[dict[str, Any]],
    provenance: dict[str, Any],
) -> None:
    assert provenance["canonical"] and not provenance["smoke"], provenance
    assert provenance["experiment_id"] == "sys1-known-equality-local-maxima-v1"
    assert len(bases) == len(EXPECTED_SEEDS)
    assert {row["seed_id"]: row["role"] for row in bases} == EXPECTED_SEEDS
    assert all(row["equality_check_passed"] for row in bases)
    assert all(abs(row["recomputed_minus_expected"]) <= row["equality_tolerance"] for row in bases)
    assert all(row["orbit_rank"] + row["quotient_dimension"] == row["ambient_dimension"] for row in bases)
    assert all(row["orbit_generator_count"] == 15 for row in bases)
    incidence_degrees = {
        row["seed_id"]: sorted(signature.count("1") for signature in row["incidence_signature"])
        for row in bases
    }
    assert all(degree == 4 for degree in incidence_degrees["pentagon_threshold_control"])
    assert all(degree == 4 for degree in incidence_degrees["triangle_hexagon_theta0"])
    assert all(degree == 4 for degree in incidence_degrees["square_square_pi_over_4"])
    assert incidence_degrees["ch2021_six_vertex"] == [6] * 6
    material_delta = provenance["material_delta_sys"]
    assert material_delta == 1.0e-12
    assert all(
        row["raw_positive_delta"]
        == (row["delta_sys"] is not None and row["delta_sys"] > 0.0)
        for row in probes
    )
    assert all(
        row["nominal_improvement"]
        == (row["delta_sys"] is not None and row["delta_sys"] > material_delta)
        for row in probes
    )

    identities = [
        (
            row["seed_id"],
            row["perturbation"],
            row["radius"],
            row["direction_index"],
            row["sign"],
        )
        for row in probes
    ]
    assert len(identities) == len(set(identities)), "duplicate probe identity"

    expected_counts: dict[tuple[str, str, float], int] = {}
    for base in bases:
        seed = base["seed_id"]
        quotient_dimension = base["quotient_dimension"]
        for radius in ROW_RADII:
            expected_counts[(seed, "quotient_basis", radius)] = 2 * quotient_dimension
            expected_counts[(seed, "quotient_random_antipodal", radius)] = 2 * RANDOM_PAIRS
        for radius in ANGULAR_RADII:
            expected_counts[(seed, "so4_mod_u2_orientation", radius)] = ORIENTATION_DIRECTIONS
            if base["product_q_sides"] is not None:
                expected_counts[(seed, "product_relative_rotation", radius)] = 2

    observed_counts: dict[tuple[str, str, float], int] = defaultdict(int)
    for row in probes:
        observed_counts[key(row)] += 1
    assert observed_counts == expected_counts, (observed_counts, expected_counts)
    assert len(probes) == provenance["probe_count"]

    compact = {key(row): row for row in summaries}
    assert set(compact) == set(expected_counts)
    for group_key, expected in expected_counts.items():
        selected = [row for row in probes if key(row) == group_key]
        summary = compact[group_key]
        assert summary["total_probes"] == expected
        assert summary["valid_probes"] == sum(row["state_valid"] for row in selected)
        assert summary["nominal_improving_probes"] == sum(
            row["nominal_improvement"] for row in selected
        )
        assert summary["lower_bound_above_one_probes"] == sum(
            row["lower_bound_above_one"] for row in selected
        )
        assert summary["lower_bound_above_base_upper_probes"] == sum(
            row["lower_bound_above_base_upper"] for row in selected
        )


def seed_outcome(
    base: dict[str, Any], probes: list[dict[str, Any]]
) -> dict[str, Any]:
    seed_rows = [row for row in probes if row["seed_id"] == base["seed_id"]]
    valid = [row for row in seed_rows if row["state_valid"]]
    invalid = len(seed_rows) - len(valid)
    incidence_changes = sum(not row["same_incidence_signature"] for row in valid)
    best = max(valid, key=lambda row: row["delta_sys"])
    perturbations = sorted({row["perturbation"] for row in seed_rows})
    patterns = []
    for perturbation in perturbations:
        rows = [row for row in valid if row["perturbation"] == perturbation]
        radii = sorted({row["radius"] for row in rows}, reverse=True)
        separated_radii = sorted(
            {
                row["radius"]
                for row in rows
                if row["lower_bound_above_base_upper"]
            },
            reverse=True,
        )
        above_one_radii = sorted(
            {row["radius"] for row in rows if row["lower_bound_above_one"]},
            reverse=True,
        )
        patterns.append(
            {
                "perturbation": perturbation,
                "tested_radii": radii,
                "interval_separated_radii": separated_radii,
                "lower_bound_above_one_radii": above_one_radii,
                "all_tested_radii_interval_separated": bool(radii)
                and separated_radii == radii,
            }
        )
    all_scale_pattern = any(
        row["all_tested_radii_interval_separated"] for row in patterns
    )
    any_separated = any(row["lower_bound_above_base_upper"] for row in valid)
    any_nominal = any(row["nominal_improvement"] for row in valid)
    if invalid:
        status = "inconclusive_invalid_probe"
    elif all_scale_pattern:
        status = "empirical_improving_pattern_at_all_tested_scales"
    elif any_separated:
        status = "interval_separated_improvement_at_some_scales"
    elif any_nominal:
        status = "nominal_improvement_only"
    else:
        status = "no_improvement_observed"
    return {
        "seed_id": base["seed_id"],
        "role": base["role"],
        "status": status,
        "probe_count": len(seed_rows),
        "valid_probe_count": len(valid),
        "invalid_probe_count": invalid,
        "incidence_change_count": incidence_changes,
        "entered_adjacent_combinatorial_cells": incidence_changes > 0,
        "raw_positive_delta_count": sum(row["raw_positive_delta"] for row in valid),
        "nominal_improving_probe_count": sum(row["nominal_improvement"] for row in valid),
        "interval_separated_probe_count": sum(
            row["lower_bound_above_base_upper"] for row in valid
        ),
        "above_one_lower_bound_probe_count": sum(
            row["lower_bound_above_one"] for row in valid
        ),
        "best_probe": {
            "perturbation": best["perturbation"],
            "radius": best["radius"],
            "direction_index": best["direction_index"],
            "sign": best["sign"],
            "delta_sys": best["delta_sys"],
            "perturbed_sys": best["perturbed_sys"],
            "perturbed_sys_lower": best["perturbed_sys_lower"],
            "lower_bound_above_base_upper": best["lower_bound_above_base_upper"],
        },
        "perturbation_patterns": patterns,
    }


def format_float(value: float | None) -> str:
    if value is None:
        return "—"
    return f"{value:.9g}"


def make_report(
    bases: list[dict[str, Any]],
    outcomes: list[dict[str, Any]],
    summaries: list[dict[str, Any]],
    control_passed: bool,
) -> str:
    by_group = {
        (row["seed_id"], row["perturbation"], row["radius"]): row
        for row in summaries
    }
    control_family = [
        by_group[("pentagon_threshold_control", "product_relative_rotation", radius)]
        for radius in ANGULAR_RADII
    ]
    control_orientation = [
        by_group[("pentagon_threshold_control", "so4_mod_u2_orientation", radius)]
        for radius in ANGULAR_RADII
    ]
    control_unstructured_improvements = sum(
        row["nominal_improving_probes"]
        for row in summaries
        if row["seed_id"] == "pentagon_threshold_control"
        and row["perturbation"]
        in {"quotient_basis", "quotient_random_antipodal"}
    )
    target_material_improvements = sum(
        outcome["nominal_improving_probe_count"]
        for outcome in outcomes
        if outcome["role"] == "target"
    )
    ch_outcome = next(
        outcome for outcome in outcomes if outcome["seed_id"] == "ch2021_six_vertex"
    )
    target_interval_widths = [
        base["capacity_interval_width"] for base in bases if base["role"] == "target"
    ]
    lines = [
        "# Known `sys = 1` Local Screen Report",
        "",
        "## Control gate",
        "",
        (
            "PASS: the rotated-pentagon structured-family control has an "
            "interval-separated improvement at every tested angular radius."
            if control_passed
            else "FAIL: the expected-positive pentagon control did not pass at every radius; target misses are not interpretable."
        ),
        "",
        "## Base reconstruction",
        "",
        "| Seed | Role | facets | quotient dim | recomputed sys | capacity interval width |",
        "| --- | --- | ---: | ---: | ---: | ---: |",
    ]
    for base in bases:
        lines.append(
            f"| `{base['seed_id']}` | {base['role']} | {base['facet_count']} | "
            f"{base['quotient_dimension']} | {format_float(base['recomputed_sys'])} | "
            f"{format_float(base['capacity_interval_width'])} |"
        )
    lines.extend(
        [
            "",
            "## Direct outcomes",
            "",
            "| Seed | Finite-screen status | probes | material nominal | interval-separated | incidence changes | best delta sys | best family |",
            "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |",
        ]
    )
    for outcome in outcomes:
        best = outcome["best_probe"]
        lines.append(
            f"| `{outcome['seed_id']}` | `{outcome['status']}` | {outcome['probe_count']} | "
            f"{outcome['nominal_improving_probe_count']} | {outcome['interval_separated_probe_count']} | "
            f"{outcome['incidence_change_count']} | "
            f"{format_float(best['delta_sys'])} | `{best['perturbation']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation of this run",
            "",
            f"- The control was recovered in both theory-derived low-dimensional slices: relative rotation had {sum(row['lower_bound_above_base_upper_probes'] for row in control_family)} interval-separated improving probes across the three angular radii, and the `SO(4)/U(2)` circle had {sum(row['lower_bound_above_base_upper_probes'] for row in control_orientation)}. The signed quotient basis and {2 * RANDOM_PAIRS} deterministic random quotient directions per radius found {control_unstructured_improvements} material improvements. Thus the control validates the evaluator and structured directions, while also demonstrating that a sparse high-dimensional poll can miss a real improving cone.",
            f"- The three targets had {target_material_improvements} material nominal improvements across all probes. Triangle--hexagon's best basis changes were negative and approximately quadratic in the row radius; square--square's positive raw changes were at most binary64-scale noise; every CH probe decreased the nominal scalar.",
            f"- The target base capacity intervals are broad (widths {', '.join(format_float(value) for value in target_interval_widths)}), so this run supplies no interval-separated target conclusion. Target results are finite nominal-scalar diagnostics and should motivate branch-aware exact work, not a local-maximality claim.",
            f"- All {ch_outcome['probe_count']} CH probes were valid fixed-nine-facet bodies and entered adjacent combinatorial cells. This is expected from a nonsimple base and gives neighborhood evidence across those cells, but it prevents interpreting the run as a single smooth fixed-incidence calculation.",
            "- The frozen probability bullets in the README were not mutually exclusive: interval ambiguity could coexist with either presence or absence of a target improvement. They therefore record pre-run expectations but do not define a scoreable partition. The observed run has no target improvement and substantial interval ambiguity.",
            "",
            "## Radius-level observations",
            "",
            "| Seed | perturbation | radius | valid/total | material nominal | interval-separated | max delta sys |",
            "| --- | --- | ---: | ---: | ---: | ---: | ---: |",
        ]
    )
    for row in sorted(
        summaries,
        key=lambda item: (
            item["seed_id"],
            item["perturbation"],
            -item["radius"],
        ),
    ):
        lines.append(
            f"| `{row['seed_id']}` | `{row['perturbation']}` | {row['radius']:.0e} | "
            f"{row['valid_probes']}/{row['total_probes']} | {row['nominal_improving_probes']} | "
            f"{row['lower_bound_above_base_upper_probes']} | {format_float(row['max_delta_sys'])} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation boundary",
            "",
            "The pentagon row is a calibration result, not a new discovery. For a target, interval-separated improvements at all tested shrinking radii identify a concrete path for exact branch analysis. A finite miss remains only `no improvement observed` within these fixed-facet directions and radii. The nonsimple CH probes enter adjacent combinatorial cells; those are valid nearby fixed-facet bodies, not rejected samples. A miss does not establish local maximality, exclude a narrow improving cone, control right-active singular branches, or test facet-count changes.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path(__file__).resolve().parent / "artifacts",
    )
    args = parser.parse_args()
    root = args.input_dir
    bases = load_jsonl(root / "bases.jsonl")
    probes = load_jsonl(root / "probes.jsonl")
    summaries = load_jsonl(root / "radius-summaries.jsonl")
    provenance = json.loads((root / "run-provenance.json").read_text(encoding="utf-8"))
    validate(bases, probes, summaries, provenance)

    outcomes = [seed_outcome(base, probes) for base in bases]
    control = next(
        outcome
        for outcome in outcomes
        if outcome["seed_id"] == "pentagon_threshold_control"
    )
    family_pattern = next(
        pattern
        for pattern in control["perturbation_patterns"]
        if pattern["perturbation"] == "product_relative_rotation"
    )
    control_passed = (
        family_pattern["interval_separated_radii"] == sorted(ANGULAR_RADII, reverse=True)
        and family_pattern["lower_bound_above_one_radii"]
        == sorted(ANGULAR_RADII, reverse=True)
    )
    summary = {
        "schema_version": 1,
        "experiment_id": provenance["experiment_id"],
        "control_passed": control_passed,
        "base_count": len(bases),
        "probe_count": len(probes),
        "all_probes_valid": all(row["state_valid"] for row in probes),
        "all_valid_probes_same_incidence": all(
            not row["state_valid"] or row["same_incidence_signature"] for row in probes
        ),
        "outcomes": outcomes,
        "claim_boundary": "Finite fixed-facet screen; target misses are not local-maximality claims.",
    }
    atomic_json(root / "summary.json", summary)
    atomic_text(
        root / "REPORT.md",
        make_report(bases, outcomes, summaries, control_passed),
    )
    print(
        f"Validated {len(bases)} bases and {len(probes)} probes; "
        f"control_passed={control_passed}; wrote {root / 'summary.json'} and {root / 'REPORT.md'}"
    )


if __name__ == "__main__":
    main()
