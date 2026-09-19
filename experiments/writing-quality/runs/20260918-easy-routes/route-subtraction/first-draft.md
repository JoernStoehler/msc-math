# Choosing examples through ridge geometry

The random search had two purposes: to find bodies with large systolic ratio and to learn which geometric features made such bodies promising. A collection of numerical ratios can answer the first question only for the examples evaluated. The second requires comparing those ratios with other measurements, interpreting the resulting patterns, and testing whether the patterns help choose new examples. We therefore used sampled polytopes both as candidates and as a source of geometric questions. The most striking association involved the symplectic areas of their two-dimensional faces. It led to a useful selection rule, an exact explanation on one family, and examples showing why the rule cannot be made universal.

The initial collection contained 4,096 generic polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian products of planar polygons. The products were divided equally among the ten pairs of side counts satisfying \(3\leq k\leq m\leq6\). The generation used seed 42 and support heights in \([0.8,1.2]\), subject to geometric acceptance conditions. In particular, the product construction retained bounded polygons with every proposed side present. These conditions matter: this is a particular distribution of shapes, rather than an unqualified sample of all convex bodies. Products also outnumber generic polytopes, so a relationship computed on the whole table need not describe the two sources equally well.

Write \(\operatorname{sys}(K)=c_{\mathrm{EHZ}}(K)^2/(2\operatorname{vol}_4(K))\). For the historical experiments, let \(\widehat{\operatorname{sys}}\) denote the stored numerical target. The original execution records do not establish the guarantees of the current certified capacity implementation for those values. Reproducing the statistical analysis reproduces an association with that recorded target; it does not retroactively certify capacities or ratios. None of the 14,336 stored targets exceeded one, and their maximum was approximately \(0.862586\). This gives a finite baseline for the search, not a bound on the mathematical ratios of the sampled class or a conclusion about Viterbo's conjecture.

A two-dimensional face of a four-dimensional polytope is called a ridge. Its symplectic area is different from its Euclidean area. In coordinates \((q_1,q_2,p_1,p_2)\), use \(\omega_0=\sum_{j=1}^2dq_j\wedge dp_j\). For a ridge \(F\) with cyclically ordered vertices \(v_0,\ldots,v_{r-1}\), put

\[
 A_\omega(F)=\left|\int_F\omega_0\right|
 =\frac12\left|\sum_{j=0}^{r-1}\omega_0(v_j,v_{j+1})\right|,
 \qquad v_r=v_0.
\]

The descriptor of interest is

\[
 R(K)=\frac{\sum_{F\text{ ridge of }K}A_\omega(F)}
 {\sqrt{\operatorname{vol}_4(K)}}.
\]

The absolute value is taken separately for every face, before summing. Thus oppositely oriented contributions cannot cancel between faces. This is not the signed integral over a collection of faces, nor the sum of their ordinary areas. A face lying in a Lagrangian plane contributes zero even when its Euclidean area is positive. The descriptor is unchanged by translation, linear symplectic maps, and common dilation: the numerator and denominator both scale quadratically under dilation. It measures a feature of symplectic geometry at a scale appropriate for comparison with the systolic ratio.

On the historical mixture, \(R\) had Spearman correlation approximately \(-0.9384\) with \(\widehat{\operatorname{sys}}\). Lower ridge sums therefore tended to accompany higher target ranks. The Pearson correlation was only about \(-0.2050\), so describing this as a strong linear relation would obscure what was observed. The association emerged in an exploratory screen of engineered features. A family-maximum permutation screen gave \(p=1/201\), a coarse resolution that does not turn the selected feature into an independently validated law. Source composition also matters: the mean stored target for products exceeded that for generic bodies by approximately \(0.0491\). These observations justify investigating the descriptor, but they neither identify a causal mechanism nor prove that changing the descriptor changes capacity in the desired direction.

For a Lagrangian product \(P\times_LQ\subset\mathbb R_q^2\times\mathbb R_p^2\), the ridge sum has a particularly concrete meaning. Let \(a_i\) and \(b_j\) be the edge vectors of the two factors and let their areas be \(A_P,A_Q\). A factor face is contained in a Lagrangian plane and contributes zero. A mixed face is the rectangle spanned by one edge of each factor; its unsigned symplectic area is \(|a_i\cdot b_j|\). Consequently

\[
 R(P\times_LQ)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]

The descriptor records the aggregate absolute pairings of the two edge systems. Relative rotation changes those pairings without changing either factor's area. This gives a way to understand how \(R\) can vary even when four-dimensional volume remains fixed. For pentagons there are 35 two-faces: ten factor faces and 25 mixed rectangles. Some mixed rectangles can have zero symplectic area too. Numerical evaluation of the sum presupposes that all faces have been found and ordered; omitting a face would create an artificially low descriptor rather than promising geometry.

An exact example comes from the centered regular pentagon \(P_5\) of circumradius one. Set

\[
 K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
 \qquad d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|
 \in[0,\pi/10].
\]

Write \(s=\sin(\pi/5)\) and \(h=\cos(\pi/5)\). Each edge has length \(2s\), while the pentagon's area is \(5sh\). For a fixed edge of the first factor, the relative directions in the second differ by \(\theta+2\pi k/5\). Substitution into the product formula gives

\[
 R(K_\theta)=\frac{4s}{h}\sum_{k=0}^4
 |\cos(\theta+2\pi k/5)|.
\]

The absolute-cosine sum is even and has period \(\pi/5\). On the reduced interval, the terms can be paired to obtain

\[
 \sum_{k=0}^4|\cos(d+2\pi k/5)|
 =[1+2\cos(2\pi/5)-2\cos(4\pi/5)]\cos d
 =4h\cos d.
\]

Thus \(R(K_\theta)=16\sin(\pi/5)\cos d(\theta)\). The proved capacity profile for this family gives

\[
 \operatorname{sys}(K_\theta)
 =\frac{5+2\sqrt5}{10}\sec^2 d(\theta),
 \qquad
 \operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).
\]

On this family the inverse-square relationship is exact. Moving from alignment to \(d=\pi/10\) lowers \(R\) from approximately \(9.40456\) to \(4\sqrt5\approx8.94427\), while the systolic ratio rises from approximately \(0.947214\) to \((3+\sqrt5)/5\approx1.047214\). Volume and combinatorial type remain fixed. Independent positive rescalings of the two factors also leave both \(R\) and the ratio unchanged, so circumradius one merely fixes a convenient normalization. This is a later analytic explanation for the observed direction on a restricted family; it was not a prospective prediction of the original random screen. Its exact values above one do not come from the historical numerical search.

The restriction to this family is essential. Two designed product paths, of types \(3\times6\) and \(4\times4\), decrease both \(R\) and the numerical systolic ratio at every retained step toward their specified endpoints. The endpoint ratios are \(3/4\) and \(1/2\), respectively. They exhibit the opposite behavior from the regular-pentagon rotation family. More generally, the geometric problem of minimizing the descriptor has its own structure: in terms of mixed area it can be written \(R(P,Q)=4V(P-P,JQ)/\sqrt{A_PA_Q}\geq8\), where \(J\) is a planar quarter-turn. Here mixed area is normalized by the coefficient of the linear term in \(\operatorname{area}(K+tL)=\operatorname{area}(K)+2tV(K,L)+t^2\operatorname{area}(L)\). Equality in the lower bound requires central symmetry of the first polygon and a matching shape for the rotated second factor. These conditions describe edge geometry; they do not contain a capacity maximization statement. A geometric minimum of \(R\) is therefore not automatically a maximum of the systolic ratio. The empirical association, the exact pentagon identity, and the failure of a universal improvement rule can all hold simultaneously.

For search, the relevant test is whether the descriptor helps choose candidates before their targets are known. Regression and feature-ablation studies found that ridge-area features carried much of the held-out predictive signal. That supports a selection experiment but does not replace it: predicting values in an existing table and selecting from newly generated shapes are different tasks. In one candidate experiment, 100,000 products were generated and target-free rules selected promising subsets. Their selected/control union comprised 1,675 evaluated bodies, not 100,000 capacity evaluations. The largest stored target was approximately \(0.867546\), and none exceeded one. A separate fresh 100,000-candidate validation met its predeclared criterion for incremental enrichment from ridge concentration. Frozen ridge and rho selectors also enriched mean targets in both \(4\times6\) and \(6\times6\) buckets on one differently generated, separately area-normalized source, with 91 distinct evaluated targets. These are positive finite selection results, even though they did not discover a new value above one. They support using ridge information to allocate evaluations, within the tested settings, without making it a universal objective for deforming bodies.

The difference between selection and deformation also appears in direct interventions. Replacing polygon support heights by one, while holding their normals fixed, makes the factors tangential. Sixteen jointly admissible pairs, eight each of types \(4\times4\) and \(4\times6\), supplied baseline and one-factor and two-factor interventions. Changing both factors increased the mean numerical ratio by about \(0.0153\), but eleven pairs improved and five worsened. The mean effect therefore cannot be turned into an instruction that tangentialization always helps. Separate factor area normalization preserves the ideal mathematical ratio, so the intended intervention changes shape rather than scale.

Orientation likewise gives a useful distinction. A search on a selected product champion improved its historical target from \(0.862586\) to \(0.878308\). But improvement of a fixed body does not establish that rotating it is the best use of an evaluation budget. In four later blocks, two each of types \(3\times3\) and \(4\times4\), sixteen prescribed orientations of the first body were compared with sixteen independently drawn bodies. Orientation improved three of the four starting bodies, while fresh sampling achieved the larger maximum in every block. The result concerns those fixed allocations and sampled bodies; it neither finds optimal orientations nor measures a population-wide advantage. These later panels use a better documented capacity evaluator, but floating-point volume still makes the reported ratios numerical rather than certified enclosures.

Local refinement supplies a complementary approach. Capacity optimization involves competing branches, so a step suggested by one branch may be limited by another. The optimizer chapter studies this issue directly. In 448 runs comparing seven policies on 64 matched starts, a branch-history policy reached a median best historical target of approximately \(0.984783\). The shared evaluator's completeness is not established, and the comparison does not certify the ranking for mathematical capacity or prove local maximality of its endpoints. Nevertheless, it shows why the random-table maximum should not be treated as a practical ceiling. Population selection and local refinement address different parts of the search. Ridge geometry contributes a measurable and sometimes explanatory signal to the first, while the pentagon family and the contrary paths identify both the possibilities and the limits of turning that signal into mathematics.
