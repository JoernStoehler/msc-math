# Read-only integration review: frozen HKO ascent section

**Verdict: mathematically coherent, with one small scope clarification recommended.** The profile, path, exact slope, and finite enumeration agree with the derivative appendix and the reviewed exact certificate. The repeated-word lemma is the necessary bridge from that finite simple-word result to the actual capacity derivative, and the section states and proves that bridge rather than relying only on existence of a simple shortest word.

## Coordinates, profile, and path

The coordinate conventions are consistent: (J(q,p)=(-p,q)), the q normals
are (n_i), and the p normals of (-J_2P) are (m_i=-J_2n_i). Their
norms are (sqrt{1+\tau^2}=\sec(\pi/5)=\sqrt5-1). On the q facet,

\[
D_i\cdot(q,p)=\frac{J_2n_i}{|n_i|}\cdot p-\frac35,
\]

and on the p facet the analogous expression is
\(-J_2m_i\cdot q/|m_i|-3/5). Thus the row bank realizes exactly the
displayed hinge profiles together with the retained old rows. The feature
vector is orthogonal to (Ja_i), so each profile is constant along its own
characteristic segments, as required by the repeated-word argument.

Expanding (H\cap L_t^{-1}H) gives the new rows
\(((1-3t/5)n_i,\,t\cos(\pi/5)J_2n_i)\) and
\((-t\cos(\pi/5)J_2m_i,\,(1-3t/5)m_i)\), exactly (a_j+tD_j), since
\(1/|n_i|=\cos(\pi/5)\). The inclusion, boundedness, full dimension for
sufficiently small (t), and Hausdorff convergence follow. The derivative
statement uses only this fixed finite row bank, so the full-rank hypothesis
from the appendix applies to the HKO product.

## Repeated-word bridge and scope

The repeated-word proof has the needed ingredients. Transfer between two
copies of one normal makes (Q(z)) affine; the repeated-word polar-loop bound
and the interior maximum at the original weights force it to be constant.
At interior transfer parameters, positive closure in each planar factor
spans that plane, giving rank five and unique affine KKT multipliers. The
translation split across the interval between duplicate visits, together
with at least three directions in each factor, forces the two duplicate
segments to move only along (Ja). Their profile contribution is affine;
the other, fixed-weight segment integrals are convex in the transfer
parameter. Hence an endpoint weakly improves the response and deletes one
visit. Iteration proves the stated reduction. Continuity on the compact
labeled facet union passes the response bound to weak limits.

The final theorem consequently follows from the actual derivative formula,
not just a feasible upper branch. A positive right derivative along (H_t\to H)
does disprove Hausdorff local maximality and implies improvement for all
sufficiently small positive (t); the explicit endpoint is not supplied.
The section correctly limits the existing facet-count result to eleven and
does not claim a quantitative neighborhood radius.

The section now explicitly verifies that all twenty rows are genuine facets
for sufficiently small positive t. On each old facet, D_i·x has positive and
negative open patches: the negative patch retains the old row, and scaling a
positive-patch point onto the new hyperplane leaves all other inequalities
strict. This closes the scope comparison with the at-most-eleven-facet result.
I have no remaining mathematical concern in the frozen section.
## Enumeration and exact constants

The proof's reduction matches the audited catalogue. Each factor closure
polytope has five positive triangular vertices. With q mass (t), the action
is (t(1-t)B_\sigma(u,v)); because (Q_*>0), a maximizer has (t=1/2).
Decomposing both factor weights into triangle vertices makes the action a
convex combination of vertex-pair actions, so every pair used with positive
coefficient must be among the 50 exact winning six-orders. The fixed-first
enumeration covers all (9!=362{,}880) cyclic ten-orders; zero insertion
covers shorter simple words. The winning-pair graph and maximal-biclique
construction yield all optimizer products, hence the 190 charts (50 points,
70 q-edges, 70 p-edges). The piecewise-quadratic hinge maximization checks
every switch and interval stationary point with exact algebraic signs.

The volume normalization includes all ten equal facets. A single opposite
pentagon cap has area (\delta^2\cot(\pi/5)\), and the integral of its hinge
depth is (\delta^3\cot(\pi/5)/3\). Thus

\[
\nu(f)=\frac{\delta^3}{3\tau A}
=\frac8{625}-\frac{8\tau^2}{9375},\qquad
\max_\gamma\mu_\gamma(f)=\frac1{125}.
\]

The gap is \((5+16\sqrt5)/9375\), so the systolic log derivative is four
times this, \((20+64\sqrt5)/9375>0\). These constants and signs agree with
the exact results. I found no coordinate, factor, or sign mismatch.
