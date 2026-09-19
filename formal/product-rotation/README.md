# Rotating a planar product: proof and convention record

The reader-facing theorem, proof, triangle example and short consequence are in
the [product-position chapter](../../thesis/chapters/11-product-position.tex).
This supporting record retains the exact endpoint calculation omitted from that
section because an upper bound already proves the required endpoint comparison.

## Source and scope

Derived from Haim-Kislev, *On the symplectic size of convex polytopes*, Theorem 1.1
(`HK2017` in the thesis bibliography). The source uses balanced nonnegative facet
weights and maximizes an ordered pairing sum; reciprocal capacity has factor 2.
Support normalization absorbs facet support numbers. For the product, closure
holds separately in each factor. No trajectory regularity assumption is used.

Upstream reviewed source: empirical tranche `4a0448e3`, packet
`docs/empirical-viterbo-design/desk/product-position-research/README.md` and
`exact_endpoint_collision.py`. The empirical root accepted the initial-regime
proof and endpoint certificate in `docs/empirical-viterbo-design/desk/root-review/README.md`.
This packet selects only the universal improvement and one worse Lagrangian
endpoint; it does not import the profile catalogue, lift counterexample or any
novelty claim.

## Conjugation and physical rotation

Write $J_0=\operatorname{diag}(J_2,-J_2)$ and
$J_L=\left[\begin{smallmatrix}0&I\\-I&0\end{smallmatrix}\right]$. Then
$K=J_0J_L=\left[\begin{smallmatrix}0&J_2\\J_2&0\end{smallmatrix}\right]$
is skew-symmetric, $K^2=-I$, $KJ_0=J_L$, and $J_0K=-J_L$.
Consequently, with $R=\cos(\theta/2)I+\sin(\theta/2)K$,

$$R J_0R^T=\cos\theta J_0+\sin\theta J_L=J_\theta.$$

The pullback form on $A\times B$ induced by rotating the body by $Q=R^T$ is
$Q^T J_0 Q=J_\theta$. Thus the theorem concerns genuine Euclidean rotations;
volume is constant. Both orthogonal rotation planes have angle magnitude
$\theta/2$. The sign of one factor's initial area form does not affect its
planar capacity (area). At the other endpoint both factor planes are Lagrangian.
The identification of their coordinates is fixed, not optimized independently.

## Exact endpoint of the triangle example

Use the normals $u_i,v_j$ listed in the section. Both triples sum to zero and
span their factor planes, so all feasible product weights are $t/3$ on the
first triple and $(1-t)/3$ on the second.

Fix an order $u_i,u_j,u_k$. Since their sum is zero, the possible sums of the
$u$ normals preceding a given $v_r$ are $0,u_i,-u_k,0$. If that prefix sum is
$P$, the signed cross sum involving $v_r$ is $2P\cdot v_r$: the following
normals sum to $-P$. The largest total signed cross sum for this $u$ order is
therefore

$$2\sum_{r=0}^2\max(0,u_i\cdot v_r,-u_k\cdot v_r).$$

Each $v_r$ can be placed in its maximizing slot independently, because there
are no $v$--$v$ contributions at the Lagrangian endpoint. For each of the six
orders of the $u$ normals the displayed quantity is 5. Thus reciprocal capacity
is bounded above by $(10/9)t(1-t)\le5/18$. The alternating order in the section
attains equality with $t=1/2$. This proves $\ell=18/5$, not just the upper bound
used in the reader-facing example.

## Checks

`python3 formal/product-rotation/verify.py` uses rational arithmetic to verify:

- the support-normalized normals and closure;
- all six prefix-sum upper bounds and the attaining witness;
- the same maximum by independent enumeration of all 120 cyclic orders;
- the matrix identities implying the conjugation formula for every angle.

The initial interval proof is the factorwise mass split in the section. The
mixed normalization is $4t(1-t)$, not $2t(1-t)$; the sufficient condition is
$\tan\theta\le\ell/(2a)$. This interval need not be maximal. For arbitrary convex
factors, approximate by polygons and pass the reciprocal bounds (valid at every
fixed angle) to the limit. Equality under the stated condition then follows,
including its boundary. Translations to put the origin inside each factor are
harmless: their rotated images differ by a translation as well.

Independent read-only check by `product_rotation_writeup/verify_rotation`
confirmed these conventions, the universal initial formula and the triangle
constants. These checks establish neither literature novelty nor human prose
acceptance.
