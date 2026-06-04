#!/usr/bin/env sage -python
"""
Construct exact witness values for the HKO feasible-section certificate.

This script is allowed to solve exact systems. It is a constructor, not the
proof-facing verifier. The companion verifier reloads the emitted witness and
asserts the exact equations.
"""

from __future__ import annotations

import json
from fractions import Fraction
from pathlib import Path

from sage.all import AA, QQ, RIF, NumberField, PolynomialRing, matrix, vector, sqrt, RR


PACKET_DIR = Path(__file__).resolve().parent
CANDIDATE_PATH = PACKET_DIR / "candidate-certificate.json"
OUTPUT_PATH = PACKET_DIR / "feasible-section-witness.json"


def q_json(q):
    q = QQ(q)
    return {"num": int(q.numerator()), "den": int(q.denominator())}


def coeff_vector_json(x, degree):
    coeffs = list(x.list())
    coeffs += [QQ(0)] * (degree - len(coeffs))
    return [q_json(c) for c in coeffs]


def vector_json(v, degree):
    return [coeff_vector_json(entry, degree) for entry in v]


def matrix_rows_json(rows, degree):
    return [vector_json(row, degree) for row in rows]


def exact_hko_geometry():
    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    polynomial = x**4 - 10 * x**2 + 5
    K = NumberField(polynomial, "t", embedding=AA.polynomial_root(polynomial, RIF(0, 1)))
    t = K.gen()
    degree = K.degree()
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
    return K, degree, t, sqrt5, duals


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
    return volume, volume_row


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


def find_free_values(K, duals, row, beta_poly, S, variables, q_min):
    q_poly = S(q_value(duals, row["sigma"], beta_poly) - q_min)
    fixed_hints = row["fixed_beta_values_f64"]
    nfree = len(variables)

    if q_poly == 0:
        for free_values in zip(*(rational_candidates(hint) for hint in fixed_hints)):
            beta = substitute_beta(K, beta_poly, variables, free_values)
            if beta_is_valid(K, duals, row["sigma"], beta, q_min):
                return [K(value) for value in free_values], beta
        raise RuntimeError(f"row {row['certificate_index']}: q polynomial is zero but no positive rational free beta was found")

    if nfree == 1:
        univariate = univariate_from_multivariate(K, S, q_poly, 0)
        if univariate is None:
            raise RuntimeError(f"row {row['certificate_index']}: one-free-variable polynomial was not univariate")
        roots = [root for root, _multiplicity in univariate.roots()]
        roots.sort(key=lambda root: abs(real_float(root) - fixed_hints[0]))
        for root in roots:
            beta = substitute_beta(K, beta_poly, variables, [root])
            if beta_is_valid(K, duals, row["sigma"], beta, q_min):
                return [root], beta
        raise RuntimeError(f"row {row['certificate_index']}: no valid root among {len(roots)} one-free-variable roots")

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
                    if beta_is_valid(K, duals, row["sigma"], beta, q_min):
                        return free_values, beta
        raise RuntimeError(f"row {row['certificate_index']}: no valid two-free-variable exact beta found")

    raise RuntimeError(f"row {row['certificate_index']}: unsupported free beta count {nfree}")


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
        assert all(entry == K(0) for entry in rows[beta_idx])
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
    return row


def construct_row(K, duals, volume, volume_row, action_min, q_min, row):
    sigma = row["sigma"]
    minor_columns = row["minor_columns_exact"]
    fixed_indices = row["fixed_beta_indices"]
    S, variables, beta_poly = beta_polynomials(K, duals, sigma, minor_columns, fixed_indices)
    free_values, beta = find_free_values(K, duals, row, beta_poly, S, variables, q_min)
    d_beta = d_beta_matrix(K, duals, sigma, beta, minor_columns, fixed_indices)
    q = q_value(duals, sigma, beta)
    d_q = d_q_row(K, duals, sigma, beta, d_beta)
    d_action = [-(entry) / (2 * q**2) for entry in d_q]
    d_sys = [
        action_min / volume * d_action[idx] - action_min**2 / (2 * volume**2) * volume_row[idx]
        for idx in range(len(volume_row))
    ]
    action = 1 / (2 * q)

    assert q == q_min
    assert action == action_min
    return {
        "certificate_index": row["certificate_index"],
        "source_feasible_section_row_index": row["source_feasible_section_row_index"],
        "sigma": sigma,
        "minor_columns": minor_columns,
        "fixed_beta_indices": fixed_indices,
        "free_beta_values_power_basis": vector_json(vector(K, free_values), K.degree()),
        "beta0_power_basis": vector_json(beta, K.degree()),
        "q_power_basis": coeff_vector_json(q, K.degree()),
        "action_power_basis": coeff_vector_json(action, K.degree()),
        "d_beta_power_basis": matrix_rows_json([vector(K, row_entries) for row_entries in d_beta], K.degree()),
        "d_action_flat_power_basis": vector_json(vector(K, d_action), K.degree()),
        "d_sys_flat_power_basis": vector_json(vector(K, d_sys), K.degree()),
        "constructor_diagnostics": {
            "min_beta_f64": min(real_float(entry) for entry in beta),
            "max_abs_d_sys_delta_vs_f64_hint": max(
                abs(real_float(entry) - hint)
                for entry, hint in zip(d_sys, row["d_sys_flat_f64"], strict=True)
            ),
        },
    }


def construct_convex_certificate(K, rows):
    row_matrix = matrix(K, [row["d_sys_exact"] for row in rows])
    rank = row_matrix.rank()
    if rank != 25:
        raise RuntimeError(f"selected exact rows have rank {rank}, expected 25")

    kernel = row_matrix.transpose().right_kernel().basis()
    if len(kernel) != 1:
        raise RuntimeError(f"left kernel dimension is {len(kernel)}, expected 1")
    lambdas = list(kernel[0])
    if all(entry < K(0) for entry in lambdas):
        lambdas = [-entry for entry in lambdas]
    if not all(entry > K(0) for entry in lambdas):
        raise RuntimeError("left-kernel vector is not strictly positive after sign choice")
    total = sum(lambdas, K(0))
    lambdas = [entry / total for entry in lambdas]
    residual = vector(K, lambdas) * row_matrix
    assert all(entry == K(0) for entry in residual)
    return rank, lambdas


def main():
    candidate = json.loads(CANDIDATE_PATH.read_text())
    K, degree, t, sqrt5, duals = exact_hko_geometry()
    volume, volume_row = volume_data(K, sqrt5, duals)
    action_min = 5 * t - t**3 / 2
    q_min = 1 / (2 * action_min)
    labels, symmetry_columns, symmetry_matrix, generator_checks = symmetry_basis(K, duals)

    constructed_rows = []
    rank_rows = []
    for row in candidate["rows"]:
        constructed = construct_row(K, duals, volume, volume_row, action_min, q_min, row)
        exact_row = [
            sum(QQ(coeff["num"]) / QQ(coeff["den"]) * t**idx for idx, coeff in enumerate(entry))
            for entry in constructed["d_sys_flat_power_basis"]
        ]
        constructed["d_sys_exact"] = exact_row
        rank_rows.append(exact_row)
        constructed_rows.append(constructed)

    rank, lambdas = construct_convex_certificate(K, constructed_rows)
    for constructed in constructed_rows:
        del constructed["d_sys_exact"]

    payload = {
        "packet": "hko-feasible-section-certificate",
        "witness_version": 1,
        "source_candidate_path": str(CANDIDATE_PATH.relative_to(PACKET_DIR)),
        "field": {
            "generator_name": "t",
            "degree": int(degree),
            "minimal_polynomial_coefficients_desc": [q_json(c) for c in K.defining_polynomial().list()[::-1]],
        },
        "geometry": {
            "facet_order": list(range(len(duals))),
            "dual_vertices_power_basis": matrix_rows_json(duals, degree),
        },
        "constants": {
            "q_min_power_basis": coeff_vector_json(q_min, degree),
            "action_min_power_basis": coeff_vector_json(action_min, degree),
            "volume_power_basis": coeff_vector_json(volume, degree),
            "volume_derivative_flat_power_basis": vector_json(vector(K, volume_row), degree),
        },
        "symmetry": {
            "labels": labels,
            "columns_power_basis": matrix_rows_json(symmetry_columns, degree),
            "rank": int(symmetry_matrix.rank()),
            "generator_sp4_checks": generator_checks,
        },
        "rows": constructed_rows,
        "convex_certificate": {
            "rank": int(rank),
            "lambda_power_basis": vector_json(vector(K, lambdas), degree),
            "min_lambda_f64": min(real_float(entry) for entry in lambdas),
            "max_lambda_f64": max(real_float(entry) for entry in lambdas),
        },
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(PACKET_DIR)}")
    print(f"Exact row rank: {rank}")
    print(f"Min lambda f64: {payload['convex_certificate']['min_lambda_f64']}")


if __name__ == "__main__":
    main()
