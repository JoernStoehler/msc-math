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

