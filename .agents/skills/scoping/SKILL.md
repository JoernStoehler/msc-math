---
name: scoping
description: "Use when choosing or reviewing what objective to pursue: especially when the request is too large or vague, has multiple plausible slices, or risks choosing a useful but too-small objective instead of the live problem. Do not use for ordinary implementation planning after the objective is clear."
---

# Scoping

Scoping chooses the target. It is not implementation planning, charter writing,
or `/goal` status accounting.

Use this skill when the next objective itself is unclear or when a smaller slice
may be chosen instead of the larger live problem.

## Target Height

A target can be coherent and useful while still being too low for the autonomous
unit of work being launched or reviewed. The agent completes a precursor,
report, diagnostic, or first slice, but the main missing capability or decision
remains mostly untouched.

Distinguish:

- wrong-direction drift: the target is not the user's real objective;
- too-low target: the target is related and useful, but completing it leaves the
  real bottleneck essentially intact.

## Choose The Target

Compare plausible target sizes and routes by project value, cost, risk, and
thesis-success impact. The point is not quantified ceremony; it is to avoid
choosing the easiest coherent artifact when it leaves the live bottleneck
intact.

For thesis-scope choices, inspect repo source truth before asking Jörn:
`FACTSHEET.md`, `tasks/README.md`, `tasks/current-state.md`,
`tasks/planning-notes.md`, and the relevant slice `MAP.md`, `README.md`, or
source artifacts. Treat maps as navigation caches, not source truth.

Before treating a slice as the target, check:

- What live problem or decision does this advance?
- If this target is completed, is the main remaining work still the live
  problem?
- Should this be its own work loop, or supporting work inside a higher target?
- What is the cost of taking the larger target now?
- What is the cost of doing only this smaller target and needing another loop?
- What larger, smaller, or differently routed targets are plausible?
- What uncertainty controls the choice, and can the repo answer it cheaply?

If the target is deliberately smaller than the live problem, make explicit:

- the larger objective;
- this target;
- what remains unsolved;
- why that remainder is acceptable;
- what this target unlocks.

Name tempting outputs that are useful but insufficient when the risk is real:
current-state reports, first diagnostic scripts, one tested hypothesis, one
passing test, open-question lists, or plans that postpone the core decision.

Ask Jörn when the missing input is expert judgment or stakeholder priority and
the cost/value case is not obvious. Do not ask permission questions when repo
inspection or ordinary agent work can resolve the uncertainty.

## Boundaries

- Do not turn scoping into a full roadmap unless the user asks for roadmap work.
  Usually choose the next target, not the next fifty sessions.
- Do not write an implementation plan unless an active planning instruction or
  the user asks for a plan.
- Use `$charter-writing` after the target is chosen and needs to remain stable
  across long autonomous work.
