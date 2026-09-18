# Ridge geometry as a guide to candidate search

Capacity evaluation is the expensive step in a search for polytopes with large systolic ratio. A geometric quantity that is cheaper to compute could help choose which bodies to evaluate. To find such quantities, we sampled polytopes, computed geometric measurements alongside numerical capacity values, and looked for statistical relationships with the systolic ratio. A sum of symplectic ridge areas showed a particularly strong association. We investigate what this association means geometrically and whether it can help select new candidates.

## An association in the sampled population

For a four-dimensional polytope $K$, define
\[
 R(K)=\frac{1}{\sqrt{\operatorname{vol}_4(K)}}
       \sum_{F\text{ a two-face of }K}\left|\int_F\omega_0\right|,
\]
where $\omega_0$ is the standard symplectic form. The absolute value is taken separately on each face, so contributions cannot cancel between faces. This is symplectic area, rather than Euclidean area: a face in a Lagrangian plane contributes zero. Dividing by the square root of volume makes $R$ invariant under common dilation.

The initial population contained 4,096 generic four-dimensional polytopes and 10,240 Lagrangian products of polygons. There were 512 generic polytopes at each facet count from five to twelve and 1,024 products in each of the ten polygon-size buckets $3\leq k\leq m\leq6$. Generation used seed 42 and support heights in $[0.8,1.2]$, retaining geometries accepted by the construction procedure. These choices specify a sampled population; they do not define a uniform distribution on convex bodies.

The targets in this table are historical numerical estimates, denoted by $\widehat{\operatorname{sys}}$. Their original computation records do not establish the guarantees of the current capacity implementation. We distinguish them from the mathematical quantity $\operatorname{sys}=c_{\mathrm{EHZ}}^2/(2\operatorname{vol}_4)$. The largest stored target was approximately $0.862586$, and none exceeded one.

Smaller $R$ accompanied larger targets: the Spearman rank correlation was $-0.938437$, compared with a Pearson correlation of $-0.205039$. The strong relationship is therefore one of ordering, with much weaker linear correlation. Its interpretation also depends on the composition of the population. Products had a mean target about $0.0491$ above generic polytopes, and a correlation in their pooled data need not persist within every family. The observation suggests two questions: can we explain the relation on a tractable family, and does it help select candidates beyond this table?

## A geometric explanation on pentagon products

Let $P$ lie in the $q$-plane and $Q$ in the $p$-plane, with cyclic edge vectors $a_i,b_j$ and areas $A_P,A_Q$. The two-faces of $P\times_L Q$ are copies of the polygon factors and mixed rectangles formed from pairs of edges. The factor faces are Lagrangian. A mixed rectangle contributes $|a_i\cdot b_j|$, giving
\[
 R(P\times_L Q)=\frac{\sum_{i,j}|a_i\cdot b_j|}{\sqrt{A_PA_Q}}.
\]
Thus $R$ measures the total absolute pairing of the edge vectors. Rotating one factor changes those pairings while preserving volume and combinatorial type.

Take a regular pentagon $P_5$ of circumradius one and put
$K_\theta=P_5\times_L\operatorname{Rot}(\theta)P_5$. Write
\[
 d(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/5|\in[0,\pi/10],
 \qquad s=\sin(\pi/5),\quad h=\cos(\pi/5).
\]
Each edge has length $2s$ and each polygon has area $5sh$, so the product formula gives
\[
 R(K_\theta)=\frac{4s}{h}\sum_{j=0}^{4}|\cos(\theta+2\pi j/5)|
            =16s\cos d(\theta).
\]
For the last equality, on $[-\pi/10,\pi/10]$ the terms indexed by $0,1,4$ are nonnegative and those indexed by $2,3$ nonpositive. Pairing them reduces the sum to
$[1+2\cos(2\pi/5)-2\cos(4\pi/5)]\cos\theta=4h\cos\theta$.
Evenness and $\pi/5$ periodicity extend the expression to every angle.

The capacity profile proved in the pentagon analysis is
\[
 \operatorname{sys}(K_\theta)=\frac{5+2\sqrt5}{10}\sec^2d(\theta).
\]
Consequently,
\[
 \operatorname{sys}(K_\theta)R(K_\theta)^2=16(3+\sqrt5).
\]
On this family, decreasing the ridge descriptor is exactly equivalent to increasing systolic ratio. The ratio rises from approximately $0.947214$ at alignment to $(3+\sqrt5)/5\approx1.047214$ at $d=\pi/10$. These analytically determined bodies were not part of the finite random table. The identity provides a later interpretation of its statistical observation; it was not a prediction tested by the original sampling.

The same direction does not hold on all product families. Two designed paths, of types $3\times6$ and $4\times4$, had both decreasing $R$ and decreasing numerical systolic ratio at every retained step, ending at ratios $3/4$ and $1/2$. Thus the pentagon identity cannot justify minimizing $R$ as a general ascent method. It remains possible that low $R$ selects promising bodies from a population even though decreasing it along a particular path makes the target worse.

## Selecting new candidates

Models fitted to the original table, together with feature ablations, found that ridge-area features carried much of the held-out predictive signal. Candidate selection asks for a further test: compute geometry first, select bodies without their targets, and only then evaluate capacity. This tests the use for which a cheap descriptor is intended.

The first such experiment generated 100,000 product candidates and applied geometric selection rules. Capacity was evaluated on the union of selected bodies and controls, comprising 1,675 distinct candidates. The largest stored target was approximately $0.867546$, with none above one. The experiment demonstrates the distinction between proposing a large pool and paying for target evaluation on that entire pool; the generated count must not be interpreted as 100,000 capacity evaluations.

A second experiment asked whether another feature improves selection within the low-$R$ tail. Two products can have similar total ridge area but distribute it differently among their faces. The proposed refinement favored a smaller largest-ridge share of that total. On a fresh pool of 100,000 products, the lowest 1% by $R$ in each bucket was split into equal halves according to largest-ridge share. Before target evaluation, the criterion was fixed: the less concentrated half must have a larger mean target overall and in at least seven of ten buckets. It did so, with an overall difference of about $0.034$ and positive differences in eight buckets. This establishes an incremental selection benefit on the specified sample. The criterion required no minimum effect size and was not a significance test.

A separate transfer experiment changed the source of proposed bodies, using a separately area-normalized construction. In both the $4\times6$ and $6\times6$ buckets, a frozen ridge selector increased mean target relative to disjoint controls. Across all selectors and controls, the study evaluated 91 distinct targets. This tests transfer to one named alternative source; it does not establish the rule's performance under arbitrary sampling procedures.

Together, these experiments give the ridge descriptor two roles. On regular pentagon products it participates in an exact geometric relation with capacity. In sampled product populations it can help allocate capacity evaluations, with concentration supplying an additional refinement in the tested design. The opposing paths distinguish this selection role from local improvement. The companion optimizer study addresses how to improve a chosen body using capacity's branch structure; a favorable population association alone does not supply that direction.
