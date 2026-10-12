# Fixed-pentagon extrema force active-fitting degeneracy or a facet bound

Derived 2026-10-04 by extremality_discovery_lead. Agent proof supplied; independent
review requested. No external novelty claim. This is a necessary local extremality
mechanism, not the global HKO inequality.

## Statement

Fix the regular pentagon Q and its ten unit normal triangles T_j. Let K(h) be
an N-sided planar polygon in an open chamber of fixed distinct outward normals
n_i, every inequality genuine, with 0 in its interior. At a local maximum of

    c_EHZ(K(h) x Q)^2 / (2 area(K(h)) area(Q)),

the complete active triangle-fitting LP dual vectors must linearly span

    ker [n_1 ... n_N],       dimension N−2.                 (R)

Consequently there are at least N−2 linearly independent active fitting duals.
If each of the ten templates has a unique optimal dual vector at K, then N≤12.
Thus an extremum with more than twelve facets requires genuine fitting-dual
nonuniqueness; assigning one arbitrary solver dual to each triangle misses the
very degeneracy that such an extremum must have.

This holds for any local maximum with respect to arbitrary nearby planar
partners as a necessary consequence of its fixed-normal subchart. It does not
assert that full rank is sufficient or that every N≤12 candidate is extremal.

## Complete finite linear envelope

Put k_ij=max_{v in T_j} n_i.v. The maximal fitting scale r_j(h) is the LP

    maximize r subject to n_i.t+k_ij r≤h_i for all i.

The unconstrained scalar r may be used: the true maximum is positive, and the
bounded polygon makes it finite. Its dual is

    minimize lambda.h,
    lambda≥0, sum_i lambda_i n_i=0, sum_i lambda_i k_ij=1.

The dual polyhedron is pointed and has finitely many vertices. Every vertex
has at most three positive coordinates (three equality rows), and an optimum
is attained at a vertex. These claims follow directly from basic LP geometry;
rank degeneracies reduce the possible positive support size further. Therefore

    r_j(h)=min_{lambda in D_j} lambda.h,
    c(h)=min_{j,lambda in D_j} lambda.h,

where D_j is the finite complete dual-vertex set. The second equality uses the
normal-triangle covering equivalence, [Balitskiy–Mitrofanov–Polyanskii,
Theorem3.3](https://arxiv.org/html/2603.12495v2).
No numerical active-orbit catalogue or smooth HK-envelope conjecture is needed.

Let Lambda be the complete active dual vertices at h0, c0=c(h0). Since the
finite envelope has positive gaps for every inactive vertex, for each fixed v
with lambda.v=0 for all lambda in Lambda there is an open interval about zero
on which c(h0+tv)=c0 exactly.

## First-order balance and strict area curvature

Write A=area K(h0), S_i=edge length_i. The directional derivative of log ratio is

    min_{lambda in Lambda} (2lambda/c0−S/A).v.

Local maximality and strict separation force

    S/(2A) in conv{lambda/c0:lambda in Lambda}.             (B)

This is a planar product-fitting analogue of the project's fixed-normal
active-orbit balance. In particular an invisible direction v has S.v=0.

The fixed-fan polygon area is a quadratic form in its support numbers, with
Hessian signature (one positive, two zero, N−3 negative); its zero space consists
of translations. This is the planar mixed-area strict signature, also covered
by [Fillastre–Izmestiev, Corollary2.11 and Remark2.14]
(https://arxiv.org/pdf/1310.1560). Equivalently,

    S.v=0 and v not a translation => A''[v,v]<0.           (A)

If the active vectors do not span ker N, choose a vector orthogonal to their
span but outside (ker N)^perp=im N^T. Such v is not a translation. By (B),(A),
area strictly decreases at order t² along h0+tv, while capacity stays exactly
constant by the complete finite envelope. The ratio increases, contradicting
local maximality. This proves (R) and the N≤12 consequence.

The area-curvature input is essential. Mere concavity, without its equality
case/kernel, would only show a nonnegative ratio variation and would not yield
the strict contradiction.

## Concrete classification use and adverse branches

A group can now investigate 5–12 facet fixed-Q candidates by solving simultaneous
active-fit equations, planar edge-area balance, and positive dual/contact
conditions; N−2 independent active duals are mandatory. This is an actual
reduction in what a maximizer can look like, rather than a capacity-free scalar
correlation. It supplies an exact falsifier for a proposed candidate: a complete
active-dual bank with deficient span gives a support deformation with unchanged
capacity and strictly lower area.

For N>12 the useful target is **fitting degeneracy**, not routine single-dual LP
sampling. A polygon might have several distinct optimal dual vertices for the
same template, especially when a triangle has more than three essential tight
support contacts or a degenerate primal placement set. A complete classification
must retain those boundary chambers. The bound does not eliminate N=13–30 local
extrema, and the already known ≤30-vertex inner-cover reduction remains relevant.

A serious rival is the P* F11 cap mechanism in this packet: it addresses a new
non-HKO robustness theorem directly, whereas this rank condition only constrains
fixed-Q competitors. The broader HKO global cover inequality remains unresolved.
