# Haim–Kislev capacity-formula contract

Status (18 September 2026): **verified for the scalar capacity formula used in
the affine-pentagon argument**. No sign, normalization, facet-word, or
hypothesis gap was found. This check does not establish novelty or independently
re-audit the subsequent matrix-containment proof.

## Sources and scope

Primary source: Pazit Haim–Kislev, *On the symplectic size of convex polytopes*,
Theorem 1.1, equation (1), [arXiv:1712.03494v3](https://arxiv.org/html/1712.03494v3#Thmtheorem1).
Checked against the repository's primary-source LaTeX
[`papers/hk2017/EHZ-polytopes.tex`](../../papers/hk2017/EHZ-polytopes.tex),
label `formula_theorem`, and the online v3 statement. The repository calls this
source HK2017; the cited published article is from 2019. Those names refer to
the same imported formula.

Local convention reference:
[`formal/hk2017-qp-conventions.tex`](../hk2017-qp-conventions.tex).
Target checked:
[`research.tex`](../../docs/pro-handoffs/pentagon-return/received/math/research.tex),
Section 2, equation `eq:HK`, and its four-block normalization.
The independent downstream review is
[`audit-affine.md`](../../docs/pro-handoffs/pentagon-return/audit-affine.md).

## Hypotheses and weights

The source theorem concerns a convex polytope with nonempty interior, its
actual facets, outward unit normals \(n_i\), and support heights
\(h_i=h_K(n_i)\). It does not require smoothness, simplicity, general position,
or central symmetry. For \(G,H\in GL(2,\mathbb R)\), both pentagon factors and
their product are bounded full-dimensional convex polytopes, so these
hypotheses hold. Singular transformations are outside the claimed result.

Translate the factors so that the origin lies inside each. The product
translation is symplectic, hence preserves capacity; it also preserves volume.
All heights are then positive. Write the normalized facet covectors as
\(a_i=n_i/h_i\), so the inequalities are \(a_i\cdot z\le1\).
If \(\lambda_i\) denotes the source theorem's weight, set
\(b_i=\lambda_i h_i\). This is a bijection between the source constraints and

\[
 b_i\ge0,\qquad \sum_i b_i=1,\qquad \sum_i b_i a_i=0.
\]

Indeed, \(b_i a_i=\lambda_i n_i\), and bilinearity gives
\(b_i b_j\omega(a_i,a_j)=\lambda_i\lambda_j\omega(n_i,n_j)\).
Thus replacing unit normals by support-normalized covectors introduces no
extra factor. In the returned proof these \(b_i\) are called \(\beta_i\).

## Sign, ordering, and the reciprocal

The returned proof fixes
\(\omega_0((q,p),(q',p'))=q\cdot p'-p\cdot q'\).
In the project's standard-form convention the source's displayed objective
puts the **later** word entry first:

\[
 Q^{\rm HK}_\rho(b)=\sum_{j<k}b_{\rho(j)}b_{\rho(k)}
       \omega_0(a_{\rho(k)},a_{\rho(j)}).
\]

The returned proof puts the **earlier** entry first:

\[
 Q_\sigma(b)=\sum_{j<k}b_{\sigma(j)}b_{\sigma(k)}
       \omega_0(a_{\sigma(j)},a_{\sigma(k)}).
\]

Reverse the ordered facet list, carrying each weight with its facet. Then
\(Q^{\rm HK}_{\operatorname{rev}\sigma}(b)=Q_\sigma(b)\).
Reversal is a bijection of the candidate family and leaves the constraints
unchanged. Consequently the maxima coincide, and the source's prefactor
\(\tfrac12[\max Q^{\rm HK}]^{-1}\) is exactly

\[
 c_{\rm EHZ}(K)=\frac{1}{2\max Q_\sigma}.
\]

Antisymmetry also means \(Q_{\operatorname{rev}\sigma}=-Q_\sigma\) when using
the *same* convention on both sides. Do not reverse a named witness without
that sign change. Nor should a named word in the source be identified directly
with physical traversal in the project: only the scalar maximum is imported
here. The source uses a complex-structure/action notation whose sign must be
translated separately if an orbit reconstruction is needed; no such
reconstruction is needed for this capacity theorem.

## Word family and cyclic grouping

The source ranges over full facet permutations and permits zero weights.
Delete all zero-weight entries to obtain a word of distinct active facets.
Conversely, append missing facets with zero weights to any distinct-facet
subset word. Constraints and objective are unchanged in both directions.
Thus the packet's subset-word maximization is exactly equivalent, not a
relaxation. Repeated facets are not introduced by this argument.

For weighted covectors \(z_1,\ldots,z_m\) with \(\sum z_i=0\), moving the
first to the end changes the earlier-first objective by
\(-2\omega_0(z_1,\sum_{k>1}z_k)=0\). This justifies cyclic representatives.
Within a product factor the symplectic pairing vanishes, so consecutive
same-factor steps can be summed for objective evaluation. These are local
algebraic consequences, not additional conclusions attributed to HK.

## Product mass and four-block factor

The product has covectors \((u_i,0)\) and \((0,v_j)\). If each factor receives
mass \(1/2\), a normalized factor step \(x=\gamma_i u_i\) contributes actual
product covector \((x/2,0)\), and similarly for \(y\). The sparse-reduction
argument establishing existence of a maximizer with this mass split is local
to the proof; it is not a hypothesis or special case of HK.

For grouped normalized steps \(x,y,-x,-y\), the unscaled earlier-first sum
is \(2x\cdot y\). Each actual pairing has factor \(1/4\), giving
\(Q=x\cdot y/2\). Therefore \(c=1/(2Q_{\max})\) becomes the reciprocal of
the maximal width-body pairing, not twice or half that reciprocal.

For regular pentagons, the width-body generators are
\(\pm n_i/(1+\cos(\pi/5))\). After independent maps their pairing is
\(n_i^T G^{-1}H^{-T}n_j/(1+\cos(\pi/5))^2\). Once the separately audited
matrix containment proves four-block sufficiency, this yields exactly
\[
 c_{\rm EHZ}(GP_5\times HP_5)
 =\frac{(1+\cos(\pi/5))^2}
 {\max_{i,j}|n_i^TG^{-1}H^{-T}n_j|}.
\]

No HKO endpoint capacity, symmetric-product formula, or claim of novelty is
needed to close this imported-formula contract.
