# A facet-splitting bank miss and an exact continuum repair

Agent-derived, 4 October 2026. This packet owns one concrete facet and tilt.
The integrating lead owns the full cut-fraction extension under
[../facet-fraction/](../facet-fraction/).

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/facet-split/verify.py
```

The exact check passes: the fixed 24-section bank misses the relevant
first-order capacity response, but an unsampled stationary continuum supplies
an explicit strict decrease of the ratio throughout `0 < tau <= 1/100`.
This is an obstruction, not a new maximum.

## First-order splitting formula

For a positive touching section j with word σ and facet f in position i, let
β=β_i and let μ be the four normal-closure multipliers, with final multiplier
1. Set

\[
k_{jf}=\Omega\left(\sum_{l>i}\beta_l a_{\sigma_l}^T-
\sum_{l<i}\beta_l a_{\sigma_l}^T\right)-\mu.
\]

Duplicate f immediately before or after its old occurrence, assign the new
label weight δ and old label weight β−δ, with 0<δ<β. At the identical-normal
base this is still a positive feasible touching section. Its five old pivot
columns remain nonsingular, so the new normal
`a_new = a_f + tau D` has a genuine positive feasible continuation.

Stationarity makes the feasible weight-jet contribution to DQ equal to the
negative multiplier times the closure perturbation. Direct differentiation
then gives

\[
DQ=\delta\,k_{jf}\cdot D
\ \pm\ \delta(\beta-\delta)(\Omega a_f^T)\cdot D,
\]

with plus for a new occurrence before the old one and minus after it. Since
Q₀=1/2 and c₀=1, the first-order capacity-loss response is

\[
\ell_{j,\delta,\pm}(D)=2\delta\left[k_{jf}\cdot D
\pm(\beta-\delta)(\Omega a_f^T)\cdot D\right].
\]

The old normals may also vary. When both copies inherit the same old-facet
variation, their gradients sum to the old section's facet gradient. Thus the
full first-order ratio upper function is

\[
G_j\cdot\dot A +2L_f(D)-2\ell_{j,\delta,\pm}(D),
\qquad
L_f(D)=\frac1{\|a_f\|}\int_{F_f}(D\cdot x)_+\,dS.
\]

This formula does not itself cancel arbitrary old-normal motion: that needs a
positive gradient balance including the chosen splitting sections. Fixed-old-
normal evidence is stated separately below.

## Concrete failed response bank

Choose the top q facet f=0, with a₀=(0,2,0,0), and

\[
D=(0,-149/100,1,0),\qquad c=149/200,\quad\eta=3/4-c=1/200.
\]

On this facet, D·x=p₁−c. Thus the new facet cuts a nontrivial fraction near the
right tip of T*, while leaving most of the old facet intact. This is a true
facet split at order τ, rather than an old-normal chart with ten facets.

Here `(Omega a0)·D = 0`. For all 20 members of the fixed 24-section bank that
use f=0, `k_q2 = 1/2` and

\[
k_{p_1}\leq48/65<149/200.
\]

Consequently every duplicate from this bank has negative capacity-loss
response for δ>0. Including sections that do not use the new facet gives bank
best response 0. The removed-volume response is nevertheless positive. This
is an exact failure of this finite bank, not evidence of an increasing actual
ratio or completeness of a missing-orbit argument.

## Explicit stationary continuum repair

The existing word

\[
\sigma=(0,6,4,8,2,1,9)
\]

admits the whole positive stationary continuum

\[
\beta(r)=(r,1/4,1/4-r/2,1/8,7r/4,1/4-9r/4,1/8),
\qquad0<r<1/9,
\]

with μ=(−1/4,−1/2,3r−3/4,2r) and final multiplier 1. Closure, stationarity
and Q=1/2 hold identically in r. Its f=0 response vector is

\[
k_0(r)=(1/4,1/2,3/4-3r,0).
\]

The sampled member r=1/260 is section 14, but the useful member here is
r=η/6=1/1200. Choose δ=r/2=1/2400 and put the new label before the old label:

\[
\widehat\sigma=(10,0,6,4,8,2,1,9).
\]

Keep δ and all old nonpivot weights fixed. On the old pivot positions
(0,2,3,4,6), whose labels are (0,4,8,2,9), prescribe the exact weight jet

\[
\dot\beta_I=(7c\delta/9,-7c\delta/18,3\delta/16,
-7c\delta/18,-3\delta/16).
\]

The pivot determinant is −4608/49. Old normals are fixed, so these linear
weight formulas satisfy closure for every τ. All weights stay positive on
`0 <= tau <= 1/100`. Direct pairwise evaluation gives

\[
Q(\tau)=\frac12+\delta(\eta-3r)\tau-
\frac56c\delta^2\tau^2.
\]

Thus the capacity-loss response is

\[
\ell=2\delta(\eta-3r)=1/480000.
\]

## Exact volume and whole-window comparison

For p₁>c, the new q-height is

\[
q_2=\frac12-h(p_1),\qquad
h(p_1)=\frac{\tau(p_1-c)}{2(1-c\tau)}.
\]

Near the top edge H* has horizontal width `5/6+2h/3`; its removed strip area
is `(5/6)h+(1/3)h²`. T* has vertical width `1-4p1/3` for positive p₁, hence

\[
I_1=\int_c^{3/4}(p_1-c)(1-4p_1/3)\,dp_1=2\eta^3/9,
\quad
I_2=\int_c^{3/4}(p_1-c)^2(1-4p_1/3)\,dp_1=\eta^4/9.
\]

Consequently the exact volume is

\[
V(\tau)=\frac12-
\frac{5\tau I_1}{12(1-c\tau)}-
\frac{\tau^2 I_2}{12(1-c\tau)^2}.
\]

The volume-loss response is L=5η³/54=1/86400000, so

\[
\ell-L=179/86400000>0.
\]

The verifier goes beyond this first-order comparison: it clears the positive
rational denominators of `8 V(tau) Q(tau)^2 - 1`, removes its τ factor, and
checks positive exact Bernstein coefficients on `[0,1/100]`. Thus the feasible
upper bound `rho <= 1/(8VQ²)` is strictly below 1 throughout the whole stated
window. It checks all ten old facets and the new facet remain genuine.

The named rival is the true active continuum: a finite bank can miss a
capacity response when its positive weights stay away from a zero-weight
endpoint. The remaining unrestricted bridge concerns arbitrary tilts, coupled
old-normal motion, equality-parameter drift, and the first-order-flat
transverse direction; none is silently certified by this one splitting ray.
