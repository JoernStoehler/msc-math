# Migration Progress Log

Purpose: durable execution log for the repo-layout migration so a later agent can resume safely after chat compaction or loss.

Last updated: 2026-04-14
Current owner: top-level Codex session
Source of truth:
- `scratch/repo-layout-migration-plan.md`
- `scratch/repo-layout-target-tree.v2.md`
- `scratch/migration/move-map.tsv`

## Current status

Active phase: residual-scope audit after the phase 4 checkpoint

Confirmed repo facts:
- Repo-root `Cargo.toml` has now been created as the workspace manifest.
- Old workspace manifest still exists at `crates/Cargo.toml` and still names pre-migration members such as `database`, `exp-hko-local-maximum`, `dev-*`, `crosspolytope`, and `visualization`.
- Moved crates now exist under `library/` and `experiments/*`.
- Old build-support files still exist under `crates/.latexmkrc` and `crates/bibliography.bib`.
- `library/src/lib.rs` now exposes `database`.

Confirmed live migration blockers already seen locally:
- The migration no longer has known live `crates/...` or canonical `run.rs` blockers in operational docs/scripts. Remaining hits are dated historical notes or copied-source provenance comments.
- The legacy repo-root `data/polytopes.jsonl` endgame is complete: owned caches were materialized and the legacy file was deleted.

Residual-scope audit (2026-04-14, after checkpoint `f6a40a21`) found:
- verification gates now include `cargo clippy -p symplectic --lib -- -D warnings` and `cd thesis && latexmk && ./check-build.sh`; both passed in this session
- one real live migration bug existed in `.devcontainer/warmup-cache.sh` (still warming `crates/Cargo.toml`); local fix is now in progress
- one real live migration bug existed in `.codex/agents/review-python.toml` (still referencing `crates/figure_config.py`); local fix is now in progress
- active tracker references in `TASKS.md` still pointed at deleted `docs/...` paths and old `crates/...` paths; local cleanup is now in progress
- dead shims still exist under `crates/`:
  - `.gitignore`
  - `.latexmkrc`
  - `Cargo.lock`
  - `Cargo.toml`
  - `bibliography.bib`
  - `database/Cargo.toml`
  - `main.tex`
- explicit follow-up leftovers still under `crates/`:
  - `dev-tube/` (target file marks as follow-up decision, not silent assignment)
  - `math-writeup-scaffold.md` (still needs a non-`crates/` home)

Residual-scope audit result after cleanup:
- `.devcontainer/warmup-cache.sh` fixed to warm the repo-root workspace
- `.codex/agents/review-python.toml` fixed to point at `experiments/figure_config.py`
- `TASKS.md` stale live pointers repaired
- `crates/math-writeup-scaffold.md` moved to `scratch/math-writeup-scaffold.md`
- dead old-root shims deleted:
  - `crates/.gitignore`
  - `crates/.latexmkrc`
  - `crates/Cargo.lock`
  - `crates/Cargo.toml`
  - `crates/bibliography.bib`
  - `crates/database/Cargo.toml`
  - `crates/main.tex`
- the remaining `crates/`, `run.rs`, `docs/`, and `data/polytopes.jsonl` scan hits are historical/provenance files only, and are now listed in `scratch/migration/stale-path-allowlist.txt`

Latest local check:
- `cargo metadata --format-version 1 --no-deps` passes from repo root.
- `cargo build --workspace --release` passes from repo root.
- `cd library && cargo test --release --lib` passes.
- `cd library && cargo clippy --lib -- -D warnings` passes.

## Phase 1 result

Completed:
- Added repo-root `Cargo.toml` workspace manifest with the moved `library/` and `experiments/` members.
- Integrated `library/src/database.rs` into the `symplectic` crate and exposed it from `library/src/lib.rs`.
- Removed dead `database = { path = "../database" }` dependencies from moved experiment manifests.
- Rewrote experiment Rust imports from the old external `database` crate to `symplectic::database`.
- Repaired stale manifest bin paths for:
  - `experiments/crosspolytope/Cargo.toml`
  - `experiments/visualization/Cargo.toml`

Still pending from the target policy:
- Experiment-owned JSONL path policy is not fully normalized yet.
- The shared-path references to `../../data/polytopes.jsonl` still exist in several binaries.
- Multi-file merge semantics are not yet encoded as a public helper API.

## Next safe resume point

Phases 2-4 completed:
- canonical experiment packet entrypoints were renamed from `run.rs` to `main.rs`
- matching `Cargo.toml` `[[bin]] path = ...` entries were updated
- `probe.rs`, `profile.rs`, `collect_poly.rs`, and other distinct non-canonical entrypoints were left in place
- live operational-doc references were repaired across `AGENTS.md`, `.agents/skills/**`, `TASKS.md`, `research/**`, `experiments/**/*.py|*.md|*.sh`, and `formal/**/*.tex`
- `formal/main.tex`, `formal/.latexmkrc`, and `formal/bibliography.bib` now exist and `cd formal && latexmk` succeeds
- `library/src/database.rs` now provides `load_many()` with conflict detection, plus tests covering merge-fill and conflict rejection
- owned caches were materialized:
  - `experiments/sys-landscape/cache.jsonl`
  - `experiments/combinatorial-cells/polytopes.jsonl`
  - `experiments/verification/orbit-recovery/polytopes.jsonl`
- sys-landscape binaries now load only `experiments/sys-landscape/cache.jsonl`
- combinatorial-cells binaries now load only `experiments/combinatorial-cells/polytopes.jsonl`
- verification orbit-recovery now loads and saves only `experiments/verification/orbit-recovery/polytopes.jsonl`
- repo-root `data/polytopes.jsonl` has been deleted
- verification:
  - `rg -n 'path = ".*/run\.rs"' experiments -g 'Cargo.toml'` returned no matches
  - `find experiments -name run.rs` returned no remaining canonical packet entrypoints
  - `cargo build --workspace --release` still passes
  - `cargo test -p symplectic --release --lib database::` passes
  - `cargo build -p exp-sys-landscape --release` passes
  - `cargo build -p exp-combinatorial-cells --release` passes
  - `cargo build -p dev-capacity-validation --release` passes
  - `bash -n scripts/codex-cloud-smoke.sh scripts/codex-cloud-rust-warmup.sh scripts/codex-cloud-setup.sh` passes
  - `cd formal && latexmk` succeeds

Next:
- commit the residual-scope cleanup
- then the migration handoff can honestly say: live migration done, remaining leftovers are explicit historical exceptions or separately-deferred follow-up decisions

Do not assume every historical note should be rewritten: the remaining `data/polytopes.jsonl` mentions in `research/` are historical descriptions of the old behavior, not live instructions.

## Plan snapshot

1. Create repo-root workspace manifest and get `cargo metadata` working.
2. Repair workspace members, path dependencies, and `symplectic::database` exposure/imports.
3. Replace live assumptions of one canonical mutable `data/polytopes.jsonl` with experiment-owned inputs/outputs, matching target policy.
4. Normalize packet entrypoints and live tooling paths.
5. Repair `formal/` root files and includes so developer math no longer depends on old `crates/`.
6. Run review/verification, update handoff docs, checkpoint commit, and finish the legacy-cache endgame.

## Running audits

Read-only subagents launched:
- Cargo/Rust audit
- Python/shell/tooling live-path audit
- Formal/thesis path audit

Instruction to later agents:
- Treat subagent output as hints only until locally verified against files/commands.
- Update this file after each meaningful phase result or failure.
- Do not recreate repo-root `data/polytopes.jsonl`; each family/experiment now owns its own cache file.
