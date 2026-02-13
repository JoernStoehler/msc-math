---
name: review-rust
description: "[DEPRECATED] Use /review-branch instead. This skill is kept for reference only."
disable-model-invocation: true
---

# [DEPRECATED] Rust code review

**This skill is deprecated. Use `/review-branch` instead, which covers Rust + Python + LaTeX + data pipelines.**

This file is kept for reference. Rust-specific content has been integrated into `/review-branch`.

---

## Core principle

You decide. Investigate, decide (MERGE / MERGE WITH IMPROVEMENTS / NEEDS WORK), implement fixes if needed, report decision. Jörn merges but does not review.

## Methodology

0. **Check for existing analysis** (before exploration):
   - Look for `docs/reports/*`, `*_INVESTIGATION.md`, `*_REVIEW*.md` in worktree
   - If found: read first, verify claims, explore gaps. Saves 20min vs redundant exploration.

1. **Exploration** (parallel): Launch review agents (git history, code structure, conventions). Use `general-purpose` (not `Explore`) when agents need to produce structured reports — Explore agents can't write files, so their output is trapped in JSONL logs.
2. **Analysis** (focused): Identify 2-4 concerns, launch Plan agents for each
3. **Synthesis**: Read critical code sections yourself
4. **Decision**: MERGE / MERGE WITH IMPROVEMENTS / NEEDS WORK

## Test coverage assessment

- **Critical paths untested**: Error paths missing, math properties unvalidated, degenerate cases unhandled
- **Core cases covered**: Happy path exists, known-good inputs work, basic errors handled
- **Edge cases tested**: Property-based tests for ∀ statements, boundaries verified, robustness validated

## Performance claims require measurement

Never state performance without benchmark. "~1ms" is claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured. Add benchmark if claim exists without measurement.

## Thesis constraints

Polytope4D: 5-16 facets typical. Research code, March 2026 deadline. Correctness > performance.

Don't suggest: Theoretical numerical analysis, O(n²) docs when n ≤ 16, production features unlikely to matter.

Do suggest: Critical path tests, benchmarks for claims, robustness fixes (timeouts, limits).

## Property-based testing

Use proptest for universal quantification: "∀ λ > 0: vol(λK) = λ⁴·vol(K)" → proptest. Not for single examples.

## Git comparison base

**Always compare against local `main`, never `origin/main`.** Jörn merges locally and pushes later, so `origin/main` is frequently stale. Comparing against `origin/main` inflates the diff with already-merged commits.

**Use three-dot diff (`main...HEAD`) for reviewing branch changes.** Two-dot (`main..HEAD`) includes divergence from main's newer commits, inflating the diff and creating false alarms (e.g. apparent "deletions" of code the branch never had). Three-dot shows only what the branch actually changed. Use two-dot only for merge-preview (what the repo will look like after merge).

**State the base explicitly in the report.** Example: "Compared against local `main` at `abc1234`." If a discrepancy appears (e.g. unexpected files in diff), investigate — it likely means the branch needs rebasing onto local `main`. See `/rebase` for the rebase checklist.

## Naming and documentation accuracy

Check that public symbols (functions, types, constants, test names) accurately describe their actual behavior:
- Do names match what the code does? (e.g., `random_f6` that silently falls back to an 8-facet hypercube is misleading)
- Do doc comments describe actual behavior, not aspirational or outdated behavior?
- Do test names reflect what's actually being tested? Fallback paths and error recovery can silently change test semantics.

## Common pitfalls

- Writing report for Jörn instead of deciding
- Comparing to failed attempts
- Overly generous when critical paths untested
- Performance claims without measurements
- Academic tangents
- Forgetting to commit before reporting
- Using `origin/main` instead of local `main` as comparison base
- Not stating which base was used for the diff

## File Location Decisions

Template: `file-location-decisions.md` (colocated). Quick reference:
- Investigation code: `*_test.rs` with `#[ignore]`
- Session reports: `docs/reports/<timestamp>-<topic>.md`
- Deprecated code: `#[cfg(test)] mod deprecated`

## Report format

50-100 lines: Decision + summary + strengths + issues + improvements + recommendation. Not 550-line comprehensive analysis.

**Decision table** (put at top):

| Item | Recommendation | Confidence | Quantified Rationale |
|------|---------------|------------|---------------------|
| Investigation code | KEEP | HIGH | Saves 1-2hr debugging, 508 LOC test module (LOW cost) |

- Quantify costs/benefits (time, LOC, frequency)
- State weights explicitly (HIGH/MEDIUM/LOW + why)
- Show actual importance, not artificial balance
- Thesis context: Jörn's time > code cleanliness
