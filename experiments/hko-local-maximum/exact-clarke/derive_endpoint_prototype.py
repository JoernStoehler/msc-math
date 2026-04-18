#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Derive exact endpoint and segment-family prototypes for the HKO2024 minimizers.

Goal: certify one symmetry-reduced six-facet endpoint prototype and one
      midpoint-style seven-facet equality-case prototype exactly from the
      quartic HKO dual geometry using closure + normalization, and record the
      exact one-parameter equality-case segment between neighboring endpoints.
Input Artifacts: None
Output Artifacts: experiments/hko-local-maximum/exact-clarke/endpoint-prototype-certificate.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, linsolve, simplify, sqrt, symbols


EXPERIMENT_DIR = Path(__file__).resolve().parent
OUTPUT_PATH = EXPERIMENT_DIR / "endpoint-prototype-certificate.json"


def main() -> None:
    t = sqrt(5 - 2 * sqrt(5))
    sqrt5 = (5 - t**2) / 2
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1

    dual_vertices = [
        Matrix([1, t, 0, 0]),
        Matrix([-alpha, beta, 0, 0]),
        Matrix([-sec36, 0, 0, 0]),
        Matrix([-alpha, -beta, 0, 0]),
        Matrix([1, -t, 0, 0]),
        Matrix([0, 0, t, -1]),
        Matrix([0, 0, beta, alpha]),
        Matrix([0, 0, 0, sec36]),
        Matrix([0, 0, -beta, alpha]),
        Matrix([0, 0, -t, -1]),
    ]

    j0 = Matrix([[0, 0, -1, 0], [0, 0, 0, -1], [1, 0, 0, 0], [0, 1, 0, 0]])

    def omega(lhs: Matrix, rhs: Matrix):
        return simplify((lhs.T * j0 * rhs)[0])

    a_value = simplify(sqrt5 / 10)
    b_value = simplify(Rational(1, 4) - sqrt5 / 20)
    c_value = simplify(b_value / 2)
    d_value = simplify((a_value + b_value) / 2)
    lambda_symbol = symbols("lambda")

    def certify(permutation: list[int], beta_profile: list) -> dict[str, object]:
        closure = Matrix([0, 0, 0, 0])
        for weight, facet in zip(beta_profile, permutation, strict=True):
            closure += weight * dual_vertices[facet]
        closure = [simplify(entry) for entry in closure]

        normalization = simplify(sum(beta_profile))

        h_matrix = Matrix.zeros(len(permutation), len(permutation))
        for i in range(len(permutation)):
            for j in range(i + 1, len(permutation)):
                value = omega(dual_vertices[permutation[i]], dual_vertices[permutation[j]])
                h_matrix[i, j] = value
                h_matrix[j, i] = value
        beta_column = Matrix(beta_profile)
        q_value = simplify(Rational(1, 2) * (beta_column.T * h_matrix * beta_column)[0])
        action = simplify(abs(Rational(1, 2) / q_value))

        return {
            "subset": sorted(permutation),
            "positive_permutation": permutation,
            "beta_profile_formulas": [str(entry) for entry in beta_profile],
            "closure_check": [str(entry) for entry in closure],
            "normalization_check": str(normalization),
            "q_value_signed": str(q_value),
            "action": str(action),
            "action_float": float(action.evalf(30)),
        }

    endpoint = certify(
        permutation=[9, 3, 6, 7, 1, 0],
        beta_profile=[a_value, a_value, b_value, b_value, b_value, b_value],
    )

    midpoint = certify(
        permutation=[9, 2, 3, 6, 7, 1, 0],
        beta_profile=[a_value, c_value, d_value, b_value, b_value, c_value, d_value],
    )

    union_facets = [0, 1, 2, 3, 6, 7, 9]
    endpoint_left_union = {
        0: b_value,
        1: b_value,
        2: 0,
        3: a_value,
        6: b_value,
        7: b_value,
        9: a_value,
    }
    endpoint_right_union = {
        0: a_value,
        1: 0,
        2: b_value,
        3: b_value,
        6: b_value,
        7: b_value,
        9: a_value,
    }
    segment_profile_by_facet = {
        facet: simplify(
            (1 - lambda_symbol) * endpoint_left_union[facet]
            + lambda_symbol * endpoint_right_union[facet]
        )
        for facet in union_facets
    }
    segment_closure = Matrix([0, 0, 0, 0])
    for facet in union_facets:
        segment_closure += segment_profile_by_facet[facet] * dual_vertices[facet]
    segment_closure = [simplify(entry) for entry in segment_closure]
    segment_normalization = simplify(sum(segment_profile_by_facet.values()))
    segment_permutation = [9, 2, 3, 6, 7, 1, 0]
    segment_beta_column = Matrix([segment_profile_by_facet[facet] for facet in segment_permutation])
    segment_h_matrix = Matrix.zeros(len(segment_permutation), len(segment_permutation))
    for i in range(len(segment_permutation)):
        for j in range(i + 1, len(segment_permutation)):
            value = omega(dual_vertices[segment_permutation[i]], dual_vertices[segment_permutation[j]])
            segment_h_matrix[i, j] = value
            segment_h_matrix[j, i] = value
    segment_q = simplify(
        Rational(1, 2) * (segment_beta_column.T * segment_h_matrix * segment_beta_column)[0]
    )
    mu0, mu1, mu2, mu3, xi = symbols("mu0 mu1 mu2 mu3 xi")
    stationarity_matrix = Matrix(
        [[dual_vertices[facet][coord] for coord in range(4)] + [1] for facet in segment_permutation]
    )
    stationarity_rhs = -segment_h_matrix * segment_beta_column
    mu_xi_solution = next(iter(linsolve((stationarity_matrix, stationarity_rhs), (mu0, mu1, mu2, mu3, xi))))
    mu_xi_solution = [simplify(entry) for entry in mu_xi_solution]
    stationarity_residual = (
        stationarity_matrix * Matrix(mu_xi_solution) - stationarity_rhs
    )
    stationarity_residual = [simplify(entry) for entry in stationarity_residual]
    xi_solution = mu_xi_solution[4]
    capacity_height_prefactor = simplify(-xi_solution / (2 * segment_q**2))

    payload = {
        "field_generator": "t = sqrt(5 - 2*sqrt(5)) = tan(pi/5)",
        "beta_names": {
            "a": "sqrt(5) / 10",
            "b": "(5 - sqrt(5)) / 20",
            "c": "(5 - sqrt(5)) / 40",
            "d": "(5 + sqrt(5)) / 40",
        },
        "endpoint_prototype": endpoint,
        "midpoint_prototype": {
            **midpoint,
            "construction": "facetwise average of neighboring endpoint beta profiles on the union subset",
        },
        "segment_family": {
            "union_subset": union_facets,
            "positive_permutation": segment_permutation,
            "parameter": "lambda",
            "beta_profile_by_facet": {
                str(facet): str(segment_profile_by_facet[facet]) for facet in union_facets
            },
            "closure_check": [str(entry) for entry in segment_closure],
            "normalization_check": str(segment_normalization),
            "q_value_signed": str(segment_q),
            "dq_dlambda": str(simplify(segment_q.diff(lambda_symbol))),
            "action": str(simplify(Rational(1, 2) / segment_q)),
            "mu_xi_solution": [str(entry) for entry in mu_xi_solution],
            "stationarity_residual": [str(entry) for entry in stationarity_residual],
            "capacity_height_prefactor": str(capacity_height_prefactor),
            "capacity_height_derivative_by_facet": {
                str(facet): str(simplify(capacity_height_prefactor * segment_profile_by_facet[facet]))
                for facet in union_facets
            },
        },
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(EXPERIMENT_DIR.parent.parent.parent)}")


if __name__ == "__main__":
    main()
