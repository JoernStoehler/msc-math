# Thesis Excerpts for `executable_proof.sage.py`

This file is a non-runnable thesis-facing companion to
`experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`.
It selects the implementation points that are useful to quote, paraphrase, or
check against the thesis explanation.

This file is not proof evidence by itself. The source truth is the runnable
Sage file; the run-output truth is `executable_proof.full.stdout.txt`. If the
Sage file changes, verify these excerpts against it before quoting them.

The selection criterion is reader-facing: each included excerpt should help Kai
check that the implementation matches the mathematical proof architecture. Do
not add internal development comments, planning notes, or source headers here.

## Reader Checkpoints

The useful bridge between thesis prose and Sage implementation is:

| Thesis claim | Sage surface to quote or check |
| --- | --- |
| The computation is exact on the open half-domain. | Coefficient field $K=\mathbb{Q}(\zeta_{20})^+$, rational-function field $K(t)$, and $t=\tan(\theta/2)$ parameterization. |
| The active branch gives the claimed formula. | `assert_formula_checks()`. |
| The finite candidate list is the intended 2- and 3-bounce raw-sigma list after transition pruning. | `blocks`, `enumerate_k_bounce_sigmas`, `transition_table_open`, `transition_pruned_sigmas_open`, and the count assertion. |
| Each branch is rejected by exact sign information, not sampling. | `sign_certificate`, `open_domain_cells`, and `classify_sigma`. |
| The script fails closed. | `ACCEPTED_STATUSES`, `requires_manual_review`, and the full loop assertion. |
| The recorded success output is only printed for the full run. | `run_certificate` and CLI `--limit` behavior. |

## Minimal Quote Set

If the thesis wants a compact implementation bridge instead of a long appendix,
quote or paraphrase these parts first:

| Priority | Current source lines | Why these lines matter |
| --- | --- | --- |
| 1 | `493-501` | `assert_formula_checks()` checks the active branch, displayed prefactor, and feasibility. |
| 2 | `209-275` | The exact sign method: roots cut the interval and one algebraic sample checks each cell. |
| 3 | `398-483` | `classify_sigma()` is the lower-bound branch classifier and shows when the script falls back to `requires_manual_review`. |
| 4 | `529-548` plus `56-63` | The full loop asserts every branch status is accepted, and `CERTIFICATE PASSED` is printed only for an unbounded run. |
| 5 | `52-54`, `289-326`, `356-382`, and `516-517` | Include these if the thesis needs to expose how the finite raw-sigma candidate list is produced and counted. |
| 6 | `34-47` and `72-83` | Include these if the thesis needs to show the exact field and tangent-half-angle parameterization explicitly. |

The line numbers above are navigation hints for the current source. Regenerate
line numbers from `executable_proof.sage.py` before final thesis quotation.

## 1. Exact Field and Half-Domain Parameter

For the exact finite computation, we first choose a coefficient field that
contains all pentagon constants, and then keep the rotation angle as a symbolic
parameter. Let

$$
\zeta_{20}=\exp(2\pi i/20)=\exp(\pi i/10),
$$

and set

$$
K=\mathbb{Q}(\zeta_{20})^+
$$

to be the maximal totally real subfield of the 20th cyclotomic field. This
field contains the fixed pentagon constants $\cos(m\pi/10)$ and
$\sin(m\pi/10)$ used for normals and support heights.

We introduce an indeterminate $t$, later specialized to $t=\tan(\theta/2)$.
The finite computation is carried out over the rational-function field

$$
F=K(t)=\operatorname{Frac}(K[t]).
$$

The tangent-half-angle substitution gives

$$
\cos\theta=\frac{1-t^2}{1+t^2},\qquad
\sin\theta=\frac{2t}{1+t^2}.
$$

Thus every KKT entry, beta, action, and action gap is a rational function in
$t$ with coefficients in $K$. The open half-domain $0<\theta<\pi/10$ becomes

$$
0<t<\tan(\pi/20).
$$

The script constructs `CyclotomicField(40)` only to get the algebraic endpoint
$\tan(\pi/20)$ as an algebraic real number for root isolation and sign checks; it
does not enlarge the rational-function coefficient field.

Implementation check:

The following source lines are not the mathematical definition. They are the
Sage realization of the field and parameter choices above.

```python
CYCLOTOMIC_20 = CyclotomicField(20)
ZETA_20 = CYCLOTOMIC_20.gen()
I_IN_CYCLOTOMIC_20 = ZETA_20**5
K, _ = CYCLOTOMIC_20.maximal_totally_real_subfield()

R = PolynomialRing(K, "t")
t = R.gen()
F = FractionField(R)

CYCLOTOMIC_40 = CyclotomicField(40)
ZETA_40 = CYCLOTOMIC_40.gen()
I_IN_CYCLOTOMIC_40 = ZETA_40**10
HALF_DOMAIN_ENDPOINT = AA(
    (ZETA_40 - ZETA_40**-1) / (I_IN_CYCLOTOMIC_40 * (ZETA_40 + ZETA_40**-1))
)
```

These source lines realize the trigonometric constants in `K` and the
tangent-half-angle identities in `F`:

```python
def lift(value):
    return F(value)


def reduced(expr):
    return F(expr)


def cos_pi_over_10_multiple(k):
    """Return cos(k*pi/10) in K = Q(zeta_20)^+."""
    return K((ZETA_20**k + ZETA_20**(-k)) / 2)


def sin_pi_over_10_multiple(k):
    """Return sin(k*pi/10) in K = Q(zeta_20)^+."""
    return K((ZETA_20**k - ZETA_20**(-k)) / (2 * I_IN_CYCLOTOMIC_20))


COS_THETA = lift(1 - t**2) / lift(1 + t**2)
SIN_THETA = lift(2 * t) / lift(1 + t**2)
```

## 2. Geometry, KKT System, and Active Branch

The script constructs the dual vertices of the two pentagons over the field
$F$, builds the KKT linear system for each candidate signature $\sigma$, and
computes the action from the resulting value of $Q_\sigma$. This is the code
surface to compare against the thesis description of the KKT calculation.

```python
def rotate(point):
    x, y = point
    return (COS_THETA * x - SIN_THETA * y, SIN_THETA * x + COS_THETA * y)


def pentagon_normals():
    # Same convention as analyze.py:
    # normal angle pi/2 + 2*pi*k/5 = (5 + 4k)*pi/10.
    return [
        (
            lift(cos_pi_over_10_multiple(5 + 4 * k)),
            lift(sin_pi_over_10_multiple(5 + 4 * k)),
        )
        for k in range(5)
    ]


def dual_vertices():
    height = lift(cos_pi_over_10_multiple(2))  # cos(pi/5)
    normals = pentagon_normals()
    vertices = []
    for x, y in normals:
        vertices.append((x / height, y / height, lift(0), lift(0)))
    for normal in normals:
        x, y = rotate(normal)
        vertices.append((lift(0), lift(0), x / height, y / height))
    return vertices


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
    amplitude = lift(1 + cos_pi_over_10_multiple(2))
    return reduced(amplitude**2 / COS_THETA)


def systolic_ratio_prefactor():
    amplitude = lift(1 + cos_pi_over_10_multiple(2))
    area = lift(5) * lift(sin_pi_over_10_multiple(4)) / 2  # (5/2) sin(2*pi/5)
    return reduced(amplitude**4 / (2 * area**2))
```

The preflight assertions make the formula claim executable: the chosen branch
has the active action, the systolic-ratio prefactor is the displayed constant,
and the branch is feasible on the open half-domain.

```python
def assert_formula_checks():
    intended_sigma = (3, 8, 1, 0, 5, 6)
    beta, q, action = solve_kkt_branch(intended_sigma)
    assert action == minimum_action()
    sqrt5 = 4 * cos_pi_over_10_multiple(2) - 1
    expected_prefactor = reduced((5 + 2 * sqrt5) / 10)
    assert systolic_ratio_prefactor() == expected_prefactor
    assert all(sign_certificate(beta_i).positive_on_open() for beta_i in beta)
    assert sign_certificate(q).positive_on_open()
```

## 3. Exact Sign Certificates

For a rational function in $F=K(t)$, the script isolates the real roots of the
numerator and denominator inside the open half-domain. These roots cut the
interval into cells. Since the sign is constant on each cell, one algebraic
sample point per cell is enough to certify positivity, negativity, or a mixed
sign pattern.

```python
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
```

## 4. Candidate Enumeration and Transition Pruning

The raw finite list is generated from single facets and ordered adjacent-pair
blocks in each factor. These blocks are interleaved to form raw 2- and
3-bounce signatures. The transition table then removes signatures whose
successive facets are not transition-feasible on the open half-domain.

```python
Q_FACETS = tuple(range(5))
P_FACETS = tuple(range(5, 10))
EXPECTED_OPEN_SIGMA_COUNT = 3340


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
```

The count assertion is the executable check that this open-domain candidate
list has the recorded size.

```python
sigmas = transition_pruned_sigmas_open()
assert len(sigmas) == EXPECTED_OPEN_SIGMA_COUNT
```

## 5. Branch Classification and Fail-Closed Logic

For each remaining signature, the script solves the KKT system and compares the
resulting action with the active branch. A signature is accepted only if it has
no KKT solution, is algebraically degenerate in one of the listed harmless ways,
is not feasible on the open domain, or has strictly positive action gap on
every feasible cell. If a feasible cell has non-positive gap, or a singular
case is not explained by the accepted statuses, classification falls back to
`requires_manual_review`.

```python
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
```

The full loop is the key fail-closed assertion: every classified sigma must
land in the accepted set. In particular, `requires_manual_review` is not in
`ACCEPTED_STATUSES`.

```python
ACCEPTED_STATUSES = {
    "no_kkt_solution",
    "singular_kkt_forced_zero_beta",
    "zero_q_identity",
    "zero_gap_identity",
    "not_feasible_on_open_domain",
    "strict_gap_positive_on_feasible_open_domain",
}


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
```

## 6. Full-Run Output to Pair With Excerpts

When quoting any code excerpt as part of a proof packet, pair it with the
recorded full-run result in `executable_proof.full.stdout.txt`:

```text
open_domain_raw_sigma_count = 3340
classified_raw_sigma_count = 3340
classification_statuses = {'no_kkt_solution': 25, 'zero_q_identity': 1680, 'singular_kkt_forced_zero_beta': 470, 'not_feasible_on_open_domain': 735, 'zero_gap_identity': 20, 'strict_gap_positive_on_feasible_open_domain': 410}
CERTIFICATE PASSED in 2010.05s
```

The stdout file, not this Markdown file, is the source for exact counts and
runtime.
