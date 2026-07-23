# Sys-Landscape

This package owns hostile-landscape search experiment binaries and legacy
producer surfaces. The maintained data-science pipeline lives at
`experiments/sys-datascience/`.

Legacy ascent and continuation observations that are still worth preserving are
summarized in `legacy-ascent-continuation-debt.md`. They are not active
data-science method rows.

## Current Directory Inventory

This table covers every immediate child directory. Several producer folders
remain useful as historical method-development packets even though the
maintained data-science flow has moved elsewhere.

| Directory | What is there |
| --- | --- |
| `src/` | reusable landscape cache, active-state, and capacity support used by this package and downstream experiment packages |
| `random-sample/` | legacy random-body producer outputs and job surface |
| `random-product-sample/` | legacy random-product producer outputs and job surface |
| `gradient-ascent-general/` | legacy general-body ascent producer and retained endpoints |
| `gradient-ascent-observed-general/` | retained fixed panel for the observed multi-direction method candidate |
| `gradient-ascent-products/` | legacy product-body ascent producer and retained endpoints |
| `variable-f-ascent/` | variable-facet-count ascent producer and retained outputs |
| `fixed-shape-orientation-search/` | retained post-selection `SO(4)/U(2)` orientation scan |
| `rejection-calibration/` | retained rejection/calibration producer and full-output artifacts |

Related method-development code also lives in
`experiments/sys-datascience/{produce,prepare,methods}/`; that is a semantic
relationship, not a reason to place those owners under a second directory
tree.

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
- `sys1-local-maxima` is retained as the binary name, but its source, manifest,
  documentation, and artifacts are in `experiments/local-maxima-check/`.
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
The tracked full-output JSONL files are evidence artifacts. Use compile checks
or documented smoke/temp modes for local command validation.

For the maintained produce -> tables -> methods flow, start at
`experiments/sys-datascience/README.md`. Its producer and preparation binaries
and several small Rust method executables are registered by
`experiments/sys-datascience/Cargo.toml`; method packets with an existing
isolated build keep their local manifests. Those packages depend on the shared
`exp-sys-landscape` library, not conversely.
