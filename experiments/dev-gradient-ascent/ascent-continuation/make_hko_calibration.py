# /// script
# requires-python = ">=3.11"
# ///
"""Freeze HKO quotient-ray perturbations as continuation-diagnostic inputs."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path


RAYS = [
    "sentinel_slice_basis_column_0",
    "sentinel_projected_rotated_pentagon_tangent",
    "random_000",
    "random_001",
]
RADII = [1.0e-4, 1.0e-3, 1.0e-2, 1.0e-1]


def flatten(vertices: list[list[float]]) -> list[float]:
    return [coordinate for vertex in vertices for coordinate in vertex]


def norm(flat: list[float]) -> float:
    return math.sqrt(sum(value * value for value in flat))


def read_selected(source: Path) -> tuple[dict, dict[tuple[str, float], dict]]:
    hko = None
    selected = {}
    wanted = {(ray, radius) for ray in RAYS for radius in RADII}
    with source.open() as stream:
        for line in stream:
            row = json.loads(line)
            if row["ray_id"] == "control_hko_auto" and row["radius"] == 0.0:
                hko = row
            key = (row["ray_id"], row["radius"])
            if key in wanted and row["phase"] == "shell":
                selected[key] = row
    if hko is None:
        raise SystemExit("control_hko_auto was not found")
    missing = wanted - set(selected)
    if missing:
        raise SystemExit(f"missing selected rows: {sorted(missing)}")
    return hko, selected


def validate(row: dict) -> None:
    expected = {
        "chart_label": "chart_nominal",
        "gauge": "gauge_nominal",
        "evaluator_label": "evaluator_available",
    }
    actual = {
        "chart_label": row["chart_label"],
        "gauge": row["gauge"]["label"],
        "evaluator_label": row["evaluator_label"],
    }
    if actual != expected:
        raise SystemExit(f"{row['ray_id']} at {row['radius']}: {actual}")


def state_from_row(row: dict, accepted_step_cap: int) -> dict:
    validate(row)
    dual_flat = flatten(row["serialized_dual_vertices"])
    distance = float(row["radius"])
    normalized_return_distance = distance / norm(dual_flat)
    return {
        "state_id": f"{row['ray_id']}--r{distance:.0e}",
        "role": "known_perturbation_of_proved_hko_local_maximum",
        "recorded_sys": row["sys_nominal"],
        "dual_flat": dual_flat,
        "direction_class": row["ray_id"],
        "source_distance": distance,
        "model_radii": [
            normalized_return_distance,
            0.3 * normalized_return_distance,
            0.1 * normalized_return_distance,
        ],
        "accepted_step_cap": accepted_step_cap,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--profile", choices=["debug", "panel"], required=True)
    args = parser.parse_args()

    hko, selected = read_selected(args.source)
    hko_flat = flatten(hko["serialized_dual_vertices"])
    control = {
        "state_id": "hko-control",
        "role": "proved_local_maximum_false_positive_control",
        "recorded_sys": hko["sys_nominal"],
        "dual_flat": hko_flat,
        "direction_class": "hko_control",
        "source_distance": 0.0,
        "model_radii": [1.0e-3, 1.0e-4, 1.0e-5],
        "accepted_step_cap": 1,
    }
    if args.profile == "debug":
        states = [control, state_from_row(selected[("random_000", 1.0e-2)], 1)]
    else:
        states = [control]
        states.extend(
            state_from_row(selected[(ray, radius)], 1)
            for ray in RAYS
            for radius in RADII
        )
    packet = {
        "schema_version": 1,
        "dataset_id": f"hko-continuation-calibration-{args.profile}",
        "source": str(args.source),
        "selection": {"rays": RAYS, "radii": RADII, "profile": args.profile},
        "reference": {
            "reference_id": "hko2024_proved_local_maximum",
            "sys": hko["sys_nominal"],
            "dual_flat": hko_flat,
        },
        "states": states,
        "expected_evidence": {
            "hko_material_move": "false-positive failure; stop interpretation",
            "high_gap_and_distance_recovery": "supports this diagnostic on the selected HKO directions and distances",
            "close_recovery_but_far_failure": "supports only a local continuation diagnostic",
            "far_recovery_but_close_failure": "indicates a numerical or proposal-resolution floor",
            "strong_direction_dependence": "miss probability depends on local direction and is not globally calibrated",
            "large_known_gap_but_no_move": "directly observed false negative for the tested proposal family",
        },
        "claim_boundary": (
            "The four directions are a development panel chosen to span two "
            "sentinels and two random rays. They do not estimate a population "
            "miss rate; the other retained random rays remain unexposed."
        ),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(packet, indent=2) + "\n")


if __name__ == "__main__":
    main()
