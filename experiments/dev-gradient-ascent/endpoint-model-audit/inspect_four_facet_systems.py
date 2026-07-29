# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy"]
# ///
"""Inspect the f64 four-facet systems behind a retained endpoint.

This reproduces the determinant threshold and the bounded-candidate tests used
by experiments/dev-quadratic-program/src/geometry.rs closely enough to explain
which counter contributed to an optimizer trace. It is a numerical diagnostic,
not an independent implementation of the production geometry route.
"""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
from pathlib import Path

import numpy as np


EPS_DET = 1.0e-12
EPS_INEQUALITY = 1.0e-9
EPS_SINGULAR_RESIDUAL = 1.0e-8
MAX_BOUNDED_VERTEX_COORD = 1.0e3


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--state-index", type=int, default=0)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    raw = args.input.read_bytes()
    packet = json.loads(raw)
    state = packet["states"][args.state_index]
    duals = np.asarray(state["dual_flat"], dtype=np.float64).reshape((-1, 4))

    systems: list[dict[str, object]] = []
    for facets in itertools.combinations(range(len(duals)), 4):
        matrix = duals[list(facets)]
        determinant = float(np.linalg.det(matrix))
        if abs(determinant) > EPS_DET:
            continue

        # NumPy and nalgebra use different SVD implementations. The thresholds
        # and the final source-level classification are recorded separately.
        solution, _, rank, singular_values = np.linalg.lstsq(
            matrix, np.ones(4), rcond=EPS_DET
        )
        residual = float(np.linalg.norm(matrix @ solution - np.ones(4)))
        gaps = duals @ solution - 1.0
        norm_inf = float(np.max(np.abs(solution)))
        bounded_candidate = (
            residual <= EPS_SINGULAR_RESIDUAL
            and norm_inf <= MAX_BOUNDED_VERTEX_COORD
            and bool(np.all(gaps <= EPS_INEQUALITY))
        )
        positive = singular_values[singular_values > 0.0]
        condition_number = (
            float(positive.max() / positive.min()) if len(positive) else None
        )
        systems.append(
            {
                "facets": list(facets),
                "determinant": determinant,
                "abs_determinant": abs(determinant),
                "svd_rank_at_rcond": int(rank),
                "singular_values": [float(value) for value in singular_values],
                "condition_number": condition_number,
                "least_squares_solution": [float(value) for value in solution],
                "solution_norm_inf": norm_inf,
                "residual_l2": residual,
                "maximum_polytope_inequality_gap": float(gaps.max()),
                "maximum_gap_facet": int(np.argmax(gaps)),
                "passes_approximate_bounded_candidate_test": bounded_candidate,
            }
        )

    systems.sort(key=lambda row: row["abs_determinant"])
    result = {
        "schema_version": 1,
        "input": str(args.input),
        "input_sha256": hashlib.sha256(raw).hexdigest(),
        "state_index": args.state_index,
        "state_id": state["state_id"],
        "facet_count": len(duals),
        "four_facet_system_count": len(list(itertools.combinations(range(len(duals)), 4))),
        "thresholds_reproduced": {
            "abs_determinant_at_most": EPS_DET,
            "bounded_candidate_residual_at_most": EPS_SINGULAR_RESIDUAL,
            "bounded_candidate_coordinate_abs_at_most": MAX_BOUNDED_VERTEX_COORD,
            "polytope_inequality_gap_at_most": EPS_INEQUALITY,
        },
        "near_singular_system_count": len(systems),
        "approximately_bounded_candidate_count": sum(
            bool(row["passes_approximate_bounded_candidate_test"]) for row in systems
        ),
        "near_singular_systems": systems,
        "claim_boundary": (
            "This f64 NumPy audit explains the retained counter. "
            "The Rust geometry producer remains authoritative."
        ),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
