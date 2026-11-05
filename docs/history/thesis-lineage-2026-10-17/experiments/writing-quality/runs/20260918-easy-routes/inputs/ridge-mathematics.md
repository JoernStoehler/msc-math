# What the ridge descriptor explains for rotated pentagons

Reviewed 2026-09-18 against the source files below. This is a scientific content packet, not a new thesis chapter or a human prose acceptance judgment.

## Writer-ready scientific account

The data associate smaller normalized sums of symplectic ridge areas with larger systolic ratios. For rotated regular pentagon products, this relation has an exact explanation. Relative rotation changes the pairings between the edges of the two factors. The normalized sum of their absolute pairings is proportional to the cosine of the reduced rotation angle; the capacity is proportional to its reciprocal. Thus, on this family, the systolic ratio is exactly proportional to the inverse square of the ridge descriptor. Neither volume nor combinatorial type changes along the family.

This supplies a geometric example in which the observed direction of association is forced by the mathematics. It does not explain the whole sampled population. Existing designed product paths show the opposite direction, and no theorem here says that reducing this descriptor improves capacity for arbitrary products.

## Exact descriptor and source correspondence

Use coordinates `(q1,q2,p1,p2)` and

\[
\omega_0((q,p),(q',p'))=q\cdot p'-p\cdot q'.
\]

This is the actual expression in `crates/symplectic/src/geom/symplectic_form.rs::omega0`.
For a polygonal two-face with cyclic vertices `v_0,...,v_{r-1}`, the feature implementation computes

\[
\left|\int_F\omega_0\right|
 =\frac12\left|\sum_j\omega_0(v_j,v_{j+1})\right|.
\]

Source: `experiments/polytope-invariant-table/features_face_symplectic.rs::two_face_symplectic_area`. Summation and normalization are in `compute_face_symplectic_fields` and `experiments/polytope-invariant-table/invariant_features.rs`. The relevant table column is exactly **`ridge_symp_area_sum_over_volume_sqrt`**, denoted here by `R`. It is neither Euclidean area nor the signed sum of oriented face integrals. Absolute value is taken separately on each face before summation.

For full-dimensional convex polygons `P,Q`, with cyclic edge vectors `a_i,b_j` and areas `A_P,A_Q`,

\[
R(P\times_L Q)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\tag{1}
\]

Indeed the product's two-faces comprise `m` copies of `P`, `k` copies of `Q`, and `km` mixed rectangles. The first `k+m` faces are Lagrangian and contribute zero; a mixed rectangle contributes `|a_i·b_j|`. For pentagon products the expected count is 35 faces: 10 factor faces and 25 rectangles. At special angles some rectangles also have zero symplectic area; there are not necessarily exactly 10 zero-area faces.

The numerical implementation skips faces whose cyclic ordering fails. Correspondence to (1) therefore requires all 35 faces to be found and ordered, with zero ordering failures. A low sum without those checks can be a missing-face artifact. The mean column includes the zero-area factor faces and equals `R/35` on correctly enumerated pentagon products. Thus changing sum to mean changes the constant in the identity below by `35²`.

## Restricted identity, with proof

Let `P_5` be the centered regular pentagon of circumradius one, and

\[
K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
\quad d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|\in[0,\pi/10].
\]

Set `s=sin(π/5)`, `h=cos(π/5)`. Its edge length is `2s` and area `5sh`. For each fixed edge in the first factor, the relative directions of edges in the other factor differ by `θ+2πk/5`. Equation (1) gives

\[
R(K_\theta)=\frac{20s^2}{5sh}
 \sum_{k=0}^4|\cos(\theta+2\pi k/5)|.
\]

On `|θ|≤π/10`, the terms indexed by `0,1,4` are nonnegative and those indexed by `2,3` nonpositive, allowing zeros at the endpoints. Pairing terms equidistant from zero gives

\[
\sum_{k=0}^4|\cos(\theta+2\pi k/5)|
 =[1+2\cos(2\pi/5)-2\cos(4\pi/5)]\cos\theta
 =4h\cos\theta.
\]

The absolute-cosine sum is even and has period `π/5` (shift indices by three and use the period `π` of absolute cosine). Consequently

\[
\boxed{R(K_\theta)=16\sin(\pi/5)\cos d(\theta).}
\tag{2}
\]

The accepted analytic capacity argument in `docs/pentagon-chapter-v2/chapter.tex` proves

\[
c_{\rm EHZ}(K_\theta)=(1+h)^2\sec d(\theta),\qquad
\operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta),
\]

where `sys=c_EHZ²/(2 vol_4)`. Combining this with (2) yields

\[
\boxed{\operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).}
\tag{3}
\]

The algebra uses `sin²(π/5)=(5−√5)/8`. Thus decreasing `R` and increasing `sys` are equivalent *within this family*. The minimum `R=4√5≈8.944272` occurs at `d=π/10`, the HKO position where `sys=(3+√5)/5≈1.047214`. At alignment `R=16sin(π/5)≈9.404564` and `sys≈0.947214`.

Independent positive rescalings of the two factors preserve (3). The numerator and denominator of (1) both acquire the same factor. For capacity, scaling factors by `a,b>0` is the composition of a symplectic reciprocal scaling and common dilation by `√(ab)`; capacity scales by `ab`, volume by `(ab)²`, and hence sys is unchanged. Translations and linear symplectic images likewise preserve the geometric descriptors and sys. The meaningful restriction remains the geometric family, not circumradius one alone.

A direct floating-point check of (1) at 1001 angles from `−2π` to `2π` agreed with (2) to a maximum absolute discrepancy of `3.56e−15`. This checks arithmetic and angle reduction only; the displayed argument proves the identity. No full capacity evaluator or historical-table reproduction was run for this note.

## Support for the capacity formula

The current chapter's argument does not require classifying all numerical minimizers. Rotation leaves the feasible facet weights unchanged. For fixed feasible weights the quadratic objective has form `A cos θ+B sin θ`. Its values inside `[-π/10,π/10]` are nonnegative linear combinations of its two endpoint values. The known HKO capacity bounds those endpoints, while an explicit six-facet word attains the interpolated bound. This proves the capacity expression used above, including endpoints.

`docs/pentagon-chapter/user-reading-feedback.md` records Jörn's explicit acceptance of this proof, separately from rejection of its exposition. The source's comment attributes endpoint interpolation to the September 15 Pro audit and the explicit word to earlier work. Its historical `.git/codex/.../03_mathematical_audit.md` pointer is not available at the named path in this checkout; the mathematical derivation in the retained chapter is self-contained apart from the stated HK formula and HKO endpoint theorem. Do not represent this note as freshly retrieving that missing audit.

`formal/FINDINGS.md` still describes the older Sage word-enumeration route. That summary is stale relative to the accepted analytic replacement and must not dictate the final narrative.

## Chronology and thesis placement

The population investigations and ridge optimization notes predate the September analytic chapter replacement. They are not independent prospective tests of (3). The precise first discovery date of the restricted ridge identity has not been established by this review; avoid inventing a chronology in which (3) predicted the old results.

Jörn explicitly rejected including the ridge-area relationship in the pentagon chapter: see `docs/pentagon-chapter/user-reading-feedback.md`. The natural integration is in DS, referring to the proved capacity profile. This keeps the argument that determines capacity separate from the later interpretation of a statistical descriptor.

## Existing counterevidence and broader geometry

`experiments/sys-datascience/methods/ridge-endpoint-path/INTERPRETATION.md` reports two specified deterministic paths where decreasing `R` from the selected starting point to an equality endpoint also decreases `sys` at every retained step. The endpoints have `sys=3/4` for `3×6` and `sys=1/2` for `4×4`. This defeats the unrestricted inference that ever-lower ridge sum is a general optimization objective, while leaving coarse population enrichment and (3) intact.

There is already more geometry than the pentagon identity. `.../ridge-endpoint-path/notes/direct-optimization.md` and its independent mathematical review derive

\[
R(P,Q)=\frac{4V(P-P,JQ)}{\sqrt{A_PA_Q}}\ge8,
\]

with mixed area normalized by `area(K+tL)=area(K)+2tV(K,L)+t²area(L)` and `J` the planar quarter-turn. Equality requires `P` centrally symmetric and `JQ` positively homothetic to `P-P`. For triangular `P`, the stronger bound is `4√6`, attained by a suitable hexagonal second factor. These describe what minimizing the descriptor does geometrically; the endpoint experiments show why that geometric minimum need not maximize sys. The proposed `3×3` minimum 12 remains conjectural in that packet and should not be promoted to a theorem.

For the DS account, the resulting scientific sequence is: an empirical association suggests studying the descriptor; a regular-pentagon family gives an exact inverse-square relation; broader descriptor geometry and designed paths show the limits of extrapolation. This is a logical exposition order, not an asserted historical discovery order.
