# Random-Generator Rejection Calibration

Question: how often does the legacy random-polytope generator accept candidates
across its fixed facet-count and height-range grid, and how much time is spent
on accepted versus rejected attempts?

`main.rs` generates 1,000 attempts for each of 18 hardcoded configurations
(`F=5..10` and three height ranges) with seed `42`, then rewrites
`acceptance.jsonl`. The retained JSONL is the answer artifact; inspect it for
exact per-configuration ratios and timings.

```bash
cargo run -p exp-sys-landscape --release --bin sys-rejection-calibration
```

There is no smoke or output-path override, so do not use that command as a
quick check. The test module runs smaller sweeps for row-count and arithmetic
invariants. Timing values are local-machine measurements, and acceptance
ratios belong to the named generator, seed policy, grid, and attempt count.
