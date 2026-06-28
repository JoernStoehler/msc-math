<!--
Purpose: dated repo-status evidence for cheap future orientation.
Context: this is supporting material for status and capability tracking (including
`CAPABILITY_CLAIM_MAP.md`), not a live task file and not a final thesis gate.
-->

# Repo Status Smoke And Core Verification 2026-05-31

## Status

- Commands listed below were run through commit `269fb7b1`.
- Commits after `269fb7b1` are not covered by these command results unless they
  only change documentation, caches, or read-only status helpers. Check
  `git log` and rerun affected checks after code, data, artifact, or
  build-contract changes.
- For a quick current-`HEAD` applicability summary, run
  `scripts/repo-status-summary.sh`.
- Working tree before checks: clean.
- Working tree after checks: clean.
- No tracked data, figures, thesis build artifacts, or canonical experiment
  outputs were intentionally refreshed.

Durable evidence retained here:

- the command list and pass/fail results below;
- explicit caveats about checks that were skipped, stopped, or too expensive;
- cache consequences for future map/current-state refreshes.

Scratch reports used during the pass:

- `/tmp/repo-status-refresh-plan.md`
- `/tmp/repo-status-smoke-navigation.md`
- `/tmp/repo-status-smoke-rust-experiments.md`
- `/tmp/repo-status-core-verification.md`
- `/tmp/repo-status-workspace-experiment-verification.md`
- `/tmp/repo-status-latex-verification.md`

The `/tmp` reports are not durable project state. They are listed only so a
current session can inspect local scratch if it still exists; this file is the
durable summary.

## Passed Smoke Checks

Navigation/thesis/formal smoke:

- map/cache/status-marker inventory scans passed;
- `thesis/main.tex` read successfully and still inputs active scaffold files;
- thesis placeholder scan found scaffold/TODO-heavy active thesis files, as
  was stated in the then-current task cache;
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
- `experiments/sys-datascience/smoke-pipeline.sh` is temp-output safe
  but not cheap. It was stopped after about two minutes while
  `sys-dataset-continuation --smoke` was still running. This caveat is now
  documented in the local README files, script header, and
  `CAPABILITY_CLAIM_MAP.md`.
- At the time of this reference, the old nested datascience path had no root
  README. The current lifted entry point is
  `experiments/sys-datascience/README.md`, with detailed entry points in
  `produce/README.md`, `prepare/README.md`, `methods/README.md`, and
  `smoke-pipeline.sh`.
- The old `cargo check -p sys-landscape` command was stale and failed; use
  `cargo check -p exp-sys-landscape`.

## Cache Consequences

- No contradiction was found for the then-current task cache, `thesis/MAP.md`,
  the then-current research index, `crates/MAP.md`, or `experiments/MAP.md`.
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
- Commands not listed above were not run in this pass.

## Artifact-Refresh Boundary

The checks above show that selected commands/builds pass. They do not prove
that tracked datasets, figures, or generated experiment reports are fresh.

Before treating tracked generated artifacts as refreshed, start from the local
README or source header that owns the producer. The high-risk refresh areas are:

- `experiments/verification/`: tracked correctness, all-minimum, orbit-recovery,
  and algorithm-comparison evidence;
- `experiments/hko-local-maximum/`: canonical HKO gradient/Sage evidence and
  historical HKO artifacts;
- `experiments/sys-landscape/`: random/product/ascent/continuation datasets,
  datascience producer caches, tables, method reports, and full-output search
  artifacts;
- `experiments/dev-quadratic-program/numerics-audit/`: canonical exactness, Sage feasibility,
  unknown-predicate, collection, and error-bound artifacts;
- `experiments/combinatorial-cells/`: producer binaries that write tracked
  evidence and currently have no smoke modes;
- `experiments/crosspolytope/` and `experiments/visualization/`: inspect local
  entrypoints before running because they are likely artifact producers;
- `submit/`: official forms and web/admin facts require current
  source checks close to final submission.

If one of these areas changes, update the owning local note or experiment
README first, then update the affected owner-local status note and/or
`CAPABILITY_CLAIM_MAP.md` only if near-term thesis routing or reusable repo
capability changed.
