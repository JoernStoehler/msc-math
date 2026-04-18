#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Derive one exact endpoint prototype for the HKO2024 minimizing family.

Goal: certify one symmetry-reduced six-facet endpoint prototype exactly from
      the quartic HKO dual geometry using closure + normalization.
Input Artifacts: None
Output Artifacts: experiments/hko-local-maximum/exact-clarke/endpoint-prototype-certificate.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, simplify, sqrt


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

    # Use the positively oriented endpoint prototype.
    permutation = [9, 3, 6, 7, 1, 0]
    a_value = simplify(sqrt5 / 10)
    b_value = simplify(Rational(1, 4) - sqrt5 / 20)
    beta_profile = [a_value, a_value, b_value, b_value, b_value, b_value]

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
    action = simplify(Rational(1, 2) / q_value)

    payload = {
        "field_generator": "t = sqrt(5 - 2*sqrt(5)) = tan(pi/5)",
        "endpoint_prototype": {
            "subset": sorted(permutation),
            "positive_permutation": permutation,
            "beta_profile_formulas": [
                str(entry) for entry in beta_profile
            ],
            "beta_names": {
                "a": "sqrt(5) / 10",
                "b": "(5 - sqrt(5)) / 20",
            },
            "closure_check": [str(entry) for entry in closure],
            "normalization_check": str(normalization),
            "q_value": str(q_value),
            "action": str(action),
            "action_float": float(action.evalf(30)),
        }
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(EXPERIMENT_DIR.parent.parent.parent)}")


if __name__ == "__main__":
    main()
