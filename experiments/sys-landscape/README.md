# Sys-Landscape

This package owns hostile-landscape search experiments and the maintained
datascience pipeline interpreted in `research/sys-landscape.md` and
`tasks/landscape.md`.

## Rust Command Contract

- `sys-random-sample` and `sys-random-product-sample` default to untracked temp
  smoke outputs and temp cache paths. Use explicit `--out` and `--cache` only
  when refreshing the tracked legacy artifacts.
- `sys-gradient-ascent-general` and `sys-gradient-ascent-products` default to
  untracked temp smoke outputs. Use explicit `--out` and database-update flags
  only when refreshing tracked endpoint artifacts and the shared cache.
- `sys-variable-f-ascent --smoke` writes temp smoke output/cache paths. Full
  mode writes `variable-f-ascent/variable-f-ascent.jsonl` and
  `variable-f-ascent/cache.jsonl`.
- `sys-pentagon-rotation-formula` defaults to smoke output names. Use
  `--canonical` only when refreshing tracked formula artifacts.
- `sys-rotated-regular-products` and `sys-rejection-calibration` are full-output
  producers without smoke modes; do not run them as quick command checks unless
  intentionally refreshing tracked artifacts.
- `sys-step-calibration` and `sys-strategy-comparison` are development stubs and
  write no artifacts.
- `sys-dataset-*` producer commands are documented in
  `datascience/produce/README.md`; their default behavior is temp smoke output
  unless explicit output/cache paths are supplied.
- `sys-dataset` writes ad hoc tables to the `--out-dir` path and is safe to run
  against a temp directory.

The tracked full-output JSONL files are evidence artifacts. Use compile checks
or documented smoke/temp modes for local command validation.
