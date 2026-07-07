---
name: planning-strategy
description: "Use when Jörn asks for a visible plan before implementation, or when the objective is known and a wrong plan could waste about 30+ minutes of Jörn recovery, discard about 2+ hours of Codex work, choose between action routes that produce different artifacts or answer different questions, let a secondary objective overtake the primary objective, test a workflow/harness strategy that may write outside scratch or create trusted artifacts, or repeat a planning-shape failure. If choosing the objective or slice is still the issue, use scoping first. Do not use for ordinary narrow implementation tasks where one route matches the request, rollback is a local patch/test rerun, and Jörn did not ask for a visible plan."
---

# Planning Strategy

Use this skill to choose the execution strategy after the objective is known.
It is a dependency-ordered check, not a list of planning tips. Its job is to
prevent expensive wrong-objective or wrong-route work while preserving fast
direct execution for bounded tasks.

## 1. Target Gate

First decide what kind of choice is live.

- If the objective, thesis slice, or target height is unclear, use `$scoping`
  instead of this skill.
- If the objective is known and the question is how to execute it, continue.
- If Jörn explicitly asks for a visible plan before implementation, use this
  skill and do not change tracked project files or generated project artifacts
  until Jörn approves the plan or asks you to implement. Scratch work outside
  tracked project content is allowed, for example under `/tmp`.

This gate dominates the rest. Do not use execution-strategy planning to choose
the objective.

## 2. Trigger Gate

Use this skill only when one or more are true:

- Jörn explicitly asks for a visible plan before implementation.
- A wrong route would likely cost Jörn about 30+ minutes to notice, explain, or
  recover, or would discard about 2+ hours of Codex work.
- Available action routes would produce different artifacts, answer different
  questions, or spend the next 2+ hours in different repo areas.
- The task combines objectives whose dominance is unclear.
- A secondary goal could silently overtake the primary objective.
- Expected synergy between goals is being assumed rather than estimated.
- The work is a workflow, harness, or delegation-strategy change that may write
  outside scratch, create a trusted artifact, consume 2+ hours of Codex work,
  or consume 30+ minutes of Jörn attention after the objective is already
  chosen.
- The work chooses which evidence-producing proof, experiment, code path, or
  report to spend 2+ hours of Codex work or 30+ minutes of Jörn attention on
  after the objective is already chosen.
- A previous attempt failed by planning shape, stopping condition, or reporting
  against the wrong objective.

Do not use this skill when all of these are true:

- one route matches the request;
- rollback is a local patch/test rerun;
- no objective tradeoff is live;
- Jörn did not ask for a visible plan.

Also do not invoke this skill only because a prior successful bounded workflow
already fixes the route, hard boundary, and report target. Preserve that
bounded behavior.

When editing this or another trigger surface, audit the trigger text itself:
every trigger must be observable from the request, repo state, planned action,
or a stated cost threshold. Do not use phrases such as "materially affects
thesis success", "high leverage", or "important" as a cutoff unless the same
sentence operationalizes them.

## 3. Objective Stack

Before comparing routes, state:

- the primary objective;
- secondary objectives;
- the dominance rule when objectives conflict.

If the dominance rule is unclear and local context cannot decide it, ask Jörn
for that crux. Do not continue by optimizing the easiest concrete artifact.

## 4. Route And Artifact Map

List routes by what each would produce or answer:

- artifact or decision each route would produce;
- repo area or source surface each route would spend time in;
- what is not done if this route is chosen;
- whether the route combines goals or keeps them separate.

Include these routes when available:

- do not combine goals;
- do not act yet;
- ask Jörn for one crux;
- delegate bounded extraction, review, or variant spike;
- get local evidence first.

Treat "combine goals" as a route that needs justification. Nearby work is not
synergy. Combined goals should share evidence, source surfaces, or stopping
criteria; otherwise keep them separate.

## 5. Constraints And Trust Map

Record constraints that can make an otherwise useful route wrong:

- worktree/main boundaries;
- quarantine paths such as `/tmp`;
- no-edit surfaces;
- artifact ownership and destination;
- scratch vs durable note vs source truth vs publishable thesis text;
- review output vs production patch.

These are success/failure criteria, not admin details.

## 6. Value, Cost, And Evidence

Compare routes by:

- expected thesis value;
- Codex time;
- Jörn attention and recovery cost;
- opportunity cost of not choosing each serious route;
- chance of producing artifacts that look useful but do not advance the primary
  objective;
- local evidence obtainable in less than about 15 minutes.

If under-15-minute local evidence can decide between routes before a route would
spend 2+ hours of Codex work or 30+ minutes of Jörn attention, get that evidence
first.

Ask Jörn only for cruxes where his expertise, stakeholder priority, private
context, or cross-session state is likely worth the attention cost. Do not ask
permission when local inspection, local tests under about 15 minutes, or a
bounded subagent can answer the uncertainty.

## 7. Success, Failure, And Falsifiers

Before acting, define:

- success criteria tied to the primary objective;
- failure criteria that should make the agent stop, rescope, ask Jörn, or
  abandon the route;
- stop/report criteria measured against the primary objective;
- facts, repo evidence, test results, review findings, or Jörn answers that
  would make the strategy wrong.

A produced artifact is not success unless it satisfies the primary objective.

## 8. Placement And Continuation

Put the strategy check where the task needs it:

- scratch for ordinary autonomous work;
- chat only when Jörn requested a visible plan or a Jörn-only crux is needed;
- a charter or durable note only for long or resumable loops;
- a subagent only for bounded extraction, review, or variant exploration whose
  result main can judge.

For ordinary autonomous work, do not report the strategy check as the
deliverable. Use it to choose the route, then keep working until the
stop/report criteria or a Jörn-only crux is reached.

Main owns objective dominance, tradeoff synthesis, final stop/report decisions,
and merge-readiness.

## 9. Reporting

Report against the primary objective and the stop/report criteria, not against
the most concrete artifact produced. If the work deliberately stops short of
the larger objective, say what remains unsolved and why stopping there is still
the right tradeoff.

When this skill triggers for a prompt, harness, workflow, or
delegation-strategy change, include the review pass and local checks that
tested the strategy against the motivating failure. Syntax or package
validators are hygiene checks, not evidence that the planning surface solves
the motivating failure.

For small implementation tasks where this skill did not trigger, do not add
ritual planning summaries.

## 10. Benchmark Feedback

When updating or reviewing this skill, read
`references/benchmark-method.md` and `references/benchmark-v0.md`. They record
the local trace-based eval method and the first audited benchmark. Do not load
them for ordinary task use.

Do not mine Codex session logs by default. If planning failures or success
examples need transcript evidence for a durable diagnosis, use
`$codex-session-log-parsing`, treat raw rollout JSONL as source truth, extract
only the focused structural evidence needed, and avoid dumping private
transcripts.
