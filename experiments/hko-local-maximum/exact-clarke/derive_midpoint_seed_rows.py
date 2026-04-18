#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Exactify the midpoint-style numerical seven-facet permutation seed orbits.

Goal: turn the current midpoint-style numerical seven-facet permutation-seed
      planning surface into exact midpoint-family `sys` rows in the shared
      facet-major `R^40` order.
Input Artifacts: experiments/hko-local-maximum/exact-clarke/numerical-family-reconciliation.json
                 experiments/hko-local-maximum/exact-clarke/numerical-permutation-orbits.json
                 experiments/hko-local-maximum/exact-clarke/hko-volume-derivative.json
Output Artifacts: experiments/hko-local-maximum/exact-clarke/midpoint-seed-rows.json
"""

from __future__ import annotations

import json
from pathlib import Path

from sympy import Matrix, Rational, simplify, sqrt, sympify
from sympy.polys.domains import QQ
from sympy.polys.matrices import DomainMatrix
from sympy.polys.numberfields import to_number_field


EXPERIMENT_DIR = Path(__file__).resolve().parent
RECONCILIATION_PATH = EXPERIMENT_DIR / "numerical-family-reconciliation.json"
PERMUTATION_ORBITS_PATH = EXPERIMENT_DIR / "numerical-permutation-orbits.json"
VOLUME_PATH = EXPERIMENT_DIR / "hko-volume-derivative.json"
OUTPUT_PATH = EXPERIMENT_DIR / "midpoint-seed-rows.json"

FIELD_GENERATOR_EXPR = sqrt(5 - 2 * sqrt(5))
FIELD_DEGREE = to_number_field(FIELD_GENERATOR_EXPR).minpoly.degree()
FIELD_DOMAIN = QQ.algebraic_field(FIELD_GENERATOR_EXPR)


def rational_json(value):
    value = sympify(value)
    num, den = value.as_numer_denom()
    return {"num": int(num), "den": int(den)}


def coeff_vector_json(expr):
    alg = to_number_field(simplify(sympify(expr)), FIELD_GENERATOR_EXPR)
    coeffs_desc = list(alg.coeffs())
    coeffs_asc = list(reversed(coeffs_desc))
    coeffs_asc += [0] * (FIELD_DEGREE - len(coeffs_asc))
    return [rational_json(coeff) for coeff in coeffs_asc]


def field_expr_str(expr):
    return str(to_number_field(simplify(sympify(expr)), FIELD_GENERATOR_EXPR).as_expr())


def exact_hko_duals():
    t = FIELD_GENERATOR_EXPR
    sqrt5 = (5 - t**2) / 2
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1
    return [
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


def omega(lhs: Matrix, rhs: Matrix, j0: Matrix):
    return simplify((lhs.T * j0 * rhs)[0])


def capacity_gradient_row(duals, permutation, beta_profile, q_value, mu_solution, j0):
    q_sq = simplify(q_value**2)
    row = []
    for facet in range(10):
        if facet not in permutation:
            row.extend([0, 0, 0, 0])
            continue
        orbit_index = permutation.index(facet)
        partial_sum = Matrix([0, 0, 0, 0])
        for index in range(orbit_index):
            partial_sum += simplify(beta_profile[index] * duals[permutation[index]])
        inner = simplify(2 * partial_sum + beta_profile[orbit_index] * duals[facet])
        # Envelope theorem: dq/da_k includes the ordered symplectic-pairing term
        # and the closure-constraint multiplier contribution.
        dq_da = simplify(beta_profile[orbit_index] * (-j0 * inner + Matrix(mu_solution[:4])))
        row.extend([simplify(-entry / (2 * q_sq)) for entry in dq_da])
    return row


def classify_exact_beta(numerical_beta):
    a_value = simplify(sqrt(5) / 10)
    b_value = simplify(Rational(1, 4) - sqrt(5) / 20)
    c_value = simplify(b_value / 2)
    d_value = simplify((a_value + b_value) / 2)
    palette = [a_value, b_value, c_value, d_value]
    palette_float = [float(entry.evalf(30)) for entry in palette]
    exact = []
    for value in numerical_beta:
        index = min(range(len(palette)), key=lambda idx: abs(value - palette_float[idx]))
        exact.append(palette[index])
    return exact


def main() -> None:
    reconciliation = json.loads(RECONCILIATION_PATH.read_text())
    permutation_orbits = json.loads(PERMUTATION_ORBITS_PATH.read_text())
    volume = json.loads(VOLUME_PATH.read_text())

    classes_by_id = {
        entry["id"]: entry
        for entry in reconciliation["size7_gradient_classes"]
    }
    duals = exact_hko_duals()
    j0 = Matrix([[0, 0, -1, 0], [0, 0, 0, -1], [1, 0, 0, 0], [0, 1, 0, 0]])
    total_volume = sympify(volume["total_volume"])
    volume_row = [sympify(entry) for entry in volume["volume_derivative_row_flat"]]

    midpoint_seed_rows = []
    row_matrix_entries = []
    for orbit in permutation_orbits["size7_permutation_orbits"]:
        seed_id = orbit["seed_id"]
        seed_class = classes_by_id[seed_id]
        coefficient = seed_class.get("segment_witness", {}).get("coefficient_on_second")
        if coefficient != 0.5:
            continue

        permutation = list(seed_class["representative_permutation"])
        beta_profile = classify_exact_beta(seed_class["representative_beta"])

        h_matrix = Matrix.zeros(len(permutation), len(permutation))
        for i in range(len(permutation)):
            for j in range(i + 1, len(permutation)):
                value = omega(duals[permutation[i]], duals[permutation[j]], j0)
                h_matrix[i, j] = value
                h_matrix[j, i] = value
        beta_column = Matrix(beta_profile)
        q_value = simplify(Rational(1, 2) * (beta_column.T * h_matrix * beta_column)[0])

        stationarity_matrix = Matrix(
            [[duals[facet][coord] for coord in range(4)] + [1] for facet in permutation]
        )
        stationarity_rhs = -h_matrix * beta_column
        mu_solution = [simplify(entry) for entry in stationarity_matrix.gauss_jordan_solve(stationarity_rhs)[0]]
        closure = Matrix([0, 0, 0, 0])
        for beta_entry, facet in zip(beta_profile, permutation, strict=True):
            closure += beta_entry * duals[facet]
        closure = [simplify(entry) for entry in closure]
        normalization = simplify(sum(beta_profile))

        capacity = simplify(1 / (2 * q_value))
        sys_value = simplify(capacity**2 / (2 * total_volume))
        capacity_row = capacity_gradient_row(duals, permutation, beta_profile, q_value, mu_solution, j0)
        sys_row = [
            simplify((capacity / total_volume) * dc - (sys_value / total_volume) * dv)
            for dc, dv in zip(capacity_row, volume_row, strict=True)
        ]
        row_matrix_entries.append(sys_row)

        midpoint_seed_rows.append(
            {
                "seed_id": seed_id,
                "subset": seed_class["subset"],
                "representative_permutation": permutation,
                "exact_beta_profile": [field_expr_str(entry) for entry in beta_profile],
                "closure_check": [field_expr_str(entry) for entry in closure],
                "normalization_check": field_expr_str(normalization),
                "q_value_signed": field_expr_str(q_value),
                "capacity": str(simplify(capacity)),
                "sys_value": str(simplify(sys_value)),
                "sys_row_flat": [field_expr_str(entry) for entry in sys_row],
                "sys_row_flat_power_basis": [coeff_vector_json(entry) for entry in sys_row],
            }
        )

    row_matrix = Matrix(row_matrix_entries)
    rank = DomainMatrix.from_Matrix(row_matrix, FIELD_DOMAIN).rank()

    payload = {
        "field_generator": "t = sqrt(5 - 2*sqrt(5)) = tan(pi/5)",
        "input_artifacts": [
            RECONCILIATION_PATH.name,
            PERMUTATION_ORBITS_PATH.name,
            VOLUME_PATH.name,
        ],
        "n_exactified_midpoint_seed_rows": len(midpoint_seed_rows),
        "rank_of_midpoint_seed_rows": rank,
        "midpoint_seed_rows": midpoint_seed_rows,
        "theorem_use": (
            "This artifact exactifies the midpoint-style seven-facet seed orbits "
            "whose current numerical segment coefficient is 1/2. It is Packet 3 "
            "scaffolding, not theorem input, because the seed choice still comes "
            "from numerical planning."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
