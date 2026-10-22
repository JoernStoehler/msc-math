# Journal-paper concept: exact capacities of polygon products

**Status:** editorial concept based on inspected repository sources and a bounded primary-literature search (22 October 2026). This is a candidate paper architecture, not a novelty clearance, proof acceptance, or journal-readiness claim.

## Recommended article

**Working title:** *Difference-body formulas for EHZ capacities of Lagrangian polygon products*

The paper should tell one story: the capacity of a polygon product is governed by a four-block width-body model, except when sparse six-block trajectories contribute an obstruction. It then proves when that obstruction disappears, first for all relative rotations of odd regular polygons and then, in stronger matrix-space forms, for independent affine deformations of the 5×5, 5×7, 7×7 and 5×9 pairs. Each theorem resolves the same question: when does the geometrically interpretable width formula give the full EHZ capacity?

A second article should treat local extremality of the HKO product among fixed-facet polytopes. That is a strong result with a different question, proof method, and reader: local variation, feasible upper sections, symmetry quotients, and exact certificates. The non-HKO ten-facet local maximum belongs naturally there as a second extremum theorem. Keeping the papers distinct gives both a clear mathematical question; this is a portfolio choice, not a recommendation to focus the project on one claim.

## Proposed abstract

For planar convex polygons A,B in complementary Lagrangian planes, the Ekeland–Hofer–Zehnder capacity of A ×_L B reduces to finitely many sparse facet words. Four-block words depend only on the polar difference bodies (A−A)° and (B−B)°; six-block words form the remaining obstruction. We determine this obstruction for several substantial polygon classes. For every pair of odd regular polygons P_n,P_m, with n,m ≥ 5, we prove the exact rotation profile

    c_EHZ(P_n ×_L R_θ P_m)
      = (1+cos(π/n))(1+cos(π/m)) sec d_L(θ),

where L=lcm(n,m) and d_L(θ) is the distance to the nearest multiple of π/L. Except for the 5×5 edge-contact case, strict containment of the six-block terms yields an open labelled neighborhood on which the width-body formula holds at every angle. We also prove full matrix-space containment for independently linearly deformed pairs of regular polygons of types 5×5, 5×7, 7×7, and 5×9. These give exact capacity formulas in all four affine families. For 5×5 and 7×7 the formulas yield sharp systolic ratios, with explicit equality classes, throughout the corresponding independent-affine families. No sharp affine ratio is claimed for 5×7 or 5×9. The proofs combine the polytope capacity formula, a sparse product reduction, geometric control of triangle coefficients, and exact finite convex-containment certificates. These results concern specified product classes and do not imply a bound for arbitrary polygons or arbitrary four-polytopes.

**Abstract caveat:** avoid “first,” “new,” “classification,” or “complete solution” until novelty checking and proof review are finished. The HKO benchmark is external and must be credited, not presented as this project’s discovery.

## Proposed principal theorem statements

Use P_j for the circumradius-one regular j-gon, h_j=cos(π/j), and n_i^(j)=(cos(2πi/j), sin(2πi/j)). For invertible planar maps G,H, the main results can be stated compactly as follows.

1. For all odd n,m≥5, with L=lcm(n,m),

       c_EHZ(P_n ×_L R_θ P_m) = (1+h_n)(1+h_m) sec d_L(θ),
       d_L(θ) = min over k in Z of |θ−kπ/L|.

   For all pairs except 5×5, six-block terms lie strictly inside the concentric disk of radius equal to the four-block hull’s inradius. The strict gap gives an open labelled neighborhood where the width-body formula holds uniformly over θ.

2. For independently affine pentagons A=a+GP_5 and B=b+HP_5, put M=G⁻¹H⁻ᵀ and m(M)=max over i,j of |(n_i^(5))ᵀ M n_j^(5)|. Then

       c_EHZ(A ×_L B) = (1+h_5)²/m(M),
       sys(A ×_L B) ≤ (3+√5)/5.

   Equality holds exactly when M=t R_(π/10) U for t>0 and U in the orthogonal dihedral symmetry group of the regular decagon conv{±n_i^(5)}. This is global within the independent-affine pentagon family only.

3. For each (n,m) in {(5,7),(7,7),(5,9)}, let P_n,P_m be the regular factors, let M=G⁻¹H⁻ᵀ, and put m_nm(M)=max over i,j of |(n_i^(n))ᵀ M n_j^(m)|. Then

       c_EHZ((a+GP_n) ×_L (b+HP_m))
         = (1+h_n)(1+h_m)/m_nm(M).

   The exact tensor certificates establish T₆(P_n,P_m) ⊂ T₄(P_n,P_m) in each case. The 5×7, 7×7, and 5×9 packets check all 840, 2,352, and 1,800 six-block tensors, respectively. No sharp systolic optimum for the 5×7 or 5×9 affine family is claimed.

4. For n=m=7, the formula above gives the sharp bound

       sys(GP_7 ×_L HP_7) ≤ (1+cos(π/7))³/[49 sin²(π/7) cos²(π/7)].

   Equality holds exactly when M=t R_(π/14) U, with t>0 and U an orthogonal dihedral symmetry of D₇=conv{±n_i^(7)}. This is global within the independent-affine 7×7 family. Its proof is an area comparison after normalizing m₇₇(M)=1; it does not use an optimizer search.

These theorem statements should be followed by precise hypotheses, definitions, and proofs in the paper. In particular, do not state that the all-odd angular theorem establishes independent-affine containment for all odd pairs.

## Introduction: opening draft

Let A and B be planar convex polygons, placed in the q- and p-planes of standard symplectic four-space. A natural question is whether the Ekeland–Hofer–Zehnder capacity of their Lagrangian product A ×_L B is determined by the widths of the two factors. Widths are accessible geometric data: they combine into the polar difference bodies (A−A)° and (B−B)°.  Yet the capacity is defined through closed characteristics on the four-dimensional product, and its polytope formula ranges over ordered facet words. The short words that produce the width expression need not, a priori, dominate every longer word.

For planar polygon factors, a sparse reduction makes the issue precise. Some capacity-maximizing word uses at most three facets from each factor. After grouping consecutive facets from the same factor, only four-block and six-block words can contribute. The convex hull of the four-block coefficients is determined exactly by the two polar difference bodies. Six-block coefficients retain the order-sensitive interaction of two closure triples; they are the complete obstruction to the width formula. Our results identify families for which these coefficients lie inside the four-block hull, and show that in several cases the containment persists after independent affine changes of the factors.

The first result gives the exact capacity at every relative angle for every pair of odd regular polygons with at least five sides. A geometric circle-deficit estimate places each six-block coefficient inside the concentric disk whose radius is the inradius of the regular polygon formed by the four-block coefficients. Strict containment, except at the 5×5 edge-contact case, also gives an open labelled neighborhood where the width formula remains true uniformly over all relative angles. In full matrix space, exact convex-containment certificates establish the same reduction for affine 5×5, 5×7, 7×7, and 5×9 pairs. The affine 5×5 and 7×7 formulas then yield sharp systolic bounds within their respective independent-affine families, with explicit equality classes. We do not obtain a sharp affine ratio for 5×7 or 5×9, nor a general affine theorem for all odd pairs.

The proof has two complementary parts. First, the analytic reduction translates the polytope capacity formula into a finite support problem: four-block terms form the width-body hull, and six-block terms are encoded by rank-one matrix tensors. For rotations, regular symmetry reduces the relevant containment to a radial estimate. A closure triple gives a triangle; translating it to the midpoint of its longest side produces a deficit whenever the third vertex lies strictly inside the diameter circle. A lower bound on the partner steps then turns that deficit into a strict radius bound. Second, the affine cases use two kinds of exact containment proof. For 5×5, explicit convex-combination identities place the six-block tensors in the four-block hull, including the known edge contacts. For 5×7, 7×7, and 5×9, exact interval certificates give strictly positive convex representations of every six-block tensor, hence strict containment. Candidate weights for those certificates may be proposed numerically, but the checks reconstruct exact targets and verify exact identities and positivity; acceptance does not rely on a floating-point hull computation.

Haim–Kislev’s combinatorial formula for the EHZ capacity of a convex polytope supplies the general capacity input. Haim–Kislev and Ostrover’s pentagon product is the motivating example and supplies the established benchmark ratio; our affine pentagon theorem asks whether independent shears and stretches can improve it. The earlier Minkowski-billiard literature gives a complementary geometric interpretation of product capacities. Here the exact evaluations follow from the polytope formula and explicit containment arguments. The results concern the stated polygon families, not arbitrary polygons or arbitrary four-polytopes.

The article proceeds from the sparse reduction and the four/six-block decomposition to the all-odd angular formula, then to the affine tensor-containment results and their systolic consequences. The final section lists the open affine cases and distinguishes strict containment, edge contact, and finite certificate evidence.

## Rewritten proof passage (reader-facing draft)

The six-block terms are the only possible source of a failure of the width formula. For a fixed pair of positive closure triples, choose a cyclic order and write its coefficient as

    2z = x₁ conjugate(y₁) − x₃ conjugate(y₂),
    x₁+x₂+x₃ = y₁+y₂+y₃ = 0.

The three points x₁, −x₃, 0 form a triangle whose side lengths are |x₁|, |x₂|, |x₃|. Translating this triangle does not change z, because the yᵢ sum to zero. If the triangle is right or obtuse, put the origin at the midpoint of its longest side. The two endpoints are then at distance ℓ/2, while the third point is at distance d ≤ ℓ/2. The triangle inequality gives

    2|z| ≤ (ℓ/2)(|y₁|+|y₂|+|y₃|) − (ℓ/2−d) minᵢ|yᵢ|.

The last term records a strict saving: all three partner steps have positive length, so placing one vertex inside the diameter circle lowers the possible coefficient below the no-deficit bound. For acute triangles the circumcircle gives the corresponding bound directly.

For regular n- and m-gons, the four-block coefficient hull is a regular 2L-gon, where L=lcm(n,m), with circumradius ρ=1/[2(1+cos(π/n))(1+cos(π/m))]. Its inradius is ρ cos(π/(2L)). It is sufficient to prove that every six-block coefficient lies in the centered disk whose radius is that inradius. The preceding diameter-circle estimate reduces this to triangle inequalities involving the shortest positive closure steps and the minimum angles of the two step triangles. The all-odd estimates give a strict radial bound inside the inradius except for the 5×5 edge-contact case. This proves the all-angle formula; the uniform strict gap, when present, also persists under small labelled perturbations uniformly in the rotation parameter.

**Editorial note:** this is an exposition draft, not a checked transcription of every constant and case split in odd-regular-angular-width.md. The full scalar inequalities and pentagon-specific cases must be integrated with exact hypotheses and cross-checked before publication. In particular, match the first estimate against the source’s definitions of S, m, r, d and its chosen translation.

## Significance and honest scope

The strongest conceptual contribution is not a new universal capacity formula: Haim–Kislev already supplied one for all convex polytopes. It is the product-specific reduction that isolates all failure of the difference-body formula in six-block terms, followed by exact containment results in continuous rotation families and four full matrix spaces. The all-odd formula is broad and clean; the affine 5×5 and 7×7 theorems raise the problem from one rotation parameter to two arbitrary nonsingular linear maps and give equality cases for sharp ratios; the 5×7 and 5×9 certificates establish heterogeneous affine formulas. Together these results are a substantial exact-geometry paper candidate, while their advance over existing product-capacity literature remains a novelty question.

The local HKO theorem is independently substantial: it proves a Hausdorff-neighborhood local maximum among exactly ten-facet four-polytopes, modulo translation, dilation and linear symplectic maps. The non-HKO local maximum adds a non-strict equality family. These deserve a companion article centered on nonsmooth local systolic extrema. Do not combine them merely to increase theorem count: their proofs and intended readers differ.

## Bounded primary-literature scan (not novelty clearance)

Inspected repository sources: docs/research-guide.md, thesis/README.md, the product research note, all-odd angular proof and audits, affine-pentagon topic README, and the 22 October exact tensor-containment packet for 7×7 and 5×9. The source records explicitly leave novelty open for these extensions.

Primary literature checked online:

- Haim–Kislev, [*On the symplectic size of convex polytopes*](https://arxiv.org/abs/1712.03494), published in *Geometric and Functional Analysis* 29 (2019), Theorem 1.1: general finite combinatorial EHZ formula for convex polytopes. This is foundational input, not a result to claim here.
- Haim–Kislev and Ostrover, [*A Counterexample to Viterbo’s Conjecture*](https://arxiv.org/abs/2405.16513), published in *Annals of Mathematics* 203 (2026): motivating Lagrangian product of two rotated regular pentagons and benchmark.
- Rudolf et al., [*The Minkowski Billiard Characterization of the EHZ-Capacity of Convex Lagrangian Products*](https://doi.org/10.1007/s10884-022-10228-0): prior product/billiard characterization, relevant to how the article motivates its exact polytope reduction.
- Targeted searches for odd regular polygon rotation profiles, all-odd Lagrangian-product formulas, and affine pentagon, heptagon, and 5×9 formulas surfaced the foundational work above but no direct match to the repository’s stated formulas. An earlier bounded search recorded in the repository also found no direct match for the all-odd angular formula or its sharp equal-factor corollary. Search engines and abstract search are not exhaustive; this does not justify a priority claim.

The search also surfaced an author-uploaded 2026 manuscript, [*On Critical Gaps in the Proposed Counterexample to Viterbo’s Volume-Capacity Conjecture*](https://www.researchgate.net/publication/403243315_On_Critical_Gaps_in_the_Proposed_Counterexample_to_Viterbo%27s_Volume-Capacity_Conjecture), which disputes HKO’s independent billiard proof. I have not independently assessed its arguments or publication status. Its stated target is that billiard proof; the capacity calculations proposed here instead use Haim–Kislev’s 2019 polytope formula, whose hypotheses and conventions have a retained source-contract review. This search result is context for accurate literature discussion, not evidence against the present formulas or a submission gate.

## Sources to assemble for a paper draft

- Main reduction and affine 5×5 theorems: topics/products/pentagon-affine-products/research.tex.
- All-odd angular theorem and proof: topics/products/pentagon-affine-products/odd-regular-angular-width.md.
- 5×7 exact containment review: topics/products/pentagon-affine-products/tensor-certificate-review-20261022.md.
- 7×7 and 5×9 exact containment certificates and 7×7 affine ratio proof: topics/products/pentagon-affine-products/tensor-extensions-20261022/report.md and topics/products/pentagon-affine-products/affine-heptagon-sharp-bound.md.
- Imported formula/convention contract: topics/products/pentagon-affine-products/source-contract.md.
- Companion local-extremality article inputs: thesis/chapters/07-hko.tex, the HKO certificate packet, and the accepted at-most-eleven-facet extension record.
