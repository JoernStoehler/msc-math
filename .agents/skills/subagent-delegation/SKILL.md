---
name: subagent-delegation
description: "Subagent delegation workflow: inside an approved session surface, choose whether bounded work should stay local or move to explorer/worker/reviewer subagents, write narrow briefs, keep the main thread focused, verify delegate output, and use parallel delegation only when scopes are independent. Use when spawning or managing subagents after Jörn asks for delegation, subagents, or parallel agent work."
---

# Subagent Delegation

Use subagents to move bounded work out of the main thread after the active focus has defined the surface. Do not use delegation to choose the focus, hide unclear scope, bypass Jörn gates, or outsource conceptual depth without an output contract.

## Delegate Or Keep Local

Delegate when the subtask is:

- Concrete, self-contained, and easy to state.
- Not the immediate blocking step on the critical path.
- Verifiable from files, commands, or source citations.
- Bounded by a read-only surface or disjoint write scope.
- Useful even if it returns after you make local progress.

Keep work local when:

- The next action depends on the result.
- The task needs tight context, mathematical judgment, thesis-scope judgment, or taste.
- The prompt would combine several concepts into one fused objective.
- Verification would be indirect or hard to check locally.
- A failed or confused delegate would cost more than doing the work locally.

## Delegate Brief

Each subagent prompt should name:

- Approved surface this subtask belongs to.
- Objective.
- Files, directories, or question scope.
- Read-only or write ownership.
- Success condition and verification command or check.
- Output format needed by the main thread.
- What the delegate must not decide.
- Stop condition that requires returning to the main thread.
- For workers: "You are not alone in the codebase; do not revert or overwrite changes made by others."

Do not duplicate the same unresolved task across delegates.

## Running Delegates

- Do the immediate blocking local work yourself.
- Start sidecar delegates for non-blocking search, review, verification, or disjoint edits.
- While delegates run, continue useful non-overlapping local work.
- Wait only when their result is needed for the next step.
- Verify delegate claims against files, commands, or sources before presenting them as facts.
- For multiple tasks, prefer a serial queue unless the subtasks are independent: brief A, integrate and verify A, then brief B. Do not fuse A+B into one delegate prompt merely because both are available in the same session.

## Parallel Delegation

Parallelize for speed only when the subtasks are independent:

- Separate read-only questions.
- Separate review surfaces.
- Disjoint write scopes with named file ownership.
- Independent verification checks that can return in any order.

Do not parallelize tasks that share a design decision, write the same files, or require one delegate's result before another can start. Split by surface first; merge results only after local verification.

Name the main thread as integration owner. Parallel delegates report evidence or patches; they do not reconcile conflicts, choose priorities, or change the approved surface.

## Integration

When a delegate returns:

- Check the referenced files or commands before accepting findings.
- Apply or refine worker changes in the main thread if needed.
- Record blockers as concrete missing evidence or Jörn-only decisions.
- Close agents that are no longer needed.
