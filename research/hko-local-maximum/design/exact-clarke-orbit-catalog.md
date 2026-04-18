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

## Symmetry-Reduced Numerical Hypothesis

Modulo diagonal `72^o` rotation and `q/p` exchange, the current exact-action
numerical artifact suggests the following reduction:

- one endpoint `6`-facet prototype;
- two equality-case `7`-facet beta prototypes:
  one midpoint split and one asymmetric split.

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

### Midpoint Split

The first `7`-facet beta prototype is the facetwise midpoint

- `beta_mid(U) = (beta(E_left; U) + beta(E_right; U)) / 2`
- `= [(a+b)/2, b/2, b/2, (a+b)/2, b, b, a]`.

With

- `c = b / 2 = (5 - sqrt(5)) / 40`,
- `d = (a + b) / 2 = (5 + sqrt(5)) / 40`,

the midpoint multiset is

- `{a, d, d, b, b, c, c}`.

This matches the dominant `7`-facet numerical family.

### Asymmetric Split

The second `7`-facet beta prototype is numerically consistent with a facetwise
convex combination

- `beta_asym(U) = (1 - lambda) beta(E_left; U) + lambda beta(E_right; U)`,

for a coefficient

- `lambda approx 0.129573855671`.

On the ordered facets `[0, 1, 2, 3, 6, 7, 9]`, this gives

- `[b + lambda (a-b), (1-lambda) b, lambda b, a - lambda (a-b), b, b, a]`.

The current numerical fit suggests

- `b + lambda (a-b) approx 0.149263529615`,
- `a - lambda (a-b) approx 0.212539869260`,
- `lambda b approx 0.017906666448`,
- `(1-lambda) b approx 0.120289934677`.

The reconciliation artifact records exact agreement at the current rounded
precision between this convex-combination formula and the collected asymmetric
`7`-facet beta profiles.

## What Is Still Missing

- A paper-derived exact proof that the equality-case surface reduces to the two
  `7`-facet beta prototypes above.
- An exact derivation of the asymmetric coefficient `lambda`, or a proof that
  it is not an independent parameter in the theorem-facing catalog.
- A proof that the equality-case `7`-facet families add no new extremal
  first-order constraints beyond the endpoint family if that is indeed the
  correct Clarke-theoretic reduction.

## Current Best Use

Use this note as the working Packet 2 target surface:

- if the paper geometry can be translated into these three symmetry-reduced
  prototypes, then Packet 3 can build the exact active-gradient matrix from
  them rather than from `150` numerical orbit payloads;
- if the paper geometry forces a different finite family surface, replace this
  note with the corrected exact catalog and record the mismatch explicitly.
