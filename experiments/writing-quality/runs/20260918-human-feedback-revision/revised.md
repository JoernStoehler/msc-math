# Ridge area and systolic ratio

## An empirical association

In the sampled polytopes, a smaller total symplectic area of the two-dimensional faces usually accompanied a larger systolic ratio. The association suggests a geometric question: what makes these face areas informative about the systolic ratio?

For a four-dimensional polytope $K$, write
\[
 R(K)=\frac{\displaystyle\sum_{F\text{ a two-face of }K}
                   \left|\int_F\omega_0\right|}
                 {\sqrt{\operatorname{vol}_4(K)}},
 \qquad
 \operatorname{sys}(K)=\frac{c_{\mathrm{EHZ}}(K)^2}{2\operatorname{vol}_4(K)}.
\]
Here $\omega_0$ is the standard symplectic form. The absolute value is taken separately on each face: $R$ records symplectic area without cancellation between faces, rather than their Euclidean areas. In the empirical plots, $\widehat{\operatorname{sys}}$ denotes the recorded numerical approximation to the systolic ratio.

![Ridge area and numerical systolic ratio in the historical random sample.](figures/association-stratified.svg)

*Figure 1. Each point represents one retained polytope. Generic polytopes and Lagrangian products are displayed separately; the panels further distinguish facet counts or polygon sizes. The horizontal scale is logarithmic. These are observations from the specified random generators, not values sampled uniformly from convex bodies. The numerical evaluations have the provenance limitation described below; generation procedures are specified in the accompanying source notes.*

The pooled rank correlation between $R$ and the numerical systolic ratio is about $-0.94$. Rank correlation compares the orderings of the bodies by the two quantities; a value of $-1$ would mean exactly reversed orderings. The ordinary linear correlation is much weaker, about $-0.21$. Thus the principal observation concerns which bodies have larger systolic ratio, rather than how much the systolic ratio changes per unit of ridge area. The inverse association also occurs within every sampled facet-count or polygon-size group: the rank correlations range from about $-0.84$ to $-1.00$. Thus it is not solely a consequence of pooling these groups. Figure 1 shows the separate populations.

The sample contains 4,096 generic polytopes with five to twelve facets and 10,240 products of polygons with three to six edges. Each facet count or polygon-size pair contributes the same number of bodies within its class. Support heights were drawn in $[0.8,1.2]$, with seed 42, and geometries failing the generation conditions were discarded. These choices restrict the population to which the empirical observation applies. Moreover, the retained capacity evaluations predate the current certified implementation: their original execution records do not establish its guarantees. The plots and correlations are therefore evidence about the recorded numerical systolic ratios. They do not provide certified bounds or a statistical law for arbitrary convex bodies.

## An exact relation for pentagon products

For products, ridge area has a direct expression in terms of the polygon edges. Let $a_i$ and $b_j$ be the cyclic edge vectors of polygons $P$ and $Q$, with respective areas $A_P$ and $A_Q$. In $P\times_L Q$, the factor faces lie in Lagrangian planes and contribute zero symplectic area. Each remaining two-face is the product of two edges and has absolute symplectic area $|a_i\cdot b_j|$. Hence
\[
 R(P\times_L Q)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]
Relative rotation changes these pairings without changing the volume or combinatorial type of the product.

For rotated regular pentagons, the systolic ratio is exactly proportional to $R^{-2}$. To see this, let $P_5$ have circumradius one and put
\[
 K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
 \qquad d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|\in[0,\pi/10].
\]
Writing $s=\sin(\pi/5)$ and $h=\cos(\pi/5)$, each edge has length $2s$ and each polygon has area $5sh$. The edge formula gives
\[
 R(K_\theta)=\frac{4s}{h}\sum_{j=0}^4|\cos(\theta+2\pi j/5)|
            =16s\cos d(\theta).
\]
For the last equality, reduce $\theta$ to $[-\pi/10,\pi/10]$, where the signs of the five cosine terms are fixed. Pairing opposite indices gives a sum of $4h\cos\theta$; evenness and $\pi/5$ periodicity then give the stated formula. Combining it with the capacity profile proved in the pentagon chapter,
\[
 \operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta),
\]
yields
\[
 \boxed{\operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).}
\]
Thus decreasing $R$ increases the systolic ratio throughout this family: from about $0.947$ at alignment to $(3+\sqrt5)/5\approx1.047$ at $d=\pi/10$. This calculation supplies an exact instance of the inverse association observed in the data.

Decreasing $R$ does not, however, provide a generally reliable way to increase the systolic ratio. Two designed product paths, of types $3\times6$ and $4\times4$, exhibit the opposite behavior numerically: both $R$ and the numerical systolic ratio decrease at every retained step. The pentagon identity therefore explains one family, while these paths warn against extrapolating its direction of improvement to other products.

## Selecting new products from the observed pattern

Can the observed association still help select promising bodies, even when it does not prescribe a direction of deformation? This question requires new candidates chosen without knowing their capacities. The following experiments separate two tests: refining selection among low-$R$ bodies, and comparing a fixed selection rule against controls from a different polygon generator.

A fresh pool of 100,000 products tested a more specific prediction: among bodies with small $R$, distributing the symplectic area more evenly among the ridges might favor larger systolic ratios. Within each polygon-size pair, the experiment retained the lowest one percent by $R$. It then split this group into equal halves according to the fraction of total ridge area contributed by the largest ridge. The half with the smaller largest-ridge fraction had a larger mean numerical systolic ratio by about $0.034$ overall; the difference was positive in eight of the ten polygon-size pairs. The sign criterion—positive overall and in at least seven pairs—was specified before evaluating capacities. It required no minimum effect size and was not a significance test.

When applied to a different polygon generator, the same two-stage rule selected bodies with larger mean numerical systolic ratio than controls from that generator. This test replaced the randomly varying support heights by a common height before separately normalizing each factor's area to one; the resulting polygons are circumscribed about circles. Normal directions were retained only when both the original and modified polygons passed the generation conditions. In each of the $4\times6$ and $6\times6$ groups, the rule selected 16 of 3,200 eligible products: first the lowest one percent by $R$, then the half with the smaller largest-ridge fraction. Sixteen controls were chosen by a fixed hash ordering from bodies outside both geometric selection sets in this experiment, before capacity evaluation.[^controls]

| Polygon sizes | Mean numerical systolic ratio, selected | Mean numerical systolic ratio, controls |
|---|---:|---:|
| $4\times6$ | $0.637$ | $0.371$ |
| $6\times6$ | $0.708$ | $0.523$ |

These comparisons establish enrichment in the two finite pools; the preceding half-versus-half comparison shows an additional association with less concentrated ridge area within the low-$R$ group. They do not establish how often enrichment would recur under other generators. The pentagon identity supplies an exact example of the inverse association, while the opposing deformation paths show why successful selection from a population need not give a rule for improving each individual body.

[^controls]: The experiment also tested a covariance-based selector, not discussed here. Its selected bodies were excluded from the shared controls as well.
