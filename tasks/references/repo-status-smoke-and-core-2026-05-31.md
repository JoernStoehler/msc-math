<!--
Purpose: dated repo-status evidence for cheap future orientation.
Context: this is supporting material for `tasks/current-state.md` and
`CAPABILITY_CLAIM_MAP.md`, not a live task file and not a final thesis gate.
-->

# Repo Status Smoke And Core Verification 2026-05-31

## Status

- Latest commit checked: `269fb7b1`.
- Working tree before checks: clean.
- Working tree after checks: clean.
- No tracked data, figures, thesis build artifacts, or canonical experiment
  outputs were intentionally refreshed.

Source reports:

- `/tmp/repo-status-refresh-plan.md`
- `/tmp/repo-status-smoke-navigation.md`
- `/tmp/repo-status-smoke-rust-experiments.md`
- `/tmp/repo-status-core-verification.md`
- `/tmp/repo-status-workspace-experiment-verification.md`
- `/tmp/repo-status-latex-verification.md`

## Passed Smoke Checks

Navigation/thesis/formal smoke:

- map/cache/status-marker inventory scans passed;
- `thesis/main.tex` read successfully and still inputs active scaffold files;
- thesis placeholder scan found scaffold/TODO-heavy active thesis files, as
  already stated in `tasks/current-state.md`;
- formal status-marker scan found many `unverified` and `TODO: JÖRN` markers,
  matching the developer-facing role of `formal/`;
- experiment README command-contract scan confirmed full/canonical/tracked
  evidence commands need explicit refresh intent.

Rust/experiment smoke:

- `cargo fmt --check` passed;
- `cargo check --workspace` passed;
- `cargo test -p euclidean-polytopes --no-run` passed;
- `cargo test -p algebraic-numbers --no-run` passed;
- `cargo test -p symplectic --release --lib --no-run` passed;
- `cargo test -p dev-capacity-validation --bin axioms-correctness --release`
  passed in `65.15s`;
- `cargo test -p exp-combinatorial-cells --all-targets --no-run` passed;
- `cargo run -p dev-capacity-validation --release --bin axioms-correctness -- --help`
  passed and confirmed the normal non-help run refreshes tracked JSONL.

## Passed Core Verification

- `cargo test -p algebraic-numbers --release` passed.
- `cargo clippy -p algebraic-numbers --all-targets -- -D warnings` passed.
- `cargo run -p algebraic-numbers --example q_sqrt5_vector` passed.
- `cargo test -p euclidean-polytopes` passed.
- `cargo clippy -p euclidean-polytopes --all-targets -- -D warnings` passed.
- `cargo test -p symplectic --release --lib` passed:
  `311` passed, `0` failed, `21` ignored.
- `cargo test -p symplectic --release --test public_capacity_api` passed:
  `4` passed, `0` failed.

## Passed Workspace And Experiment Compile Verification

- `cargo build --workspace --release` passed in about `1m09s`.
- Planned command `cargo check -p sys-landscape` failed because the package name
  is stale. The package is `exp-sys-landscape`.
- `cargo check -p exp-sys-landscape` passed.
- `cargo test -p exp-combinatorial-cells --all-targets` passed.
- `cargo clippy -p exp-combinatorial-cells --all-targets` passed.
- `AGENTS.md` was updated from `cargo check -p sys-landscape` to
  `cargo check -p exp-sys-landscape`.

## Passed LaTeX Build Verification

- `cd formal/ && latexmk` passed; `formal/main.pdf` was already up to date.
- `cd thesis/ && latexmk && ./check-build.sh` passed; `thesis/build/main.pdf`
  was already up to date and `check-build.sh` reported `Build clean`.
- The working tree remained clean after both checks.

## Warnings

- `dev-capacity-validation` correctness tests pass but are not cheap;
  monotonicity dominated runtime in the smoke pass.
- `experiments/sys-landscape/datascience/smoke-pipeline.sh` is temp-output safe
  but not cheap. It was stopped after about two minutes while
  `sys-dataset-continuation --smoke` was still running. This caveat is now
  documented in the local README files, script header, and
  `CAPABILITY_CLAIM_MAP.md`.
- `experiments/sys-landscape/datascience/README.md` does not exist. The
  maintained entry points are `produce/README.md`, `tables/README.md`,
  `methods/README.md`, and `smoke-pipeline.sh`.
- The old `cargo check -p sys-landscape` command was stale and failed; use
  `cargo check -p exp-sys-landscape`.

## Cache Consequences

- No contradiction was found for `tasks/current-state.md`,
  `tasks/planning-notes.md`, `thesis/MAP.md`, `research/INDEX.md`,
  `crates/MAP.md`, or `experiments/MAP.md`.
- `CAPABILITY_CLAIM_MAP.md` and local datascience docs were updated to record
  the `smoke-pipeline.sh` runtime caveat.
- These checks support current cache claims for exact arithmetic,
  euclidean-polytopes ordinary geometry, symplectic release-lib behavior,
  public capacity API shape, selected validation command safety, and command
  contract caution.
- `AGENTS.md` command documentation needed and received the package-name fix for
  sys-landscape checking.

## Not Checked

- Full artifact-refreshing producers were not run.
- LICCA/Slurm, profiling, web/admin, and current university source checks were
  not run.
- Normal verification commands outside the core crate/API set were not run.
