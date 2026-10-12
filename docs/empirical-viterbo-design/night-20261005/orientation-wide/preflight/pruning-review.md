# Independent pruning, symmetry, and exact-field preflight

Reviewed 2026-10-05 UTC for `direct_cover_lead`. Scope is the selected bounded
preflight, not a full orientation-wide theorem or a completed denominator
arrangement. Source contract: `../../comparison/orientation-preflight-contract.md`.

## Primary-source pruning and its domain

Independently opened [Balitskiy–Mitrofanov–Polyanskii,
arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1), dated 12 March 2026.
Theorem 1.2 states the planar EHZ product inequality
\(\operatorname{area}A\operatorname{area}B\ge c(A\times_L B)^2/2\) when
A is any convex shape and B is any quadrilateral. Their convex shapes are
compact convex planar sets with nonempty interior. Section 2.2 also expressly
states factor-exchange symmetry of this capacity. No central symmetry,
regularity, or matching normal fan is required for the quadrilateral factor.

Here is a direct explanation of the exchange step. The map
\(S(q,p)=(p,q)\) takes \(A\times_L B\) to \(B\times_L A\) and pulls the
standard symplectic form back to its negative. On a closed characteristic,
the action integral changes sign under S; reversing its time parametrization
restores positive action. This gives a bijection of positive-action closed
characteristics with equal action, including generalized characteristics on
convex nonsmooth boundaries, so their least actions agree. This is an EHZ
argument; one must not invoke arbitrary-capacity antisymplectic invariance
without explaining it.

Thus if the varying partner K is a genuine quadrilateral and fixed Q is the
regular pentagon, apply Theorem 1.2 to \(Q\times_L K\), exchange factors,
and obtain \(\operatorname{sys}(K\times_L Q)\le1\).

The same bound holds when K is a genuine triangle, without assuming capacity
continuity. For triangle vertices \(v_0,v_1,v_2\), take

\[
K_\varepsilon=\operatorname{conv}\{v_0+\varepsilon(v_2-v_1),
v_0-\varepsilon(v_2-v_1),v_1,v_2\}.
\]

For sufficiently small \(\varepsilon>0\) this is a genuine outer
quadrilateral, contains K, and its area tends to the triangle's area. Product
inclusion and capacity monotonicity give

\[
c(K\times_L Q)^2\le c(K_\varepsilon\times_L Q)^2
\le2\operatorname{area}K_\varepsilon\operatorname{area}Q.
\]

Letting only the areas tend to their limits proves the claim. No limiting
capacity equality is used.

A canonical body in the closed prescribed five-normal type cone with a zero
edge has three or four genuine sides, provided it has interior. There are no
two-sided bodies because the regular five normals, and all their invertible
images, contain no opposite pair. Capacity at least one supplies a contained
genuine template triangle and excludes the collapsed point class. Consequently
every physical zero-edge candidate over the whole domain \(x>0\) has ratio
at most one. It can neither attain nor violate
\(H=(3+\sqrt5)/5>1\). This includes zero-edge parameter intervals, isolated
parameter values, and coalescing edge labels. The singular transform at
\(x=0\) is outside the domain; no genuine-body assertion at that formal
endpoint is implied.

## What exactly survives: ten fitting triples

Use canonical supports and quotient out translations. The accepted vertex
theorem is stronger than existence of one vertex minimum: **every** area
minimizer at fixed \((x,\theta)\) is a vertex. It follows from canonical
Minkowski linearity, Brunn–Minkowski equality, and positive area. Raw redundant
support offsets must first be lowered to actual supports; artificial raw
coordinate fibers cannot create additional body minima.

If any body violates the HKO threshold, normalize its capacity to one and
minimize area in that fixed fan. The attained vertex minimum has at least as
large a ratio, hence also violates. The preceding theorem excludes every
vertex with an active zero-edge inequality. Therefore the violating vertex
has all five edges positive. All of its active rows are fitting inequalities;
their rank is three in the translation quotient. Some three of the five
fitting rows form an independent basis, including when four or five rows
are active.

In fact all ten fitting triples are nonsingular in the gauge \(h_3=h_4=0\).
The tiny retained `verify_pruning.py` computes each exact determinant with
standard-library rational-pair arithmetic. The two possible determinant
values are \((5-\sqrt5)/50\) and \((5-3\sqrt5)/50\), both nonzero. Thus
exactly ten systems suffice before feasibility and duplicate filtering.
This assertion concerns bases, not ten distinct feasible optimizers.

Once a global HKO upper bound is proved, any body attaining HKO is an area
minimizer in its normalized fixed fan and therefore is a vertex. The same
ten triples then capture all equality bodies, modulo translation and
dilation. Before that upper bound is proved, a nonminimizing body can have
ratio exactly HKO while a different body in its fan has a larger ratio;
one must not already classify every threshold-level body as a vertex.
Isolated ties and vertices where more than three fits are active must be
retained and deduplicated exactly. Candidates acquiring a zero edge at an
endpoint are covered by the source-backed pruning, rather than silently
deleted from a closed feasibility interval.

## Full-family orientation symmetries

Write physical unscaled normals as
\(m_i(x,\theta)=G_xR_\theta n_i\), with indices modulo five. Since

\[
R_{\pi/5}n_i=-n_{i+3},
\]

the fan at \(\theta+\pi/5\) is the negative of the fan at \(\theta\), with
that relabeling. Explicitly, if \(h'_j=h_{j-3}\), then
\(K_{x,\theta+\pi/5}(h)=-K_{x,\theta}(h')\). Area is unchanged. The complete
ten templates satisfy \(T_{j+5}=-T_j\); consequently fitting scales for -K
are those for K with opposite template labels, and their minimum is unchanged.
This proves the \(\pi/5\) orientation period on the actual full family,
not merely on a numerical denominator sample.

Let \(F=\operatorname{diag}(1,-1)\) be reflection across the x axis.
It commutes with \(G_x\), preserves regular Q, and satisfies
\(FR_\theta n_i=R_{-\theta}n_{-i}\). Simultaneously reflecting q and p is
symplectic because F is orthogonal, so ratio is invariant. Reflection also
permutes the complete normal-triangle catalogue. In the standard catalogue
\(T_j=R_{j\pi/5}T_0\), with the upward isosceles base template symmetric
across the y axis, this is \(FT_j=T_{5-j}\). Equivalently the permutation
follows directly from the Q-invariant definition of normal triangles.

Period and reflection reduce every angle to
\(0\le\theta\le\pi/10\). With
\(\tau^2=5-2\sqrt5\) and \(u=\tan\theta/\tau\), the complete fundamental
domain is therefore

\[
x>0,\qquad0\le u\le1/\sqrt5.
\]

No interchange of the two anisotropy axes, or identification of x with its
reciprocal, is needed for this domain claim.

## Covectors, determinant, and capacity

For computational points \(q_c=T^{-1}q\), where
\(T=\operatorname{diag}(1,\tau)\), covectors become \(n_c=T n\). The scaled
physical rotation has computational covector conjugate

\[
C_u=T(R_\theta/\cos\theta)T^{-1}
=\begin{pmatrix}1&-u\\\tau^2u&1\end{pmatrix},\qquad
d(u)=1+\tau^2u^2>0.
\]

On the reduced angle interval \(\cos\theta>0\), so the common normal scale
is absorbed by dividing every support offset by that cosine. Both offsets
and template support evaluations must use the same convention.
The physical normal transform is \(M'=G_xR_\theta/\cos\theta\), of
determinant \(xd(u)\). For \(Y=M'^TK\) the canonical regular-fan area
numerator P gives

\[
\operatorname{area}K=\frac{P}{xd(u)\sin(2\pi/5)},\qquad
\operatorname{sys}=\frac{xd(u)}{5P}\quad(c=1).
\]

The computational point/area Jacobian cancels exactly as in the two accepted
slices. The proposed HKO test is thus
\(P-xd(u)/(3+\sqrt5)\ge0\).

Invertibility preserves the closure cone and its five normalized circuit
weights. The complete formula remains

\[
c=\min_l\frac{\lambda_l\cdot h}{D_l(x,u)},\qquad
D_l(x,u)=\max_j\sum_i\lambda_{li}h_{T_j}(G_xC_u n_{c,i}).
\]

Every denominator is positive: translate each genuine template to contain
zero in its interior and use closure invariance of the weighted support sum.
This formula requires all ten templates and all five circuits. The pruning
does not authorize replacing it by one fitted template or one circuit.

## Exact fields and degree bounds

Let \(\mathbb F=\mathbb Q(\sqrt5)\). Each weighted support choice is bilinear
in x and u, so on a denominator cell it has the form
\(\alpha(u)x+\beta(u)\), with \(\alpha,\beta\in\mathbb F[u]\) of degree
at most one. Pair slope/intercept coincidences have degree at most one in u;
triple x-line concurrency has degree at most two, from the determinant of
two slope/intercept differences. Accordingly an individual genuine quadratic
kinetic-event root has degree at most two over \(\mathbb F\), hence at most
four over \(\mathbb Q\). Identically zero determinants are persistent
coincidences, not isolated events.

For any of the ten fixed fitting bases, its inverse matrix is constant in
\(\mathbb F\). Candidate supports are bilinear in x,u, and the HKO gap is

\[
q(x,u)=A(u)x^2+B(u)x+C(u),\qquad \deg A,\deg B,\deg C\le2.
\]

Each feasibility residual is bilinear. A finite x endpoint, where its slope
is nonzero, has the linear-fractional form \(-b(u)/a(u)\). Clearing the
positive square \(a(u)^2\) from q at this endpoint gives
\(A b^2-Bba+C a^2\), of degree at most four. A convex quadratic's stationary
minimum has numerator \(4AC-B^2\), also of degree at most four; the condition
that the stationary point lies in the feasible interval must still be tested
with correct denominator signs. Unbounded x tails also require leading and
linear coefficient signs. Bound switches, zero slopes, repeated roots, and
identically zero gaps must receive separate exact treatment.

An individual root of one of these quartics has degree at most four over
\(\mathbb F\), hence at most eight over \(\mathbb Q\). This bounds individual
roots, not a common field containing all event and positivity roots: their
composita can have much larger degree. Exact algebraic-real isolation with
defining polynomials and rational isolating intervals, plus exact ordering
or Sturm comparisons, is sufficient; floating-point sorting is not. These
degree bounds do not predict arrangement size, machine runtime, or proof
completion. Completeness of the active envelope events and positivity across
all resulting cells remains the unexecuted next programme.

Tiny determinant replay:

```sh
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/preflight/verify_pruning.py
```

It passed all ten bases, uses only the standard library, and writes no files.
