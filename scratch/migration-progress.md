# Migration Progress Log

Purpose: durable execution log for the repo-layout migration so a later agent can resume safely after chat compaction or loss.

Last updated: 2026-04-14
Current owner: top-level Codex session
Source of truth:
- `scratch/repo-layout-migration-plan.md`
- `scratch/repo-layout-target-tree.v2.md`
- `scratch/migration/move-map.tsv`

## Current status

Active phase: phase 3 (live-path repair across operational docs, research/design notes, experiment script headers, and formal comments)

Confirmed repo facts:
- Repo-root `Cargo.toml` has now been created as the workspace manifest.
- Old workspace manifest still exists at `crates/Cargo.toml` and still names pre-migration members such as `database`, `exp-hko-local-maximum`, `dev-*`, `crosspolytope`, and `visualization`.
- Moved crates now exist under `library/` and `experiments/*`.
- Old build-support files still exist under `crates/.latexmkrc` and `crates/bibliography.bib`.
- `library/src/lib.rs` now exposes `database`.

Confirmed live migration blockers already seen locally:
- Many live scripts/docs still reference `crates/...` paths.

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

Phase 2 completed:
- canonical experiment packet entrypoints were renamed from `run.rs` to `main.rs`
- matching `Cargo.toml` `[[bin]] path = ...` entries were updated
- `probe.rs`, `profile.rs`, `collect_poly.rs`, and other distinct non-canonical entrypoints were left in place
- verification:
  - `rg -n 'path = ".*/run\.rs"' experiments -g 'Cargo.toml'` returned no matches
  - `find experiments -name run.rs` returned no remaining canonical packet entrypoints
  - `cargo build --workspace --release` still passes

Next:
- repair live Python/shell/doc references that still mention `run.rs`, `crates/...`, or old cargo entrypoint commands
- then normalize experiment-owned JSONL path policy
- then repair the `formal/` root build and include structure

Do not assume phase 1 solved the dataset-ownership policy; that still needs explicit edits after the entrypoint rename.

## Plan snapshot

1. Create repo-root workspace manifest and get `cargo metadata` working.
2. Repair workspace members, path dependencies, and `symplectic::database` exposure/imports.
3. Replace live assumptions of one canonical mutable `data/polytopes.jsonl` with experiment-owned inputs/outputs, matching target policy.
4. Normalize packet entrypoints and live tooling paths.
5. Repair `formal/` root files and includes so developer math no longer depends on old `crates/`.
6. Run review/verification, update handoff docs, and checkpoint commit.

## Running audits

Read-only subagents launched:
- Cargo/Rust audit
- Python/shell/tooling live-path audit
- Formal/thesis path audit

Instruction to later agents:
- Treat subagent output as hints only until locally verified against files/commands.
- Update this file after each meaningful phase result or failure.
