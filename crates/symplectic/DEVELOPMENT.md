# symplectic Development Notes

## Profiling And Coverage

Use `experiments/performance/` for reusable performance targets, phase-event
JSONL, post-processing scripts, and call-stack profiler wrapper commands.

Use `experiments/performance/README.md` for current rerunnable HK2017 profiling
targets, trace summaries, and profiler wrapper commands. The older crate-local
pruned HK2017 profiler is legacy/ad-hoc only.

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
