# Proposed additions to chapter 8

These are candidate paragraphs, not edits to the selected manuscript. Prefer the first two as the core explanatory addition; use the simplex law as a compact worked example. The new heptagon theorem merits a short discovery bridge here and its proof in a product chapter or accompanying appendix if selected. Preserve the finite-label caveats with the empirical paragraphs. Source proofs have independent agent review, not Jörn's proof/prose acceptance or an external novelty assessment.

## A. Relative position decides whether central symmetrization helps — include

A statistical association with symmetry does not determine the effect of making a body more symmetric. Let $P=\operatorname{conv}\{(-1,-1),(2,-1),(-1,2)\}$, let $J$ be a planar quarter-turn, and put $S=(P-P)/2$. Both $P\times_LP$ and $P\times_LJP$ have systolic ratio $1/2$. Replacing each factor by its central symmetrization changes these ratios to $2/9$ and $8/9$, respectively. Thus even the original ratio and the individual factor-asymmetry amounts agree, while the sign of the intervention reverses. The difference is their relative position. Exact triangular closure enumeration and the capacity theorem for two centrally symmetric factors establish these values.

The same distinction matters in the retained sample. Area asymmetry and the historical numerical ratio have Spearman correlation about $-0.517$, but paired central symmetrization lowers the ratio in $10{,}117$ of the $10{,}240$ products. The transformed product has a known capacity from its difference bodies; the original labels remain the historical numerical evaluations. This comparison concerns those accepted sampled bodies and does not assert a universal direction for symmetrization.

**Figure/source:** [symmetrization.png](product-shape/symmetrization.png), [exact rational control and population contract](product-shape/REPORT.md). Add after the current discussion of geometric changes versus statistical association. This adds an exact counterexample, not another correlation.

## B. Low moments cannot see a known threshold — include

Even a complete collection of low-order moments can miss the information that controls the ratio. Consider $K_\theta=P_5\times_LR_\theta P_5$, and give its 25 extreme vertices equal weight. For a regular pentagon, the complex moment $\mathbb E[z^a\bar z^b]$ vanishes unless $5$ divides $a-b$. At total degree at most four only $a=b$ survives, so all real vertex moments through degree four of $K_\theta$ are independent of $\theta$. Nevertheless the exact rotation formula gives ratios from $(5+2\sqrt5)/10$ to $(3+\sqrt5)/5$, a difference of exactly $1/10$, crossing one. No function of these moments can decide the above-one threshold on this family. The first angle-dependent factor moment has degree five: $\mathbb E[z^5]=e^{5i\theta}$. This is a limitation of the measured information, independently of how a regression is fitted.

This obstruction sharpens the interpretation of the covariance association. On $14{,}334$ common rows, a quadratic model using log ridge sum and log symplectic covariance eccentricity explains about $97.3\%$ of pooled held-out log-ratio variance when each generator bucket is omitted in turn. On the pooled $1{,}446$ held-out rows selected from the top decile separately within each bucket, its log-target $R^2$ is $-0.071$. The tail statistic uses the pooled selected mean, without bucket centering. A strong bulk score therefore does not by itself explain near-maximizers; the exact pentagon family shows a specific missing angular variable.

**Figure/source:** [covariance-patterns.png](features/covariance-patterns.png), [moment proof, holdout definitions and exclusions](features/REPORT.md). The existing pentagon capacity theorem should be cross-referenced, not presented as newly proved here. The fifth-harmonic descriptor is specific to the displayed factor coordinates, not a general symplectic invariant.

## C. A complete explanation within the simplex bucket — include if space permits

For the five-facet stratum the ridge relationship admits an exact refinement. Write the simplex inequalities as $a_i\cdot x\leq1$, let $\beta_i>0$ be their unique normalized closure weights, and set $B_{ij}=\omega(\beta_i a_i,\beta_j a_j)$. The unsigned symplectic area of the ridge shared by facets $i,j$ is $12V|B_{ij}|$. Define $\chi$ as the maximal signed forward sum of $B$ over facet orders, divided by $\sum_{i<j}|B_{ij}|$. Then
$$
 \operatorname{sys}(K)=\frac{18}{\chi(K)^2R(K)^2},\qquad
 \frac13\leq\chi(K)\leq\frac35.
$$
Thus unsigned ridge sum omits the signing and order of the ridge circulation. Two rational simplices even have identical volume $1/192$ and identical $R^2=363$, but ratios $6/25$ and $3/8$. The unsigned descriptor therefore cannot determine the ratio. The resulting sharp envelope is $50\leq\operatorname{sys}(K)R(K)^2\leq162$. A separate sharp bound gives $\operatorname{sys}(K)\leq3/4$ for every four-simplex, with equality for the ordinary coordinate simplex. These formulas reconstruct all 512 historical five-facet rows; the largest recorded ratio, approximately $0.7144$, lies below a mathematically explained ceiling rather than merely below the largest number found in a random sample.

**Figure/source:** [simplex-ridge.png](simplex/simplex-ridge.png), [full finite proof and exact polytope verifier](simplex/REPORT.md). This run rederived the result without assessing literature novelty. The proof reduces to 24 cyclic orders and one exact 20-vertex polytope lemma. Computing $\chi$ solves that capacity problem; do not advertise it as a cheap independent predictor. Keep the finite proof outside the main chapter and label the floating sample reconstruction separately.

## D. A discovery that closes an entire affine family — short bridge; prove elsewhere

The regular-product screen also resolved a larger structural question. For two independently affinely regular heptagons, all paired three-step competitors lie strictly inside the four-block tensor hull. A 48-case exact interval certificate therefore gives
$$
 c(GP_7\times_LHP_7)=
 \frac{(1+\cos(\pi/7))^2}
 {\max_{i,j}|n_i^TG^{-1}H^{-T}n_j|}.
$$
Its sharp systolic maximum is approximately $0.9174081964$, attained at the regular relative midpoint position, up to the stated affine symmetries. Independent shears and stretches cannot improve that maximum. This extends the retained rotation-only result to arbitrary independent invertible affine maps of the factors; it does not cover arbitrary heptagons. The exploratory calculation produced a finite proof excluding an entire deformed family from the above-one search.

**Figure/source:** [heptagon-structure.png](regular/heptagon-structure.png), [certificate, proof and equality class](regular/REPORT.md), [independent review](reviews/regular.md). New computational proof in this run; human acceptance and external novelty unassessed. Use the exact expression in the theorem statement rather than only the decimal. Do not imply the general odd-polygon conjecture has been settled.

## E. Width is a rejection test, not a sufficient search objective — optional short inset

The difference-body two-bounce capacity $c_2$ supplies an upper bound on the full capacity. Among all $3{,}072$ retained $5\times5$, $5\times6$ and $6\times6$ products, the corresponding ratio ceiling exceeds one in only 26 cases. Their recorded actual ratios are $0.4745$–$0.7947$. Even the largest ceiling, $1.32957$, belongs to a body with recorded ratio $0.52615$. Competing three-bounce paths can remove the apparent room for a large ratio. The ceiling remains useful for rejecting candidates, while optimizing it alone does not control this obstruction.

**Figure/source:** [width-screen.png](product-shape/width-screen.png), [all survivors and geometry checks](product-shape/REPORT.md). Retain numerical-label caveats; no universal takeover classification is supplied. This mechanism and the two-bounce bound were known; the complete threshold-survivor falsification is the new evidence here.

## Keep out of the proposed chapter claim set

Do not promote the high pooled model score alone as a discovery, treat one returned word as a complete orbit classification, or infer that near-maximal arbitrary polygon products are regular. Do not present new tiny numerical bounds or further unproved odd-polygon scans as results. No unrestricted Viterbo statement follows from these packets.

[review: /root/message_reviewer]
