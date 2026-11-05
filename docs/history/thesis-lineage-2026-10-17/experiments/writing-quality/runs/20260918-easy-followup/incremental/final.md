# Ridge geometry as a guide to candidate search

Random sampling can do more than return the largest systolic ratio encountered. By measuring geometry alongside capacity, it can reveal patterns that suggest where to look next and which geometric questions deserve an explanation. The experiments considered here use this approach to search for bodies with large systolic ratio. Their principal observation concerns a sum of symplectic areas of two-dimensional faces. An exact calculation for rotated pentagon products explains the observed direction on one family; further experiments show both the usefulness and the limits of using that observation for search.

The initial population comprised 4,096 generic four-dimensional polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian products $P\times_LQ=\{(q,p):q\in P,\ p\in Q\}$, with polygons $P,Q\subset\mathbb R^2$ in the complementary $q$- and $p$-planes. The latter contributed 1,024 bodies in each of the ten polygon-size buckets with $3\leq k\leq m\leq6$, where $k$ and $m$ count the sides of the two factors. Generation used seed 42 and support heights in $[0.8,1.2]$, retaining accepted geometries. Thus the population represents particular generation procedures and their acceptance conditions, rather than a uniform sample of convex bodies. Its largest stored systolic-ratio value was approximately $0.862586$, with none above one. These are historical numerical evaluator observations: the original computation records do not establish that they satisfy the guarantees of the current certified capacity implementation. We write their target as $\widehat{\operatorname{sys}}$, reserving $\operatorname{sys}=c_{\mathrm{EHZ}}^2/(2\operatorname{vol}_4)$ for the mathematical quantity.

For a four-dimensional polytope $K$, define the ridge descriptor
\[
 R(K)=\frac{1}{\sqrt{\operatorname{vol}_4(K)}}
       \sum_{F\text{ a two-face of }K}\left|\int_F\omega_0\right|.
\]
Here $\omega_0$ is the standard symplectic form. Absolute values are taken face by face: cancellation between differently oriented faces is not allowed. This measures symplectic rather than Euclidean area. Its normalization removes common scale, since both the face integrals and the square root of four-dimensional volume scale quadratically.

Smaller $R$ strongly accompanied larger stored targets in this population. The Spearman rank correlation was $-0.938437$, whereas the Pearson correlation was only $-0.205039$. The finding is therefore much better described as an ordering tendency than as a linear relationship. It also belongs to a mixture of sources: products had a mean target about $0.0491$ above generic polytopes. A correlation over the mixture does not establish the same relationship within every component, still less for all convex bodies. The useful question is whether the descriptor carries geometric information that can help select candidates beyond the rows in which it was observed.

Products make that information more concrete. Put $P$ in the $q$-plane and $Q$ in the $p$-plane, and let $a_i,b_j$ denote their cyclic edge vectors. For $K=P\times_LQ$, the factor faces lie in Lagrangian planes and contribute zero. Each mixed rectangle, formed from one edge of each factor, contributes $|a_i\cdot b_j|$. Consequently
\[
 R(P\times_LQ)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}},
\]
where $A_P,A_Q$ are the polygon areas. The descriptor measures the total absolute pairing of edge directions and lengths. Relative rotation can change it while leaving volume and combinatorial type unchanged.

Consider in particular a regular pentagon $P_5$ of circumradius one and
$K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5$. Reduce the rotation to
$d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|\in[0,\pi/10]$.
Writing $s=\sin(\pi/5)$ and $h=\cos(\pi/5)$, each edge has length $2s$ and each polygon area $5sh$. The preceding expression becomes
\[
 R(K_\theta)=\frac{4s}{h}\sum_{j=0}^{4}|\cos(\theta+2\pi j/5)|
            =16s\cos d(\theta).
\]
The last equality follows by pairing the cosine terms on $[-\pi/10,\pi/10]$, where their signs are fixed, then using the evenness and $\pi/5$ periodicity of the absolute-cosine sum. The rotated-pentagon profile theorem (Theorem \ref{thm:rotated-pentagon-product-formula}) gives
\[
 \operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta).
\]
Combining the two yields the exact relation
\[
 \operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).
\]
Thus, within this family, decreasing the ridge descriptor is precisely equivalent to increasing systolic ratio. Alignment has ratio approximately $0.947214$; at $d=\pi/10$, the ratio is $(3+\sqrt5)/5\approx1.047214$. The endpoint is the Haim–Kislev–Ostrover counterexample discussed in the pentagon analysis. It lies outside the finite random table, so its value above one does not conflict with that table's recorded maximum. The identity supplies a later geometric interpretation, not a claim that the exploratory sampling was designed to test it.

Other product families give a warning against using the descriptor as an ascent objective. On two designed paths, of types $3\times6$ and $4\times4$, decreasing $R$ also decreased the recorded numerical target at every sampled step. Their endpoints have independently derived ratios $3/4$ and $1/2$, respectively, consistent with the endpoint evaluations. The intermediate values remain numerical observations on these two paths; they do not prove a general relation between $R$ and capacity. They do show why the population association alone is insufficient reason to drive a search by decreasing $R$. Selecting a promising starting point and finding a direction of improvement from it are different tasks.

The candidate-selection studies separate two steps: generating a large pool using inexpensive geometry, and choosing which bodies receive a target evaluation. Models and feature ablations on existing rows had identified ridge-area features as carrying much of the held-out predictive signal. The next question was whether a rule using those features could select useful new bodies before their targets were known.

An initial study generated 100,000 products and applied 30 geometric selection rules. Overlap between selections left 485 distinct selected bodies; including controls gave a union of 1,675 bodies for target evaluation. The maximum recorded target was approximately $0.867546$, and none exceeded one. This study demonstrated the practical separation of candidate generation from capacity evaluation. Its union maximum is not a comparison against controls, and cannot by itself establish a benefit from selection.

A fresh 100,000-candidate study tested one specific refinement. Each of the ten polygon-size buckets contained 10,000 candidates. Within each bucket, retain the 100 bodies with lowest $R$, then split these into two groups of 50 according to the share of total symplectic ridge area contributed by the largest ridge. The question is whether the less concentrated group has larger targets than the other half of the same low-$R$ tail. Both groups were fixed before evaluating their targets. This holds the initial low-$R$ selection rule fixed while testing whether the additional concentration feature improves it.

The predeclared criterion required a positive difference in mean target both across the pooled groups and in at least seven of the ten buckets. The observed pooled difference was approximately $0.034$, and eight bucket differences were positive. Thus the refinement satisfied its criterion on the new pool. No minimum effect size or significance threshold was part of that criterion: the result is an observed improvement for the specified selection rule and generator.

A smaller study transferred this two-stage rule to a different product source, with the two polygon factors separately normalized by area. From 3,200 eligible candidates in each of the $4\times6$ and $6\times6$ buckets, it selected the lowest 1% by $R$ and then the less concentrated half, leaving 16 selected bodies per bucket. Each was compared with a disjoint control group of 16 bodies. The mean recorded targets were:

| Polygon sizes | Selected | Control | Difference |
|---|---:|---:|---:|
| $4\times6$ | 0.637 | 0.371 | 0.266 |
| $6\times6$ | 0.708 | 0.523 | 0.185 |

The selected group had the larger mean in both buckets. This comparison uses 64 bodies; the study evaluated 91 distinct bodies in total because it also tested another selector, with some overlap between selected groups. It provides finite evidence of enrichment on one additional source, not a claim that the same gain holds for arbitrary product generators.

The retained study descriptions and results are located in the source notes below. Their comparisons support a use of ridge geometry narrower than a universal optimization objective: it can help allocate expensive evaluations among generated candidates. Local refinement asks how to improve a body already chosen and is developed in the companion optimizer study. The exact pentagon calculation and the opposing numerical paths help distinguish these questions. They explain why a descriptor can guide selection without providing an ascent direction at every starting point.

## Sources for the empirical comparisons

The historical population and feature analysis are documented in `experiments/sys-datascience/README.md` and `methods/standard-baseline-p2/README.md`. The candidate-generation study and concentration refinement are in `methods/extreme-scalar-rejection-proposer/README.md` and its `artifacts/100k-ridge-concentration-validation/` packet. The transfer protocol and bucket means are in `methods/alternative-source-transfer/README.md` and `artifacts/transfer-v1/analysis.json`. Here `methods/` abbreviates `experiments/sys-datascience/methods/`. These are retained study sources; final thesis assembly should replace this source note with stable study references.
