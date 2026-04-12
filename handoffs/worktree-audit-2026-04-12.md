# Handoff: Worktree audit — 2026-04-12

## Scope

Addresses the `[open]` TASKS.md item "Worktree audit" (Infrastructure + tooling section). One entry per non-`main` worktree. Read-only: no `git worktree remove`, no `git branch -D`, no `git reset`, no force ops. Jörn decides what to keep, merge, or discard.

`git worktree list` at 2026-04-12 (from inside the `hygiene-audit` worktree itself):

```
/workspaces/msc-math                                     9d29951a [main]
/workspaces/msc-math/.claude/worktrees/paranoia-numerics e8549fa5 [paranoia-numerics]
/workspaces/msc-math/.claude/worktrees/hygiene-audit     <new>   [hygiene-audit]
```

The `hygiene-audit` worktree is the one this audit is being written from and is excluded per DoD scoping (it is the audit artifact, not an audit target).

## Entry: `.claude/worktrees/paranoia-numerics`

- **Path:** `.claude/worktrees/paranoia-numerics`
- **Branch name:** `paranoia-numerics`
- **Commits ahead of main:** 5
- **Commits behind main:** 37
  (Source: `git rev-list --left-right --count main...paranoia-numerics` → `37   5`.)
- **Last commit date + author:** `2026-04-08 12:20:06 +0000` by `JoernStoehler` — "Fix benchmark fitting method and ablation pruning range"
- **Number of changed files vs main:** 19
- **1-line diff summary:** 19 files changed, 251 insertions(+), 66 deletions(-) — almost all logbook edits under `crates/dev-*/` and `crates/exp-*/` tightening stale or miscomputed numerical claims, plus `paranoia-numerics-report.md` at the repo root, plus edits to `dev-numerical-analysis/error-bounds/tests.rs` and `dev-numerical-analysis/unknown-predicates/run.rs`.
- **Merge-status verdict:** `unmerged-wip`
  - `git branch --merged main` does **not** list `paranoia-numerics`, so it is unambiguously unmerged.
  - `TASKS.md` line 247 has an `[active]` entry "Paranoia: numerical claims (session launched 2026-04-07)" whose scope matches this branch 1:1. The work is live, not abandoned.
- **1-line purpose guess:** Active session fixing stale/wrong numerical claims across experiment logbooks after a cross-check pass; the accompanying `paranoia-numerics-report.md` at the repo root is the session's report-out artifact (header: "Paranoia: Numerical Claims Cross-Check Report. Generated 2026-04-07 by review-claims agents (sonnet) across 29 experiment logbooks.").

### Files touched (for Jörn's review)

```
crates/crosspolytope/logbook.md
crates/dev-algorithm-comparison/ablation/logbook.md
crates/dev-algorithm-comparison/benchmark/logbook.md
crates/dev-capacity-validation/correctness/logbook.md
crates/dev-numerical-analysis/error-bounds/tests.rs
crates/dev-numerical-analysis/kkt-inertia/logbook.md
crates/dev-numerical-analysis/unknown-predicates/logbook.md
crates/dev-numerical-analysis/unknown-predicates/run.rs
crates/exp-combinatorial-cells/boundary-characterization/logbook.md
crates/exp-combinatorial-cells/cell-widths/logbook.md
crates/exp-combinatorial-cells/convexity/logbook.md
crates/exp-combinatorial-cells/gradient-discontinuity/logbook.md
crates/exp-combinatorial-cells/multiple-crossings/logbook.md
crates/exp-combinatorial-cells/omega-hypothesis/logbook.md
crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md
crates/exp-hko-local-maximum/perturbation-neighborhood/logbook.md
crates/exp-sys-landscape/random-sample/logbook.md
crates/exp-sys-landscape/rejection-calibration/logbook.md
paranoia-numerics-report.md
```

### Commit list (for Jörn's review)

```
Fix benchmark fitting method and ablation pruning range
Fix stale numbers in benchmark and convexity logbooks
fix: correct stale numerical claims in ablation logbook
Fix stale/wrong numerical claims across 15 experiment logbooks
Fix two cross-reference errors in lagrangian-boundary logbook
```

## Recommendation to Jörn

- `paranoia-numerics` is live work, not dead; match the TASKS.md `[active]` entry. Do not delete the worktree or branch.
- 37 commits behind main is wide. Before merge, rebase or merge main into the branch and re-verify that the numerical fixes still agree with the current main-side logbook numbering. The 37 main-side commits may have changed line numbers or even the numbers themselves in files the branch also edits.
- Decision for Jörn: rebase-and-merge vs squash-merge vs continue accumulating work on-branch until session ends. This audit does not prescribe.

## Ground truth commands (for reproducibility)

```
git worktree list
git rev-list --left-right --count main...paranoia-numerics   # 37    5
git log -1 --format='%ai | %an | %s' paranoia-numerics
git diff --name-only main...paranoia-numerics | wc -l        # 19
git diff --stat main...paranoia-numerics | tail -1
git branch --merged main                                     # paranoia-numerics not listed
git log main..paranoia-numerics --format='%s'
```
