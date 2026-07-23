# Sys-Landscape

This package owns hostile-landscape search experiment binaries and legacy
producer surfaces. The maintained data-science pipeline lives at
`experiments/sys-datascience/`.

Legacy ascent and continuation observations that are still worth preserving are
summarized in `legacy-ascent-continuation-debt.md`. They are not active
data-science method rows.

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
- `sys1-local-maxima` is retained as the binary name, but its root topic,
  documentation, and artifacts are `experiments/local-maxima-check/`.
- `sys-fixed-shape-orientation-search` owns a retained post-selection scan of
  the generic/product source champions over `SO(4)/U(2)`; its complete output
  and claim boundary are in `fixed-shape-orientation-search/README.md`.
- `sys-fixed-shape-linear-search` extends that scan over determinant-one linear
  maps modulo symplectic maps, with the orientation scan as its compact
  zero-distortion stratum. Its sparse random global-transform route is stopped
  after a negative two-body pilot; the owner README records what would justify
  reopening it.
- `sys-rejection-calibration` is a full-output producer without a smoke mode; do
  not run it as a quick command check unless intentionally refreshing tracked
  artifacts.
- `sys-dataset-*` producer commands are documented in
  `experiments/sys-datascience/produce/README.md`; their default behavior is
  temp smoke output unless explicit output/cache paths are supplied.
- `sys-dataset` writes tables to the `--out-dir` path. For method waves, use
  `experiments/sys-datascience/prepare/`; temp output is only for
  one-off smoke/scratch runs.

The tracked full-output JSONL files are evidence artifacts. Use compile checks
or documented smoke/temp modes for local command validation.

For the maintained produce -> tables -> methods flow, start at
`experiments/sys-datascience/README.md`.
