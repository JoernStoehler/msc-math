# symplectic Development Notes

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
