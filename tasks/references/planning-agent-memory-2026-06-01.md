# Planning Agent Memory

Status: dated reference for planning agents. Not source truth. Not a task
queue. Not a thesis-success plan.

Purpose: preserve the non-obvious lessons from the failed planning session on
2026-06-01 without bloating `tasks/planning-notes.md`.

Related recurring-feedback reference:
`tasks/references/recurring-agent-feedback-2026-06-01.md` records broader
Jörn feedback about review targets, stopping conditions, focused questions,
evidence strength, and avoiding local closure. Read it for planning or
multi-step thesis-success work where Jörn would otherwise need to repeat those
instructions in chat.

<!--
Why this exists:
The session failure was not that one prompt was bad. The failure was that a
planning agent produced plausible-looking progress before making the decision
auditable. This file is for future agents doing high-level planning, not for
ordinary execution agents.

Location choice:
Inline `planning-notes.md` would make ordinary execution agents read too much
session-specific failure analysis. `/tmp` would lose useful multi-day memory.
A prompt template would not be found by agents reading task state. A repo-local
skill may be better if this pattern recurs, but this dated reference is the
smallest durable location for now.

Review status:
This file was first committed before its own review rule was applied. After
commit `22b14128`, three independent read-only reviews checked sanity,
reasoning/completeness, and style/epistemics. The follow-up amend addresses
their needs-amend findings; this file remains a dated memory, not source truth.

Confidence: high that these failures occurred; medium that this dated reference
is the best durable location. A repo-local skill may be better later.
-->

## Core Failure

The agent acted before it had a written decision test.

Before making roadmap or next-packet recommendations, write down:

- what the planning session must decide;
- what success measure ranks work;
- which alternatives are being compared;
- which claims are source-backed versus judgment calls;
- what risks or missing facts may invalidate the recommendation;
- what review will check the plan.

If this does not exist, call the output a provisional sketch, not a
recommendation.

<!--
Why include:
This was the primary observed correction. Most later failures followed from the
missing decision test.
Confidence: high.
-->

## Decision Quality

A task is not justified by having positive value. Compare it against
alternatives using:

- value;
- cost;
- opportunity cost;
- Jörn attention/review cost;
- risk and failure modes;
- likely outcome distribution;
- dependencies and parallelism;
- reversibility;
- relevant reference classes from similar tasks.

<!--
Why include:
This prevents "sounds like progress" planning in the observed failure mode.
Confidence: high.
-->

## Epistemics

Planning notes should distinguish:

- source-backed fact;
- inference;
- guess / best current model;
- stale-check-needed;
- Jörn judgment;
- advisor/external context.

Do not use confident style to hide which one a claim is.

## Questions To Jörn

Bad planning questions ask Jörn to decode the agent's model.

Better pattern:

1. extract repo-known facts first;
2. state the current default;
3. ask for the concrete missing fact or expert judgment;
4. say why the answer changes the next action.

Prefer row/table correction questions over abstract yes/no questions.

Example:

- Bad: "Is theorem-strength acceptable?"
- Better after extraction: "This HKO subclaim row has no repo proof surface. Is
  it already known to you, or should it be treated as open?"

<!--
Why include:
Question quality was a major time sink. The rule is simple, but the example is
worth keeping because the failure recurred.
Confidence: high.
-->

## Missing Work

Do not summarize missing work away.

For method-table or story-completion work, every relevant row needs a meaningful
state, such as:

- inapplicable;
- failed on our side and not worth more;
- no useful pattern found;
- meaningful pattern found;
- positive/conjectured-positive follow-up;
- still blocking.

Implementation failure can be a legitimate terminal row only if the reason and
value/cost decision are explicit.

## Review Before Use

High-level planning needs review before being treated as usable:

- sanity review: obvious bad assumptions, omissions, hidden gaps, gloss;
- reasoning/completeness review: can the reviewer reproduce the reasoning;
- style/epistemics review: unclear language, overconfidence, imprecision,
  embellishment.

For subagents: no written brief, no delegation; no written review note, no
accepted result.

## Durable Artifacts

Do not create durable planning or thesis-working artifacts during a planning
dispute unless Jörn explicitly asks.

This warning is about authority leakage during unresolved disagreement. It does
not prohibit intentionally durable side-by-side working material once the
artifact has a plain header, bounded purpose, source-truth limits, and review
gate.

If an artifact is created anyway, prevent authority leakage:

- write a plain file header;
- say what it is and is not;
- say how future agents may use it;
- say what source truth overrules it.

## Jörn Attention

Treat Jörn attention as a scarce project resource, not free validation.

Recommendations should say when they consume Jörn review time and when they
reduce it.
