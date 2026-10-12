# Independent review: equal covariance and volume do not determine capacity

Reviewed 2026-10-04, approximately 22:55 UTC. Owner: product_covariance;
receiving/integrating owner: geometry_discovery_lead. This is a bounded mathematical
review, not a novelty claim or a thesis promotion. No capacity producer was run.

**Result:** the proposed construction works. The stronger witness below has the
same centered uniform covariance, the same four-dimensional volume, the same
number of facets and the same face lattice, but different EHZ capacity. All
formulas are exact. Capacity uses the normalization in which a planar disc has
capacity its area.

## 1. Symplectic product formula, including nonsmooth boundaries

Use coordinates `(q1,p1,q2,p2)` and symplectic form
`dq1∧dp1 + dq2∧dp2`. The planar factors are **symplectic coordinate
planes**. This is not the Lagrangian product `R²_q × R²_p`; permute coordinates
when expressing the witness in the repository's `(q1,q2,p1,p2)` convention.

For planar convex polygons A and B with nonempty interiors,

\[
 c_{\rm EHZ}(A\times B)=\min\{|A|,|B|\}.
\]

Primary source for the nonsmooth capacity characterization:
Pazit Haim-Kislev, *On the symplectic size of convex polytopes*,
[arXiv:1712.03494v3](https://arxiv.org/pdf/1712.03494), Sections 2.1–2.2,
PDF pages 5–7. Section 2.2 defines generalized characteristics through the
normal cone and identifies capacity with least positive characteristic action.
The following is an independent deduction from that characterization, rather
than an invocation of a product formula with an uncertain product convention.

Write a generalized characteristic as γ=(a,b). Normal cones split:

\[
 N_{A\times B}(a,b)=N_A(a)\times N_B(b),\qquad
 \dot a\in J_2N_A(a),\quad \dot b\in J_2N_B(b)
 \quad\text{a.e.}
\]

Here the interior normal cone is {0}. Each projected loop is either constant
or lies on its polygon's boundary. To justify that assertion, a connected time
interval during which a projection is interior has zero derivative almost
everywhere, so the projection is constant there; it cannot reach the boundary
at an endpoint of that interval. On an open edge, the permitted nonzero velocity
is a positive multiple of the positively oriented tangent. At vertices, an
absolutely continuous projected curve has zero derivative almost everywhere
on the preimage of each individual vertex. Thus a nonconstant projection moves
monotonically around the polygon boundary, possibly with pauses. Its closed
oriented arclength lift has positive integer winding.

The standard action splits into planar actions, and Green's formula gives

\[
 \mathcal A(\gamma)
 =\frac12\int(a_1\,da_2-a_2\,da_1)
  +\frac12\int(b_1\,db_2-b_2\,db_1)
 =m|A|+n|B|,
\]

where m,n are nonnegative integers and at least one is positive for a
nonconstant characteristic. Consequently every positive-action characteristic
has action at least min(|A|,|B|). Conversely, traverse ∂A once while holding b
fixed in int(B), or traverse ∂B once with a fixed in int(A). These are valid
polygonal generalized characteristics and have actions |A| and |B|. This
proves both bounds and handles product corners directly. Constant loops are
excluded from the least-positive-action characterization.

This proof suffices for all witnesses here. Extension to arbitrary planar
convex bodies can use polygonal approximation and continuity, but is unnecessary
for the claimed examples.

## 2. Exact planar whitening

For a planar body P, let μ_P and Σ_P be the mean and covariance of its uniform
probability measure. Set

\[
 \widehat P=\Sigma_P^{-1/2}(P-\mu_P),\qquad
 r(P)=|\widehat P|=\frac{|P|}{\sqrt{\det\Sigma_P}}.
\]

The whitened body has mean zero and covariance I₂. Whitening is used to
construct new bodies; it is not asserted to preserve capacity. The capacity of
the new product is calculated from its new planar areas by Section 1.

Use the nondegenerate trapezoid family, for 0<t≤1,

\[
 T_t=\{(x,y):0\le y\le1,\ |x|\le1-(1-t)y\}
 =\operatorname{conv}\{(-1,0),(1,0),(t,1),(-t,1)\}.
\]

Direct integration gives

\[
 |T_t|=1+t,\qquad
 \mu_t=\left(0,\frac{1+2t}{3(1+t)}\right),
\]
\[
 \Sigma_t=\operatorname{diag}\left(
 \frac{1+t^2}{6},\frac{1+4t+t^2}{18(1+t)^2}\right),
 \qquad
 r(t)^2=\frac{108(1+t)^4}{(1+t^2)(1+4t+t^2)}.
\]

Thus the witness vertices are fully explicit: subtract μ_t from the four
listed vertices and multiply respectively by

\[
 \sqrt{\frac6{1+t^2}},\qquad
 \frac{\sqrt{18}(1+t)}{\sqrt{1+4t+t^2}}.
\]

The useful substitution u=t+t⁻¹ yields

\[
 r(t)^2=\frac{108(u+2)^2}{u(u+4)},\qquad u\ge2.
\]

The triangle limit t=0 has normalized area 6√3 and the parallelogram endpoint
t=1 has normalized area 12. For square S=[−√3,√3]², mean=0,
covariance=I₂ and |S|=12.

## 3. Main witness: equal eight-facet combinatorics

Define

\[
 u_0=-2+162\sqrt{\frac3{2783}},\qquad
 t_0=\frac{u_0-\sqrt{u_0^2-4}}2
 \approx0.335152879019720.
\]

Here u₀>2, so 0<t₀<1. Exact substitution into the preceding formula gives

\[
 r(1/2)=54\sqrt{\frac3{65}},\qquad
 r(t_0)=\frac{729}{65}.
\]

The positive square roots are selected because areas are positive. Set

\[
 K_{\rm bal}=\widehat T_{1/2}\times\widehat T_{1/2},\qquad
 K_{\rm unbal}=S\times\widehat T_{t_0}.
\]

The uniform measure on a Cartesian product is the product of the uniform
factor measures. Both bodies consequently have mean zero and covariance I₄.
Their volumes agree exactly:

\[
 \operatorname{vol}_4(K_{\rm bal})
 =\left(54\sqrt{3/65}\right)^2
 =\frac{8748}{65}
 =12\cdot\frac{729}{65}
 =\operatorname{vol}_4(K_{\rm unbal}).
\]

But their capacities differ:

\[
 c_{\rm EHZ}(K_{\rm bal})=54\sqrt{\frac3{65}},\qquad
 c_{\rm EHZ}(K_{\rm unbal})=\frac{729}{65}
 <54\sqrt{\frac3{65}}<12.
\]

Both are products of two genuine quadrilaterals, hence simple polytopes with
eight facets, sixteen vertices, and the face lattice of the four-dimensional
cube. Their f-vector is (16,32,24,8). The factor shapes and supporting
hyperplanes differ, but combinatorial incidence agrees.

## 4. Secondary witness: square pair versus hexagon–trapezoid

Let H be the regular hexagon with vertices
`(cos(kπ/3),sin(kπ/3))`, k=0,…,5. Polygon moment formulas give

\[
 |H|=\frac{3\sqrt3}2,\quad\mu_H=0,\quad
 \Sigma_H=\frac5{24}I_2,\quad
 |\widehat H|=\frac{36\sqrt3}5.
\]

For

\[
 u_1=-2+\frac{20}{\sqrt{19}},\qquad
 t_1=\frac{u_1-\sqrt{u_1^2-4}}2
 \approx0.472668922886133,
\]

exact substitution gives r(t₁)=20√3/3. Therefore

\[
 K_0=S\times S,\qquad K_1=\widehat H\times\widehat T_{t_1}
\]

both have mean zero, covariance I₄, and volume 144, whereas

\[
 c_{\rm EHZ}(K_0)=12,\qquad
 c_{\rm EHZ}(K_1)=\frac{20\sqrt3}{3}<12.
\]

This pair has different facet counts (8 and 10), so the eight-facet pair is the
stronger answer when combinatorics are among the controlled features.

## 5. Verification and claim boundary

A scratch SymPy calculation independently evaluated the standard polygon
shoelace moment formulas for the hexagon and trapezoid, checked the expression
in u against the expression in t, and simplified both target-area residuals
exactly to zero. Command:

`uv run --with sympy python /tmp/msc-math/geometry-product-review/verify.py`

That script is disposable corroboration, not the retained mathematical basis;
the moment formulas, parameters, whitening maps and complete capacity argument
above suffice to reproduce the witness. No Rust integration or polytope capacity
certificate was attempted. Those would provide independent implementation
checks rather than establish the underlying implication for the first time.

The result refutes determination of EHZ capacity by volume plus centered uniform
covariance, even within the fixed eight-facet cube combinatorial class. It does
not refute Viterbo's conjecture, establish broad empirical predictive performance,
or establish a novel theorem. Literature novelty remains unknown. These witnesses
are not centrally symmetric; no claim about the centrally symmetric subclass
is made. The use of a symplectic rather than Lagrangian product is essential.
