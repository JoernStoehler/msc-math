# Independent mathematical audit: full affine HKO orbit

Reviewer: `/root/affine_hko_math_audit`. Date: 28 October 2026.

## Verdict and boundary

I found no substantive gap in the proposed implication from the exact finite predicates to the full GL(4) inequality and its equality classification. The proof needs one small convention correction: its earlier-first ordered action is not literally Haim–Kislev's displayed ordering. Reversing the word, with its weights, gives the same scalar maximum. This does not change any certificate row or any inequality.

I audited the mathematics independently of verifier success. A separate agent, `/root/affine_hko_exact_replay`, reports an exact replay and five negative controls; its report owns those executable checks. I inspected the complete verifier source and the proof, but did not independently reconstruct all 268 coefficient rows. This audit does not establish priority or imply maximality among arbitrary convex bodies or arbitrary facet perturbations.

Frozen files inspected:

- `PROOF.md`, SHA-256 `1609102cc5226e8cbed6b12e61122dc8cd0e0176394d99c4a195d4c506daea26`.
- `verify.py`, SHA-256 `e0d9b776e5f9b6072e66ea1987aea247c30aae0f385ed89626003bec6a385c59`.
- `certificate.json`, SHA-256 `1c9db567025993decab28c22d1792e1dd9ae326dbb6666d4f05a7a589a8ee9e0`.

All three are in `/tmp/msc-math/restart/geometry-frontiers/affine-hko-certificate/`.

## Imported formula and benchmark

I opened the actual primary statements online, in addition to reading the local convention contract:

- [Haim–Kislev, Theorem 1.1](https://arxiv.org/html/1712.03494v3#Thmtheorem1): full facet permutations, nonnegative weights, normalization by support heights, and reciprocal prefactor 1/2. It does not require general position, simplicity, central symmetry, or smoothness.
- [Haim–Kislev–Ostrover, Proposition 1.4 and preceding coordinates](https://arxiv.org/html/2405.16513v3): the circumradius-one pentagon with a vertex at (1,0), and its rotation by −π/2, have capacity `2 cos(π/10)(1+cos(π/5))`. This is exactly the packet's body, not an unaccounted rotated or reflected variant.

For the first statement, replacing the source weight λ_i by β_i=λ_i h_i gives `Σβ_i=1`, `Σβ_i a_i=0`, and preserves every pairing because `a_i=n_i/h_i`. The displayed source order pairs the later facet first. Reversing the ordered list and carrying the weights gives the packet's earlier-first convention. Missing facets can be inserted with weight zero, and zero-weight facets can be deleted; thus the packet's distinct-facet subset words have precisely the required semantics.

Writing `s=√5` and `t=tan(π/5)`, the positive root satisfies `t²=5−2s`. Also `cos(π/5)=(1+s)/4` and `cos(π/10)=t(3+s)/4`. Therefore the source capacity is `t(5+2s)/2=5t−t³/2`, exactly c₀. The factor area is `(5/2)sin(2π/5)`, whose square is `25(5+s)/32`. Hence `c₀²/(2V₀)=(3+s)/5`. The normal vectors in the verifier are support-normalized facet vectors for these exact pentagons: for example `(1,±t)` meet at the vertex `(1,0)`, and the remaining first-factor normals are their rotations by multiples of 2π/5. The second block is their −π/2 rotation.

## Action body and affine reduction

For a skew matrix Ω, the ordinary six-coordinate dot product with the wedge coefficient is precisely `aᵀΩb`; no additional wedge factor 1/2 is needed. Feasible weights depend only on the linear closure and normalization equations. Under A, the normals become `A⁻ᵀ a_i`, so closure is equivalent and the form in the objective becomes `Ω=A⁻¹ J A⁻ᵀ`.

Word reversal makes the action body W centrally symmetric. Finitely many compact feasible-weight sets, followed by convex hull in finite dimension, give compact W. Its support function is positive on every nonzero skew form: choose weights strictly positive on every facet with zero weighted sum (the facet normals positively span). If every ordering had zero value, an adjacent swap of any pair would force that pair's Ω-pairing to vanish. The normals span R⁴, so Ω would vanish. Thus the normalization `q=h_W(Ω)/q₀` is legitimate.

In coordinate order `(01,02,03,12,13,23)`, `Pf(Ω)=x01*x23−x02*x13+x03*x12`; `Pf(J)=−1`. Consequently `P(Ω)=−Pf(Ω)=1/det(A)`. For positive determinant, volume is `det(A)V₀`, and direct substitution yields

`ρ(AH)=P(Ω)/(8V₀ h_W(Ω)²)`.

Every certified row is a genuine feasible objective divided by q₀. Therefore `B_j x≤h_W(x)/q₀` without any exhaustiveness premise. The proof does not require the finite row hull to equal W, the finite polar C to equal W°, or the witnesses to remain optimizers under A. These are strengths of the argument's actual logic, not missing coverage assumptions.

## Complete projection-polytope argument

The selfdual coordinates expand exactly to

`P(x)=|U(x)|²−|V(x)|²`.

The rational value r=143/200 is simply a valid choice of a containing polytope; no approximation assertion enters the theorem. The exact checks `rD⊂E` and `r²+a²<1` suffice for the following argument.

Here is a full justification of the potentially delicate slice claim. At height `z=aτ`, `0≤τ≤1`, a convex combination of the equator and two outer layers has total outer-layer weight s≥τ. Its planar coordinate lies in `(1−s)E+s rD`. Because `rD⊂E`, this lies in `(1−τ)E+τ rD`: absorb the extra weight `s−τ` into E. The reverse inclusion follows by combining only the equator and upper layer. Reflection gives the negative-height slices.

For `0<τ<1`, the normal fan of this planar Minkowski sum is the common refinement of the two polygon fans. In dimension two its edge-normal directions are their union. Support is `(1−τ)h_E(n)+τ r h_D(n)`. At τ=0 or 1, the extra directions merely give redundant valid inequalities. Hence the 62 displayed inequalities cut out exactly T; this is not inferred from checking that 50 proposed vertices satisfy them.

For each halfspace `h₀+h·y≥0`, the certificate's identity and nonnegative λ with total mass one give

`−h·U(x)/h₀ = Σλ_j B_jx≤1` for every `x∈C`.

All h₀ are positive. This proves `U(C)⊂T` with the correct inequality direction. Every vertex of T has norm at most one, and exactly the equatorial ten have norm one. Strict convexity of the Euclidean ball ensures that its only sphere contacts are those ten points: a convex combination involving different points of the closed ball cannot have norm one.

Thus for `q=h_W(x)/q₀`, `x/q∈C` and

`P(x)/q²=|U(x/q)|²−|V(x/q)|²≤1`.

This is the required bound; it applies to all forms, and therefore in particular to all positive-determinant affine transforms.

## Equality and negative determinant

Equality in the preceding chain forces `V(x)=0` and `U(x)/q` to be an equatorial decagon vertex. Solving these six linear conditions gives zero same-factor entries and cross block

`[[cos(kπ/5),sin(kπ/5)],[-sin(kπ/5),cos(kπ/5)]]`.

Rotating one pentagon factor by multiples of 2π/5 gives the five even-angle forms. Let `F=diag(1,−1)`. The simultaneous reflection `T₋=diag(F,−F)` preserves H: F preserves P, and −F preserves its −π/2 rotation. It sends J to −J under the required inverse congruence. Composition therefore supplies the other five forms. All ten contacts have the form `Ω_T=T⁻¹ J T⁻ᵀ` for an automorphism T of H.

From `Ω_A=qΩ_T`, set `S=√q A T⁻¹`. The order is correct:

`S J Sᵀ=q A Ω_T Aᵀ=A Ω_A Aᵀ=J`.

This is equivalent to the usual linear symplectic condition. As `T⁻¹H=H`, the body is `AH=q⁻¹/² SH`. Conversely, capacity scaling and symplectic invariance give the benchmark ratio for every such body. The argument classifies bodies rather than demanding that the particular A itself be conformally symplectic; the automorphism factor matters.

For `det A<0`, the q-only map `R=diag(F,I)` has determinant −1 and preserves H. Then `AR` has positive determinant and `(AR)H=AH`. This proves both the bound and the same body-level equality classification. No assumption that an arbitrary orientation-reversing linear map preserves capacity is used. Translations preserve volume and capacity and can be added to the affine statement.

## Delivery recommendation

Promote the theorem after the separate executable certificate review is integrated and the small HK-order wording correction is made. Preserve the finite data/verifier as part of the proof artifact and give a reader the analytic slice and equality arguments above. No new mathematical obstruction emerged in this audit.

## Addendum: preferred 18-word, four-identity compression

At 23:06 UTC the author supplied `/tmp/msc-math/restart/geometry-frontiers/affine-hko-compressed/`. I read its complete `PROOF.md` and `verify.py`. The former corrects the source-order wording and zero-weight bridge noted above. The compressed analytic argument also passes this audit.

Compressed artifact SHA-256 values:

- `PROOF.md`: `067c0ecc079a2a20cd28ed21cd5762b0c77ca6a8e083f0c6631e882ea316d762`.
- `verify.py`: `3c0efef89a4f584c7206278cb0c1ed81fa18c668951a200d15c0c43b1e0c647a`.
- `certificate.json`: `f43d29f84f39f65523f73f4f3c68643459469724bae14b0bbffed4fcdd382818`.

The new argument concerns the full action polar `C_W={x:h_W(x)≤q₀}`, not invariance of the selected 18-row polyhedron C. This distinction is essential and is made explicitly in the revised proof.

If `Tᵀ` permutes the support-normalized normal set, then `Ω↦TΩTᵀ` preserves h_W: replace each normal in a word by its permuted normal `Tᵀa_i`. The same weights are feasible since their transformed weighted sum is zero. Invertibility makes this a bijection of all feasible words, so equality of support values holds in both directions. Negation also preserves h_W by word reversal. Thus each six-dimensional generator checked by the verifier preserves C_W. The intertwining equation `L G=g L` implies that the corresponding three-dimensional generator g preserves U(C_W).

I independently checked the signs in the actual generator definitions. The q-plane block of Trot is rotation by −2π/5, and its Ω-congruence induces rotation by +2π/5 in the first two U-coordinates. For `Tref=diag(1,−1,−1,1)`, the map `Ω↦−Tref Ω Trefᵀ` induces `(u,v,z)↦(u,−v,z)`. Negation induces total inversion. All generators therefore have the claimed mathematical origin.

The verifier normalizes each affine facet inequality to constant coefficient one and checks that each spatial row is `h_rep g`, with g a product of the verified generators. This is the correct dual-action direction: if `y∈U(C_W)`, then `g y∈U(C_W)`, so the representative inequality at `g y` gives `1+h_rep g y≥0`. Therefore the four certified representative inequalities imply all 62 inequalities on U(C_W). The script need not separately prove that its listed 20 products form a group; every listed product already preserves U(C_W), which is all this coverage argument uses.

No other analytic step changes. Subject to the separate replay of the compressed exact predicates, the compressed packet is suitable as the preferred proof artifact. In particular, it has no remaining mathematical correction from this audit.
