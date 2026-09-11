# Single-crate quality calibration

## Project goal

Finish the thesis with a passing grade. Our robust proxy is Jörn saying, after
reading the thesis PDF, that he thinks it deserves a PASS.

## Session goal

Work directly with Jörn on `crates/algebraic-numbers` to calibrate a useful
process for bringing mathematical code to ordinary standards of quality across
several dimensions at once. Jörn wants to observe the process and result so
misunderstandings can be caught before scaling to more crates or cross-crate work.
You are a top-level collaborator in your own conversation with him.

The purpose is code that authors can understand and describe accurately and
extend without avoidable friction. This is a bounded learning task; neither
broad refactoring nor this crate's quality is established as the main thesis
bottleneck. A no-change conclusion for a sound part of the crate is useful.

## First work

Inspect the crate's implementation, public contracts, tests and real consumers.
Discuss your concrete assessment and first coherent change with Jörn so he can
observe what standards you apply and which tradeoffs you accept. Avoid proposing
a whole-codebase programme or treating a checklist score as the outcome.

A read-only assessment found these candidates; verify them yourself:

- `linear_solve.rs` reduces the augmented matrix and then reduces the coefficient
  matrix again through `kernel_basis`. Reusing reduction data could simplify the
  explanation and avoid repeated exact arithmetic without changing the API.
- Generated linear-algebra tests check solutions and kernel membership, but do
  not fully establish that the returned kernel spans all null directions.
- `DEVELOPMENT.md`'s capability/architecture account omits the fraction-free
  solver and calls `ExactScalar` a marker although it requires conversion.
- Source inspection and an independent binary64 calculation expose an endpoint
  conversion panic: for the valid degree-one field with polynomial `t - 1` and
  interval `(1 - 2^-60, 2)`, the lower endpoint rounds to `1.0`, so default
  `root_f64()` asserts. Even converting the rational constant `7` calls that
  method unconditionally. This has not been reproduced in a Rust regression
  and is not an observed thesis-computation failure. Treat numerical conversion
  semantics as a separate bounded problem, not incidental solver cleanup.

These findings do not require implementing all four. Select a coherent first
unit with Jörn. The existing small architecture did not suggest a wholesale
replacement. Runtime benefit from removing repeated elimination is unmeasured.

## Constraints and their reasons

Preserve exact zero/pivot decisions, inconsistency classification, the complete
affine solution set, kernel dimensions (including empty dimensions), deterministic
free-column ordering, rational/algebraic support and fraction-free fallback
semantics. Missing null directions can change KKT feasibility conclusions.

Prefer familiar, readable designs. Improve several quality dimensions together;
do not trade mathematical clarity for a generalized framework or optimize a
single style metric. Explain material tradeoffs using actual code and callers.

Use bounded subagents for independent work where useful. Do not launch top-level
agents without Jörn's approval of their prompts. Coordinate ownership before
cross-crate edits. Ordinary focused checks are appropriate; expensive experiments
need discussion because past verification consumed substantial CPU unexpectedly.

Ask for feedback when a concrete artifact and applicable checks are available,
with the exact judgment needed. Do not make Jörn discover defects you can find.
Use async questions and self-contained finals for consequential communication;
he does not reliably monitor commentary. Commit coherent changes using the
required Codex-Thread trailer. Preserve pre-existing `tmp/`.

## Evidence and completion of the pilot

Use existing crate tests, relevant caller tests, and checks that actually test
the changed mathematical contract. For solver work, completeness and independence
of the kernel matter in addition to membership; include rational and algebraic
cases and boundary dimensions. Broaden checks when failures or changes justify it.

Assess the crate across the quality dimensions that matter here, explain where
its current state is already satisfactory, and identify unresolved tradeoffs.
A first change is a calibration step; continue within the crate until the process
and resulting quality are concrete enough for Jörn to judge readiness to scale.

Give Jörn the actual result, preserved contracts, meaningful check results,
remaining tradeoffs and what this trial taught about the process. His assessment
of the pilot informs whether to repeat or scale it; do not claim that one change
certifies the entire crate or that all quality dimensions have been maximized.

Start with the crate's README, DEVELOPMENT and LINEAR_ALGEBRA_FEATURES documents,
its sources/tests, `crates/euclidean-polytopes/src/linalg.rs`, and
`crates/symplectic/src/kkt/rational_solver.rs`. Broader migration proposals in
`memories/architecture-migration.md` are context, not current assignments.
