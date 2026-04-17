Regenerate with `bash scripts/dataflow.sh`.

# Experiment Artifact Dataflow Audit

- Generated at: `2026-04-17T17:06:12Z`
- Scope: declared `Input Artifacts:` / `Output Artifacts:` on Cargo binary entrypoints and experiment Python scripts
- Artifact suffixes: `.jsonl, .png, .csv`

## Summary

- Sources scanned: `68`
- Artifact files seen or declared: `143`
- Missing declaration fields: `0`
- Tracked artifacts without a producer: `10`
- Tracked artifacts without a consumer: `98`
- Artifacts with multiple producers: `1`
- Artifacts with freshness-risk flags: `32`
- Unmatched declared patterns: `7`

## Tracked Artifacts Without Producer

- `experiments/numerics/error-bounds/testdata/eigendirection_scaling.jsonl`
- `experiments/numerics/error-bounds/testdata/eta_bound_validity.jsonl`
- `experiments/verification/orbit-recovery/cache-extension.jsonl`
- `experiments/visualization/main/viz-hko-pentagon-edges.png`
- `experiments/visualization/main/viz-hko-pentagon-traj.png`
- `experiments/visualization/main/viz-hypercube-edges.png`
- `experiments/visualization/main/viz-hypercube-ridges.png`
- `experiments/visualization/main/viz-hypercube-traj.png`
- `experiments/visualization/main/viz-lagrangian-tri-product-traj.png`
- `experiments/visualization/main/viz-simplex-traj.png`

## Tracked Artifacts Without Consumer

- `experiments/combinatorial-cells/boundary-characterization/boundary_density_cdf.png`
- `experiments/combinatorial-cells/boundary-characterization/boundary_event_types.png`
- `experiments/combinatorial-cells/boundary-characterization/boundary_sys_continuity.png`
- `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_by_direction.png`
- `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_distribution.png`
- `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_vs_F.png`
- `experiments/combinatorial-cells/boundary-characterization/orbit_gap_distribution.png`
- `experiments/combinatorial-cells/boundary-characterization/orbit_gap_vs_switch.png`
- `experiments/combinatorial-cells/cell-widths/cell_anisotropy.png`
- `experiments/combinatorial-cells/cell-widths/cell_orbit_vs_nonorbit.png`
- `experiments/combinatorial-cells/cell-widths/cell_width_by_F.png`
- `experiments/combinatorial-cells/cell-widths/profiling_event_types.png`
- `experiments/combinatorial-cells/convexity/cell_convexity.png`
- `experiments/combinatorial-cells/gradient-discontinuity/boundary_gradient_angle.png`
- `experiments/combinatorial-cells/gradient-discontinuity/gradient_cell_alignment.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_dots.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_neighbor_split.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_omega_vs_dot.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_mean_vs_sys.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_min_vs_sys.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_vs_nonorbit.png`
- `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_ridge_min_vs_sys.png`
- `experiments/crosspolytope/main/crosspolytope.jsonl`
- `experiments/hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl`
- `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.png`
- `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-gradient.png`
- `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-orbits.png`
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-probe.jsonl`
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_probe_radii.png`
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_fraction.png`
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_sys_vs_eps.png`
- `experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png`
- `experiments/hko-local-maximum/second-order/second-order-random.jsonl`
- `experiments/hko-local-maximum/second-order/second_order_curvatures.png`
- `experiments/hko-local-maximum/second-order/second_order_curves.png`
- `experiments/hko-local-maximum/second-order/second_order_random_hist.png`
- `experiments/numerics/error-bounds/testdata/eigendirection_scaling.jsonl`
- `experiments/numerics/error-bounds/testdata/eta_bound_validity.jsonl`
- `experiments/numerics/gradient/numerics-edge-cases/gc_q3_gap.png`
- `experiments/numerics/gradient/numerics-edge-cases/gc_q4_delta.png`
- `experiments/numerics/gradient/numerics-subdifferential/gc_q5_convergence.png`
- `experiments/numerics/gradient/numerics-subdifferential/gc_q5_switching.png`
- `experiments/numerics/gradient/numerics-subdifferential/gc_q5b_boundary.png`
- `experiments/numerics/gradient/numerics/gc_convergence.png`
- `experiments/numerics/gradient/numerics/gc_slopes.png`
- `experiments/numerics/unknown-predicates/unknown_predicates_beta_min.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_convergence.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_distribution.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_escape.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_improvement.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_strategy.png`
- `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_tail.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products-trace.jsonl`
- `experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_convergence.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_distribution.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_escape.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_improvement.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_strategy.png`
- `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_tail.png`
- `experiments/sys-landscape/random-product-sample/random_product_sweep_sys_vs_pair.png`
- `experiments/sys-landscape/random-sample/random_sweep_sys_vs_f.png`
- `experiments/sys-landscape/rejection-calibration/acceptance.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x3-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x4-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x5-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x5-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x6-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x6-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-6x6-6deg.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-7x7.jsonl`
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_5x5.png`
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_7x7.png`
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_polygon_pairs.png`
- `experiments/sys-landscape/variable-f-ascent/cache.jsonl`
- `experiments/sys-landscape/variable-f-ascent/variable-f-rq1.png`
- `experiments/sys-landscape/variable-f-ascent/variable-f-rq2.png`
- `experiments/verification/algorithm-comparison/ablation/ablation_timing.png`
- `experiments/verification/algorithm-comparison/benchmark/benchmark_timing.png`
- `experiments/verification/algorithm-comparison/benchmark/profiling/micro_benchmarks.png`
- `experiments/verification/algorithm-comparison/benchmark/profiling/phase_breakdown.png`
- `experiments/verification/algorithm-comparison/profiling/logbook.jsonl`
- `experiments/verification/algorithm-comparison/profiling/profile.jsonl`
- `experiments/verification/algorithm-comparison/profiling/test_timing.png`
- `experiments/verification/correctness/correctness.jsonl`
- `experiments/verification/orbit-recovery/cache-extension.jsonl`
- `experiments/verification/orbit-recovery/orbit_recovery_errors.png`
- `experiments/visualization/main/viz-hko-pentagon-edges.png`
- `experiments/visualization/main/viz-hko-pentagon-traj.png`
- `experiments/visualization/main/viz-hypercube-edges.png`
- `experiments/visualization/main/viz-hypercube-ridges.png`
- `experiments/visualization/main/viz-hypercube-traj.png`
- `experiments/visualization/main/viz-lagrangian-tri-product-traj.png`
- `experiments/visualization/main/viz-simplex-traj.png`

## Artifacts With Multiple Producers

- `experiments/sys-landscape/cache.jsonl`: `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)`<br>`experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)`<br>`experiments/sys-landscape/random-product-sample/main.rs (cargo bin sys-random-product-sample)`<br>`experiments/sys-landscape/random-sample/main.rs (cargo bin sys-random-sample)`

## Freshness Risks

- `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-gradient.png`: input-git-newer
- `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-orbits.png`: input-git-newer
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-probe.jsonl`: source-git-newer
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_probe_radii.png`: source-git-newer, input-git-newer
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_fraction.png`: source-git-newer, input-git-newer
- `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_sys_vs_eps.png`: source-git-newer, input-git-newer
- `experiments/hko-local-maximum/second-order/second_order_curvatures.png`: source-git-newer, input-git-newer
- `experiments/hko-local-maximum/second-order/second_order_curves.png`: source-git-newer, input-git-newer
- `experiments/hko-local-maximum/second-order/second_order_random_hist.png`: source-git-newer, input-git-newer
- `experiments/numerics/error-bounds/collected_poly.jsonl`: missing
- `experiments/numerics/gradient/numerics-edge-cases/gc_q3_gap.png`: source-git-newer, input-git-newer
- `experiments/numerics/gradient/numerics-edge-cases/gc_q4_delta.png`: source-git-newer, input-git-newer
- `experiments/numerics/gradient/numerics-subdifferential/gc_q5_convergence.png`: source-git-newer, input-git-newer
- `experiments/numerics/gradient/numerics-subdifferential/gc_q5_switching.png`: source-git-newer, input-git-newer
- `experiments/numerics/gradient/numerics/gc_convergence.png`: source-git-newer, input-git-newer
- `experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl`: source-git-newer, input-git-newer
- `experiments/sys-landscape/random-product-sample/random_product_sweep_sys_vs_pair.png`: source-git-newer
- `experiments/sys-landscape/random-sample/random-sweep.jsonl`: source-git-newer, input-git-newer
- `experiments/sys-landscape/random-sample/random_sweep_sys_vs_f.png`: source-git-newer
- `experiments/sys-landscape/rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl`: missing
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_5x5.png`: source-git-newer, input-git-newer
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_7x7.png`: source-git-newer, input-git-newer
- `experiments/sys-landscape/rotated-regular-products/lagrangian_products_polygon_pairs.png`: source-git-newer, input-git-newer
- `experiments/verification/algorithm-comparison/benchmark/profiling/micro_benchmarks.png`: source-git-newer
- `experiments/verification/algorithm-comparison/benchmark/profiling/phase_breakdown.png`: source-git-newer
- `experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl`: missing
- `experiments/verification/all-minimum/smoke-all-minimum.jsonl`: missing
- `experiments/verification/correctness/correctness.jsonl`: source-git-newer
- `experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl`: source-git-newer
- `experiments/verification/orbit-recovery/orbit-recovery.jsonl`: source-git-newer
- `experiments/verification/orbit-recovery/smoke-orbit-recovery-orbits.jsonl`: missing
- `experiments/verification/orbit-recovery/smoke-orbit-recovery.jsonl`: missing

## Unmatched Declared Patterns

- `experiments/combinatorial-cells/multiple-crossings/analyze.py` `Output Artifacts` -> `experiments/combinatorial-cells/multiple-crossings/*.png`
- `experiments/hko-local-maximum/perturbation-neighborhood/analyze.py` `Input Artifacts` -> `experiments/hko-local-maximum/perturbation-neighborhood/data/licca-eps-*.jsonl`
- `experiments/numerics/error-bounds/analyze.py` `Input Artifacts` -> `experiments/numerics/error-bounds/results_*.jsonl`
- `experiments/sys-landscape/gradient-ascent-general/analyze.py` `Input Artifacts` -> `experiments/sys-landscape/gradient-ascent-general/data/*.jsonl`
- `experiments/sys-landscape/gradient-ascent-general/analyze.py` `Input Artifacts` -> `experiments/sys-landscape/gradient-ascent-general/licca-shard-*.jsonl`
- `experiments/sys-landscape/gradient-ascent-products/analyze.py` `Input Artifacts` -> `experiments/sys-landscape/gradient-ascent-products/data/*.jsonl`
- `experiments/sys-landscape/gradient-ascent-products/analyze.py` `Input Artifacts` -> `experiments/sys-landscape/gradient-ascent-products/licca-shard-*.jsonl`

## Artifacts

| Artifact | Present | Tracked | Ignored | Mtime | Git Last Change | Producer | Consumers | Freshness |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `experiments/combinatorial-cells/boundary-characterization/boundary_density_cdf.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/boundary_event_types.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/boundary_sys_continuity.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_by_direction.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_distribution.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/boundary_tmax_vs_F.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-anatomy.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/main.rs (cargo bin cell-boundary-characterization)` | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)`<br>`experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-crossing.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/main.rs (cargo bin cell-boundary-characterization)` | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-gradient.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/main.rs (cargo bin cell-boundary-characterization)` | `experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/boundary-characterization/orbit_gap_distribution.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/boundary-characterization/orbit_gap_vs_switch.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/cell-widths/cell_anisotropy.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/cell-widths/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/cell-widths/cell_orbit_vs_nonorbit.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/cell-widths/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/cell-widths/cell_width_by_F.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/cell-widths/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/cell-widths/main.rs (cargo bin cell-widths)` | `experiments/combinatorial-cells/cell-widths/analyze.py (python)`<br>`experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/cell-widths/profiling_event_types.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/cell-widths/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/convexity/cell_convexity.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/convexity/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/convexity/combinatorial-boundaries-convexity.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/convexity/main.rs (cargo bin cell-convexity)` | `experiments/combinatorial-cells/convexity/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/gradient-discontinuity/boundary_gradient_angle.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/gradient-discontinuity/gradient_cell_alignment.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/multiple-crossings/main.rs (cargo bin cell-multiple-crossings)` | `experiments/combinatorial-cells/multiple-crossings/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/main.rs (cargo bin cell-omega)` | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_dots.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_neighbor_split.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_omega_vs_dot.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_mean_vs_sys.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_min_vs_sys.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_vs_nonorbit.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_ridge_min_vs_sys.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | - | ok |
| `experiments/combinatorial-cells/polytopes.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/combinatorial-cells/omega-hypothesis/main.rs (cargo bin cell-omega)` | `experiments/combinatorial-cells/boundary-characterization/main.rs (cargo bin cell-boundary-characterization)`<br>`experiments/combinatorial-cells/cell-widths/main.rs (cargo bin cell-widths)`<br>`experiments/combinatorial-cells/convexity/main.rs (cargo bin cell-convexity)`<br>`experiments/combinatorial-cells/multiple-crossings/main.rs (cargo bin cell-multiple-crossings)` | ok |
| `experiments/crosspolytope/main/crosspolytope.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/crosspolytope/main/main.rs (cargo bin crosspolytope)` | - | ok |
| `experiments/hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/cut-and-ascent/main.rs (cargo bin hko-cut-and-ascent)` | - | ok |
| `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/facet-splitting/main.rs (cargo bin hko-facet-splitting)` | `experiments/hko-local-maximum/facet-splitting/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/facet-splitting/analyze.py (python)` | - | ok |
| `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-ascent.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/gradient-analysis/main.rs (cargo bin hko-gradient-analysis)` | `experiments/hko-local-maximum/gradient-analysis/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-gradient.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/gradient-analysis/analyze.py (python)` | - | input-git-newer |
| `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-orbits.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/gradient-analysis/analyze.py (python)` | - | input-git-newer |
| `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/gradient-analysis/main.rs (cargo bin hko-gradient-analysis)` | `experiments/hko-local-maximum/gradient-analysis/analyze.py (python)`<br>`experiments/hko-local-maximum/subdifferential-lp/phase_c_lp_test.py (python)` | ok |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-probe.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/lagrangian-boundary/probe.rs (cargo bin hko-lagrangian-probe)` | - | source-git-newer |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search-levels.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/lagrangian-boundary/main.rs (cargo bin hko-lagrangian-boundary)` | `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/lagrangian-boundary/main.rs (cargo bin hko-lagrangian-boundary)` | `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_probe_radii.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_fraction.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_sys_vs_eps.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/hko-local-maximum/perturbation-neighborhood/pentagon-perturb.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/perturbation-neighborhood/main.rs (cargo bin hko-perturbation)` | `experiments/hko-local-maximum/perturbation-neighborhood/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/perturbation-neighborhood/analyze.py (python)` | - | ok |
| `experiments/hko-local-maximum/second-order/second-order-base.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/second-order/main.rs (cargo bin hko-second-order)` | `experiments/hko-local-maximum/second-order/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/second-order/second-order-curves.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/second-order/main.rs (cargo bin hko-second-order)` | `experiments/hko-local-maximum/second-order/analyze.py (python)` | ok |
| `experiments/hko-local-maximum/second-order/second-order-random.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/hko-local-maximum/second-order/main.rs (cargo bin hko-second-order)` | - | ok |
| `experiments/hko-local-maximum/second-order/second_order_curvatures.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/second-order/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/hko-local-maximum/second-order/second_order_curves.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/second-order/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/hko-local-maximum/second-order/second_order_random_hist.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/hko-local-maximum/second-order/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/error-bounds/collected_poly.jsonl` | no | no | yes | - | - | `experiments/numerics/error-bounds/collect_poly.rs (cargo bin num-collect-poly)` | - | missing |
| `experiments/numerics/error-bounds/testdata/eigendirection_scaling.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/numerics/error-bounds/testdata/eta_bound_validity.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/numerics/gradient/numerics-edge-cases/gc_q3_gap.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/numerics/gradient/numerics-edge-cases/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/gradient/numerics-edge-cases/gc_q4_delta.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/numerics/gradient/numerics-edge-cases/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q3-degeneracy.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics-edge-cases/main.rs (cargo bin dev_numerics_edge_cases)` | `experiments/numerics/gradient/numerics-edge-cases/analyze.py (python)` | ok |
| `experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q4-redundant.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics-edge-cases/main.rs (cargo bin dev_numerics_edge_cases)` | `experiments/numerics/gradient/numerics-edge-cases/analyze.py (python)` | ok |
| `experiments/numerics/gradient/numerics-subdifferential/gc_q5_convergence.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/gradient/numerics-subdifferential/gc_q5_switching.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/gradient/numerics-subdifferential/gc_q5b_boundary.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | - | ok |
| `experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5-subdiff.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics-subdifferential/main.rs (cargo bin dev_numerics_subdifferential)` | `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | ok |
| `experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5b-symmetric.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics-subdifferential/main.rs (cargo bin dev_numerics_subdifferential)` | `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | ok |
| `experiments/numerics/gradient/numerics/gc_convergence.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/numerics/gradient/numerics/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/numerics/gradient/numerics/gc_slopes.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics/analyze.py (python)` | - | ok |
| `experiments/numerics/gradient/numerics/gradient-correctness-q1-generic.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics/main.rs (cargo bin dev_numerics)` | `experiments/numerics/gradient/numerics/analyze.py (python)` | ok |
| `experiments/numerics/gradient/numerics/gradient-correctness-q2-nongeneric.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/gradient/numerics/main.rs (cargo bin dev_numerics)` | `experiments/numerics/gradient/numerics/analyze.py (python)` | ok |
| `experiments/numerics/unknown-predicates/unknown-predicates.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/unknown-predicates/main.rs (cargo bin num-unknown-predicates)` | `experiments/numerics/unknown-predicates/analyze.py (python)` | ok |
| `experiments/numerics/unknown-predicates/unknown_predicates_beta_min.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/numerics/unknown-predicates/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/cache.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)`<br>`experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)`<br>`experiments/sys-landscape/random-product-sample/main.rs (cargo bin sys-random-product-sample)`<br>`experiments/sys-landscape/random-sample/main.rs (cargo bin sys-random-sample)` | `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)`<br>`experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)`<br>`experiments/sys-landscape/random-product-sample/main.rs (cargo bin sys-random-product-sample)`<br>`experiments/sys-landscape/random-sample/main.rs (cargo bin sys-random-sample)` | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)` | `experiments/sys-landscape/variable-f-ascent/main.rs (cargo bin sys-variable-f-ascent)` | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_convergence.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_distribution.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_escape.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_improvement.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_strategy.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_tail.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-general/licca.jsonl` | no | no | no | - | - | - | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | ok |
| `experiments/sys-landscape/gradient-ascent-general/smoke.jsonl` | no | no | yes | - | - | - | `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products-trace.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_convergence.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_distribution.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_escape.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_improvement.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_strategy.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_tail.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/gradient-ascent-products/licca.jsonl` | no | no | no | - | - | - | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | ok |
| `experiments/sys-landscape/gradient-ascent-products/smoke.jsonl` | no | no | yes | - | - | - | `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | ok |
| `experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/random-product-sample/main.rs (cargo bin sys-random-product-sample)` | `experiments/sys-landscape/random-product-sample/analyze.py (python)` | source-git-newer, input-git-newer |
| `experiments/sys-landscape/random-product-sample/random_product_sweep_sys_vs_pair.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/random-product-sample/analyze.py (python)` | - | source-git-newer |
| `experiments/sys-landscape/random-sample/random-sweep.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/random-sample/main.rs (cargo bin sys-random-sample)` | `experiments/sys-landscape/random-sample/analyze.py (python)` | source-git-newer, input-git-newer |
| `experiments/sys-landscape/random-sample/random_sweep_sys_vs_f.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/random-sample/analyze.py (python)` | - | source-git-newer |
| `experiments/sys-landscape/rejection-calibration/acceptance.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rejection-calibration/main.rs (cargo bin sys-rejection-calibration)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x3-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x4-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x5-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x5-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x6-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x6-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-6x6-6deg.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-7x7.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | - | ok |
| `experiments/sys-landscape/rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl` | no | no | no | - | - | `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | missing |
| `experiments/sys-landscape/rotated-regular-products/lagrangian_products_5x5.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/sys-landscape/rotated-regular-products/lagrangian_products_7x7.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/sys-landscape/rotated-regular-products/lagrangian_products_polygon_pairs.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | - | source-git-newer, input-git-newer |
| `experiments/sys-landscape/variable-f-ascent/cache.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/variable-f-ascent/main.rs (cargo bin sys-variable-f-ascent)` | - | ok |
| `experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/variable-f-ascent/main.rs (cargo bin sys-variable-f-ascent)` | `experiments/sys-landscape/variable-f-ascent/analyze.py (python)` | ok |
| `experiments/sys-landscape/variable-f-ascent/variable-f-rq1.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/variable-f-ascent/analyze.py (python)` | - | ok |
| `experiments/sys-landscape/variable-f-ascent/variable-f-rq2.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/sys-landscape/variable-f-ascent/analyze.py (python)` | - | ok |
| `experiments/verification/algorithm-comparison/ablation/ablation.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/ablation/main.rs (cargo bin cmp-ablation)` | `experiments/verification/algorithm-comparison/ablation/analyze.py (python)` | ok |
| `experiments/verification/algorithm-comparison/ablation/ablation_timing.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/ablation/analyze.py (python)` | - | ok |
| `experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/benchmark/main.rs (cargo bin cmp-benchmark)` | `experiments/verification/algorithm-comparison/benchmark/analyze.py (python)` | ok |
| `experiments/verification/algorithm-comparison/benchmark/benchmark_timing.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/benchmark/analyze.py (python)` | - | ok |
| `experiments/verification/algorithm-comparison/benchmark/profiling/micro_benchmarks.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/verification/algorithm-comparison/benchmark/profiling/analyze_profiling.py (python)` | - | source-git-newer |
| `experiments/verification/algorithm-comparison/benchmark/profiling/phase_breakdown.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/verification/algorithm-comparison/benchmark/profiling/analyze_profiling.py (python)` | - | source-git-newer |
| `experiments/verification/algorithm-comparison/profiling/logbook.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/profiling/analyze.py (python)` | - | ok |
| `experiments/verification/algorithm-comparison/profiling/profile.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/profiling/analyze.py (python)` | - | ok |
| `experiments/verification/algorithm-comparison/profiling/test_timing.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | `experiments/verification/algorithm-comparison/profiling/analyze.py (python)` | - | ok |
| `experiments/verification/all-minimum/all-minimum-orbits.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T12:44:58Z | `experiments/verification/all-minimum/main.rs (cargo bin axioms-all-minimum)` | `experiments/verification/all-minimum/analyze.py (python)`<br>`experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | ok |
| `experiments/verification/all-minimum/all-minimum.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T12:44:58Z | `experiments/verification/all-minimum/main.rs (cargo bin axioms-all-minimum)` | `experiments/verification/all-minimum/analyze.py (python)` | ok |
| `experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl` | no | no | yes | - | - | `experiments/verification/all-minimum/main.rs (cargo bin axioms-all-minimum)` | `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | missing |
| `experiments/verification/all-minimum/smoke-all-minimum.jsonl` | no | no | yes | - | - | `experiments/verification/all-minimum/main.rs (cargo bin axioms-all-minimum)` | - | missing |
| `experiments/verification/correctness/correctness.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | `experiments/verification/correctness/main.rs (cargo bin axioms-correctness)` | - | source-git-newer |
| `experiments/verification/orbit-recovery/cache-extension.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T16:03:17Z | - | - | ok |
| `experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T12:44:58Z | `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | `experiments/verification/orbit-recovery/analyze.py (python)` | source-git-newer |
| `experiments/verification/orbit-recovery/orbit-recovery.jsonl` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T12:44:58Z | `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | `experiments/verification/orbit-recovery/analyze.py (python)`<br>`experiments/verification/orbit-recovery/plot_orbit_recovery.py (python)` | source-git-newer |
| `experiments/verification/orbit-recovery/orbit_recovery_errors.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-17T12:44:58Z | `experiments/verification/orbit-recovery/plot_orbit_recovery.py (python)` | - | ok |
| `experiments/verification/orbit-recovery/smoke-orbit-recovery-orbits.jsonl` | no | no | yes | - | - | `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | - | missing |
| `experiments/verification/orbit-recovery/smoke-orbit-recovery.jsonl` | no | no | yes | - | - | `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | - | missing |
| `experiments/visualization/main/viz-hko-pentagon-edges.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-hko-pentagon-traj.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-hypercube-edges.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-hypercube-ridges.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-hypercube-traj.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-lagrangian-tri-product-traj.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |
| `experiments/visualization/main/viz-simplex-traj.png` | yes | yes | no | 2026-04-17T16:39:00Z | 2026-04-14T21:47:11Z | - | - | ok |

## Sources

| Source | Command | Input Artifacts | Output Artifacts |
| --- | --- | --- | --- |
| `experiments/combinatorial-cells/boundary-characterization/analyze.py (python)` | `uv run experiments/combinatorial-cells/boundary-characterization/analyze.py` | `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-anatomy.jsonl`<br>`experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-crossing.jsonl` | `experiments/combinatorial-cells/boundary-characterization/boundary_density_cdf.png`<br>`experiments/combinatorial-cells/boundary-characterization/boundary_event_types.png`<br>`experiments/combinatorial-cells/boundary-characterization/boundary_sys_continuity.png`<br>`experiments/combinatorial-cells/boundary-characterization/boundary_tmax_by_direction.png`<br>`experiments/combinatorial-cells/boundary-characterization/boundary_tmax_distribution.png`<br>`experiments/combinatorial-cells/boundary-characterization/boundary_tmax_vs_F.png`<br>`experiments/combinatorial-cells/boundary-characterization/orbit_gap_distribution.png`<br>`experiments/combinatorial-cells/boundary-characterization/orbit_gap_vs_switch.png` |
| `experiments/combinatorial-cells/boundary-characterization/main.rs (cargo bin cell-boundary-characterization)` | `cargo run -p exp-combinatorial-cells --release --bin cell-boundary-characterization` | `experiments/combinatorial-cells/polytopes.jsonl` | `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-anatomy.jsonl`<br>`experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-crossing.jsonl`<br>`experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-gradient.jsonl` |
| `experiments/combinatorial-cells/cell-widths/analyze.py (python)` | `uv run experiments/combinatorial-cells/cell-widths/analyze.py` | `experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl` | `experiments/combinatorial-cells/cell-widths/cell_anisotropy.png`<br>`experiments/combinatorial-cells/cell-widths/cell_orbit_vs_nonorbit.png`<br>`experiments/combinatorial-cells/cell-widths/cell_width_by_F.png`<br>`experiments/combinatorial-cells/cell-widths/profiling_event_types.png` |
| `experiments/combinatorial-cells/cell-widths/main.rs (cargo bin cell-widths)` | `cargo run -p exp-combinatorial-cells --release --bin cell-widths` | `experiments/combinatorial-cells/polytopes.jsonl` | `experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl` |
| `experiments/combinatorial-cells/convexity/analyze.py (python)` | `uv run experiments/combinatorial-cells/convexity/analyze.py` | `experiments/combinatorial-cells/convexity/combinatorial-boundaries-convexity.jsonl` | `experiments/combinatorial-cells/convexity/cell_convexity.png` |
| `experiments/combinatorial-cells/convexity/main.rs (cargo bin cell-convexity)` | `cargo run -p exp-combinatorial-cells --release --bin cell-convexity` | `experiments/combinatorial-cells/polytopes.jsonl` | `experiments/combinatorial-cells/convexity/combinatorial-boundaries-convexity.jsonl` |
| `experiments/combinatorial-cells/gradient-discontinuity/analyze.py (python)` | `uv run experiments/combinatorial-cells/gradient-discontinuity/analyze.py` | `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-gradient.jsonl`<br>`experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-anatomy.jsonl`<br>`experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl` | `experiments/combinatorial-cells/gradient-discontinuity/boundary_gradient_angle.png`<br>`experiments/combinatorial-cells/gradient-discontinuity/gradient_cell_alignment.png` |
| `experiments/combinatorial-cells/multiple-crossings/analyze.py (python)` | `uv run experiments/combinatorial-cells/multiple-crossings/analyze.py` | `experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl` | - |
| `experiments/combinatorial-cells/multiple-crossings/main.rs (cargo bin cell-multiple-crossings)` | `cargo run -p exp-combinatorial-cells --release --bin cell-multiple-crossings` | `experiments/combinatorial-cells/polytopes.jsonl` | `experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl` |
| `experiments/combinatorial-cells/omega-hypothesis/analyze.py (python)` | `uv run experiments/combinatorial-cells/omega-hypothesis/analyze.py` | `experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl` | `experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_dots.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_gradient_neighbor_split.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_omega_vs_dot.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_mean_vs_sys.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_min_vs_sys.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_orbit_vs_nonorbit.png`<br>`experiments/combinatorial-cells/omega-hypothesis/omega_obstacle_ridge_min_vs_sys.png` |
| `experiments/combinatorial-cells/omega-hypothesis/main.rs (cargo bin cell-omega)` | `cargo run -p exp-combinatorial-cells --release --bin cell-omega` | - | `experiments/combinatorial-cells/polytopes.jsonl`<br>`experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl` |
| `experiments/crosspolytope/main/main.rs (cargo bin crosspolytope)` | `cargo run -p crosspolytope --release --bin crosspolytope` | - | `experiments/crosspolytope/main/crosspolytope.jsonl` |
| `experiments/figure_config.py (python)` | `uv run experiments/figure_config.py` | - | - |
| `experiments/hko-local-maximum/cut-and-ascent/main.rs (cargo bin hko-cut-and-ascent)` | `cargo run -p exp-hko-local-maximum --release --bin hko-cut-and-ascent` | - | `experiments/hko-local-maximum/cut-and-ascent/cut-and-ascent.jsonl` |
| `experiments/hko-local-maximum/facet-splitting/analyze.py (python)` | `uv run experiments/hko-local-maximum/facet-splitting/analyze.py` | `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.jsonl` | `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.png` |
| `experiments/hko-local-maximum/facet-splitting/main.rs (cargo bin hko-facet-splitting)` | `cargo run -p exp-hko-local-maximum --release --bin hko-facet-splitting` | - | `experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.jsonl` |
| `experiments/hko-local-maximum/gradient-analysis/analyze.py (python)` | `uv run experiments/hko-local-maximum/gradient-analysis/analyze.py` | `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl`<br>`experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-ascent.jsonl` | `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-gradient.png`<br>`experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-orbits.png` |
| `experiments/hko-local-maximum/gradient-analysis/main.rs (cargo bin hko-gradient-analysis)` | `cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis` | - | `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl`<br>`experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-ascent.jsonl` |
| `experiments/hko-local-maximum/lagrangian-boundary/analyze.py (python)` | `uv run experiments/hko-local-maximum/lagrangian-boundary/analyze.py` | `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search.jsonl`<br>`experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search-levels.jsonl` | `experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_fraction.png`<br>`experiments/hko-local-maximum/lagrangian-boundary/lagrangian_search_sys_vs_eps.png`<br>`experiments/hko-local-maximum/lagrangian-boundary/lagrangian_probe_radii.png` |
| `experiments/hko-local-maximum/lagrangian-boundary/main.rs (cargo bin hko-lagrangian-boundary)` | `cargo run -p exp-hko-local-maximum --release --bin hko-lagrangian-boundary` | - | `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search.jsonl`<br>`experiments/hko-local-maximum/lagrangian-boundary/lagrangian-search-levels.jsonl` |
| `experiments/hko-local-maximum/lagrangian-boundary/probe.rs (cargo bin hko-lagrangian-probe)` | `cargo run -p exp-hko-local-maximum --release --bin hko-lagrangian-probe` | - | `experiments/hko-local-maximum/lagrangian-boundary/lagrangian-probe.jsonl` |
| `experiments/hko-local-maximum/perturbation-neighborhood/analyze.py (python)` | `uv run experiments/hko-local-maximum/perturbation-neighborhood/analyze.py` | `experiments/hko-local-maximum/perturbation-neighborhood/pentagon-perturb.jsonl` | `experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png` |
| `experiments/hko-local-maximum/perturbation-neighborhood/main.rs (cargo bin hko-perturbation)` | `cargo run -p exp-hko-local-maximum --release --bin hko-perturbation` | - | `experiments/hko-local-maximum/perturbation-neighborhood/pentagon-perturb.jsonl` |
| `experiments/hko-local-maximum/second-order/analyze.py (python)` | `uv run experiments/hko-local-maximum/second-order/analyze.py` | `experiments/hko-local-maximum/second-order/second-order-base.jsonl`<br>`experiments/hko-local-maximum/second-order/second-order-curves.jsonl` | `experiments/hko-local-maximum/second-order/second_order_curves.png`<br>`experiments/hko-local-maximum/second-order/second_order_curvatures.png`<br>`experiments/hko-local-maximum/second-order/second_order_random_hist.png` |
| `experiments/hko-local-maximum/second-order/main.rs (cargo bin hko-second-order)` | `cargo run -p exp-hko-local-maximum --release --bin hko-second-order` | - | `experiments/hko-local-maximum/second-order/second-order-base.jsonl`<br>`experiments/hko-local-maximum/second-order/second-order-curves.jsonl`<br>`experiments/hko-local-maximum/second-order/second-order-random.jsonl` |
| `experiments/hko-local-maximum/subdifferential-lp/phase_c_lp_test.py (python)` | `uv run experiments/hko-local-maximum/subdifferential-lp/phase_c_lp_test.py` | `experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl` | - |
| `experiments/numerics/error-bounds/analyze.py (python)` | `uv run experiments/numerics/error-bounds/analyze.py` | - | - |
| `experiments/numerics/error-bounds/collect_poly.rs (cargo bin num-collect-poly)` | `cargo run -p dev-numerical-analysis --release --bin num-collect-poly` | - | `experiments/numerics/error-bounds/collected_poly.jsonl` |
| `experiments/numerics/error-bounds/main.rs (cargo bin num-error-bounds)` | `cargo run -p dev-numerical-analysis --release --bin num-error-bounds` | - | - |
| `experiments/numerics/gradient/numerics-edge-cases/analyze.py (python)` | `uv run experiments/numerics/gradient/numerics-edge-cases/analyze.py` | `experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q3-degeneracy.jsonl`<br>`experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q4-redundant.jsonl` | `experiments/numerics/gradient/numerics-edge-cases/gc_q3_gap.png`<br>`experiments/numerics/gradient/numerics-edge-cases/gc_q4_delta.png` |
| `experiments/numerics/gradient/numerics-edge-cases/main.rs (cargo bin dev_numerics_edge_cases)` | `cargo run -p dev-gradient --release --bin dev_numerics_edge_cases` | - | `experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q3-degeneracy.jsonl`<br>`experiments/numerics/gradient/numerics-edge-cases/gradient-correctness-q4-redundant.jsonl` |
| `experiments/numerics/gradient/numerics-subdifferential/analyze.py (python)` | `uv run experiments/numerics/gradient/numerics-subdifferential/analyze.py` | `experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5-subdiff.jsonl`<br>`experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5b-symmetric.jsonl` | `experiments/numerics/gradient/numerics-subdifferential/gc_q5_convergence.png`<br>`experiments/numerics/gradient/numerics-subdifferential/gc_q5_switching.png`<br>`experiments/numerics/gradient/numerics-subdifferential/gc_q5b_boundary.png` |
| `experiments/numerics/gradient/numerics-subdifferential/main.rs (cargo bin dev_numerics_subdifferential)` | `cargo run -p dev-gradient --release --bin dev_numerics_subdifferential` | - | `experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5-subdiff.jsonl`<br>`experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5b-symmetric.jsonl` |
| `experiments/numerics/gradient/numerics/analyze.py (python)` | `uv run experiments/numerics/gradient/numerics/analyze.py` | `experiments/numerics/gradient/numerics/gradient-correctness-q1-generic.jsonl`<br>`experiments/numerics/gradient/numerics/gradient-correctness-q2-nongeneric.jsonl` | `experiments/numerics/gradient/numerics/gc_convergence.png`<br>`experiments/numerics/gradient/numerics/gc_slopes.png` |
| `experiments/numerics/gradient/numerics/main.rs (cargo bin dev_numerics)` | `cargo run -p dev-gradient --release --bin dev_numerics` | - | `experiments/numerics/gradient/numerics/gradient-correctness-q1-generic.jsonl`<br>`experiments/numerics/gradient/numerics/gradient-correctness-q2-nongeneric.jsonl` |
| `experiments/numerics/kkt-inertia/main.rs (cargo bin num-kkt-inertia)` | `cargo run -p dev-numerical-analysis --release --bin num-kkt-inertia` | - | - |
| `experiments/numerics/q-error/main.rs (cargo bin num-q-error)` | `cargo run -p dev-numerical-analysis --release --bin num-q-error` | - | - |
| `experiments/numerics/unknown-predicates/analyze.py (python)` | `uv run experiments/numerics/unknown-predicates/analyze.py` | `experiments/numerics/unknown-predicates/unknown-predicates.jsonl` | `experiments/numerics/unknown-predicates/unknown_predicates_beta_min.png` |
| `experiments/numerics/unknown-predicates/main.rs (cargo bin num-unknown-predicates)` | `cargo run -p dev-numerical-analysis --release --bin num-unknown-predicates` | - | `experiments/numerics/unknown-predicates/unknown-predicates.jsonl` |
| `experiments/sys-landscape/gradient-ascent-dev/step-calibration/main.rs (cargo bin dev_step_calibration)` | `cargo run -p dev-gradient-ascent --release --bin dev_step_calibration` | - | - |
| `experiments/sys-landscape/gradient-ascent-dev/strategy-comparison/main.rs (cargo bin dev_strategy_comparison)` | `cargo run -p dev-gradient-ascent --release --bin dev_strategy_comparison` | - | - |
| `experiments/sys-landscape/gradient-ascent-general/analyze.py (python)` | `uv run experiments/sys-landscape/gradient-ascent-general/analyze.py` | `experiments/sys-landscape/gradient-ascent-general/licca.jsonl`<br>`experiments/sys-landscape/gradient-ascent-general/smoke.jsonl` | `experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_distribution.png`<br>`experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_tail.png`<br>`experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_improvement.png`<br>`experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_strategy.png`<br>`experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_escape.png`<br>`experiments/sys-landscape/gradient-ascent-general/gradient_ascent_general_convergence.png` |
| `experiments/sys-landscape/gradient-ascent-general/main.rs (cargo bin sys-gradient-ascent-general)` | `cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-general` | `experiments/sys-landscape/cache.jsonl` | `experiments/sys-landscape/cache.jsonl`<br>`experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl`<br>`experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl` |
| `experiments/sys-landscape/gradient-ascent-products/analyze.py (python)` | `uv run experiments/sys-landscape/gradient-ascent-products/analyze.py` | `experiments/sys-landscape/gradient-ascent-products/licca.jsonl`<br>`experiments/sys-landscape/gradient-ascent-products/smoke.jsonl` | `experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_distribution.png`<br>`experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_tail.png`<br>`experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_improvement.png`<br>`experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_strategy.png`<br>`experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_escape.png`<br>`experiments/sys-landscape/gradient-ascent-products/gradient_ascent_products_convergence.png` |
| `experiments/sys-landscape/gradient-ascent-products/main.rs (cargo bin sys-gradient-ascent-products)` | `cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-products` | `experiments/sys-landscape/cache.jsonl` | `experiments/sys-landscape/cache.jsonl`<br>`experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl`<br>`experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products-trace.jsonl` |
| `experiments/sys-landscape/random-product-sample/analyze.py (python)` | `uv run experiments/sys-landscape/random-product-sample/analyze.py` | `experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl` | `experiments/sys-landscape/random-product-sample/random_product_sweep_sys_vs_pair.png` |
| `experiments/sys-landscape/random-product-sample/main.rs (cargo bin sys-random-product-sample)` | `cargo run -p exp-sys-landscape --release --bin sys-random-product-sample` | `experiments/sys-landscape/cache.jsonl` | `experiments/sys-landscape/cache.jsonl`<br>`experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl` |
| `experiments/sys-landscape/random-sample/analyze.py (python)` | `uv run experiments/sys-landscape/random-sample/analyze.py` | `experiments/sys-landscape/random-sample/random-sweep.jsonl` | `experiments/sys-landscape/random-sample/random_sweep_sys_vs_f.png` |
| `experiments/sys-landscape/random-sample/main.rs (cargo bin sys-random-sample)` | `cargo run -p exp-sys-landscape --release --bin sys-random-sample` | `experiments/sys-landscape/cache.jsonl` | `experiments/sys-landscape/random-sample/random-sweep.jsonl`<br>`experiments/sys-landscape/cache.jsonl` |
| `experiments/sys-landscape/rejection-calibration/main.rs (cargo bin sys-rejection-calibration)` | `cargo run -p exp-sys-landscape --release --bin sys-rejection-calibration` | - | `experiments/sys-landscape/rejection-calibration/acceptance.jsonl` |
| `experiments/sys-landscape/rotated-regular-products/analyze.py (python)` | `uv run experiments/sys-landscape/rotated-regular-products/analyze.py` | `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl` | `experiments/sys-landscape/rotated-regular-products/lagrangian_products_5x5.png`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian_products_7x7.png`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian_products_polygon_pairs.png` |
| `experiments/sys-landscape/rotated-regular-products/main.rs (cargo bin sys-rotated-regular-products)` | `cargo run -p exp-sys-landscape --release --bin sys-rotated-regular-products` | - | `experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-7x7.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x3-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x4-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x5-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x5-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-4x6-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x6-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-6x6-6deg.jsonl`<br>`experiments/sys-landscape/rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl` |
| `experiments/sys-landscape/variable-f-ascent/analyze.py (python)` | `uv run experiments/sys-landscape/variable-f-ascent/analyze.py` | `experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl` | `experiments/sys-landscape/variable-f-ascent/variable-f-rq1.png`<br>`experiments/sys-landscape/variable-f-ascent/variable-f-rq2.png` |
| `experiments/sys-landscape/variable-f-ascent/main.rs (cargo bin sys-variable-f-ascent)` | `cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent` | `experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl` | `experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl`<br>`experiments/sys-landscape/variable-f-ascent/cache.jsonl` |
| `experiments/verification/algorithm-comparison/ablation/analyze.py (python)` | `uv run experiments/verification/algorithm-comparison/ablation/analyze.py` | `experiments/verification/algorithm-comparison/ablation/ablation.jsonl` | `experiments/verification/algorithm-comparison/ablation/ablation_timing.png` |
| `experiments/verification/algorithm-comparison/ablation/main.rs (cargo bin cmp-ablation)` | `cargo run -p dev-algorithm-comparison --release --bin cmp-ablation` | - | `experiments/verification/algorithm-comparison/ablation/ablation.jsonl` |
| `experiments/verification/algorithm-comparison/benchmark/analyze.py (python)` | `uv run experiments/verification/algorithm-comparison/benchmark/analyze.py` | `experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl` | `experiments/verification/algorithm-comparison/benchmark/benchmark_timing.png` |
| `experiments/verification/algorithm-comparison/benchmark/main.rs (cargo bin cmp-benchmark)` | `cargo run -p dev-algorithm-comparison --release --bin cmp-benchmark` | - | `experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl` |
| `experiments/verification/algorithm-comparison/benchmark/profile.rs (cargo bin cmp-benchmark-profile)` | `cargo run -p dev-algorithm-comparison --release --bin cmp-benchmark-profile` | - | - |
| `experiments/verification/algorithm-comparison/benchmark/profiling/analyze_profiling.py (python)` | `uv run experiments/verification/algorithm-comparison/benchmark/profiling/analyze_profiling.py` | - | `experiments/verification/algorithm-comparison/benchmark/profiling/phase_breakdown.png`<br>`experiments/verification/algorithm-comparison/benchmark/profiling/micro_benchmarks.png` |
| `experiments/verification/algorithm-comparison/profiling/analyze.py (python)` | `uv run experiments/verification/algorithm-comparison/profiling/analyze.py` | - | `experiments/verification/algorithm-comparison/profiling/profile.jsonl`<br>`experiments/verification/algorithm-comparison/profiling/logbook.jsonl`<br>`experiments/verification/algorithm-comparison/profiling/test_timing.png` |
| `experiments/verification/all-minimum/analyze.py (python)` | `uv run experiments/verification/all-minimum/analyze.py` | `experiments/verification/all-minimum/all-minimum.jsonl`<br>`experiments/verification/all-minimum/all-minimum-orbits.jsonl` | - |
| `experiments/verification/all-minimum/main.rs (cargo bin axioms-all-minimum)` | `cargo run -p dev-capacity-validation --release --bin axioms-all-minimum` | - | `experiments/verification/all-minimum/all-minimum.jsonl`<br>`experiments/verification/all-minimum/all-minimum-orbits.jsonl`<br>`experiments/verification/all-minimum/smoke-all-minimum.jsonl`<br>`experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl` |
| `experiments/verification/correctness/main.rs (cargo bin axioms-correctness)` | `cargo run -p dev-capacity-validation --release --bin axioms-correctness` | - | `experiments/verification/correctness/correctness.jsonl` |
| `experiments/verification/orbit-recovery/analyze.py (python)` | `uv run experiments/verification/orbit-recovery/analyze.py` | `experiments/verification/orbit-recovery/orbit-recovery.jsonl`<br>`experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl` | - |
| `experiments/verification/orbit-recovery/main.rs (cargo bin axioms-orbit-recovery)` | `cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery` | `experiments/verification/all-minimum/all-minimum-orbits.jsonl`<br>`experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl` | `experiments/verification/orbit-recovery/orbit-recovery.jsonl`<br>`experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl`<br>`experiments/verification/orbit-recovery/smoke-orbit-recovery.jsonl`<br>`experiments/verification/orbit-recovery/smoke-orbit-recovery-orbits.jsonl` |
| `experiments/verification/orbit-recovery/plot_orbit_recovery.py (python)` | `uv run experiments/verification/orbit-recovery/plot_orbit_recovery.py` | `experiments/verification/orbit-recovery/orbit-recovery.jsonl` | `experiments/verification/orbit-recovery/orbit_recovery_errors.png` |
| `experiments/visualization/main/main.rs (cargo bin visualization)` | `cargo run -p visualization --release --bin visualization` | - | - |
