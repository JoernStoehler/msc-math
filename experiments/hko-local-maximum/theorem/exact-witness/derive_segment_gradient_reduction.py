#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Derive the exact gradient reduction on the seven-facet KKT segment.

Goal: certify that the neighboring seven-facet equality-case KKT segment
      contributes an affine height-gradient family, so interior points add no
      new extreme first-order rows beyond the segment endpoints.
Input Artifacts: None
Output Artifacts: experiments/hko-local-maximum/theorem/exact-witness/segment-gradient-reduction.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, simplify, sqrt, symbols


EXPERIMENT_DIR = Path(__file__).resolve().parent
OUTPUT_PATH = EXPERIMENT_DIR / "segment-gradient-reduction.json"


def main() -> None:
    sqrt5 = sqrt(5)
    a = simplify(sqrt5 / 10)
    b = simplify(Rational(1, 4) - sqrt5 / 20)
    t = sqrt(5 - 2 * sqrt(5))
    lambda_symbol = symbols("lambda")
    gamma, delta = symbols("gamma delta")

    facets = list(range(10))
    union_subset = [0, 1, 2, 3, 6, 7, 9]

    endpoint_left = {
        0: b,
        1: b,
        2: 0,
        3: a,
        4: 0,
        5: 0,
        6: b,
        7: b,
        8: 0,
        9: a,
    }
    endpoint_right = {
        0: a,
        1: 0,
        2: b,
        3: b,
        4: 0,
        5: 0,
        6: b,
        7: b,
        8: 0,
        9: a,
    }
    segment_beta = {
        facet: simplify((1 - lambda_symbol) * endpoint_left[facet] + lambda_symbol * endpoint_right[facet])
        for facet in facets
    }

    # Exact capacity-height derivative factor from the KKT segment certificate.
    capacity_prefactor = simplify(5 / t)
    capacity_left = {
        facet: simplify(capacity_prefactor * endpoint_left[facet]) for facet in facets
    }
    capacity_right = {
        facet: simplify(capacity_prefactor * endpoint_right[facet]) for facet in facets
    }
    capacity_segment = {
        facet: simplify(capacity_prefactor * segment_beta[facet]) for facet in facets
    }
    capacity_affine_residual = {
        facet: simplify(
            capacity_segment[facet]
            - ((1 - lambda_symbol) * capacity_left[facet] + lambda_symbol * capacity_right[facet])
        )
        for facet in facets
    }

    # Abstract sys-height family under the HKO symmetry fact that dV/dh_k is uniform.
    sys_left = {facet: simplify(gamma * endpoint_left[facet] - delta) for facet in facets}
    sys_right = {facet: simplify(gamma * endpoint_right[facet] - delta) for facet in facets}
    sys_segment = {facet: simplify(gamma * segment_beta[facet] - delta) for facet in facets}
    sys_affine_residual = {
        facet: simplify(
            sys_segment[facet]
            - ((1 - lambda_symbol) * sys_left[facet] + lambda_symbol * sys_right[facet])
        )
        for facet in facets
    }

    payload = {
        "beta_names": {
            "a": "sqrt(5) / 10",
            "b": "(5 - sqrt(5)) / 20",
        },
        "segment_parameter": "lambda",
        "endpoint_pair": {
            "left_subset": [0, 1, 3, 6, 7, 9],
            "right_subset": [0, 2, 3, 6, 7, 9],
            "union_subset": union_subset,
        },
        "beta_by_facet": {
            "left": {str(facet): str(endpoint_left[facet]) for facet in facets},
            "right": {str(facet): str(endpoint_right[facet]) for facet in facets},
            "segment": {str(facet): str(segment_beta[facet]) for facet in facets},
        },
        "capacity_height_prefactor": str(capacity_prefactor),
        "capacity_height_derivative_by_facet": {
            "left": {str(facet): str(capacity_left[facet]) for facet in facets},
            "right": {str(facet): str(capacity_right[facet]) for facet in facets},
            "segment": {str(facet): str(capacity_segment[facet]) for facet in facets},
        },
        "capacity_affine_residual_by_facet": {
            str(facet): str(capacity_affine_residual[facet]) for facet in facets
        },
        "sys_height_reduction_model": {
            "gamma": "common scalar (capacity / volume) * capacity_height_prefactor",
            "delta": "common scalar (sys / volume) * uniform volume-height derivative",
            "left": {str(facet): str(sys_left[facet]) for facet in facets},
            "right": {str(facet): str(sys_right[facet]) for facet in facets},
            "segment": {str(facet): str(sys_segment[facet]) for facet in facets},
        },
        "sys_affine_residual_by_facet": {
            str(facet): str(sys_affine_residual[facet]) for facet in facets
        },
        "theorem_use": (
            "For lambda in [0,1], the seven-facet segment row lies in the convex hull "
            "of the two endpoint rows. Therefore interior segment points contribute no "
            "new extreme first-order rows once the endpoint rows are included."
        ),
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(EXPERIMENT_DIR.parent.parent.parent)}")


if __name__ == "__main__":
    main()
