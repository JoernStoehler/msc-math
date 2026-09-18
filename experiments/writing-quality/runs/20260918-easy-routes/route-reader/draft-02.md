# Ridge geometry as a guide to candidate search

Random sampling can do more than return the largest systolic ratio encountered. By measuring geometry alongside capacity, it can reveal patterns that suggest where to look next and which geometric questions deserve an explanation. The experiments considered here use this approach to search for bodies with large systolic ratio. Their principal observation concerns a sum of symplectic areas of two-dimensional faces. An exact calculation for rotated pentagon products explains the observed direction on one family; further experiments show both the usefulness and the limits of using that observation for search.

The initial population comprised 4,096 generic four-dimensional polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian products of polygons. The latter contributed 1,024 bodies in each of the ten polygon-size buckets with $3\leq k\leq m\leq6$. Generation used seed 42 and support heights in $[0.8,1.2]$, retaining accepted geometries. Thus the population represents particular generation procedures and their acceptance conditions, rather than a uniform sample of convex bodies. Its largest stored systolic-ratio value was approximately $0.862586$, with none above one. These are historical numerical evaluator observations: the original computation records do not establish that they satisfy the guarantees of the current certified capacity implementation. We write their target as $\widehat{\operatorname{sys}}$, reserving $\operatorname{sys}=c_{\mathrm{EHZ}}^2/(2\operatorname{vol}_4)$ for the mathematical quantity.

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
The last equality follows by pairing the cosine terms on $[-\pi/10,\pi/10]$, where their signs are fixed, then using the evenness and $\pi/5$ periodicity of the absolute-cosine sum. The capacity profile proved in the pentagon analysis gives
\[
 \operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta).
\]
Combining the two yields the exact relation
\[
 \operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).
\]
Thus, within this family, decreasing the ridge descriptor is precisely equivalent to increasing systolic ratio. Alignment has ratio approximately $0.947214$; at $d=\pi/10$, the ratio is $(3+\sqrt5)/5\approx1.047214$. This is an analytic example outside the finite random table, so its value above one does not conflict with that table's recorded maximum. The identity supplies a later geometric interpretation, not a claim that the exploratory sampling was designed to test it.

The restriction to this family is essential. On two retained designed product paths, of types $3\times6$ and $4\times4$, decreasing $R$ also decreased the numerical systolic ratio at every retained step, ending at ratios $3/4$ and $1/2$, respectively. Small ridge sum is therefore not a universal ascent objective. A population association can still be useful for choosing a promising starting point without prescribing a direction of improvement from that point.

Candidate-selection experiments address that more limited use. Predictive models and feature ablations found that ridge-area features carried much of the held-out signal. A more direct search test generated 100,000 product candidates, selected bodies using geometric features before evaluating their targets, and then evaluated a selected/control union of 1,675 bodies. The distinction matters: generating inexpensive candidates did not require 100,000 capacity evaluations. The largest stored target was approximately $0.867546$, and none exceeded one. A separate fresh 100,000-candidate experiment tested a refinement: after retaining the lowest 1% by $R$ within each product bucket, prefer bodies whose total ridge area is less concentrated in their largest ridge. The selected tail was split into halves by that largest-ridge share. Before target evaluation, success was defined as a larger mean target in the less concentrated half than in its complement, both overall and in at least seven of ten buckets; no minimum effect size was required. The observed overall difference was about $0.034$, with positive differences in eight buckets. This is a concrete additional selection benefit under the frozen design, not a significance test. Frozen ridge and rho selectors also increased mean target against disjoint controls in both $4\times6$ and $6\times6$ buckets of a separately area-normalized source, with 91 distinct targets evaluated. These findings support finite enrichment under the tested designs; they do not identify a universally best selection rule.

Geometry-guided selection and local refinement consequently answer different questions. The former allocates expensive evaluations among proposed bodies; the latter tries to improve a chosen body. The companion optimizer study develops the latter using capacity's branch structure. Here the ridge observation supplies a concrete connection between population statistics, an exact family calculation, and candidate selection. Its positive content survives the failure of a universal ridge objective: the descriptor helps organize search, while the pentagon identity and opposing paths delimit what that organization can explain.
