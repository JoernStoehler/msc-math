---
name: review-rust
description: Independent critical review of Rust code in worktree branches. Make merge decision (MERGE / MERGE WITH IMPROVEMENTS / NEEDS WORK) autonomously.
---

# Rust code review

## Core principle

You decide. Investigate, decide (MERGE / MERGE WITH IMPROVEMENTS / NEEDS WORK), implement fixes if needed, report decision. Jörn merges but does not review.

## Methodology

1. **Exploration** (parallel): Launch 3 Explore agents (git history, code structure, conventions)
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

## Common pitfalls

- Writing report for Jörn instead of deciding
- Comparing to failed attempts
- Overly generous when critical paths untested
- Performance claims without measurements
- Academic tangents
- Forgetting to commit before reporting

## Report format

50-100 lines: Decision + summary + strengths + issues + improvements + recommendation. Not 550-line comprehensive analysis.
