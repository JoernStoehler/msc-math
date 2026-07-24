# Product closure-vertex result

Status: pre-production research result measured in release mode on 2026-07-23.
The retained producer output is `sample5.jsonl`.

## Verdict

The KKT-free closure-vertex route works in practice for the tested scalar
capacity and sparse-witness contract. It is exact for the binary64 input,
substantially faster than the old product KKT route once products cease to be
tiny, and remained correct on deliberately degenerate and near-degenerate
inputs.

This is enough evidence to design the production product-capacity API. It is
not evidence that the route returns every minimizing branch, and it has not
yet been migrated into `crates/symplectic`.

## Behavioral and numerical evidence

The retained 88 cases comprise three known products, three adversarial
products, two scaling cases, and 80 generated products with three to six
facets per factor. The audit found:

- zero errors and zero hybrid/exact capacity or winner-value disagreements;
- zero failures of the exact transition test or allowed winner type patterns;
- zero interval violations among 1,835 closure weights and 173,496 objectives;
- zero wrong determinate ternary objective signs;
- zero disagreements with the general exact KKT route on its 18 cheap cases;
- 36 exact support fallbacks and 987 exact final contenders; and
- no complete-route exact fallback on the tested gradual-underflow platform.

Literal `q > 0` disagreed with the exact sign 23,559 times, mostly at exact
zero or tiny objectives. The interval predicate made those cases
indeterminate instead of making a wrong claim.

The largest closure-weight absolute error was `2.22e-16`. The largest relative
error was `0.414`, on a regular 16-facet product whose smallest positive exact
binary64-rational closure weight was only `3.93e-17`. This is a useful
near-degeneracy stress: the floating value alone is poor, while interval
classification plus exact fallback remains correct.

The smallest nonzero exact objective magnitude was `9.43e-50`. The largest
objective absolute error was `8.33e-17`, or `5.96e-16` relative to that case's
maximum objective. The largest objective interval width was `5.05e-15`.

### Retained datascience product population

The separate retained sys-datascience population provides a larger
falsification check for the six-facet existence claim. It contains 10,240
accepted random products: 1,024 rows in each polygon-pair bucket
`3x3, 3x4, ..., 6x6`. Matching the producer geometries against the shared
capacity cache found:

- all 10,240 producer geometries exactly once;
- a cached literal winning word of length exactly six for every geometry;
- zero cached winners of length at least seven; and
- exact equality, as stored binary64 values, between every product record's
  scalar capacity and its winning sigma action.

Thus the proposed relative-action relaxation was unnecessary on this
population: a length-six literal winner was already selected in every row.
The shared cache also contains 100 earlier product-sample rows, ten per bucket;
their literal winners all have length six as well.

This is empirical support, not a proof of the six-facet theorem and not a claim
that longer tied minimizers do not exist. The check used the LFS objects
`66bf82010e...` (`produce/random-product.jsonl`) and `abf6ce2189...`
(`produce/shared-cache.jsonl`) as retained on 2026-07-24. The producer/cache
join key was the exact serialized `dual_vertices_rational` array, not a rounded
geometric identifier.

## Performance

Times are route-internal totals from one release-mode run on the same machine.
The hybrid and old-kernel columns begin after common geometric input
validation; each includes its own candidate construction and numerical work.
The exact-all column is a reference oracle, not a production candidate.

| Case | Facets | Orders | Hybrid | Old product KKT kernel | Exact all |
| --- | ---: | ---: | ---: | ---: | ---: |
| triangle product | 6 | 120 | 1.62 ms | 0.61 ms | 70 ms |
| HKO pentagon product | 10 | 3,000 | 27.2 ms | 202 ms | 1.29 s |
| square product with exact zeros | 8 | 24 | 0.19 ms | 36.4 ms | 0.55 ms |
| regular 7-by-7 product | 14 | 23,520 | 49.0 ms | not run | 12.4 s |
| regular 8-by-8 product | 16 | 48,000 | 481 ms | not run | 29.5 s |

Across the 86 cases cheap enough for both production-shaped routes, the hybrid
route took 268 ms and the old product candidate/KKT kernel 3.91 s, an aggregate
`14.6x` speedup. For the 51 cases with at least nine facets, the totals were
208 ms and 3.81 s, an aggregate `18.3x` speedup. Including old-route validation
changes the comparison only slightly.

The hybrid route was slower on 16 of the 86 individual cases. These were tiny
products or cases with several exact ties; the exact final comparison is then
more expensive than the old approximate result. The triangle product is the
clearest example, but both routes remain below two milliseconds. On the HKO
fixture, exact final contender resolution is now the dominant phase. Caching
exact factor closure vertices reduced that case by about 12%; further
specialization was not retained because it would add complexity to save
milliseconds rather than change the route choice.

## What this establishes

The mathematical completeness claim comes from
`formal/product-qp-six-facet-reduction.tex`, not from these finite tests. The
tests exercise the implementation against independent exact arithmetic,
including intermediate quantities and adversarial boundary cases.

The practical result is:

- use closure-vertex enumeration instead of KKT solves for scalar capacities
  of validated four-dimensional Lagrangian products;
- retain ternary outward predicates and exact final resolution;
- do not replace them by raw f64 signs; and
- keep the old branch until production API and consumer-output requirements
  are checked, because this route intentionally returns only sparse winners.

## Review

A fresh read-only adversarial review checked the proof-to-code bridge,
closure-support and cyclic-order coverage, interval arithmetic, exact contender
selection, output scope, retained evidence, and performance interpretation. It
found no material correctness or evidence gap.

It found one audit-only overflow panic outside the accepted coordinate range:
the hybrid correctly selected complete exact fallback, but the diagnostic tried
to convert an infinite interval endpoint to a rational. The audit now treats
infinite endpoints as unbounded, and a `1e308` square-product regression test
exercises that path. This does not weaken the accepted-input contract or turn
nonfinite input into accepted input.

The exact-all reference shares support enumeration, cyclic-order enumeration,
exact objective evaluation, and reporting with the hybrid. It independently
checks interval pruning, but it is not an independent implementation. The
independent evidence consists of the general exact KKT comparisons on 18 cheap
cases, old-route comparisons on 86 cases, known capacity values, and the
mathematical/code review.
