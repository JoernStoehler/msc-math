# Global vertex reduction for the nonconformal five-normal family

Independent mathematical review, 2026-10-04 UTC. Receiving owner:
`direct_cover_lead`. This note proves the finite-vertex premise at each fixed
parameter; it does not enumerate vertices, derive the denominator envelopes,
or establish the selected global ratio/equality answer across parameters.

## Verdict and precise objects

Fix \(x>0\), \(G=\operatorname{diag}(x,1)\), and the regular unit normals
\(n_i=(\cos(2\pi i/5),\sin(2\pi i/5))\). A raw offset vector defines

\[
K_x(h)=\{q:(G n_i)\cdot q\le h_i\}.
\]

Writing \(y=G^Tq\) gives exactly

\[
Y(h)=\{y:n_i\cdot y\le h_i\},\qquad
\operatorname{area}K_x(h)=\operatorname{area}Y(h)/x.
\]

This is a coordinate identity, not a claim that changing only one product
factor preserves capacity.

Assume the complete fitting-capacity formula with the genuine ten fixed-Q
triangles \(T_j\) has been established. In particular assume

\[
c(K_x(h)\times_L Q)=\min_{l=0}^4
\frac{\lambda^{(l)}\cdot h}{D_l(x)},\qquad
D_l(x)=\max_j\sum_i\lambda_i^{(l)}h_{T_j}(G n_i)>0.
\tag{1}
\]

The five \(\lambda^{(l)}\) are the normalized nonnegative extreme closure
rays for the regular normals. Since \(G\) is invertible,
\(\sum_i\lambda_i G n_i=0\) iff \(\sum_i\lambda_i n_i=0\), so closure-ray
completeness transfers directly. Positive denominators follow by translating
each genuine triangle to contain zero in its interior: all five transformed
normal supports are then positive, and the weighted sum is invariant under
translation by the closure identity. Formula (1), including the complete
template list and capacity convention, is an input to this vertex review.

Under these hypotheses the global area minimum with capacity at least one
exists, has capacity exactly one, and **every minimizing canonical support
class is a vertex** of the specified ten-inequality polyhedron. Consequently
enumerating its vertices suffices for the area minimum and its entire body
optimizer list at this fixed \(x\).

## Canonical supports and the closed type cone

Arbitrary raw offsets can include redundant inequalities. Replace them by

\[
\bar h_i=h_{Y(h)}(n_i).
\]

This leaves the body unchanged. Indeed \(Y(h)\subseteq Y(\bar h)\) by the
definition of support, whereas \(\bar h_i\le h_i\) gives the reverse
containment. Capacity and area therefore do not change. For a missing edge,
its canonical support line passes through a vertex; it cannot be left at an
arbitrary larger raw offset for the following type-cone argument.

Put \(\delta=2\pi/5\). For canonical supports the oriented edge lengths are

\[
L_i(h)=\frac{h_{i-1}+h_{i+1}-2\cos\delta\,h_i}{\sin\delta}.
\tag{2}
\]

The closed cone \(C=\{h:L_i(h)\ge0\text{ for all }i\}\) consists exactly of
the support vectors of polygons whose normal fan coarsens the given regular
fan, together with translated points. To see the geometry, intersect each
successive pair of support lines. Formula (2) is the signed distance between
successive intersections along edge \(i\); nonnegative lengths give the
closed convex boundary in cyclic order. A zero length makes those intersections
coincide and loses the corresponding edge. Conversely the canonical supports
of any bounded polygon defined with these normals give this construction,
including the coincident intersections at its missing edges.

The kernel of the five lengths consists precisely of translations:
\(h_i=n_i\cdot z\). Their two-dimensional span is the kernel of the cyclic
linear recurrence in (2). Thus the quotient cone has dimension three and is
pointed. Every nonzero class in this cone has positive area: its nonzero edge
lengths close a polygon; two nonzero directions cannot close because no pair
of these five directions is opposite, and three or more closing directions
give a polygon with interior. The only degenerate canonical object here is
a translated point. Actual facet counts on the nonzero cone boundary are
three, four, or five; genuine zero-edge boundaries must remain included.

Define the translation-quotient polyhedron

\[
P_x=\{[h]\in C:\lambda^{(l)}\cdot h\ge D_l(x),\ l=0,\ldots,4\}.
\tag{3}
\]

Both types of inequalities are invariant under translation. Large constant
supports satisfy all ten inequalities strictly, so \(P_x\) has quotient
dimension three. The positive \(D_l\) exclude its point class. In fact (1)
implies that each feasible body contains a translate of every \(T_j\) in
physical K coordinates, and hence contains at least one genuine triangle.
The corresponding Y triangle is \(G^T T_j\), also genuine.

## Existence and coercivity, including recession

Fix one template \(S=G^TT_j\), and a disk of radius \(r>0\) inside it. Every
\(Y\) in (3) contains a translated copy of that disk, centered at some \(z\).
For every \(p\in Y\), the convex hull contains the triangle formed by \(p\)
and a diameter of the disk perpendicular to \(p-z\). Therefore

\[
\operatorname{area}Y\ge r\lVert p-z\rVert.
\tag{4}
\]

On any area sublevel, translate that disk center to zero. Equation (4) bounds
the whole body and therefore all its canonical supports. Passing to any fixed
linear translation gauge, for example \(h_0=h_1=0\), preserves boundedness:
the required translating vector solves a fixed invertible two-by-two system.
The polyhedron is closed in this gauge, and area is continuous. Thus every
nonempty area sublevel is compact in the quotient, and a global minimum
exists. The disk radius depends on fixed \(x\); no uniform coercivity as
\(x\downarrow0\) or \(x\to\infty\) is asserted.

Its recession cone is exactly \(C\) modulo translation. Indeed the
recession inequalities are \(L_i(r)\ge0\) and
\(\lambda^{(l)}\cdot r\ge0\). The latter already hold for \(r\in C\):
translate its polygon to contain zero, making its supports nonnegative;
closure preserves the weighted sum. For nonzero \([r]\in C\), all five sums
are actually positive because that polygon has interior. Thus there are no
recession lines in the quotient.

There are finitely many vertices. Every vertex has three independent active
rows in quotient dimension three, so at most \(\binom{10}{3}=120\) triples
need be considered before singular, infeasible, and duplicate systems are
removed. This is solely a combinatorial candidate bound, not an actual count
or cost claim.

## Minkowski linearity and reduction to vertices

For canonical \(h,k\in C\), the normal fans of \(Y(h)\) and \(Y(k)\) coarsen
the same fan. Their Minkowski sum has a fan that also coarsens it, and its
support at each \(n_i\) is \(h_i+k_i\). Canonical uniqueness consequently
gives

\[
Y(h+k)=Y(h)+Y(k),\qquad Y(t h)=tY(h)\quad(t\ge0),
\tag{5}
\]

up to the corresponding translations. This identity includes zero edges.
It is the essential property that fails if one blindly interpolates
noncanonical redundant raw offsets.

Brunn–Minkowski in dimension two now gives concavity of
\(f(h)=\sqrt{\operatorname{area}Y(h)}\) on the closed type cone. For convex
bodies with interior, equality in its nontrivial convex-combination form holds
precisely when the two bodies are positively homothetic, including a
translation.

There are two equivalent ways to close the global reduction. First,
Minkowski–Weyl expresses every \([h]\in P_x\) as

\[
[h]=\sum_v\theta_v[v]+[r],\qquad
\theta_v\ge0,\quad\sum_v\theta_v=1,\quad[r]\in C.
\]

Using (5) and Brunn–Minkowski,

\[
f(h)\ge\sum_v\theta_v f(v)+f(r)\ge\min_v f(v).
\tag{6}
\]

Here \(f(r)=0\) only for the zero quotient recession direction. This proves
that the vertex minimum is the global minimum despite unboundedness.

Second, and more strongly, let \([h]\) be any global minimizer. If it is not
extreme, write it as a nontrivial convex combination of two distinct feasible
classes \([h^-],[h^+]\). Each endpoint has area at least the minimum.
Concavity then forces both endpoint areas to equal the minimum and equality
in Brunn–Minkowski. Their bodies are therefore homothetic and have the same
positive area, hence are translates of one another. Canonical supports then
give \([h^-]=[h^+]\), a contradiction. Thus **every** minimizing class is a
vertex. In particular there is no nonvertex constant-minimal-area segment
in this canonical polyhedron. Artificial flat fibers of redundant raw
offsets, or the translation directions before quotienting, do not contradict
this conclusion.

## Capacity normalization and limits of the conclusion

If a minimizer had \(c>1\), replace its body by \(c^{-1}K\). Formula (1) is
homogeneous in the K supports, so the new capacity is one; the canonical type
cone is homogeneous, and the new body remains in (3). Its area is divided by
\(c^2\), contradicting minimality. Hence every minimizing vertex has capacity
exactly one. Enumerated feasible vertices with capacity greater than one can
be discarded as optimizers; they still must not be omitted without either
checking feasibility or using this normalization argument.

For this fixed \(x\), maximizing the product's systolic ratio is equivalent
to minimizing area at capacity one: Q is fixed, capacity scales linearly
under dilating K, and area scales quadratically. The factor \(1/x\) converting
Y area to K area is fixed throughout that optimization.

The exact denominator envelopes, candidate coordinates, feasibility
intervals, comparisons, and parameter-endpoint behavior are additional work.
This proof supplies no uniform finite optimizer list as \(x\) varies until
those cells and candidates have been derived. It also does not cover normals
outside the prescribed fan. The invalid premise to avoid is treating the
ten-inequality **canonical** polyhedron as though it literally represented
every redundant raw support vector without canonicalization.

Inputs reviewed:
`../comparison/nonconformal-fan-contract.md` and
`../geometry-discovery/fixed-q-rank/equiangular-capacity-core.md`.

## Consequential arithmetic audit of the candidate reduction

Extended 2026-10-04 23:38 UTC. Reviewed `body_candidates.py` and
`body-candidates.json` without editing either. The denominator packet
`denominators.json` was used as an input; the derivation of its complete
envelopes is independently owned elsewhere. The following conclusions are
conditional on that input representing the exact complete denominators in
(1), not sampled or incomplete fitting constraints.

The implemented gauge \(h_3=h_4=0\) is legitimate because \(n_3,n_4\) are
linearly independent. Its ten inequality rows agree with (2)–(3): multiplying
edge lengths by the positive \(\sin\delta\) does not change feasibility.
Independently recomputing determinants gives exactly 100 nonsingular row
triples and 20 singular triples among the 120 combinations. The singular
triple list matched the retained packet exactly.

The six denominator cells have breakpoints

\[
0,\quad\frac{4\sqrt5-5}{11},\quad\frac{\sqrt5}{3},\quad
\frac{4\sqrt5+5}{11},\quad\sqrt5,\quad2+\sqrt5,
\]

with the final right endpoint infinite. For each nonsingular triple, two
Cramer solves give the constant and linear coefficients of its affine
candidate. Independently clipping its parameter interval against all ten
affine residual inequalities reproduced all records, including singleton
intervals at positive parameter values. The zero parameter is only a formal
endpoint for the first cell; a record supported solely at \(x=0\) is excluded.
The per-cell counts, as (feasible bases, distinct affine candidates), are

\[
(15,12),\ (20,17),\ (20,11),\ (20,17),\ (29,23),\ (29,8).
\]

All 600 nonsingular cell systems were recalculated, and all 88 distinct
candidate records matched in their complete affine supports, exact clipped
intervals, and area-polynomial coefficients. This was an independent inline
calculation using pairs of rational numbers \(a+b\sqrt5\), Python `Fraction`
arithmetic, direct three-by-three determinant expansion, and Cramer's rule.
It did not import the producer's `DomainMatrix`, inverse matrices, sign
function, or interval routine. It took about one second; the inline audit
created no retained data or replacement producer output.

For canonical supports, the area identity is

\[
\operatorname{area}Y=\tfrac12\sum_i h_iL_i
=\frac{P(h)}{\sin\delta},\qquad
P(h)=\sum_i h_i h_{i+1}-\frac{\sqrt5-1}{4}\sum_i h_i^2.
\]

Thus physical K area is \(P/(x\sin\delta)\). Q has circumradius one and
area \(\tfrac52\sin\delta\), so at complete capacity one the product's
systolic ratio is exactly

\[
\operatorname{sys}=\frac{x}{5P}.
\tag{7}
\]

This normalization is consistent with the script, including its absence of
an extra factor of two.

First, an independent exact minimum test confirmed
\(P(x)-x/5>0\) on every candidate interval. It required 170 endpoint or
stationary-point evaluations. For a quadratic with coefficients \((A,B,C)\)
and \(C>0\), the stationary value was computed as
\((4AC-B^2)/(4C)\), independently of evaluating the producer's expression at
the stationary point. For a finite interval, endpoints and any interior
stationary minimum suffice; for an infinite interval the leading coefficient
and, when needed, linear coefficient were also checked. This proves strict
ratio below one for the entire family, not merely at sampled parameter values,
once (1) and the vertex reduction are combined.

The stronger exact target supplied by the lead is

\[
B_* = \frac{3(1+\sqrt5)}{10},\qquad
\frac1{5B_*}=\frac{\sqrt5-1}{6}.
\]

A second independent rational-pair audit confirmed

\[
P(x)-\frac{\sqrt5-1}{6}x\ge0
\tag{8}
\]

on every verified candidate interval, again with 170 necessary evaluations.
The records include 19 singleton intervals and eight final-cell unbounded
intervals; both kinds were included in this audit. No candidate's target
quadratic is identically zero. All zero values, occurring in eight records,
have the single parameter

\[
x_* = \frac{\sqrt5}{3}.
\]

Evaluating their independently verified affine supports at that value gives
exactly one class in the \(h_3=h_4=0\) gauge:

\[
h_* = \left(\frac{\sqrt5-1}{6},\frac{\sqrt5+1}{6},\frac13,0,0\right).
\tag{9}
\]

For completeness the five edge-length numerators at this class are

\[
\left(\frac{\sqrt5-1}{3},\frac{\sqrt5-1}{6},\frac13,
\frac13,\frac{\sqrt5-1}{6}\right),
\]

all positive. Its area numerator is \(P_*=(5-\sqrt5)/18\), exactly
\(((\sqrt5-1)/6)x_*\). The five normalized closure sums are

\[
\left(\frac{\sqrt5-1}{6},\frac{5+\sqrt5}{30},
\frac{5-\sqrt5}{15},\frac{5+\sqrt5}{30},
\frac{\sqrt5-1}{6}\right).
\]

These are the five denominator values at \(x_*\) in the supplied packet,
so all five fitting inequalities are active and the capacity is exactly one.
The independent lead-owned optimizer check remains the retained source for
these optimizer evaluations.

Combining (8) with the proven vertex reduction gives the sharp bound
\(\operatorname{sys}\le B_*\) for all fixed-family bodies over all \(x>0\),
conditional on the denominator-envelope input. Equality completeness does
not require guessing that all minima occur at vertices: it uses the stronger
proved statement that **every** canonical minimizer is a vertex. Hence the
only equality parameter is \(x_*\), and the only capacity-one body class is
(9), modulo translation. Undoing normalization allows arbitrary positive
dilations. This is equality within the prescribed nonconformal normal-fan
family, not a claim about arbitrary pentagon partners or global Viterbo
extremizers.

## Retained audit replay and affine-regular exclusion

The independent rational-pair/Cramer calculation is now retained as
`review_candidates.py`, with the original inline checks consolidated and a
path relative to that script. Replay from the repository root:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/review_candidates.py
```

The 2026-10-04 23:48 UTC replay passed: 100 nonsingular and 20 singular
systems, all 88 records matched, 170 strict-one checks and 170 sharp-bound
checks, 19 singleton records, eight unbounded records, and a single equality
support class. Runtime was about one second. The checker uses only the standard
library, imports no producer code, reads the two retained packets, and writes
no files.

I also read the lead's `verify_optimizer.py` affine-correspondence exclusion.
The ten possibilities are complete: an invertible affine map of full-dimensional
convex pentagons must preserve or reverse cyclic vertex order, giving five
shifts in each orientation. Three noncollinear source vertices uniquely fix
that affine map. The script computes the other two required images exactly
and requires at least one nonzero residual for every correspondence. Its
coordinates are an invertible linear reparametrization of physical coordinates,
so affine-regularity is unchanged. This is a valid exact exclusion of all
affinely regular pentagons; it does not depend on a shortcut using unequal
denominators. I did not rerun this producer, which writes the lead-owned
optimizer certificate; its exact execution and certificate remain owned by
the lead.
