#!/usr/bin/env sage -python
"""
Executable proof for the pentagon rotation formula.

This script is intended to become the readable SageMath proof certificate for
the formula

    sys(P_5 x_L R(theta) P_5)
      = ((5 + 2 sqrt(5)) / 10) sec^2(theta)

on the half-domain 0 <= theta <= pi/10. The second half of the fundamental
domain is then obtained by symmetry.

Current development contract:
    - Default invocation runs the full all-raw-sigma certificate.
    - `--limit N` runs the same assertions on only the first N raw sigmas.
    - `CERTIFICATE PASSED` must only be printed when no limit is used.

The source code is the proof surface. Assertions encode proof obligations; the
default output is a compact summary and formulas of interest.
"""

import argparse
import time
from dataclasses import dataclass
from itertools import combinations, permutations

from sage.all import AA, CyclotomicField, FractionField, Matrix, PolynomialRing, vector


# ---------------------------------------------------------------------------
# Exact field and parameter setup
# ---------------------------------------------------------------------------

C20 = CyclotomicField(20)
z20 = C20.gen()
I20 = z20**5
K, _ = C20.maximal_totally_real_subfield()

R = PolynomialRing(K, "t")
t = R.gen()
F = FractionField(R)

C40 = CyclotomicField(40)
z40 = C40.gen()
I40 = z40**10
HALF_DOMAIN_ENDPOINT = AA((z40 - z40**-1) / (I40 * (z40 + z40**-1)))

Q_FACETS = tuple(range(5))
P_FACETS = tuple(range(5, 10))
EXPECTED_OPEN_SIGMA_COUNT = 3340
DEFAULT_PROGRESS_EVERY = 500
ACCEPTED_STATUSES = {
    "no_kkt_solution",
    "singular_kkt_forced_zero_beta",
    "zero_q_identity",
    "zero_gap_identity",
    "not_feasible_on_open_domain",
    "strict_gap_positive_on_feasible_open_domain",
}


def lift(value):
    return F(value)


def cos_units(units):
    """cos(units*pi/10) in the pentagon coefficient field."""
    return K((z20**units + z20**(-units)) / 2)


def sin_units(units):
    """sin(units*pi/10) in the pentagon coefficient field."""
    return K((z20**units - z20**(-units)) / (2 * I20))


def cos_theta():
    return lift(1 - t**2) / lift(1 + t**2)


def sin_theta():
    return lift(2 * t) / lift(1 + t**2)


def reduced(expr):
    return F(expr)


# ---------------------------------------------------------------------------
# Pentagon geometry and KKT system
# ---------------------------------------------------------------------------


def rotate(point):
    x, y = point
    c = cos_theta()
    s = sin_theta()
    return (c * x - s * y, s * x + c * y)


def pentagon_normals():
    # Same convention as analyze.py:
    # normal angle pi/2 + 2*pi*k/5 = (5 + 4k)*pi/10.
    return [
        (lift(cos_units(5 + 4 * k)), lift(sin_units(5 + 4 * k)))
        for k in range(5)
    ]


def dual_vertices():
    height = lift(cos_units(2))  # cos(pi/5)
    normals = pentagon_normals()
    vertices = []
    for x, y in normals:
        vertices.append((x / height, y / height, lift(0), lift(0)))
    for normal in normals:
        x, y = rotate(normal)
        vertices.append((lift(0), lift(0), x / height, y / height))
    return vertices


DUALS = dual_vertices()


def omega(u, v):
    return u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]


def kkt_matrix(sigma):
    m = len(sigma)
    size = m + 5
    mat = Matrix(F, size, size, 0)
    rhs = vector(F, [0] * size)

    for i in range(m):
        for j in range(i + 1, m):
            value = omega(DUALS[sigma[i]], DUALS[sigma[j]])
            mat[i, j] = value
            mat[j, i] = value

    for i in range(m):
        for d in range(4):
            value = DUALS[sigma[i]][d]
            mat[i, m + d] = value
            mat[m + d, i] = value

    for i in range(m):
        mat[i, m + 4] = 1
        mat[m + 4, i] = 1
    rhs[m + 4] = 1
    return mat, rhs


def q_value(sigma, beta):
    q = lift(0)
    for i in range(1, len(sigma)):
        for j in range(i):
            q += beta[i] * beta[j] * omega(DUALS[sigma[j]], DUALS[sigma[i]])
    return reduced(q)


def solve_kkt_branch(sigma):
    mat, rhs = kkt_matrix(sigma)
    solution = mat.solve_right(rhs)
    beta = [reduced(solution[i]) for i in range(len(sigma))]
    q = q_value(sigma, beta)
    if q == 0:
        return beta, q, None
    action = reduced(lift(1) / (2 * q))
    return beta, q, action


def minimum_action():
    amplitude = lift(1 + cos_units(2))
    return reduced(amplitude**2 / cos_theta())


def systolic_ratio_prefactor():
    amplitude = lift(1 + cos_units(2))
    area = lift(5) * lift(sin_units(4)) / 2  # (5/2) sin(2*pi/5)
    return reduced(amplitude**4 / (2 * area**2))


def formula_summary():
    return {
        "minimum_action": "((1 + cos(pi/5))^2) / cos(theta)",
        "systolic_ratio_half_domain": "((5 + 2*sqrt(5)) / 10) * sec(theta)^2",
        "theta_domain": "0 <= theta <= pi/10; open proof uses 0 < theta < pi/10",
    }


# ---------------------------------------------------------------------------
# Exact sign certificates
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class SignCertificate:
    status: str
    interior_cut_count: int
    endpoint: str

    def positive_on_open(self):
        return self.status == "positive_open"


def aa_polynomial(poly):
    return poly.change_ring(AA)


def real_roots_in_open_half_domain(poly):
    roots = []
    for root, _multiplicity in aa_polynomial(poly).roots():
        root = AA(root)
        if AA(0) < root < HALF_DOMAIN_ENDPOINT:
            roots.append(root)
    return sorted(set(roots))


def endpoint_label(num_aa, den_aa):
    den_value = den_aa(HALF_DOMAIN_ENDPOINT)
    if den_value == 0:
        return "pole"
    sign = (num_aa(HALF_DOMAIN_ENDPOINT) / den_value).sign()
    if sign > 0:
        return "positive"
    if sign < 0:
        return "negative"
    return "zero"


def sign_certificate(expr):
    expr = reduced(expr)
    numerator = expr.numerator()
    denominator = expr.denominator()
    if numerator == 0:
        return SignCertificate("zero", 0, "zero")

    cuts = sorted(
        set(
            real_roots_in_open_half_domain(numerator)
            + real_roots_in_open_half_domain(denominator)
        )
    )
    points = [AA(0)] + cuts + [HALF_DOMAIN_ENDPOINT]
    num_aa = aa_polynomial(numerator)
    den_aa = aa_polynomial(denominator)
    signs = []
    for left, right in zip(points, points[1:]):
        sample = (left + right) / 2
        signs.append((num_aa(sample) / den_aa(sample)).sign())

    endpoint = endpoint_label(num_aa, den_aa)
    if all(sign > 0 for sign in signs):
        return SignCertificate("positive_open", len(cuts), endpoint)
    if all(sign < 0 for sign in signs):
        return SignCertificate("negative_open", len(cuts), endpoint)
    return SignCertificate("mixed_open", len(cuts), endpoint)


def open_domain_cells(expressions):
    cuts = []
    for expr in expressions:
        expr = reduced(expr)
        cuts.extend(real_roots_in_open_half_domain(expr.numerator()))
        cuts.extend(real_roots_in_open_half_domain(expr.denominator()))
    points = [AA(0)] + sorted(set(cuts)) + [HALF_DOMAIN_ENDPOINT]
    return [(left + right) / 2 for left, right in zip(points, points[1:])]


def sign_at(expr, sample):
    expr = reduced(expr)
    num_aa = aa_polynomial(expr.numerator())
    den_aa = aa_polynomial(expr.denominator())
    denominator = den_aa(sample)
    assert denominator != 0
    return (num_aa(sample) / denominator).sign()


# ---------------------------------------------------------------------------
# Sigma enumeration and transition-sign constancy
# ---------------------------------------------------------------------------


def adjacent_same_factor(a, b):
    return (a - b) % 5 in {1, 4}


def blocks(facets):
    result = [(i,) for i in facets]
    for i, j in combinations(facets, 2):
        if adjacent_same_factor(i % 5, j % 5):
            result.append((i, j))
            result.append((j, i))
    return result


def non_overlapping(selection):
    used = set()
    for block in selection:
        if used & set(block):
            return False
        used.update(block)
    return True


def non_overlapping_selections(block_list, k):
    for selection in combinations(block_list, k):
        if non_overlapping(selection):
            yield selection


def enumerate_k_bounce_sigmas(k):
    q_blocks = blocks(Q_FACETS)
    p_blocks = blocks(P_FACETS)
    for q_selection in non_overlapping_selections(q_blocks, k):
        for p_selection in non_overlapping_selections(p_blocks, k):
            for q_rest_perm in permutations(range(k - 1)):
                for p_perm in permutations(range(k)):
                    sigma = []
                    sigma.extend(q_selection[0])
                    sigma.extend(p_selection[p_perm[0]])
                    for round_index in range(1, k):
                        sigma.extend(q_selection[1 + q_rest_perm[round_index - 1]])
                        sigma.extend(p_selection[p_perm[round_index]])
                    yield tuple(sigma)


def facet_intersection_nonempty(i, j):
    if i == j:
        return True
    if (i < 5) != (j < 5):
        return True
    return adjacent_same_factor(i % 5, j % 5)


def assert_facet_conventions():
    height = lift(cos_units(2))
    assert DUALS[0] == (lift(0), lift(1) / height, lift(0), lift(0))
    assert DUALS[5] == (
        lift(0),
        lift(0),
        -sin_theta() / height,
        cos_theta() / height,
    )
    assert facet_intersection_nonempty(0, 1)
    assert facet_intersection_nonempty(0, 4)
    assert not facet_intersection_nonempty(0, 2)
    assert not facet_intersection_nonempty(0, 3)


def mixed_omega(i, j):
    return reduced(omega(DUALS[i], DUALS[j]))


def transition_table_open():
    table = {}
    for i in range(10):
        for j in range(10):
            omega_ij = mixed_omega(i, j)
            cert = sign_certificate(omega_ij)
            if (i < 5 and j < 5) or (i >= 5 and j >= 5):
                assert cert.status == "zero", (i, j, cert)
                table[(i, j)] = facet_intersection_nonempty(i, j)
            else:
                assert cert.status in {"positive_open", "negative_open"}, (i, j, cert)
                assert cert.interior_cut_count == 0, (i, j, cert)
                table[(i, j)] = (
                    facet_intersection_nonempty(i, j)
                    and cert.status == "positive_open"
                )
    return table


def transition_pruned_sigmas_open():
    table = transition_table_open()
    sigmas = []
    for k in (2, 3):
        for sigma in enumerate_k_bounce_sigmas(k):
            if all(table[(i, j)] for i, j in zip(sigma, sigma[1:] + sigma[:1])):
                sigmas.append(sigma)
    return sigmas


# ---------------------------------------------------------------------------
# Branch classification
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class BranchClassification:
    sigma: tuple
    status: str
    beta_statuses: tuple = ()
    q_status: SignCertificate | None = None
    gap_status: SignCertificate | None = None
    feasible_cell_count: int = 0


def classify_sigma(sigma, min_action):
    mat, rhs = kkt_matrix(sigma)
    try:
        solution = mat.solve_right(rhs)
    except ValueError:
        augmented = mat.augment(rhs.column())
        if augmented.rank() > mat.rank():
            return BranchClassification(sigma, "no_kkt_solution")
        return BranchClassification(sigma, "requires_manual_review")

    beta = [reduced(solution[i]) for i in range(len(sigma))]
    q = q_value(sigma, beta)
    kernel = None

    if q == 0:
        kernel = mat.right_kernel().basis()
        if kernel:
            for i in range(len(sigma)):
                if beta[i] == 0 and all(reduced(vec[i]) == 0 for vec in kernel):
                    return BranchClassification(sigma, "singular_kkt_forced_zero_beta")
            return BranchClassification(sigma, "requires_manual_review")
        return BranchClassification(sigma, "zero_q_identity")
    action = reduced(lift(1) / (2 * q))
    gap = reduced(action - min_action)
    if gap == 0:
        return BranchClassification(sigma, "zero_gap_identity")

    beta_statuses = tuple(sign_certificate(beta_i) for beta_i in beta)
    q_status = sign_certificate(q)
    gap_status = sign_certificate(gap)

    feasible_cell_count = 0
    for sample in open_domain_cells(tuple(beta) + (q, gap)):
        feasible = all(sign_at(beta_i, sample) > 0 for beta_i in beta)
        feasible = feasible and sign_at(q, sample) > 0
        if not feasible:
            continue
        feasible_cell_count += 1
        if sign_at(gap, sample) <= 0:
            return BranchClassification(
                sigma,
                "requires_manual_review",
                beta_statuses,
                q_status,
                gap_status,
                feasible_cell_count,
            )

    if feasible_cell_count == 0:
        if kernel is None:
            kernel = mat.right_kernel().basis()
        if kernel:
            for i in range(len(sigma)):
                if beta[i] == 0 and all(reduced(vec[i]) == 0 for vec in kernel):
                    return BranchClassification(
                        sigma,
                        "singular_kkt_forced_zero_beta",
                        beta_statuses,
                        q_status,
                        gap_status,
                        feasible_cell_count,
                    )
            return BranchClassification(
                sigma,
                "requires_manual_review",
                beta_statuses,
                q_status,
                gap_status,
                feasible_cell_count,
            )
        return BranchClassification(
            sigma,
            "not_feasible_on_open_domain",
            beta_statuses,
            q_status,
            gap_status,
            feasible_cell_count,
        )
    return BranchClassification(
        sigma,
        "strict_gap_positive_on_feasible_open_domain",
        beta_statuses,
        q_status,
        gap_status,
        feasible_cell_count,
    )


# ---------------------------------------------------------------------------
# Preflight and certificate run
# ---------------------------------------------------------------------------


def assert_formula_checks():
    intended_sigma = (3, 8, 1, 0, 5, 6)
    beta, q, action = solve_kkt_branch(intended_sigma)
    assert action == minimum_action()
    sqrt5 = 4 * cos_units(2) - 1
    expected_prefactor = reduced((5 + 2 * sqrt5) / 10)
    assert systolic_ratio_prefactor() == expected_prefactor
    assert all(sign_certificate(beta_i).positive_on_open() for beta_i in beta)
    assert sign_certificate(q).positive_on_open()
    print(f"minimum_action(t) = {minimum_action()}")
    print(f"sys_prefactor = {systolic_ratio_prefactor()}")


def assert_status(statuses, sigma, expected_status):
    min_action = minimum_action()
    classification = classify_sigma(sigma, min_action)
    statuses[classification.status] = statuses.get(classification.status, 0) + 1
    assert classification.status == expected_status, classification


def run_preflight():
    assert_facet_conventions()
    assert_formula_checks()
    sigmas = transition_pruned_sigmas_open()
    assert len(sigmas) == EXPECTED_OPEN_SIGMA_COUNT

    statuses = {}
    assert_status(statuses, (0, 5, 3, 8, 1, 7), "strict_gap_positive_on_feasible_open_domain")
    assert_status(statuses, (0, 5, 2, 3, 7, 8), "zero_gap_identity")
    assert_status(statuses, (0, 5, 9, 1, 7), "zero_q_identity")
    assert_status(statuses, (0, 9, 1, 7), "no_kkt_solution")
    assert_status(statuses, (0, 5, 9, 1, 6, 7), "singular_kkt_forced_zero_beta")
    print(f"preflight_statuses = {statuses}")
    return sigmas


def run_certificate(progress_every, limit=None):
    start = time.perf_counter()
    all_sigmas = run_preflight()
    min_action = minimum_action()
    sigmas = all_sigmas
    if limit is not None:
        sigmas = all_sigmas[:limit]
    statuses = {}
    for index, sigma in enumerate(sigmas):
        classification = classify_sigma(sigma, min_action)
        statuses[classification.status] = statuses.get(classification.status, 0) + 1
        assert classification.status in ACCEPTED_STATUSES, classification
        if progress_every and index % progress_every == 0:
            print(f"{index:04d}/{len(sigmas)} {classification.status} {sigma}", flush=True)
    elapsed = time.perf_counter() - start
    print(f"open_domain_raw_sigma_count = {len(all_sigmas)}")
    print(f"classified_raw_sigma_count = {len(sigmas)}")
    print(f"classification_statuses = {statuses}")
    if limit is None:
        print(f"CERTIFICATE PASSED in {elapsed:.2f}s")
    else:
        print(f"LIMITED PREFIX PASSED in {elapsed:.2f}s")


def main():
    parser = argparse.ArgumentParser(
        description=(
            "Exact SageMath executable proof for the pentagon-rotation formula. "
            "Default mode is the full certificate. Use --limit N for a prefix run. "
            "Only an unbounded run can print CERTIFICATE PASSED."
        )
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Restrict classification to the first N raw sigmas; disables certificate output.",
    )
    parser.add_argument(
        "--progress-every",
        type=int,
        default=DEFAULT_PROGRESS_EVERY,
        help="Print one progress line per N sigmas.",
    )
    args = parser.parse_args()

    for key, value in formula_summary().items():
        print(f"{key} = {value}")

    run_certificate(args.progress_every, args.limit)


if __name__ == "__main__":
    main()
