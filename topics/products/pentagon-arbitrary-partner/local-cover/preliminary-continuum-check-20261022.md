# Preliminary continuum check, 22 October 2026

Private root-facing checkpoint from `/root/msc_today_scope` (Astra), within the newly admitted ten-minute source investigation starting 08:42:37 UTC and ending no later than 08:52:37 UTC. This is a bounded mathematical inspection, **not overall proof approval**, a new experiment, or manuscript promotion. Source baseline: `56e408494789eb49dd6092dadfafaa7199576cbd`. Worktree: `/workspaces/msc-math-local-cover-prelim-20261022`. Shared Backlog: `w-806e05193a0a4ab1a57146a636272741`.

## Partial verdict

I found no concrete counterconfiguration to the two inspected continuum implications. The loop estimate can be decomposed into explicit, checkable local geometry lemmas; the placement-to-body implication follows by compactness once the stated unique-placement premise and local placement inequality are granted. There is one notation repair in the anchoring step: translate the hull **and its containing body** together. This is not evidence that the theorem is false.

This inspection does not independently certify the finite rank/positivity computation, template completeness, the external capacity theorem, or the assembled local theorem. It supplies a sharper next review target than “review all the continuum proof.” No numerical producer, build or manuscript edit ran.

## 1. Local loop estimate: a checkable decomposition

Source: [local-cover-rigidity.md, section 1](../local-cover-rigidity.md#1-a-local-polygonal-lower-bound-lemma), especially the crossing/deletion claim at lines 58–73. Let H have fixed strictly convex counterclockwise vertices v_i, and let the chosen incoming/outgoing points a_i,b_i lie in the radius-r disk about v_i. The loop alternates the short segments a_i b_i and long segments b_i a_(i+1).

**Crossing localization.** Every long segment lies within distance r of its original side by endpoint interpolation. Nonadjacent original sides have a positive separation, so their perturbed segments remain disjoint for small r. A vertex and a nonincident original side are likewise separated. Adjacent side directions have a fixed nonzero determinant; solving their nearby supporting-line intersection therefore puts any crossing of the two adjacent long segments within C_H r of their common vertex, with C_H independent of the endpoint choices. Thus vertex disks of radius C'_H r can be chosen disjoint. On each long side, a crossing near its initial vertex precedes one near its final vertex because its limiting length is positive.

Within a vertex disk the only potential proper crossing is between its two long segments. The connector shares an endpoint with each of those segments, so it cannot properly cross either in general position. A nonincident connector/edge is excluded by the separations above. For a triangle, all pairs of long segments are adjacent; the same three distinct vertex disks handle them.

**Deletion and error.** If the two long segments cross at p_i, the consecutive subloop p_i → a_i → b_i → p_i is a triangle contained in the C'_H r disk. Delete that subloop, replacing the route by passage through p_i. Its signed area has absolute value at most C''_H r². Different deletions occur in disjoint disks and in the correct order on shared long sides, so they do not create additional intersections. The resulting polygon has no proper crossings in general position. Every new vertex p_i belongs to the convex hull of the selected points, since it lies on their segments.

**Orientation, the compressed point in the source.** Fix o strictly inside H. For r smaller than the distance from o to the boundary, straight-line interpolation of each selected vertex back to its v_i gives a homotopy of the original loop that avoids o: each long edge stays close to its original side and each short edge stays close to its original vertex. Its winding number about o is therefore +1. Removing triangle subloops in vertex disks disjoint from o preserves that number. The simple remaining polygon is consequently counterclockwise. Its area is at most the area of the selected-point hull, hence at most A(P(u)). Adding back at most n signed triangles gives

`A(P(u)) >= signed_area(original loop) − n C''_H r²`.

Coincidences and overlaps can be obtained by perturbing the finite labelled point configuration into general position and taking a limit. The constants arise from the fixed polygon's separation, lengths and adjacent determinants, so they remain uniform in this limiting process. If an incoming/outgoing choice is the same label, use the corresponding zero connector as a limit; no independence of the physical displacements is needed for the inequality.

**What this establishes at this checkpoint.** It identifies a plausible complete local topological justification for the source's compressed “simple counterclockwise polygon” step, including a reason the orientation survives. It does not use the false global rule that every arbitrary self-intersecting polygon has signed area bounded by its hull. An independent review should explicitly check the general-position deletion argument and its limiting passage against the above uniform constants before upgrading the source's status.

I also checked the linear shoelace coefficients symbolically: for J counterclockwise, variation at a_i gives `J(v_(i−1)−v_i)/2`, and variation at b_i gives `J(v_i−v_(i+1))/2`, matching the note. The shoelace remainder is quadratic, and finitely many choices permit a common remainder bound. No arithmetic certificate was rerun.

## 2. From placements to arbitrary nearby convex bodies

Source: [local-cover-rigidity.md, section 3](../local-cover-rigidity.md#3-from-placements-to-arbitrary-nearby-convex-bodies), lines 138–160.

**Unique placement premise.** If two template points have difference d that is an exposed vertex of H−H, choose a functional exposing just d. A placement of those points in H attains equality in the difference-body support bound. The maximizing and minimizing faces of H for that functional must both be singletons: their Minkowski difference is the singleton exposed face {d}. Therefore both endpoints, and hence the translation of the whole fixed template, are forced.

The [checker source](verify_local_cover.py), lines 61–73, explicitly constructs the hull of H−H, selects a triangle endpoint pair whose difference or negative lies among its vertices, and checks their placed endpoints belong to H. The [retained output](verification/20260924.json) lists ten pairs. I inspected those source predicates and the retained result; I did not independently recompute the geometry or execution identity.

**Uniform compactness lemma.** Given finitely many fixed templates T_j with unique translations t_j* into H, for every ε>0 there is δ>0 such that any compact convex K with Hausdorff distance less than δ from H and any placements T_j+t_j ⊆ K satisfy `max_j |t_j−t_j*| < ε`. Otherwise take K_l→H and offending placements. Passing to a subsequence fixes an offending j. A fixed template vertex and boundedness of K_l bound t_j(l). A convergent subsequence tends to t; closed containment under Hausdorff convergence gives T_j+t⊆H, contradicting uniqueness. Thus this conclusion holds for **every** choice of feasible placements, not merely a specially chosen continuous branch.

**Anchoring and inclusion.** Put `a = t_0−t_0*` and `u_j = t_j−t_j*−a`, so u_0=0. The anchored placement hull is `P' = conv union_j(T_j+t_j*+u_j)` and satisfies `P' ⊆ K−a`, rather than necessarily `P' ⊆ K`. Areas are translation invariant, so the local placement inequality gives

`A(K)=A(K−a) >= A(P') >= m + c ||u||`.

Equivalently, apply the inequality to the unshifted hull after using translation invariance. The source's shorthand “remove their common translation … P subset K” should name this simultaneous shift to avoid an incorrect literal inclusion. Uniform smallness of u follows from the compactness lemma and `|u_j| <= |t_j−t_j*|+|t_0−t_0*|`.

If A(K)=m, the strict placement inequality forces u=0. Then H+a⊆K; equality of areas for nested planar convex bodies with interior forces K=H+a. If a proper inclusion existed, an exterior point and an interior disk of the smaller body would give a positive-area region in the difference.

**Capacity normalization boundary.** The source uses continuity and degree-one homogeneity of c(K×Q) under scaling K, plus c(H×Q)=1 and the complete template-cover criterion. Under those inputs, K/c(K×Q)→H and capacity becomes one; area scales quadratically, preserving c²/(2A(K)A(Q)). This supports the stated homothety equality transfer. The repository [research charter](../research-charter.md#evaluator-contract) states the criterion and normalization convention. I did not recheck the external theorem or continuity hypotheses in this preliminary; this remains an explicit imported interface rather than a newly established result.

## Stop and next review point

Checkpoint recorded at 2026-10-22T08:47:58.590239+00:00.

The preliminary is complete at this checkpoint. No concrete fatal gap was found in sections 1 or 3, and the original theorem remains a research candidate. The useful next bounded action, **if selected**, is independent review of the explicit crossing/deletion/winding lemma above and the finite-certificate-to-placement interface, followed by a separately scoped assembled proof review. Do not launch another producer or call the local theorem accepted on the basis of this note. The global arbitrary-partner optimum, numerical exclusion radii, section 4's buffer, novelty and manuscript inclusion were not reviewed.
