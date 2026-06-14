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
- Last updated: 2026-06-13.
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

`experiments/` contains topic-grouped and target-grouped experiment packages,
binaries, analyses, local helper crates, generated data, and figures.
`AGENTS.md` owns first-entry routing; this file is an inventory and local
navigation cache.

Current boundary facts:

- Experiment code imports `symplectic` directly.
- New exploratory algorithms start in `dev-<algo>/` before durable crate
  promotion or promotion into a target evidence home.
- Organize by the axis that should move together. If one method should be
  modernized across many targets, keep the method together while it is being
  developed. If many methods answer one target question, keep the target
  together.
- Example: numerical analysis of `flow_graph` can belong in
  `experiments/dev-flow-graph/` when the analysis should move with changing
  flow-graph algorithm design, or in `experiments/numerics/` when the analysis
  should move with reusable f64/exact methodology shared across algorithms.
- Copying and heavily editing algorithms across experiments is allowed.
  Extract shared code only when multiple current users should be modernized
  together or duplication is causing concrete error or maintenance risk.
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

## Routing Homes

| Home | Route here when |
| --- | --- |
| `experiments/dev-<algo>/` | active algorithm development, diagnostics, case-finding, and representation spikes before a settled downstream evidence home exists |
| `experiments/numerics/` | f64 vs exact behavior, numerical stability, numerical-error audits, derivative/numerical validation, and reusable numerical methodology |
| `experiments/performance/` | runtime, memory, counters, profiling, and compute-budget measurement once the measured target is stable enough to profile as a target |
| `experiments/verification/` | correctness/regression checks, capacity axioms, algorithm agreement, literature values, error paths, and slower artifact-backed validation |
| `experiments/sys-datascience/` | hostile `sys` search data-science pipeline, retained tables, and method-table packets |
| topic folders | thesis-slice or topic-local producers and evidence when the local README says the topic owns them |

## Algorithm Units

An algorithm unit is a reusable or cross-experiment method or capability
developed in this project. Path-style labels below are semantic labels, not
required filesystem paths. Code may match through folders, filenames, module
paths, or snake_case symbols that are recognizably equivalent.

This inventory excludes one-off method-local code owned entirely by
`experiments/sys-datascience/methods/<packet>/README.md`. Use the method packet
README for those local algorithms. Use this table for units that multiple
experiments, thesis sections, or later cleanup decisions may care about.

| Label | Meaning | Current home | Evidence / next read |
| --- | --- | --- | --- |
| `QP` | HK2017 quadratic-program family: sigma enumeration, one-sigma KKT/QP solve, and orbit aggregation. | `crates/symplectic/src/algorithms/hk2017/`, `crates/symplectic/src/kkt/`, `crates/symplectic/src/algorithms/orbit_search.rs` | Main comparison target for correctness, numerics, performance, and flow-graph checks. |
| `QP/enumerate/unpruned` | General HK2017 sigma enumeration without transition pruning. | `crates/symplectic/src/algorithms/hk2017/enumeration.rs`, `crates/symplectic/src/algorithms/hk2017/permutations.rs`, `crates/symplectic/src/algorithms/hk2017/combinatorics.rs` | Checked against pruned enumeration in crate tests and correctness surfaces. |
| `QP/enumerate/pruned` | HK2017 sigma enumeration pruned by facet-intersection and `omega_0` transition data. | `crates/symplectic/src/algorithms/hk2017/enumeration.rs`, `crates/symplectic/src/algorithms/facet_adjacency.rs` | Current ordinary general-polytope enumeration path. |
| `QP/enumerate/billiard` | Lagrangian-product/billiard sigma enumeration simplification; feeds the same KKT/QP solve and aggregation layer. | `crates/symplectic/src/algorithms/billiard/` | Read with product/HKO/regular-product work; not a separate downstream solver stack. |
| `QP/solve/kkt/f64` | One-sigma f64 KKT/QP solve. | `crates/symplectic/src/kkt/saddle_point_solver.rs`, `crates/symplectic/src/kkt/projection_solver.rs`, `crates/symplectic/src/kkt/qp_assembly.rs` | Numerical behavior belongs in `experiments/numerics/` when it is reusable. |
| `QP/solve/kkt/exact` | One-sigma exact KKT/QP solve. | `crates/symplectic/src/kkt/rational_solver.rs`, `crates/symplectic/src/exact/orbit.rs` | Exact one-sigma solve only; not a full exact capacity search by itself. |
| `QP/capacity/f64` | Capacity route using f64 candidate solve/filtering only. | assembled from QP enumeration plus `QP/solve/kkt/f64` | Use only when f64-only ambiguity policy is intended by the caller or experiment. |
| `QP/capacity/fallback` | Capacity route using f64 solve/filtering with exact fallback for candidates needed by the selected guarantee mode. | `crates/symplectic/src/algorithms/orbit_search.rs` | Current ordinary crate capacity style via `OrbitGuaranteeMode`. |
| `QP/capacity/certified` | f64 candidate fast path with exact-certified returned capacity, minimizers, and optional gap-window orbit set. | `aggregate_certified_orbits_with_dual_vertices_exact` | Candidate generation is still the f64 fast path; returned orbit-set values are exact. |
| `QP/capacity/exact` | Reserved label for a full exact/CAS-backed QP capacity search. | theorem/Sage-style routes when present, not the current ordinary crate path | Do not use this label for current f64-fast-path crate aggregation. |
| `QP/recover-orbit` | Recover geometric Reeb trajectory data from QP/HK2017 KKT output. | `crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs`, `crates/symplectic/src/geom/reeb_trajectory.rs` | Validated in `experiments/verification/orbit-recovery/`. |
| `FG` | CH2021 flow-graph family. | `crates/symplectic/src/algorithms/flow_graph/`, `experiments/dev-flow-graph/` | First read `crates/symplectic/src/algorithms/flow_graph/README.md`. |
| `FG/closed-word/f64` | f64 flow-graph closed-word/tube construction and fixed-point diagnostics. | `crates/symplectic/src/algorithms/flow_graph/mod.rs` | Keep algorithm-local while design/failure taxonomy is changing. |
| `FG/closed-word/exact` | Exact flow-graph closed-word/tube resolution. | `crates/symplectic/src/algorithms/flow_graph/exact.rs` | Used by flow-graph exact and f64-resolution tests. |
| `FG/capacity/f64` | f64 flow-graph capacity route. | `crates/symplectic/src/algorithms/flow_graph/mod.rs` | Development evidence, not an exact certificate by itself. |
| `FG/capacity/fallback` | f64 flow-graph route with exact resolution of problematic closed words. | `capacity_f64` in `flow_graph/mod.rs` plus exact closed-word code | Current live flow-graph README defines the accepted meaning. |
| `FG/capacity/exact` | Exact flow-graph capacity search. | `crates/symplectic/src/algorithms/flow_graph/exact.rs` | Compared to certified QP scalar capacity in flow-graph tests. |
| `vol/4d/f64` | 4D volume from known incidence using f64. | `crates/euclidean-polytopes/src/volume.rs` | Ordinary Euclidean geometry, not symplectic capacity. |
| `vol/4d/exact` | 4D volume from known incidence exactly. | `crates/euclidean-polytopes/src/volume.rs` | Used when exact geometry/volume support matters. |
| `vol/facet-3d/f64` | Facet 3-volume from known incidence using f64. | `crates/euclidean-polytopes/src/volume.rs`, `crates/symplectic/src/geom/facet_volume.rs` | Supports volume derivatives and geometry checks. |
| `geom/random-dual-vertices` | Candidate random dual-vertex generation and accepted-fixture policy. | `crates/euclidean-polytopes/src/random.rs`, `crates/symplectic/src/random.rs`, experiment producers | Euclidean sampler proposes candidates; acceptance and row policy live with callers. |
| `geom/polar-and-incidence` | Exact polar vertices, vertex-facet incidence, faces, and facet-intersection data. | `crates/euclidean-polytopes/`, `crates/symplectic/src/geom/vertex_enumeration/` | Geometry source for QP pruning, volume, and validation surfaces. |
| `data/polytope-records` | Reusable JSONL polytope/capacity/orbit record helpers. | `crates/symplectic/src/database.rs`, `crates/symplectic/src/dataset.rs` | Callers choose cache paths; no repo-wide canonical catalog is implied here. |
| `data/sys-datascience-tables` | Maintained hostile-search producer caches and retained flat tables. | `experiments/sys-datascience/produce/`, `experiments/sys-datascience/tables/` | Method-local algorithms consume these via `experiments/sys-datascience/methods/`. |
| `HKO/symmetry-quotient-certificate` | HKO theorem-local symmetry, quotient, and feasible-section certificate machinery. | `experiments/hko-local-maximum/theorem/`, `research/hko-local-maximum*.md`, `formal/hko-feasible-section-upper-branches.tex` | HKO-specific; not a generic symmetry-action library unless later promoted. |

## Lenses And Homes

A lens is the question being asked of an algorithm unit. A home is the package
where related code, data, notes, and evidence should move together. Folder names
such as `experiments/dev-<algo>/`, `experiments/<topic>/`, and
`experiments/sys-datascience/` are homes, not lenses.

| Lens | Question | Usual home |
| --- | --- | --- |
| `library` | What is the clean reusable implementation/API for non-instrumented callers? | `crates/**`, with crate tests for cheap durable checks |
| `numerics` | How do f64 decisions, tolerances, ambiguity handling, and exact/reference behavior compare? | `experiments/numerics/`, unless the numerical question is still coupled to active algorithm design |
| `performance` | What are the runtime, memory, counters, pruning wins, and scaling behavior? | `experiments/performance/` |
| `correctness` | Do outputs or intermediate invariants satisfy the intended mathematical or software contract? | `experiments/verification/` for reusable experiment-level evidence, or crate tests when cheap and durable |
| `data` | Which reusable records, caches, schemas, and retained tables are produced or consumed? | `crates/**` record helpers, `experiments/sys-datascience/produce/`, and `experiments/sys-datascience/tables/` |
| `thesis-support` | Which evidence or machinery supports one theorem, figure, side result, or thesis slice? | the owning topic folder, e.g. `experiments/hko-local-maximum/` or `experiments/regular-products/` |
| `formal` | Which proof-facing statements, exact derivations, or theorem-local certificates need formal tracking? | `formal/`, with links back from the owning experiment or research note |

Routing rule: route by the question that should own future changes. Reusable
algorithm behavior belongs in the cross-cutting homes above. If an algorithm
family is still moving and its design notes, diagnostics, numerics, correctness
checks, and performance probes are tightly coupled, keep them together in
`experiments/dev-<algo>/` until a cleaner split is worth the churn. If the
question is a theorem, thesis topic, or retained hostile-search dataset question,
keep it with the topic or data pipeline even when it uses reusable algorithms.
For example, finite-difference checks of a derivative formula can be
`correctness`, but "does gradient ascent increase `sys` on this search class?"
belongs with `experiments/dev-gradient-ascent/` or `experiments/sys-datascience/`
depending on whether the owner is method development or retained-dataset
analysis.

## Topic Packages

| Area | Current role | Related task/research surfaces |
| --- | --- | --- |
| `experiments/hko-local-maximum/` | HKO local-maximality experiments: theorem certificate tooling under `theorem/`, empirical support checks under `empirical/`, and shared topic helpers under `src/` | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/hko-local-maximum*.md`, `experiments/hko-local-maximum/README.md` |
| `experiments/algorithm-comparison/` | README-only routing and reasoning note for cross-algorithm comparisons; no active commands or evidence artifacts | `experiments/algorithm-comparison/README.md`, `experiments/dev-quadratic-program/README.md`, `experiments/performance/README.md`, `experiments/numerics/README.md`, `experiments/verification/README.md` |
| `experiments/dev-quadratic-program/` | README-only coordination packet for QP/HK2017 library-surface and cleanup questions before code or evidence has a better home | `experiments/dev-quadratic-program/README.md`, `experiments/MAP.md` section `Algorithm Units`, `crates/symplectic/src/algorithms/hk2017/`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| `experiments/dev-gradient-ascent/` | active top-level development packet for a heuristic gradient-ascent method for nonsmooth high-dimensional `sys(a)`; owns question ledger, schema-smoke artifacts, and future method-development probes before promotion | `experiments/dev-gradient-ascent/README.md`, `experiments/sys-datascience/README.md`, `research/sys-first-order-local-behavior.md` |
| `experiments/dev-flow-graph/` | active flow-graph algorithm-development packet: frontier counts, endpoint/closed-word representation spikes, case-finding, mismatch visualization, and unresolved-word diagnostics before promotion into numerics, performance, or verification | `experiments/dev-flow-graph/README.md`, `crates/symplectic/src/algorithms/flow_graph/README.md`, `tasks/current-state.md`, `tasks/planning-notes.md` |
| `experiments/dev-f64-capacity/` | active pure-`f64` capacity-development packet: generated and retained datascience-style scans, validation/capacity policy comparison, product preprocessing diagnostics, and promotion-readiness evidence before library or `sys-datascience` integration | `experiments/dev-f64-capacity/README.md`, `experiments/performance/README.md`, `experiments/sys-datascience/README.md` |
| `experiments/sys-landscape/` | hostile sys-search landscape legacy and producer surfaces: random/product searches, gradient ascent, variable-`F` continuation, and rejection calibration | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/sys-landscape*.md`, `experiments/sys-datascience/README.md` |
| `experiments/sys-datascience/` | maintained hostile `sys` search data-science pipeline: producer caches, retained tables, and method packets for the thesis method table | `experiments/sys-datascience/README.md`, `experiments/sys-datascience/produce/README.md`, `experiments/sys-datascience/tables/README.md`, `experiments/sys-datascience/methods/README.md` |
| `experiments/regular-products/` | regular polygon product side result: broad rotated-product sweeps, pentagon empirical figures/viewer, and exact pentagon formula proof packet | `experiments/regular-products/README.md`, `thesis/rotated-regular-polygons-content.md` |
| `experiments/local-sys-methods/` | narrow smoke packet for local `sys(a0 + t d)` prediction diagnostics against HK2017 recomputation; method-development code, not thesis evidence | `experiments/local-sys-methods/README.md` |
| `experiments/numerics/` | single-threaded numerical error audit: structured JSONL observations, f64-vs-oracle summaries, and generated reports for KKT variables and predicates | `experiments/numerics/README.md`, `tasks/current-state.md`, `tasks/planning-notes.md` |
| `experiments/verification/` | experiment-level correctness and regression evidence, minimum-set validation, orbit recovery, and reusable Sage validation experiments | `tasks/current-state.md`, `tasks/planning-notes.md`, `research/verification*.md`, `experiments/verification/README.md`, `experiments/verification/sage/README.md` |
| `experiments/performance/` | shared runtime and memory profiling targets, reusable measurement practice, and post-processing scripts; generated outputs normally go under `/tmp` | `experiments/performance/README.md` |
| `experiments/combinatorial-cells/` | combinatorial-cell exploration: boundary characterization, cell widths, convexity, multiple crossings, omega hypothesis, and gradient-discontinuity analysis | `research/combinatorial-cells.md` |
| `experiments/crosspolytope/` | one-off crosspolytope computation and checkpointing | `research/crosspolytope.md` |
| `experiments/visualization/` | visualization data/PNG generation and browser rendering assets for negative-exploration support | `research/visualization.md` |

## Helper Crates

Topic helper crates already exist at:

- `experiments/combinatorial-cells/src/lib.rs`
- `experiments/hko-local-maximum/src/lib.rs`
- `experiments/dev-gradient-ascent/src/lib.rs`
- `experiments/regular-products/src/lib.rs`
- `experiments/numerics/src/lib.rs`
- `experiments/verification/src/lib.rs`
- `experiments/sys-landscape/src/lib.rs`
- `experiments/dev-f64-capacity/src/lib.rs`

Current observed pattern:

- `experiments/dev-gradient-ascent/`, `experiments/dev-f64-capacity/`,
  `experiments/numerics/`, and `experiments/numerics/gradient/` are Rust-heavy
  or feature-incubator packages where `src/` is an appropriate package surface.
- `experiments/combinatorial-cells/`, `experiments/hko-local-maximum/`,
  `experiments/sys-landscape/`, and `experiments/verification/` expose
  package-level helpers today; keep `src/lib.rs` as an index and put real code
  in named modules.
- Script/workflow packages such as `experiments/crosspolytope/`,
  `experiments/visualization/` should keep helper modules beside the workflow
  that owns them.
- Some shared logic is still copied across binaries instead of extracted.
- Extraction is future/follow-up unless it unblocks retained thesis evidence,
  verification, or writing.

Current helper families:

| Helper family | Current shape |
| --- | --- |
| step-bound event logic | implemented in `experiments/sys-landscape/src/step_bound.rs` and `experiments/combinatorial-cells/src/boundary_events.rs`; shared durable home is still an open boundary |
| sys quotient / ascent scaffold | `experiments/sys-landscape/src/ascent.rs` and `datasets.rs` hold reusable landscape helpers, while individual binaries still own backend policy |
| datascience producer/table plumbing | `experiments/sys-datascience/produce/` writes producer caches; `experiments/sys-datascience/tables/` loads/enriches/writes final tables; `experiments/sys-datascience/methods/` reads those tables |
| exact HKO row bank and instrumented searches | `experiments/hko-local-maximum/src/exact_bank.rs` owns exact-bank constants; `instrumented_search.rs` owns local instrumented capacity helpers |
| numerics audit helpers | `experiments/numerics/src/lib.rs` indexes the audit runner, event schema, argument parsing, and output-directory helpers |
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
| datascience pipeline caches | maintained producer caches and final tables under `experiments/sys-datascience/`; see local `produce/`, `tables/`, and `methods/` READMEs |
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

- `experiments/sys-datascience/produce/shared-cache.jsonl` and
  `continuation-cache.jsonl` are maintained producer-stage caches for the
  datascience pipeline, not mirrors of the old root `cache.jsonl`.
- `experiments/sys-datascience/tables/` writes flat retained table
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
