---
name: goal-tool
description: Use before creating, updating, checkpointing, resuming, blocking, or completing `/goal`. This skill covers `/goal` mechanics and status accounting; use `scoping` to choose the target and `charter-writing` to write or review a charter.
---

# /goal Tool

`/goal` recalls the objective and accounts goal status. It is not the source of
truth for a complex objective.

Simple explicit objectives can fit in the `/goal` text. For nontrivial,
long-running, high-cost, or easily confused work, point `/goal` to a charter
instead of compressing the objective into the tool field.

Use `$scoping` if the target is still too large, vague, or possibly a useful
precursor rather than the live problem. Use `$charter-writing` before writing or
materially revising the charter.

Good `/goal` objective:

```text
Execute the objective charter at <path>. Mark complete only under the charter's stopping conditions.
```

For high-stakes or easily confused work, include the objective phrase too:

```text
Execute the objective charter at <path>: <short objective>. Mark complete only under the charter's stopping conditions.
```

## Before Creating `/goal`

- Create `/goal` only when explicitly requested by Jörn or by system/developer
  instructions.
- For nontrivial, long-running, high-cost, or easily confused goals, make sure
  the objective source is an appropriate charter or self-contained equivalent.
- If the target is smaller than the live problem, confirm that the charter or
  objective text declares that scope choice.
- For a chartered or otherwise complex objective source, follow the
  `$charter-writing` review gate. Do not proceed with an objective whose target,
  scope, or expert judgment still needs Jörn.
- Before starting a chartered `/goal` loop, ask Jörn for feedback, corrections,
  or additions to the charter. It is usually cheaper to fix the objective before
  the loop starts than to discover the mismatch after a long run; while the loop
  runs, Jörn may not be available.

## During `/goal`

- Preserve the charter's objective, context, constraints, and stopping
  conditions. Do not silently replace them with a cheaper local target.
- If evidence shows the charter target is wrong or too low, do not complete
  against the stale target. Stop and report the mismatch, re-scope or revise the
  charter if current instructions allow it, or mark blocked only when the tool
  rule applies.
- After compaction or interruption, re-read the charter or objective source
  before continuing. Reconstruct changed state from source truth, artifacts, and
  tool state; do not trust earlier assistant confidence as source truth.
- Use `blocked` only under the active `/goal` tool rule. Current rule: the same
  blocker has repeated for at least three consecutive goal turns, no meaningful
  progress is possible, and user input or external state is required.

## Before Marking Complete

Mark complete only when the objective source's stopping conditions are actually
met.

Do not mark complete because:

- budget is nearly exhausted;
- partial work is useful;
- visible artifacts exist;
- tests passed;
- a useful precursor was completed;
- the agent knows the next step.

Compare the result to any declared scope choice. Important remaining work is
acceptable only if it was deliberately out of scope and the target was
completed.

If the user or objective source requires subagent or programmatic review, it
must pass after the final relevant changes or be explicitly superseded and
accounted for.

When completing a budgeted goal, report final token/time usage from the tool
result.
