#!/usr/bin/env sage -python
"""
Exception-based verifier for the HKO feasible-section certificate witness.

This script does not solve for witness values. It reconstructs the HKO source
objects from definitions, reloads the witness, and checks the equations needed
by the formal feasible-section proof route.
"""

from __future__ import annotations

import json
from pathlib import Path

from sage.all import AA, QQ, RIF, NumberField, PolynomialRing, matrix, vector


PACKET_DIR = Path(__file__).resolve().parent
WITNESS_PATH = PACKET_DIR / "feasible-section-witness.json"
SUMMARY_PATH = PACKET_DIR / "verification-summary.json"


def check(condition, message):
    if not bool(condition):
        raise ValueError(f"verification failed: {message}")


def q_from_json(entry):
    return QQ(entry["num"]) / QQ(entry["den"])


def polynomial_from_desc_coefficients(coefficients_desc):
    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    degree = len(coefficients_desc) - 1
    return sum(q_from_json(coeff) * x ** (degree - idx) for idx, coeff in enumerate(coefficients_desc))


def number_field_from_witness(field_desc):
    polynomial = polynomial_from_desc_coefficients(field_desc["minimal_polynomial_coefficients_desc"])
    x = polynomial.parent().gen()
    check(polynomial == x**4 - 10 * x**2 + 5, "field polynomial must be t^4 - 10 t^2 + 5")
    check(field_desc["generator_name"] == "t", "expected field generator name t")
    check(field_desc["degree"] == 4, "expected degree-four HKO field")
    return NumberField(
        polynomial,
        field_desc["generator_name"],
        embedding=AA.polynomial_root(polynomial, RIF(0, 1)),
    )


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


def check_partial_permutation(sigma, facet_count):
    check(all(isinstance(facet, int) for facet in sigma), "sigma entries must be integers")
    check(all(0 <= facet < facet_count for facet in sigma), "sigma entries must be valid facet indices")
    check(len(set(sigma)) == len(sigma), "sigma must be a partial permutation")


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
        check(residual == vector(K, [0, 0, 0, 0, 0]), "D beta must satisfy differentiated feasibility")
    for beta_idx in fixed_indices:
        check(
            all(d_beta[beta_idx][flat_idx] == K(0) for flat_idx in range(ambient_dimension)),
            "fixed beta coordinates must have zero derivative",
        )


def verify_row(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, row):
    sigma = row["sigma"]
    minor_columns = row["minor_columns"]
    fixed_indices = row["fixed_beta_indices"]
    check_partial_permutation(sigma, len(duals))
    expected_indices = list(range(len(sigma)))
    check(len(minor_columns) == 5, "selected minor must have five columns")
    check(sorted(minor_columns) == list(minor_columns), "minor columns must be sorted")
    check(sorted(fixed_indices) == list(fixed_indices), "fixed beta indices must be sorted")
    check(
        sorted(minor_columns + fixed_indices) == expected_indices,
        "minor columns and fixed beta indices must be complementary",
    )
    check(set(minor_columns).isdisjoint(set(fixed_indices)), "minor and fixed beta indices must be disjoint")
    check(
        len(row["free_beta_values_power_basis"]) == len(fixed_indices),
        "free beta witness count must match fixed beta index count",
    )

    beta = vector_from_json(K, row["beta0_power_basis"])
    free_beta_values = vector_from_json(K, row["free_beta_values_power_basis"])
    d_beta = matrix_rows_from_json(K, row["d_beta_power_basis"])
    q = field_element_from_coeff_vector(K, row["q_power_basis"])
    action = field_element_from_coeff_vector(K, row["action_power_basis"])
    d_action = vector_from_json(K, row["d_action_flat_power_basis"])
    d_sys = vector_from_json(K, row["d_sys_flat_power_basis"])

    C = constraint_matrix(K, duals, sigma)
    e = vector(K, [0, 0, 0, 0, 1])
    check(C * beta == e, "beta must satisfy closure plus normalization")
    check(all(entry > K(0) for entry in beta), "beta must be strictly positive in the chosen real field embedding")
    check(
        all(free_beta_values[idx] == beta[fixed_indices[idx]] for idx in range(len(fixed_indices))),
        "stored free beta values must match beta at the fixed beta indices",
    )
    check(C[:, minor_columns].det() != K(0), "selected feasible-section minor must be invertible")
    check(q == q_min, "row q must equal q_min")
    check(q_value(duals, sigma, beta) == q, "row q must match the HK2017 quadratic formula")
    check(action == action_min, "row action must equal the HKO action")
    check(action == 1 / (2 * q), "row action must equal 1/(2q)")

    verify_d_beta(K, duals, sigma, beta, d_beta, fixed_indices)
    recomputed_d_q = d_q_row(K, duals, sigma, beta, d_beta)
    recomputed_d_action = vector(K, [-(entry) / (2 * q**2) for entry in recomputed_d_q])
    check(d_action == recomputed_d_action, "D action row must match differentiated action formula")
    recomputed_d_sys = vector(
        K,
        [
            action_min / volume * d_action[idx] - action_min**2 / (2 * volume**2) * volume_row[idx]
            for idx in range(len(volume_row))
        ],
    )
    check(d_sys == recomputed_d_sys, "D sys row must match differentiated systolic-ratio formula")
    for column in symmetry_columns:
        check(d_sys.dot_product(column) == K(0), "D sys row must annihilate each symmetry tangent column")
    return d_sys


def main():
    witness = json.loads(WITNESS_PATH.read_text())
    check(witness["packet"] == "hko-feasible-section-certificate", "unexpected packet id")
    check(witness["witness_version"] == 1, "unexpected witness version")
    check(witness["source_candidate_path"] == "candidate-certificate.json", "unexpected source candidate path")

    K = number_field_from_witness(witness["field"])
    t = K.gen()
    check(t > 0, "field generator t must be positive")
    check(t < 1, "field generator t must be the root in (0,1)")
    sqrt5, duals = exact_hko_geometry(K)
    check(sqrt5 > K(0), "sqrt5 expression must be positive in the chosen real field embedding")
    volume, volume_row = volume_data(K, sqrt5, duals)
    action_min = 5 * t - t**3 / 2
    q_min = 1 / (2 * action_min)
    cos36 = 1 / (sqrt5 - 1)
    cos18 = (11 * t - t**3) / 8
    hko2024_capacity_formula = 2 * cos18 * (1 + cos36)
    check(cos18 > K(0), "cos(pi/10) expression must be positive")
    check(cos36 > K(0), "cos(pi/5) expression must be positive")
    check(action_min == hko2024_capacity_formula, "action_min must match HKO2024 Proposition counterexample_prop")
    check(action_min > K(0), "action_min must be positive")
    check(q_min > K(0), "q_min must be positive")
    check(volume > K(0), "volume must be positive")
    labels, symmetry_columns, sp4_checks = symmetry_basis(K, duals)
    symmetry_matrix = matrix(K, [list(column) for column in symmetry_columns]).transpose()

    witness_duals = matrix_rows_from_json(K, witness["geometry"]["dual_vertices_power_basis"])
    check(witness["geometry"]["facet_order"] == list(range(len(duals))), "unexpected facet order")
    check(witness_duals == duals, "witness dual vertices must match reconstructed HKO geometry")
    check(field_element_from_coeff_vector(K, witness["constants"]["q_min_power_basis"]) == q_min, "q_min witness must match formula")
    check(
        field_element_from_coeff_vector(K, witness["constants"]["action_min_power_basis"]) == action_min,
        "action_min witness must match formula",
    )
    check(
        field_element_from_coeff_vector(K, witness["constants"]["volume_power_basis"]) == volume,
        "volume witness must match formula",
    )
    check(
        vector_from_json(K, witness["constants"]["volume_derivative_flat_power_basis"]) == volume_row,
        "volume derivative witness must match formula",
    )
    check(witness["symmetry"]["labels"] == labels, "symmetry labels must match reconstructed generators")
    check(
        matrix_rows_from_json(K, witness["symmetry"]["columns_power_basis"]) == symmetry_columns,
        "symmetry columns must match reconstructed generators",
    )
    check(symmetry_matrix.rank() == 15, "symmetry tangent matrix must have rank 15")
    check(witness["symmetry"]["rank"] == 15, "witness symmetry rank must be 15")
    expected_sp4_checks = [{"label": label, "is_sp4": bool(passed)} for label, passed in sp4_checks]
    check(all(passed for _label, passed in sp4_checks), "reconstructed sp4 generators must pass the sp4 test")
    check(
        witness["symmetry"]["generator_sp4_checks"] == expected_sp4_checks,
        "witness sp4 generator checks must match reconstructed labels and results",
    )
    check(len(witness["rows"]) == 26, "witness must contain 26 selected rows")
    check(
        [row["certificate_index"] for row in witness["rows"]] == list(range(len(witness["rows"]))),
        "certificate indices must be consecutive",
    )

    verified_rows = [
        verify_row(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, row)
        for row in witness["rows"]
    ]
    row_matrix = matrix(K, [list(row) for row in verified_rows])
    check(row_matrix.rank() == 25, "verified rows must have rank 25")

    lambdas = vector_from_json(K, witness["convex_certificate"]["lambda_power_basis"])
    check(len(lambdas) == len(verified_rows), "lambda count must match verified row count")
    check(all(entry > K(0) for entry in lambdas), "convex coefficients must be strictly positive")
    check(sum(lambdas, K(0)) == K(1), "convex coefficients must sum to 1")
    check(
        lambdas * row_matrix == vector(K, [0 for _ in range(row_matrix.ncols())]),
        "convex coefficients must combine rows to zero",
    )
    check(witness["convex_certificate"]["rank"] == 25, "convex certificate rank field must be 25")

    summary = {
        "packet": "hko-feasible-section-certificate",
        "verified": True,
        "row_count": len(verified_rows),
        "ambient_dimension": row_matrix.ncols(),
        "row_rank": int(row_matrix.rank()),
        "symmetry_rank": int(symmetry_matrix.rank()),
        "lambda_count": len(lambdas),
        "checks": [
            "ordered number field pinned to the root t in (0,1) of t^4 - 10 t^2 + 5",
            "hko geometry reconstructed from definition",
            "action_min matched HKO2024 Proposition counterexample_prop in the exact field",
            "volume and volume derivative matched source formulas",
            "symmetry tangent generators reconstructed and rank checked",
            "each beta is positive and satisfies closure plus normalization",
            "stored free beta values match beta at the fixed beta indices",
            "each minor-column set is complementary to the fixed beta index set",
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
