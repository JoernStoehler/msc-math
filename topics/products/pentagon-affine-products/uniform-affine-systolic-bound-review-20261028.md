# Independent review of the uniform odd affine-product bound

28 October 2026 UTC. Reviewer: native agent `uniform_bound_review`.

**Verdict: complete relative to the stated established inputs.** I found no
mathematical error or missing hypothesis in inequalities (1)–(2), their strict
gap below one outside the pentagon pair, or the HKO equality classification.
No repair is required to the proof. An optional terminology clarification and
an optional shorter construction are given below. This is independent agent
review, not human acceptance or a novelty assessment.

The reviewed candidate is
[`uniform-affine-systolic-bound-20261028.md`](uniform-affine-systolic-bound-20261028.md),
SHA-256 `f16c5ff9e2c71bfea33b77ec0419fdd583f60edfdbc387b30c7f015c0b8a6d9e`.
I read that actual file, the relevant definitions, Lemma `lem:width`, Theorem
`thm:decomp` and their proofs in [`research.tex`](research.tex), and its affine
pentagon theorem and systolic equality argument. I also read
[`source-contract.md`](source-contract.md), the local
[HK convention account](../../capacity-and-qp/proofs/hk2017-qp-conventions.tex),
the actual `formula_theorem` statement in the
[retained HK source](../../literature/sources/hk2017/EHZ-polytopes.tex), and the
monotonicity argument in the
[odd symmetric-partner note](../pentagon-arbitrary-partner/odd-regular-symmetric-partner.md).
This review does not rely on the candidate author's summary or the existence
of prior audits as evidence that the new implication is correct.

## Four-block feasibility and normalization

The polar identity is correct. For odd regular $j$, the opposite direction to
a facet normal is a vertex direction, so the corresponding difference-body
support height is $h_j+1=w_j$. Its edge normals are the union of the normals
of the polygon and its negative. Consequently its polar is $D_j/w_j$.

There is also a direct explicit construction that checks the precise part of
the width lemma used here. At a normal index $i$, use the three normals with
indices

$$
i,\qquad i+(j-1)/2,\qquad i+(j+1)/2 \pmod j.
$$

The last two sum to $-2h_j n_i$. The support-normalized dual vertices are
$u_k=n_k/h_j$, and the positive closure weights are

$$
\frac{h_j}{w_j},\qquad \frac1{2w_j},\qquad \frac1{2w_j}.
$$

They sum to one and close. The singled-out step is $n_i/w_j$; the other two
sum to its negative. The three facets are distinct for every odd $j\ge5$.
After applying $G$, the steps transform by $G^{-T}$ with the weights
unchanged, including when $G$ reverses orientation.

Give each factor half the product mass and order the four groups as
$x,y,-x,-y$. The earlier-first symplectic sum before the half-mass factors is
$2x\cdot y$. The product factors contribute $1/4$, giving $Q=x\cdot y/2$.
Changing the alternating order supplies its negative. Thus maximizing over
normal indices and signs gives exactly the candidate's feasible value
$Q_4=\mu(M)/(2w_nw_m)$. Nonsingularity and spanning normal sets ensure
$\mu(M)>0$.

The retained HK statement uses a reciprocal prefactor $1/2$. Changing its
height weights to support-normalized weights preserves the objective, and
reversing the whole word family reconciles its later-first convention with
the candidate's earlier-first convention. Extending the six-facet word with
zero weights is allowed. Hence $Q_{\max}\ge Q_4$ indeed implies
$c\le w_nw_m/\mu(M)$, with no sign or factor-of-two error. No global
four-block sufficiency, six-block exclusion, or tensor containment is needed.

## Area and determinant calculation

For every vertex $y$ of $D_m$, all defining inequalities of $D_n^\circ$ hold
at $My/\mu(M)$. Convexity gives the stated inclusion. Applying area
monotonicity and repeating for the transpose gives both ratios in (4).
Absolute determinants are necessary and are correctly used throughout.

The three area formulas are correct:

$$
S_j=j\sin(\pi/j)\cos(\pi/j),\quad
|D_j|=j\sin(\pi/j),\quad
|D_j^\circ|=2j\tan(\pi/(2j)).
$$

The product volume is $S_nS_m/|\det M|$. Substituting the first area ratio
into $w_n^2w_m^2|\det M|/(2S_nS_m\mu(M)^2)$ gives

$$
\frac{w_nw_m^2}{m^2h_nh_m\sin^2(\pi/m)}
=\frac12r_n\kappa_m,
$$

as stated. The transpose yields the other term. No assumption of equal side
counts, equal areas, positive determinants, or orthogonal maps enters this
calculation.

## Monotonicity, strict bound, and equality

The expression for $\kappa(x)$ and its logarithmic derivative are correct.
For $0<x\le\pi/5$, $\sin x>x\cos x$ implies

$$
\frac1{\sin x}-\frac1x
<\frac{1-\cos x}{\sin x}=\tan(x/2).
$$

Since $0<\tan(x/2)<1$, the final strict derivative comparison is valid.
Thus both sequences strictly decrease with side count. The pentagon values
$r_5=\sqrt5$ and $\kappa_5=(10+6\sqrt5)/25$ give exactly the HKO constant.
If either index is at least seven, selecting the term with that index on
$\kappa$ proves (2); it is not necessary to compare which of the two terms
is actually smaller.

I independently checked the rational arithmetic using Python's exact
`fractions.Fraction`, without a capacity producer. For the stated $l,u$, the
three required positive differences are

$$
1-u^2/2+u^4/24-u^6/720-9/10
=\frac{1109229001}{1245715848090}>0,
$$
$$
901/1000-(1-l^2/2+l^4/24)
=\frac{12983725264079}{921984000000000000}>0,
$$
$$
l-l^3/6-43/100
=\frac{61259732813}{16464000000000}>0.
$$

Monotonicity of sine and cosine on this interval justifies using the lower
or upper rational endpoint, and alternating Taylor bounds have the stated
directions. The resulting ceiling reduces exactly to
$3613801/3624040<1$. The illustrative decimal independently evaluates to
$0.9722599541188647$. The usual stated rational bounds for $\pi$ suffice;
their classical derivations were not re-proved in this review.

The strict side-count reduction forces $n=m=5$ for equality at HKO. The
candidate then invokes exactly the established affine-pentagon hypotheses.
The equality statement in `research.tex` is precisely
$M=tR_{\pi/10}U$, $t>0$, with $U$ a decagon symmetry. Its area proof also
checks the orientation-reversing cases: equal areas in the inclusion force
equal convex bodies, and a linear automorphism of a regular decagon permutes
vertices and preserves or reverses adjacency, hence is dihedral. Conversely
the displayed matrices attain the bound. Translations cause no issue.

## Optional clarification and review limits

The introductory phrase “a feasible trajectory suffices” should preferably
say “a feasible HK word suffices.” The proof uses a feasible algebraic word;
it does not establish boundary placement of an associated characteristic.
This is a terminology clarification, not a gap in the displayed argument.
The explicit three-facet construction above could replace the general
width/decomposition citation if a more self-contained proof is desired.

I accepted the HK theorem and the established exact affine-pentagon capacity
theorem as inputs, after checking their relevant statements, normalization,
applicability, and the latter's equality argument. I did not re-prove the HK
theorem or independently re-audit the entire pentagon tensor-classification
proof. I did not check new external literature, novelty, computational
capacity producers, or manuscript integration. None is required for the new
inequality relative to these inputs. The result supplies an upper bound,
not an exact general affine capacity formula or a sharp mixed-pair constant.
Any subsequent extension to even side counts is outside this reviewed text.
