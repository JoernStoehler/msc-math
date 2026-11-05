# Choosing examples through ridge geometry

The random search sought both high systolic ratios and geometric patterns that could guide further sampling. We compared ratios with other measurements, then asked whether the strongest associations helped choose new examples before evaluating their capacities. The symplectic areas of two-dimensional faces supplied a useful signal. A regular-pentagon family gives an exact explanation of its behavior, while other product families show why it cannot serve as a universal improvement rule.

The initial collection contained 4,096 generic polytopes with five to twelve facets and 10,240 Lagrangian products of planar polygons, divided equally among the ten side-count pairs \(3\leq k\leq m\leq6\). Generation used seed 42 and support heights in \([0.8,1.2]\), with acceptance conditions; the product construction retained bounded polygons with every proposed side present. This defines a particular sampling distribution. In the following historical results, \(\widehat{\operatorname{sys}}\) denotes the stored numerical target: the original execution records do not establish certified capacity or ratio guarantees. Its maximum was \(0.862586\), with no value above one. This finite baseline does not bound the mathematical ratios of the sampled class.

A two-dimensional face of a four-dimensional polytope is a ridge. Define its unsigned symplectic area and the normalized ridge sum by
\[
 A_\omega(F)=\left|\int_F\omega_0\right|,
 \qquad
 R(K)=\frac{\sum_{F\text{ ridge of }K}A_\omega(F)}
 {\sqrt{\operatorname{vol}_4(K)}}.
\]
The absolute value is taken on each face before summing. A Lagrangian face contributes zero even when its Euclidean area is positive. Both numerator and denominator scale quadratically under dilation; \(R\) is also invariant under translations and linear symplectic maps.

In the historical mixture, \(R\) had Spearman correlation \(-0.9384\) with \(\widehat{\operatorname{sys}}\): smaller ridge sums tended to accompany higher target ranks. The Pearson correlation, \(-0.2050\), was much weaker, so this was not a strong linear relationship. It emerged from an exploratory feature screen. Moreover, the products' mean target exceeded the generic polytopes' mean by \(0.0491\), making source composition relevant to interpreting the pooled association.

For a Lagrangian product \(P\times_LQ\subset\mathbb R_q^2\times\mathbb R_p^2\), the descriptor has a concrete geometric interpretation. Let \(a_i,b_j\) be the factors' edge vectors and \(A_P,A_Q\) their areas. Factor faces are Lagrangian. Each remaining face is a rectangle spanned by an edge from each factor, with unsigned symplectic area \(|a_i\cdot b_j|\). Hence
\[
 R(P\times_LQ)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]
Relative rotation changes these edge pairings while leaving volume fixed.

Take the centered regular pentagon \(P_5\) of circumradius one and put
\[
 K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
 \qquad
 d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|
 \in[0,\pi/10].
\]
Writing \(s=\sin(\pi/5)\) and \(h=\cos(\pi/5)\), its edge length is \(2s\) and its area \(5sh\). The product formula becomes
\[
 R(K_\theta)=\frac{4s}{h}
 \sum_{k=0}^4|\cos(\theta+2\pi k/5)|.
\]
The sum is even and \(\pi/5\)-periodic. Pairing its terms on the reduced interval gives \(4h\cos d\), so
\[
 R(K_\theta)=16\sin(\pi/5)\cos d(\theta).
\]
The proved capacity profile of this family gives
\[
 \operatorname{sys}(K_\theta)
 =\frac{5+2\sqrt5}{10}\sec^2d(\theta),
 \qquad
 \boxed{\operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).}
\]
Thus the inverse-square relationship is exact here. From alignment to \(d=\pi/10\), the descriptor falls from approximately \(9.40456\) to \(4\sqrt5\approx8.94427\), while the ratio rises from approximately \(0.947214\) to \((3+\sqrt5)/5\approx1.047214\). Neither volume nor combinatorial type changes. Independent positive rescalings of the factors preserve both quantities in the identity, so circumradius one is only a convenient normalization. This later analytic explanation concerns one family; it was not a prospective prediction of the random screen, and its exact value above one was not discovered by that numerical search.

The family restriction matters. Along two designed product paths, of types \(3\times6\) and \(4\times4\), both the descriptor and the numerical systolic ratio decrease at every retained step toward the specified endpoints. Their endpoint ratios are \(3/4\) and \(1/2\). These paths display the opposite direction from pentagon rotation. A low ridge sum can therefore characterize promising members of a sampled population without telling us how to improve an individual body. Minimizing the descriptor alone does not supply a general capacity optimization principle.

Its practical use is instead tested by selecting candidates before their targets are known. Regression and feature-ablation studies found that ridge-area features carried much of the held-out predictive signal. A candidate experiment generated 100,000 products and evaluated a selected/control union of 1,675 bodies. The largest stored target was \(0.867546\); none exceeded one. The generated count must be distinguished from the number of capacity evaluations.

A separate fresh 100,000-candidate validation met its predeclared criterion for incremental enrichment from ridge concentration. Frozen ridge and rho selectors also raised mean targets relative to controls in both \(4\times6\) and \(6\times6\) buckets on one differently generated, separately area-normalized source, using 91 distinct target evaluations. These finite results support allocating evaluations using ridge information, even though they produced no new value above one. They establish neither unrestricted transfer to other distributions nor a universal deformation rule.

Local refinement addresses another part of the search. The optimizer chapter examines how competing branches of the capacity objective affect improvement steps. Its comparison of seven policies on 64 matched starts, totaling 448 runs, obtained a median best historical target of \(0.984783\) with a branch-history policy. The evaluator's completeness is not established, so this does not certify an optimizer ranking for mathematical capacity or local maximality of its endpoints. It does show why the initial random-table maximum is not a practical ceiling. Population selection and local refinement offer complementary routes; the ridge experiments connect the former to geometry through an exact example and explicit limits on its generalization.
