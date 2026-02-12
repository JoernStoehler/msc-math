---
name: rebase
description: Use when rebasing a git branch onto main. Checklist prevents common mistakes (wrong base, stale refs, lost commits).
---

# Rebase checklist

Run this checklist every time you rebase. Do not skip steps.

## Before rebase

1. **Commit or stash all changes.** `git status` must show clean working tree.
2. **Record current commit count.** `git log main..HEAD --oneline | wc -l` — write this number down.
3. **Verify target is local `main`, not `origin/main`.**
   ```bash
   git log --oneline -1 main
   git log --oneline -1 origin/main
   ```
   If these differ, you MUST use `main`. Jörn merges locally and pushes later — `origin/main` is frequently stale.

## Rebase

```bash
git rebase main
```

Never `git rebase origin/main`. Never `git fetch origin main && git rebase origin/main`.

## After rebase

4. **Verify commit count.** `git log main..HEAD --oneline | wc -l` — must equal the number from step 2, or less (if some commits were already on main). If it's MORE, you rebased onto a stale base.
5. **Spot-check the diff.** `git diff main..HEAD --stat` — every file listed must belong to your branch's work. If you see unexpected files (CLAUDE.md moves, monitoring additions, etc.), the rebase picked up stale commits.
6. **Run tests.** `cd crates/ && cargo test --lib`

## If something looks wrong

Do NOT force-push or proceed. Investigate:
- Did you rebase onto `origin/main` instead of `main`?
- Does the branch need to be recreated from local `main`?
- Ask Jörn if uncertain.
