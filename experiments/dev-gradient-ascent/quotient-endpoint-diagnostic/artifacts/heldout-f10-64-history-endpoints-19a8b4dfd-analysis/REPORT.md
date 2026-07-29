# Optimizer endpoint diagnostic

The signed quotient-basis diagnostic tested 16 population-stratified `history-baseline` endpoints from an F=10 optimizer run. Each state was probed in both signs of all 25 quotient-basis axes at relative radii `0.001`, `0.0001`, `1e-05`. Every probe recomputed full `sys`.

**7/16 endpoints have an explicit improving basis direction, while 9/16 have no observed improvement at all three radii.** Of the latter, 5 also keep the same facet-incidence signature in every probe. This is a finite necessary-condition check, not a local-maximality proof.

| relative radius | positive observed | no positive observed | invalid | states with incidence changes | median best change | 10–90% | median best slope |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1e-03 | 2 | 14 | 0 | 7 | -0.000314375 | -0.000477922–9.59486e-05 | -0.0825143 |
| 1e-04 | 4 | 12 | 0 | 0 | -2.28003e-05 | -4.54934e-05–0.00012745 | -0.0662332 |
| 1e-05 | 7 | 9 | 0 | 0 | -1.27786e-06 | -4.20245e-06–2.3181e-05 | -0.0372282 |

A state fails this check whenever any tested direction has raw positive change. Invalid probes make the corresponding radius inconclusive. An incidence change is reported separately: it leaves the full-sys comparison valid, but means that the finite move crossed into a different combinatorial cell. `state-summary.csv` records every endpoint.

The diagnostic removes the tangent span of translations, scaling, and the identity-component linear symplectic action. Its signed orthonormal basis is positive spanning but not dense on the quotient sphere. Passing cannot exclude ascent between basis axes, below the smallest radius, or through branch behavior not resolved by finite probing. Failing does establish an explicit finite improving move under the current full evaluator.

## What the radius dependence says

The raw changes are small partly because the steps are small; the normalized slopes are the relevant scale. For a smooth strict local maximum one expects the best symmetric-poll change to be of order \(r^2\), hence change divided by step norm tends to zero from below. At a sharp nonsmooth maximum it can be of order \(-r\), so the normalized slope tends to a negative constant. A saddle or other nonstationary point has a positive first-order direction.

Among the 9 endpoints with no tested ascent, 8 have smallest-radius and largest-radius negative slopes within a factor of two; this is compatible with a sharp ridge or corner. 1 instead shrink by more than a factor of two toward zero; this is compatible with smoother second-order behavior. Three radii and a basis poll do not determine a convergence law. The per-state curves are in `endpoint-slope-by-state.png`.

## Incidence changes and numerical predicates

The source optimizer evaluation had no f64 geometry-predicate uncertainty at 16/16 endpoints. Incidence changes occurred only at the largest tested radius; all probes at the two smaller radii kept the base incidence signature. The 7 incidence-changing states and 7 states with tested ascent overlap in only 3 states, so the equal-looking counts do not describe one common failure set. This pattern is evidence for ordinary finite combinatorial wall crossings, not incidence flicker at numerical scale. It does not rule out a genuinely short or nearly redundant facet; testing facet deletion would be a different, dimension-changing perturbation.

Raw evidence: `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/heldout-f10-64-history-endpoints-19a8b4dfd`. The endpoint selection was fixed by the optimizer comparison before these poll outcomes were computed.

A same-start larger-compute follow-up and its repeated endpoint polls are reported in `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/history-f10-16-compute-depth-endpoints-426ec7a7c-analysis/REPORT.md`.
