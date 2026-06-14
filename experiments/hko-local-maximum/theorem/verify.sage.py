#!/usr/bin/env sage -python
"""
Verifier for the HKO feasible-section certificate.

This script reads `witness.json`, reconstructs the HKO source objects from
definitions, computes the exact algebraic data needed for the witness entries,
and checks the exact finite predicate used by the formal feasible-section proof
route.
"""

from __future__ import annotations

import json
from fractions import Fraction
from pathlib import Path

from sage.all import AA, QQ, RIF, NumberField, PolynomialRing, matrix, vector, sqrt, RR


PACKET_DIR = Path(__file__).resolve().parent
WITNESS_PATH = PACKET_DIR / "witness.json"
SUMMARY_PATH = PACKET_DIR / "verification-summary.json"


def check(condition, message):
    if not bool(condition):
        raise ValueError(f"verification failed: {message}")


def exact_hko_geometry():
    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    polynomial = x**4 - 10 * x**2 + 5
    K = NumberField(polynomial, "t", embedding=AA.polynomial_root(polynomial, RIF(0, 1)))
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
    return K, t, sqrt5, duals


def real_float(x):
    t_real = sqrt(RR(5) - RR(2) * sqrt(RR(5)))
    return float(sum(RR(coeff) * t_real**idx for idx, coeff in enumerate(x.list())))


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


def flatten_columns(columns):
    return [columns[col][row] for row in range(len(columns[0])) for col in range(len(columns))]


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
    generator_checks = []
    for label, X in sp4_generators(K):
        generator_checks.append({"label": label, "is_sp4": (X.transpose() * J + J * X).is_zero()})
        labels.append(label)
        entries = []
        for dual in duals:
            entries.extend(list(-(X.transpose()) * dual))
        columns.append(vector(K, entries))

    tangent_matrix = matrix(K, len(columns[0]), len(columns), flatten_columns(columns))
    return labels, columns, tangent_matrix, generator_checks


def rational_candidates(hint):
    values = []
    for max_denominator in [10, 25, 50, 100, 250, 1000, 10_000, 1_000_000]:
        values.append(Fraction(str(hint)).limit_denominator(max_denominator))
    values.append(Fraction(str(hint)))
    values.extend(Fraction(n, 100) for n in range(1, 50))

    deduped = []
    seen = set()
    for value in sorted(values, key=lambda q: abs(float(q) - hint)):
        if value not in seen:
            deduped.append(value)
            seen.add(value)
    return deduped


def univariate_from_multivariate(K, S, poly, solve_idx):
    U = PolynomialRing(K, f"u{solve_idx}")
    u = U.gen()
    result = U(0)
    for exponents, coeff in poly.dict().items():
        if isinstance(exponents, int):
            exponents = (exponents,)
        for idx, exponent in enumerate(exponents):
            if idx != solve_idx and exponent != 0:
                return None
        result += coeff * u ** exponents[solve_idx]
    return result


def beta_polynomials(K, duals, sigma, minor_columns, fixed_indices):
    S = PolynomialRing(K, [f"free_{idx}" for idx in fixed_indices])
    variables = S.gens()
    C = constraint_matrix(K, duals, sigma)
    e = vector(K, [0, 0, 0, 0, 1])
    C_I = C[:, minor_columns]
    C_J = C[:, fixed_indices]

    C_I_S = matrix(S, C_I.nrows(), C_I.ncols(), [S(entry) for entry in C_I.list()])
    C_J_S = matrix(S, C_J.nrows(), C_J.ncols(), [S(entry) for entry in C_J.list()])
    e_S = vector(S, [S(entry) for entry in e])
    beta_I = C_I_S.inverse() * (e_S - C_J_S * vector(S, variables))

    beta = [None] * len(sigma)
    for idx, column in enumerate(minor_columns):
        beta[column] = beta_I[idx]
    for idx, column in enumerate(fixed_indices):
        beta[column] = variables[idx]
    return S, variables, beta


def substitute_beta(K, beta_poly, variables, free_values):
    substitutions = {variables[idx]: K(free_values[idx]) for idx in range(len(variables))}
    return vector(K, [K(entry.subs(substitutions)) for entry in beta_poly])


def beta_is_valid(K, duals, sigma, beta, q_min):
    C = constraint_matrix(K, duals, sigma)
    e = vector(K, [0, 0, 0, 0, 1])
    return C * beta == e and all(entry > K(0) for entry in beta) and q_value(duals, sigma, beta) == q_min


def find_free_values(K, duals, witness_row, beta_poly, S, variables, q_min):
    q_poly = S(q_value(duals, witness_row["sigma"], beta_poly) - q_min)
    fixed_hints = witness_row["fixed_beta_values_f64"]
    nfree = len(variables)

    if q_poly == 0:
        for free_values in zip(*(rational_candidates(hint) for hint in fixed_hints)):
            beta = substitute_beta(K, beta_poly, variables, free_values)
            if beta_is_valid(K, duals, witness_row["sigma"], beta, q_min):
                return [K(value) for value in free_values], beta
        raise RuntimeError(
            f"entry {witness_row['certificate_index']}: q polynomial is zero but no positive rational free beta was found"
        )

    if nfree == 1:
        univariate = univariate_from_multivariate(K, S, q_poly, 0)
        if univariate is None:
            raise RuntimeError(f"entry {witness_row['certificate_index']}: one-free-variable polynomial was not univariate")
        roots = [root for root, _multiplicity in univariate.roots()]
        roots.sort(key=lambda root: abs(real_float(root) - fixed_hints[0]))
        for root in roots:
            beta = substitute_beta(K, beta_poly, variables, [root])
            if beta_is_valid(K, duals, witness_row["sigma"], beta, q_min):
                return [root], beta
        raise RuntimeError(f"entry {witness_row['certificate_index']}: no valid root among {len(roots)} one-free-variable roots")

    if nfree == 2:
        for solve_idx in [0, 1]:
            other_idx = 1 - solve_idx
            for rational_value in rational_candidates(fixed_hints[other_idx]):
                substituted = q_poly.subs({variables[other_idx]: K(rational_value)})
                univariate = univariate_from_multivariate(K, S, substituted, solve_idx)
                if univariate is None:
                    continue
                if univariate == 0:
                    root_candidates = [K(value) for value in rational_candidates(fixed_hints[solve_idx])[:12]]
                else:
                    root_candidates = [root for root, _multiplicity in univariate.roots()]
                root_candidates.sort(key=lambda root: abs(real_float(root) - fixed_hints[solve_idx]))
                for root in root_candidates:
                    free_values = [None, None]
                    free_values[solve_idx] = root
                    free_values[other_idx] = K(rational_value)
                    beta = substitute_beta(K, beta_poly, variables, free_values)
                    if beta_is_valid(K, duals, witness_row["sigma"], beta, q_min):
                        return free_values, beta
        raise RuntimeError(f"entry {witness_row['certificate_index']}: no valid two-free-variable exact beta found")

    raise RuntimeError(f"entry {witness_row['certificate_index']}: unsupported free beta count {nfree}")


def d_beta_matrix(K, duals, sigma, beta, minor_columns, fixed_indices):
    C = constraint_matrix(K, duals, sigma)
    C_I = C[:, minor_columns]
    ambient_dimension = len(duals) * 4
    rows = [[K(0) for _ in range(ambient_dimension)] for _ in sigma]

    for flat_idx in range(ambient_dimension):
        facet = flat_idx // 4
        coord = flat_idx % 4
        rhs = vector(K, [0, 0, 0, 0, 0])
        if facet in sigma:
            sigma_idx = sigma.index(facet)
            rhs[coord] = -beta[sigma_idx]
        beta_I_prime = C_I.solve_right(rhs)
        for minor_idx, beta_idx in enumerate(minor_columns):
            rows[beta_idx][flat_idx] = beta_I_prime[minor_idx]

    for beta_idx in fixed_indices:
        check(all(entry == K(0) for entry in rows[beta_idx]), "fixed beta coordinates must have zero derivative")
    return rows


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


def compute_entry(K, duals, volume, volume_row, action_min, q_min, witness_row):
    sigma = witness_row["sigma"]
    minor_columns = witness_row["minor_columns_exact"]
    fixed_indices = witness_row["fixed_beta_indices"]
    S, variables, beta_poly = beta_polynomials(K, duals, sigma, minor_columns, fixed_indices)
    free_values, beta = find_free_values(K, duals, witness_row, beta_poly, S, variables, q_min)
    d_beta = d_beta_matrix(K, duals, sigma, beta, minor_columns, fixed_indices)
    q = q_value(duals, sigma, beta)
    d_q = d_q_row(K, duals, sigma, beta, d_beta)
    d_action = vector(K, [-(entry) / (2 * q**2) for entry in d_q])
    d_sys = vector(
        K,
        [
            action_min / volume * d_action[idx] - action_min**2 / (2 * volume**2) * volume_row[idx]
            for idx in range(len(volume_row))
        ],
    )
    action = 1 / (2 * q)

    check(q == q_min, "computed q must equal q_min")
    check(action == action_min, "computed action must equal the HKO action")
    return {
        "certificate_index": witness_row["certificate_index"],
        "source_feasible_section_row_index": witness_row["source_feasible_section_row_index"],
        "sigma": sigma,
        "minor_columns": minor_columns,
        "fixed_beta_indices": fixed_indices,
        "free_beta_values": vector(K, free_values),
        "beta": beta,
        "q": q,
        "action": action,
        "d_beta": d_beta,
        "d_action": d_action,
        "d_sys": d_sys,
        "max_abs_d_sys_delta_vs_f64_hint": max(
            abs(real_float(entry) - hint)
            for entry, hint in zip(d_sys, witness_row["d_sys_flat_f64"], strict=True)
        ),
    }


def check_partial_permutation(sigma, facet_count):
    check(all(isinstance(facet, int) for facet in sigma), "sigma entries must be integers")
    check(all(0 <= facet < facet_count for facet in sigma), "sigma entries must be valid facet indices")
    check(len(set(sigma)) == len(sigma), "sigma must be a partial permutation")


def check_witness_rows(witness, facet_count):
    check(len(witness["entries"]) == 26, "witness must contain 26 selected entries")
    check(
        [row["certificate_index"] for row in witness["entries"]] == list(range(len(witness["entries"]))),
        "certificate indices must be consecutive",
    )
    for row in witness["entries"]:
        sigma = row["sigma"]
        minor_columns = row["minor_columns_exact"]
        fixed_indices = row["fixed_beta_indices"]
        expected_indices = list(range(len(sigma)))
        check_partial_permutation(sigma, facet_count)
        check(len(minor_columns) == 5, "selected minor must have five columns")
        check(sorted(minor_columns) == list(minor_columns), "minor columns must be sorted")
        check(sorted(fixed_indices) == list(fixed_indices), "fixed beta indices must be sorted")
        check(
            sorted(minor_columns + fixed_indices) == expected_indices,
            "minor columns and fixed beta indices must be complementary",
        )
        check(set(minor_columns).isdisjoint(set(fixed_indices)), "minor and fixed beta indices must be disjoint")
        check(
            len(row["fixed_beta_values_f64"]) == len(fixed_indices),
            "fixed beta value hint count must match fixed beta index count",
        )


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


def verify_entry(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, entry):
    sigma = entry["sigma"]
    minor_columns = entry["minor_columns"]
    fixed_indices = entry["fixed_beta_indices"]
    beta = entry["beta"]
    free_beta_values = entry["free_beta_values"]
    d_beta = entry["d_beta"]
    q = entry["q"]
    action = entry["action"]
    d_action = entry["d_action"]
    d_sys = entry["d_sys"]

    C = constraint_matrix(K, duals, sigma)
    e = vector(K, [0, 0, 0, 0, 1])
    check(C * beta == e, "beta must satisfy closure plus normalization")
    check(all(beta_i > K(0) for beta_i in beta), "beta must be strictly positive in the chosen real field embedding")
    check(
        all(free_beta_values[idx] == beta[fixed_indices[idx]] for idx in range(len(fixed_indices))),
        "computed free beta values must match beta at the fixed beta indices",
    )
    check(C[:, minor_columns].det() != K(0), "selected feasible-section minor must be invertible")
    check(q == q_min, "entry q must equal q_min")
    check(q_value(duals, sigma, beta) == q, "entry q must match the HK2017 quadratic formula")
    check(action == action_min, "entry action must equal the HKO action")
    check(action == 1 / (2 * q), "entry action must equal 1/(2q)")

    verify_d_beta(K, duals, sigma, beta, d_beta, fixed_indices)
    recomputed_d_q = d_q_row(K, duals, sigma, beta, d_beta)
    recomputed_d_action = vector(K, [-(item) / (2 * q**2) for item in recomputed_d_q])
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


def compute_convex_coefficients(K, verified_rows):
    row_matrix = matrix(K, [list(row) for row in verified_rows])
    rank = row_matrix.rank()
    check(rank == 25, "verified rows must have rank 25")

    kernel = row_matrix.transpose().right_kernel().basis()
    check(len(kernel) == 1, "left kernel must be one-dimensional")
    lambdas = list(kernel[0])
    if all(entry < K(0) for entry in lambdas):
        lambdas = [-entry for entry in lambdas]
    check(all(entry > K(0) for entry in lambdas), "convex coefficients must be strictly positive after sign choice")
    total = sum(lambdas, K(0))
    lambdas = vector(K, [entry / total for entry in lambdas])
    check(sum(lambdas, K(0)) == K(1), "convex coefficients must sum to 1")
    check(lambdas * row_matrix == vector(K, [0 for _ in range(row_matrix.ncols())]), "convex coefficients must combine rows to zero")
    return row_matrix, lambdas


def main():
    witness = json.loads(WITNESS_PATH.read_text())
    check(witness["packet"] == "hko-feasible-section-certificate", "unexpected packet id")
    check(witness["witness_version"] == 1, "unexpected witness version")

    K, t, sqrt5, duals = exact_hko_geometry()
    check(t > 0, "field generator t must be positive")
    check(t < 1, "field generator t must be the root in (0,1)")
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

    labels, symmetry_columns, symmetry_matrix, sp4_checks = symmetry_basis(K, duals)
    check(all(item["is_sp4"] for item in sp4_checks), "reconstructed sp4 generators must pass the sp4 test")
    check(symmetry_matrix.rank() == 15, "symmetry tangent matrix must have rank 15")
    check(witness["certificate_goal"]["selected_entry_count"] == 26, "witness goal must request 26 entries")
    check(witness["certificate_goal"]["ambient_dimension"] == 40, "witness goal must use ambient dimension 40")
    check(witness["certificate_goal"]["symmetry_dimension"] == 15, "witness goal must use symmetry dimension 15")
    check(witness["certificate_goal"]["quotient_dimension"] == 25, "witness goal must use quotient dimension 25")
    check_witness_rows(witness, len(duals))

    entries = [
        compute_entry(K, duals, volume, volume_row, action_min, q_min, row)
        for row in witness["entries"]
    ]
    verified_rows = [
        verify_entry(K, duals, volume, volume_row, action_min, q_min, symmetry_columns, entry)
        for entry in entries
    ]
    row_matrix, lambdas = compute_convex_coefficients(K, verified_rows)

    summary = {
        "packet": "hko-feasible-section-certificate",
        "source_witness_path": str(WITNESS_PATH.relative_to(PACKET_DIR)),
        "verified": True,
        "row_count": len(verified_rows),
        "ambient_dimension": row_matrix.ncols(),
        "row_rank": int(row_matrix.rank()),
        "symmetry_rank": int(symmetry_matrix.rank()),
        "symmetry_label_count": len(labels),
        "lambda_count": len(lambdas),
        "min_lambda_f64": min(real_float(entry) for entry in lambdas),
        "max_lambda_f64": max(real_float(entry) for entry in lambdas),
        "max_abs_d_sys_delta_vs_f64_hint": max(entry["max_abs_d_sys_delta_vs_f64_hint"] for entry in entries),
        "checks": [
            "ordered number field pinned to the root t in (0,1) of t^4 - 10 t^2 + 5",
            "hko geometry reconstructed from definition",
            "action_min matched HKO2024 Proposition counterexample_prop in the exact field",
            "volume and volume derivative matched source formulas",
            "symmetry tangent generators reconstructed and rank checked",
            "witness entries are partial permutations with complementary minor and fixed beta indices",
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
    print("Verified HKO feasible-section certificate")
    print(f"Witness: {WITNESS_PATH.relative_to(PACKET_DIR)}")
    print(f"Rows: {summary['row_count']}")
    print(f"Row rank: {summary['row_rank']}")
    print(f"Symmetry rank: {summary['symmetry_rank']}")


if __name__ == "__main__":
    main()
