# Sys-Landscape

This package is the physical home of hostile-landscape search experiment
binaries and legacy producer surfaces. The maintained data-science pipeline
lives at `experiments/sys-datascience/`.

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
| `random-sample/` | legacy random-body producer and figure; old raw data moved into, then was superseded by, the active data-science producer |
| `random-product-sample/` | legacy random-product producer and figure; old raw data moved into, then was superseded by, the active data-science producer |
| `gradient-ascent-general/` | legacy general-body ascent producer and figures; raw panel removed, bounded result preserved in the debt note |
| `gradient-ascent-observed-general/` | retained fixed panel for the observed multi-direction method candidate |
| `gradient-ascent-products/` | legacy product-body ascent producer and figures; raw panel removed, bounded result preserved in the debt note |
| `variable-f-ascent/` | legacy variable-facet producer and figures; raw panel and required ascent input removed |
| `fixed-shape-orientation-search/` | retained post-selection `SO(4)/U(2)` orientation scan |
| `rejection-calibration/` | retained rejection/calibration producer and full-output artifacts |

Read the relevant child README before source, figures, or commands. A tracked
figure does not imply that its generating JSONL is still present.

Related method-development code also lives in
`experiments/sys-datascience/{produce,prepare,methods}/`; that is a semantic
relationship, not a reason to place those packages under a second directory
tree.

## Rust Command Contract

- `sys-random-sample` and `sys-random-product-sample` default to untracked temp
  smoke outputs and temp cache paths. The active canonical random/product
  producers and artifacts live under `experiments/sys-datascience/produce/`;
  explicit legacy paths require a deliberate reopened run.
- `sys-gradient-ascent-general` and `sys-gradient-ascent-products` default to
  untracked temp smoke outputs. Use explicit `--out` and database-update flags
  only for a deliberately reopened experiment; the old tracked panels were
  removed.
- `sys-variable-f-ascent --smoke` writes temp smoke output/cache paths. Full
  mode expects a removed legacy general-ascent input and is not reproducible
  from the current tree without an explicit reopen decision.
- `sys1-local-maxima` is retained as the binary name, but its source, manifest,
  documentation, and artifacts are in `experiments/local-maxima-check/`.
- `fixed-shape-orientation-search/` is the physical home of a retained
  post-selection scan of
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
Where a child still retains full-output JSONL, that file is its evidence
artifact. Use compile checks or documented smoke/temp modes for local command
validation.

For the maintained produce -> prepare -> methods flow, start at
`experiments/sys-datascience/README.md`. Its producer and preparation binaries
and several small Rust method executables are registered by
`experiments/sys-datascience/Cargo.toml`; method packets with an existing
isolated build keep their local manifests. Those packages depend on the shared
`exp-sys-landscape` library, not conversely.
