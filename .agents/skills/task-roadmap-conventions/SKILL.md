---
name: task-roadmap-conventions
description: Use when editing ROADMAP.md, tasks/*.md, old task-tracker migration, or task-bundle style/routing for the thesis closeout; covers pruning rules, Steering Cache vs Agent Cache, value classes, and roadmap refresh checks.
---

# Task Roadmap Conventions

Read `tasks/README.md` before editing `ROADMAP.md`, `tasks/*.md`, or migrating
old tracker content.

## Workflow

1. Start from `ROADMAP.md` to identify the relevant topic bundle.
2. Edit the topic bundle, not old tracker files.
3. Classify retained content as `Steering Cache`, `Work Map`, `Agent Cache`, or
   `Pruned / Stale`.
4. Preserve Jorn/Kai/external decisions that change future work.
5. Prune stale schedules, obsolete ownership, old packet queues, and derivable
   state.
6. Update `ROADMAP.md` only when the global overview changes.
7. If claim strength changes, update or flag `RESULTS.md`.
8. If final done gates change, update or flag `FINAL-VERIFICATION.md`.

## Checks

- `git diff --check`
- If an old tracker file is edited or restored, explain why it was not enough
  to edit `ROADMAP.md` or `tasks/*.md`.
