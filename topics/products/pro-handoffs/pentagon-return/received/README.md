# Research handoff: beyond the rotated regular pentagon

**Main deliverable:** `math/research.tex`, a standalone mathematical note with definitions, theorem statements, proofs, and primary-source references. This package answers `GOAL.md` in `pentagon-generalization-pro.zip`. It does not alter the supplied project material.

## Main result: arbitrary independent affine pentagons

Let

\[
 n_i=(\cos(2\pi i/5),\sin(2\pi i/5)),\quad
 h=\cos(\pi/5),\quad
 P_5=\{q:n_i\cdot q\le h\},\quad w=1+h.
\]

For **any** invertible real two-by-two matrices \(G,H\) and translations \(a,b\), set

\[
 A=a+GP_5,\qquad B=b+HP_5,\qquad
 M=G^{-1}H^{-T},\qquad
 m(M)=\max_{0\le i,j<5}|n_i^TMn_j|.
\]

The note proves

\[
 \boxed{c_{\mathrm{EHZ}}(A\times B)=\frac{w^2}{m(M)}}.
\]

This handles independent shears, stretches, reflections, and scales, not just relative rotations or a common affine deformation. Twenty-five absolute pairings determine the capacity. The proof provides a four-block sparse witness and excludes every genuinely alternating six-block competitor. It does **not** assert that all optimizers have four blocks.

The resulting sharp bound on the entire affine class is

\[
 \boxed{\frac{c_{\mathrm{EHZ}}(A\times B)^2}
 {2\operatorname{area}(A)\operatorname{area}(B)}
 \le \frac{3+\sqrt5}{5}}.
\]

Let \(D=\operatorname{conv}\{\pm n_i\}\), a regular decagon, and let \(\operatorname{Sym}(D)\) denote its orthogonal dihedral group. Equality holds exactly when

\[
 \frac{M}{m(M)}D=D^\circ,
 \qquad\text{equivalently}\qquad
 M=tR_{\pi/10}U\quad(t>0,\ U\in\operatorname{Sym}(D)).
\]

Consequently, independently affinely deforming regular pentagon factors cannot improve the known HKO systolic ratio. This is not an upper bound for general polygon products. The formula also recovers the supplied full rotated-pentagon profile **without importing its endpoint capacity**.

## What makes the extension work

The general part of the note separates the sparse optimization into four-block and six-block words. For a polygon \(A\), define the width body

\[
 W_A=(A-A)^\circ.
\]

The convex hull of signed individual steps from normalized closure vertices is exactly \(W_A\). Thus the four-block matrices form

\[
 \mathcal T_4(A,B)=\operatorname{conv}
 \left\{\tfrac12xy^T:x\in W_A,\ y\in W_B\right\}.
\]

Every remaining sparse competitor is a matrix

\[
 \tfrac12(x_1y_1^T-x_3y_2^T),
\]

where each factor supplies a positive normalized closure triple. There are twelve cyclic alternating orders per triple pair. Containment of these matrices in \(\mathcal T_4\) is preserved by **independent** linear changes of the factors. This is stronger than containment of their two-dimensional angular projections.

For pentagons, put \(\lambda=(\sqrt5-1)/2\), \(e=n_0\), \(a=\lambda n_2\), and \(b=\lambda n_3\). Two exact identities are decisive:

\[
 ee^T-aa^T
 =\lambda^2ee^T-\lambda^2en_3^T-\lambda^3n_1n_2^T,
\]

\[
 eb^T-ae^T
 =\lambda^2n_1n_3^T-\lambda^2n_2n_4^T.
\]

The first has positive convex mass \(2\lambda^2+\lambda^3=1\) on signed rank-one generators; the second has mass \(2\lambda^2<1\), completed at zero. An explicit six-entry table, reflections, and independent rotations cover all twelve orders and all closure pairs. No numerical convex-hull inclusion is used as a proof.

The systolic bound then follows from the elementary containment \((M/m(M))D\subseteq D^\circ\) and comparison of areas. This separates the capacity computation from the extremal-ratio argument.

## Further results in the note

### Other complete profiles and nonregular families

For the same facet-normal orientation and unit circumradius, put \(h_n=\cos(\pi/n)\) and \(d_L(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/L|\). The following are proved at **every** angle, including all switches:

\[
 c(P_7\times R_\theta P_7)=(1+h_7)^2\sec d_7(\theta),
\]

\[
 c(P_5\times R_\theta P_7)=(1+h_5)(1+h_7)\sec d_{35}(\theta).
\]

Here a different argument applies: all six-block angular coefficients lie strictly inside the incircle of the four-block coefficient polygon. The proof classifies the heptagon's fourteen closure vertices into two isosceles types, covers all twelve orders, and gives exact rational bounds.

The strict gap persists in open neighborhoods of these two polygon pairs. This gives genuinely nonregular and unequal families satisfying, simultaneously for all relative rotations,

\[
 c(A\times R_\theta B)
 =\frac1{\max\{x\cdot R_\theta y:
                  x\in(A-A)^\circ,\ y\in(B-B)^\circ\}}.
\]

The regular secant formulas need not survive nonregular perturbations; the last width-body formula does. No explicit perturbation radius or arbitrary-affine heptagon theorem is asserted.

### Sharpness and obstruction

Positive endpoint interpolation is sharp at a strict interior angle exactly when one feasible coefficient maximizes at both endpoints; equivalently, the endpoint exposed faces intersect. Then it is sharp throughout that interval.

Pentagons have genuinely six-block optimizers at angular switches: an explicit alternating coefficient lies in the relative interior of a four-block hull edge. This explains why an existence theorem for a sparse/four-block optimizer must not be promoted to a classification of all optimizers.

The width-body rule is not universal. For equilateral triangles the six-block hull strictly contains the four-block hull, and the note gives the entire exact profile. In particular, the actual unrotated capacity is \(3/2\), whereas the four-block-only answer would be \(9/4\). No novelty claim is made for this triangle capacity calculation.

## Status and scope

**Proofs supplied:** the affine-pentagon formula, affine-class systolic bound with equality classification, width/tensor decomposition, two heptagon-related profiles, stability statement, and interpolation criterion. These are research proofs submitted for independent mathematical review, not previously accepted project theorems.

**Imported:** the Haim–Kislev finite capacity formula. The accepted six-facet reduction from the input is recalled with its proof and is not credited as a new contribution. The Artstein–Avidan–Karasev–Ostrover symmetric-product formula is used only to interpret width-body equality as preservation of capacity under simultaneous central symmetrization. The HKO ratio is the known benchmark, not a newly discovered constant. Primary references and precise source locations are in the note's bibliography and status section.

**Relation to neighboring work:** normalized closure-step triangles agree with the normal-triangle objects of Balitskiy–Mitrofanov–Polyanskii, Definition 3.2. The representation by paired rank-one matrices is the mechanism used here. Their quadrilateral and affinely regular hexagon results are not used as substitutes for a pentagon theorem. A limited targeted literature check did not locate the principal independent-affine-pentagon statements; this is not an exhaustive novelty or priority assessment.

**Unresolved:** the candidate odd-regular formula

\[
 c(P_n\times R_\theta P_m)
 \stackrel{?}{=}(1+h_n)(1+h_m)
 \sec d_{\operatorname{lcm}(n,m)}(\theta),\qquad n,m\ge5\text{ odd},
\]

is not proved here outside the displayed cases. The exact missing step is containment of every paired closure-triangle angular coefficient in the regular four-block polygon. Exploratory floating-point observations beyond the proved cases are not promoted to results or required for any proof. No extra project data are needed to state this remaining problem.

## Files and review route

Read `math/research.tex` first. Sections 2–5 contain the reduction, width-body lemma, two tensor identities, affine theorem, and sharp systolic bound. Sections 6–8 contain the angular criterion, obstruction, additional profiles, and stability argument. Section 9 distinguishes proved, imported, and unresolved material.

`verification/check.py` and `verification/results.json` are optional algebra and floating-point cross-checks. They are not needed to read or validate the written proofs. `verification/README.md` explains their exact scope. `MANIFEST.json` records input provenance and output hashes.

The most consequential independent audit points are the width-body vertex realization, the twelve-order tensor reduction, and the decagon area/equality argument. The heptagon extension is separable from the main affine theorem.

No external articles, original input files, fonts, or generated PDF are bundled. The LaTeX source uses standard packages and can be read directly by the project agent.
