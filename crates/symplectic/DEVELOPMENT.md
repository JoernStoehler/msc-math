# symplectic Development Notes

## KKT/QP Solver Split

`src/kkt/saddle_point_solver.rs` solves the augmented KKT matrix and remains
the main HK2017 one-sigma path.

`src/kkt/projection_solver.rs` has two projection surfaces:

- `solve_projected_critical_point`: solves only the projected stationarity
  equation for `Q` on `C beta = d`. It returns one representative, the critical
  value, flat-direction count, and residuals. It deliberately does not decide
  `beta > 0` and does not run the max-margin LP.
- `solve_projected`: preserves the older positivity-solving behavior by running
  max-margin over flat critical directions before returning `Solution`.

Use the critical-point surface for f64 diagnostic/value experiments where beta
positivity is a later resolver decision. Use `solve_projected` only when the
caller really wants the route-local f64 positivity verdict immediately.

`ProjectedCriticalPointData::q_error_bound` is a residual-based bound for the
computed projected stationarity problem. It bounds the Q-value gap caused by
the reported stationarity residual in the retained eigenspace. It is not an
exact-arithmetic certificate for the input polytope, and it is intentionally
`None` when accepted near-flat residuals leave a nonzero linear term along
flat directions.

## Exact arithmetic in `capacity_4d`

The production capacity route interprets each supplied binary64 coordinate as
that exact dyadic rational. Exact arithmetic is not a default assurance layer:
keep it only for an exact public output, an indeterminate correctness-relevant
predicate, or a measured cheap certificate whose removal would require a
proved conservative replacement.

| Site | Retained exact work | Reason and evidence |
| --- | --- | --- |
| `exact_binary64_polytope_geometry` | Polar vertices and incidence for candidates not rejected by the one-sided f64 filter; exact origin and facet-rank work only after an indeterminate f64 result | `PolytopeGeometry4d` promises exact dyadic primal geometry, and the transition graph must not lose a feasible word. The orientation-sign proof is `formal/f64-orientation-sign-filters.tex`; integer-scaled polar/f64-filter comparisons are in `crates/euclidean-polytopes/tests/polar_vertices.rs`. Generic exact enumeration of every four-facet subset is deliberately not used. |
| `capacity_transition_graph` | Signs of the `F x F` symplectic products, using already-constructed exact dual coordinates | A wrong forbidden edge can make enumeration incomplete. A conservative f64 supergraph could replace this exact check, but no proved and tested implementation is currently selected. This is the current certificate implementation, not a claim that exact arithmetic is mathematically necessary. |
| General words of length below five | Exact `C beta = d` only if the one-sided determinant filter cannot prove inconsistency | The rank-five verified KKT enclosure does not apply. The retained general cohort had 350 interval rejections and zero short exact solves; see `experiments/dev-quadratic-program/tools/general_algorithm_ablation/RESULTS.md`. |
| General words of length at least five | Exact KKT only when the verified inverse-defect/curvature route is indeterminate or the floating-point environment is unsupported | Determinate claims are proved in `formal/hk2017-qp-precision.tex`, especially `lem:kkt-verified-inverse-defect`, `rem:kkt-staged-defect-enclosure`, and `rem:kkt-batched-binary64-contract`. `formal/hk2017-qp-core.tex` (`rem:trinary-beta`) explains why Indeterminate cannot be silently accepted or rejected. The scalar API does not exact-resolve final contenders merely to produce an exact output. |
| General minimizers, action windows, and `solve_sigma_exact` | Exact resolution of requested witnesses/actions | These APIs explicitly return exact binary64-rational answers. The scalar bounds API is the cheaper surface when the caller does not need them. |
| Structural-product route | Exact support resolution only for indeterminate interval predicates, then exact comparison of possible winners | `ProductCapacity4d` currently returns an exact capacity and sparse exact winners. The proof/code bridge, intermediate interval audit, exact-all oracle, and fallback counts are in `formal/product-qp-six-facet-reduction.tex` and `experiments/dev-quadratic-program/tools/product_closure_route/{README.md,RESULTS.md}`. |

When changing one of these sites, measure the caller-shaped stages documented
in `experiments/dev-quadratic-program/performance/CURRENT_COSTS.md`. Do not use
the existence of an exact reference implementation as a reason to put it in
the production call graph.

## Profiling And Coverage

Use `experiments/performance/` for reusable profiling targets, JSONL outputs,
trace summaries, and profiler wrapper commands.

Use source-based coverage when the question is whether tests execute a
line/region path:

```bash
cargo llvm-cov -p symplectic --lib --summary-only -- TEST_FILTER
```

`cargo llvm-cov` additionally requires the `cargo-llvm-cov` executable and the
Rust `llvm-tools-preview` component. Their availability is environment-local;
check it before profiling. Stable Rust gives useful line, function, and region
coverage. `cargo llvm-cov --branch` requires nightly at the time this note was
written, so do not present stable coverage output as branch coverage.

Coverage does not prove mathematical correctness, numerical robustness, or
representative sampling. It also does not provide trustworthy timings because
coverage instrumentation changes optimization and execution shape.
