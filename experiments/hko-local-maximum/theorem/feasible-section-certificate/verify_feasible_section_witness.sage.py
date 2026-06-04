#!/usr/bin/env sage -python
"""
Assert-only verifier for the HKO feasible-section certificate witness.

This script does not solve for witness values. It reconstructs the HKO source
objects from definitions, reloads the witness, and checks the equations needed
by the formal feasible-section proof route.
"""

from __future__ import annotations

import json
from pathlib import Path

from sage.all import QQ, NumberField, PolynomialRing, matrix, vector, sqrt, RR


PACKET_DIR = Path(__file__).resolve().parent
WITNESS_PATH = PACKET_DIR / "feasible-section-witness.json"
SUMMARY_PATH = PACKET_DIR / "verification-summary.json"


def q_from_json(entry):
    return QQ(entry["num"]) / QQ(entry["den"])


def polynomial_from_desc_coefficients(coefficients_desc):
    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    degree = len(coefficients_desc) - 1
    return sum(q_from_json(coeff) * x ** (degree - idx) for idx, coeff in enumerate(coefficients_desc))


def field_element_from_coeff_vector(K, coeffs):
    t = K.gen()
    return sum(q_from_json(coeff) * t**idx for idx, coeff in enumerate(coeffs))


def vector_from_json(K, entries):
    return vector(K, [field_element_from_coeff_vector(K, entry) for entry in entries])


def matrix_rows_from_json(K, rows):
    return [vector_from_json(K, row) for row in rows]


def exact_hko_geometry(K):
    t = K.gen()
    sqrt5 = (QQ(5) - t**2) / 2
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
    return sqrt5, duals


def omega(a, b):
    return a[0] * b[2] + a[1] * b[3] - a[2] * b[0] - a[3] * b[1]


def unit_vector(K, coord):
    entries = [K(0), K(0), K(0), K(0)]
    entries[coord] = K(1)
    return vector(K, entries)


def constraint_matrix(K, duals, sigma):
    return matrix(K, [[duals[facet][row] for facet in sigma] for row in range(4)] + [[K(1) for _ in sigma]])


def q_value(duals, sigma, beta):
    return sum(
        beta[i] * beta[j] * omega(duals[sigma[j]], duals[sigma[i]])
        for i in range(1, len(sigma))
        for j in range(i)
    )


def volume_data(K, sqrt5, duals):
    volume = QQ(25) * (QQ(5) + sqrt5) / QQ(32)
    scalar = QQ(25) / QQ(32) + QQ(5) * sqrt5 / QQ(16)
    volume_row = []
    for dual in duals:
        volume_row.extend([-scalar * entry for entry in dual])
    return volume, vector(K, volume_row)


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


def sp4_generators(K):
    z = K(0)
    o = K(1)

    def mat(rows):
        return matrix(K, rows)

    return [
        ("sp_a11", mat([[o, z, z, z], [z, z, z, z], [z, z, -o, z], [z, z, z, z]])),
        ("sp_a12", mat([[z, o, z, z], [z, z, z, z], [z, z, z, z], [z, z, -o, z]])),
        ("sp_a21", mat([[z, z, z, z], [o, z, z, z], [z, z, z, -o], [z, z, z, z]])),
        ("sp_a22", mat([[z, z, z, z], [z, o, z, z], [z, z, z, z], [z, z, z, -o]])),
        ("sp_b11", mat([[z, z, o, z], [z, z, z, z], [z, z, z, z], [z, z, z, z]])),
        ("sp_b12", mat([[z, z, z, o], [z, z, o, z], [z, z, z, z], [z, z, z, z]])),
        ("sp_b22", mat([[z, z, z, z], [z, z, z, o], [z, z, z, z], [z, z, z, z]])),
        ("sp_c11", mat([[z, z, z, z], [z, z, z, z], [o, z, z, z], [z, z, z, z]])),
        ("sp_c12", mat([[z, z, z, z], [z, z, z, z], [z, o, z, z], [o, z, z, z]])),
        ("sp_c22", mat([[z, z, z, z], [z, z, z, z], [z, z, z, z], [z, o, z, z]])),
    ]


def symmetry_basis(K, duals):
    labels = []
    columns = []
    for coord in range(4):
        labels.append(f"translation_e{coord}")
        entries = []
        for dual in duals:
            entries.extend(list(-dual[coord] * dual))
        columns.append(vector(K, entries))

    labels.append("scaling")
    entries = []
    for dual in duals:
        entries.extend(list(-dual))
    columns.append(vector(K, entries))

    J = standard_j_matrix(K)
    sp4_checks = []
    for label, X in sp4_generators(K):
        sp4_checks.append((label, (X.transpose() * J + J * X).is_zero()))
        labels.append(label)
        entries = []
        for dual in duals:
            entries.extend(list(-(X.transpose()) * dual))
        columns.append(vector(K, entries))
    return labels, columns, sp4_checks


def d_q_row(K, duals, sigma, beta, d_beta):
    ambient_dimension = len(duals) * 4
    row = []
    for flat_idx in range(ambient_dimension):
        facet = flat_idx // 4
        coord = flat_idx % 4
        unit = unit_vector(K, coord)
        total = K(0)
        for i in range(1, len(sigma)):
            for j in range(i):
                beta_i_prime = d_beta[i][flat_idx]
                beta_j_prime = d_beta[j][flat_idx]
                a_i = duals[sigma[i]]
                a_j = duals[sigma[j]]
                total += (beta_i_prime * beta[j] + beta[i] * beta_j_prime) * omega(a_j, a_i)
                if sigma[j] == facet:
                    total += beta[i] * beta[j] * omega(unit, a_i)
                if sigma[i] == facet:
                    total += beta[i] * beta[j] * omega(a_j, unit)
        row.append(total)
    return vector(K, row)


def assert_partial_permutation(sigma, facet_count):
    assert all(isinstance(facet, int) for facet in sigma)
    assert all(0 <= facet < facet_count for facet in sigma)
    assert len(set(sigma)) == len(sigma)


def verify_d_beta(K, duals, sigma, beta, d_beta, fixed_indices):
    C = constraint_matrix(K, duals, sigma)
    ambient_dimension = len(duals) * 4
    for flat_idx in range(ambient_dimension):
        facet = flat_idx // 4
        coord = flat_idx % 4
        d_beta_column = vector(K, [d_beta[beta_idx][flat_idx] for beta_idx in range(len(sigma))])
        residual = C * d_beta_column
        if facet in sigma:
            residual[coord] += beta[sigma.index(facet)]
        assert residual == vector(K, [0, 0, 0, 0, 0])
    for beta_idx in fixed_indices:
        assert all(d_beta[beta_idx][flat_idx] == K(0) for flat_idx in range(ambient_dimension))


def verify_row(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, row):
    sigma = row["sigma"]
    minor_columns = row["minor_columns"]
    fixed_indices = row["fixed_beta_indices"]
    assert_partial_permutation(sigma, len(duals))

    beta = vector_from_json(K, row["beta0_power_basis"])
    d_beta = matrix_rows_from_json(K, row["d_beta_power_basis"])
    q = field_element_from_coeff_vector(K, row["q_power_basis"])
    action = field_element_from_coeff_vector(K, row["action_power_basis"])
    d_action = vector_from_json(K, row["d_action_flat_power_basis"])
    d_sys = vector_from_json(K, row["d_sys_flat_power_basis"])

    C = constraint_matrix(K, duals, sigma)
    e = vector(K, [0, 0, 0, 0, 1])
    assert C * beta == e
    assert all(entry > K(0) for entry in beta)
    assert C[:, minor_columns].det() != K(0)
    assert q == q_min
    assert q_value(duals, sigma, beta) == q
    assert action == action_min
    assert action == 1 / (2 * q)

    verify_d_beta(K, duals, sigma, beta, d_beta, fixed_indices)
    recomputed_d_q = d_q_row(K, duals, sigma, beta, d_beta)
    recomputed_d_action = vector(K, [-(entry) / (2 * q**2) for entry in recomputed_d_q])
    assert d_action == recomputed_d_action
    recomputed_d_sys = vector(
        K,
        [
            action_min / volume * d_action[idx] - action_min**2 / (2 * volume**2) * volume_row[idx]
            for idx in range(len(volume_row))
        ],
    )
    assert d_sys == recomputed_d_sys
    for column in symmetry_columns:
        assert d_sys.dot_product(column) == K(0)
    return d_sys


def main():
    witness = json.loads(WITNESS_PATH.read_text())
    polynomial = polynomial_from_desc_coefficients(witness["field"]["minimal_polynomial_coefficients_desc"])
    K = NumberField(polynomial, witness["field"]["generator_name"])
    t = K.gen()
    sqrt5, duals = exact_hko_geometry(K)
    volume, volume_row = volume_data(K, sqrt5, duals)
    action_min = 5 * t - t**3 / 2
    q_min = 1 / (2 * action_min)
    labels, symmetry_columns, sp4_checks = symmetry_basis(K, duals)
    symmetry_matrix = matrix(K, [list(column) for column in symmetry_columns]).transpose()

    witness_duals = matrix_rows_from_json(K, witness["geometry"]["dual_vertices_power_basis"])
    assert witness_duals == duals
    assert field_element_from_coeff_vector(K, witness["constants"]["q_min_power_basis"]) == q_min
    assert field_element_from_coeff_vector(K, witness["constants"]["action_min_power_basis"]) == action_min
    assert field_element_from_coeff_vector(K, witness["constants"]["volume_power_basis"]) == volume
    assert vector_from_json(K, witness["constants"]["volume_derivative_flat_power_basis"]) == volume_row
    assert witness["symmetry"]["labels"] == labels
    assert matrix_rows_from_json(K, witness["symmetry"]["columns_power_basis"]) == symmetry_columns
    assert symmetry_matrix.rank() == 15
    assert witness["symmetry"]["rank"] == 15
    assert all(passed for _label, passed in sp4_checks)
    assert all(entry["is_sp4"] for entry in witness["symmetry"]["generator_sp4_checks"])

    verified_rows = [
        verify_row(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, row)
        for row in witness["rows"]
    ]
    row_matrix = matrix(K, [list(row) for row in verified_rows])
    assert row_matrix.rank() == 25

    lambdas = vector_from_json(K, witness["convex_certificate"]["lambda_power_basis"])
    assert len(lambdas) == len(verified_rows)
    assert all(entry > K(0) for entry in lambdas)
    assert sum(lambdas, K(0)) == K(1)
    assert lambdas * row_matrix == vector(K, [0 for _ in range(row_matrix.ncols())])
    assert witness["convex_certificate"]["rank"] == 25

    summary = {
        "packet": "hko-feasible-section-certificate",
        "verified": True,
        "row_count": len(verified_rows),
        "ambient_dimension": row_matrix.ncols(),
        "row_rank": int(row_matrix.rank()),
        "symmetry_rank": int(symmetry_matrix.rank()),
        "lambda_count": len(lambdas),
        "checks": [
            "hko geometry reconstructed from definition",
            "volume and volume derivative matched source formulas",
            "symmetry tangent generators reconstructed and rank checked",
            "each beta is positive and satisfies closure plus normalization",
            "each selected minor is invertible",
            "each action equals the HKO action",
            "each D beta satisfies the feasible-section derivative equation",
            "each D action and D sys row matches the formula",
            "each D sys row annihilates the symmetry tangent space",
            "selected rows have exact rank 25",
            "positive lambdas sum to 1 and combine rows to 0",
        ],
    }
    SUMMARY_PATH.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print("Verified HKO feasible-section witness")
    print(f"Rows: {summary['row_count']}")
    print(f"Row rank: {summary['row_rank']}")
    print(f"Symmetry rank: {summary['symmetry_rank']}")


if __name__ == "__main__":
    main()
