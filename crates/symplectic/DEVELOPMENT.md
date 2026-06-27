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

## Profiling And Coverage

Use `experiments/performance/` for reusable profiling targets, JSONL outputs,
trace summaries, and profiler wrapper commands.

Use source-based coverage when the question is whether tests execute a
line/region path:

```bash
cargo llvm-cov -p symplectic --lib --summary-only -- TEST_FILTER
```

In the local devcontainer, `cargo-llvm-cov` is installed with the Rust
`llvm-tools-preview` component. Stable Rust gives useful line, function, and
region coverage. `cargo llvm-cov --branch` requires nightly at the time this
note was written, so do not present stable coverage output as branch coverage.

Coverage does not prove mathematical correctness, numerical robustness, or
representative sampling. It also does not provide trustworthy timings because
coverage instrumentation changes optimization and execution shape.
