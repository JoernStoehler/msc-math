---
name: roadmap-maintenance
description: Use when editing or reasoning about ROADMAP.md, tasks/*.md, task-bundle conventions, closeout sequencing, task graph cleanup, prioritization, ownership, dependencies, blocker handling, steering caches, or deciding how to split work between Jörn and agents.
---

# Roadmap Maintenance

This skill is a combinable protocol for project-management and roadmap work. It
does not require the whole session to stay in a pure PM mode. Use it when the
conversation touches task surfaces, sequencing, ownership, or agent/Jörn
division of labor.

`ROADMAP.md` and `tasks/*.md` are the project-management notebook for agents.
They route work, preserve steering decisions, and cache resume points. They are
not proof databases, experiment interpretations, or final done authority.

## Method

Use the agent's fast reading, comparison, synthesis, and writing to make project state legible. Use Jörn's expertise to classify depth, thesis priority, mathematical risk, and what agents can actually handle.

Do project management with Jörn, not for Jörn in a hidden plan. Surface only the decisions that need him; do the reading, inventory, comparison, compression, and rewrite work yourself.

## Operating Loop

1. Start from `ROADMAP.md`, then open the relevant `tasks/*.md` bundle.
2. Check repo evidence before asking Jörn: linked files, logs, results, scratch notes, and prior task entries.
3. Classify each task by status, owner, blocker, dependency, thesis relevance, next action, and acceptance check.
4. Rewrite the relevant task bundle so a later agent can resume from it:
   sections carry status and key state; bodies carry decisions, evidence links,
   blockers, resume points, and verification checks.
5. When the plan is unclear, compare concrete decompositions, bundles, owners, or execution orders and ask Jörn the smallest question that separates the plausible choices.
6. Default to serial work: plan, execute, and verify `A`, then repeat for `B`. Propose fused `A+B` only when the same evidence, files, and verification check cover both and fusion adds no new design decision.
7. Compress long findings into a short surface Jörn can skim; expand only when he asks or when detail is needed for a decision.

## Decision Surfaces

When a PM decision needs Jörn, present a compact surface:

- Question.
- Current evidence.
- Candidate options.
- Tradeoffs stated as concrete consequences: deadline impact, conceptual depth, verification difficulty, coupling to other tasks, and likely agent failure mode.
- Recommended default if the evidence supports one.
- What you will update in `ROADMAP.md` or `tasks/*.md` after his answer.

Ask concrete questions. Prefer "Which of these two decompositions matches your view of agent difficulty?" over "What should I do?"

Iterate by narrowing choices. If Jörn rejects a surface, ask what changed the classification: depth, coupling, verification, priority, ownership, or wording. Then rewrite the surface instead of continuing with a hidden plan.

When proposing agent execution, add these fields:

- Unit of work.
- Decision points.
- Dependencies.
- Expected output artifact or finding.
- Why it is shallow or where it may be deep.
- Serial, parallel, Jörn-owned, or focus-switch shape.
- Files or artifacts likely touched.
- The verification check.
- Stop condition.

Load `$subagent-delegation` when drafting a surface that may involve explorers,
workers, reviewers, serial queues, or parallel work. After Jörn approves the
surface, use it to write bounded subagent briefs and integrate results.

When choosing whether a candidate task is shallow enough to delegate, read `references/delegation-calibration.md` if recent examples would help. It records past work packets whose actual difficulty differed from expectation.

## Roadmap Rules

- Read `tasks/README.md` before editing `ROADMAP.md`, `tasks/*.md`, or old
  tracker migrations.
- Use the task-bundle status vocabulary from `tasks/README.md`.
- Task bundles use `Steering Cache`, `Work Map`, `Agent Cache`, and
  `Pruned / Stale` sections.
- Classify retained content as steering cache, current work, agent cache, or
  pruned/stale state.
- Preserve Jörn/Kai/external decisions that change future work.
- Prune stale schedules, obsolete ownership, old packet queues, and derivable
  state.
- Status must match the actual owner and state.
- `[active]` means one session owns the whole work-map item.
- Mark an item `[done]` only when the acceptance check is met or Jörn explicitly closes it.
- Link to `research/*.md` notes, formal files, result docs, commits, or handoffs instead of duplicating evidence.
- Preserve why a task is blocked, stale, deferred, or Jörn-owned.
- Update notes for new evidence, blockers, and resume points. Change a header status or owner only when Jörn assigned or approved that ownership in the current PM surface.
- Update `ROADMAP.md` only when the global overview changes.
- If claim strength changes, update or flag `RESULTS.md`.
- If final done gates change, update or flag `FINAL-VERIFICATION.md`.

## Jörn Gates

Ask Jörn for:

- Thesis priority and what to cut or defer.
- Whether a task surface is shallow enough for agents.
- Which decomposition matches his understanding of the math, thesis, or agent risk.
- Advisor-facing framing and deadline tradeoffs.
- Changing ownership of Jörn-owned or active tasks.

Do not ask Jörn to do project-management labor that agents can do: inventorying files, reading old task entries, comparing options, rewriting roadmap bundles, checking whether paths exist, or drafting concrete choices.

## Stop

Stop and ask when:

- The PM question turns into mathematical judgment, research interpretation, prose taste, or advisor strategy.
- A task decomposition depends on your guess about agent cognitive limits and Jörn has not skimmed it.
- `ROADMAP.md` / `tasks/*.md` and repo evidence disagree.
- The update would change ownership or status for another active session.
- You cannot state the next safe resume point.

## Checks

- `git diff --check`
- If an old tracker file is edited or restored, explain why editing
  `ROADMAP.md` or `tasks/*.md` was not enough.
