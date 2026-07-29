# Matched trajectory geometry

7 algorithms on 64 matched starts were compared at fixed measured-compute checkpoints.

At 1000 ms the closest median pair is `literal-eta1e-2` / `safeguarded-adaptive-d1e-1` (0.06058 initial norms); the farthest is `cma-s1e-1-l8` / `history-baseline` (0.3791).

## Endpoint comparisons involving branch history

| other algorithm | median ambient distance | 10–90% | median absolute sys difference | same winning word |
|---|---:|---:|---:|---:|
| directional-above-8e-2 | 0.2459 | 0.1025–0.5758 | 0.01192 | 6.2% |
| gap-w1e-1-adaptive-d1e-1 | 0.2588 | 0.1141–0.5626 | 0.01594 | 1.6% |
| safeguarded-adaptive-d1e-1 | 0.2986 | 0.2109–0.5693 | 0.1217 | 1.6% |
| literal-eta1e-2 | 0.2998 | 0.2116–0.5627 | 0.1028 | 1.6% |
| pattern-r3e-2 | 0.3717 | 0.2587–0.5956 | 0.4404 | 0.0% |
| cma-s1e-1-l8 | 0.3791 | 0.2828–0.5731 | 0.1678 | 1.6% |

## Movement from the matched start

| algorithm | median movement / initial norm | 10–90% |
|---|---:|---:|
| history-baseline | 0.3745 | 0.2656–0.601 |
| directional-above-8e-2 | 0.3129 | 0.2172–0.4093 |
| gap-w1e-1-adaptive-d1e-1 | 0.268 | 0.1796–0.3653 |
| cma-s1e-1-l8 | 0.2207 | 0.1537–0.3023 |
| safeguarded-adaptive-d1e-1 | 0.1515 | 0.07895–0.2682 |
| literal-eta1e-2 | 0.1446 | 0.08755–0.2019 |
| pattern-r3e-2 | 0.06702 | 0.03–0.1601 |

## Distance to the recorded endpoint at 500 ms

| algorithm | median coordinate distance / initial norm | median later sys gain |
|---|---:|---:|
| safeguarded-adaptive-d1e-1 | 7.975e-05 | 0.0002363 |
| history-baseline | 0.004097 | 0.001573 |
| directional-above-8e-2 | 0.007943 | 0.003609 |
| gap-w1e-1-adaptive-d1e-1 | 0.01007 | 0.005105 |
| literal-eta1e-2 | 0.04396 | 0.0498 |
| pattern-r3e-2 | 0.06003 | 0.08565 |
| cma-s1e-1-l8 | 0.1459 | 0.07601 |

The endpoint here is only the best state recorded by the end of this run, not a certified local maximum. A small value therefore diagnoses an early plateau relative to the method's own one-second result; it does not show successful optimization. `movement-vs-compute.png` gives the full curves.

## Dimension of the matched trajectory cloud

For each start, all nonzero-compute checkpoints were projected away from the 15-dimensional symmetry tangent at that start. Repeated identical checkpoints after an optimizer stopped were counted once. Principal components then summarized the resulting point cloud in the 25-dimensional linearized quotient slice.

| quantity | median | 10–90% across starts |
|---|---:|---:|
| unique recorded points | 30 | 27.3–32.7 |
| observed linear rank | 25 | 25–25 |
| components for 90% | 3 | 2–4 |
| components for 95% | 4 | 3–5 |
| components for 99% | 8.5 | 7–10 |
| fraction in first component | 0.7552 | 0.6529–0.8466 |

The branch-aware methods reach similarly high objective values without following one common coordinate path: the four-anchor branch-history method is separated from the directional and gap variants by about one quarter of an initial-state norm at the median. The two single-branch gradient variants remain much closer to each other. This is separation, not a claim that every individual pair immediately diverges or that the paths reach distinct basins.

These are ambient coordinate distances with matched facet labels. They show whether recorded paths separate, but they do not quotient the continuous symmetry group and therefore cannot establish distinct local maxima. The PCA uses only the tangent space at the start, so it is a local linear removal of symmetry directions rather than a global alignment of paths.
