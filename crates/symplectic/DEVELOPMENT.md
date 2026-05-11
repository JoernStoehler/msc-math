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

For a true end-to-end flamegraph of random F=8 pruned HK2017 capacity from
PRNG sampling through aggregation:

```bash
cargo build -p symplectic --release --bin profile-pruned-hk2017

sudo perf record -F 997 --call-graph dwarf \
  -o /tmp/perf-random-f8-e2e.data -- \
  target/release/profile-pruned-hk2017 --facet-counts 8 --samples 20 --jsonl \
  > /tmp/profile-pruned-hk2017-f8-20.jsonl

sudo chown "$USER:$USER" /tmp/perf-random-f8-e2e.data
flamegraph --perfdata /tmp/perf-random-f8-e2e.data \
  -o /tmp/flamegraph-random-f8-e2e-capacity.svg \
  --title 'Random F=8 E2E pruned HK2017 capacity' \
  --subtitle '20 accepted PRNG-generated samples; sudo perf; release build' \
  --palette rust --image-width 2200 --min-width 0.03 \
  --skip-after profile_pruned_hk2017::main

convert -density 180 \
  /tmp/flamegraph-random-f8-e2e-capacity.svg \
  /tmp/flamegraph-random-f8-e2e-capacity.png
```
