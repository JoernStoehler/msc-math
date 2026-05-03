# Harness Objectives Draft

This draft is working material for redesigning the repo-local harness around
GPT-5.5. It is not active project policy.

During this migration, all old harness material is legacy input. Existing
`AGENTS.md` sections, `SKILL.md` files, `.codex/reference/harness/*`, subagent
definitions, task-routing conventions, and prompt packets may still be useful,
but they should be treated as suggestions and evidence to improve upon rather
than as settled target-state policy.

## Purpose

The harness is repo-local agent-facing material whose only value is
instrumental: it helps agents do work that advances Jörn's thesis project.

The harness should be judged by whether it improves thesis-project outcomes,
not by whether it is internally elegant, comprehensive, or process-heavy.

## Style Guardrails

- Define concepts by objective, authority, and observable use.
- Avoid analogies, slogans, and clever summaries when literal prose works.
- Do not make agents reason through metaphors before acting.
- Tie motivation to thesis success, multi-session continuity, or known
  high-cost agent failure modes.
- Distinguish suggestions, context, and binding constraints.
- Treat constraints as justified only when they are part of the objective or
  prevent a failure expensive enough to warrant durable instruction.

## Objective Shape

Each harness objective should state:

- the agent capability it supports;
- why that capability is necessary for thesis success;
- what repo surface or workflow should carry the support;
- how an agent or reviewer can tell whether it worked.

## Candidate Objectives

- Agents can find the current authority surface for a task quickly.
  Motivation: prevents stale-map edits, duplicate planning, and questions whose
  answers are already in repo state.
- Agents can separate domain truth, task state, historical notes, temporary
  packets, and active instructions.
  Motivation: prevents temporary or historical text from becoming accidental
  policy or thesis evidence.
- Agents can convert Jörn's objectives into concrete work without requiring
  rigid step-by-step process for ordinary cases.
  Motivation: GPT-5.5 can usually choose a path when the outcome, success
  criteria, allowed side effects, and evidence rules are clear.
- Agents can verify work at the right layer.
  Motivation: build success, mathematical correctness, data freshness, claim
  support, and reproducibility are different thesis risks.
- Agents can maintain long-running project continuity.
  Motivation: thesis work spans many sessions, agents, task bundles, and
  partially settled research directions.
- Agents can reduce Jörn bottlenecks without hiding decisions from Jörn.
  Motivation: agents should spend time on inspection, narrowing choices,
  verification, and cleanup, while reserving mathematical judgment,
  advisor-facing framing, taste, and external actions for Jörn.
- Agents can improve the harness without making short-term task success damage
  long-term harness health.
  Motivation: harness text steers repeated future sessions, so local fixes
  should not accumulate stale assumptions, over-specific processes, or broad
  constraints copied from older models.

## GPT-5.5 Working Assumptions

These assumptions should be validated against current OpenAI guidance and local
experience before promotion:

- Prefer outcome-first harness text: state objective, success criteria, allowed
  side effects, evidence rules, authority boundaries, and output shape.
- Avoid detailed step-by-step process unless the exact path matters.
- Remove older-model scaffolding when it only compensated for weaker planning,
  weak tool choice, or weak instruction following.
- Keep enough orchestration for complex coding work: reuse expectations,
  delegation boundaries, verification expectations, acceptance criteria, and
  stop/ask conditions still matter.
- Treat GPT-5.5-specific claims as refreshable assumptions, not timeless repo
  facts.

## Draft Design Direction

Suggested harness shape to evaluate against the objectives:

- `AGENTS.md`: thin always-loaded repo map and broadly useful project context.
- `MAP.md` / `INDEX.md`: descriptive navigation caches, not hidden policy.
- Skills: repeatable procedural knowledge and convention sets.
- `.codex/reference/`: durable reference notes too detailed for `AGENTS.md`
  and too broad for one skill.
- `ROADMAP.md` / `tasks/*.md`: durable task routing and current task state.
- `/tmp/`: temporary prompts, worker packets, draft reports, and artifacts to
  inspect or show Jörn.

## Migration Questions

- Which existing skill text is still needed for GPT-5.5?
- Which constraints should become context or suggestions?
- Which historical notes should be deleted, archived, or marked as historical?
- Which skill names, descriptions, or trigger scopes cause wrong routing?
- Which domain conventions belong in domain skills, and which are just local
  implementation details?
- Which absent conventions should be added because they prevent repeated
  high-cost failures?
