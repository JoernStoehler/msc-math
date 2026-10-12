# Exact premises for the moving-old facet-split bridge

Agent-derived, 4 October 2026, 23:42 UTC. All retained exact checks pass.
The integrating lead owns the analytic theorem in
[../../moving-old-bridge.md](../../moving-old-bridge.md); its independent
mathematical review is separate from the acceptance below.
Independent assembled acceptance was returned at23:44 UTC: see
[the review record](../../independent-moving-old-review.md). The checker pass
supplies exact premises; the analytic theorem additionally uses the retained
F10 chart, gradient-rank, symmetry and transverse-volume inputs.

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/facet-split/moving-old/verify.py
```

This packet supplies the missing premises for a genuine new facet while all
old normals move. It concerns an open patch about
`D0=(0,-149/100,1,0)`. It does not classify every facet-splitting direction or
approaches to the boundary of that patch.

## Exact positive gradient balance

Use the stationary continuum from [../README.md](../README.md), at
r₀=1/1200, and split its facet-0 weight with δ=1/2400. Its old-normal ratio
gradient is Gᵣ. Duplicating the normal leaves the sum of the two copies'
gradients equal to Gᵣ. The existing 24 old gradients Gⱼ have rank 22.

`certificate.json` retains 24 strictly positive rational coefficients λⱼ and
γ=1/125 such that

\[
\sum_{j=1}^{24}\lambda_j+\gamma=1,\qquad
\sum_j\lambda_jG_j+\gamma G_r=0
\quad\text{in the full forty-dimensional normal chart}.
\]

The verifier recomputes all 24 old gradients and Gᵣ, rather than accepting an
unchecked extra row. The added row leaves the rank 22. For the displayed tilt,
its capacity-loss response is ℓ=1/480000 and the volume-loss response is
L=1/86400000. The balanced mean therefore has strict splitting response

\[
\gamma\ell-L=\frac{11}{2160000000}>0,
\qquad F_\tau=-2(\gamma\ell-L)<0.
\]

The finite 24-section bank alone misses this response. A positive balance
including the unsampled stationary contact repairs the miss, so the first-order
split loss survives arbitrary old-normal first-order motion.

## Equality-family contact and row-space interface

For E(v,s), write the word as `(0,6,4,8,2,1,9)`. Over QQ(v,s,r), its retained
weight is

\[
\begin{aligned}
\beta_0&=r,&\beta_6&=v/2,&\beta_4&=rv-r+1/4,\\
\beta_8&=(v-v^2)/2,&
\beta_2&=(2rsv^2-4rsv+2rs-rv^2+rv)/(2s),\\
\beta_1&=(-4rsv^2+4rsv-4rs+2rv^2-2rv+s)/(4s),&
\beta_9&=(v-1)^2/2.
\end{aligned}
\]

The four closure multipliers are retained in the certificate. Every row of the
full stationary system is checked identically, with final multiplier 1; thus
closure holds and Q=1/2 identically. All coefficients are regular at the base
and all weights are positive there at r=r₀. The old five pivot positions
`[0,2,3,4,6]` have base determinant −4608/49. They therefore define an actual
positive smooth feasible section near the equality family, including the
new-facet parameter, rather than only a formal first jet.

The contact gradient annihilates the 15 symmetry tangents, both equality-family
tangents, and the maintained W(v,s), over QQ(v,s,r). The verifier also matches
W to the exact normalized four-row construction of the existing proof.
Together with that proof's constant rank 22 and 18 independent kernel columns,
this places Gᵣ in the old gradient row space throughout a common parameter
neighborhood.

Consequently the base positive balance extends smoothly near `(v,s)=(1/2,1/12)`
with γ held fixed: solve the rank-23 system consisting of the 22 quotient
balance equations plus normalization, holding the remaining free old coefficient
at its base value. The rank follows from the old rank-22 matrix and its positive
normalized null relation. Strict positivity persists by continuity. An explicit
parameter radius is not asserted.

## The missing transverse direction has the same negative curvature

Keep all p weights fixed. Along A(v,s)+zW(v,s), vary only the q weights at labels
`[0,4,2]` by the retained rational jet. It satisfies both closure and sum-zero
identically. At the base the word-position jet is

\[
(-7/216,0,77/864,0,-49/864,0,0).
\]

Every first edge variation lies in the q Lagrangian plane, so the coefficient
of z² in Q is zero. The verifier separately checks the coefficient of z is
zero over QQ(v,s,r). Thus this is an actual feasible linear weight section with

\[
Q(A(v,s)+zW(v,s),\beta(v,s)+z\dot\beta(v,s))=1/2
\]

identically. At the base the known exact volume line is
V=1/2+13z²/1728, hence the added touching upper function is exactly

\[
U_r(z)=\frac{864}{864+13z^2},\qquad U_{r,zz}(0)=-13/432.
\]

This is the same negative curvature as the original 24 sections. Its positive
inclusion in the balanced mean therefore preserves negative W curvature;
continuity preserves that sign near the equality family.

The old pivot continuation realizes this W jet automatically: the old nonpivot
weights and the new duplicate weight are fixed, p closure leaves p weights
unchanged, and the three q pivot weights absorb the W displacement.

## Joint volume regularity at the facet birth

At the base, facet 0 has eight vertices. D₀·x=p₁−149/200 is nonzero at each:
p₁ takes only values −1/4, 0 and 3/4. The sign pattern persists for old normals
and tilt directions in sufficiently small neighborhoods. It cuts two old facet
vertices and crosses a fixed collection of edges.

New vertices involving both the old facet and new facet satisfy
`a_f x=1` and `(a_f+tau D)x=1`. Subtracting these equations and dividing by τ
replaces the new equation by `D x=0`. The remaining matrix is nonsingular:
D varies along each crossed old edge because its endpoints have opposite signs.
This removes the apparent τ singularity. New vertices near an old vertex that
do not involve both copies have an ordinary nonsingular old vertex matrix at
τ=0. Therefore all relevant vertex functions are regular rational functions
of old normals, τ and D on this fixed `tau>0` chamber.

A fixed pulling triangulation now gives a volume function with a smooth
one-sided extension through τ=0. Its first derivative is precisely
`−integral_facet (D·x)_+/norm(a_f)`, with smooth dependence on the old normal
chart and the tilt patch. This supplies the Taylor-remainder interface needed
by the integrating analytic argument; the capacity sections already have
regular pivot continuation.

## Analytic bridge and honest quantifier

These premises support the independently accepted local theorem: old normals
range over a neighborhood of P* in
the ten-facet chart, D ranges over a sufficiently small open patch about D₀,
and the new normal is `a_f + tau D`, with τ≥0 sufficiently small. The ratio is
at most 1, with strict loss for a genuine split τ>0. The equality-family and
symmetry variables are retained rather than incorrectly treated as positive
curvature directions.

The mean upper bound has zero old-normal gradient, strictly negative W² term,
and strictly negative τ term; the old min-envelope retains its linear cusp
in the 22 remaining transverse directions. A two-regime argument using
`norm(y) <= C(z²+tau)` versus its complement controls mixed Taylor terms.
The exact packet supplies the premises; the analytic implication is owned and
reviewed in the linked bridge note. No explicit neighborhood radius, global
classification, or all-direction facet-birth theorem is asserted here.
