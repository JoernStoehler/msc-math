# Independent audit: repeated-word elimination

## Conclusion

The proposed elimination lemma is valid under its stated hypotheses and the
polar-loop/KKT reconstruction cited in `multicut/theorem.md`. It gives a
response-specific reduction: for a facetwise convex profile invariant along
each facet's characteristic direction, no maximizing word with repeated
facet visits has larger occupation response than every simple maximizing
word. I found no gap in the weight perturbation, displacement, or endpoint
argument.

## Main checks

1. **Feasibility and objective.** The two occurrences have the same full
   normal, so \(\beta(z)=\beta+z(e_r-e_s)\) preserves both closure and total
   mass on the closed interval \([-\beta_r,\beta_s]\). The quadratic
   coefficient in \(z\) vanishes. On the open interval the weights remain
   positive; the repeated-word polar-loop bound applies there and at the
   endpoints. Since \(z=0\) is an interior maximum with value \(Q_*\), the
   affine function \(Q(\beta(z))\le Q_*\) is constant at \(Q_*\).
   The objective need not be constant before using this maximality argument.

2. **Reconstruction and rank.** Positive \(Q_*\) requires occurrences from
   both complementary Lagrangian factors. Closure separates across the two
   planes. With no antipodal normal pair, a positive closed combination in
   either plane has at least three distinct directions and spans that plane.
   Thus active normals span \(\mathbb R^4\). If
   \(v\cdot a_i+s=0\) on the positive support, summing with the positive
   weights gives \(s=0\), and then spanning gives \(v=0\); the augmented
   columns have rank five. The KKT multiplier is consequently unique and
   affine in \(z\), since the active matrix is fixed and \(\nabla Q\) is
   affine in the weights. The reconstruction formula therefore gives
   affine vertex paths on the open interval.

3. **Block displacement.** For a repeated normal \(a=(n,0)\), write
   \(v=2TJa=(0,v_p)\), with \(v_p\ne0\). The starting-vertex derivatives
   are \(u\) outside the interval between occurrences and \(u+v\) inside.
   Differentiating each active facet equation is legitimate because the
   reconstructed segment lies in that facet. Every active first-factor
   normal therefore annihilates \(u_q\); since those normals span, \(u_q=0\).
   Active second-factor normals in the two intervals respectively annihilate
   \(u_p\) or \(u_p+v_p\). There are at least three distinct second-factor
   directions, so one interval contains two distinct, hence nonparallel,
   directions. Its equations force \(u_p=0\) or \(u_p=-v_p\). Therefore
   \(u\) and both interval displacements are parallel to \(Ja\). The same
   reasoning applies with the factors interchanged.

4. **Response convexity.** Both duplicated segments have the same facet
   label. Their positions, endpoints, and changing lengths vary only along
   \(Ja\), so line-invariance fixes their profile values; their combined
   contribution is affine in \(z\). Every other segment has fixed weight
   and is translated affinely in \(z\). At a fixed relative segment
   parameter, its position is affine in \(z\); convexity of its facet
   profile makes its integral convex. Thus the whole response \(R(z)\) is
   convex on the open interval.

5. **Endpoints and iteration.** The affine vertex paths have limits at the
   endpoints. Retained segments remain on their facets and in \(K\) by
   closedness; the disappearing segment has vanishing mass. Continuity of
   each profile on the compact facet gives the endpoint response of the
   word with zero weights deleted. Both endpoint words still have objective
   \(Q_*\). Convexity yields
   \(R(0)\le\max(R(-\beta_r),R(\beta_s))\), so an endpoint deletes at least
   one occurrence without lowering response. Repetition terminates after
   finitely many deletions at a simple maximizing word.

Because the disjoint union of labeled facets is compact and the profile is
continuous there, its integral is continuous under weak convergence. The
finite-word domination therefore passes to the weak closure of maximizing
occupation measures and proves the stated supremum identity.

## Scope and remaining work

The conclusion is specific to profiles invariant along each facet's own
characteristic direction; it does not identify all maximizing measures or
prove arbitrary convex-order domination. It reduces the radial-hinge
response search to simple maximizing words. The separate classification or
complete optimization over all simple words and their maximizing weights
remains necessary; the cited 710 sampled charts are not a completeness
certificate. The proof also relies on the established repeated-word
polar-loop bound and KKT reconstruction; this audit does not reprove those
capacity inputs.
