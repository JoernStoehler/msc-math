# HKO Empirical Support Experiments

This directory contains supporting evidence around HKO2024. These experiments
can falsify or sanity-check the local-maximum picture, but they are not the
final theorem certificate.

## Subdirectories

| Path | Role |
| --- | --- |
| `first-order/` | First-order numerical support and active-gradient/orbit bookkeeping. |
| `second-order/` | Flat-direction curvature evidence in the fixed `F=10` setting. |
| `neighborhood-sampling/` | Random nearby-polytope samplers for fixed-`F=10`, `F=11` facet-splitting, and fixed-`F=10` Lagrangian-product modes. |
| `m11-ascent/` | Cut-then-ascent checks starting from `F=11` facet additions. |

## Interpretation

The empirical checks are deliberately separate from `theorem/`. They are useful
for detecting mistakes in the proof methodology and for writing honest context,
but thesis theorem wording must rely on the exact witness/proof route.
