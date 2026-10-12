# Independent review: escape from one vertex–edge contact

Reviewed 2026-10-04, UTC. Scope: the proposed single-contact slide, including
triangles that own several hull vertices. This review establishes no global
minimum, finite contact classification, or ruler-edge theorem.

## Verdict

The single-contact lemma is valid under the explicit strictness conditions
below. A triangle may own any number of vertices of the prescribed hull cycle;
this does not spoil the affine area argument. One slide reaches an additional
physical boundary incidence in finite parameter distance. An iteration through
arbitrary higher-contact strata is **not** established by this lemma.

## Exact hypotheses and inequalities

Let the triangles be fixed, nondegenerate planar triangles, independently
translated. The number ten is immaterial. Labels distinguish all triangle
vertices. At the placement under consideration, let

\[
H=(h_1,\ldots,h_m)
\]

be the counterclockwise cycle of the actual, strictly convex hull vertices,
each with a unique vertex label. Let a vertex \(a\) of triangle \(A\) be absent
from \(H\), and let consecutive hull vertices \(b,c\) belong to triangles
\(B,C\), with \(A,B,C\) distinct. Suppose

\[
a=b+\lambda_0(c-b),\qquad 0<\lambda_0<1.
\]

For every directed cycle edge \(e_i=[h_i,h_{i+1}]\) and every input vertex label
\(p\), define

\[
q_{i,p}=\det(h_{i+1}-h_i,p-h_i).
\]

Indices of the cycle are cyclic. Omit the two **endpoint labels** from each
edge's inequalities: their determinants are identities equal to zero. Require
\(q_{BC,a}=0\), and require \(q_{i,p}>0\) for every other pair not omitted.
Thus even a vertex that belongs to the same triangle as an edge endpoint is
tested unless it is that endpoint label. A triangle's own third vertex is not
a tautological zero; when both edge endpoints belong to that triangle, its
determinant is a fixed nonzero intrinsic oriented area. For nondegenerate
triangles there are no other intrinsic three-vertex collinearity identities.

These conditions exclude duplicate hull labels, other collinear boundary
vertices, and additional vertex–support-line incidences. They are a precise
version of “all other hull inequalities strict.” They also imply strict hull
turns, by testing the next cycle vertex against the previous edge. For polygons
with redundant collinear vertices, additional persistent zeros would have to
be specified separately; the present formulation is for nondegenerate
triangles.

## Proof

Freeze every triangle except \(A\), and translate \(A\) by \(s d\), where
\(d=c-b\ne0\). In particular

\[
a(s)=b+(\lambda_0+s)d.
\]

The prescribed supporting-line equality persists identically. For each of the
remaining determinants, write each point as its initial position plus either
\(s d\) or zero. On expanding the determinant, its only possible quadratic
term is a scalar multiple of \(s^2\det(d,d)=0\). Thus every \(q_{i,p}(s)\) is
affine, including edges whose two endpoints belong to \(A\), edges with only
one such endpoint, and tests against other vertices of \(A\).

Let \(I\) be the set where all these strict inequalities hold and
\(0<\lambda_0+s<1\). It is an open interval containing zero, because it is a
finite intersection of open affine half-lines and an open interval. Its
closure is bounded by

\[
-\lambda_0\le s\le1-\lambda_0.
\]

Throughout \(I\), the cycle \(H\) remains the actual strict hull cycle, and
\(a\) remains in the relative interior of its edge \([b,c]\). The hull area is
the shoelace expression

\[
F(s)=\tfrac12\sum_i\det(h_i(s),h_{i+1}(s)).
\]

Its quadratic terms likewise vanish, so \(F(s)=F(0)+\beta s\). This does not
require \(A\) to own only one hull vertex.

If the initial placement is a local minimum of hull area, either in the full
translation space or on this single-contact stratum, both signs of sufficiently
small \(s\) are admissible within the stratum. Therefore \(\beta=0\), and the
area is constant on all of \(I\). It is also constant at both endpoints by
continuity of convex-hull area. In particular, the argument does not rely on
the cycle remaining nondegenerate at an endpoint.

At either endpoint of \(I\), one of two events occurs:

1. \(\lambda_0+s\) reaches zero or one, so \(a\) coincides with \(b\) or \(c\).
2. A previously strict \(q_{i,p}\) becomes zero. If the corresponding edge
   endpoints coincide, this is a new coincidence of labeled hull vertices.
   Otherwise their line is still a supporting line of the limiting hull, and
   \(p\) lies on its exposed face. This is a new boundary collinearity.

The second event need not put \(p\) inside the *old pair's* segment
\([h_i,h_{i+1}]\): adjacent hull edges can become collinear and the exposed
face can extend beyond that segment. “Additional supporting-face incidence or
vertex coincidence” is the precise safe conclusion. An equality of an omitted
endpoint determinant alone cannot be the stopping event, since it was already
an identity throughout the slide.

## Consequences and stopping boundary

For at least two triangles, the slide is nonconstant modulo common
translation: a fixed triangle's zero displacement forces any putative common
translation to be zero, whereas \(A\) moves by \(s d\). Hence no local minimum
on this singly-contact stratum is isolated modulo common translation. Nearby
equal-area placements are themselves local minima in the same ambient domain:
take a small neighborhood contained in the original local-minimum
neighborhood.

The finite escape assertion refers to **this one interval**. If the initial
placement is a global minimizer, its endpoint is also a global minimizer.
For an initial local minimizer, the distant endpoint need not itself be a
local minimizer; only its equal area follows.

At an endpoint the one-contact strictness assumptions fail. Continuing the
same procedure while preserving all old contacts would require a new
existence-and-progress argument on coupled constraint strata. This proof
supplies neither a finite iterative termination bound nor a finite list of
contact patterns.

Source reviewed: `topics/products/pentagon-arbitrary-partner/smooth-cell-reduction.md`.

## Extension: deficient active rank in one translation block

The same implication extends to an arbitrary number of current boundary
incidences, provided the hull cycle is chosen as follows. Use the strictly
convex cycle of the actual geometric extreme points, and choose one input
vertex label to represent each extreme point. Do not include redundant
collinear points in that cycle. All input labels, including duplicate labels
at a represented extreme point, remain in the support determinant tests.
This convention accommodates coincident input vertices without requiring a
cycle edge of zero length.

Include all \(q_{i,p}\ge0\), and let \(E\) be the set with \(q_{i,p}=0\) at the
initial placement. Endpoint-label identities may be included: their gradients
are zero, so they contribute no rank. Freeze every block except the translation
\(t_j\in\mathbb R^2\) of one triangle. Each determinant is affine in that entire
two-dimensional block, since its potential quadratic term is proportional to
\(\det(t_j,t_j)=0\). Form the matrix whose rows are

\[
\nabla_{t_j}q_{i,p},\qquad (i,p)\in E.
\]

If its rank is less than two, choose a nonzero vector \(d\) in its kernel.
Along \(t_j(s)=t_j(0)+s d\), every active determinant remains exactly zero for
all \(s\); this is an affine identity, not just a first-order tangency. All
initially positive determinants remain positive for small positive and
negative \(s\). The tests of neighboring cycle vertices against cycle edges
were positive at the initial strict geometric hull, so the hull cycle stays
strict on that interval. The shoelace area is affine in \(s\). At a local
minimum its derivative vanishes, and it is constant throughout the open
interval \(I\) where all initially positive determinants stay positive.

This interval is bounded whenever some other frozen triangle is
nondegenerate. Indeed, put a closed disk of radius \(r>0\), centered at \(z\),
inside that fixed triangle. For any vertex \(p(s)\) of the moving triangle,
the full hull contains the triangle formed by \(p(s)\) and the endpoints of
a diameter of that disk perpendicular to \(p(s)-z\). Hence

\[
\operatorname{area}(K(s))\ge r\lVert p(s)-z\rVert.
\]

The constant area therefore bounds \(\lVert p(s)-z\rVert\); since
\(p(s)=p(0)+s d\) and \(d\ne0\), it bounds \(s\). At an endpoint of \(I\),
some previously positive determinant becomes zero. The same supporting-face
incidence or vertex-coincidence interpretation applies as in the single-contact
proof. All old support equalities remain true, although an old edge can merge
into a longer exposed face or collapse; do not infer preservation of a strict
edge combinatorics at the endpoint.

Consequently an **isolated labeled placement local minimum**, modulo common
translation, must have active block rank two for every triangle. To test block
\(j\), anchor a different triangle: then this kernel slide has nonzero
displacement in the quotient. With one fixed anchor, the argument directly
tests only the unanchored blocks; reanchoring gives the condition for the
original anchor as well. Rank two is necessary, not sufficient.

The rank is calculated using the stated representative-label convention and
all support tests. Different choices of representative labels can change the
displayed rows; the proof applies to any such choice. Thus an isolated
placement would have to pass the rank-two test for each choice. No
representation-independent complete contact classification is asserted.

This conclusion concerns placement coordinates. A triangle that is invisible
inside the hull can move without changing the hull body at all, so placement
nonisolation does not establish nonisolation among distinct convex bodies.
Nor does this extension prove that repeated deficient-rank slides terminate
at a rigid placement: a new equality can have zero gradient in a later block,
and old cycle combinatorics can degenerate. The finite result established here
is attainment of the next boundary along each chosen constant-area slide.
