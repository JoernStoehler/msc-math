# P* vertex birth: an exact obstruction to a new nonproduct maximum

**Return, 4 October 2026, 23:06 UTC.** This bounded probe produced a new
agent-derived obstruction, with exact rational-function checks. It did not
produce a new maximum. Independent mathematical review is pending. The base
capacity and volume are taken from
`experiments/hko-local-maximum/non-hko/PROOF.md`; no unrestricted local-maximum
claim is made for P*.

The retained full-cone packet is [pstar-f11/vertex-cones/](pstar-f11/vertex-cones/).
Its verifier and 24 witnesses are lasting project evidence; exploratory scripts
and failed constructions remain disposable under
`/tmp/msc-math/nonproduct-mechanisms/`.

## Named statement: full vertex-cone cut obstruction

Let P*=H*×T* be the rational ten-facet body in the existing proof, with
capacity 1 and volume 1/2. At any of its 24 vertices v, let I(v) be the four
incident facet indices and write their row-normal matrix as N. For any fixed
positive coefficients t_i with sum 1, set

\[
w=\sum_{i\in I(v)}t_i a_i,\qquad
P_{v,t}(\varepsilon)=P_*\cap\{x:w\cdot x\leq1-\varepsilon\}.
\]

**Agent-derived theorem.** For every v and every such t, there exists
ε₀(v,t)>0 such that

\[
0<\varepsilon<\varepsilon_0(v,t)
\quad\Longrightarrow\quad
\rho(P_{v,t}(\varepsilon))<1.
\]

These are genuine eleven-facet nonproduct perturbations. For each of the 24
barycentric directions t_i=1/4, the retained exact checks give the common
window 0<ε≤1/100. This common window is not asserted over entire normal cones.

## Derivation and explicit witness

Use fixed normals and variable support numbers, as in
`formal/active-orbit-facet-coverage.tex`. For any closed nonnegative normal
weight b, a word of distinct facets gives the feasible HK action upper bound

\[
c(P)\leq \frac{(b\cdot h)^2}{2Q(b)},\qquad
Q(b)=\sum_{j<k}b_jb_k\,a_{\sigma_j}\Omega a_{\sigma_k}^T.
\]

The new facet normal is w, with support 1−ε; old supports are 1. For every
vertex cone the retained witness supplies a word and weights
b(δ)=b⁰+δd(t) with

\[
\sum_jb_j^0=1,\quad\sum_jb_j^0a_{\sigma_j}=0,
\quad\sum_jd_j=0,\quad\sum_jd_ja_{\sigma_j}=0,
\]
\[
b^0_{\rm new}=0,\quad d_{\rm new}=1,\qquad
Q(b^0+\delta d)=\tfrac12-C(t)\delta^2.
\]

Every old zero-base-weight coordinate of d is a homogeneous linear polynomial
with nonnegative coefficients. Thus b(δ)≥0 for all sufficiently small δ≥0
for each fixed t. Every C(t) is a nonzero homogeneous quadratic polynomial
with nonnegative coefficients. Consequently C(t)>0 whenever all t_i>0.
The witness verifier checks identities after substituting
`t3=1-t0-t1-t2`, rather than evaluating only sampled parameter values.

For example, take v=(−5/12,1/2,3/4,0), with incident facets
(0,5,6,9), and t=(t₀,t₁,t₂,t₃) in this order. An explicit word is

\[
\sigma=(1,2,6,3,9,0,10,5,7),
\]

where 10 is the new facet, with

\[
b^0=(0,1/4,1/8,0,1/4,0,0,1/4,1/8),
\]
\[
d=(0,-t_0,-t_2,t_0,-t_3,0,1,-t_0-t_1,0),
\]
\[
C(t)=\frac47(7t_0t_2+7t_0t_3+2t_1t_2+6t_1t_3)>0.
\]

The normalization gives b(δ)·h=1−εδ. Set δ=ε/(2C(t)), which is feasible
for all sufficiently small ε. Direct substitution gives the exact bound

\[
c(P_{v,t}(\varepsilon))\leq
\frac{(1-\varepsilon\delta)^2}{1-2C(t)\delta^2}
=1-\frac{\varepsilon^2}{2C(t)}.
\]

At the simple vertex use slack coordinates y_i=1−a_i·x. For a sufficiently
shallow cut the removed cap is exactly the simplex

\[
y_i\geq0,\qquad\sum_i t_i y_i<\varepsilon.
\]

Hence

\[
V(P_{v,t}(\varepsilon))=\tfrac12-D(t)\varepsilon^4,
\qquad D(t)=\frac1{24|\det N|\prod_i t_i},
\]

and therefore

\[
\rho(P_{v,t}(\varepsilon))\leq
\frac{(1-\varepsilon^2/(2C(t)))^2}{1-2D(t)\varepsilon^4}<1
\]

for sufficiently small ε>0. The capacity loss at order ε² dominates the
volume loss at order ε⁴. No completeness of an eleven-facet capacity bank
is needed: a single feasible action upper bound suffices at each t.

## Scientific consequence, falsifier and remaining quantifier

The naive mechanism “cut a vertex invisible to the minimizing orbits” fails
already at P*: an independent exact enumeration of the 12 primitive tight
product words reconstructed trajectories whose endpoints cover all 24
vertices. The stronger cut theorem above does not rest on coverage alone.
It constructs new-facet branches, allowing previously zero old-facet weights
to become positive. A simple redistribution only among incident facets failed
at every vertex; those extra zero-weight appearances were essential.

A falsifier is a failed retained polynomial identity, a negative coefficient
in a zero-base-weight coordinate or C(t), or an actual ε-small interior-cone
cut with capacity above the displayed feasible-action bound. A numerical
capacity result exceeding that bound must first resolve conventions and
feasibility before being treated as new geometry.

The open quantifier is on cone boundaries and coupled perturbations: cuts
exposing edges, ridges or facets, cuts whose t depends on ε and tends to a
boundary, several simultaneous cuts, and facet birth combined with motion
along the two-parameter equality family. The theorem's radius may collapse
as t approaches a cone boundary; per-ray control supplies no uniform endpoint
statement. It also does not exclude a new maximum at finite cut depth.

The main rival route is a mixed q/p ridge cut. Its removed volume is already
order ε², so it competes with the newly derived capacity loss at the same
order. The next useful group should compute those two coefficients exactly,
then resolve tied coefficients and the edge-to-ridge transition. This offers
a claim-changing positive or negative outcome, unlike more generic directions.

For literature context, Haim-Kislev's
[Dynamical extensions of Zoll to nonsmooth convex bodies, v1](https://arxiv.org/html/2511.16644v1)
defines cuts extremizers and develops a dynamical test for cut additivity;
its Example 1.12 and Proposition 1.13 discuss HKO. The present P* construction
uses explicit HK feasible weights rather than assuming those HKO statements
apply to P*.

## Reproduction

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/vertex-cones/verify.py
```

The verifier reads `witnesses.json`, rebuilds closure and quadratic matrices,
checks all 24 symbolic branches and positivity certificates, and checks the
barycentric window exactly. It performs no optimizer call and writes no old
retained experiment output. The scientific trust boundary remains the HK
formula, the existing exact base value, and elementary simplex volume.
