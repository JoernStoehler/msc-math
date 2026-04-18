#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Derive the exact dual-vertex row reduction on the seven-facet KKT segment.

Goal: certify the theorem-facing Packet 2 reduction in `R^40`: for the
      neighboring seven-facet equality-case KKT segment, the exact capacity row
      family is quadratic in `lambda` and is therefore spanned by three exact
      prototype rows (left endpoint, midpoint, right endpoint). Because all
      active minimizers have the same capacity and share the same volume
      derivative row, the same three-row reduction holds for the exact `sys`
      rows used by the Clarke checker.
Input Artifacts: None
Output Artifacts: experiments/hko-local-maximum/exact-clarke/segment-a-gradient-reduction.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, expand, linsolve, simplify, sqrt, sympify, symbols


EXPERIMENT_DIR = Path(__file__).resolve().parent
OUTPUT_PATH = EXPERIMENT_DIR / "segment-a-gradient-reduction.json"


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
    lambda_symbol = symbols("lambda")
    midpoint_lambda = Rational(1, 2)
    mu0, mu1, mu2, mu3, xi = symbols("mu0 mu1 mu2 mu3 xi")

    segment_permutation = [9, 2, 3, 6, 7, 1, 0]
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

    def beta_profile(parameter) -> list:
        return [
            simplify(
                (1 - parameter) * endpoint_left_union[facet]
                + parameter * endpoint_right_union[facet]
            )
            for facet in segment_permutation
        ]

    h_matrix = Matrix.zeros(len(segment_permutation), len(segment_permutation))
    for i in range(len(segment_permutation)):
        for j in range(i + 1, len(segment_permutation)):
            value = omega(dual_vertices[segment_permutation[i]], dual_vertices[segment_permutation[j]])
            h_matrix[i, j] = value
            h_matrix[j, i] = value

    stationarity_matrix = Matrix(
        [[dual_vertices[facet][coord] for coord in range(4)] + [1] for facet in segment_permutation]
    )

    def solve_mu_xi(profile: list) -> list:
        stationarity_rhs = -h_matrix * Matrix(profile)
        solution = next(
            iter(linsolve((stationarity_matrix, stationarity_rhs), (mu0, mu1, mu2, mu3, xi)))
        )
        return [simplify(entry) for entry in solution]

    def q_value(profile: list):
        profile_column = Matrix(profile)
        return simplify(Rational(1, 2) * (profile_column.T * h_matrix * profile_column)[0])

    def capacity_gradient_row(permutation: list[int], profile: list, mu_xi_solution: list, q) -> list:
        q_sq = simplify(q**2)
        result = []
        for facet in range(10):
            if facet not in permutation:
                result.extend([0, 0, 0, 0])
                continue

            orbit_index = permutation.index(facet)
            partial_sum = Matrix([0, 0, 0, 0])
            for index in range(orbit_index):
                partial_sum += simplify(profile[index] * dual_vertices[permutation[index]])

            inner = simplify(2 * partial_sum + profile[orbit_index] * dual_vertices[facet])
            # Envelope theorem: dq/da_k includes the ordered symplectic-pairing term
            # and the closure-constraint multiplier contribution.
            dq_da = simplify(profile[orbit_index] * (-j0 * inner + Matrix(mu_xi_solution[:4])))
            result.extend([simplify(-entry / (2 * q_sq)) for entry in dq_da])
        return result

    def row_data(parameter) -> dict[str, object]:
        profile = beta_profile(parameter)
        mu_xi_solution = solve_mu_xi(profile)
        q = q_value(profile)
        return {
            "parameter": str(parameter),
            "beta_profile_by_facet": {
                str(facet): str(entry)
                for facet, entry in zip(segment_permutation, profile, strict=True)
            },
            "q_value_signed": str(q),
            "mu_xi_solution": [str(entry) for entry in mu_xi_solution],
            "capacity_row_flat": [
                str(entry)
                for entry in capacity_gradient_row(segment_permutation, profile, mu_xi_solution, q)
            ],
        }

    def endpoint_row_data(permutation: list[int], profile: list) -> dict[str, object]:
        endpoint_h = Matrix.zeros(len(permutation), len(permutation))
        for i in range(len(permutation)):
            for j in range(i + 1, len(permutation)):
                value = omega(dual_vertices[permutation[i]], dual_vertices[permutation[j]])
                endpoint_h[i, j] = value
                endpoint_h[j, i] = value
        endpoint_stationarity_matrix = Matrix(
            [[dual_vertices[facet][coord] for coord in range(4)] + [1] for facet in permutation]
        )
        stationarity_rhs = -endpoint_h * Matrix(profile)
        mu_xi_solution = next(
            iter(linsolve((endpoint_stationarity_matrix, stationarity_rhs), (mu0, mu1, mu2, mu3, xi)))
        )
        mu_xi_solution = [simplify(entry) for entry in mu_xi_solution]
        q = simplify(Rational(1, 2) * (Matrix(profile).T * endpoint_h * Matrix(profile))[0])
        return {
            "permutation": permutation,
            "q_value_signed": str(q),
            "mu_xi_solution": [str(entry) for entry in mu_xi_solution],
            "capacity_row_flat": [
                str(entry) for entry in capacity_gradient_row(permutation, profile, mu_xi_solution, q)
            ],
        }

    left_row = row_data(Rational(0))
    midpoint_row = row_data(midpoint_lambda)
    right_row = row_data(Rational(1))
    left_endpoint_row = endpoint_row_data([9, 3, 6, 7, 1, 0], [a_value, a_value, b_value, b_value, b_value, b_value])
    right_endpoint_row = endpoint_row_data([9, 2, 3, 6, 7, 0], [a_value, b_value, b_value, b_value, b_value, a_value])

    symbolic_profile = beta_profile(lambda_symbol)
    symbolic_mu_xi = solve_mu_xi(symbolic_profile)
    symbolic_q = q_value(symbolic_profile)
    symbolic_capacity_row = capacity_gradient_row(
        segment_permutation, symbolic_profile, symbolic_mu_xi, symbolic_q
    )

    max_degree = 0
    for entry in symbolic_capacity_row:
        polynomial = expand(entry)
        if polynomial == 0:
            continue
        max_degree = max(max_degree, polynomial.as_poly(lambda_symbol).degree())

    lagrange_left = simplify(2 * (lambda_symbol - midpoint_lambda) * (lambda_symbol - 1))
    lagrange_mid = simplify(-4 * lambda_symbol * (lambda_symbol - 1))
    lagrange_right = simplify(2 * lambda_symbol * (lambda_symbol - midpoint_lambda))
    coefficient_sum = simplify(lagrange_left + lagrange_mid + lagrange_right)

    interpolation_residual = [
        simplify(
            symbolic_entry
            - (
                lagrange_left * simplify(left_entry)
                + lagrange_mid * simplify(midpoint_entry)
                + lagrange_right * simplify(right_entry)
            )
        )
        for symbolic_entry, left_entry, midpoint_entry, right_entry in zip(
            symbolic_capacity_row,
            left_row["capacity_row_flat"],
            midpoint_row["capacity_row_flat"],
            right_row["capacity_row_flat"],
            strict=True,
        )
    ]
    endpoint_left_residual = [
        simplify(sympify(segment_entry) - sympify(endpoint_entry))
        for segment_entry, endpoint_entry in zip(
            left_row["capacity_row_flat"], left_endpoint_row["capacity_row_flat"], strict=True
        )
    ]
    endpoint_right_residual = [
        simplify(sympify(segment_entry) - sympify(endpoint_entry))
        for segment_entry, endpoint_entry in zip(
            right_row["capacity_row_flat"], right_endpoint_row["capacity_row_flat"], strict=True
        )
    ]

    coordinate_labels = [
        f"facet_{facet}_coord_{coord}"
        for facet in range(10)
        for coord in range(4)
    ]

    payload = {
        "field_generator": "t = sqrt(5 - 2*sqrt(5)) = tan(pi/5)",
        "segment_parameter": "lambda",
        "midpoint_lambda": str(midpoint_lambda),
        "segment_permutation": segment_permutation,
        "coordinate_order": coordinate_labels,
        "lagrange_coefficients": {
            "left": str(lagrange_left),
            "midpoint": str(lagrange_mid),
            "right": str(lagrange_right),
            "sum_check": str(coefficient_sum),
        },
        "capacity_row_polynomial_degree_max": max_degree,
        "prototype_rows": {
            "left_endpoint": left_row,
            "midpoint": midpoint_row,
            "right_endpoint": right_row,
            "endpoint_family_left": left_endpoint_row,
            "endpoint_family_right": right_endpoint_row,
            "symbolic_segment_q_value": str(symbolic_q),
            "symbolic_capacity_row_flat": [str(entry) for entry in symbolic_capacity_row],
        },
        "capacity_row_interpolation_residual_by_coordinate": {
            label: str(entry) for label, entry in zip(coordinate_labels, interpolation_residual, strict=True)
        },
        "endpoint_coincidence_residual_by_coordinate": {
            "left": {
                label: str(entry)
                for label, entry in zip(coordinate_labels, endpoint_left_residual, strict=True)
            },
            "right": {
                label: str(entry)
                for label, entry in zip(coordinate_labels, endpoint_right_residual, strict=True)
            },
        },
        "theorem_use": (
            "The exact capacity row on the seven-facet KKT segment is a degree-2 polynomial "
            "in lambda and equals the Lagrange interpolation through lambda = 0, 1/2, 1. "
            "Since all minimizing rows share the same exact capacity and the same volume "
            "derivative row, the exact sys row satisfies the same three-row interpolation. "
            "The segment endpoints coincide exactly with the corresponding six-facet endpoint "
            "rows, so this neighboring seven-facet family contributes no new rows outside the "
            "span of the existing endpoint rows together with the midpoint prototype row."
        ),
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(EXPERIMENT_DIR.parent.parent.parent)}")


if __name__ == "__main__":
    main()
