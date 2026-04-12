# Handoff: Stale branch audit — 2026-04-12

## Scope

Addresses the `[open]` TASKS.md item "Stale branch cleanup" (Infrastructure + tooling section). One entry per local branch that is (a) not `main` and (b) not attached to an active worktree.

Read-only audit. No `git branch -D`, no `git push --force`, no destructive ops. Jörn decides per-branch whether to delete or keep.

## Enumeration

```
git branch --format='%(refname:short)'
```

yields (at 2026-04-12):

```
citation-verification
citation-verification-d
database-cleanup
delete-api-reference
hygiene-audit           ← this worktree's own branch (excluded)
housekeeping-triage
main                    ← excluded
numerical-story-expand
paranoia-numerics       ← attached to worktree .claude/worktrees/paranoia-numerics (excluded; covered in worktree audit)
```

After excluding `main`, `hygiene-audit`, and `paranoia-numerics`, six branches remain in scope for this audit.

## Merge index

Authoritative source: `git branch --merged main` →

```
citation-verification
citation-verification-d
database-cleanup
delete-api-reference
housekeeping-triage
```

Five of the six in-scope branches are merged. `numerical-story-expand` is the only unmerged one.

## Per-branch entries

### `citation-verification`

- **Commits ahead of main:** 0
- **Commits behind main:** 33
- **Last commit date + author:** `2026-04-07 21:42:48 +0000` by `JoernStoehler` — "Expand papers/CLAUDE.md with download instructions and verification workflow"
- **Merged-into-main?** `merged` (listed in `git branch --merged main`; 0 commits ahead confirms all work is on main)
- **Diff vs main:** 0 files changed
- **1-line purpose guess:** Session branch for the `TASKS.md` "Citation verification pass" item (marked `[done] [2026-04-07]`). Delivered work lives on main; branch is a post-merge leftover.

### `citation-verification-d`

- **Commits ahead of main:** 0
- **Commits behind main:** 17
- **Last commit date + author:** `2026-04-07 21:51:55 +0000` by `JoernStoehler` — "citation: add unique improvements from parallel verification"
- **Merged-into-main?** `merged` (0 commits ahead; listed in merged set)
- **Diff vs main:** 0 files changed
- **1-line purpose guess:** Parallel/duplicate branch of the citation-verification session (the `-d` suffix reads as "duplicate" or "direct"; last-commit subject says "parallel verification" explicitly). Both its unique improvements and citation-verification's work reached main; post-merge leftover.

### `database-cleanup`

- **Commits ahead of main:** 0
- **Commits behind main:** 26
- **Last commit date + author:** `2026-04-07 22:01:07 +0000` by `JoernStoehler` — "Fix output path mismatches: experiments now write to their own subdirs"
- **Merged-into-main?** `merged`
- **Diff vs main:** 0 files changed
- **1-line purpose guess:** Session branch for the `TASKS.md` "Database cleanup" item (marked `[done] [2026-04-07]`). Post-merge leftover.

### `delete-api-reference`

- **Commits ahead of main:** 0
- **Commits behind main:** 1
- **Last commit date + author:** `2026-04-12 10:43:04 +0000` by `JoernStoehler` — "Delete api-reference/ and api-extract tool"
- **Merged-into-main?** `merged` (0 commits ahead; this branch's commit *is* main's tip minus one)
- **Diff vs main:** 0 files changed
- **1-line purpose guess:** Session branch for the `TASKS.md` "Delete api-reference/" item (marked `[done] [2026-04-12]`, i.e. today). Just-merged leftover.

### `housekeeping-triage`

- **Commits ahead of main:** 0
- **Commits behind main:** 35
- **Last commit date + author:** `2026-04-07 17:48:08 +0000` by `JoernStoehler` — "Delete stale handoff: experiment-api-fixes.md"
- **Merged-into-main?** `merged`
- **Diff vs main:** 0 files changed
- **1-line purpose guess:** Session branch for a broad housekeeping pass (the commit subject indicates stale-handoff triage). No matching `TASKS.md` header, but the 2026-04-07 timestamp clusters with the citation-verification and database-cleanup cleanup sprint. Post-merge leftover.

### `numerical-story-expand`

- **Commits ahead of main:** 1
- **Commits behind main:** 9
- **Last commit date + author:** `2026-04-08 08:47:20 +0000` by `JoernStoehler` — "Expand numerical-story.md outline to full depth"
- **Merged-into-main?** `unmerged` (not in `git branch --merged main`; 1 commit ahead)
- **Diff vs main:** 1 file changed, 458 insertions(+), 160 deletions(-) — edits `thesis/numerical-story.md` (file exists on main; this branch rewrites / expands it).
- **1-line purpose guess:** Single-commit expansion of the thesis numerical-story outline (`thesis/numerical-story.md`), building on Jörn's earlier `WIP: Jörn's Part 0a expansion` commit already on main. **This is the only unmerged branch in this audit and contains work that is not on main.**

## Recommendation to Jörn

- Five of six branches (`citation-verification`, `citation-verification-d`, `database-cleanup`, `delete-api-reference`, `housekeeping-triage`) are fully merged and have **0 commits ahead** and **0-file diffs** vs main. They are safe candidates for deletion (`git branch -d <name>` — the safe variant, which refuses to delete unmerged branches — would succeed on all five). This audit does **not** delete them; Jörn to confirm deletion explicitly.
- `numerical-story-expand` is **not safe to delete** without review: it has one commit ahead of main touching `thesis/numerical-story.md` with a 458/-160 diff. Options for Jörn: (a) rebase and merge into main, (b) cherry-pick the expansion onto a fresh branch keyed to current thesis layout, (c) explicitly abandon if the expansion has been superseded by other thesis work. The branch has been idle since 2026-04-08 — four days — so a quick review is warranted before either merge or delete.

## Ground truth commands (for reproducibility)

```
git branch --format='%(refname:short)'
git branch --merged main

for b in citation-verification citation-verification-d database-cleanup \
         delete-api-reference housekeeping-triage numerical-story-expand; do
  echo "=== $b ==="
  git rev-list --left-right --count main...$b
  git log -1 --format='%ai | %an | %s' $b
  git diff --name-only main...$b | wc -l
  git diff --stat main...$b | tail -1
done
```

Run at worktree `hygiene-audit` on commit base `9d29951a` (branch point from `main`).
