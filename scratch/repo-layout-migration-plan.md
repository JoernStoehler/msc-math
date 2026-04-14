# Repo Layout Migration Plan

This file is the execution handoff after a partial migration pass already happened.
The move-map relocations in `scratch/migration/move-map.tsv` were executed. The repo is intentionally left before the high-volume repair phase.

## Current Status

Completed in the partial move pass:

- New top-level areas exist: `experiments/`, `research/`, `formal/`, `library/`, `.codex/reference/`.
- The move-map relocations that were executed are:
  - `crates/library -> library`
  - `crates/exp-hko-local-maximum -> experiments/hko-local-maximum`
  - `crates/exp-sys-landscape -> experiments/sys-landscape`
  - `crates/exp-combinatorial-cells -> experiments/combinatorial-cells`
  - `crates/dev-capacity-validation -> experiments/verification`
  - `crates/dev-numerical-analysis -> experiments/numerics`
  - `crates/dev-gradient -> experiments/numerics/gradient`
  - `crates/dev-gradient-ascent -> experiments/sys-landscape/gradient-ascent-dev`
  - `crates/dev-algorithm-comparison -> experiments/verification/algorithm-comparison`
  - `crates/crosspolytope -> experiments/crosspolytope/main`
  - `crates/visualization -> experiments/visualization/main`
  - `crates/figure_config.py -> experiments/figure_config.py`
  - `crates/library/src/math-preamble.tex -> formal/preamble.tex`
  - `docs/sys-search-program-2026-04-13.md -> research/sys-landscape/design/witness-search-program.md`
  - `docs/imported/sys-search-chatgpt-pro-extended-2026-04-13.md -> research/sys-landscape/design/imported-sys-search-chatgpt-pro-extended-2026-04-13.md`
  - `docs/codex-cli-config-reference.md -> .codex/reference/codex-cli-config-reference.md`
  - `papers/BenziGolubLiesen2005.pdf -> papers/bgl2005/BenziGolubLiesen2005.pdf`
  - `papers/CHLS2007.pdf -> papers/chls2007/CHLS2007.pdf`
- Experiment `logbook.md` files were relocated into `research/<family>/design/*.md`.
- Developer `math.tex` files were relocated into `formal/` whole-file-first.
- `crates/database/src/lib.rs` was moved to `library/src/database.rs`.
- `crates/figure_config.py` was moved to `experiments/figure_config.py`.
- Loose paper PDFs were moved into `papers/bgl2005/` and `papers/chls2007/`.
- Verification inventories were written under `scratch/migration/`.
- The old shared database file is still physically at repo-root `data/polytopes.jsonl`, but it is not part of the target architecture and should be deleted during the migration endgame rather than kept as a canonical shared cache.

Not yet done:

- final historical/provenance audit for remaining `crates/...` mentions
- semantic validation and shim cleanup

## Context Files For Later Agents

Read these first:

1. `scratch/repo-layout-target-tree.v2.md`
2. `scratch/migration/move-map.tsv`
3. `scratch/migration/content-inventory.json`
4. `scratch/migration/rust-inventory.json`
5. `scratch/migration/tex-label-inventory.txt`
6. `scratch/migration/figure-data-inventory.txt`
7. `scratch/migration/stale-path-allowlist.txt`

The current partial tree under `experiments/`, `research/`, `formal/`, and `library/` is also ground truth.

## Success Condition

This migration is done only when all of these are true:

- the repo layout matches `scratch/repo-layout-target-tree.v2.md`
- repo-root Cargo build/test/clippy commands work from the new layout
- developer math builds from `formal/` without dependency on the old `crates/` math root
- thesis build still works
- migration inventories and exception checks pass
- no future agent needs to invent path policy, dataset policy, or known exception handling to continue thesis work safely

Current verified progress on 2026-04-14:
- repo-root Cargo workspace works
- canonical packet entrypoints have been renamed to `main.rs`
- live Python/shell/tooling path repair is done
- `formal/main.tex`, `formal/.latexmkrc`, and `formal/bibliography.bib` now exist and `cd formal && latexmk` succeeds
- multi-file JSONL merge semantics are implemented in `library/src/database.rs`
- affected binaries now write only to owned family/experiment caches
- owned caches have been materialized at `experiments/sys-landscape/cache.jsonl`, `experiments/combinatorial-cells/polytopes.jsonl`, and `experiments/verification/orbit-recovery/polytopes.jsonl`
- repo-root `data/polytopes.jsonl` has been deleted

## Evidence Rules

Treat every claim in execution as one of these:

- repo fact: verified from the current tree, inventories, or command output
- target decision: stated in `scratch/repo-layout-target-tree.v2.md`
- implementation detail: allowed to vary as long as it preserves the target decision and verification gates

Do not upgrade an implementation choice into target policy unless it is written in the target file.

## Live-vs-Historical Path Policy

Before editing a stale `crates/` or `docs/` reference, classify it:

- live: command, script, build input, import path, operational instruction, or comment that claims current behavior
- historical: dated note, retrospective logbook entry, provenance note, or copied record of past commands/results
- ambiguous: not safely classifiable from local context

Rules:

- live references must be repaired during migration
- historical references may remain unchanged if they are clearly historical
- ambiguous references block that local edit until a separate audit resolves them

## Execution Protocol

From this point on, the next work is no longer low-volume. It will touch many files.

Use this order:

1. Repair Cargo and Rust first.
2. Repair Python, shell, and operational docs next.
3. Repair TeX roots and include paths after the code paths are stable.
4. Run semantic validation only after build-level failures are gone.
5. Remove temporary shims only after every check passes.

Do not add new research prose, new formal prose, or non-migration refactors in the same branch.

For each phase:

- one local owner keeps the plan and decides whether delegate output is usable
- subagents may gather evidence or make bounded disjoint edits
- local owner verifies subagent findings against the repo before acting on them
- if a phase changes the expected inputs of a later phase, update this file before handing off

## Delegation Protocol

Use subagents only for bounded work with observable outputs.

Good delegate tasks here:

- read-only mismatch scans
- stale-live-path classification in a named file set
- Cargo/bin manifest consistency checks
- formal label/reference consistency checks
- disjoint code edits in one subtree

Rules:

- immediate blocking design decisions stay local
- do not trust subagent conclusions without local verification
- trust only quotes, exact file paths, and command outputs until locally checked
- if two delegate runs disagree, stop using delegates on that question and resolve it locally
- after any meaningful delegate result, re-check whether the current phase objective or exception list changed

## Failure Routing And Replanning

If a phase fails, do this instead of pushing forward:

- Cargo/Rust failure caused by unresolved path or manifest breakage:
  - stay in phase 1
  - shrink the scope to the smallest manifest/import cluster that reproduces the failure
- Python/shell/tooling failure caused by stale paths:
  - classify the failing references as live/historical/ambiguous
  - repair only the live set
- formal failure caused by missing include/build-root files:
  - stay in phase 4
  - limit edits to include/build-path repair
- formal failure caused by mathematical content drift:
  - stop
  - hand off rather than silently rewriting math
- semantic-validation failure caused by a missing exception:
  - update the target/plan exception lists first
  - then rerun the failed validation
- semantic-validation failure caused by content mismatch:
  - determine whether the file was supposed to be move-only
  - if yes, treat as migration blocker
  - if no, document the intentional rewrite explicitly before continuing

## Resume Artifact Rule

Before any handoff or session end during migration:

- update `scratch/repo-layout-migration-plan.md` if the current state, exception list, or next safe resume point changed
- keep new exceptions explicit in this file rather than only in chat
- do not leave a later agent to reconstruct which verification gate failed

## Phase Ownership

- phase 1 Cargo and Rust repair: local owner, with optional read-only manifest/path audit delegates
- phase 2 packet entrypoint normalization: local owner, with optional delegate per disjoint crate
- phase 3 Python/shell/tooling path repair: local owner, with optional delegate split by subtree (`scripts/`, `.devcontainer/`, `.agents/` + `.codex/`, docs)
- phase 4 formal and thesis path repair: local owner, with optional read-only label/include audit delegates
- phase 5 semantic validation: local owner coordinating multiple read-only verification delegates in parallel

## Remaining Work

### 1. Cargo and Rust repair

Objective:
- make the moved code build again from the new layout

Required changes:
- create repo-root `Cargo.toml` as the global workspace manifest
- move workspace configuration from `crates/Cargo.toml` into the new root manifest
- update workspace members to point at `library/` and all active experiment family crates under `experiments/`
- update path dependencies that still refer to `../database` or old `crates/...` paths
- expose the moved database module from `library/src/lib.rs`
- rewrite all `database::...` imports to `symplectic::database::...`
- remove the assumption of one canonical mutable cache file
- update loaders so experiments can read multiple `.jsonl` inputs
- update loader/write logic so each experiment or family owns its own `.jsonl` file or files
- allow an experiment to populate or refresh its owned `.jsonl` file from loaded input files
- allow an experiment to add newly computed values to its owned `.jsonl` file after that population step
- merge rows fieldwise rather than by row priority
- allow missing or unknown fields in one input row to be filled from a concrete value in another row for the same polytope
- if two input rows provide conflicting concrete values for a field that should be unique for that polytope, fail loudly so the caller is informed early in the run
- allow provenance/metadata accumulation only when it does not hide a concrete-data conflict
- treat placement of the multi-file merge helper as an implementation detail; the hard requirement is the documented behavior, not a specific helper location
- decide whether the old `crates/Cargo.toml` becomes a shim or is removed after the root workspace works

Verification:
- `cargo metadata` from repo root
- `cargo build --workspace --release`
- `cd library && cargo test --release --lib`
- `cd library && cargo clippy --lib -- -D warnings`

### 2. Packet entrypoint normalization

Objective:
- make experiment packet naming match the target layout without changing semantics

Required changes:
- rename packet `run.rs` files to `main.rs`
- keep sibling specialized entrypoints such as `probe.rs`, `profile.rs`, `collect_poly.rs` only where they are genuinely distinct binaries
- update all `Cargo.toml` `[[bin]] path = ...` entries

Verification:
- compare packet inventory before/after rename
- `cargo metadata`
- family-local `cargo build -p <crate>`

### 3. Python, shell, and tooling path repair

Objective:
- restore runnable scripts and operational commands under the new tree

Required changes:
- update `figure_config` imports to point to `experiments/figure_config.py`
- update cloud/devcontainer scripts under `scripts/` and `.devcontainer/`
- update `.agents/skills/*`, `.codex/*`, and `AGENTS.md` command paths
- update LICCA job and scp path examples
- update remaining live references to `docs/` and moved paper paths

Verification:
- `rg -n 'crates/|docs/' AGENTS.md .agents .codex scripts .devcontainer codex-cloud.md thesis formal experiments library`
- smoke-run changed shell scripts where cheap and safe

### 4. Formal and thesis path repair

Objective:
- make developer math builds work again from `formal/`

Required changes:
- create `formal/main.tex`
- create `formal/.latexmkrc` from the current `crates/.latexmkrc`
- create `formal/bibliography.bib` from the current `crates/bibliography.bib`
- create `formal/library/main.tex` includes that reflect the moved files
- update `\input{}` and `\graphicspath{}` references in moved formal files
- remove dependency on `crates/.latexmkrc` and `crates/bibliography.bib`
- decide whether `crates/main.tex` is a temporary shim or is removed after `formal/main.tex` works
- update thesis references that intentionally point to developer math paths

Verification:
- `cd formal && latexmk`
- `cd thesis && latexmk && ./check-build.sh`
- compare `scratch/migration/tex-label-inventory.txt` against the post-repair label set

### 5. Semantic validation

Objective:
- prove the migration did not lose content and did not leave live stale-path breakage

Required checks:
- moved content still matches `scratch/migration/content-inventory.json` where files were moved without rewriting
- every moved experiment directory that contains `run.rs` during migration or `main.rs` in the target has a matching `research/.../design/*.md` note; explicit no-runnable-pair exceptions are the notes `research/hko-local-maximum/design/subdifferential-lp.md`, `research/sys-landscape/design/witness-search-program.md`, `research/sys-landscape/design/imported-sys-search-chatgpt-pro-extended-2026-04-13.md`, `research/verification/design/algorithm-comparison/profiling.md`, and `research/combinatorial-cells/design/gradient-discontinuity.md`; among those, the analysis-only experiment directories are `experiments/hko-local-maximum/subdifferential-lp/`, `experiments/combinatorial-cells/gradient-discontinuity/`, and `experiments/verification/algorithm-comparison/profiling/`
- every moved former packet-level `math.tex` has a matching `formal/.../*.tex`; explicit exclusions are `crates/main.tex`, `crates/library/src/math.tex`, and `crates/library/src/math-preamble.tex`
- no unallowlisted live references to top-level `docs/` remain
- no live runtime/build dependency on the old `crates/` layout remains
- no loader still assumes the old single global cache architecture
- the legacy repo-root `data/polytopes.jsonl` file is removed by the end of the migration

Suggested bounded subagent checks:
- stale-live-path scan
- Cargo/bin manifest consistency
- formal label/reference consistency
- research/experiment packet pairing
- figure/data inventory consistency

## Stop Conditions

A migration session should stop and hand off if any of these become true:

- path repair expands into many unrelated doc rewrites
- `formal/` repair turns into semantic math editing instead of include-path repair
- the database merge causes API redesign instead of import-path rewrites
- historical references and live references become hard to distinguish without a separate audit pass

## Deferred Items

These are explicitly outside the migration critical path:

- creating `agenda.md` and `interpretation.md`
- thematic splitting or prose cleanup inside `formal/`
- style cleanup of moved notes
- resolving `dev-tube/`
- assigning `AGENTS.new.rules.md`
- assigning `paranoia-numerics-report.md`
- any new thesis writing

## Recommended Next Resume Point

Start with repo-root Cargo workspace repair. That is the smallest next chunk that restores the strongest verification signal and unlocks the rest of the migration.
