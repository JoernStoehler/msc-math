#!/usr/bin/env sage
"""
Exact HKO2024 geometry and symmetry-tangent certificate.

Goal: produce the exact HKO dual-vertex geometry record and the exact
symmetry-tangent-space certificate for the `M_10` first-order route.
Input Artifacts: experiments/hko-local-maximum/empirical/first-order/hko-neighborhood-sensitivity.jsonl
Output Artifacts: experiments/hko-local-maximum/history/exact-witness/hko-geometry.json
                  experiments/hko-local-maximum/history/exact-witness/hko-symmetry-tangent.json
"""

import json
from pathlib import Path


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


def q_json(q):
    q = QQ(q)
    return {"num": int(q.numerator()), "den": int(q.denominator())}


def coeff_vector_json(x, degree):
    coeffs = list(x.list())
    coeffs += [QQ(0)] * (degree - len(coeffs))
    return [q_json(c) for c in coeffs]


def vector_json(v, degree):
    return [coeff_vector_json(entry, degree) for entry in v]


def matrix_columns_json(columns, degree):
    return [vector_json(column, degree) for column in columns]


def real_generator_approx():
    return sqrt(RR(5) - RR(2) * sqrt(RR(5)))


def real_float(x):
    t_real = real_generator_approx()
    return float(sum(RR(coeff) * t_real**idx for idx, coeff in enumerate(x.list())))


def flatten_columns(columns):
    rows = len(columns[0])
    cols = len(columns)
    data = []
    for row in range(rows):
        for col in range(cols):
            data.append(columns[col][row])
    return data


def standard_j_matrix(K):
    return matrix(
        K,
        [
            [0, 0, -1, 0],
            [0, 0, 0, -1],
            [1, 0, 0, 0],
            [0, 1, 0, 0],
        ],
    )


def exact_hko_geometry():
    R.<x> = QQ[]
    K.<t> = NumberField(x^4 - 10 * x^2 + 5)
    degree = K.degree()
    sqrt5 = (QQ(5) - t^2) / 2
    alpha = (QQ(3) - sqrt5) / 2
    beta = t * (QQ(1) + sqrt5) / 2
    sec36 = sqrt5 - 1

    duals = [
        vector(K, [1, t, 0, 0]),
        vector(K, [-alpha, beta, 0, 0]),
        vector(K, [-sec36, 0, 0, 0]),
        vector(K, [-alpha, -beta, 0, 0]),
        vector(K, [1, -t, 0, 0]),
        vector(K, [0, 0, t, -1]),
        vector(K, [0, 0, beta, alpha]),
        vector(K, [0, 0, 0, sec36]),
        vector(K, [0, 0, -beta, alpha]),
        vector(K, [0, 0, -t, -1]),
    ]

    formulas = {
        "generator": "t = tan(pi/5)",
        "minimal_polynomial": "x^4 - 10*x^2 + 5",
        "sqrt5": "(5 - t^2) / 2",
        "alpha": "(3 - sqrt5) / 2",
        "beta": "t * (1 + sqrt5) / 2",
        "sec36": "sqrt5 - 1",
    }
    return K, degree, formulas, duals


def load_current_numerical_duals():
    row = json.loads(CURRENT_NUMERICAL_PATH.read_text().splitlines()[0])
    return row["dual_vertices"]


def geometry_cross_check_report(duals):
    current = load_current_numerical_duals()
    max_abs_diff = 0.0
    per_facet = []
    for idx, (exact_dual, current_dual) in enumerate(zip(duals, current)):
        exact_float = [real_float(entry) for entry in exact_dual]
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


def sp4_generators(K):
    zero = K.zero()
    one = K.one()

    def mat(rows):
        return matrix(K, rows)

    generators = [
        (
            "sp_a11",
            mat(
                [
                    [one, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, -one, zero],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_a12",
            mat(
                [
                    [zero, one, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, -one, zero],
                ]
            ),
        ),
        (
            "sp_a21",
            mat(
                [
                    [zero, zero, zero, zero],
                    [one, zero, zero, zero],
                    [zero, zero, zero, -one],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_a22",
            mat(
                [
                    [zero, zero, zero, zero],
                    [zero, one, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, -one],
                ]
            ),
        ),
        (
            "sp_b11",
            mat(
                [
                    [zero, zero, one, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_b12",
            mat(
                [
                    [zero, zero, zero, one],
                    [zero, zero, one, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_b22",
            mat(
                [
                    [zero, zero, zero, zero],
                    [zero, zero, zero, one],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_c11",
            mat(
                [
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [one, zero, zero, zero],
                    [zero, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_c12",
            mat(
                [
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, one, zero, zero],
                    [one, zero, zero, zero],
                ]
            ),
        ),
        (
            "sp_c22",
            mat(
                [
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, zero, zero, zero],
                    [zero, one, zero, zero],
                ]
            ),
        ),
    ]
    return generators


def translation_tangent_columns(duals, K):
    columns = []
    for coord in range(4):
        column_entries = []
        for a in duals:
            scalar = a[coord]
            column_entries.extend(list(-(scalar) * a))
        columns.append(vector(K, column_entries))
    return columns


def scaling_tangent_column(duals, K):
    entries = []
    for a in duals:
        entries.extend(list(-a))
    return vector(K, entries)


def linear_symplectic_tangent_columns(duals, K):
    columns = []
    J = standard_j_matrix(K)
    generator_checks = []
    for label, X in sp4_generators(K):
        check = X.transpose() * J + J * X
        generator_checks.append(
            {
                "label": label,
                "is_zero": check.is_zero(),
            }
        )
        entries = []
        for a in duals:
            entries.extend(list(-(X.transpose()) * a))
        columns.append((label, vector(K, entries)))
    return columns, generator_checks


def build_symmetry_basis(duals, K):
    labels = []
    columns = []

    for idx, column in enumerate(translation_tangent_columns(duals, K)):
        labels.append(f"translation_e{idx}")
        columns.append(column)

    labels.append("scaling")
    columns.append(scaling_tangent_column(duals, K))

    linear_columns, generator_checks = linear_symplectic_tangent_columns(duals, K)
    for label, column in linear_columns:
        labels.append(label)
        columns.append(column)

    tangent_matrix = matrix(K, len(columns[0]), len(columns), flatten_columns(columns))
    return labels, columns, tangent_matrix, generator_checks


def write_json(path, payload):
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def main():
    K, degree, formulas, duals = exact_hko_geometry()
    geometry_payload = {
        "field": {
            "generator_name": "t",
            "generator_approx": float(real_generator_approx()),
            "degree": int(degree),
            "minimal_polynomial": [q_json(c) for c in K.defining_polynomial().list()],
            "formulas": formulas,
        },
        "facet_order": list(range(len(duals))),
        "dual_vertices_power_basis": matrix_columns_json(duals, degree),
        "dual_vertices_float": [[real_float(entry) for entry in dual] for dual in duals],
        "cross_check_current_numerical": geometry_cross_check_report(duals),
    }
    write_json(GEOMETRY_OUT, geometry_payload)

    labels, columns, tangent_matrix, generator_checks = build_symmetry_basis(duals, K)
    symmetry_payload = {
        "field_generator_name": "t",
        "dimension_ambient": int(tangent_matrix.nrows()),
        "dimension_generators": int(tangent_matrix.ncols()),
        "rank": int(tangent_matrix.rank()),
        "labels": labels,
        "columns_power_basis": matrix_columns_json(columns, degree),
        "generator_sp4_checks": generator_checks,
    }
    write_json(SYMMETRY_OUT, symmetry_payload)

    print("Wrote", GEOMETRY_OUT.relative_to(ROOT))
    print("Wrote", SYMMETRY_OUT.relative_to(ROOT))
    print("Field degree:", degree)
    print("Symmetry tangent rank:", tangent_matrix.rank())


if __name__ == "__main__":
    main()
