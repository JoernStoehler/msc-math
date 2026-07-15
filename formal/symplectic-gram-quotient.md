# Symplectic-Gram quotient distance

Status: **agent-written, not reviewed by Jörn** (2026-07-15). This is a
developer-facing exact proof note. The accompanying Python packet is only a
bounded exact prototype and is not evidence for the theorem.

## Convention and domain

Let $V=\mathbb R^4$, in coordinate order $(q_1,q_2,p_1,p_2)$, with

\[
J=\begin{pmatrix}0&I_2\\-I_2&0\end{pmatrix},\qquad
\omega(u,v)=u^T Jv.
\]

Represent the covectors $a_1,\ldots,a_F$ by coordinate columns and put them
in the $4\times F$ matrix $A=[a_1\ \cdots\ a_F]$. Thus

\[
\Omega(A)=A^TJA,\qquad \Omega(A)_{ij}=\omega(a_i,a_j).
\]

The configuration domain in this note consists of matrices $A$ of rank four.
The polytope domain consists of full-dimensional bounded polytopes with exactly
$F$ irredundant facets. Facet count is fixed throughout a metric stratum.

## Labeled completeness

**Theorem (complete labeled invariant).** Let $A,B\in\mathbb R^{4\times F}$
both have rank four. Then

\[
A^TJA=B^TJB
\]

if and only if there is a unique $T\in\operatorname{Sp}(4,\mathbb R)$ such
that $B=TA$.

**Proof.** The forward direction uses the kernel premise from the proposed
construction. Since $A:\mathbb R^F\to V$ is onto, $A^T:V\to\mathbb R^F$
is injective. Consequently

\[
\ker(A^TJA)=\ker A.
\]

Indeed, one inclusion is immediate. If $A^TJAx=0$, injectivity of $A^T$
gives $JAx=0$, and invertibility of $J$ gives $Ax=0$. The same holds for
$B$, so equality of Gram matrices gives $\ker A=\ker B$.

Define $T:V\to V$ by $T(Ax)=Bx$. Equality of kernels makes this
well-defined. It is onto because $B$ is onto, hence invertible. For all
$x,y\in\mathbb R^F$,

\[
\omega(TAx,TAy)=x^TB^TJBy=x^TA^TJAy=\omega(Ax,Ay).
\]

Surjectivity of $A$ therefore gives $T^TJT=J$. If also $B=T'A$, then
$(T-T')A=0$, and surjectivity of $A$ gives $T=T'$.

Conversely, $B=TA$ with $T^TJT=J$ immediately gives
$B^TJB=A^TJA$. ∎

Rank four is essential. Without it, the kernel argument fails and the
restricted alternating form need not determine how the lower-dimensional span
sits inside $V$.

## Facet permutations

For a permutation matrix $P$, relabeling columns gives $A'=AP^T$ and

\[
\Omega(A')=P\Omega(A)P^T.
\]

It follows from labeled completeness that

\[
\Omega(A)=P\Omega(B)P^T
\]

holds exactly when, after the corresponding relabeling, the two configurations
are related by one linear symplectic map. No genericity or absence of symmetry
is needed. Symmetric configurations merely give more than one minimizing
permutation. Here the theorem's $T$ acts on represented covectors. The
corresponding primal map is $R=T^{-T}$, which is again symplectic.

## From an inequality presentation to a polytope object

Write an irredundant presentation as

\[
K=\{x:\langle n_i,x\rangle\le h_i,\ 1\le i\le F\}.
\]

Choose the analytic center

\[
c(K)=\operatorname*{argmin}_{x\in\operatorname{int}K}
  -\sum_{i=1}^F\log(h_i-\langle n_i,x\rangle).
\]

The spanning facet normals make the barrier strictly convex, hence give
uniqueness. Boundedness and divergence of the barrier at the boundary give
existence. Multiplying one inequality by a positive scalar adds a constant to
the barrier, so the center does not depend on the positive scaling used to
write a facet. It is equivariant under invertible affine maps.

Let $V_K=\operatorname{vol}_4(K)>0$. In volume-one centered coordinates

\[
y=V_K^{-1/4}(x-c(K)),
\]

the uniquely normalized facet covectors are

\[
a_i(K)=V_K^{1/4}\frac{n_i}{h_i-\langle n_i,c(K)\rangle},
\qquad
\widehat K=\{y:\langle a_i(K),y\rangle\le1\}.
\]

This construction depends only on the polytope and an ordering of its facets:
positive rescaling of an inequality cancels between numerator and denominator.
Irredundancy is necessary. Adding a redundant inequality adds a Gram row and
column and changes $F$, so a raw presentation with redundant rows is not in
the domain.

The following actions behave as required.

- Translation: $c(K+t)=c(K)+t$, while the centered normalized covectors are
  unchanged.
- Positive homothety: for $\lambda>0$, volume changes by $\lambda^4$, the
  right-hand-side-normalized covectors change by $\lambda^{-1}$, and the two
  factors cancel.
- Linear symplectic map: if $R\in\operatorname{Sp}(4,\mathbb R)$, then
  $|\det R|=1$, $c(RK)=Rc(K)$, and the normalized represented covectors are
  $R^{-T}a_i(K)$. The matrix $R^{-T}$ is symplectic, so $\Omega$ is
  unchanged.
- Facet relabeling: $\Omega$ changes by simultaneous row-column
  permutation.

Thus the Gram orbit is invariant under exactly the declared nuisance actions.
By labeled completeness, equality of Gram orbits also separates normalized
polytopes up to a linear symplectic map. Undoing the normalization gives
separation up to translation, positive scale, a linear symplectic map, and
facet relabeling.

## Metric statement

For fixed $F$, define

\[
d_\Omega(A,B)=\frac1F\min_{P\in S_F}
  \|\Omega(A)-P\Omega(B)P^T\|_F.
\]

**Theorem (metric on the fixed-$F$ quotient).** On the quotient of the
rank-four configuration domain by
$\operatorname{Sp}(4,\mathbb R)\times S_F$, $d_\Omega$ is a metric. Hence,
after the analytic-center and volume-one normalization above, it is a metric on
the stated fixed-$F$ polytope quotient.

**Proof.** Frobenius distance is a metric on matrices, and conjugation by a
permutation matrix is an isometry. Changing either configuration representative
by a symplectic map does not change its Gram matrix; changing either facet
ordering only reindexes the same finite set of comparison permutations. Thus
the formula is well-defined on the quotient. The minimum over the finite group
exists. Nonnegativity and symmetry are immediate. Zero distance means equality
after one permutation, which separates quotient points by the completeness
theorem.

For the triangle inequality, choose minimizing permutations for the first and
second legs. Apply the inverse first permutation to the second comparison and
compose the two permutations. The ordinary Frobenius triangle inequality gives
an admissible comparison for the outer leg. Division by the positive constant
$F$ preserves the inequality. ∎

The formula is not a cross-stratum metric: it is undefined when facet counts
differ. Assigning $+\infty$ would give an extended metric, not the finite
metric needed by ordinary nearest-neighbor and MDS code.

## Topological status

One limited topology claim is justified at the configuration level. Let

\[
\mathcal C_F=\{A\in\mathbb R^{4\times F}:\operatorname{rank}A=4\}
\]

and let $\mathcal R_F$ be the rank-four skew-symmetric $F\times F$
matrices. The Gram map induces a bijection

\[
\mathcal C_F/\operatorname{Sp}(4,\mathbb R)\longrightarrow\mathcal R_F.
\]

Every rank-four alternating form on $\mathbb R^F$ is the pullback of the
standard form on the four-dimensional quotient by its kernel, so every element
of $\mathcal R_F$ occurs. The induced bijection is a homeomorphism. To see the
nontrivial direction locally, use the fact that a rank-four skew matrix has a
nondegenerate principal $4\times4$ minor and choose its four indices. On a
smaller open patch, symplectic Gram--Schmidt with fixed nonzero pivots gives a
continuous choice of the four corresponding columns. Every remaining column
is then recovered continuously and uniquely from its four pairings with that
basis; the rank-four condition makes its remaining pairings agree with the
given matrix. Composing this local section with the orbit map gives a local
continuous inverse. These patches cover $\mathcal R_F$. Taking the further
quotient by the finite permutation group, the minimum-Frobenius metric induces
the finite-group quotient topology on this Gram image.

This note does **not** prove that the resulting metric topology agrees with a
chosen Hausdorff, support-function, orbifold, or stratified topology on the
union of all polytope types. In particular, facet birth/death, redundant-row
limits, and movement between different $F$ are outside the theorem. Do not
use this construction to claim topology, components, holes, or support coverage
for such a union without a separate statement and proof.

## Computational consequence

Exact evaluation reduces to a finite graph-matching problem on the complete
directed signed edge-weighted graph with weights $\Omega_{ij}$. Exhaustive
search is exact but factorial. The companion packet therefore certifies only
$F\le8$, reports all $F!$ evaluations, returns exact squared distance plus a
symbolic square root, and returns no distance on timeout. Numerical controls
can test implementation agreement but cannot strengthen any theorem in this
note.
