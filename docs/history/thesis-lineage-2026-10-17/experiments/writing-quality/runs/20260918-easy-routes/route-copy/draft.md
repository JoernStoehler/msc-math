# Copy route: extracted substrate, before copy-editing

These are verbatim excerpts from the frozen AI scientific notes, arranged for this route. They are not human-authored notes or a finished section.

<!-- Source: search-account.md; verbatim extracted AI scientific notes -->

There are two complementary search questions. Can cheap geometric descriptions
identify promising bodies before expensive capacity evaluation? Can local
refinement exploit the branch structure of capacity to improve random starts?
The retained work gives bounded positive answers to both, although neither
found a new positive source. A chapter that says only “many methods failed to
find counterexamples” would discard its most useful results.

The first question starts with a finite population of random polytopes. It
requires distinguishing a relationship observed after computing the target
from a rule that selects new candidates without knowing their targets.
Ordinary models and feature ablations show where the association lives;
frozen scalar selectors then test whether that information can actually enrich
new candidate pools. The second question uses local branch information rather
than population correlations. Its comparatively high attained values show why
one cannot identify the retained random-table maximum with a limit of the
search machinery. However, evaluator lineage differs between packets, so their
numerical maxima are not a controlled head-to-head method comparison.

<!-- Source: README.md; verbatim extracted AI scientific notes -->

1. **Question and baseline.** Search for high systolic ratio using both population-level geometric guidance and local refinement. State the actual distributions: 4,096 generic rows (512 each at 5–12 facets) and 10,240 products (1,024 each in ten polygon-pair buckets, 3≤k≤m≤6), seed 42, support heights [0.8, 1.2], acceptance-conditioned generation. The retained numerical targets contain no value above one and maximum 0.86258589584944. Neither finite sampling nor these historical targets settle Viterbo's conjecture.
2. **Observed structure.** The normalized sum of unsigned symplectic two-face areas is strongly negatively rank-associated with stored sys: Spearman −0.9384368671850424 on this mixture. Pearson −0.2050393154151063 is much weaker, so a linear-law description would misstate the finding. Family-max permutation p=1/201 is a coarse screen across engineered features; it is not a general-law probability, a causal result or an independent validation. Product-minus-generic mean 0.04908416085647144 also warns that source composition matters.

<!-- Source: ridge-mathematics.md; verbatim extracted AI scientific notes -->

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

<!-- Source: ridge-mathematics.md; verbatim extracted AI scientific notes -->

`experiments/sys-datascience/methods/ridge-endpoint-path/INTERPRETATION.md` reports two specified deterministic paths where decreasing `R` from the selected starting point to an equality endpoint also decreases `sys` at every retained step. The endpoints have `sys=3/4` for `3×6` and `sys=1/2` for `4×4`. This defeats the unrestricted inference that ever-lower ridge sum is a general optimization objective, while leaving coarse population enrichment and (3) intact.

<!-- Source: search-account.md; verbatim extracted AI scientific notes -->

| Ordinary feature methods | P2 runs lasso, elastic net, boosting regression/classification and family ablations; associated packets cover ridge/RF, rules, projections, clustering, anomalies and association screens. 45 active invariant numeric features, grouped holdout by source/facet count. Ridge-area features carry most of the held-out signal. This is not generated-candidate validation or exhaustive coverage of all methods. | `experiments/sys-datascience/methods/standard-baseline-p2/README.md`, its `artifacts/`, and `trusted-random-product-method-dispositions.md` |
| Selection before target evaluation | In the original 100k product-candidate packet, 30 selection sets yielded 485 unique selected candidates and a 1,675-row selected/control union. Only the union received target evaluations; maximum 0.867546058507634 and zero above one. A separate fresh 100k concentration validation passed its predeclared incremental-enrichment criterion. Generated count must not be reported as capacity-evaluated count. | `methods/extreme-scalar-rejection-proposer/README.md`, `artifacts/100k-promising-scalars/`, `artifacts/100k-ridge-concentration-validation/` |
| Transfer to a different named source | Frozen rho and ridge selectors enriched mean target versus disjoint controls in both `4x6` and `6x6` buckets on one separately area-normalized `factorial-both` source. 91 distinct evaluated targets, five overlaps, no positives. This is finite-design sub-threshold enrichment; the packet's `strong_transfer` label is not a population theorem or superiority claim. | `methods/alternative-source-transfer/POST-TARGET-ACCOUNT.md`, `artifacts/transfer-v1/analysis.json` |
