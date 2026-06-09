# Algorithm Comparison

This package owns variant-consistency experiments and historical comparison
artifacts for capacity-algorithm implementation choices. Use
`experiments/performance/` for new reusable profiling targets and profiling
post-processing.

## Rust Command Contract

- `cmp-ablation --smoke` writes `ablation/ablation-smoke.jsonl`; full mode
  writes `ablation/ablation.jsonl`.
- `cmp-benchmark --smoke` writes `benchmark/benchmark-smoke.jsonl`; full mode
  writes `benchmark/benchmark.jsonl`.
- `cmp-benchmark-profile [F] [iterations]` writes no JSONL; profiler output is
  owned by the external profiling command.

The tracked full-output JSONL files are experiment evidence. Use smoke mode for
local command checks unless intentionally refreshing those artifacts.
