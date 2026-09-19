# Ridge geometry as a guide to candidate search

A geometric measurement can guide the search for large systolic ratios in two different ways. It might identify promising bodies among newly generated candidates, or indicate how to improve a body already chosen. These uses require different evidence. Our ridge-area experiments support the first use and give an exact example of the second, but also show why the two cannot generally be identified. Their purpose is to extract geometric structure from random samples, then ask which parts of that structure survive mathematical analysis and further search.

For a four-dimensional polytope \(K\), define

\[
R(K)=\frac{1}{\sqrt{\operatorname{vol}_4(K)}}
       \sum_{F\text{ a two-face of }K}
       \left|\int_F\omega_0\right|.
\]

The absolute value is taken separately on each face, so different faces cannot cancel through their orientations. This measures symplectic area rather than Euclidean area. The normalization makes \(R\) unchanged under common dilation: both a face integral and the square root of four-volume scale quadratically.

The exploratory population comprised 4,096 generic polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian polygon products, with 1,024 in each bucket \(3\leq k\leq m\leq6\). Generation used seed 42 and support heights in \([0.8,1.2]\), subject to acceptance conditions. Consequently, the observed population is specific to this sampling procedure. It is not a distribution-free sample of convex bodies.

Write \(\widehat{\operatorname{sys}}\) for the historical numerical targets, distinguishing them from the mathematical ratio

\[
\operatorname{sys}(K)=\frac{c_{\mathrm{EHZ}}(K)^2}
                              {2\operatorname{vol}_4(K)}.
\]

The retained data and their association analysis are reproducible, but incomplete original evaluator records prevent attributing these targets to the current certified implementation. Reproducing their correlation does not certify either their absolute accuracy or their ordering by mathematical capacity. On these recorded targets, \(R\) had Spearman correlation \(-0.9384\), whereas Pearson correlation was only \(-0.2050\). Smaller ridge sums thus strongly accompanied larger targets in rank, without suggesting a comparable linear relation. This coefficient describes the pooled population. Products had a mean target about \(0.0491\) above the generic group, so source composition matters; the pooled result alone does not establish the same association within every bucket. The largest recorded target was \(0.862586\), and none exceeded one.

For products, the descriptor has a direct geometric interpretation. Let \(P\) lie in the \(q\)-plane and \(Q\) in the \(p\)-plane, with cyclic edge vectors \(a_i,b_j\) and areas \(A_P,A_Q\). Their factor faces are Lagrangian and contribute zero. Each remaining face is a rectangle spanned by \((a_i,0)\) and \((0,b_j)\). Since \(\omega_0((q,p),(q',p'))=q\cdot p'-p\cdot q'\), its unsigned contribution is \(|a_i\cdot b_j|\). Therefore

\[
R(P\times_L Q)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]

Relative rotation changes these edge pairings while preserving both polygon shapes and the volume of their product. This supplies a way to study the descriptor without changing all geometric variables at once.

Consider specifically

\[
K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5,
\qquad
 d=\min_{j\in\mathbb Z}|\theta-j\pi/5|\in[0,\pi/10],
\]

where \(P_5\) is the regular pentagon of circumradius one. Put \(s=\sin(\pi/5)\) and \(h=\cos(\pi/5)\). Its edges have length \(2s\), and its area is \(5sh\). Grouping edge pairs by their five relative directions gives

\[
R(K_\theta)=\frac{4s}{h}
 \sum_{j=0}^4|\cos(\theta+2\pi j/5)|.
\]

The cosine sum is even and \(\pi/5\)-periodic. On the reduced interval its signs are fixed, and pairing the terms yields \(4h\cos d\). Hence

\[
R(K_\theta)=16\sin(\pi/5)\cos d.
\]

Determining capacity requires a separate argument. The pentagon capacity proof uses the variational formula: feasible facet weights remain unchanged under rotation, while each fixed-weight quadratic expression has the form \(A\cos\theta+B\sin\theta\). Endpoint bounds and an explicit attaining six-facet word determine the capacity throughout the reduced interval. That proof gives

\[
c_{\mathrm{EHZ}}(K_\theta)=(1+h)^2\sec d,
\qquad
\operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d.
\]

Combining the capacity profile with the edge calculation gives the exact relation

\[
\boxed{\operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).}
\]

Thus, within this family, decreasing \(R\) is equivalent to increasing the systolic ratio. Neither volume nor combinatorial type changes. At \(d=\pi/10\), the known HKO position has ratio \((3+\sqrt5)/5\approx1.047214\). This exact structured example and the random table's subunit maximum describe different evidence; the finite screen places no bound on the family. The identity provides a later analytic interpretation of the descriptor, not a formula that prospectively predicted the original observations.

Its restriction to this family is substantive. Along two designed product paths, of types \(3\times6\) and \(4\times4\), the retained numerical ratios decreased together with \(R\) at every recorded step. Their endpoints have mathematical ratios \(3/4\) and \(1/2\), respectively. These numerical path observations do not have the proof status of the pentagon identity, but they provide direct counterevidence to using ridge reduction as an unconditional search rule. A population association cannot by itself determine the effect of deforming an individual body.

Candidate selection asks a less demanding question: can descriptors concentrate promising targets before capacity is evaluated? In one experiment, a pool of 100,000 generated products supplied 30 selection sets containing 485 distinct selected candidates. The selected/control union contained 1,675 bodies; only those bodies received target evaluations. Its maximum \(\widehat{\operatorname{sys}}\) was \(0.867546\), with no value above one. The large generation count must therefore be distinguished from the evaluation budget.

A separate transfer experiment used frozen rho and ridge selectors on a different, separately factor-area-normalized source, called `factorial-both`. Selected means exceeded disjoint-control means in both its \(4\times6\) and \(6\times6\) buckets, across 91 distinct evaluated bodies. This is finite evidence of enrichment under that design, not an isolated causal effect of \(R\) or a population guarantee. Together with the exact pentagon calculation and the adverse paths, it assigns the descriptor a useful but bounded role: prioritize candidates, then evaluate capacity directly. Local improvement using the capacity objective's branch structure is a complementary strategy, developed in the optimizer section.
