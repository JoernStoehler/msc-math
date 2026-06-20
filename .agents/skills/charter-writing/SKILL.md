---
name: charter-writing
description: "Use when writing, reviewing, revising, or replacing a charter: an objective anchor intended to govern a long autonomous or resumable GPT-5.5 work loop, including `/goal` charters. Do not use for ordinary implementation plans, status reports, or task notes unless they are meant to serve as that control anchor."
---

# Charter Writing

A charter stabilizes a scoped target. It should let a long or resumable GPT-5.5
work loop return the intended outcome instead of a cheaper nearby substitute.

Chartering is not scoping. If the target is still too large, vague, or only a
precursor, use `$scoping` first or make the scope choice explicit in the
charter.

## Preserve

A charter should preserve the objective without the `/goal` field's concision
pressure:

- the user's active target, without rewriting it into a cleaner but different
  objective;
- the scoped objective and why it serves thesis success;
- the live problem or decision the objective advances;
- current state, source truth, evidence, and epistemic status for important
  claims;
- constraints, Jörn approval gates, and worktree/main boundaries;
- relevant alternatives when success depends on comparing options;
- useful outputs that would still be insufficient completion;
- stopping conditions tied to the intended outcome, not only activity performed.

Keep current state, objective, constraints, evidence, and stopping conditions
separate:

- current state says what is known now;
- objective says what the loop must accomplish;
- constraints say what must remain true while accomplishing it;
- evidence says what claims the loop may rely on and with what confidence;
- stopping conditions say how completion or blocked status will be recognized.

Draft difficult charters in `/tmp` so they can be edited before they become the
anchor for a long loop.

## Scope And Epistemic Status

If the target is smaller than the live problem, include the larger objective,
this target, why the smaller target is worth the loop cost, what remains
unsolved, and what it unlocks. If that is hard to defend, the target is probably
too low or needs Jörn's judgment before starting.

Mark known facts, source-backed claims, Jörn steering, prior scratch, inferences,
guesses, examples, likely approaches, hypotheses, and hard constraints
differently. Do not turn "this might be useful" into "the solution must have
this shape" unless that constraint is justified.

## Charter Review Gate

Before using a charter for `/goal` or another long loop, check:

- active target is preserved;
- scope choice is explicit when the target is smaller than the live problem or
  could be confused with a precursor;
- current state, objective, constraints, evidence, and stopping conditions are
  not blurred;
- important claims have source truth and epistemic status;
- tempting but insufficient outputs are named when they are not completion;
- stopping conditions are close enough to sufficient, not just necessary;
- open search has an evaluation target and cost-aware stopping, not fixed
  quotas or a pre-decided answer shape;
- Jörn review is requested only if objective, scope, stakeholder priority, or
  expert judgment remains unresolved.

Bad charter smells:

- it reads like a status report, roadmap, or implementation plan;
- it can be completed by producing a report, table, tool, or first result while
  the live decision remains open;
- it asks the loop to search or iterate, but lets the first batch of results
  count as completion without saying what makes the search sufficient;
- it lists activities but no outcome that makes stopping legitimate;
- it depends on unstated Jörn judgment or hidden source truth;
- it makes guesses, examples, or likely approaches sound like hard constraints;
- it says to run a review, test, or comparison but not that the result must pass
  or be accounted for.

For difficult charters, use fresh-agent review when available and worth the
cost. Ask what was clear to that reviewer, what they had to infer, what they
misread, what seemed obvious, irrelevant, unmotivated, or too strong, and which
constraints lack an argument that every best solution should satisfy them.

## Open Search

For open-ended search, do not pre-decide the answer form unless evidence already
justifies it. State what correctness or utility the final hypothesis or decision
must support, what evidence or counterexamples matter, what outputs are
insufficient by themselves, and when another round of search is no longer worth
its cost.

A current-state report, diagnostic script, or one tested hypothesis is
insufficient unless the charter deliberately scopes to that groundwork and says
what search work remains.

## Boundaries

- A charter is not an implementation plan. Include process only when it is
  necessary for correctness, safety, required tooling, or the interaction
  contract.
- Session-control charters usually live in `/tmp`, not git. Use a tracked repo
  artifact only when Jörn asks for durable project state or when the charter is
  intentionally becoming a durable task, experiment, or harness artifact.
- Use `$goal-tool` before creating, updating, checkpointing, or completing a
  `/goal`.
