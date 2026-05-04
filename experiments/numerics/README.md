# Numerics Experiments

This package owns exploratory and validation numerics artifacts interpreted in
`research/numerics.md` and `research/numerics-error-bounds.md`.

## Rust Command Contract

- `num-algebraic-exactness` defaults to smoke outputs. Use `--canonical` only
  when refreshing `algebraic-exactness/exact-*.jsonl`.
- `num-sage-feasibility` defaults to smoke input. Use `--canonical` only when
  refreshing `sage-feasibility/sage-feasibility-input.jsonl`.
- `num-unknown-predicates --smoke` writes
  `unknown-predicates/unknown-predicates-smoke.jsonl`; full mode writes the
  tracked `unknown-predicates/unknown-predicates.jsonl`.
- `num-error-bounds` writes only the output path passed on the CLI.
- `num-collect-poly` writes `error-bounds/collected_poly.jsonl`; run it only
  when intentionally refreshing that stage input.
- `num-q-error` and `num-kkt-inertia` print diagnostic summaries and do not
  write JSONL.

The gradient-validation subpackage has its own smoke helpers under
`experiments/numerics/gradient/`.
