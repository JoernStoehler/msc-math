# HKO is locally maximal among four-polytopes with at most eleven facets

**Status, 24 September 2026.** A complete computer-assisted proof, taking the accepted sharp ten-facet theorem as input. The previously proved other-face neighborhoods and redundant-limit theorem are retained in `inherited/`; this packet proves the missing universal interior inequality. The new finite checks all pass. This is not proof-assistant formalization, external refereeing, or a literature-priority assessment.

## 1. The theorem and the previously missing statement

Let H be the Haim–Kislev–Ostrover pentagon product in standard symplectic R^4. Put

    rho(K) = c_EHZ(K)^2 / (2 Vol_4(K)),
    rho_H = (3+sqrt(5))/5.

**Theorem.** There is epsilon>0 such that every convex four-polytope K with at most eleven genuine facets and d_H(K,H)<epsilon satisfies rho(K)<=rho_H.

The neighborhood is existential. No equality classification beyond the accepted ten-facet result is claimed. The theorem does not imply local maximality for unbounded facet count or global maximality.

The accepted ten-facet input is sharpness transverse to the local symmetry orbit: in a local slice A=A_H+H_old, with r=||H_old||,

    rho(P(A))/rho_H - 1 <= -kappa r,       kappa>0.             (1)

Previously proved neighborhoods cover an extra normal limiting to the polar interior or an open positive-dimensional face. The only possible residual is a duplicate normal. Write

    b = a_0(A) + t D.

At the base facet F_0, normalize osc_{F_0}(D dot x)=1, and put

    mu = max_{F_0} D dot x,       0<mu<1.                     (2)

The moving-facet normalization has the same limiting D and mu. The endpoint theorem `inherited/math/ENDPOINTS.md` excludes any improving sequence for which mu tends to 0 or 1, uniformly over old-normal motions and relative scales. It asserts an existential endpoint collar, not an explicit one.

The missing statement was this: for every genuinely sign-changing D, find finitely many positive touching HK sections and a convex relation cancelling all forty old-normal derivatives whose capacity response strictly exceeds

    v(D) = integral_{F_0} (D dot x)_+ dnu,
    dnu = dS / (Vol(H) ||a_0||).                              (3)

This packet proves that statement on **the entire interval 0<mu<1**. There is no gap between an arbitrarily chosen numerical core and an unquantified endpoint collar.

## 2. What a cancellation response proves

For a positive touching section k, let G_k be the forty-dimensional derivative of its normalized systolic upper bound with respect to aggregate old-normal motion, and let c_k be its four-dimensional capacity response to the additional split. The derivative of b induced by motion of a_0 is included in G_k. Then

    S(A,b) <= G_k dot H_old + t [v(D)-c_k dot D]
                + O((r+t)^2),
    S = rho/rho_H - 1.                                      (4)

These are actual smooth feasible upper sections, obtained by fixing all but five positive weights and solving closure and normalization. They need not optimize the nearby capacity. Haim–Kislev's inverse-max formula gives the upper bound. Repeated/redundant normals are permitted by the polar-loop version, Remark 1.4 of [HK].

If lambda_k>=0, sum lambda_k=1, and sum lambda_k G_k=0, their response c=sum lambda_k c_k satisfies

    S <= t [v(D)-c dot D] + O((r+t)^2).                      (5)

Let C be the set of responses of finite such convex relations using strictly positive rank-five touching sections, including old sections with the new label unused. We will also use its closure. A point of closure(C) with c dot D>v(D) implies a point of C with the same strict inequality. The closure issue is important: a zero weight is not silently used to assert a smooth two-sided feasible section.

### The exact anchor used to pass from limits to positive sections

Sixty-five old sections have a strictly positive normalized relation and gradient rank 25. An exact 26-by-26 minor of their mass-plus-gradient matrix is invertible on its image. Every tight smooth upper gradient annihilates the fifteen symmetry directions; limiting gradients do too. Thus small cancellation errors have bounded signed corrections using the old rows, with total mass zero. Mixing a sufficiently small positive multiple of the old anchor absorbs the negative correction weights.

More explicitly, if a proposed relation has residual g, solve

    sum delta_i=0,       sum delta_i G_i=-g.

Add theta*anchor+(1-theta)*delta to the old rows and scale the proposed relation by 1-theta. Choosing theta=O(||g||) ensures nonnegativity and gives exact cancellation. The response changes continuously. This proves the closure assertion and is also how rounded numerical core relations are converted to exact relations. The checker verifies all forty coordinates, not just a projected LP.

## 3. The complete central comparison

**Core certificate.** For every normalized D with 1/10<=mu<=9/10, there is c in C with

    c dot D - v(D) >= 1/1,000,000.                           (6)

The facet is an interval times a pentagon. Let x be the variation along the interval and p the p-plane tilt. Its oscillation is

    |x| + h_W(p)+h_W(-p).

The unit tilt surface is the union of twenty triangles: join each of the two interval-axis tips to each edge of the polar decagon (W-W)^polar. The offset mu makes twenty triangular prisms. Their sixty initial tetrahedra cover the entire normalized core.

On each tetrahedron, the certificate supplies one exact response c that beats v at all four vertices by the displayed margin. Since v is convex, the same inequality holds throughout the tetrahedron. Longest-edge bisection partitions the root tetrahedra; the complete tree is checked.

There are 1,968 leaves, 824 exact relations, and 3,258 positive touching rows. Every base weight is strictly positive. The smallest is approximately 1.73e-10; this is not a numerical tolerance and no numerical spatial radius is inferred from it. The margin comparisons, positivity, constraints, objectives, derivatives, and complete coverage are exact. See `math/CORE_THEOREM.md` and `core/verify.py`.

## 4. The new idea for all arbitrarily small split fractions

A fixed finite bank of positive weights cannot handle all redundant limits. Instead, we let the **internal weight of a touching family depend on the actual positive cut depth at a facet vertex**, not merely on the maximum depth.

There are 257 selected affine paths of touching weights. A path associated with vertex v has

    beta(s)=(1-s) beta_start+s beta_end,       0<=s<=1,

where beta_start>=0, its new-normal weight is zero, beta_end>0, and Q(beta(s))=Q_H identically. The constraint matrix has rank five. The exact response and aggregate gradient have degree at most two:

    c_i(s)=s C_i1+s^2 C_i2,
    G_i(s)=quadratic in s,
    C_i1=4 (beta_end)_new v,
    C_i2 dot a_0=0.                                        (7)

These identities are checked from the literal quadratic objective and the constraint derivative. They are not hypotheses about an unknown capacity derivative.

Fix eta=1/10 and choose, at each of the ten facet vertices,

    x_v=(D dot v)_+,       s_v=x_v/eta.                     (8)

For 0<=mu<=eta, every s_v is in [0,1]. In each of the twenty tilt cones, constant nonnegative masses are assigned to selected paths. Old-section masses are nonnegative quadratic Bernstein polynomials in the independent variables s_v. Their aggregate mass and all forty aggregate gradients cancel as polynomial identities. An exact small anchor correction handles rational reconstruction.

These cancellations are verified on the **entire cube [0,1]^10**, a stronger domain than the physically realized s_v. No sampling is used for this assertion.

The resulting response dot D is the explicit function

    Phi_j(D) = sum_v (D dot v)_+^2 [alpha_jv + ell_jv dot D]. (9)

The coefficients are exact algebraic numbers. This response belongs to closure(C). At s_v=0 we use the closure argument of Section 2; at positive s_v the path weights are already strictly positive. No boundary branch is asserted smooth without that argument.

**Collar certificate.** On each complete tilt cone j,

    Phi_j(D) - v(D) >= mu^2/1000,       0<=mu<=1/10.         (10)

In particular, it is strict for every mu>0, however small. This is the inequality that was absent from all earlier returns.

## 5. Why (10) is a finite exact polynomial check

Write a direction in a tilt cone as

    D = u T_0 + w T_1 + (1-u-w) T_2 + mu a_0,
    u,w>=0, u+w<=1, 0<=mu<=1/10,                            (11)

where the T_i have maximum zero and oscillation one. This has only three free scalar parameters. The ten signs of D dot v partition this triangular prism into eight full-dimensional cells. The checker constructs **all** cells by enumerating intersections of the complete set of defining planes, classifying their signs, and checking dimension. It does not infer the arrangement from a numerical hull.

On a sign cell, (9) is polynomial of degree at most three. The facet integral is rational. Here is an elementary exact formula for it.

Let W be the CCW pentagon with edges e_i=w_(i+1)-w_i. For a p-plane tilt p with p dot e_i nonzero, set f_i=c+p dot w_i. Then

    integral_W (c+p dot w)_+^2 dw
       = -sum_i det(e_(i-1),e_i) (f_i)_+^4
             / [12 (p dot e_(i-1))(p dot e_i)].             (12)

To derive (12), slice W by u=p dot w. The perpendicular slice length is piecewise affine. Its derivative jumps at vertex projections; two integrations by parts against (c+u)_+^4/12 give the displayed coefficients. Alternatively, integrating the tangent cones gives the same expression. Degenerate projection directions follow by continuity.

The interval factor is integrated first. If its endpoint constants are c_max and c_min, with difference u, then

    v(D)=[I_2(c_max,p)-I_2(c_min,p)]/[5 u Area(W)],          (13)

where I_2 is the left side of (12). The factor uses nu(F_0)=2/5. The code independently checks (12)--(13) against clipped-pentagon Fubini integration at one exact point of every sign cell.

The signs of p dot e_i are constant in the interior of a tilt cone. Multiply (10) by the positive denominator

    u product_i |p dot e_i|.

After choosing the active signs, the result is a polynomial of degree at most nine. Vanishing denominators occur only on boundary directions; the original continuous inequality follows there by limits from the interior.

Every sign-cell polytope is triangulated exactly by coning triangulated facets to its vertex barycenter. Selected midpoint bisections give 887 final tetrahedra over all twenty cones. On each, homogenize the polynomial to degree nine in the four barycentric coordinates. **All 220 coefficients are nonnegative in the ordered field.** These are ordinary homogeneous coefficients; dividing by the positive multinomial factors gives the equivalent Bernstein coefficients.

The computation checks 195,140 coefficients, including 55,928 exact zeros. It clears denominators first and works with integers in Z[tau], tau^4-10 tau^2+5=0. Signs are decided by exact nested-square-root comparisons. No floating-point tolerance enters acceptance.

Thus (10) holds on all sign cells, their boundaries, and the whole collar. Its relative mu^2 margin is proved algebraically; no positive minimum mu is assumed.

## 6. The upper collar and universal strictness

Swapping duplicate labels 0 and 10 preserves the aggregate G. For an individual response it gives

    c'_k = w_0 - G_(k,0) - c_k,
    w_0 = integral_F x dnu.

After a cancelling relation, c'=w_0-c. Since v(D)-v(-D)=w_0 dot D, a strict certificate for -D gives one for D. Normalization sends mu to 1-mu. Consequently the lower collar certificate covers 9/10<=mu<1 as well.

Together with (6), every normalized genuinely splitting direction has a strict finite positive-section certificate. For example, the constructed response in closure(C) has a gap at least

    min { 10^-6, 10^-3 mu^2, 10^-3(1-mu)^2 } > 0.           (14)

The minimum is a convenient global lower bound, not a claim that each construction is used outside its stated domain. Strictness permits approximation by a response in C, losing at most half the gap.

## 7. The final Hausdorff argument

Suppose the theorem fails. Choose improving K_n converging to H, with at most eleven facets. Their polars converge to H^polar and have at most eleven vertices. Each of the ten distinct exposed vertices of H^polar forces a nearby polar vertex; these can be labelled as A_n. There is at most one extra vertex b_n.

With no extra vertex, (1) applies. An extra vertex limiting to the polar interior or an open positive-dimensional face is excluded by the retained completed neighborhood theorems. Thus, after a subsequence and symmetry, b_n tends to a_0.

Redundant configurations reduce to ten genuine facets. Otherwise normalize a genuine split as above. The endpoint theorem excludes limiting mu=0 or 1. After a subsequence D_n tends to a genuinely splitting D. Cutting loses O(t_n) volume, so monotonicity and (1) give

    0<S_n<=-kappa_1 r_n+C t_n,
    hence r_n=O(t_n).

Choose a strict finite positive-section certificate for the limiting D, now supplied by Sections 3--6. It has a fixed positive margin m and is valid at nearby D_n. Its sections have one common local feasibility radius because the list is finite and the base weights are positive. Average (4):

    S_n <= -m t_n+O((r_n+t_n)^2)
        <= -m t_n+O(t_n^2)<0

for large n, a contradiction. This proves one Hausdorff neighborhood for all polytopes with at most eleven genuine facets. Arbitrary nonlinear paths and all coupled relative scales are included; there is no raywise-to-uniformity shortcut.

## 8. What is assumed, checked, and not claimed

- **Accepted mathematical input:** the user's sharp F=10 theorem and local symmetry chart. It is not re-proved by this packet.
- **Retained mathematical dependencies:** all other-face neighborhoods and the endpoint theorem. Their full proof texts and finite-data checks are included; historical residual sections are explicitly superseded by this proof.
- **New checks:** every core row, cancellation relation and covering leaf; every collar weight path and polynomial cancellation identity; complete sign-cell geometry and every nonnegative polynomial coefficient.
- **Continuum arguments supplied here:** the closure correction, convex core interpolation, planar integral formula, polynomial positivity implication, copy interchange, and final contradiction argument.
- **Not claimed:** proof-assistant formalization, external review, an explicit Hausdorff radius, an exact formula for the actual capacity derivatives, equality classification for all eleven-facet bodies, or any F>=12/unrestricted-body result.

### Primary mathematical sources

[HK] P. Haim-Kislev, *On the symplectic size of convex polytopes*, Theorem 1.1 and Remark 1.4, arXiv:1712.03494v3, https://arxiv.org/html/1712.03494v3 . The word convention here puts the earlier normal first; reversing the ordering reconciles conventions without changing the maximum.

[HKO] P. Haim-Kislev and Y. Ostrover, *A Counterexample to Viterbo's Conjecture*, arXiv:2405.16513v3, https://arxiv.org/html/2405.16513v3 . This supplies the benchmark capacity/value, not the new local theorem.
