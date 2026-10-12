# From a ridge-area signal to a tested selection rule

Cheap geometric measurements could help search for large systolic ratios if they identify promising bodies before capacity is evaluated. The initial evidence is a strong association, but three different questions must be distinguished: whether a measurement orders a sampled population, whether geometry forces that ordering in a particular family, and whether a frozen rule enriches a newly generated population.

For a four-dimensional polytope, define
\[
 R(K)=\frac{\sum_{F}\left|\int_F\omega_0\right|}
 {\sqrt{\operatorname{vol}_4(K)}},
 \qquad \operatorname{sys}(K)=\frac{c_{\mathrm{EHZ}}(K)^2}
 {2\operatorname{vol}_4(K)},
\]
where the sum runs over two-dimensional faces, or ridges. Each face contributes its absolute symplectic area; this is neither Euclidean area nor a signed sum allowing cancellation. Both quantities are invariant under translation, common dilation and linear symplectic maps. Write \(\widehat{\operatorname{sys}}\) for the recorded numerical ratio, whose floating-point volume is not certified.

The exploratory population contains 4,096 generic polytopes, equally divided among facet counts five through twelve, and 10,240 Lagrangian polygon products, equally divided among the ten side-count pairs \(3\leq k\leq m\leq6\). Generation uses independent uniform unit normals and support heights in \([0.8,1.2)\), conditional on boundedness and retention of every proposed facet. The pooled Spearman correlation between \(R\) and the numerical ratio is \(-0.9384\), whereas Pearson correlation is only \(-0.2050\): the strong observation concerns ordering, not a linear response. Negative rank associations also occur within every sampled group. The largest stored ratio is \(0.862586\). Later reevaluation supplies certified capacity intervals for 14,335 of these 14,336 bodies, but does not certify their historical execution or complete ratios. These finite observations do not settle Viterbo's conjecture.

Product geometry provides an exact counterpart to the association. For polygons \(P,Q\) in the complementary Lagrangian coordinate planes, let \(a_i,b_j\) be their cyclic edge vectors and \(A_P,A_Q\) their areas. Factor-plane faces contribute zero, while each mixed rectangle contributes \(|a_i\cdot b_j|\). Hence
\[
 R(P\times Q)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]
For the centered unit-circumradius regular pentagon \(P_5\), set \(K_\theta=P_5\times\operatorname{Rot}(\theta)P_5\) and \(d=\operatorname{dist}(\theta,(\pi/5)\mathbb Z)\in[0,\pi/10]\). Its edge sum reduces to
\[
 R(K_\theta)=16\sin(\pi/5)\cos d.
\]
Combining this with the proved capacity profile gives
\[
 \operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d,
 \qquad \operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).
\]
Here relative rotation changes edge pairings while volume and combinatorial type remain fixed. Lower \(R\) therefore means higher ratio within this family. This exact explanation is a later interpretation of the exploratory signal, not a reconstruction of what originally motivated the screen.

The family restriction matters. Equilateral-triangle products have the opposite exact ordering: with reduced angle \(d_3\in[0,\pi/6]\), their ridge sum is \(8\sqrt3\cos d_3\) and ratio is \(\tfrac12\sec^2(\pi/6-d_3)\); both decrease as \(d_3\) increases. Moreover, the universal product bound \(R\geq8\) is attained by the cube, whose ratio is only \(1/2\). Two designed triangle–hexagon and square–square paths likewise decrease both measurements at their retained positions; these finite path observations do not establish monotonicity between positions. Thus a useful population signal need not be an objective for improving each selected body.

To test practical selection, candidates must be chosen without their targets. In the initial fresh pool of 100,000 products, selecting the ten smallest ridge sums per side-count group gave mean numerical ratio \(0.626\), against \(0.316\) for equal-size, disjoint hash-selected controls. Thirty scalar selections and their controls together required 1,675 distinct capacity evaluations, not 100,000. Further restriction was less reliable: in another low-ridge panel, groupwise rank correlations ranged from \(-0.315\) to \(+0.301\). A separate test nevertheless found incremental information in ridge concentration: among the lowest one percent by \(R\), the half with smaller largest-ridge share (maximum area divided by total area) had a mean advantage of \(0.034\). These comparisons establish finite enrichment without a universal ordering inside the selected tail.

A changed-source test asks whether that usefulness survives one specified change of distribution. The `factorial-both` source sets support heights to one, requires both original and changed factors to be admissible, and separately normalizes changed factors to area one. One seed supplied 3,200 eligible products in each of the \(4\times6\) and \(6\times6\) groups. Two previously frozen rules selected sixteen per group: the lowest \(0.5\%\) by vertex-covariance imbalance \(\rho=\nu_2/\nu_1\), where the ordered \(\nu_i\) are the Williamson eigenvalues of the centered covariance of distinct vertices, and the lowest \(1\%\) by \(R\) followed by its lower half in largest-ridge share. Each group also supplied sixteen disjoint controls. Five overlaps between selectors left 91 distinct evaluations.

Both rules raised mean numerical ratio in both groups. Equally weighted gains were \(0.259\) for covariance and \(0.225\) for ridge selection. None exceeded one. Shared controls and overlapping selections make the comparisons dependent; resampling ranges condition on these realized panels. The result is a successful transfer to this particular conditioned source, with no established mechanism, covariance superiority, or stability across other seeds and laws. Together, the exact family and the generated tests support geometric interpretation and useful finite selection while keeping their different evidential strengths visible.

Source note: historical population and replay are owned by [the closure account](../../../../../../topics/empirical-studies/evidence-closure/README.md); exact ridge correspondence by [ridge mathematics](../../../../../../topics/empirical-studies/evidence-closure/ridge-mathematics.md). The capacity profile is proved in [Chapter 9](../../../../../../thesis/chapters/09-rotated-regular-polygons.tex). Current reevaluation, triangle proof and conditional comparisons are recorded in [Chapter 8](../../../../../../thesis/chapters/08-data-science.tex) and [the DS appendix](../../../../../../thesis/appendices/data-science.tex). Fresh-selection provenance is owned by [the scalar-proposer packet](../../../../../../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/README.md); path limits by [the endpoint interpretation](../../../../../../topics/empirical-studies/sys-datascience/methods/ridge-endpoint-path/INTERPRETATION.md); transfer counts and effects by [the transfer README](../../../../../../topics/empirical-studies/sys-datascience/methods/alternative-source-transfer/README.md) and [its frozen analysis](../../../../../../topics/empirical-studies/sys-datascience/methods/alternative-source-transfer/artifacts/transfer-v1/analysis.json).
