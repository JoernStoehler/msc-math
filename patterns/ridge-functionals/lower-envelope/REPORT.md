# Exact counterexample to the maximum-ridge pentagon lower envelope

The proposed inequality

$$c(P\times_LQ)A_{\max}\geq\frac45\operatorname{area}(P)\operatorname{area}(Q)$$

fails for genuine convex pentagons. This settles the lower-envelope question left open in the [parent ridge-functional report](../REPORT.md). It does not alter the exact identity on independent affine-regular pentagon factors, and it does not establish the exact capacity of the counterexample.

Take the following integer vertices in counterclockwise order:

$$
P=\operatorname{conv}\{(1,-2),(1,3),(-1,4),(-1,-1),(0,-4)\},
$$
$$
Q=\operatorname{conv}\{(0,-2),(2,-1),(1,1),(-1,2),(-2,0)\}.
$$

All five edges of each factor have positive length; every other vertex lies strictly inside each edge's supporting halfplane. Both contain the origin strictly inside. Direct exact geometry gives

$$\operatorname{area}P=25/2,\qquad\operatorname{area}Q=9,\qquad
 A_{\max}=\max_{i,j}|e_i\cdot f_j|=10.$$

A feasible five-facet Haim–Kislev word supplies $c(P\times_LQ)\leq7$. Therefore

$$
\frac{c(P\times_LQ)A_{\max}}{\operatorname{area}P\operatorname{area}Q}
\leq\frac{28}{45}<\frac45,
$$

and, equivalently,

$$
\operatorname{sys}(P\times_LQ)R_\infty(P\times_LQ)^2
\leq\frac{392}{2025}<\frac8{25}.
$$

The former upper bound is $2/9$ below the proposed lower bound as a fraction of that bound. Exact feasible-word arithmetic suffices; a capacity minimizer, complete enumeration and numerical capacity call are unnecessary.

## Independent geometric capacity bound

The linear change of coordinates

$$(Q_1,Q_2,P_1,P_2)=(q_1,q_2-q_1/2,p_1+p_2/2,p_2)$$

is symplectic: $dQ_1\wedge dP_1+dQ_2\wedge dP_2=dq_1\wedge dp_1+dq_2\wedge dp_2$. On this product, $Q_1\in[-1,1]$ and $P_1\in[-2,3/2]$. Hence its image lies in the symplectic cylinder whose base is the rectangle $[-1,1]\times[-2,3/2]$ of area $7$. Capacity normalization and monotonicity give the same upper bound $c\leq7$ without HK word enumeration. The capacity properties are stated in [the thesis preliminaries](../../../thesis/chapters/02-preliminaries-ehz-capacity.tex). Equivalently, bound the other canonical coordinate pair by a rectangle of area $400$ and use the [symplectic planar-product capacity formula](../../../thesis/chapters/11-product-position.tex), obtaining $c\leq\min(7,400)=7$.

The first and third edges of $P$ are parallel. Since affine maps preserve parallelism while regular pentagons have no parallel edges, this pair lies outside the independent affine-regular equality family.

## Exact word

Number each polygon's edges starting with zero in the vertex order displayed above, and use the repository's coordinates $(q_1,q_2,p_1,p_2)$. The word and its positive weights are

| Facet | Embedded polar normal | Weight |
| --- | --- | --- |
| $P_0$ | $(1,0,0,0)$ | $1/4$ |
| $Q_1$ | $(0,0,2/3,1/3)$ | $3/14$ |
| $P_2$ | $(-1,0,0,0)$ | $1/4$ |
| $Q_3$ | $(0,0,-1/2,1/4)$ | $2/21$ |
| $Q_4$ | $(0,0,-1/2,-1/2)$ | $4/21$ |

The weights sum to one, each factor has mass $1/2$, and their weighted normal sum is zero. With $\omega((q,p),(q',p'))=q\cdot p'-p\cdot q'$,

$$Q_{\mathrm{HK}}=\sum_{i<j}\beta_i\beta_j\omega(a_i,a_j)=1/14.$$

The [maintained HK convention](../../../topics/capacity-and-qp/README.md#formula-word-conventions-and-the-kkt-witness) identifies capacity as the reciprocal of twice the maximum feasible objective, giving $c\leq1/(2Q_{\mathrm{HK}})=7$. The capacity implication imports that finite formula; the certificate independently checks all displayed geometry and arithmetic.

## Selection and bounded discovery

End state: one rigorously feasible HK word violating the conjectured bound, or an explicitly bounded non-result. An exact witness cannot be skipped because the conjecture concerns all pentagons and floating geometry could create a false counterexample. Full capacity evaluation can be skipped because a capacity upper bound already refutes a lower bound on $cA_{\max}$.

Three options were compared: independent affine-regular pentagons with nonaffine perturbations, deformations of pentagons obtained by deleting a vertex of the known symmetric hexagon control, and arbitrary centered convex pentagons. A lowest-value rubric targets the conjecture but can reward degenerate or floating-invalid inputs; a diversity/count rubric rewards bulk data without settling anything. We selected exact rationalizable separation, checking strict genuine pentagons, over extra coverage or numerical polishing.

All six simple hexagon deletions paired with their quarter-turn copies gave a floating feasible-word normalized bound $0.96$, above $0.8$. The first seeded local run on independent affine-regular factors found a strict floating violation after three feasible pair evaluations: $0.7858133132689084$. Optimization stopped immediately. Coarse rounding of both factors to half-integer coordinates, followed by uniform scaling to integers, yielded the simpler exact pair above. Later families were not run. The hexagon checks and numerical regular equality control are discovery diagnostics, not proof premises. The search stayed within the assigned 15-minute local budget; no installation, network, retained-population regeneration or full capacity producer was used.

The [search receipt](search-receipt.json) records the seed, three evaluated pairs and stopping condition; [floating discovery](floating-discovery.json) preserves the pre-rounding candidate. The discovery loop took about $0.003$ seconds, excluding setup, reading and rational verification. This is a falsification result, not a statistical performance estimate or an exhaustive search. Mathematical novelty and human acceptance are unclaimed.

## Reproduce

The retained verifier uses only the standard library:

```sh
python3 patterns/ridge-functionals/lower-envelope/exact_verify.py
```

It reconstructs every polar facet from the primal vertices, checks strict convexity, origin containment, nonredundancy, all 25 edge pairings, closure, weights and the strict rational violation. Its complete output is [certificate.json](certificate.json). The separate [root verifier](independent_verify.py) reconstructs the geometry and witness afresh and checks the cylinder ranges, with [independent output](independent-verification.json):

```sh
python3 patterns/ridge-functionals/lower-envelope/independent_verify.py \
  patterns/ridge-functionals/lower-envelope/certificate.json
```

Optional discovery reproduction uses the already available NumPy dependency and writes new outputs only to the selected scratch directory:

```sh
OPENBLAS_NUM_THREADS=1 python3 patterns/ridge-functionals/lower-envelope/discover.py \
  --out /tmp/msc-math/lower-envelope-reproduce
```

The general support-three enumeration is guided by the [product closure reduction](../../../topics/products/product-qp-six-facet-reduction.tex); the final five-facet certificate needs only feasibility in the full HK family.

[review: /root/batch_review]
