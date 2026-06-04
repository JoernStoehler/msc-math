#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Packet 1 exact HKO geometry and symmetry-tangent certificate.

Goal: produce the exact HKO dual-vertex geometry record and the exact
      symmetry-tangent-space certificate for the `M_10` first-order route
      using a pure-Python algebraic fallback when Sage is unavailable.
Input Artifacts: experiments/hko-local-maximum/empirical/first-order/hko-neighborhood-sensitivity.jsonl
Output Artifacts: experiments/hko-local-maximum/theorem/exact-witness/hko-geometry.json
                  experiments/hko-local-maximum/theorem/exact-witness/hko-symmetry-tangent.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, sqrt, sympify
from sympy.polys.numberfields import to_number_field


ROOT = Path(__file__).resolve().parents[4]
EXPERIMENT_DIR = Path(__file__).resolve().parent
CURRENT_NUMERICAL_PATH = (
    ROOT
    / "experiments"
    / "hko-local-maximum"
    / "empirical"
    / "first-order"
    / "hko-neighborhood-sensitivity.jsonl"
)
GEOMETRY_OUT = EXPERIMENT_DIR / "hko-geometry.json"
SYMMETRY_OUT = EXPERIMENT_DIR / "hko-symmetry-tangent.json"


T_EXPR = sqrt(5 - 2 * sqrt(5))
T_ALG = to_number_field(T_EXPR)
MINPOLY = T_ALG.minpoly
FIELD_DEGREE = MINPOLY.degree()


def rational_json(value):
    value = sympify(value)
    num, den = value.as_numer_denom()
    return {"num": int(num), "den": int(den)}


def coeff_vector_json(expr):
    alg = to_number_field(expr, T_EXPR)
    coeffs_desc = list(alg.coeffs())
    coeffs_asc = list(reversed(coeffs_desc))
    coeffs_asc += [0] * (FIELD_DEGREE - len(coeffs_asc))
    return [rational_json(coeff) for coeff in coeffs_asc]


def vector_json(vec):
    return [coeff_vector_json(entry) for entry in vec]


def matrix_columns_json(columns):
    return [vector_json(column) for column in columns]


def flatten_columns(columns):
    rows = len(columns[0])
    cols = len(columns)
    data = []
    for row in range(rows):
        for col in range(cols):
            data.append(columns[col][row])
    return data


def standard_j_matrix():
    return Matrix(
        [
            [0, 0, -1, 0],
            [0, 0, 0, -1],
            [1, 0, 0, 0],
            [0, 1, 0, 0],
        ]
    )


def exact_hko_geometry():
    t = T_EXPR
    sqrt5 = (5 - t**2) / 2
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1

    duals = [
        [1, t, 0, 0],
        [-alpha, beta, 0, 0],
        [-sec36, 0, 0, 0],
        [-alpha, -beta, 0, 0],
        [1, -t, 0, 0],
        [0, 0, t, -1],
        [0, 0, beta, alpha],
        [0, 0, 0, sec36],
        [0, 0, -beta, alpha],
        [0, 0, -t, -1],
    ]
    formulas = {
        "generator": "t = tan(pi/5)",
        "minimal_polynomial": "x^4 - 10*x^2 + 5",
        "sqrt5": "(5 - t^2) / 2",
        "alpha": "(3 - sqrt5) / 2",
        "beta": "t * (1 + sqrt5) / 2",
        "sec36": "sqrt5 - 1",
    }
    return duals, formulas


def load_current_numerical_duals():
    row = json.loads(CURRENT_NUMERICAL_PATH.read_text().splitlines()[0])
    return row["dual_vertices"]


def geometry_cross_check_report(duals):
    current = load_current_numerical_duals()
    max_abs_diff = 0.0
    per_facet = []
    for idx, (exact_dual, current_dual) in enumerate(zip(duals, current)):
        exact_float = [float(entry) for entry in exact_dual]
        diffs = [abs(a - b) for a, b in zip(exact_float, current_dual)]
        max_abs_diff = max(max_abs_diff, max(diffs))
        per_facet.append(
            {
                "facet": idx,
                "exact_float": exact_float,
                "current": current_dual,
                "abs_diff": diffs,
            }
        )
    return {
        "path": str(CURRENT_NUMERICAL_PATH.relative_to(ROOT)),
        "max_abs_diff": max_abs_diff,
        "facets": per_facet,
    }


def sp4_generators():
    return [
        (
            "sp_a11",
            Matrix(
                [
                    [1, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, -1, 0],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_a12",
            Matrix(
                [
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, -1, 0],
                ]
            ),
        ),
        (
            "sp_a21",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [1, 0, 0, 0],
                    [0, 0, 0, -1],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_a22",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, -1],
                ]
            ),
        ),
        (
            "sp_b11",
            Matrix(
                [
                    [0, 0, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_b12",
            Matrix(
                [
                    [0, 0, 0, 1],
                    [0, 0, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_b22",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 1],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_c11",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [1, 0, 0, 0],
                    [0, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_c12",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 1, 0, 0],
                    [1, 0, 0, 0],
                ]
            ),
        ),
        (
            "sp_c22",
            Matrix(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 1, 0, 0],
                ]
            ),
        ),
    ]


def translation_tangent_columns(duals):
    columns = []
    for coord in range(4):
        column_entries = []
        for dual in duals:
            scalar = dual[coord]
            column_entries.extend([-(scalar * entry) for entry in dual])
        columns.append(column_entries)
    return columns


def scaling_tangent_column(duals):
    entries = []
    for dual in duals:
        entries.extend([-entry for entry in dual])
    return entries


def linear_symplectic_tangent_columns(duals):
    columns = []
    checks = []
    J = standard_j_matrix()
    for label, X in sp4_generators():
        check = X.T * J + J * X
        checks.append({"label": label, "is_zero": all(value == 0 for value in check)})
        entries = []
        for dual in duals:
            tangent = -(X.T * Matrix(dual))
            entries.extend(list(tangent))
        columns.append((label, entries))
    return columns, checks


def build_symmetry_basis(duals):
    labels = []
    columns = []

    for index, column in enumerate(translation_tangent_columns(duals)):
        labels.append(f"translation_e{index}")
        columns.append(column)

    labels.append("scaling")
    columns.append(scaling_tangent_column(duals))

    linear_columns, checks = linear_symplectic_tangent_columns(duals)
    for label, column in linear_columns:
        labels.append(label)
        columns.append(column)

    tangent_matrix = Matrix.hstack(*[Matrix(column) for column in columns])
    return labels, columns, tangent_matrix, checks


def write_json(path, payload):
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def main():
    duals, formulas = exact_hko_geometry()
    geometry_payload = {
        "field": {
            "generator_name": "t",
            "generator_approx": float(T_EXPR),
            "degree": FIELD_DEGREE,
            "minimal_polynomial": [
                rational_json(coeff) for coeff in list(reversed(MINPOLY.all_coeffs()))
            ],
            "formulas": formulas,
        },
        "facet_order": list(range(len(duals))),
        "dual_vertices_power_basis": matrix_columns_json(duals),
        "dual_vertices_float": [[float(entry) for entry in dual] for dual in duals],
        "cross_check_current_numerical": geometry_cross_check_report(duals),
    }
    write_json(GEOMETRY_OUT, geometry_payload)

    labels, columns, tangent_matrix, checks = build_symmetry_basis(duals)
    symmetry_payload = {
        "field_generator_name": "t",
        "dimension_ambient": tangent_matrix.rows,
        "dimension_generators": tangent_matrix.cols,
        "rank": int(tangent_matrix.rank()),
        "labels": labels,
        "columns_power_basis": matrix_columns_json(columns),
        "generator_sp4_checks": checks,
    }
    write_json(SYMMETRY_OUT, symmetry_payload)

    print(f"Wrote {GEOMETRY_OUT.relative_to(ROOT)}")
    print(f"Wrote {SYMMETRY_OUT.relative_to(ROOT)}")
    print(f"Field degree: {FIELD_DEGREE}")
    print(f"Symmetry tangent rank: {tangent_matrix.rank()}")


if __name__ == "__main__":
    main()
