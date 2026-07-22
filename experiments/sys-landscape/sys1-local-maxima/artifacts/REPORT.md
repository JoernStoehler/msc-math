# Known `sys = 1` Local Screen Report

## Control gate

PASS: the rotated-pentagon structured-family control has an interval-separated improvement at every tested angular radius.

## Base reconstruction

| Seed | Role | facets | quotient dim | recomputed sys | capacity interval width |
| --- | --- | ---: | ---: | ---: | ---: |
| `pentagon_threshold_control` | expected_positive_control | 10 | 25 | 1 | 0 |
| `triangle_hexagon_theta0` | target | 9 | 21 | 1 | 1.98814345 |
| `square_square_pi_over_4` | target | 8 | 17 | 1 | 2.82842712 |
| `ch2021_six_vertex` | target | 9 | 21 | 1 | 0.499983104 |

## Direct outcomes

| Seed | Finite-screen status | probes | material nominal | interval-separated | incidence changes | best delta sys | best family |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `pentagon_threshold_control` | `empirical_improving_pattern_at_all_tested_scales` | 396 | 12 | 12 | 0 | 0.00991520575 | `so4_mod_u2_orientation` |
| `triangle_hexagon_theta0` | `no_improvement_observed` | 372 | 0 | 0 | 0 | -1.38889122e-10 | `quotient_basis` |
| `square_square_pi_over_4` | `no_improvement_observed` | 348 | 0 | 0 | 0 | 5.55111512e-16 | `quotient_basis` |
| `ch2021_six_vertex` | `no_improvement_observed` | 366 | 0 | 0 | 366 | -1.10991763e-05 | `quotient_basis` |

## Interpretation of this run

- The control was recovered in both theory-derived low-dimensional slices: relative rotation had 3 interval-separated improving probes across the three angular radii, and the `SO(4)/U(2)` circle had 9. The signed quotient basis and 64 deterministic random quotient directions per radius found 0 material improvements. Thus the control validates the evaluator and structured directions, while also demonstrating that a sparse high-dimensional poll can miss a real improving cone.
- The three targets had 0 material nominal improvements across all probes. Triangle--hexagon's best basis changes were negative and approximately quadratic in the row radius; square--square's positive raw changes were at most binary64-scale noise; every CH probe decreased the nominal scalar.
- The target base capacity intervals are broad (widths 1.98814345, 2.82842712, 0.499983104), so this run supplies no interval-separated target conclusion. Target results are finite nominal-scalar diagnostics and should motivate branch-aware exact work, not a local-maximality claim.
- All 366 CH probes were valid fixed-nine-facet bodies and entered adjacent combinatorial cells. This is expected from a nonsimple base and gives neighborhood evidence across those cells, but it prevents interpreting the run as a single smooth fixed-incidence calculation.
- The frozen probability bullets in the README were not mutually exclusive: interval ambiguity could coexist with either presence or absence of a target improvement. They therefore record pre-run expectations but do not define a scoreable partition. The observed run has no target improvement and substantial interval ambiguity.

## Radius-level observations

| Seed | perturbation | radius | valid/total | material nominal | interval-separated | max delta sys |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `ch2021_six_vertex` | `quotient_basis` | 1e-03 | 42/42 | 0 | 0 | -0.00110687965 |
| `ch2021_six_vertex` | `quotient_basis` | 1e-04 | 42/42 | 0 | 0 | -0.000110964015 |
| `ch2021_six_vertex` | `quotient_basis` | 1e-05 | 42/42 | 0 | 0 | -1.10991763e-05 |
| `ch2021_six_vertex` | `quotient_random_antipodal` | 1e-03 | 64/64 | 0 | 0 | -0.00208424333 |
| `ch2021_six_vertex` | `quotient_random_antipodal` | 1e-04 | 64/64 | 0 | 0 | -0.000209020236 |
| `ch2021_six_vertex` | `quotient_random_antipodal` | 1e-05 | 64/64 | 0 | 0 | -2.09080135e-05 |
| `ch2021_six_vertex` | `so4_mod_u2_orientation` | 1e-02 | 16/16 | 0 | 0 | -0.0165899291 |
| `ch2021_six_vertex` | `so4_mod_u2_orientation` | 1e-03 | 16/16 | 0 | 0 | -0.00166836562 |
| `ch2021_six_vertex` | `so4_mod_u2_orientation` | 1e-04 | 16/16 | 0 | 0 | -0.000166924511 |
| `pentagon_threshold_control` | `product_relative_rotation` | 1e-02 | 2/2 | 1 | 1 | 0.00483876979 |
| `pentagon_threshold_control` | `product_relative_rotation` | 1e-03 | 2/2 | 1 | 1 | 0.000473303822 |
| `pentagon_threshold_control` | `product_relative_rotation` | 1e-04 | 2/2 | 1 | 1 | 4.7225268e-05 |
| `pentagon_threshold_control` | `quotient_basis` | 1e-03 | 50/50 | 0 | 0 | -0.000513096429 |
| `pentagon_threshold_control` | `quotient_basis` | 1e-04 | 50/50 | 0 | 0 | -5.13387634e-05 |
| `pentagon_threshold_control` | `quotient_basis` | 1e-05 | 50/50 | 0 | 0 | -5.13416766e-06 |
| `pentagon_threshold_control` | `quotient_random_antipodal` | 1e-03 | 64/64 | 0 | 0 | -0.000677193768 |
| `pentagon_threshold_control` | `quotient_random_antipodal` | 1e-04 | 64/64 | 0 | 0 | -6.7645672e-05 |
| `pentagon_threshold_control` | `quotient_random_antipodal` | 1e-05 | 64/64 | 0 | 0 | -6.7638307e-06 |
| `pentagon_threshold_control` | `so4_mod_u2_orientation` | 1e-02 | 16/16 | 3 | 3 | 0.00991520575 |
| `pentagon_threshold_control` | `so4_mod_u2_orientation` | 1e-03 | 16/16 | 3 | 3 | 0.00094894612 |
| `pentagon_threshold_control` | `so4_mod_u2_orientation` | 1e-04 | 16/16 | 3 | 3 | 9.44738838e-05 |
| `square_square_pi_over_4` | `product_relative_rotation` | 1e-02 | 2/2 | 0 | 0 | -0.0196065616 |
| `square_square_pi_over_4` | `product_relative_rotation` | 1e-03 | 2/2 | 0 | 0 | -0.00199600666 |
| `square_square_pi_over_4` | `product_relative_rotation` | 1e-04 | 2/2 | 0 | 0 | -0.000199960007 |
| `square_square_pi_over_4` | `quotient_basis` | 1e-03 | 34/34 | 0 | 0 | 5.55111512e-16 |
| `square_square_pi_over_4` | `quotient_basis` | 1e-04 | 34/34 | 0 | 0 | 5.55111512e-16 |
| `square_square_pi_over_4` | `quotient_basis` | 1e-05 | 34/34 | 0 | 0 | 3.33066907e-16 |
| `square_square_pi_over_4` | `quotient_random_antipodal` | 1e-03 | 64/64 | 0 | 0 | -0.000674433719 |
| `square_square_pi_over_4` | `quotient_random_antipodal` | 1e-04 | 64/64 | 0 | 0 | -6.74634849e-05 |
| `square_square_pi_over_4` | `quotient_random_antipodal` | 1e-05 | 64/64 | 0 | 0 | -6.74654927e-06 |
| `square_square_pi_over_4` | `so4_mod_u2_orientation` | 1e-02 | 16/16 | 0 | 0 | -0.0136995406 |
| `square_square_pi_over_4` | `so4_mod_u2_orientation` | 1e-03 | 16/16 | 0 | 0 | -0.00140972109 |
| `square_square_pi_over_4` | `so4_mod_u2_orientation` | 1e-04 | 16/16 | 0 | 0 | -0.000141376364 |
| `triangle_hexagon_theta0` | `product_relative_rotation` | 1e-02 | 2/2 | 0 | 0 | -0.0113492862 |
| `triangle_hexagon_theta0` | `product_relative_rotation` | 1e-03 | 2/2 | 0 | 0 | -0.00115270284 |
| `triangle_hexagon_theta0` | `product_relative_rotation` | 1e-04 | 2/2 | 0 | 0 | -0.000115450056 |
| `triangle_hexagon_theta0` | `quotient_basis` | 1e-03 | 42/42 | 0 | 0 | -1.38888696e-06 |
| `triangle_hexagon_theta0` | `quotient_basis` | 1e-04 | 42/42 | 0 | 0 | -1.38888884e-08 |
| `triangle_hexagon_theta0` | `quotient_basis` | 1e-05 | 42/42 | 0 | 0 | -1.38889122e-10 |
| `triangle_hexagon_theta0` | `quotient_random_antipodal` | 1e-03 | 64/64 | 0 | 0 | -0.00118604452 |
| `triangle_hexagon_theta0` | `quotient_random_antipodal` | 1e-04 | 64/64 | 0 | 0 | -0.000118737952 |
| `triangle_hexagon_theta0` | `quotient_random_antipodal` | 1e-05 | 64/64 | 0 | 0 | -1.18751309e-05 |
| `triangle_hexagon_theta0` | `so4_mod_u2_orientation` | 1e-02 | 16/16 | 0 | 0 | -0.0195104969 |
| `triangle_hexagon_theta0` | `so4_mod_u2_orientation` | 1e-03 | 16/16 | 0 | 0 | -0.00199501065 |
| `triangle_hexagon_theta0` | `so4_mod_u2_orientation` | 1e-04 | 16/16 | 0 | 0 | -0.000199950011 |

## Interpretation boundary

The pentagon row is a calibration result, not a new discovery. For a target, interval-separated improvements at all tested shrinking radii identify a concrete path for exact branch analysis. A finite miss remains only `no improvement observed` within these fixed-facet directions and radii. The nonsimple CH probes enter adjacent combinatorial cells; those are valid nearby fixed-facet bodies, not rejected samples. A miss does not establish local maximality, exclude a narrow improving cone, control right-active singular branches, or test facet-count changes.
