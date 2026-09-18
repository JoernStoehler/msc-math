# Current-source note: Lagrangian products, HKO, and the local theorem

Prepared 2026-09-16 as candidate evidence for Sol’s literature rewrite. This is a bounded source check, not a complete literature review.

## Primary sources and version pinpoints

- Alexey Balitskiy, Ivan Mitrofanov, Alexander Polyanskii, *Triangle covering problems and the Viterbo inequality in the plane*, arXiv:2603.12495v2 (19 August 2026), [arXiv record](https://arxiv.org/abs/2603.12495), [v2 HTML](https://arxiv.org/html/2603.12495). The record explicitly identifies v2 as the latest version and says the changes are minor referee-related changes.
- Pazit Haim-Kislev, *Dynamical extensions of Zoll to nonsmooth convex bodies*, arXiv:2511.16644v1 (20 November 2025), [arXiv record](https://arxiv.org/abs/2511.16644), [HTML](https://arxiv.org/html/2511.16644). Only v1 is listed.
- Pazit Haim-Kislev and Yaron Ostrover, *A Counterexample to Viterbo’s Conjecture*, arXiv:2405.16513v3 (20 November 2025), [v3 HTML](https://arxiv.org/html/2405.16513). This is the source for the explicit regular-pentagon counterexample and its capacity calculation.

Throughout, in dimension four the source’s systolic ratio is
\[
 \operatorname{Sys}(A)=c_{\rm EHZ}(A)^2/(2\operatorname{Vol}(A)).
\]
Thus “excluded from \(\operatorname{sys}>1\)” below means that the cited theorem proves the Viterbo inequality \(\operatorname{Sys}\leq1\) for that class, not that every member has strict ratio below one.

## Which planar product factors are ruled out?

The useful current result is B–M–P v2, Theorem 1.2 (HTML lines 92–99): for every convex shape \(K\subset\mathbb R^2\), and for \(Q\subset\mathbb R^2\) either (i) an arbitrary convex quadrilateral or (ii) an affinely regular hexagon,
\[
 \operatorname{area}(K)\operatorname{area}(Q)\geq \tfrac12 c(K\times Q)^2.
\]
Consequently every planar Lagrangian product with a quadrilateral second factor satisfies \(\operatorname{Sys}(K\times Q)\leq1\). The proof is Section 5, especially Theorem 5.1: after affine normalization to a quadrilateral with perpendicular unit diagonals, every \(Q\)-cover has area at least 1, with the unit square giving equality (HTML lines 476–485). This includes degenerations described in Section 5.2: triangles (the \(a=1\) boundary), parallelograms, trapezoids, and generic quadrilaterals (HTML lines 557–574).

The same paper’s triangular case is Section 4, with the area/action argument at HTML lines 378–403 (the constant action is \(1/(2\operatorname{area}Q)\)); the resulting Viterbo inequality for triangular \(Q\) is stated in the discussion at lines 401–405. Hence triangular factors are also ruled out from \(\operatorname{Sys}>1\). Do not present this as a newly conjectural triangle claim: it is proved in that source. B–M–P’s headline theorem, Theorem 1.2, packages the quadrilateral and affinely regular hexagon classes, while the triangle proof is the earlier warm-up in Section 4.

The exclusion is only about a planar factor in the ordinary Lagrangian product \(K\times Q\). It does not say that arbitrary higher-dimensional products or \(p\)-products have ratio at most one. HKO v3 defines the symplectic \(p\)-product separately (HTML lines 102–108), and uses equal-capacity symplectic 2-products to propagate a ratio \(>1\) to higher dimensions (lines 111–114). That operation is not the same statement as a planar quadrilateral factor in \(K\times Q\).

## What HKO actually establishes

HKO v3, Proposition 1.4 (HTML lines 92–100), takes \(K\) to be the regular pentagon in the \(q\)-plane and \(T\) its 90-degree rotation in the \(p\)-plane. It proves
\[
 c_{\rm EHZ}(K\times T)=2\cos(\pi/10)(1+\cos(\pi/5)),
\]
by identifying a shortest 2-bounce trajectory along a pentagon diagonal. Substitution into the definition of \(\operatorname{Sys}\) gives
\[
 \operatorname{Sys}(K\times T)=(\sqrt5+3)/5>1
\]
(HKO v3, proof of Theorem 1.3, HTML lines 109–114). This is the counterexample relevant to the thesis’s ten-facet body: a product of two pentagons in relative 90-degree position, with ten total facets in dimension four.

HKO v3 also says (HTML lines 99–100) that the Reeb trajectories through an open dense subset of the boundary are closed. That is a dynamical observation, not a statement that the entire boundary is foliated by action-minimizing closed characteristics, nor a Zoll theorem.

The current Haim-Kislev Zoll paper is especially important for wording. Its Example 1.12 (HTML lines 203–215) records the HKO product \(P\times T\) with \(\rho_{\rm sys}=(\sqrt5+3)/5\), reports numerical evidence that it is a cuts extremizer but not cuts additive, and states explicitly: “We do not know whether it is generalized Zoll.” Proposition 1.13 only proves cuts additivity near the boundary for specified normals \(u=(u_p,u_q)\), with \(u_p\in N_P(v_i)\) and \(u_q\in N_T(w_j)\) for \(j\in\{i-1,i,i+1\}\), including coordinate-type normals (HTML lines 212–214). It is not all-direction cuts additivity.

For context, the same paper defines cuts additive by equality \(c_{\rm EHZ}(K)=c_{\rm EHZ}(K_1)+c_{\rm EHZ}(K_2)\) for every hyperplane cut (Definition 1.1, HTML lines 70–78). Theorem A (lines 80–86) proves generalized Zoll plus well-defined characteristic dynamics implies cuts additive; Theorem B (lines 119–126) gives the converse only under the additional contractibility hypothesis for every cut-direction/height space \(\Lambda_{v,t}\). These hypotheses matter: the HKO example is not certified by the paper as generalized Zoll, and its numerically suggested cuts-extremizer property does not supply cuts additivity.

## Relation to the thesis ten-facet local-max theorem

The thesis theorem is a different claim and should remain explicitly scoped: it proves a Hausdorff-local maximum of the HKO systolic ratio in the stratum of polytopes with exactly ten facets, modulo the stated ratio-preserving symmetries. It does not follow from HKO’s counterexample, from the open-dense closed-orbit observation, or from Haim-Kislev’s partial near-boundary cut result. Nor does it assert local maximality among all convex bodies, among products with a changing facet count, or any Zoll/cuts-additivity property of the HKO body.

## Placement and uncertainty

Introductory mathematical context can safely state: the EHZ capacity of a Lagrangian product is linked to Minkowski billiards; Viterbo’s inequality is true for planar triangular and quadrilateral factors (and affinely regular hexagon factors) by B–M–P v2; and the rotated regular-pentagon product is an explicit \(\operatorname{Sys}>1\) counterexample by HKO v3.

Claims about cuts, generalized Zoll, open-dense closed Reeb trajectories, or numerical dynamical behavior belong in the specialized interpretation/DS discussion unless the precise hypotheses and theorem status are given. In particular, do not turn “open dense set of closed trajectories” into “Zoll,” and do not turn the thesis’s ten-facet local maximum into a global or facet-count-free result.

Remaining uncertainty after this check: B–M–P v2 says the smallest-cover shape for the regular pentagon example “seems likely” but is not proved (HTML lines 588–596), and Haim-Kislev v1 leaves generalized Zoll status of the HKO product open (lines 203–211). The note does not infer broader open status from absence of a search hit.
