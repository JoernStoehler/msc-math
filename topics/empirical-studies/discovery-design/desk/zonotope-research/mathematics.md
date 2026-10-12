# Pairing-volume candidate: derivations and boundaries

Use the symplectic matrix `J4 = [[0,I],[-I,0]]` and planar quarter-turn `J2`.
Signs of these choices do not affect the absolute quantities below.
Let `Z = sum_i [-v_i,v_i]` be a zonotope in R4 and put

\[
W_{ij}=\omega(v_i,v_j),\quad
\Phi=\sum_{i<j}|W_{ij}|,\quad
\Psi=\sum_{i<j<k<l}|W_{ij}W_{kl}-W_{ik}W_{jl}+W_{il}W_{jk}|.
\]

The standard zonotope volume formula and `omega wedge omega / 2` give
`vol4 Z = 16 Psi`. This is a known identity. The candidate is `4 Psi <= Phi^2`.
Every real skew matrix of rank at most four is realizable as `V J4 V^T`;
thus the rank-four algebraic formulation is exactly the geometric question.

## Low generator count, proved

Assign edge weights `a_ij = |W_ij|` to the complete graph on the generators.
The Pfaffian triangle inequality bounds Psi by the sum `a_e a_f` over
disjoint unordered edge pairs. Regard disjointness as adjacency in a new
graph. For at most five generators that graph has no triangle.

For a triangle-free graph with nonnegative vertex weights summing to Phi,
the edge-product sum is at most `Phi^2/4`. To see this, maximize the sum on
the weight simplex. If two nonadjacent vertices have positive weight, move
their combined weight to the one with larger weighted-neighbor sum; the
objective does not decrease and support shrinks. A maximizer therefore has
clique support. A clique has at most two vertices, and their product is at
most a quarter of their squared sum. This proves the candidate for at most
five generators.

For general generator count, the same triangle inequality only gives
`Psi <= Phi^2/2`, hence `vol <= 8 Phi^2`. Rank four must do additional work
to establish the proposed factor four.

## Planar products, classical explanation

Suppose `P = sum [-a_i,a_i]` and `Q = sum [-b_j,b_j]` are planar zonogons.
For the Lagrangian product, only cross pairings remain:

\[
\Phi(P\times_L Q)=\sum_{i,j}|a_i\cdot b_j|
=\tfrac12 V_2(P,J_2Q).
\]

Here `|P+tQ| = |P|+2t V_2(P,Q)+t^2|Q|`. The planar mixed-area inequality
`|P||Q| <= V_2(P,J2Q)^2` proves the candidate for every such product.
Equality holds when `P` and `J2Q` are homothetic. In particular all
`P x_L J2P` are equality cases.

For products in symplectically orthogonal planes, Phi is one quarter of the
sum of the factor areas, so the candidate is the arithmetic–geometric mean
inequality. These extensive product controls are not discoveries.

## Any one or two segments added to a matched planar product, derived lemma

Let `Z=P x_L J2P`, `A=area(P)`, and `x=Phi(Z)=A/2`. For arbitrary
`u=(r,s)` and `w=(t,z)` define

\[
a=h_P(J_2r)+h_P(s)=h_Z(J_4u),\qquad
b=h_P(J_2t)+h_P(z)=h_Z(J_4w),\qquad c=|\omega(u,w)|.
\]

The pairing polynomial is exactly

\[
\Phi(Z+\lambda[-u,u]+\mu[-w,w])=x+a\lambda+b\mu+c\lambda\mu.
\]

Projection-body brightness of a Cartesian product, or the facet volume
formula, gives the volume polynomial

\[
\operatorname{vol}(Z+\lambda[-u,u]+\mu[-w,w])
=A^2+4Aa\lambda+4Ab\mu+D\lambda\mu.
\]

There are no squared segment variables in volume. Consequently with one
added segment the candidate deficit is **exactly** `4 a^2 lambda^2`.

For two segments the determinant sum gives `D <= 16ab` as follows. Write
`P=sum[-g_i,g_i]` and divide the old-old-new-new determinant contribution
by 16. The two old generators can both lie in the first plane, both in the
second, or one in each. The first two contributions are

\[
\frac A4|\det(s,z)|+\frac A4|\det(r,t)|.
\]

The mixed contribution is at most
`h_P(J2r)h_P(z)+h_P(J2t)h_P(s)` by determinant expansion and the triangle
inequality. For any planar directions `v,w`, containment of P in its two
support strips gives

\[
\frac A4|\det(v,w)|\le h_P(v)h_P(w).
\]

Applying this to `(s,z)` and `(J2r,J2t)` bounds the sum of all three
contributions by `ab`. Thus `D <= 16ab`.

It follows for all nonnegative lambda, mu that

\[
\begin{aligned}
4\Phi^2-\operatorname{vol}
={}&4a^2\lambda^2+4b^2\mu^2
+(8ab+8xc-D)\lambda\mu\\
&+8ac\lambda^2\mu+8bc\lambda\mu^2
+4c^2\lambda^2\mu^2\ge0.
\end{aligned}
\]

The quadratic part is bounded below by
`4(a lambda-b mu)^2 + 8xc lambda mu`. This proves the stated lemma for
arbitrary centrally symmetric planar polygons P and arbitrary added directions,
not just the sampled integer paths. Extension to centrally symmetric planar
convex bodies follows by zonogon approximation and continuity.

The proof was checked independently by the bounded mathematical auditor.
No claim of literature novelty is made. It does not prove the generic
zonotope conjecture.

## Exact sufficient certificates for retained two-segment paths

For an arbitrary base, let its polynomials be

\[
V(t,s)=v_0+v_t t+v_s s+v_{ts}ts,\qquad
\Phi(t,s)=x+at+bs+cts.
\]

The exact decomposition

\[
\begin{aligned}
4\Phi^2-V={}&(4x^2-v_0)+(8xa-v_t)t+(8xb-v_s)s\\
&+4(at-bs)^2+(16ab+8xc-v_{ts})ts\\
&+8ac t^2s+8bc ts^2+4c^2t^2s^2
\end{aligned}
\]

is nonnegative throughout the nonnegative quadrant whenever the first three
coefficients and `16ab+8xc-v_ts` are nonnegative. All a,b,c are nonnegative.
This is a sufficient certificate for general bases; a negative coefficient
is **not** a counterexample. On the matched-product equality bases, the
constant and linear terms vanish, and quadratic copositivity makes the last
condition necessary as well as sufficient.

The validator checks the coefficients independently using arbitrary-precision
integer Leibniz determinants. Of 351 retained paths, 326 satisfy this sufficient
criterion; the remaining 25 are uncertified by this criterion, not disproved.
