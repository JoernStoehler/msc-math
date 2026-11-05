# An explicit rational counterexample without affine product structure

16 October 2026. Constructive corollary of the HKO value and elementary
capacity monotonicity, with an exact finite geometry certificate. The openness
of the counterexample condition and smooth approximations are already in
[Haim–Kislev–Ostrover](https://arxiv.org/abs/2405.16513), including the remark
after their Question 3; no novelty is claimed for that fact. The contribution
here is a concrete, reproducible rational fixture and a quantitative bound that
also proves loss of **every nontrivial affine Cartesian product decomposition**.

Let `sys(K)=c_EHZ(K)^2/(2 vol_4(K))`. The retained [fixture](fixture.json)
defines `K={x:a_i·x≤1}`, in `(q1,q2,p1,p2)` order, by ten integer row
numerators with common denominator `2^20`. Its forty coordinates are exact
dyadic rationals, hence also exactly representable in binary64.

**Proposition.** This K is a bounded simple four-polytope with ten genuine
facets and 25 vertices. Any four distinct facet normals are linearly independent.
It is not the affine image of a Cartesian product of two positive-dimensional
convex bodies. Nevertheless

```
sys(K) > 1.0463761596 > 1.
```

This does not improve HKO's ratio `(3+sqrt(5))/5`; it answers a different
necessity question. Exact regular factors, a linear product splitting, and
dependent same-factor normals are unnecessary for a violation. It says nothing
about global maximizers, and does not rule out a nonlinear symplectic equivalence.

## Quantitative inclusion argument

The following elementary lemma applies to any convex bodies containing zero
in their interiors. If `alpha H ⊆ K ⊆ beta H` for positive `alpha,beta`,
capacity monotonicity, degree-two conformality and degree-four volume scaling give

```
sys(K) ≥ (alpha/beta)^4 sys(H).                         (1)
```

For this fixture, take `H=P×R(-pi/2)P`, with P a regular circumradius-one
pentagon whose facet unit normals are `n_i=(cos(2pi i/5),sin(2pi i/5))`.
This P is a common planar rotation of the paper's vertex convention; applying
the same rotation in both factors is symplectic, so the HKO value applies.
Put `h=cos(pi/5)`. The ten dual rows of H are

```
b_i=(n_i/h,0),   b_(i+5)=(0,R(-pi/2)n_i/h),   i=0,...,4.
```

Both factors have circumradius one, so `H⊆[-1,1]^4`. The exact interval
calculation proves, with `e=1/10000`,

```
max_i ||a_i-b_i||_1 < e.                              (2)
```

Write `g_H(x)=max_i b_i·x`, the positive gauge of H. Its sublevel set is H,
so `||x||_infinity≤g_H(x)`. For `x∈H`, (2) gives `a_i·x≤1+e`, hence
`(1+e)^(-1)H⊆K`. Conversely, if `x∈K`, then

```
g_H(x) ≤ max_i a_i·x + e||x||_infinity ≤ 1+e g_H(x),
```

which implies `K⊆(1-e)^(-1)H` (and proves boundedness). Thus (1) gives

```
sys(K) ≥ [(1-e)/(1+e)]^4 (3+sqrt(5))/5
       > 1.0463761596.
```

The final decimal is strictly below the rational lower bound in
[certificate.json](certificate.json); it is a display of a proved inequality,
not a floating-point capacity estimate.

## Why the fixture is not an affine product

The exact verifier enumerates all `binom(10,4)=210` four-row subsets, checks
rank four over Q, solves all four supporting-plane equations, and retains
exactly the feasible intersections. It finds 25 distinct vertices, each incident
to exactly four facets. For each of the ten rows, its incident vertices span
a three-dimensional affine space (ten vertices per facet). Thus these rows
are genuine, irredundant facets. A rational interior point is zero; the inclusion
argument gives boundedness and full dimension. These checks establish the
polytope claim, rather than assuming stability of a numerically drawn body.

Suppose K were an invertible affine image of `A×B`, with dimensions `d` and
`4-d`, `1≤d≤3`. The factors must be polytopes: each is an affine image of
a slice, or a projection, of this bounded polytope. Facet normals of their
product partition into subspaces of dimensions d and `4-d`; inverse transpose
of the affine map preserves this property. A bounded d-polytope has at least
d+1 facets. Consequently one proper subspace contains d+1 of K's facet
normals, a linearly dependent subset of size at most four. Extending that
subset to four normals contradicts the verified general-position property.
Translations affect support heights and never normal directions. This excludes
all affine product splittings, including `1+3` as well as `2+2`.

## Reproduction and arithmetic contract

From the repository root:

```sh
python3 topics/variation-and-local-maxima/hko-research-20261016/generic-counterexample/verify.py \
  --output-dir /tmp/msc-math/hko-20261016/generic-rerun
cargo run --release --locked -p exp-hko-local-maximum --bin hko-generic-counterexample -- \
  /tmp/msc-math/hko-20261016/generic-rerun/fixture.json \
  /tmp/msc-math/hko-20261016/generic-rerun/capacity.json
```

The Python verifier uses only the standard library. All predicates use
`fractions.Fraction`; interval square roots use integer square root and outward
rational rounding at denominator `2^70`. Trigonometric coordinates are replaced
by the standard radical identities for fifth roots of unity. A seeded integer
perturbation makes the fixture deterministic; the seed is not an assumption of
the proof. Every finite predicate raises explicitly on failure, including under
`python -O`. Outputs go only to the requested directory. The retained JSON is
small deliberate evidence, not an absent external dataset.

The Rust binary separately calls the repository's production
`capacity_from_dual_vertices` and exact binary64 geometry and incidence-volume
helpers. Its capacity interval concerns the **dyadic fixture**, not the
algebraic HKO source. The analytic bound above does not depend on this second
route. Production implementation guarantees and limitations remain those of
[capacity and QP](../../../capacity-and-qp/README.md).

The executed [production result](capacity.json) encloses the fixture's ratio by

```
1.047191884343 ≤ sys(K) ≤ 1.047191884346.
```

Both capacity endpoints are outward bounds from the general dispatcher. The
volume is an exact rational from the exact incidence geometry; the binary64
capacity endpoints are converted to exact rationals before squaring and dividing
by twice that volume. The displayed decimal endpoints are then rounded outward
at `10^-12`. Thus the extra precision here does not rely on a floating-point
volume. The result is certified within the production implementation contract;
the independent inclusion proof already proves the strict violation without
assuming implementation correspondence on this input.

[Independent mathematical and execution review](review.md) records the exact
finite replay and the second determinant/vertex implementation.
