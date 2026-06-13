# Sys-Landscape

This package owns hostile-landscape search experiment binaries and legacy
producer surfaces interpreted in `research/sys-landscape.md`,
`tasks/current-state.md`, and `tasks/planning-notes.md`. The maintained
data-science pipeline lives at `experiments/sys-datascience/`.

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
- `sys-rejection-calibration` is a full-output producer without a smoke mode; do
  not run it as a quick command check unless intentionally refreshing tracked
  artifacts.
- `sys-step-calibration` and `sys-strategy-comparison` are development stubs and
  write no artifacts.
- `sys-dataset-*` producer commands are documented in
  `experiments/sys-datascience/produce/README.md`; their default behavior is
  temp smoke output unless explicit output/cache paths are supplied.
- `sys-dataset` writes tables to the `--out-dir` path. For method waves, use
  `experiments/sys-datascience/tables/`; temp output is only for
  one-off smoke/scratch runs.

The tracked full-output JSONL files are evidence artifacts. Use compile checks
or documented smoke/temp modes for local command validation.

For the maintained produce -> tables -> methods flow, start at
`experiments/sys-datascience/README.md`.
