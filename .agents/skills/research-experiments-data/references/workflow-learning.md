# Experiment Workflow Learning

Read this only during an explicitly authorized workflow-review or skill-editing
task. Ordinary packet work should use the current skill and may record a
candidate process observation at the packet owner when an observed failure or
benefit is plausibly reusable; do not create process notes after every packet.

## What To Record

A reusable candidate should name:

- the observed failure or benefit, not a hypothetical preference;
- the task and downstream transition where it occurred;
- what the agent, self-review, or reviewer missed or caught;
- whether two checks overloaded one review, duplicated effort, or usefully
  shared the same evidence;
- the likely recurrence and cost of the failure;
- the smallest default, reminder, review question, invariant, or hard gate that
  could address it;
- possible false positives or extra work the change could encourage;
- a concrete future packet on which to test the change.

Do not promote a stylistic preference or one-off packet fact into the skill.

## Choosing Intervention Strength

Use the weakest mechanism likely to prevent the observed failure. Possible
interventions include a retained observation, a reminder, a default review
question, an expected invariant with explicit exceptions, or a hard promotion
gate. These are points on a spectrum, not a required maturity sequence or a
complete classification.

Increasing or decreasing intervention strength should be evidence-driven. Name
the observed burden or false-positive pattern when removing a constraint.
Never weaken a gate to make the packet currently under review pass.

## Review Architecture

Choose review structure from failure independence and context needs:

- combine checks when they inspect the same source and neither is likely to be
  crowded out;
- split technical/provenance review from interpretation review when each can
  pass while the other fails;
- use a fresh-context reviewer when successful use by a fresh downstream
  session is part of readiness;
- use a context-rich parent or reviewer when a stakeholder decision or subtle
  epistemic boundary is not durable yet;
- use repeated narrow reviews only when their expected error reduction exceeds
  coordination and wall-time cost.

Record which checks disappeared under overload and which combinations reduced
duplication. Do not infer that more reviewers are always safer.

## Change Gate

Skill changes should be a separate reviewable diff. Check trigger precision,
overlap with nearby skills, duplicated owner-local truth, accidental work
creation, and preservation of Jörn/Kai gates. Validate the skill and forward-
test consequential changes on a realistic packet before merging. Jörn reviews
the accumulated process knowledge before it becomes main-branch behavior.
