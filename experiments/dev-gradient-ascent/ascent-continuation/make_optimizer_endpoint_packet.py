# /// script
# requires-python = ">=3.11"
# ///
"""Select retained optimizer endpoints for branch-informed continuation."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def hko_reference(path: Path) -> dict:
    for row in read_jsonl(path):
        if row["ray_id"] == "control_hko_auto" and row["radius"] == 0.0:
            return {
                "reference_id": "hko2024_proved_local_maximum",
                "sys": row["sys_nominal"],
                "dual_flat": [
                    coordinate
                    for vertex in row["serialized_dual_vertices"]
                    for coordinate in vertex
                ],
            }
    raise SystemExit(f"control_hko_auto was not found in {path}")


def distance(left: list[float], right: list[float]) -> float:
    if len(left) != len(right):
        raise SystemExit("endpoint and HKO coordinate counts differ")
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(left, right)))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("dataset", type=Path)
    parser.add_argument("hko_rays", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--algorithm", required=True)
    parser.add_argument("--top", type=int, default=1)
    parser.add_argument("--accepted-step-cap", type=int, default=1)
    parser.add_argument(
        "--radii",
        default="1e-3,3e-4,1e-4,3e-5,1e-5",
        help="comma-separated normalized proposal radii",
    )
    args = parser.parse_args()

    if args.top < 1 or args.accepted_step_cap < 1:
        raise SystemExit("--top and --accepted-step-cap must be positive")
    radii = [float(value) for value in args.radii.split(",")]
    if not radii or any(not math.isfinite(value) or value <= 0.0 for value in radii):
        raise SystemExit("--radii must contain positive finite values")

    evaluations = {
        row["evaluation_id"]: row
        for row in read_jsonl(args.dataset / "evaluations.jsonl")
    }
    runs = [
        row
        for row in read_jsonl(args.dataset / "runs.jsonl")
        if row["algorithm_id"] == args.algorithm
    ]
    if not runs:
        raise SystemExit(f"algorithm {args.algorithm!r} has no completed runs")
    runs.sort(key=lambda row: row["best_sys"], reverse=True)

    reference = hko_reference(args.hko_rays)
    states = []
    for rank, run in enumerate(runs[: args.top], start=1):
        evaluation = evaluations[run["best_evaluation_id"]]
        dual_flat = evaluation["dual_flat"]
        states.append(
            {
                "state_id": f"rank-{rank:03d}--{run['start_id']}",
                "role": "retained_optimizer_endpoint",
                "recorded_sys": run["best_sys"],
                "dual_flat": dual_flat,
                "direction_class": "retained_optimizer_endpoint",
                "source_distance": distance(dual_flat, reference["dual_flat"]),
                "model_radii": radii,
                "accepted_step_cap": args.accepted_step_cap,
            }
        )

    packet = {
        "schema_version": 1,
        "dataset_id": "optimizer-endpoint-continuation",
        "source": str(args.dataset),
        "selection": {
            "algorithm": args.algorithm,
            "order": "descending_best_sys",
            "top": args.top,
            "accepted_step_cap": args.accepted_step_cap,
            "model_radii": radii,
        },
        "reference": reference,
        "states": states,
        "expected_evidence": {
            "accepted_move_crosses_one": (
                "new numerical sys>1 candidate; preserve and run the separate "
                "numerical-validation route"
            ),
            "accepted_move_below_one": (
                "the continuation adds useful late-stage gain; compare gain "
                "per compute before a longer run"
            ),
            "no_accepted_move": (
                "the tested branch model stopped; this does not establish "
                "local maximality"
            ),
        },
        "claim_boundary": (
            "Selection is outcome-based from a retained tuning dataset. The "
            "run tests late-stage continuation of named endpoints, not held-out "
            "optimizer performance or endpoint local maximality."
        ),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(packet, indent=2) + "\n")


if __name__ == "__main__":
    main()
