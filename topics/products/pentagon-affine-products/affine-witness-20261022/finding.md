# Affine 5×7 sprint finding (2026-10-22)

## Source scope

Read `/workspaces/msc-math/README.md`, `AGENTS.md.commentary.md`, `topics/products/pentagon-affine-products/next-obstructions-20261022.md`, the relevant setup and principal reduction in `research.tex`, `tensor-certificate-review-20261022.md`, and `tensor-certificate-20261022/report.md`. The review accepts strict full-four-dimensional 5×7 tensor containment for the checked finite family and its consequence for independent affine 5×7 capacity. It explicitly does not establish the sharp affine systolic optimum or novelty. The reviewed witness is retained here; no selected manuscript changes were made.

## Candidate exact lower bound

Let $n_i^{(k)}=(\cos(2\pi i/k),\sin(2\pi i/k))$. Let $M_*$ be the unique solution in $M_2(\mathbb R)$ of

\[
(n_0^{(5)})^TM_*n_3^{(7)}=1,\quad
(n_3^{(5)})^TM_*n_0^{(7)}=1,\quad
-(n_1^{(5)})^TM_*n_1^{(7)}=1,\quad
-(n_4^{(5)})^TM_*n_5^{(7)}=1.
\]

These exact algebraic equations define a point (the four coefficient rows are independent). The standalone interval script `certify_vertex.py` imports the exact trig enclosure and interval class from the retained tensor checker (SHA-256 `a183fc7a8da82859694d54a9fb8ecef870695587291af2c4912a12e31c3ed4dc`); both the helper and this adaptation have independent review; the latter was also replayed, as recorded below. It performs interval Gauss–Jordan elimination and certifies all 66 other signed pairings strictly below 1, with minimum slack lower bound $0.029926273871775$, and

\[
\det M_*>1.0063055.
\]

The four contacts ensure $m(M_*)=1$. By the already established independent-affine capacity formula and area transformation, the affine systolic ratio is $B_{57}\det M_*$, so this is a rigorous explicit lower bound $>1.0063055 B_{57}$. A separate exact interval comparison in the script certifies
$\det M_*-\sec^2(\pi/70)>0.0042$, where $\sec^2(\pi/70)$ is the best rotation value. This is a candidate point certificate, not a sharpness proof.
## Why an edge search is finite

For the matrix polytope (\mathcal P=\{M: |(n_i^{(5)})^TM n_j^{(7)}|\le1\}), every active facet normal is rank one. At a relative-interior point of a 2-face, choose two independent active rank-one constraints. If neither factor normal is shared, invertible row/column coordinate changes put the constraints in the form (M_{11}=c_1, M_{22}=c_2), and the determinant on the face has a saddle quadratic term (-M_{12}M_{21}), so it has no local maximum in the face interior. If a factor normal is shared, the determinant restricts to an affine function on the face; a nonconstant affine function has no relative-interior maximum, while a constant one ties its boundary. For faces of dimension at least 3, intersect the tangent space with the conformal subspace ({aI+bJ}): a nonzero tangent direction (N) there has (det N=a^2+b^2>0), so (det(M+sN)) increases for one sufficiently small sign (or both if the linear term vanishes). Thus a maximizer lies on an edge or vertex. (Corrected detail: in the shared-normal case the restricted determinant is affine, generally not identically zero.)

This reduction suggests exact finite enumeration of the 70 signed contacts. `edge_search.py` enumerated all floating-point triples of signed constraints and optimized determinant over their feasible lines; it found the above four-contact candidate with determinant about 1.006305539955406 (floating estimate). This floating search is exploratory only and does not prove global sharpness. Exact edge enumeration/certification remains open.

## Limits

Exact arithmetic applies to the finite candidate verification, not the full edge search. The imported tensor certificate does not by itself imply systolic sharpness. The currently available result is a stronger feasible lower bound plus a finite reduction to edges/vertices; global upper bound, optimizer classification, novelty, and human paper-quality review remain unsettled.

Independent bounded audit of `certify_vertex.py` was completed by `/root/affine_optimum/audit_vertex` before 20:49 UTC. The reviewer replayed the script, checked the contact encoding, row-major vectorization, four equality rows, exact coverage of the other 66 signed constraints, interval elimination/pivot logic, determinant and rotation-gap assertions, and found no defect. Scope remains only the explicit feasible lower-bound witness, not global optimality or novelty.
