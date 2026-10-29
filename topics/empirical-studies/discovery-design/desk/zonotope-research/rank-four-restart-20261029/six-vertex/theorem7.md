# Seven-generator reduction by minimal support

Assume the sparse-degree lemma: every rank-at-most-four skew pairing matrix whose support graph has a vertex of degree one or two satisfies

\[
4\Psi(W)\le \Phi(W)^2,
\qquad
\Phi(W)=\sum_{i<j}|W_{ij}|,
\quad
\Psi(W)=\sum_{|I|=4}|\operatorname{Pf}(W_I)|.
\tag{1}
\]

Then the same inequality holds for every real skew 7×7 matrix of rank at most four. The proof reduces any putative counterexample to a degree-at-most-two case. It relies on (1), whose proof is owned separately.

## Choose a sparse global maximizer

The set

\[
\mathcal K=\{W\in\mathbb R^{7\times7}:W^T=-W,\ \operatorname{rank}W\le4,\ \Phi(W)=1\}
\]

is compact: it is closed, and every entry is bounded by one. The continuous function Ψ therefore has a maximum on K. Its value is at least 1/4, by embedding the six-cycle equality matrix and adjoining a zero row and column. Among its maximizers choose W with the fewest nonzero upper-triangular entries.

It is enough to rule out a maximum value greater than 1/4. Suppose, for contradiction, that this sparse maximizer has Ψ(W)>1/4. It has no degree-zero vertex: deleting an isolated vertex leaves Φ and Ψ unchanged and produces a 6×6 matrix of rank at most four, contrary to the established six-generator inequality. Thus every vertex has positive degree.

## Deleting one index

Fix a vertex i, delete its row and column to obtain the old skew matrix W₀, and write y_j=W_{ji} for the deleted column. Let

\[
\Phi_0=\sum_{j<k,\ j,k\ne i}|(W_0)_{jk}|,
\qquad
\Psi_0=\sum_{|I|=4,\ i\notin I}|\operatorname{Pf}(W_I)|,
\qquad
a=\sum_{j\ne i}|y_j|.
\]

Then Φ(W)=Φ₀+a. The Pfaffians involving i are

\[
(Ty)_{jkl}=(W_0)_{jk}y_l-(W_0)_{jl}y_k+(W_0)_{kl}y_j,
\qquad b(y)=\sum_{j<k<l}|(Ty)_{jkl}|,
\]

so Ψ(W)=Ψ₀+b(y). If rank W₀≤2, then Ψ₀=0, and the triangle inequality gives

\[
b(y)\le a\Phi_0.
\]

Indeed, each old edge |(W₀)_{jk}| is multiplied only by |y_l| with l distinct from j,k, whose sum is at most a. Hence

\[
4\Psi(W)\le4a\Phi_0\le(a+\Phi_0)^2=\Phi(W)^2,
\]

contradicting Ψ(W)>1/4. Thus for every vertex i, rank W₀=4.

For rank W₀=4, the block skew matrix

\[
W=\begin{pmatrix}W_0&y\\-y^T&0\end{pmatrix}
\]

has rank at most four exactly when y∈range(W₀). To see this, decompose the old space as range(W₀)⊕ker(W₀); here dim ker(W₀)=2. Any nonzero component of y in ker(W₀) couples the new coordinate to one kernel direction, creates an additional two-dimensional skew block and raises rank from four to six. If y lies in range(W₀), a skew congruence eliminates the last row and column without increasing rank.

## Replace the deleted column by an extreme point

Let S={j≠i:y_j≠0} be the current support, and keep W₀ fixed. Consider

\[
P_S=\{z\in\mathbb R^S:z\in\operatorname{range}(W_0),\ \|z\|_1\le a\}.
\]

For z in this set, adjoining z as the new column preserves rank at most four. The function b(z)=||Tz||₁ is convex and homogeneous. The current y has b(y)>0: otherwise Ψ(W)=Ψ₀ and Φ(W)=Φ₀+a>Φ₀, so the padded old matrix W₀ would have a strictly larger Ψ/Φ² ratio than W, contrary to global maximality.

The maximum of b on P_S occurs at a vertex z*. If ||z*||₁<a, scaling z* up to norm a remains feasible and strictly increases b, since b(z*)≥b(y)>0. Thus ||z*||₁=a. Replacing y by z* leaves Φ unchanged and cannot lower Ψ. Global maximality forces b(z*)=b(y), so the replacement is another global maximizer. Minimality of the support of W forces |supp(z*)|≥|supp(y)|; since z* is supported in S, equality holds. In particular the current degree |S| equals the support size of an extreme point of P_S.

The kernel of W₀ has dimension two. The condition z∈range(W₀)=ker(W₀)^⊥ imposes at most two independent linear constraints on coordinates in S. On a fixed sign face of the ℓ1 ball supported on k coordinates, the face dimension is k−1; imposing c≤2 independent constraints can yield an extreme point only when k−1≤c. Therefore

\[
\deg_W(i)=|S|\le3.
\tag{2}
\]

This holds for every vertex i.

## Parity gives a sparse vertex

If every vertex had degree three, the sum of the seven degrees would be 21, impossible because every graph has even degree sum. Since all degrees are at most three, some vertex has degree at most two. It cannot have degree zero by the preceding paragraph, so its degree is one or two. Applying the sparse-degree lemma (1) to this vertex contradicts Ψ(W)>1/4. Thus the seven-generator inequality follows from (1) and the established six-generator case.
