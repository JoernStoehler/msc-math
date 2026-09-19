# Ridge geometry as a guide to candidate selection

Can cheap geometric descriptions identify promising bodies before expensive capacity evaluation? This question starts with a finite population of random polytopes: sample varied bodies, look for statistical patterns in their measurements, and seek geometric explanations. It requires distinguishing a relationship observed after computing the target from a rule that selects new candidates without knowing their targets. Local refinement asks a complementary question, using the branch structure of capacity to improve random starts; that subject belongs to the optimizer account.

The retained random table contains 4,096 generic polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian products, with 1,024 in each of the ten polygon-pair buckets \(3\leq k\leq m\leq6\). Generation used seed 42 and support heights in \([0.8,1.2]\), conditioned on acceptance. No stored systolic-ratio target exceeds one; the largest is approximately 0.862586. These are historical evaluator observations. The original run records do not establish that the targets were computed by the current certified implementation, so they cannot be promoted to certified capacity or ratio claims. Neither the finite sample nor its recorded maximum settles Viterbo's conjecture.

The normalized sum of unsigned symplectic two-face areas is strongly negatively rank-associated with the stored target: Spearman's coefficient is approximately \(-0.9384\) on this mixture. Pearson's coefficient, approximately \(-0.2050\), is much weaker, so a linear-law description would misstate the finding. Source composition also matters: products have a mean stored target about 0.0491 above that of generic polytopes. The pooled association therefore gives a promising descriptor to investigate, without establishing one relationship that holds across all these geometries.

Write this descriptor as
\[
R(K)=\frac{\sum_{F\text{ a two-face of }K}\left|\int_F\omega_0\right|}
 {\sqrt{\operatorname{vol}_4(K)}}.
\]
The absolute value is taken separately on each face before summation. Thus this measures symplectic area, rather than Euclidean area or a signed sum in which contributions could cancel. In coordinates \((q_1,q_2,p_1,p_2)\),
\[
\omega_0((q,p),(q',p'))=q\cdot p'-p\cdot q'.
\]
For full-dimensional convex polygons \(P,Q\), with cyclic edge vectors \(a_i,b_j\) and areas \(A_P,A_Q\),
\[
R(P\times_LQ)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]
Indeed, the product's two-faces comprise copies of the two factors and mixed rectangles. The factor faces are Lagrangian and contribute zero; each mixed rectangle contributes \(|a_i\cdot b_j|\). The descriptor consequently records the absolute pairings between edges of the two factors.

For rotated regular pentagon products, the direction of the observed association has an exact explanation. Relative rotation changes those pairings while leaving volume and combinatorial type unchanged. Let \(P_5\) be the centered regular pentagon of circumradius one, and set
\[
K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
\qquad d(\theta)=\min_{j\in\mathbb Z}|\theta-j\pi/5|\in[0,\pi/10].
\]
Put \(s=\sin(\pi/5)\) and \(h=\cos(\pi/5)\). The edge length is \(2s\) and the area is \(5sh\). For a fixed edge in the first factor, the relative directions of the edges in the other factor differ by \(\theta+2\pi j/5\). Hence
\[
R(K_\theta)=\frac{20s^2}{5sh}
\sum_{j=0}^4|\cos(\theta+2\pi j/5)|.
\]
On \(|\theta|\leq\pi/10\), pairing the cosine terms gives
\[
\sum_{j=0}^4|\cos(\theta+2\pi j/5)|
=[1+2\cos(2\pi/5)-2\cos(4\pi/5)]\cos\theta
=4h\cos\theta.
\]
The absolute-cosine sum is even and has period \(\pi/5\), so
\[
R(K_\theta)=16\sin(\pi/5)\cos d(\theta).
\]
The analytic capacity profile proved in the pentagon account gives
\[
c_{\rm EHZ}(K_\theta)=(1+h)^2\sec d(\theta),
\qquad
\operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta).
\]
Combining these formulas yields the restricted identity
\[
\boxed{\operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).}
\]
Thus decreasing \(R\) and increasing \(\operatorname{sys}\) are equivalent within this family. At alignment the ratio is approximately 0.947214; at \(d=\pi/10\), the HKO position, \(R\) reaches its minimum \(4\sqrt5\) and the ratio is \((3+\sqrt5)/5\approx1.047214\). These exact family results have a different status from the historical numerical targets above. Independent positive rescalings of the two factors preserve the identity: capacity scales by the product of the scaling factors and volume by its square, while the numerator and denominator defining \(R\) acquire the same factor.

This supplies a geometric example in which the observed direction of association is forced by the mathematics. It does not explain the whole sampled population, and the earlier population investigations were not prospective tests of this later analytic explanation. Existing designed product paths show the opposite direction. Along the retained steps of specified \(3\times6\) and \(4\times4\) paths, decreasing \(R\) also decreases the reported systolic ratio, towards endpoints with ratios \(3/4\) and \(1/2\), respectively. Consequently, ever-lower ridge sum is not a general ascent objective. The pentagon identity and a useful population association can coexist with these failures.

Candidate selection asks whether the association remains useful without that universal implication. Ordinary models and feature ablations found that ridge-area features carry most of the held-out predictive signal. Frozen scalar selectors then tested whether geometric information could enrich newly generated pools before their capacity targets were known. In the original 100,000-product candidate experiment, 30 selection sets produced 485 distinct selected candidates. The selected/control union contained 1,675 bodies, and only that union received target evaluations. Its maximum stored target was approximately 0.867546, with none above one; generating 100,000 geometries did not amount to evaluating 100,000 capacities.

A separate fresh 100,000-candidate concentration experiment met its predeclared incremental-enrichment criterion. Frozen selectors also enriched mean target relative to disjoint controls in both the \(4\times6\) and \(6\times6\) buckets of one separately area-normalized source, using 91 distinct evaluated targets. These results support finite enrichment under the named designs. They make the descriptor useful for choosing bodies to evaluate, while leaving open how far that advantage transfers to other distributions. The exact pentagon example explains one relation between ridge geometry and capacity; the designed paths explain why selection must still be judged against evaluated controls rather than by descriptor reduction alone.
