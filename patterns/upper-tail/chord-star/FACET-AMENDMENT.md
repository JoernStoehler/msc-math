# The coverage certificate collapses when a small sixth facet is born

17 October 2026. This amendment preserves the approved REPORT-v1 and changes its practical scope: the chord-star certificate's useful five-facet locality does **not** transfer across a facet birth. Along the existing fixed vertex-cut path below, its geometry-only coverage fraction tends to zero although the physical factors converge to genuine five-facet polygons near HKO. The feasible capacity ceiling itself can remain accurate. This is a structural obstruction to the certificate, with no new capacity value or global-ratio claim.

## Frozen single-body challenge

FACET-PLAN.md selected the first input-order geometry from the existing frozen facet-transfer packet, `facet_transfer_s0_d1000`. No error-selected replacement, new cohort or oracle was used. Its appended sixth normal cuts the vertex between old normals0 and1 in each factor, at depth $\delta=1/1000$. The unchanged mathematical rule and existing checker were applied. Prediction was saved before joining its exact capacity and ratio from the prior facet-transfer result.

The exact finite outputs are:

| Quantity | Value |
|---|---:|
| True ratio, imported exact result | 1.032242359561489... |
| Chord-star ratio ceiling | 1.034768692535837... |
| Capacity ceiling / true capacity | 1.001222963332548... |
| Geometry-only coverage $\eta$ | 0.011829543174983... |
| New-facet cone-volume weight, A | 0.000577453172048... |
| New-facet cone-volume weight, B | 0.000582819470433... |

The nominal standalone capacity interval factor is therefore about84.53. The prior exact winner has zero weight on the added facet in both factors. **The finite cut is capacity-inactive:** the same HK witness is feasible for the outer five-facet product and gives $c_{\rm outer}\le c_{\rm cut}$. Inclusion gives $c_{\rm cut}\le c_{\rm outer}$, hence equality. This deduction uses the imported exact optimum and its old-facet witness, without evaluating a new outer capacity oracle. Thus the certificate can lose almost all force even when a physical facet birth leaves the true capacity exactly unchanged.

The upper-bound miss is small. The unchanged independent checker verifies this one body's feasibility, contracted-vertex convex combinations and star-response maxima using416 direct four-dimensional HK sums. Its receipt records PASS. The additional limit checker also verifies that the imported exact winner has both added-facet weights zero and reconstructs its four-dimensional HK objective for the five-facet outer product.

## Exact limiting mechanism

Keep the existing seed0 perturbed five-facet base fixed. In each factor, let the appended uncentered normal vary by the original construction

$$a_5(\delta)=\frac{a_0+a_1}{2(1-\delta)},\qquad \delta\downarrow0.$$

This is the same existing cut-depth parameter, with no new perturbation direction or sampled geometry. At zero the new normal is a strict convex combination of its adjacent old normals. The primal bodies converge to the original five-facet body. The new edge length tends to zero, so the cone-volume center satisfies

$$c(\delta)\longrightarrow c^0=(\bar c_0,\ldots,\bar c_4,0),\qquad \bar c_i>0.$$

After centroid normalization, the limiting new normal remains a strict convex combination of the corresponding old normals: if $z$ is the old centroid, its coefficient at the first normal is

$$\lambda=\frac{1-a_0\cdot z}{2-(a_0+a_1)\cdot z}\in(0,1).$$

The six-coordinate closure body keeps dimension3 at this limit. Its face $\alpha_5=0$ is the full old two-dimensional closure polygon. Its canonical center has moved to the relative interior of that face.

Let $\Pi^0$ be the limiting six-coordinate tangent projector. Exact rational calculation checks that **every entry** in its new-coordinate row is nonzero. The row values, rounded only for display, are

$$\begin{aligned}
A:\ (-.31478210,-.31654766,-.06983592,.08494615,-.06811111,.68433063),\\
B:\ (-.31708822,-.31468043,-.07007915,.08457078,-.06684263,.68411965).
\end{aligned}$$

Fix any projected coordinate ray $r=\Pi^0e_i$. Its new component is nonzero. The feasible limiting line through $c^0$ is therefore one-sided in its new coordinate: one chord endpoint is $c^0$, and the other has strictly positive new coordinate. The old components of $c^0$ are all positive, so that other endpoint is distinct from the center. The exact checker verifies this for all six rays of both factors. Consequently the limiting star hull $X^0$ satisfies

$$X^0\cap\{\alpha_5=0\}=\{c^0\}.$$

**Proposition.** Along this fixed path, $\eta_A(\delta)\to0$ and $\eta_B(\delta)\to0$, hence $\eta(\delta)\to0$.

**Proof.** Consider one factor. An old closure vertex lifts with sixth coordinate zero to a feasible closure $v(\delta)$ for every small positive cut depth; recentering only rescales its old weights and does not change that zero coordinate. It tends to a noncentral point $v^0$ of the old closure face. All radial endpoints converge: their one-sided collapse and the strictly positive opposite endpoints follow from the nonzero new projector coordinates just checked. Thus their convex hulls converge to $X^0$.

If some subsequence had $\eta(\delta)\to\eta_0>0$, the defining inclusion would place

$$c(\delta)+\eta(\delta)(v(\delta)-c(\delta))\in X(\delta).$$

Its limit would be the noncentral point $c^0+\eta_0(v^0-c^0)$ on $\alpha_5=0$, belonging to $X^0$. This contradicts the displayed face intersection. Therefore the factor coverage tends to zero. Apply the same argument to the other factor and take their maximum. ∎

This proof diagnoses a mismatch between the vanishing cone-volume center coordinate and the extra closure dimension. The physical cut is small; the six-coordinate closure body still includes an entire old closure face, while its coordinate-ray star touches that limiting face only at one point. The original fixed-facet continuity theorem remains valid. Across removal of the zero-length facet, the five-coordinate certificate has positive coverage whereas the six-coordinate fractions tend to zero.

## Consequence and verification

The universal bracket remains mathematically valid, but its certificate width can diverge under arbitrarily small physical cuts near the upper tail. This obstructs a geometry-independent across-shape locality claim for the present center/projector/chord architecture. The finite ceiling's0.1223% miss is not evidence that the upper-bound value itself has an order-one limiting error; no such limit is asserted or needed for the coverage obstruction. No metric, center, ray or feature variant is added.

`facet-limit.json` retains the exact limiting normals, center, projector row, convex-combination coefficient and old closure vertices. `facet_check.py` imports only the independent linear solver from verify.py, recomputes the limiting projector row, checks all six noncentral endpoints per factor and verifies the old closure face. `facet-limit-verification.json` records PASS. `facet-verification.json` records the unchanged finite check. The additional feature took3.92 seconds and finite verification0.93 seconds, excluding independent review. This was one fixed retrospective body and an exact analysis of its existing cut-depth path; no scan, new capacity oracle, new cohort, fit, production change, network, cloud, push or merge occurred.

Root owns independent review, retention, integration and commits. The receiving owner can retain this amendment as a material across-facet obstruction; the earlier broad-scan stop remains in force.

[review: /root/closure_review]
