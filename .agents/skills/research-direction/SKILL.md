---
name: research-direction
description: Use for research framing, methodology selection, experiment interpretation, proof-route status, thesis-scope tradeoffs, evidence gaps, or deciding what evidence would answer a mathematical or thesis-facing question.
---

# Research Direction

This skill is a combinable protocol for research-direction work. Use it when
the task needs a research surface before execution: question, hypotheses,
evidence, method failures, thesis implication, and Jörn-only judgment points.

Research interpretation and proof-route state should become first-class repo
artifacts, usually under `research/`, not only task rows or chat summaries.

## Core Principle

Build the research surface before proposing an experiment, proof task, literature check, or implementation. First state what question is being answered, what evidence would change the answer, and which decisions require Jörn.

Agents may gather evidence, test implementations, check sources, and draft options. Jörn owns mathematical judgment, thesis priorities, advisor-facing framing, and final interpretation.

## Research Surface

State:

- Research question.
- Candidate answers or hypotheses.
- Evidence needed to distinguish them.
- Method options and their failure modes.
- Existing files, data, papers, or formal labels that matter.
- Jörn-only decisions.
- Evidence-producing next steps after Jörn chooses the research surface.

Route task graph work to `$roadmap-maintenance`: prioritization, bundling,
sequencing, ownership, `ROADMAP.md` / `tasks/*.md` edits, or deciding which
agents should take which units.

Use `$subagent-delegation` after the output contract is clear for a bounded evidence-gathering, source-checking, implementation, or review task.

## Method Discipline

- Separate observation, inference, and speculation.
- Prefer checks that can falsify a hypothesis over work that only adds detail.
- Keep failed methods in the record with the reason they failed.
- Do not upgrade an empirical pattern into a theorem, or a proof sketch into an accepted proof.
- Preserve thesis implications: state whether the result appears to affect the main argument, supporting evidence, exposition polish, or future work, and surface uncertain classification to Jörn or `$roadmap-maintenance`.

## Jörn Gates

Ask Jörn for:

- Which research question matters for the thesis.
- Whether a proof idea is mathematically acceptable.
- Whether an empirical result supports the intended interpretation.
- Whether a method is worth the time or compute cost.
- How to frame a result for Kai, Elizabeth, or the thesis.

Do not ask Jörn to do agent labor inside the approved research surface: reading local sources, checking citations in downloaded papers, running small experiments, comparing methods, separating observation from inference, or preparing concrete research options.

## Stop And Surface

Stop and report back when:

- The question changed.
- The method would require a new experiment family, long compute run, or thesis-scope shift.
- The evidence cannot distinguish the candidate answers.
- A local result contradicts `RESULTS.md`, `ROADMAP.md`, `tasks/*.md`, formal sources, or committed data.
- A proof or interpretation depends on a gap that only Jörn can judge.
