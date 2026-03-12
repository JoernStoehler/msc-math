---
paths:
  - "*"
---

# Git Conventions

## Always use local `main`, never `origin/main`

Jörn merges locally and pushes later, so `origin/main` is frequently stale. Comparing against `origin/main` inflates diffs with already-merged commits.

## Three-dot diffs for code reviews

Use `git diff main...HEAD` to show only what the branch changed. Two-dot diff (`main..HEAD`) includes divergence and creates false alarms.

## State the base explicitly

"Compared against local `main` at `abc1234`."

If unexpected files appear in diff, investigate — likely means branch needs rebasing.

## Commit checklist

Before final report:
- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Critical paths have tests
- [ ] Performance claims have benchmarks
- [ ] Working tree clean (no uncommitted changes)
