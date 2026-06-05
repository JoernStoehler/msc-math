# HKO Formula Sanity-Check Report

Status: sanity-check evidence only, not part of the proof.

Date: 2026-06-05.

Environment: SageMath version 10.7, Release Date: 2025-08-09.

Command:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/sanity_check_formulas.sage.py
```

Observed output:

```text
# HKO formula finite-difference sanity check

Status: sanity-check evidence only, not part of the proof.
Rows checked: 26 of 26
Full 40-dimensional directions checked: 3
Lagrangian-product-preserving directions checked: 3
Epsilon sweep: 1e-02, 1e-03, 1e-04, 1e-05, 1e-06, 1e-07
Full directions check beta/action derivatives.
Lagrangian-product-preserving directions check beta/action/sys/volume.
Volume is checked by central finite differences of the elementary
product-of-planar-areas formula, not by a general 4D volume backend.

## Full 40-dimensional beta/action checks

| eps | max beta abs | max beta rel | max action abs | max action rel |
| --- | ---: | ---: | ---: | ---: |
| 1e-02 | 2.011e-05 | 2.011e-05 | 5.632e-04 | 5.632e-04 |
| 1e-03 | 2.011e-07 | 2.011e-07 | 5.630e-06 | 5.630e-06 |
| 1e-04 | 2.011e-09 | 2.011e-09 | 5.629e-08 | 5.629e-08 |
| 1e-05 | 2.853e-11 | 2.853e-11 | 4.835e-10 | 4.835e-10 |
| 1e-06 | 1.101e-10 | 1.101e-10 | 2.227e-09 | 2.227e-09 |
| 1e-07 | 1.469e-09 | 1.469e-09 | 2.475e-08 | 2.475e-08 |

## Lagrangian-product beta/action/sys/volume checks

| eps | max beta abs | max beta rel | max action abs | max action rel | max sys abs | max sys rel | max volume abs | max volume rel |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1e-02 | 2.253e-05 | 2.253e-05 | 4.376e-04 | 4.376e-04 | 2.778e-04 | 2.778e-04 | 4.002e-05 | 1.128e-05 |
| 1e-03 | 2.253e-07 | 2.253e-07 | 4.374e-06 | 4.374e-06 | 2.777e-06 | 2.777e-06 | 4.002e-07 | 1.128e-07 |
| 1e-04 | 2.252e-09 | 2.252e-09 | 4.374e-08 | 4.374e-08 | 2.777e-08 | 2.777e-08 | 4.014e-09 | 1.131e-09 |
| 1e-05 | 2.210e-11 | 2.210e-11 | 5.374e-10 | 5.374e-10 | 3.549e-10 | 3.549e-10 | 1.076e-10 | 1.076e-10 |
| 1e-06 | 8.547e-11 | 8.547e-11 | 1.583e-09 | 1.583e-09 | 1.282e-09 | 1.282e-09 | 1.536e-09 | 1.536e-09 |
| 1e-07 | 8.899e-10 | 8.899e-10 | 1.250e-08 | 1.250e-08 | 6.873e-09 | 6.873e-09 | 2.869e-09 | 2.869e-09 |

Max |D sys row * symmetry tangent| over checked rows: 2.585e-16

Interpretation: for a correct first derivative, central-difference errors
should decrease as eps shrinks until floating-point noise dominates.
Large errors at all eps values are a formula/indexing bug signal.
```
