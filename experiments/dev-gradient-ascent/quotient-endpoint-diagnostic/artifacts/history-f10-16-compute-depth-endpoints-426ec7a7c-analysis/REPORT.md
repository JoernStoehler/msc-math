# Optimizer endpoint diagnostic

The signed quotient-basis diagnostic tested 16 population-stratified `history-baseline` endpoints from an F=10 optimizer run. Each state was probed in both signs of all 25 quotient-basis axes at relative radii `0.001`, `0.0001`, `1e-05`. Every probe recomputed full `sys`.

**2/16 endpoints have an explicit improving basis direction, while 14/16 have no observed improvement at all three radii.** Of the latter, 9 also keep the same facet-incidence signature in every probe. This is a finite necessary-condition check, not a local-maximality proof.

| relative radius | positive observed | no positive observed | invalid | states with incidence changes | median best change | 10–90% | median best slope |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1e-03 | 0 | 16 | 0 | 6 | -0.000319257 | -0.000477922–-0.000138333 | -0.0857366 |
| 1e-04 | 0 | 16 | 0 | 0 | -3.18415e-05 | -4.54937e-05–-1.03957e-05 | -0.0855273 |
| 1e-05 | 2 | 14 | 0 | 0 | -1.93809e-06 | -4.18823e-06–-2.42005e-07 | -0.056787 |

A state fails this check whenever any tested direction has raw positive change. Invalid probes make the corresponding radius inconclusive. An incidence change is reported separately: it leaves the full-sys comparison valid, but means that the finite move crossed into a different combinatorial cell. `state-summary.csv` records every endpoint.

The diagnostic removes the tangent span of translations, scaling, and the identity-component linear symplectic action. Its signed orthonormal basis is positive spanning but not dense on the quotient sphere. Passing cannot exclude ascent between basis axes, below the smallest radius, or through branch behavior not resolved by finite probing. Failing does establish an explicit finite improving move under the current full evaluator.

## What the radius dependence says

The raw changes are small partly because the steps are small; the normalized slopes are the relevant scale. For a smooth strict local maximum one expects the best symmetric-poll change to be of order \(r^2\), hence change divided by step norm tends to zero from below. At a sharp nonsmooth maximum it can be of order \(-r\), so the normalized slope tends to a negative constant. A saddle or other nonstationary point has a positive first-order direction.

Among the 14 endpoints with no tested ascent, 12 have smallest-radius and largest-radius negative slopes within a factor of two; this is compatible with a sharp ridge or corner. 2 instead shrink by more than a factor of two toward zero; this is compatible with smoother second-order behavior. Three radii and a basis poll do not determine a convergence law. The per-state curves are in `endpoint-slope-by-state.png`.

## Incidence changes and numerical predicates

The source optimizer evaluation had no f64 geometry-predicate uncertainty at 16/16 endpoints. Incidence changes occurred only at the largest tested radius; all probes at the two smaller radii kept the base incidence signature. The 6 incidence-changing states and 2 states with tested ascent overlap in only 1 state, so the equal-looking counts do not describe one common failure set. This pattern is evidence for ordinary finite combinatorial wall crossings, not incidence flicker at numerical scale. It does not rule out a genuinely short or nearly redundant facet; testing facet deletion would be a different, dimension-changing perturbation.

## Same starts with a larger optimizer budget

On the matched 16-start panel, the larger-budget run changed 7/16 endpoint objective values and reduced explicit basis-poll failures from 7/16 to 2/16. It removed the observed ascent at 5 of the original 7 failing endpoints. Most runs stopped because the optimizer returned no proposals or reached its minimum internal distance, so the population curve plateaus rather than following one common logarithmic or power-law convergence curve.

| remaining endpoint | sys gain from larger budget | best tested one-step gain | current best slope |
|---|---:|---:|---:|
| random_F10_s0_34 | 0.00231607 | 9.37233e-07 | 0.0269251 |
| random_F10_s2_44 | 0 | 5.06132e-09 | 0.000136626 |

The first remaining positive slope is a real unresolved direction under this evaluator, although much smaller than before. The other is about four orders of magnitude smaller and changes `sys` by only about five billionths at the smallest tested radius; it should not be used as evidence of material non-convergence without a numerical-scale repeat. A slope alone gives no upper bound on the objective gap to an unknown local maximum.

Raw evidence: `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/history-f10-16-compute-depth-endpoints-426ec7a7c`. The endpoint selection was fixed by the optimizer comparison before these poll outcomes were computed.
