# Independent audit: complete-simple radial hinge certificate

**Verdict: the finite simple-word certificate checks out.** Its exact result is
a strict positive first variation for the stated simultaneous facet hinge
direction, conditional on the separately audited reduction from repeated
maximizers to simple ones for profiles constant along each labeled
characteristic segment. This is a directional result; it does not establish a
neighborhood theorem.

## Coverage of the word and optimizer sets

For a cyclic order, split the weights into q- and p-factor parts and let their
total q mass be (t). Same-factor pairings vanish, so the quadratic action is

\[
Q=t(1-t)B_\sigma(x,y),
\]

where (x,y) are the normalized factor closure weights and (B_\sigma) is
bilinear. Each planar closure polytope has five vertices: the positive
barycentric closures on the five triples containing the origin. There is no
two-normal positive closure because no pair is antipodal. A bilinear function
on the product of these polytopes attains its maximum at a vertex pair. Since
the known maximum (Q_0=\tau/5>0) is attained, (t(1-t)) forces (t=1/2).
Decomposing any maximizing (x,y) into closure vertices expresses its value
as an average of vertex-pair values. Since none can exceed (Q_0), every
positive pair in the two decompositions must be a winning six-word. This is
exactly the biclique condition used by the catalogue.

`prepare.py` checks all (5\cdot5\cdot120=3000) oriented six-orders after
fixing the smallest label to quotient cyclic rotation, using exact field
arithmetic and the known capacity value. It finds 50 winners. Every simple
word of length at most ten is covered: add its omitted labels at zero weight
to a ten-label cyclic order; its nonzero restriction is the original cyclic
word. The C++ pass fixes label 0 first and scans all (9!=362{,}880) such
orders. For each it builds the winning (5\times5) graph and enumerates
maximal bicliques by common-neighbor closure. These bicliques are precisely
the maximal products of vertex supports that can occur in a maximizer. The
chart key retains both the restricted cyclic order and its two vertex masks,
so deduplication does not merge distinct optimizer sets. The resulting 190
charts comprise 50 point charts and 140 one-parameter charts; no larger
biclique occurs. I checked the enumeration logic against the JSON and
reproduced the reported 121 nonempty active graphs.

## Exact hinge optimization

Each chart is a product of either one q and one p closure vertex, or one
vertex on one side and an edge between two vertices on the other. Thus its
weights are constant or affine in a single (z\in[0,1]). The multiplier and
segment placement are affine in those weights. For the selected direction,
the feature (\ell_i) is constant along every segment labeled (i): its
vector is perpendicular to the segment velocity (Ja_i). The orbit response
is therefore

\[
G(z)=\sum_i \beta_i(z)\bigl(\ell_i(z)-3/5\bigr)_+.
\]

The script finds every exact zero of each affine hinge argument, partitions
\([0,1]) at those breakpoints, and on each interval (G) is quadratic. It
checks interval endpoints and every interior stationary point when the
quadratic is concave. A convex quadratic or linear piece has its maximum at an
endpoint. This covers the complete chart, including hinge-switch points and
the chart endpoints. All inversions, roots, comparisons, and candidate values
use the exact (\mathbb Q(\tau)) field; sign decisions use exact isolation,
not floating-point tolerances.

I copied the four producer inputs to a fresh temporary directory and reran
`prepare.py`, compiled and ran `catalogue.cpp`, and ran `optimize_exact.py`.
The resulting primitive, catalogue, and exact-result JSON agreed with the
retained files after ignoring runtime fields. The rerun found the maximum
response (1/125), at (z=3/10-\tau^2/50) or its reflected mate
(7/10+\tau^2/50) on the attaining charts.

## Volume normalization and sign

The cut profile is present on all ten facets, with the same normalized hinge
depth (f_i=(\ell_i-3/5)_+). Each facet has cone-volume mass (1/10). In the
opposite pentagon, the cut is a vertex cap of depth
\(\delta=1-3/5=2/5\); for \(\delta<1-\cos72^\circ\), its area is
\(\cot(\pi/5)\delta^2\). Integrating the hinge depth over that cap gives
\(\int f_i=\cot(\pi/5)\delta^3/3\). The pentagon area is
\(A=5\tau/(1+\tau^2)\), so summing the ten equal facet contributions gives

\[
\nu(f)=\frac{\cot(\pi/5)}{3A}\delta^3
       =\frac{\delta^3}{3\tau A}
       =\frac{8}{625}-\frac{8\tau^2}{9375}.
\]

The factor ten is essential here; for a cut on only one facet the value would
be one tenth as large. The exact optimizer value (1/125=75/9375) is below
the volume response. The gap is

\[
\nu(f)-\max_\gamma\mu_\gamma(f)
=\frac{45-8\tau^2}{9375}
=\frac{5+16\sqrt5}{9375}>0.
\]

Since (D^+\log\operatorname{sys}=4(\nu(f)-\max_\gamma\mu_\gamma(f))
\) in dimension four, the exact derivative is
\((20+64\sqrt5)/9375>0\), matching `exact-results.json`.

## Boundary of this audit

This folder proves completeness for simple cyclic words and computes their
exact maximal hinge response. The actual derivative theorem allows repeated
words. The conclusion for the true capacity derivative therefore also needs
the stated separate lemma that this facet profile is constant along each
labeled segment and that repeated-word removal preserves or improves the
response. The present producer files do not prove that lemma; I treated it as
the separately reviewed input identified in the task. Without that bridge,
the finite catalog alone would only bound the simple-word response, not the
maximum over all derivative-active words. With the bridge, the strictly
positive right derivative along the explicit path \(K_t\to H\) disproves
Hausdorff local maximality at \(H\): arbitrarily small positive \(t\) increase
the ratio. It supplies no quantitative radius or bound at a specified finite
parameter.
