# Exact Clarke Orbit-Catalog Working Note

## Purpose

Record the current Packet 2 working hypothesis for the minimizing-family
catalog behind the exact Clarke checker for HKO2024.

This is not yet the final theorem-facing catalog. It is the current compressed
paper-plus-numerics surface that later exact work should either prove or
replace.

## Current Source Surface

- Paper source:
  `papers/hko2024/counterexample.tex`, especially the proof of
  Proposition 3 and the minimizing-family remark immediately after it.
- Numerical reconciliation artifact:
  `experiments/hko-local-maximum/exact-clarke/numerical-family-reconciliation.json`.
- Exact endpoint certificate:
  `experiments/hko-local-maximum/exact-clarke/endpoint-prototype-certificate.json`.
- Exact gradient-reduction certificate:
  `experiments/hko-local-maximum/exact-clarke/segment-gradient-reduction.json`.
- Exact dual-vertex row-reduction certificate:
  `experiments/hko-local-maximum/exact-clarke/segment-a-gradient-reduction.json`.
- Billiard sigma-surface count ladder:
  `experiments/hko-local-maximum/exact-clarke/billiard-sigma-counts.json`.
- Exact-sigma feasibility probe:
  `experiments/hko-local-maximum/exact-clarke/billiard-exact-probe.json`.

## Paper-Level Picture

The paper gives two minimizing mechanisms:

- `2`-bounce minimizing trajectories connecting a pentagon vertex to the
  opposite edge. This includes the diagonal trajectory as a special case and
  already yields the minimum action.
- Additional `3`-bounce minimizing trajectories coming from equality cases in
  the proof after Proposition 3. For fixed `x_1`, the minimizing trajectories
  fill a cone in which `x_2` and `x_3` may vary while keeping the same
  `T^circ`-length.

So the remaining Packet 2 task is not to discover new numerical minima but to
translate these two minimizing mechanisms into a finite repo-facing catalog.

## Candidate Surface Scale

The repo now records the current size of the HKO2024 finite candidate surfaces
at each pruning rung:

- raw billiard block words:
  `50,400`, coming from the shape
  `([Q|QQ][P|PP])^k` with `k = 2, 3`,
  `15` blocks per side,
  `60` non-overlapping block selections per side for each `k`,
  and permutation factors
  `60 * 60 * 1 * 2 = 7,200` for `k = 2`,
  `60 * 60 * 2 * 6 = 43,200` for `k = 3`;
- directed-feasible sigma words after the current `omega_0` cycle filter:
  `6,240`;
- current numerically valid KKT orbits in the committed HKO artifact:
  `717`;
- current exact minima in that artifact:
  `150`.

So the practical difference between two possible theorem inputs is large:

- an exhaustive route starting from the already-valid orbit surface is only a
  small multiple of the exact minima;
- an exhaustive route starting from the theorem-native billiard sigma-word
  surface pays a much larger front-end certification cost.

The current sympy probe makes this more concrete in the present environment:

- one sampled directed-feasible sigma took about `8.08s` for the exact quartic
  KKT linear solve attempt;
- the resulting one-sample projection is about `50,438s`, roughly `14` hours,
  for all `6240` directed-feasible sigma words;
- that probe does not yet include exact positivity certification or exact
  action-gap elimination, so it is a lower-bound style estimate on the full
  theorem workload for this route.

So the statement “`6240` exact checks sounds feasible” depends strongly on the
backend. In the current sympy-based exact tooling, it does not yet look cheap.

## Symmetry-Reduced Numerical Hypothesis

Modulo diagonal `72^o` rotation and `q/p` exchange, the current exact-action
numerical artifact and the exact prototype certificates suggest the following
reduction:

- one endpoint `6`-facet prototype;
- one `7`-facet equality-case segment family joining neighboring endpoints.

The raw numerical collector reports:

- `44` exact minima on `6` visited facets;
- `106` exact minima on `7` visited facets;
- `10` visited `6`-subsets and `10` visited `7`-subsets;
- one symmetry orbit of visited `6`-subsets and one symmetry orbit of visited
  `7`-subsets.

## Endpoint Prototype

Use the neighboring endpoint pair

- `E_left`:
  `subset = [0, 1, 3, 6, 7, 9]`,
  `permutation = [0, 1, 7, 6, 3, 9]`;
- `E_right`:
  `subset = [0, 2, 3, 6, 7, 9]`,
  `permutation = [0, 6, 7, 2, 3, 9]`.

Let

- `a = sqrt(5) / 10`,
- `b = (5 - sqrt(5)) / 20`.

The numerical endpoint beta pattern is consistent with the exact facetwise
profiles

- `beta(E_left)` on facets `[0, 1, 3, 6, 7, 9]`:
  `[b, b, a, b, b, a]`;
- `beta(E_right)` on facets `[0, 2, 3, 6, 7, 9]`:
  `[a, b, b, b, b, a]`.

Up to symmetry and cyclic relabeling, the `6`-facet exact minima appear to use
only this endpoint beta multiset:

- `{a, a, b, b, b, b}`.

This endpoint prototype is now certified exactly in the repo:

- positive permutation `[9, 3, 6, 7, 1, 0]`,
- beta profile `[a, a, b, b, b, b]`,
- exact closure `0`,
- exact normalization `1`,
- exact `Q = sqrt(5 - 2 sqrt(5)) / 5`,
- exact action `5 / (2 sqrt(5 - 2 sqrt(5)))`.

## Equality-Case Prototypes

Take the union subset

- `U = [0, 1, 2, 3, 6, 7, 9] = subset(E_left) union subset(E_right)`.

On this ordered facet set, extend the endpoint beta profiles by zero on the
missing facet:

- `beta(E_left; U) = [b, b, 0, a, b, b, a]`,
- `beta(E_right; U) = [a, 0, b, b, b, b, a]`.

### Equality-Case Segment

The paper's equality cases on the union subset `U` are naturally represented by
the exact one-parameter facetwise segment

- `beta_seg(U; lambda) = (1 - lambda) beta(E_left; U) + lambda beta(E_right; U)`.

The repo now contains an exact certificate that for every `lambda`:

- closure remains exactly `0`,
- normalization remains exactly `1`,
- the stationarity equations admit exact multiplier formulas
  `mu(lambda), xi(lambda)`,
- the top KKT residual is exactly `0`,
- `Q(lambda) = sqrt(5 - 2 sqrt(5)) / 5`,
- therefore the action remains
  `5 / (2 sqrt(5 - 2 sqrt(5)))`.

So the current exact evidence says that the theorem-facing equality-case
surface is not a finite list of isolated `7`-facet beta points. It is a
constant-action exact KKT segment between neighboring endpoint beta profiles.

Moreover, the exact segment certificate shows that `xi` is constant along this
segment. Since the height derivative of the action uses the KKT formula
`dA/dh_k = -xi beta_k / (2 Q^2)` on visited facets and `0` on unvisited
facets, the capacity-height derivative family is affine in `lambda`.

The repo now also contains an exact reduction artifact for the corresponding
`sys` height-gradient family. It records:

- the exact endpoint facetwise beta data;
- the exact segment facetwise beta data;
- the exact capacity-height derivative profiles on both endpoints and on the
  whole segment;
- zero affine residual on every facet for the capacity-height family;
- an abstract `sys`-height model
  `grad_sys = gamma * grad_capacity - delta * 1`
  with common scalars `gamma, delta`, again with zero affine residual on every
  facet.

This height-space reduction is useful, but it is not yet the theorem-facing
`R^40` reduction, because the exact dual-vertex row family does not collapse
all the way to the two endpoint rows.

The new dual-vertex row artifact records the correct Packet 2 reduction for the
checker:

- in exact dual-vertex coordinates, the seven-facet capacity row family is a
  degree-`2` polynomial in `lambda`;
- its `40` coordinates are exactly recovered by the Lagrange interpolation
  through `lambda = 0`, `1/2`, and `1`;
- because the Lagrange coefficients sum to `1` and the volume derivative row
  is orbit-independent, the same three-row interpolation holds for the exact
  `sys` rows.

So the theorem-facing reduction surface is currently:

- the neighboring seven-facet family does not reduce to the endpoint rows
  alone in `R^40`;
- it does reduce to the span of three exact prototype rows:
  left endpoint, midpoint, and right endpoint.
- the two segment endpoints coincide exactly with the corresponding six-facet
  endpoint-family rows, so the only genuinely new prototype row contributed by
  this neighboring seven-facet family is the midpoint row.

### Midpoint Representative

The midpoint representative is the special value `lambda = 1/2`, giving

- `beta_mid(U) = (beta(E_left; U) + beta(E_right; U)) / 2`
- `= [(a+b)/2, b/2, b/2, (a+b)/2, b, b, a]`.

With

- `c = b / 2 = (5 - sqrt(5)) / 40`,
- `d = (a + b) / 2 = (5 + sqrt(5)) / 40`,

the midpoint multiset is

- `{a, d, d, b, b, c, c}`.

This matches the dominant `7`-facet numerical family.

This midpoint prototype is now certified exactly in the repo as a point on that
exact KKT segment:

- positive permutation `[9, 2, 3, 6, 7, 1, 0]`,
- beta profile `[a, c, d, b, b, c, d]`,
- exact closure `0`,
- exact normalization `1`,
- the same exact action `5 / (2 sqrt(5 - 2 sqrt(5)))`.

### Asymmetric Numerical Representative

One numerical collector representative on this exact segment uses

- `lambda approx 0.129573855671`.

On the ordered facets `[0, 1, 2, 3, 6, 7, 9]`, the segment formula gives

- `[b + lambda (a-b), (1-lambda) b, lambda b, a - lambda (a-b), b, b, a]`.

The current numerical artifact then shows

- `b + lambda (a-b) approx 0.149263529615`,
- `a - lambda (a-b) approx 0.212539869260`,
- `lambda b approx 0.017906666448`,
- `(1-lambda) b approx 0.120289934677`.

The reconciliation artifact records exact agreement at the current rounded
precision between this segment formula and the collected asymmetric
`7`-facet beta profiles. The important point is that this `lambda` is now best
viewed as one numerical representative on the exact equality-case segment, not
as a distinguished theorem-facing constant.

## What Is Still Missing

- A paper-derived exact proof that the equality-case surface in the paper is
  exactly the KKT segment above in repo notation.
- A paper-derived proof that the equality-case surface in the HKO billiard
  geometry is exactly this neighboring-endpoint segment in repo notation.
- The actual reduced active-gradient matrix built from the three-row segment
  reduction surface and compared against the symmetry tangent space.

## Current Best Use

Use this note as the working Packet 2 target surface:

- if the paper geometry can be translated into these symmetry-reduced
  prototypes, then Packet 3 can build the exact active-gradient matrix from
  the endpoint row together with the exact three-row reduction for each
  neighboring equality-case segment, rather than from `150` numerical orbit
  payloads;
- if the paper geometry forces a different finite family surface, replace this
  note with the corrected exact catalog and record the mismatch explicitly.
