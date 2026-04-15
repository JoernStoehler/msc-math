---
name: subagent-delegation
description: "Subagent delegation workflow: proactively move bounded, verifiable side work to explorer/worker/reviewer subagents while keeping the top-level session responsible for integration and verification. Use when a task has independent read-only searches, disjoint implementation slices, review surfaces, or parallel checks; load before spawning or managing subagents."
---

# Subagent Delegation

Use subagents to move bounded work out of the main thread after the active task or focus surface is clear. Do not use delegation to choose the focus, hide unclear scope, bypass Jörn gates, or outsource conceptual depth without an output contract.

Delegation changes who does first-pass labor. It does not change who is responsible for correctness. Treat delegate output as untrusted input until the top-level session verifies it against files, commands, sources, tests, or a bounded review result.

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
- The result cannot be verified cheaply enough for the top-level session to own it.
- A failed or confused delegate would cost more than doing the work locally.

Do not wait for Jörn to request subagents for narrow, low-risk side work. Do ask Jörn before delegation changes the approved task surface, task ownership, thesis/research direction, or merge readiness.

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

## Verification Labor

Verification labor can be delegated; verification responsibility cannot.

Good delegated verification tasks:

- Review a file for clear writing, stale paths, proof gaps, claim/source mismatches, or convention violations.
- Run a command and report the exact failure or success.
- Compare a claim against `RESULTS.md`, `TASKS.md`, formal sources, downloaded papers, or generated data.
- Inspect a patch for a named risk surface.

The top-level session decides whether that evidence is enough. Do not say "verified" merely because a subagent said so. Say what evidence the top-level session checked, or state the residual risk.

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
