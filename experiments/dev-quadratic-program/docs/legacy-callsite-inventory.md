# Legacy QP call-site inventory

Status: source-wide migration inventory at `b239e98b6` plus the uncommitted
fixed-shape/cost-smoke slice. This is a migration gate, not a permanent manual
dependency map.

## Search scope

The direct inventory is the 74 Rust files returned by:

```bash
rg -l --glob '*.rs' \
  '\b(capacity_auto|capacity_pruned_hk2017|capacity_unpruned_hk2017|capacity_billiard|solve_pruned_hk2017_candidates|solve_unpruned_hk2017_candidates|solve_billiard_candidates|aggregate_orbits_with_dual_vertices_exact|aggregate_certified_orbits_with_dual_vertices_exact|solve_orbit_sigma_saddle_point)\s*\(' \
  crates experiments
```

Indirect consumers of `exp_sys_landscape::compute_capacity_result`,
`compute_sys_computation`, and `compute_active_sys_state` were searched
separately because they do not name the legacy solver. The categories below
cover every direct result and those indirect consumers. Re-run both searches
after rebasing or adding a capacity route; this file does not claim authority
over later source.

## Disposition

| Source group | Why it uses the old surface | Migration disposition |
| --- | --- | --- |
| `crates/symplectic/src/algorithms/{orbit_search,hk2017,billiard}/**` and their unit/regression/literature tests | Defines the retained f64 orbit/KKT payload and the old HK/billiard implementations | Keep. These are implementation, compatibility, and independent-control surfaces, not ordinary capacity callers. |
| `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs` | Exact-certifies a caller-defined flow-graph candidate stream | Keep under its stream-coverage contract; the production capacity route is not a substitute for the flow-graph experiment. |
| `crates/symplectic/benches/profiling.rs` and `crates/symplectic/src/kkt/test_saddle_point_solver.rs` | Measures/tests legacy and one-sigma kernels | Keep as explicitly named performance/test controls. |
| `crates/symplectic/tests/public_capacity_api.rs` | Includes old one-sigma behavior as a correspondence/control assertion alongside the production API | Keep the old call only where the test states that comparison. |
| `experiments/dev-quadratic-program/**`, `experiments/qp-error-bounds/**`, `experiments/verification/**`, and `experiments/performance/**` | Exact, legacy, mutation, numerical, recovery, and performance controls | Keep. Their purpose is to judge or explain the production route, so removing the independent or intentionally weaker paths would reduce evidence. |
| `experiments/regular-products/pentagon-rotation-empirics/main.rs`, `experiments/sys-datascience/methods/product-bounce-distribution/{class-minima.rs,audit-null-availability.rs}`, and `experiments/sys-datascience/methods/ridge-endpoint-path/{src/main.rs,artifacts/evaluator-source-v2-before-promotion.rs}` | Enumerated non-minimal branches, per-word solve outcomes, or a frozen candidate-stream certificate | Keep. These packets request an orbit/branch object that `qp_minimizers` deliberately does not claim to provide. Retained artifacts remain immutable. |
| `experiments/combinatorial-cells/**` | Inactive boundary-transition evidence whose selected word, gaps, and branch diagnostics are part of the recorded method | Keep as historical/instrumented experiment code. Its README prohibits routine producer reruns and requires a new question before revival. |
| `experiments/hko-local-maximum/**` | Theorem-support and historical empirical packets use active branches, derivatives, bounce counts, or old-route comparisons | Keep for reproduction and branch evidence. The active HKO theorem certificate is the separate exact feasible-section route, so changing these old empirical producers has no current thesis-critical benefit. |
| `experiments/dev-gradient-ascent/**` and `experiments/dev-sys-prediction/**` | Optimizer-local branch windows, beta-nonpositive/transition-blocked models, derivatives, and branch diagnostics | Keep until the optimizer owner applies the production scalar/admissible-window handoff. Do not broaden `capacity_4d` to absorb these deliberate heuristic branch models. |
| `experiments/polytope-datasets/{random.rs,random-product.rs}` and `experiments/sys-landscape/{random-sample,random-product-sample,variable-f-ascent}/**` | Superseded canonical or removed-input producers retained for historical reproduction | Keep as legacy machinery. Fresh maintained random/product/reference production goes through the migrated run-local computed-polytope cache. |
| `experiments/sys-datascience/methods/alternative-source-transfer/src/evaluator.rs`, `generic-ridge-tail-stage1-target/{src/main.rs,provenance/target-run-main.rs}`, and `ridge-symmetry-completion/src/main.rs` | Completed, identity-bound target evaluations | Freeze. Do not relabel or rerun their accepted rows as production-route evidence. A new scientific run must use a new schema and evaluator decision. |
| `experiments/sys-landscape/fixed-shape-orientation-search/main.rs` | Scalar capacity plus an explicit paired legacy control | Migrated for fresh rows: production is the default and writes v2 bounds/route fields; legacy remains opt-in. |
| `experiments/sys-landscape/fixed-shape-orientation-search/global.rs` | Scalar capacity on strongly transformed, often exact-fallback-heavy inputs | Named exception. Production spot-check mode writes certified v2 bounds, but the stopped full scan retains its legacy default after a paired 26-row smoke measured a `9.25x` production regression with `3.13e-15` maximum route disagreement. |
| `experiments/canonization-t-search/src/bin/sys_cost_smoke.rs` | Previously called the shared legacy result only to test scalar availability and also timed rounded exact volume | Migrated to production capacity and f64 incidence volume; the printed method labels prevent future cost-scope confusion. |
| `experiments/sys-landscape/src/lib.rs` legacy wrappers | Compatibility owner for the retained packets above and the still-unmigrated ascent state | Keep until the indirect-consumer blocker below is resolved. It is not the recommended ordinary scalar API. |
| `experiments/sys-landscape/src/ascent/**` and indirect users in `experiments/{local-maxima-check,dev-gradient-ascent,dev-sys-prediction}/**` | `ActiveSysState`/`SysComputation` exposes the complete legacy `OrbitSearchResult`; consumers use capacity bounds, active tied branches, KKT fields, derivatives, counts, or cache rows in different combinations | Remaining blocker/handoff. Scalar-only indirect consumers should split onto production capacity. Active-set/derivative consumers require the exact minimizer/window plus one-sigma adapter and, for products, an accepted product subdifferential contract. Do not synthesize an `OrbitSearchResult` from production outputs. |
| `experiments/sys-datascience/methods/equal-budget-product-search/**` | Parked target-free prototype stores the complete old orbit payload and derived word-count/support diagnostics | Explicitly parked, not merge debt for the current active pipeline. If reopened, ranking capacity migrates and the orbit diagnostics remain a separately fingerprinted optional block. |
| `experiments/sys-landscape/src/datascience_cache.rs`, the run-local polytope producer, regular-product production sweeps, and `extreme-scalar-rejection-proposer` fresh target evaluation | Ordinary capacity and exact minimizing words | Migrated. Current schemas store production bounds/exact values/family/words and do not manufacture legacy orbit diagnostics; historical rows remain historical. |

## Merge consequence

The remaining ordinary architectural issue is the shared ascent state, not an
unclassified scalar caller. It crosses an optimizer-owned branch and requires
field-by-field migration rather than a wrapper substitution. Before merge,
either:

1. land the coordinated ascent/optimizer adapter and rerun its derivative and
   cache-schema checks; or
2. record that surface as an explicit merge blocker with the owning branch and
   do not present the QP migration as complete.

All other direct old calls above are either production/control definitions,
retained evidence requiring their actual payload, immutable historical
evaluators, inactive machinery, or a measured exception.
