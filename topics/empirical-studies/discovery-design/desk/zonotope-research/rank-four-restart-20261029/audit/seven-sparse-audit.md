# Independent audit: arbitrary sparse rows and the seven-generator reduction

I checked `sparse-degree-two.md`, `six-vertex/theorem7.md`, and the primary
source trace in `priorart/dimension-three-source.md`. I did not find a gap in
the degree-one/two argument or in the seven-generator sparsification, conditional
on the cited three-dimensional zonoid projection theorem and the established
five-generator result.

## Sparse-degree-two argument

The quoted input is Theorem 5.1 of Fradelizi–Madiman–Meyer–Zvavitch,
*On the volume of the Minkowski sum of zonoids* (JFA 286 (2024), 110247),
arXiv:2206.02123. I opened the primary source at
https://arxiv.org/html/2206.02123#S5 (Theorem 5.1, displayed (36)–(39)). For
orthogonal normals `e1,e3`, the inequality is

`|Z|_3 |P_{span(e1,e3)^perp} Z|_1
 <= |P_{e1^perp}Z|_2 |P_{e3^perp}Z|_2`.

For the centered zonotope with half-generators `(g_i,h_i,k_i)`, these four
quantities are `8T`, `2H`, `4B`, and `4P`, respectively. Thus the exact
normalization is `TH <= PB`, with no missing factor. The source's proof is an
actual theorem for 3D zonoids; it is separate from the stronger projection
conjecture that fails in dimension three.

For a degree-two row, the 6-Pfaffians using that row and any four indices of
the complementary block force every 4-Pfaffian of that block to vanish.
Consequently the block has rank at most two and is decomposable (or zero).
The 6-Pfaffians using the two neighbors, the sparse row, and three block
indices give `h wedge B=0`. If `h != 0`, this implies `B=h wedge k` for some
`k`, so `||g wedge B||_1=||g wedge h wedge k||_1`. Applying `TH<=PB` and the
planar strip lemma gives the stated auxiliary estimate. If `h=0`, then
`s=(beta/alpha)r`, `R=||g||_1`, and the direct coordinate wedge triangle
bound suffices. The decomposition of old minors into zero all-block terms,
the two one-neighbor terms, and the `cB-r wedge s` terms has the asserted
norms. New minors sum to `t(||B||_1+H)`.

The degree-one block argument is valid in all sizes: the same 6-Pfaffians
force its complementary skew block to have rank at most two, and the
coordinate wedge triangle bound gives
`||r wedge B||_1 <= ||r||_1 ||B||_1`. Small blocks where there are no
4-Pfaffians satisfy that rank conclusion by dimension automatically.

## Seven-generator reduction

The normalized rank-at-most-four set with `Phi=1` is compact, and `Psi` is
continuous, so the maximizer with least support exists. If deleting a row
leaves rank at most two, all old 4-Pfaffians vanish and the bound
`b(y)<=a Phi_0` follows by assigning each old edge to its possible new
third index. This rules out a value above `1/4`. When the old 6x6 block has
rank four, the block matrix has rank at most four exactly when `y` lies in
its range: the kernel of the old block has dimension two, and a nonzero
component of `y` in that kernel adds a rank-two skew block.

The fixed-support polytope `range(W_0) intersect R^S intersect {||z||_1<=a}`
is compact. The convex homogeneous function `b(z)` attains its maximum at a
vertex. The proof correctly shows this maximum is positive: if `b(y)=0`,
then `Psi_0=Psi(W)>0`, and padding the old block to norm one gives the
strictly larger normalized ratio `Psi_0/Phi_0^2 > Psi(W)/Phi(W)^2`.
Any maximizing vertex of norm below `a` can be scaled up, so it has norm
exactly `a`. Replacing the deleted column preserves `Phi`, rank, and global
maximality. Minimality of the total number of nonzero entries then forces the
replacement to have the same support size as the original column.

On a sign face with `k` nonzero coordinates, the `l1`-sphere face has
dimension `k-1`; `range(W_0)` imposes at most two independent constraints.
Thus a vertex can have at most three nonzero coordinates. This holds for
each row. If every degree were three, the degree sum would be 21, contradicting
the handshake lemma. Hence some row has degree at most two.

One small integration detail: if that row has degree zero, the matrix is
supported on at most six indices and the six-generator theorem should be
invoked directly. The sparse-degree proposition itself handles degrees one
and two; it cannot by itself dispose of an isolated row for arbitrary support
size. Since the six-generator theorem is now available, the seven-generator
proof closes this case immediately.

No arbitrary-generator theorem follows from the seven-generator result alone.
The sparse-row reduction controls degrees one/two in any dimension, but no
argument here establishes a row of such degree for a minimal-support
maximizer when there are eight or more generators.
