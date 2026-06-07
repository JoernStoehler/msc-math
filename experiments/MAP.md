<!--
Purpose: navigation cache for experiment packages and experiment-owned data.
Context: this map helps agents find topic packages, helper crates, and
experiment artifact patterns. It is descriptive, not a task tracker.

Map maintenance:
- Source truth is package manifests, entrypoints, local READMEs, helper crates,
  generated artifact locations, research notes, and task progress files.
- To check staleness, compare affected sections against those sources with
  targeted `rg`, `rg --files`, manifests, and local package headers.
- To refresh, update navigation and provenance facts; route thesis
  interpretation to `research/` and task state to `tasks/`.
- Keep entries short; point to source files instead of duplicating details.
-->

# Experiments Map

## Status

- State: split from the old root `ARCHITECTURE.md`.
- Last updated: 2026-06-04.
- Source surfaces: `experiments/**/Cargo.toml`, `experiments/**/src/lib.rs`,
  local `README.md` files, experiment entrypoints, `research/*.md`, and the
  task progress files.
- Refresh when: topic packages move, helper-crate boundaries change, artifact
  ownership changes, or retained thesis-facing experiments change.

## Map Type And Authority

- Type: subtree navigation cache.
- Agent question: which experiment topic package, helper crate, artifact
  pattern, or provenance surface should I inspect first?
- Authority: package manifests, experiment entrypoints, local helper crates,
  retained research interpretation notes, and task progress files overrule this
  map.
- Non-authority: this file does not decide thesis claim strength, canonical data
  ownership, or which future/follow-up experiments should run.

## Role

`experiments/` contains topic-grouped experiment packages, binaries, analyses,
local helper crates, generated data, and figures.

Current boundary facts:

- Experiment code imports `symplectic` directly.
- New exploratory algorithms start here before any durable crate promotion.
- Most experiment code is script-like. Helper `.rs` files live beside the
  binary or in the smallest shared parent directory that contains all binaries
  using them.
- `src/` marks a Rust-heavy package or crate-incubator surface. In those
  packages, `src/lib.rs` should stay a thin index over named modules.
- Slow validation, broad random sweeps, and generated evidence datasets stay
  here unless they become fast crate tests.
- Research interpretation belongs in `research/`; execution-facing package
  notes can live beside the experiment.
- Thesis publication assets are copied or owned by `thesis/`; thesis
  correctness must not depend on runtime links into `experiments/`.

## Topic Packages

| Area | Current role | Related task/research surfaces |
| --- | --- | --- |
| `experiments/hko-local-maximum/` | HKO local-maximality experiments: theorem certificate tooling under `theorem/`, empirical support checks under `empirical/`, and shared topic helpers under `src/` | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/hko-local-maximum*.md`, `experiments/hko-local-maximum/README.md` |
| `experiments/sys-landscape/` | hostile sys-search landscape: random/product searches, gradient ascent, variable-`F` continuation, rejection calibration, and datascience pipeline | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/sys-landscape*.md`, `experiments/sys-landscape/datascience/README.md` |
| `experiments/regular-products/` | regular polygon product side result: broad rotated-product sweeps, pentagon empirical figures/viewer, and exact pentagon formula proof packet | `experiments/regular-products/README.md`, `thesis/rotated-regular-polygons-content.md` |
| `experiments/sys-landscape/gradient-ascent-dev/` | method-development helper package for step calibration and strategy comparison | `experiments/sys-landscape/gradient-ascent-dev/src/lib.rs` |
| `experiments/numerics/` | numerical-method validation, error bounds, algebraic exactness, Sage feasibility, unknown predicates, and KKT diagnostics | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/numerics*.md` |
| `experiments/numerics/gradient/` | separate gradient-validation package for first-order derivative checks, edge cases, and subdifferential tests | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/numerics*.md` |
| `experiments/verification/` | cross-topic correctness, minimum-set, orbit-recovery, and reusable Sage validation experiments | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/verification*.md`, `experiments/verification/sage/README.md` |
| `experiments/verification/algorithm-comparison/` | algorithm comparison, ablation, benchmark, and profiling evidence | `research/verification.md` |
| `experiments/combinatorial-cells/` | combinatorial-cell exploration: boundary characterization, cell widths, convexity, multiple crossings, omega hypothesis, and gradient-discontinuity analysis | `research/combinatorial-cells.md` |
| `experiments/crosspolytope/` | one-off crosspolytope computation and checkpointing | `research/crosspolytope.md` |
| `experiments/visualization/` | visualization data/PNG generation and browser rendering assets for negative-exploration support | `research/visualization.md` |

## Helper Crates

Topic helper crates already exist at:

- `experiments/combinatorial-cells/src/lib.rs`
- `experiments/hko-local-maximum/src/lib.rs`
- `experiments/numerics/gradient/src/lib.rs`
- `experiments/regular-products/src/lib.rs`
- `experiments/numerics/src/lib.rs`
- `experiments/verification/src/lib.rs`
- `experiments/sys-landscape/src/lib.rs`
- `experiments/sys-landscape/gradient-ascent-dev/src/lib.rs`

Current observed pattern:

- `experiments/numerics/`, `experiments/numerics/gradient/`, and
  `experiments/sys-landscape/gradient-ascent-dev/` are Rust-heavy or
  feature-incubator packages where `src/` is an appropriate package surface.
- `experiments/combinatorial-cells/`, `experiments/hko-local-maximum/`,
  `experiments/sys-landscape/`, and `experiments/verification/` expose
  package-level helpers today; keep `src/lib.rs` as an index and put real code
  in named modules.
- Script/workflow packages such as `experiments/crosspolytope/`,
  `experiments/visualization/`, and
  `experiments/verification/algorithm-comparison/` should keep helper modules
  beside the workflow that owns them.
- Some shared logic is still copied across binaries instead of extracted.
- Extraction is future/follow-up unless it unblocks retained thesis evidence,
  verification, or writing.

Current helper families:

| Helper family | Current shape |
| --- | --- |
| step-bound event logic | implemented in `experiments/sys-landscape/src/step_bound.rs` and `experiments/combinatorial-cells/src/boundary_events.rs`; shared durable home is still an open boundary |
| sys quotient / ascent scaffold | `experiments/sys-landscape/src/ascent.rs` and `datasets.rs` hold reusable landscape helpers, while individual binaries still own backend policy |
| datascience producer/table plumbing | `experiments/sys-landscape/datascience/produce/` writes producer caches; `datascience/tables/` loads/enriches/writes final tables; `datascience/methods/` reads those tables |
| exact HKO row bank and instrumented searches | `experiments/hko-local-maximum/src/exact_bank.rs` owns exact-bank constants; `instrumented_search.rs` owns local instrumented capacity helpers |
| numerics exactness helpers | `experiments/numerics/src/lib.rs` exposes the algebraic exactness spike under `src/algebraic/` |
| gradient validation helpers | `experiments/numerics/gradient/src/lib.rs` owns random-direction sampling, first-order row schemas, and small smoke-run helpers |
| verification target plumbing | `experiments/verification/src/target_pool.rs` owns target selection; `io.rs` owns run modes and shared JSONL writers |

## Artifact And Data Patterns

Generated artifacts stay beside the producer that writes them.

Freshness status is not owned by this map. The latest repo-status pass is
`tasks/references/repo-status-smoke-and-core-2026-05-31.md`: selected
commands/builds passed, but full artifact-refreshing producers were not run and
tracked experiment datasets, figures, and generated reports are not thereby
proven fresh.

Current persisted-data classes:

| Class | Current meaning |
| --- | --- |
| shared polytope catalog rows | reusable polytope records with rational geometry, source, volume, capacity, and best-sigma-style data |
| historical mirror catalogs | byte-identical copies of shared catalog content in different experiment areas observed in an earlier pass; current research notes give package-local ownership to at least `experiments/combinatorial-cells/polytopes.jsonl` |
| topic-local transient caches | local caches that store intermediate search states and are not intended as shared catalogs |
| datascience pipeline caches | maintained producer caches and final tables under `experiments/sys-landscape/datascience/`; see local `produce/`, `tables/`, and `methods/` READMEs |
| analysis outputs | experiment-owned JSONL files consumed by nearby `analyze.py` scripts |
| resume artifacts | outputs that also serve as later-run inputs or resume sources |

Historical shared-catalog mirror cluster from the old architecture pass:

| Path | Current observed role |
| --- | --- |
| `experiments/combinatorial-cells/polytopes.jsonl` | shared-catalog candidate currently read and written within combinatorial-cells |
| `experiments/sys-landscape/cache.jsonl` | mirror candidate |
| `experiments/verification/orbit-recovery/polytopes.jsonl` | mirror candidate |

These three files were byte-identical on 2026-04-16 with SHA-256
`8679b89763a10bf1380410f288845f03bcdc8e365035aa31235ff00c9cc07363`.
Byte identity is an observation, not a settled canonical-path policy. Current
code still reads these paths in some validation surfaces, but current research
notes also describe `experiments/combinatorial-cells/polytopes.jsonl` as the
canonical local cache for that package. Do not infer repo-wide canonical
ownership from either fact without checking the current task or research note.

Local-cache exception:

- `experiments/sys-landscape/variable-f-ascent/cache.jsonl` is intentionally
  local and stores intermediate search states rather than acting as part of the
  shared catalog.

Datascience pipeline exception:

- `experiments/sys-landscape/datascience/produce/shared-cache.jsonl` and
  `continuation-cache.jsonl` are maintained producer-stage caches for the
  datascience pipeline, not mirrors of the old root `cache.jsonl`.
- `experiments/sys-landscape/datascience/tables/` writes flat retained table
  files next to the table builder: one polytope-level table, one provenance
  table, and one ascent-run table. Method scripts read these retained tables
  and build method-local rectangular inputs when needed.

## Provenance Search

There is no repo-wide generated dataflow map. For artifact provenance, use
targeted search and local source inspection:

```bash
rg -n "<artifact-name>|Input Artifacts:|Output Artifacts:" experiments thesis research tasks
```

Then read the producer entrypoint, nearby analyzer, and relevant research note.

## Open Edges

- Which path, if any, should become the explicitly canonical shared polytope
  catalog?
- Which topic helper extractions are worth doing before thesis submission?
- Which experiment outputs are thesis evidence, preserved historical records,
  or future/follow-up material?
- Which cached fields can downstream consumers trust as stable contracts?
- Which datascience method rows still need current evidence packets before they
  can support thesis wording?
