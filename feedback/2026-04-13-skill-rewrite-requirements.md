# 2026-04-13 — Handoff For The Next Agent

Use this only for the replacement of the removed `orchestrate` skill.

## Live task

- `TASKS.md`: `Design co-project-owner / coordinator skill`
- Goal: `.agents/skills/<name>/SKILL.md` for the task-graph-clerk work
- Stated constraint from `TASKS.md`: ask Jörn about the workflow before
  guessing; do not design from priors or from this failed session's tentative
  core
- Intended output: minimal skill file, stable core only

## Tasks handed off

1. Replace the removed `orchestrate` skill with one minimal live skill, but
   only after confirming the workflow with Jörn instead of guessing from this
   failed session.
2. Review and resolve the unreviewed repo fallout from this failed session
   before relying on it or building further workflow text on top of it.

## What Jörn said in this session

- Optimize for low review surface area. The issue is review surface and text
  error rate, not token count or newline count.
- Do not overformalize session packets. Keep only the minimum handoff contract.
- Do not push ordinary execution prep up into the top-level coordinator.
- Layer model:
  - layer `-1`: produces prompts that Jörn turns into layer `0` sessions
  - layer `0`: execution session talking directly with Jörn about the task
  - layer `1`: bounded subagents spawned by layer `0`; cannot talk with Jörn
- Use precise words. Avoid vague words such as `role`.
- If one skill is trying to do several different things, split it.
- Do the agent-reviewable passes before asking Jörn to review.
- Do not import `feedback/` into the replacement unless Jörn explicitly asks.

## Already decided by Jörn

- The earlier rewrite/split attempt failed.
- The replacement skills created in that attempt were to be deleted.
- The old `orchestrate` skill was also to be deleted.
- The next step from this failed session is handoff to a fresh, stronger agent,
  not continuing the same design locally.

## Still unresolved

- What the replacement skill should be called.
- Whether the replacement is one skill or a small split that keeps review
  surface low.
- Which of this failed session's edits in `AGENTS.md`, `TASKS.md`, and
  `feedback/skills.md` should be kept versus reverted.

## Current repo state

- `.agents/skills/orchestrate/SKILL.md` is deleted.
- There is currently no replacement skill in `.agents/skills/`.

## Unreviewed fallout from this failed session

- `AGENTS.md` is modified.
  - The added "Do the agent-reviewable passes before pinging Jörn" reminder was
    explicitly requested by Jörn in this session.
  - The terminology change from `Orchestration session` to `Top-level session`
    was my choice and should be reviewed, not assumed.
- `TASKS.md` is modified.
  - The wording changes away from `orchestrate` were my choice and should be
    reviewed, not assumed.
- `feedback/skills.md` is modified.
  - The incident entry was my choice to record the failure pattern.
- This file is new.

Do not assume those edits are correct just because they exist.

## Default source set

- this file
- Jörn's chat in this session
- `AGENTS.md`
- `TASKS.md`

`TASKS.md` currently points at `feedback/2026-04-12-co-ownership-v2-postmortem.md`
for context, but Jörn later said in this session not to import `feedback/`
into the replacement unless he explicitly asks. Treat that as a conflict to
resolve with Jörn, not as automatic input.
