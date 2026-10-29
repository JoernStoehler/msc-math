# MSc worktree cleanup — 29 October 2026

After the earlier removal of the completed research-restart worktree, main had
ten non-main registered worktrees. This bounded cleanup removed seven clean
inactive checkouts using ordinary `git worktree remove`; no branch was deleted.
All unique commits remain reachable from their existing local branch names.

## Removed checkouts and retained branches

| Removed checkout | Branch retained | Commits ahead of main |
| --- | --- | ---: |
| `/home/joern/.codex/worktrees/8983/msc-math` | `archive/autonomous-research-stopped-20261028` | 1 |
| `/workspaces/msc-math-worktrees/hko-research-2026-10-16` | `claude/hko-research-2026-10-16` | 1 |
| `/workspaces/msc-math-worktrees/opening-pass-2026-10-23` | `claude/opening-pass-2026-10-23` | 17 |
| `/workspaces/msc-math-worktrees/patterns-2026-10-17` | `codex/patterns-2026-10-17` | 32 |
| `/workspaces/msc-math-worktrees/thesis-2026-10-16` | `claude/next-2026-10-16` | 106 |
| `/workspaces/msc-math-worktrees/thesis-evidence-refs-20261022` | `claude/thesis-evidence-refs-20261022` | 2 |
| `/workspaces/msc-math/.worktrees/status-fix-2026-10-23` | `claude/status-fix-2026-10-23` | 0 |

The stopped-attempt archive is now branch-only; its earlier worktree and
ignored Python bytecode cache were removed. Other ignored contents in the
removed checkouts were regenerable Rust `target/` output, LaTeX `thesis/build/`
output, or Python bytecode caches. No tracked changes or untracked source files
were present in these seven checkouts at removal.

## Preserved checkouts

Three worktrees were left registered because they contain local changes. Their
contents were not copied, reset, staged, or discarded.

| Checkout | Branch / HEAD | Preserved local changes |
| --- | --- | --- |
| `/workspaces/msc-math-local-cover-prelim-20261022` | `research/local-cover-prelim-20261022` at `ca3bdcc5` | Deleted tracked `.claude/skills/problem-structuring`; untracked `.claude/skills/orchestrate-project` symlink; one unique commit ahead of main. |
| `/workspaces/msc-math-local-cover-review-20261022` | `research/local-cover-review-20261022` at `beff369c` | Same deleted tracked symlink and untracked orchestrate-project symlink; nine unique commits ahead of main. |
| `/workspaces/msc-math-worktrees/official-status-fix-20261022` | `luna/official-status-fix-20261022` at `9667e351` | Tracked edits to README and three thesis-submission documents; untracked `official-status-checklist.md`; no unique commits ahead of main. |

Next cleanup requires reviewing these local changes and deciding their
retention. Do not remove these checkouts before that decision. The project
status links this action as unassigned.

## Verification and limits

Before removal, all ten non-main worktrees were checked for tracked changes,
untracked files, ignored paths, branch reachability and size. Process cwd checks
found no process working inside a non-main checkout. A symlink scan found only
checkout-internal symlinks, with no external symlink consumer pointing into a
removed checkout. The seven removals succeeded without force; their branches
were verified to remain. The non-main worktree count is now three.

`df -h` reported the filesystem's used space falling from 614G to 609G and
available space increasing from 257G to 261G (rounded display), approximately
5 GB reclaimed. This is a filesystem-level estimate, not an attribution of
every freed block to a single checkout.
