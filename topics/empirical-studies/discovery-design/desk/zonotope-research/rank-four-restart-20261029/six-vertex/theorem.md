# Six-generator rank-four Pfaffian inequality

This note proves

\[
4\sum_{|I|=4}|\operatorname{Pf}(W_I)|\le
\left(\sum_{i<j}|W_{ij}|\right)^2
\]

for every real skew 6×6 matrix W of rank at most four, assuming the established five-generator case. The proof is exact and analytic. It does not prove the arbitrary-generator conjecture or establish novelty.

## One-segment reduction

Fix the first five indices. Let Φ and Ψ be the old edge-mass and four-Pfaffian sums, and put δ=Φ²−4Ψ≥0. Write y_i=W_{i6}. Let P_i be the signed Pfaffian of the 4×4 principal submatrix of W_0 obtained by omitting i, with the alternating cofactor sign. Pfaffian expansion along index 6 gives

\[
\operatorname{Pf}(W)=P\cdot y.
\]

Thus rank W≤4 iff y∈P^⊥. The ten new four-Pfaffians are the coordinates of the linear map

\[
(Ty)_{ijk}=W_{ij}y_k-W_{ik}y_j+W_{jk}y_i,\qquad 1\le i<j<k\le5.
\]

Set a=||y||_1 and b=||Ty||_1. Scaling the sixth generator by t≥0 gives the exact gap

\[
D(t)=\Phi^2-4\Psi +(2\Phi a-4b)t+a^2t^2
=\delta +(2\Phi a-4b)t+a^2t^2.
\]

It suffices to prove b/a≤(Φ+√δ)/2 for every nonzero y∈P^⊥. The function y↦||Ty||_1 is convex. Its maximum on the polytope P^⊥∩{||y||_1≤1} occurs at a vertex. Every vertex has support at most two: a face of the ℓ1 ball supported on k coordinates has dimension k−1, and intersection with one hyperplane can leave a vertex only when k≤2. If P=0 the polytope is the full ℓ1 ball, whose vertices have support one. So only directions with one or two nonzero coordinates remain.

## One-coordinate directions

Suppose y=t e_i. Feasibility requires P_i=0. Let s be the total old edge mass among the other four indices, and r the mass of the star at i, so Φ=s+r. The new Pfaffian sum is t s. Since P_i=0, the old four-Pfaffians are all those containing i. Triangle inequality bounds their sum Ψ by the mass of disjoint star-edge/internal-edge pairs, at most r s. If s≤Φ/2, then s≤(Φ+√δ)/2. If s>Φ/2, then

\[
\delta=\Phi^2-4\Psi\ge \Phi^2-4s(\Phi-s)=(2s-\Phi)^2,
\]

so again s≤(Φ+√δ)/2. This proves the required direction bound.

## A planar wedge lemma

For r,s∈R^m and α,β≥0 with α+β=1, define

\[
R=||r||_1+||s||_1,\qquad h=\alpha s-\beta r,\qquad H=||h||_1.
\]

Then

\[
||r\wedge s||_1\le H(R-H).
\tag{1}
\]

Indeed, form the planar zonotope Z=Σ_i[-(r_i,s_i),(r_i,s_i)]. Its area is 4||r∧s||_1. It lies in the rectangle |x|≤A, |y|≤B, where A=||r||_1 and B=||s||_1, and in the strip |−βx+αy|≤H. The triangle and reverse-triangle inequalities give |βA−αB|≤H≤βA+αB. If αβ>0, the rectangle cut by the strip has area

\[
4AB-\frac{(\beta A+\alpha B-H)^2}{\alpha\beta}.
\]

The difference between 4H(A+B−H) and this area is

\[
\frac{(\beta A-\alpha B+(\alpha-\beta)H)^2}{\alpha\beta}\ge0,
\]

proving (1). If αβ=0, (1) reduces to ||r∧s||_1≤||r||_1||s||_1, which follows by expanding the determinants. This proof also applies to any coordinate restriction of r,s.

## Two-coordinate directions

After reorienting the first two generators, write y=t(α,β,0,0,0), with t>0, α,β>0 and α+β=1. Let B be the skew block on indices 3,4,5 and identify it with b=*(B)=(W_{45},−W_{35},W_{34}); put B_m=||b||_1. Define

\[
r=(W_{13},W_{14},W_{15}),\quad s=(W_{23},W_{24},W_{25}),
\quad g=r+s,\quad h=\alpha s-\beta r,\quad H=||h||_1,\quad R=||r||_1+||s||_1,
\quad c=W_{12}.
\]

Pfaffian expansion gives b·h=0. The ten new Pfaffians sum to t(B_m+H). The five old Pfaffians sum exactly to

\[
C=|b\cdot g|+||c b-g\times h||_1.
\tag{2}
\]

To check (2), the Pfaffians on 2345 and 1345 are b·s and b·r. Since b·h=0, these are β(b·g) and α(b·g), so their absolute values sum to |b·g|. The other three Pfaffians are the coordinates of c b−*(r∧s), up to a coordinate sign. Since r=αg−h and s=βg+h, r∧s=g∧h, giving (2).

We next bound its two terms. First,

\[
||g\times h||_1=||r\wedge s||_1\le H(R-H)
\]

by (1) for m=3. For the scalar term, maximize |b·g| subject to b·h=0 and ||b||_1≤1. The extreme points of this section of the ℓ1 ball have support one or two. A two-coordinate extreme point on {i,j} gives

\[
\frac{|b\cdot g|}{||b||_1}
=\frac{|g_i h_j-g_j h_i|}{|h_i|+|h_j|}
=\frac{|r_i s_j-r_j s_i|}{|h_i|+|h_j|}.
\]

Applying (1) to the two coordinates gives a bound R'−H'≤R−H, where R'=|r_i|+|s_i|+|r_j|+|s_j| and H'=|h_i|+|h_j|. A singleton extreme point requires h_i=0, and then |g_i|≤|r_i|+|s_i|≤R−H. Therefore

\[
|b\cdot g|\le B_m(R-H).
\]

Using (2) and the triangle inequality,

\[
C\le B_m(R-H)+|c|B_m+H(R-H)
\le (B_m+H)(|c|+R-H)=UV,
\]

where U=B_m+H and V=|c|+R−H. Thus the full sums obey

\[
\Phi_{new}=t+U+V,\qquad \Psi_{new}\le tU+UV=U(t+V).
\]

Consequently

\[
\Phi_{new}^2-4\Psi_{new}\ge(t+U+V)^2-4U(t+V)=(U-(t+V))^2\ge0.
\]

This proves the required bound on every two-coordinate extreme direction, including all t≥0.

## Finish the reduction

For every vertex y/a of P^⊥∩{||y||_1≤1}, the one- or two-coordinate arguments show that the one-segment gap polynomial is nonnegative for all t≥0. Equivalently, each vertex satisfies b/a≤(Φ+√δ)/2. Convexity of ||Ty||_1 extends this bound to every feasible y. Substitution in D(t) gives

\[
D(t)\ge \delta-2\sqrt\delta\,(ta)+(ta)^2=(ta-\sqrt\delta)^2\ge0.
\]

The six-generator inequality follows.

The proof is a finite-generator extension of the retained five-generator result. It does not establish the inequality for arbitrary generator count.
