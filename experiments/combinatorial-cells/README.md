# Combinatorial Cells

This package owns combinatorial-boundary exploration outputs interpreted in
`research/combinatorial-cells.md`.

## Rust Command Contract

- `cell-omega` is the upstream producer. It reads and updates the package-local
  canonical cache `polytopes.jsonl` and writes
  `omega-hypothesis/omega-obstacle.jsonl`.
- `cell-widths`, `cell-boundary-characterization`, `cell-convexity`, and
  `cell-multiple-crossings` read `polytopes.jsonl` and write tracked evidence
  JSONL files beside each binary.
- These binaries do not currently have smoke modes. Do not run them as quick
  command checks unless intentionally refreshing the tracked artifacts.
- For compile-only checks, use `cargo test -p exp-combinatorial-cells
  --all-targets` or `cargo clippy -p exp-combinatorial-cells --all-targets`.
