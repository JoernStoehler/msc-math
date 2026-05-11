# symplectic Development Notes

## Profiling And Coverage

Use the crate-local pruned HK2017 profiler for quick phase timings on
deterministic random fixtures:

```bash
cargo run -p symplectic --release --bin profile-pruned-hk2017 -- \
  --facet-counts 5,6,7,8 --samples 3 --jsonl
```

Each JSONL row reports fixture generation, exact geometry construction,
transition-matrix construction, candidate solving, aggregation, capacity, and
the existing search counts exposed by the production API. Treat this as the
first question surface: it says which named phase is worth inspecting next. It
does not expose internal KKT or pruning sub-counters.

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
