# Independent audit: eight-generator nonproduct equality family

I checked `nonproduct-equality.md` directly. The matrix identities, Pfaffian
count, equality condition, and geometric nonproduct argument are correct for
`p=x^2+y^2+z^2>0`, `t>=0`; in particular, `xyz != 0` gives a genuinely
nonproduct equality zonotope at `t=sqrt(p)`.

## Matrix and Pfaffian checks

Direct multiplication gives `A^2=-p I`; the Pfaffian is
`A12 A34-A13 A24+A14 A23=x^2+y^2+z^2=p`, and `||A||_1=2s`. Hence
`A^{-1}=-A/p`. The Schur complement of the top-left block in `W(t)` is
`(t^2/p)A+t^2 A^{-1}=0`, so `rank W(t)=4`.

The edge sum is `Phi=2s+2st^2/p+4t`. For the four-Pfaffian sum, the 4+0 and
0+4 classes contribute `p+t^4/p`. The 3+1 class contributes `4ts`: for
each fixed singleton index, the three complementary top-block edges have
magnitudes `|x|,|y|,|z|`. The 1+3 class analogously contributes
`4t^3s/p`. In the 2+2 class, summing the product formula over all 36 ordered
pair choices first gives `4s^2t^2/p`; for the six coincident pairs the
actual value `t^2(1-A_ij^2/p)` replaces the counted term
`t^2 A_ij^2/p`, giving the net correction
`6t^2-2(t^2/p) sum_{i<j}A_ij^2=2t^2`, since the sum of squares is `2p`.
Thus the stated formula
`Psi=(p+t^2+2ts)^2/p` is exact.

With `u=sqrt(p)`, the difference is exactly
`Phi/2-sqrt(Psi)=(s-u)(t-u)^2/p`. Since `s>=u`, equality occurs exactly
when `t=u` or `s=u`; the latter is equivalent to at most one of `x,y,z`
being nonzero. The family therefore proves equality on the asserted parameter
sets, not a strict inequality away from them.

## Geometric realization and nonproduct claim

For `J=A/u`, one has `J^T=-J`, `J^2=-I`, and `Pf(J)=1`. The pairing matrix
of `e_1,...,e_4,-Je_1,...,-Je_4` under `omega_J(v,w)=v^T Jw` is exactly
`[[J,I],[-I,J]]`; their zonotope is `C+JC`. The scale-normalized family
`W(sqrt(p))/sqrt(p)` is this matrix. An orientation-compatible orthogonal
complex structure on `R^4` is orthogonally conjugate to the standard one, so
these yield standard-symplectic examples after the corresponding orthogonal
change of coordinates. At this normalization,
`Phi=4(1+s/u)` and `vol=16 Psi=64(1+s/u)^2=4Phi^2`.

When `xyz!=0`, every `Je_i` has three nonzero standard coordinates. Therefore
no such vector lies in a coordinate two-plane, proving independence of any
triple containing two `e_i` and one `Je_j`. Applying the invertible map `J`
gives the corresponding assertion for one `e_i` and two `Je_j`; triples
within either orthonormal basis are independent. The eight generator lines
are distinct. Each is an edge direction of the full-dimensional zonotope,
and edge directions are intrinsic to a zonotope. A linear image of a product
of planar zonotopes has all edge directions in the union of two complementary
two-dimensional subspaces. Each such subspace could contain at most two of
these eight directions, since any three of the directions are independent;
their union could contain at most four. This contradicts the eight intrinsic
edge directions, so the body is not linearly a product of two planar bodies.

For `x=y=z=1`, the support graph consists of two internal `K_4` graphs and
a cross-block perfect matching. Its complement is the cube graph, and each
vertex has degree four as stated. No broader extremal or stability conclusion
follows from this equality family alone.
