# Lagrangian addition preserves equality under contact and projection matching

29 October 2026. Developed jointly by `/root/rank_four` and
`/root/rank_four/paired_inverse`. Scoped independent mathematical audit passed; see `equality-audit.md`.
This conditional equality-preserving operation does not prove the unrestricted
rank-four inequality.

## Conventions

Let $V$ be a four-dimensional symplectic vector space with symplectic volume
$\mu=\omega^2/2$. For a finite generator list $z_1,\ldots,z_n$, with
zonotope $Z=\sum_i[-z_i,z_i]$, put

\[
 \Phi(Z)=\sum_{i<j}|\omega(z_i,z_j)|,\qquad
 \Psi(Z)=\sum_{i<j<k<l}|\mu(z_i,z_j,z_k,z_l)|.
\]

The second summand is the absolute Pfaffian of the four-generator pairing
matrix. Thus equality means $4\Psi(Z)=\Phi(Z)^2$. Define the contact
functionals for a direction $h\in V$ by

\[
 a_Z(h)=\sum_i|\omega(h,z_i)|,\qquad
 b_Z(h)=\sum_{i<j<k}|\mu(h,z_i,z_j,z_k)|.
\]

Repeated or parallel generators are allowed in these sums; their zero
contributions cause no issue.

## Proposition

Suppose $Z$ is an equality zonotope, $4\Psi(Z)=\Phi(Z)^2$, and $L\subset V$
is a Lagrangian plane. Let $P=\sum_a[-h_a,h_a]\subset L$ be a planar
zonotope. Choose a complementary Lagrangian plane $L^*$ and compatible
coordinates so that, for each old generator $z_i$, its projection $p_i \in L^*$ satisfies the identity below after the quarter-turn identification $R:L^*\to L$.



\[
 \det_L(h,Rp_i)=\omega(h,z_i)\quad(h\in L).
\]
Let $Q=\sum_i[-Rp_i,Rp_i]\subset L$ be this rotated complementary
projection. Assume:

1. every added generator is a contact direction:
   \(b_Z(h_a)=\frac{\Phi(Z)}2a_Z(h_a)\);
2. $Q=cP$ as planar zonotopes for some $c>0$.

Then for every $t\ge0$,  $Z+tP$ is an equality zonotope:
\[
 4\Psi(Z+tP)=\Phi(Z+tP)^2.
\]

## Proof

Write $M=\sum_a a_Z(h_a)=\sum_{a,i}|\omega(h_a,z_i)|$. Pairings among
added generators vanish because $L$ is Lagrangian, hence
\[
 \Phi(Z+tP)=\Phi(Z)+tM.
\]
In the four-generator sum, at most two added vectors can occur in a nonzero
determinant: any three lie in the two-dimensional plane $L$. The linear
coefficient is \(\sum_a b_Z(h_a)=\Phi(Z)M/2\), by the contact assumption.

For the quadratic coefficient, with two added generators and two old ones,
compatibility of the Lagrangian splitting gives
\[
 |\mu(h_a,h_b,z_i,z_j)|
 =|\det_L(h_a,h_b)|\,|\det_L(Rp_i,Rp_j)|.
\]
Summing independently over pairs yields
\[
 [t^2]\Psi(Z+tP)
 =\Big(\sum_{a<b}|\det_L(h_a,h_b)|\Big)
  \Big(\sum_{i<j}|\det_L(Rp_i,Rp_j)|\Big)
 =\frac{\operatorname{area}(P)\operatorname{area}(Q)}{16},
\]
using the planar zonotope convention
\(\operatorname{area}(\sum[-x_i,x_i])=4\sum_{i<j}|\det(x_i,x_j)|\).
For homothetic planar zonotopes, the planar Minkowski area polynomial
\(\operatorname{area}(P+sQ)=\operatorname{area}(P)+4Ms+\operatorname{area}(Q)s^2\)
has zero discriminant, equivalently
\(\operatorname{area}(P)\operatorname{area}(Q)=4M^2\). Thus the quadratic
coefficient is $M^2/4$. Coefficients of $t^k$ for $k\ge3$ vanish. Since
$\Psi(Z)=\Phi(Z)^2/4$, this gives
\[
 \Psi(Z+tP)=\frac{\Phi(Z)^2}{4}+\frac{\Phi(Z)M}{2}t+\frac{M^2}{4}t^2
 =\frac{(\Phi(Z)+tM)^2}{4}.
\]

## Exact 10-generator example

Use the skew form with matrix
\[
 A=\begin{pmatrix}
 0&1&2&2\\-1&0&2&-2\\-2&-2&0&1\\-2&2&-1&0
 \end{pmatrix},\qquad A^2=-9I,\quad\operatorname{Pf}(A)=9,
\]
and the base generators $e_1,\ldots,e_4,w_1,\ldots,w_4$, where
$w_i=-Ae_i/3$. Then \(\Phi(Z)=32,\Psi(Z)=256\), using symplectic volume
\(\mu=\omega^2/2=9\,dx_1\wedge\cdots\wedge dx_4\).

Let $x=(0,0,1,-1)$, $L=\operatorname{span}(e_1,x)$, and add the four half
generators
\[
 h_1=e_1/6,\quad h_2=w_2/6,\quad
 h_3=(4e_1+x)/36,\quad h_4=x/12.
\]
The plane is Lagrangian since \(\omega(e_1,x)=0\). Each unscaled direction
$e_1,w_2,4e_1+x,x$ has respectively
$(a_Z,b_Z)=(8,128),(8,128),(36,576),(12,192)$; hence each satisfies
$b_Z(h)=16a_Z(h)=(\Phi(Z)/2)a_Z(h)$. Their scaled $a_Z$-masses sum to
\(M=4/3+4/3+1+1=14/3\).

In coordinates relative to \((e_1,x)\), the four added generators group by
direction as follows (slope is second coordinate divided by first):

| Direction slope | Total coefficient in $P$ | Total coefficient in $Q$ |
|---:|---:|---:|
| 0 | 1/6 | 6 |
| -2 | 1/18 | 2 |
| 1/4 | 1/9 | 4 |
| vertical | 1/12 | 3 |

For transparency, the rotated projections $q(z)=(-\omega(x,z),\omega(e_1,z))$
of the eight old generators are
\[
 (0,0),(4,1),(-1,2),(-1,2),(0,3),(0,0),(-3,0),(3,0).
\]
The table shows $Q=36P$ as zonotopes, not just equality of their areas.
The proposition therefore gives
\[
 \Phi(Z+tP)=32+\frac{14}{3}t,\qquad
 \Psi(Z+tP)=256+\frac{224}{3}t+\frac{49}{9}t^2
 =\frac14\Big(32+\frac{14}{3}t\Big)^2
\]
for every $t \ge 0$. This is a mixed-volume explanation of the exact ray;
it avoids expanding all four-Pfaffians of the ten-generator list.
