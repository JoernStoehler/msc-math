# Fresh mathematical audit: preferred triangle–triangle example

Reviewer: `/root/affine_hko_math_audit`, 28 October 2026, 23:31–23:37 UTC. Receiving/integration owner: root.

## Verdict

The assembled theorem and proof in `triangle-position-section.tex` pass this audit. For K=Δ×Δ, the sharp maximum among all positions with both original factor planes Lagrangian is 2/3, while the displayed rational affine image has ratio 18/25. The proof therefore rules out every Lagrangian position as a full-affine maximizer. It applies, by affine equivalence preserving the distinguished factor planes, to every product of two nondegenerate triangles.

No substantive mathematical correction remains. This is the preferred smaller example; the independently validated triangle–quadrilateral example remains preserved in `MATHEMATICAL-AUDIT.md` as a fallback.

Reviewed frozen source files, under `/tmp/msc-math/restart/geometry-frontiers/triangle-quadrilateral/`:

- `triangle-position-section.tex`: SHA-256 `4394e566543e67deb107df7eff9904f728b2199bdf15925930ffdc943d843483`.
- `exact_triangles_compact.py`: SHA-256 `8642702942c0c9e4ddb59ff44e837d45bd7d7c2063a7f1ea04bac97591f7b6f7`.

I read both complete files, independently checked their mathematical implications and correspondence, and replayed the compact verifier from a copy in this audit directory. The output is `exact-triangles-compact-summary.json`. I also replayed the larger `exact_triangles.py` independently and inspected its exact vertex list, and ran the separate integer coefficient-table script. The underlying published HK formula is imported with the normalization/order bridge checked in my earlier HKO audit; this report does not re-prove that published theorem or establish priority.

## Complete capacity calculation

Each triangle has area 9/2, so the product volume is 81/4. Closure of the six normalized normals forces three identical weights t/3 in the first factor and three identical weights (1−t)/3 in the second. Thus every feasible weight vector lies on exactly the stated segment. Its endpoints cover all possible smaller supports.

For any closed weighted-normal list, cyclic rotation preserves the ordered action. Hence 120 orders beginning with facet 0 exhaust all full permutations, including endpoint cases where some weights vanish. Inserting zero weights covers distinct-facet subset words. Reversal handles the HK source's opposite ordered-pair convention.

For the chosen form x=(1,1,−2,1,1,−1), the coefficient table was independently recomputed using integer pairings. The four coefficient pairs (a,b)=(1,1),(1,−1),(−1,1),(−1,−1) each have 30 words, and their maximal mixed coefficients are respectively 4,6,6,4. Because t(1−t)≥0, these are genuine pointwise envelopes of every word in their groups. The corresponding four quadratics, after division by 9, have maxima

`1/6, 5/27, 5/27, 1/18`,

at `t=1/2, 2/3, 1/3, 1/2`. Each table row names a word attaining its envelope. Therefore the complete support value is exactly 5/27, not just bounded below by an exhibited word.

I independently checked the explicit inverse of the displayed A:

`A⁻¹ = [[1,0,0,0],[0,0,1,0],[−1,1,1,0],[−1,0,−2,−4]]`.

Its row pairings under J give precisely the six stated form coordinates. Also det A=1/4 and `P=1*1−1*(−1)−(−2)*1=4`. Consequently

`c(AK)=27/10`, `Vol(AK)=81/16`, and `ρ(AK)=18/25`.

The first two columns of A have symplectic pairing 1/4, and the last two have pairing −1/4. Thus both factor planes of this explicit witness are indeed non-Lagrangian.

## All Lagrangian positions, not a selected family

For every invertible B, `BᵀJB=−Ω⁻¹` when `Ω=B⁻¹JB⁻ᵀ`. A nondegenerate skew matrix has zero diagonal 2×2 blocks if and only if its inverse does. Hence both original factor planes are Lagrangian exactly when Ω is crossblock. Conversely, every invertible crossblock M is realized explicitly by `B=diag(M⁻¹,I₂)`. There is no restriction to a rigid rotation or a fixed identification of the two factors.

The general ratio formula in this subclass is `|det M|/(162h(M)²)`. The absolute value correctly includes orientation-reversing maps. Same-factor pairings vanish, so global word maximization reduces to t=1/2, after reversal supplies the positive sign. This makes the finite cross rows the exact support function, rather than merely a feasible lower bound.

The normalization C={h≤1/12} is correct. At equal weights 1/6, the coefficient of a cross pairing is 1/36; multiplying by 12 gives the verifier's row coefficient 1/3. Its 120-word loop constructs precisely these rows. Rational PPL reports a bounded four-dimensional polytope, and the verifier asserts exact equality of its vertices with

`{±(3/2)u_i u_jᵀ : i,j=1,2,3}`, where `u₁=e₁`, `u₂=e₂`, `u₃=e₁−e₂`.

I independently matched the 18 vertices printed by the larger replay to this set. The compact replay verifies this identity directly, without the previous face-stationarity calculation.

## Analytic determinant bound

The Cauchy–Binet argument in the final proof is correct. Group a convex combination of the signed rank-one generators by its first index. This gives

`M=(3/2)Σ u_i v_iᵀ`, with `v_i∈α_iD`, `α_i≥0`, `Σα_i=1`,

where D=conv{±u₁,±u₂,±u₃}. Every pair of these hexagon vertices has determinant of absolute value at most one; bilinearity and convex combinations then give `|det(v_i,v_j)|≤α_iα_j`. Since all `|det(u_i,u_j)|≤1`, Cauchy–Binet yields

`|det M|≤(9/4)Σ_{i<j}α_iα_j≤3/4`.

The final inequality follows from Σα_i²≥1/3. Equality occurs for `v_i=u_i/3`, giving `M_*=[[1,−1/2],[−1/2,1]]` with determinant 3/4. Membership in C follows directly from its convex-combination expression. Its support equals 1/12, since a smaller value would permit radial enlargement within C and violate the determinant bound.

Rescaling arbitrary nonzero M to `M/(12h(M))∈C` therefore gives

`ρ≤(144/162)(3/4)=2/3`,

with equality attained by the explicit invertible M_*. This proves the sharp global Lagrangian comparison.

## Attainment, affine equivalence, and limits

The action support is a norm by the previously audited action-body lemma. Maximizing |Pf| on its compact unit ball gives a positive maximum, so every maximizing form is nondegenerate and realized by an invertible affine linear part. Thus a full-affine optimum actually exists. The strict comparison 18/25>2/3 excludes all Lagrangian positions from that optimum; it does not determine the optimum's value.

Any two nondegenerate triangles are affine images of Δ. Their product equivalence has block-diagonal linear part and a translation, and therefore preserves the two distinguished factor-plane directions. It identifies both the full affine orbit and the subclass with Lagrangian factor planes. The final theorem's extension to arbitrary triangle pairs is consequently justified, including triangles not centered at the origin.

Both displayed ratios are below one. No violation of Viterbo's inequality, optimality of 18/25, or literature priority is established or claimed. The paper's prior-work comparison was read for scope; its external novelty search was not independently repeated in this mathematical audit.
