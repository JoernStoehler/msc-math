# MSc worktree cleanup — 29 October 2026

After the earlier removal of the completed research-restart worktree, main had
ten non-main registered worktrees. The cleanup removed seven clean inactive
checkouts, then preserved the local changes from the remaining three as
explicitly unreviewed WIP rescue commits on their existing branches and removed
those checkouts too. All ten are now retired; no branch was deleted, and all
unique commits remain reachable from existing local branch names.

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

## Rescued local changes

The three dirty checkouts had no ignored files or runtime/private state. Their
changes were limited to project documentation and two skill symlinks. They were
staged exactly as found and committed with WIP/unreviewed labels, without
editing, reviewing, adopting, or merging the content.

| Former checkout | Branch | Rescue commit | Original local changes |
| --- | --- | --- |
| `/workspaces/msc-math-local-cover-prelim-20261022` | `research/local-cover-prelim-20261022` | `c6b0459d` — `WIP unreviewed rescue: preserve local skill links` | Deleted tracked `.claude/skills/problem-structuring`; added `.claude/skills/orchestrate-project` symlink; one prior unique commit remains on branch. |
| `/workspaces/msc-math-local-cover-review-20261022` | `research/local-cover-review-20261022` | `d014d7d7` — `WIP unreviewed rescue: preserve local skill links` | Same two symlink changes; nine prior unique commits remain on branch. |
| `/workspaces/msc-math-worktrees/official-status-fix-20261022` | `luna/official-status-fix-20261022` | `97db4dfd` — `WIP unreviewed rescue: preserve official status notes` | Edits to README and three thesis-submission documents; added `official-status-checklist.md`; no prior unique commits. |

These branch tips are recovery routes for later review or explicit abandonment.
They are not accepted, validated, or integrated in main. The official-status
checklist itself says current individual facts remain unknown and no inquiry
was sent; no thesis administrative claim is adopted by this rescue.

## Verification and limits

Before removal, all ten non-main worktrees were checked for tracked changes,
untracked files, ignored paths, branch reachability and size. Process cwd checks
found no process working inside a non-main checkout. A symlink scan found no
external consumer pointing into a removed checkout. All ten removals succeeded
without force; each original or rescue branch was verified to remain. The
non-main worktree count is now zero.

`df -h` reported the filesystem's used space falling from 614G to 609G and
available space increasing from 257G to 261G (rounded display), approximately
5 GB reclaimed. This is a filesystem-level estimate, not an attribution of
every freed block to a single checkout.
