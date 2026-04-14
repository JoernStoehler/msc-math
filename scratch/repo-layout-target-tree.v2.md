# Repo Layout Target Tree v2

This is the replacement target description after the original target file was deleted.
Its job is to state the intended destination clearly and preserve the reasoning behind the decisions,
so later agents do not have to infer policy from the partially migrated tree.

## Purpose

The repo is being split by role:

- `research/` holds thinking packets for Jörn as lead researcher.
- `experiments/` holds runnable engineering packets: binaries, scripts, datasets, figures.
- `library/` holds reusable, validated Rust code.
- `formal/` holds developer-facing mathematical writeups and proof packets.
- `thesis/` remains the sealed publication workspace.

The point of the split is to stop mixing:

- research notes with runnable experiment artifacts,
- reusable library code with one-off experiment code,
- durable formalization with publication prose,
- current operational docs with historical or scratch notes.

## Decision Notes

### 1. `library/` is one crate, and `database` folds into it

Reason:
- keeping a separate `database` crate while also moving to a single top-level `library/` area makes the boundary ambiguous
- the current `database` code is not a separate product; it is infrastructure used by the library and many experiments
- the intended stable reusable software surface is one Rust library crate, not a library-plus-helper-crate bundle

Decision:
- final reusable Rust crate is `library/`
- shared database code lives at `library/src/database.rs`
- experiments import it through the library crate

### 2. Cargo workspace root belongs at repo root

Reason:
- after the split, `library/` and `experiments/` are siblings
- keeping the workspace root under old `crates/` would preserve the old conceptual center after the repo has moved on
- repo-root workspace makes the build topology match the actual layout

Decision:
- repo-root `Cargo.toml` becomes the workspace manifest
- family crates under `experiments/` and the `library/` crate are workspace members

### 2a. Experiment data is owned locally, not by one giant global cache

Reason:
- path refactors are cheap compared to long-term semantic confusion
- one giant merged cache file hides ownership and encourages unrelated experiments to mutate the same artifact
- experiments often need to load multiple sources, filter them, and write their own derived data
- reusable loading logic can still support multi-file inputs without forcing one canonical mutable dataset

Decision:
- each experiment or family owns its own `.jsonl` file or files
- experiments may load multiple `.jsonl` input files
- experiments filter/merge the inputs they need
- experiments may populate or refresh their owned `.jsonl` file from loaded input files
- experiments may then add newly computed values to their owned `.jsonl` file
- merges are fieldwise, not row-priority-based
- missing or unknown fields may be filled from a concrete value in another input row for the same polytope
- if two input rows give conflicting concrete values for a field that should be unique for that polytope, the merge must fail loudly so the caller is informed early
- metadata/provenance may be accumulated only when that does not hide a conflict in concrete data
- `data/polytopes.jsonl` is not part of the target architecture and should be deleted rather than retained as a canonical shared cache
- whether multi-file merge support lives centrally in `library/src/database.rs` or partly in experiment code is an implementation detail, not a target-layout decision

### 3. `formal/` is the developer math root, but migration keeps math whole-file-first

Reason:
- the current repo already has many colocated `math.tex` packets with real labels and cross-references
- immediately re-splitting them thematically would risk content loss, duplicate lemmas, and broken references
- the migration needs a content-preserving move first, not a semantic rewrite

Decision:
- `formal/` becomes the canonical developer math area
- migration moves existing `math.tex` packets into `formal/` largely as-is
- later work may reorganize them thematically, but that is explicitly post-migration
- a directory that did not have a pre-migration `math.tex` does not gain a new
  `formal/.../*.tex` packet during migration
- current explicit no-source example: `experiments/numerics/gradient/numerics-edge-cases/`
  has experiment and research packets but no historical `math.tex`, so migration
  does not require `formal/numerics/gradient/numerics-edge-cases.tex`

### 3a. `formal/` owns its own build support files

Reason:
- once `formal/` is the canonical developer math root, it must not depend on stranded support files under old `crates/`
- the current developer math build already depends on `crates/.latexmkrc` and `crates/bibliography.bib`

Decision:
- `formal/.latexmkrc` is the developer math build config
- `formal/bibliography.bib` is the developer math bibliography
- after migration, `formal/` should build without depending on `crates/main.tex`, `crates/.latexmkrc`, or `crates/bibliography.bib`

### 4. `research/` gets the old `logbook.md` content

Reason:
- logbooks are not runnable artifacts
- they are the design/interpretation history for experiments
- keeping them inside experiment folders continues the current conflation between engineering packets and research notes

Decision:
- every experiment directory that owns a runnable packet entrypoint (`run.rs` during migration, `main.rs` in the target) gets a paired `research/.../design/*.md` note
- `research/.../design/*.md` may also contain notes with no runnable packet pair; current explicit examples are `research/hko-local-maximum/design/subdifferential-lp.md`, `research/sys-landscape/design/witness-search-program.md`, `research/sys-landscape/design/imported-sys-search-chatgpt-pro-extended-2026-04-13.md`, `research/verification/design/algorithm-comparison/profiling.md`, and `research/combinatorial-cells/design/gradient-discontinuity.md`
- some of those notes correspond to analysis-only experiment directories with scripts but no runnable packet entrypoint; current explicit examples are `experiments/hko-local-maximum/subdifferential-lp/`, `experiments/combinatorial-cells/gradient-discontinuity/`, and `experiments/verification/algorithm-comparison/profiling/`
- some notes are research-only and have no experiment directory pair; current explicit examples are `research/sys-landscape/design/witness-search-program.md` and `research/sys-landscape/design/imported-sys-search-chatgpt-pro-extended-2026-04-13.md`
- old `logbook.md` material lands under `research/<family>/design/*.md`
- `agenda.md` and `interpretation.md` belong in the target, but creating them is later work

### 5. `combinatorial-cells` stays its own family

Reason:
- it is a real experiment family with multiple packets and a coherent subject
- folding it into `sys-landscape` would hide a real topic boundary and make the migration plan invent a grouping that was not agreed

Decision:
- `research/combinatorial-cells/`
- `experiments/combinatorial-cells/`

### 6. `crosspolytope` and `visualization` each get their own paired buckets

Reason:
- a generic `standalone/` bucket throws away useful meaning
- `crosspolytope` and `visualization` are different topics, not one family
- each has both runnable artifacts and research context

Decision:
- `research/crosspolytope/` + `experiments/crosspolytope/`
- `research/visualization/` + `experiments/visualization/`

### 7. Shared plotting config stays shared

Reason:
- many experiment scripts already rely on the same plotting conventions
- per-family copies would create drift for no benefit

Decision:
- shared file lives at `experiments/figure_config.py`

### 8. Not everything gets assigned now

Reason:
- some files are genuine leftovers or special cases
- forcing every leftover into the new layout during migration would invent policy instead of documenting it

Decision:
- `dev-tube/`, `AGENTS.new.rules.md`, and `paranoia-numerics-report.md` are follow-up decisions, not silent target assignments
- any other path not explicitly assigned in this target description must not be assigned by guessing

### 9. Infrastructure roots stay top-level

Reason:
- agent/runtime infrastructure is cross-cutting and should not be folded into research, experiments, library, or formal material
- the migration changes research/code/formal layout, not the basic operational tooling roots

Decision:
- `.agents/`, `.codex/`, `.devcontainer/`, `scripts/`, `feedback/`, and `codex-cloud.md` remain top-level
- `.codex/reference/` is the target home for the moved Codex CLI reference note

## Intended Target Tree

```text
AGENTS.md
TASKS.md
RESULTS.md
Cargo.toml
.agents/
.codex/
  reference/
    codex-cli-config-reference.md
.devcontainer/
feedback/
scripts/
codex-cloud.md
data/
  ...
scratch/
  ...

research/
  hko-local-maximum/
    agenda.md
    interpretation.md
    design/
      gradient-analysis.md
      facet-splitting.md
      lagrangian-boundary.md
      perturbation-neighborhood.md
      second-order.md
      cut-and-ascent.md
      subdifferential-lp.md
      ...
  sys-landscape/
    agenda.md
    interpretation.md
    design/
      witness-search-program.md
      random-sample.md
      random-product-sample.md
      rotated-regular-products.md
      gradient-ascent-general.md
      gradient-ascent-products.md
      variable-f-ascent.md
      gradient-ascent-dev/
        step-calibration.md
        strategy-comparison.md
      ...
  combinatorial-cells/
    agenda.md
    interpretation.md
    design/
      boundary-characterization.md
      cell-widths.md
      convexity.md
      gradient-discontinuity.md
      multiple-crossings.md
      omega-hypothesis.md
      ...
  verification/
    agenda.md
    interpretation.md
    design/
      correctness.md
      orbit-recovery.md
      algorithm-comparison/
        ablation.md
        benchmark.md
        profiling.md
      ...
  numerics/
    agenda.md
    interpretation.md
    design/
      error-bounds.md
      kkt-inertia.md
      q-error.md
      unknown-predicates.md
      gradient/
        numerics.md
        numerics-subdifferential.md
        numerics-edge-cases.md
      ...
  crosspolytope/
    agenda.md
    interpretation.md
    design/
      main.md
  visualization/
    agenda.md
    interpretation.md
    design/
      main.md

experiments/
  figure_config.py
  hko-local-maximum/
    Cargo.toml
    src/
      lib.rs
    gradient-analysis/
      main.rs
      analyze.py
      ...
    facet-splitting/
      main.rs
      analyze.py
      ...
    lagrangian-boundary/
      main.rs
      probe.rs
      analyze.py
      ...
    perturbation-neighborhood/
      main.rs
      analyze.py
      data/
        ...
      ...
    second-order/
      main.rs
      analyze.py
      ...
    cut-and-ascent/
      main.rs
      ...
    subdifferential-lp/
      ...
  sys-landscape/
    Cargo.toml
    src/
      lib.rs
    random-sample/
      main.rs
      analyze.py
      ...
    random-product-sample/
      main.rs
      analyze.py
      ...
    rotated-regular-products/
      main.rs
      analyze.py
      ...
    gradient-ascent-general/
      main.rs
      analyze.py
      data/
        ...
      ...
    gradient-ascent-products/
      main.rs
      analyze.py
      data/
        ...
      ...
    variable-f-ascent/
      main.rs
      analyze.py
      ...
    gradient-ascent-dev/
      Cargo.toml
      src/
        lib.rs
      step-calibration/
        main.rs
      strategy-comparison/
        main.rs
  combinatorial-cells/
    Cargo.toml
    src/
      lib.rs
    boundary-characterization/
      main.rs
      analyze.py
      ...
    cell-widths/
      main.rs
      analyze.py
      ...
    convexity/
      main.rs
      analyze.py
      ...
    gradient-discontinuity/
      analyze.py
      ...
    multiple-crossings/
      main.rs
      analyze.py
      ...
    omega-hypothesis/
      main.rs
      analyze.py
      ...
  verification/
    Cargo.toml
    correctness/
      main.rs
      ...
    orbit-recovery/
      main.rs
      analyze.py
      ...
    algorithm-comparison/
      Cargo.toml
      ablation/
        main.rs
        analyze.py
        ...
      benchmark/
        main.rs
        profile.rs
        analyze.py
        profiling/
          ...
      profiling/
        analyze.py
        ...
  numerics/
    Cargo.toml
    src/
      lib.rs
    error-bounds/
      main.rs
      collect_poly.rs
      analyze.py
      testdata/
        ...
      ...
    kkt-inertia/
      main.rs
      ...
    q-error/
      main.rs
      ...
    unknown-predicates/
      main.rs
      analyze.py
      ...
    gradient/
      Cargo.toml
      src/
        lib.rs
      numerics/
        main.rs
        analyze.py
        ...
      numerics-subdifferential/
        main.rs
        analyze.py
        ...
      numerics-edge-cases/
        main.rs
        analyze.py
        ...
  crosspolytope/
    Cargo.toml
    main/
      main.rs
      ...
  visualization/
    Cargo.toml
    main/
      main.rs
      viz/
        ...

library/
  Cargo.toml
  src/
    lib.rs
    constants.rs
    dataset.rs
    derivatives.rs
    random.rs
    database.rs
    geom/
      ...
    kkt/
      ...
    algorithms/
      ...
  tests/
    ...
  benches/
    ...

formal/
  main.tex
  preamble.tex
  bibliography.bib
  .latexmkrc
  library/
    main.tex
    geom.tex
    kkt.tex
    algorithms.tex
  hko-local-maximum/
    ...
  sys-landscape/
    ...
  combinatorial-cells/
    ...
  verification/
    ...
  numerics/
    ...
  crosspolytope/
    main.tex
  visualization/
    main.tex

thesis/
  main.tex
  preamble.tex
  bibliography.bib
  algorithms.tex
  tube-algorithm.tex
  proofs.tex
  experiments.tex
  assets/
    ...
  notes/
    ...
  check-build.sh
  label-map.py
  lookup.sh
  .latexmkrc

papers/
  AGENTS.md
  citation-index.md
  .gitignore
  hk2017/
    ...
  hko2024/
    ...
  bblm2023/
    ...
  ch2021/
    ...
  bgl2005/
    BenziGolubLiesen2005.pdf
    ...
  chls2007/
    CHLS2007.pdf
    ...
```

## Explicit Non-Goals During Migration

- do not rewrite research prose
- do not rewrite proofs
- do not split formal material semantically
- do not improve code style beyond what path repair forces
- do not assign unmatched leftovers by guessing

## Current Fact

The repo is only partially migrated toward this target.
This file states the intended destination, not the current finished state.
