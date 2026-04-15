---
name: subagent-delegation
description: "Subagent delegation workflow: proactively move bounded, verifiable side work to explorer/worker/reviewer subagents while keeping the top-level session responsible for integration and verification. Use when a task has independent read-only searches, disjoint implementation slices, review surfaces, or parallel checks; load before spawning or managing multiple subagents."
---

# Subagent Delegation

Use subagents for bounded first-pass labor after the active task or focus surface is clear. The top-level session keeps integration and correctness ownership.

Treat delegate output as evidence to check. Before presenting a delegate claim as fact, verify it against files, commands, sources, tests, or a bounded review result.

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
- The subtask has no bounded output contract, or the delegate would have to choose the task boundary.
- The result cannot be verified cheaply enough for the top-level session to own it.
- A failed or confused delegate would cost more than doing the work locally.

Use narrow subagents proactively for low-risk side work.

Ask Jörn before delegation changes the approved task surface, task ownership, thesis/research direction, or merge readiness.

## Delegate Brief

Each subagent prompt should name:

- Approved surface.
- Objective.
- Scope: files, directories, or question.
- Ownership: read-only or named write scope.
- Success check.
- Output format.
- Decisions reserved for the main thread or Jörn.
- Stop condition.
- For workers: "You are not alone in the codebase; do not revert or overwrite changes made by others."

Do not duplicate the same unresolved task across delegates.

## Running Delegates

- Do the immediate blocking local work yourself.
- Start sidecar delegates for non-blocking search, review, verification, or disjoint edits.
- While delegates run, continue useful non-overlapping local work.
- Wait only when their result is needed for the next step.
- Verify delegate claims against files, commands, or sources before presenting them as facts.
- For multiple tasks, default to a serial queue: brief A, integrate and verify A, then brief B. Parallelize only independent subtasks.

## Verification Labor

Verification labor can be delegated; verification responsibility cannot.

Good delegated verification tasks:

- Review a file for clear writing, stale paths, proof gaps, claim/source mismatches, or convention violations.
- Run a command and report the exact failure or success.
- Compare a claim against `RESULTS.md`, `TASKS.md`, formal sources, downloaded papers, or generated data.
- Inspect a patch for a named risk surface.

The top-level session decides whether the evidence is enough. When reporting verification, name the evidence the top-level session checked and state any residual risk.

## Parallel Delegation

Parallelize for speed only when the subtasks are independent:

- Separate read-only questions.
- Separate review surfaces.
- Disjoint write scopes with named file ownership.
- Independent verification checks that can return in any order.

Do not parallelize shared design decisions, shared write scopes, or dependent tasks. Split by independent surface, then merge results only after local verification.

Name the main thread as integration owner. Parallel delegates report evidence or patches; they do not reconcile conflicts, choose priorities, or change the approved surface.

## Integration

When a delegate returns:

- Check the referenced files or commands before accepting findings.
- Apply or refine worker changes in the main thread if needed.
- Record blockers as concrete missing evidence or Jörn-only decisions.
- Close agents that are no longer needed.
