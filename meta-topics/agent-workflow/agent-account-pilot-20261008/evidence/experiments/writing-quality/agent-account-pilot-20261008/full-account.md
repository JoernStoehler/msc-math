# Scientific account: fixed regular pentagon, arbitrary symmetric partner

Prototype for agent consumption, 2026-10-08. This reconstructs the selected result from the [proof packet](../../../docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md), [focused literature check](../../../docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/source-check.md), and [historical exposition candidate](../../../docs/history/symmetric-partner-candidate-2026-10-03/README.md). It is outside the active manuscript. The result is agent-derived and independently checked; human mathematical/prose acceptance and publication-level novelty judgment remain outstanding.

## Statement and conventions

The ambient product is the four-dimensional Lagrangian product (K\times_L Q), with (c=c_{\mathrm{EHZ}}). Fix
\[
 n_i=(\cos(2\pi i/5),\sin(2\pi i/5)),\quad
 h=\cos(\pi/5),\quad Q=\{p:n_i\cdot p\le h, 0\le i<5\}.
\]
Thus (Q) is the regular pentagon of circumradius one, with outward facet normals (n_i); its orientation is part of the convention. Put (D=Q-Q), (w=1+h), and use (A^\circ=\{x:x\cdot y\le1\ \forall y\in A\}). Define
\[
 B=\tfrac12D^\circ=\operatorname{conv}\{\pm n_i/(2w):0\le i<5\},\qquad
 W(K)=\max\{r\ge0:rD^\circ\subseteq K-K\}.
\]
(B) is a regular decagon with circumradius (R=1/(2w)). For every centrally symmetric planar convex body (K) with nonempty interior, including smooth bodies,
\[
 c(K\times_L Q)=W(K),\qquad
 \operatorname{sys}(K\times_L Q):=
 \frac{c(K\times_L Q)^2}{2\operatorname{area}(K)\operatorname{area}(Q)}
 \le\kappa_{\mathrm{sym}}=\frac{10+6\sqrt5}{25}=0.936656314599949\ldots.
\]
The denominator is four-dimensional symplectic normalization (2!\operatorname{vol}(K\times_L Q)). Equality in the ratio occurs exactly for translates and positive dilates of (B), with the orientation specified above. Symmetry means symmetry about some center, not necessarily the origin.

## Imported statements and proof implications

The mathematical input is Balitskiy–Mitrofanov–Polyanskii's translation-cover characterization and normal-triangle reduction: [v2](https://arxiv.org/html/2603.12495v2), Theorems 2.5 and 3.3, Definition 3.2, §6.1. The proof packet cites v1; the historical candidate records a check of these statements in v2. For oriented edge vectors (v_j), (Q)-length is \(\sum_j s_Q(v_j)\), where (s_Q(v)=\max_{p\in Q}v\cdot p). Because (Q) is asymmetric this is orientation-sensitive. Capacity at least one is equivalent to every closed unit-(Q)-length curve translating into (K); for polygonal (Q), normal triangles suffice. Their oriented edges follow outward normals. The general definition admits degeneracy when opposite normals occur; this pentagon has none. Here there are ten translation classes, with underlying angles (\pi/5,\pi/5,3\pi/5), rotated through multiples of (\pi/5). Odd rotations reverse traversal orientation to restore outward-normal edge directions.

The additional project calculation calibrates all ten templates in one body (B). Let (\theta=\pi/5). If a template's longest side has length (L), the equal other sides have length (L/(2\cos\theta)). Each oriented edge has support length (h) times Euclidean length. Unit perimeter therefore gives
\[
 1=hL(1+1/h)=L(1+h),\qquad L=2R.
\]
Translate the longest side's midpoint to zero. Its endpoints are antipodal vertices of (B); the third vertex has distance (R\tan\theta), strictly below the incircle radius (R\cos(\theta/2)), since
\[
 \tan^2(\pi/5)=5-2\sqrt5<(5+\sqrt5)/8=\cos^2(\pi/10).
\]
Convexity puts each whole triangle in (B), so the criterion yields (c(B\times_L Q)\ge1).

For every convex partner, symmetric or otherwise, the curve criterion also gives (c(K\times_L Q)\le W(K)). A segment traversed out and back with displacement (v) has length (s_Q(v)+s_Q(-v)=s_D(v)). It translates into (K) exactly when (v\in K-K). Rescaling (K) by (1/r), capacity at least (r>0) therefore forces all unit segments to fit, hence (rD^\circ\subseteq K-K); convexity supplies the shorter displacements. Since (B-B=D^\circ), (W(B)=1) and the calibrated capacity is exactly one.

Centering a symmetric (K) gives (K-K=2K), hence (W(K)B\subseteq K). Monotonicity and homogeneity in the first factor give (c(K\times_L Q)\ge W(K)c(B\times_L Q)=W(K)), completing equality with the universal upper bound. Translation invariance removes the centering choice. These standard capacity properties are inputs alongside the covering criterion.

Containment also gives (\operatorname{area}(K)\ge W(K)^2\operatorname{area}(B)). Proper inclusion of full-dimensional convex bodies strictly increases area, proving the precise equality class. Finally
\[
 \operatorname{area}(B)=5R^2\sin(\pi/5),\quad
 \operatorname{area}(Q)=\tfrac52\sin(2\pi/5),\quad
 [2\operatorname{area}(B)\operatorname{area}(Q)]^{-1}=\kappa_{\mathrm{sym}}.
\]
No experiment supplies any implication in this chain.

## Scientific role, figure, evidence

This closes an infinite control class for the arbitrary-partner campaign: a counterexample with this fixed pentagon must have an asymmetric partner. The independent-affine-pentagon theorem concerns independently deforming two pentagons; it is a different class and is not a proof input here. The symmetric optimum is below one and below HKO. Nothing here proves HKO globally optimal among arbitrary partners. The historical candidate proposes placement after the affine theorem and before product-position variation; the latter can use this capacity as its Lagrangian endpoint input, without being established here.

The actual proof figure is TikZ in [section.tex](../../../docs/history/symmetric-partner-candidate-2026-10-03/section.tex). In units of (R), it draws decagon vertices at multiples of (36^\circ), the template ((-1,0),(1,0),(0,\tan36^\circ)), and the dashed incircle of radius (\cos18^\circ). Its meaning is precisely the containment step, not a sample partner or numerical fit. Rotations realize the remaining placements; traversal conventions remain separate from this unoriented drawing.

The [retained numerical collection](../../../docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetrization-v1.jsonl), record `universal_symmetric_reference`, stores floating-point geometry, ten fits, (c=0.9999999999999996), and (W=0.9999999999999998). This checks implementation consistency only. The historical candidate's two-page PDF was built and visually checked with resolved citations and no reported layout warnings; that establishes neither proof acceptance nor readability in the integrated thesis.

## Consequences and unresolved obligations

For arbitrary convex (K), (S=(K-K)/2) has (S-S=K-K). Consequently (W(K)=c(S\times_L Q)), and (c(K\times_L Q)/W(K)) measures capacity retained relative to central symmetrization. (W) is also the repository's difference-body two-bounce quantity (A2); this proof requires only its geometric definition, not the stronger billiard-class identification. The quotient depends on chosen Lagrangian factors, not arbitrary symplectomorphism invariance.

Optional consequences in the proof packet: along (K_t=(1-t)K+t(-K)), widths and (W) stay fixed; capacity concavity and reflection symmetry imply a central plateau once (W) is reached, potentially only at (t=1/2). Observed onset locations remain empirical. With (\delta=\operatorname{area}(K-K)/(4\operatorname{area}(K))-1), symmetrization gives \(\operatorname{sys}\le\kappa_{\mathrm{sym}}(1+\delta)\); violation requires (\delta>0.0676274578121\ldots), not sufficient. Sharpness of this bound and improvement over universal bounds are unclaimed. The exposition candidate deliberately omits these two consequences to avoid another measure and experiment linkage; omission is not rejection.

Acceptance still requires human proof/prose review and manuscript integration checks. The focused source check found no duplicate in the immediately inspected literature, but keyword absence is weak evidence. Two-symmetric-factor polar-inradius theorems do not directly apply to asymmetric (Q); the template containment supplies the missing equality here. Present this as an elementary fixed-pentagon consequence of the attributed criterion, with novelty unresolved pending specialist judgment. Strict asymmetric gaps remain possible and observed. No broader asymmetric theorem, global HKO optimum, or literature-wide novelty certificate follows.
