# Independent cross-check of the two extremality bridges

2026-10-04 23:06 UTC. Reviewer: `conditional_mechanism`. Receiving owner:
`extremality_discovery_lead`. Narrow review of the necessary span theorem in
`fixed-pentagon-active-fit-rank.md` and the fixed-normal premise discharge in
`../geometry-discovery/finite-product-envelope.md`. No new research or
producer runs; examples and the secondary moving-normal extension were not
audited.

**Outcome: both specified bridges accepted on their stated fixed-normal
domains. No material gap found.** Mathematical review remains agent review,
not Jörn acceptance or thesis promotion.

## (a) Fixed regular pentagon: active fitting duals must span

The fitting LP and its dual are correctly paired when the fitting scalar and
translation coordinates are unrestricted. Since each template is a
nondegenerate triangle and the target polygon is bounded with interior, the
fitting maximum is positive and finite. Feasibility and strong LP duality
therefore supply an optimal dual vertex. The nonnegative dual feasible
polyhedron is pointed and has finitely many vertices; three equality rows
bound each vertex's positive support by three. Its definition is independent
of the support vector `h`.

The exact normalization in the capacity bridge is justified by
[Balitskiy–Mitrofanov–Polyanskii, Theorem 3.3 and Section 6.1](https://arxiv.org/html/2603.12495v2):
the regular pentagon has ten unit normal-triangle translation classes, and
covering all of them is equivalent to capacity at least one. Rescaling the
planar partner alone gives
\(c((K/r)\times Q)=c(K\times Q)/r\). This follows, for example, by applying
the reciprocal symplectic q/p dilation and then uniform scaling. Thus

\[
 r\leq\min_j r_j(K)
 \iff\text{all unit templates fit into }K/r
 \iff c(K\times Q)\geq r,
\]

so \(c(K\times Q)=\min_jr_j(K)\), with no missing factor of two or square.
Combining all template dual vertices yields one finite linear envelope.

At the base point, inactive envelope branches have strictly positive gaps.
Consequently a direction annihilating every active dual keeps the capacity
exactly constant on a sufficiently short interval. The separation argument
for first-order maximality correctly yields

\[
 \frac{S}{2A}\in\operatorname{conv}\{\lambda/c_0:\lambda\text{ active}\}.
\]

The same direction therefore has zero first area derivative. The planar
fixed-fan area quadratic has one positive eigenvalue, translation kernel of
dimension two, and all other eigenvalues negative. Euler's identity makes the
zero first-area-derivative slice the negative subspace modulo translations.
Thus an invisible nontranslation direction strictly decreases area at order
two while keeping capacity constant.

Every dual lies in \(E=\ker[n_1\ \cdots\ n_N]\). If their span is proper in
`E`, its orthogonal complement contains a nontranslation direction: the
translation space is exactly \(E^\perp\). This proves the asserted necessary
span and the `N − 2` independent-active-dual count. With at most one optimal
dual per each of the ten templates, there are at most ten active duals, hence
`N ≤ 12`. For `N > 12`, a local maximum must have more than one optimal dual
for at least one template; one arbitrary solver dual per template is
insufficient.

**Accepted domain:** genuine facets in an open fixed-normal planar chamber,
regular fixed pentagon, complete template list and complete active dual-vertex
set. The theorem is necessary on that subchart and hence necessary for a full
partner-shape maximum. It gives neither sufficiency nor an unconditional
12-facet ceiling, and does not apply at facet-loss chamber walls without a
separate one-sided argument.

## (b) Polygon products: finite circuit envelope discharges CM/CS premises

The circuit-to-vertex correspondence is exact. For any minimal positive
normal-closure circuit \(\lambda\), its factor dual-closure vertex at supports
`h` is

\[
 x_i(h)=\frac{\lambda_i h_i}{\sum_k\lambda_kh_k}.
\]

Conversely a normalized dual-closure vertex comes from an extreme ray of the
positive normal-closure cone. In the planar setting its minimal support has
two or three facets and one-dimensional closure. All such rays and supports
depend only on the fixed normals, not `h`. Antipodal two-facet circuits are
included; a rank-deficient triple containing such a circuit is not a new
minimal ray.

For a pair of these rays and any distinct-label word, same-factor pairings
vanish and the normals-form objective is exactly \(\kappa rs\). Its support
normalization is \(H_qr+H_ps=1\), so for \(\kappa>0\)

\[
 Q_{\max}=\frac{\kappa}{4H_qH_p},\qquad
 A=\frac1{2Q_{\max}}=\frac{2H_qH_p}{\kappa}.
\]

This independently verifies the coefficient. The accepted six-facet
capacity-value theorem applies at **every** nearby support vector. Enumerating
all fixed circuit pairs and all their words therefore gives a single finite
complete quadratic capacity envelope throughout the product chamber.

Each branch's restricted normal-closure space is the direct sum of its two
circuit rays, of dimension two. Normalization leaves a one-dimensional tangent;
along it \(\kappa rs\) has strictly negative second derivative. Thus every
branch meets the reduced-Hessian regularity condition in
`conditional-mechanism.md`. Finite positive gaps remove inactive branches
locally; all base ties are retained. These establish both premises needed by
the CM/CS argument.

Zeros outside a minimal support cause no omitted branch: all possible minimal
support rays are already in the fixed list. If a larger/dense HK optimizer
appears or moves among zero coordinates, the sparse capacity-value theorem
still represents its **scalar value** by this same finite list. CM/CS requires
an exact scalar envelope, not a smooth classification of every optimizing HK
datum. This is why no strict complementarity or generic-normal assumption is
needed here.

The support-spanning conclusion is consequently valid throughout the stated
fixed-normal product chart, without an additional conditional completeness
hypothesis:

\[
 \operatorname{span}\{E_I:\text{active sparse branches}\}=\ker N.
\]

Because the normal matrix is block diagonal, this is equivalent to separately
spanning both factor closure spaces by their active circuit rays. It is not a
statement that the active normalized-weight affine hull must span the full
support quotient.

Finally, for \(a_q=\lambda_q\cdot v_q/H_q\),
\(a_p=\lambda_p\cdot v_p/H_p\), the branch formula gives
\(A(h+tv)=c_0(1+ta_q)(1+ta_p)\). The normalized weight satisfies
\(\alpha\cdot v=(a_q+a_p)/2\); hence invisibility gives `a_p = −a_q` and
mobility \(M=c_0a_q^2\). Substitution into CM produces precisely

\[
 \max_{\mathrm{active}} a_q^2\geq-\frac{D^2V[v,v]}{4V_0}.
\]

**Accepted domain:** full-dimensional planar Lagrangian factors, fixed genuine
facet normals, positive supports in an open product chamber, and complete
circuit/word enumeration. The proof discharges the scalar completeness and
strict tangent regularity premises, including fixed zero entries and dense
ties. It does not establish completeness in ambient moving-normal charts or
certify a numerical implementation. The proposed moving-normal extension and
hexagon/triangle arithmetic control are outside this bounded review.
