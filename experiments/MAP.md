<!--
Purpose: navigation cache for experiment packages and experiment-owned data.
Context: this map helps agents find topic packages, helper crates, and
experiment artifact patterns. It is descriptive, not a task tracker.
-->

# Experiments Map

## Status

- State: split from the old root `ARCHITECTURE.md`.
- Last updated: 2026-04-25.
- Source surfaces: `experiments/**/Cargo.toml`, `experiments/**/src/lib.rs`,
  experiment entrypoints, `research/*.md`, `tasks/*.md`, and
  `.agents/skills/experiment-conventions/`.
- Refresh when: topic packages move, helper-crate boundaries change, artifact
  ownership changes, or retained thesis-facing experiments change.

## Role

`experiments/` contains topic-grouped experiment packages, binaries, analyses,
local helper crates, generated data, and figures.

Current boundary facts:

- Experiment code imports `symplectic` directly.
- New exploratory algorithms start here before any durable crate promotion.
- Slow validation, broad random sweeps, and generated evidence datasets stay
  here unless they become fast crate tests.
- Research interpretation belongs in `research/`; execution-facing package
  notes can live beside the experiment.
- Thesis publication assets are copied or owned by `thesis/`; thesis
  correctness must not depend on runtime links into `experiments/`.

## Topic Packages

| Area | Current role | Related task/research surfaces |
| --- | --- | --- |
| `experiments/hko-local-maximum/` | HKO local-maximality experiments, exact-Clarke route, perturbation and neighborhood evidence | `tasks/hko.md`, `research/hko-local-maximum*.md` |
| `experiments/sys-landscape/` | hostile sys-search landscape, product searches, gradient ascent, data-science methods, pentagon rotation formula | `tasks/landscape.md`, `research/sys-landscape*.md` |
| `experiments/numerics/` | numerical-method validation, exactness comparisons, gradient and error-bound experiments | `tasks/numerics.md`, `research/numerics*.md` |
| `experiments/verification/` | cross-topic validation experiments and reusable verification evidence | `tasks/reproducibility.md`, `research/verification*.md` |
| `experiments/combinatorial-cells/` | combinatorial-cell exploration and future/follow-up landscape work | `research/combinatorial-cells.md` |
| `experiments/crosspolytope/` | crosspolytope computation and checkpointing | `research/crosspolytope.md` |
| `experiments/visualization/` | visualization outputs and negative-exploration support | `research/visualization.md` |

## Helper Crates

Topic helper crates already exist at:

- `experiments/combinatorial-cells/src/lib.rs`
- `experiments/hko-local-maximum/src/lib.rs`
- `experiments/numerics/gradient/src/lib.rs`
- `experiments/sys-landscape/src/lib.rs`

Current observed pattern:

- Topic helper crates are the natural place for shared experiment-local code.
- Some helper crates are still thin.
- Some shared logic is still copied across binaries instead of extracted.
- Extraction is future/follow-up unless it unblocks retained thesis evidence,
  verification, or writing.

Current helper families recorded in earlier discovery:

| Helper family | Current shape |
| --- | --- |
| step-bound event logic | repeated across sys-landscape and combinatorial-cells; common logic has a shared-home candidate in `experiments/sys-landscape/src/lib.rs` |
| sys quotient / ascent scaffold | arithmetic repeats across multiple ascent binaries, while backend policy varies per binary |
| orbit-enumeration wrappers | repeated in numerics binaries while `experiments/numerics/gradient/src/lib.rs` remains mostly empty |
| solver instrumentation helpers | shared core exists, but result payloads differ enough that the boundary is still open |

## Artifact And Data Patterns

Generated artifacts stay beside the producer that writes them.

Current persisted-data classes:

| Class | Current meaning |
| --- | --- |
| shared polytope catalog rows | reusable polytope records with rational geometry, source, volume, capacity, and best-sigma-style data |
| mirror catalogs | byte-identical copies of shared catalog content in different experiment areas; no canonical path is settled |
| topic-local transient caches | local caches that store intermediate search states and are not intended as shared catalogs |
| analysis outputs | experiment-owned JSONL files consumed by nearby `analyze.py` scripts |
| resume artifacts | outputs that also serve as later-run inputs or resume sources |

Current observed shared-catalog mirror cluster from the old architecture pass:

| Path | Current observed role |
| --- | --- |
| `experiments/combinatorial-cells/polytopes.jsonl` | shared-catalog candidate currently read and written within combinatorial-cells |
| `experiments/sys-landscape/cache.jsonl` | mirror candidate |
| `experiments/verification/orbit-recovery/polytopes.jsonl` | mirror candidate |

These three files were byte-identical on 2026-04-16 with SHA-256
`8679b89763a10bf1380410f288845f03bcdc8e365035aa31235ff00c9cc07363`.
Byte identity is an observation, not a settled canonical-path policy.

Local-cache exception:

- `experiments/sys-landscape/variable-f-ascent/cache.jsonl` is intentionally
  local and stores intermediate search states rather than acting as part of the
  shared catalog.

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
