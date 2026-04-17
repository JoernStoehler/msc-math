---
name: improvement-focus
description: "Session focus for simplification and maintainability-improvement passes: scout complexity hotspots, triage what is actually worth touching, split bounded work packets, run parallel worktree execution, review results, and integrate only safe wins. Use when Jörn asks for a cleanup, simplification, refactor-for-clarity, or maintainability pass rather than new functionality."
---

# Improvement Focus

You are the top-level session talking with Jörn. Your job is to turn a vague
"simplify this area" request into a short list of real improvement packets,
execute the shallow ones safely, and discard the ones whose boundary was not
actually good.

Use this focus when the goal is to make the codebase simpler, clearer, or
easier for later agents to modify without changing the research surface.

This focus owns:

- the shortlist of real improvement opportunities
- the execution packets for workers
- the keep/discard filter after review
- the integration and verification sequence

It does not own new research, broad architecture redesign, theorem changes, or
"clean up everything" wish lists.

Route thesis-task tracking and ownership changes to `$project-management-focus`.
Route proof or interpretation questions to `$research-focus`.

## Default Shape

Prefer this order:

1. Read enough code to find repeated glue or mixed-concern files.
2. Write down a triaged packet list before spawning workers.
3. Create a dedicated feature worktree for integration.
4. Create one worktree per accepted packet.
5. Run workers on disjoint write scopes.
6. Review each packet before keeping it.
7. Merge only the kept packets into the feature worktree.
8. Re-run focused verification on the integration branch.

Do not merge improvement packets directly into `main` unless Jörn explicitly
chooses that tradeoff in the current turn.

## What To Look For

Bias toward:

- repeated experiment-local helper logic that belongs in
  `experiments/<topic>/src/lib.rs`
- mixed-concern binaries where one real stage can be extracted cleanly
- duplicated run plumbing, JSONL writing, mode parsing, or traversal helpers
- duplicated internal library helpers with the same mathematical stage

Bias away from:

- large files that are only "big" because the math is genuinely dense
- broad generic abstractions created only to reduce line count
- library promotion of experiment code unless the boundary is already settled
- cleanup that would require re-litigating architecture choices first

## Method

Start from file evidence, not from aesthetic complaints. A packet is worth
running only when you can point to one of these:

- repeated code across multiple callers
- an existing helper crate or module that is clearly underused
- a mixed-concern file with a real stage boundary
- an advertised API or workflow branch that the codebase does not truly use

Prefer "make later edits cheaper" over "make this diff look clever."

## Triage Output

Before execution, write two files under `scratch/`:

1. `improvement-<date>-packets.md`
2. `improvement-<date>-worker-context.md`

The packet file should contain three buckets:

- `Implement Now`
- `Look Deeper Before Implementing`
- `Probably Leave Alone`

For each implementation packet, record:

1. **Unit of work**
   - Worktree: short branch name.
   - Scope: exact files.
   - Why now: the concrete simplification seam.
   - Expected shape: what kind of refactor is allowed.
   - Verification: exact command.
   - Risk: what must stay local or unchanged.
   - Keep/discard bias: what would make this packet not worth keeping.

The worker-context file should state:

- goal of the round
- non-goals
- audit summary
- working rules
- required output format

This avoids forcing later workers to reconstruct the conversation.

## Worktree Discipline

Create a dedicated integration worktree first, for example:

```bash
git worktree add -b improvement-<topic>-exec .codex/worktrees/improvement-<topic>-exec main
```

Use the root checkout or local `main` only as a read-only coordination surface
 unless the task explicitly targets `main`.

Then create one worktree per accepted packet from the same base branch.

Packets must have disjoint write scopes. If two packets want the same file,
they are not parallel packets yet.

## Worker Briefing

Use workers for implementation and reviewers for packet review.

Worker prompts should always specify:

- required cwd
- exact owned files
- success command
- stop condition
- "You are not alone in the codebase; do not revert or overwrite changes made
  by others."

For improvement work, good stop conditions are:

- "stop if this turns into callback soup"
- "stop if this starts homogenizing real policy differences"
- "stop if preserving behavior becomes unclear"

These guardrails are high-value because they prevent bad abstractions, not
because the model could not imagine them.

## Jörn Gates

Ask Jörn for:

- whether an improvement round should stay experiment-local or change a library
  boundary
- whether a packet is worth doing if the payoff is mostly structural
- whether to stop after the safe wins or keep pushing into deeper refactors
- whether to merge to a feature branch or, exceptionally, directly to `main`

Do not ask Jörn to do agent labor inside an approved improvement round:

- finding repeated helpers
- writing packet files
- splitting disjoint worktree scopes
- reviewing local diffs for obvious regressions
- comparing the reviewed packet against the dirty integration branch

## Hard-Won Tactics

Keep these behaviors:

- Prefer existing topic helper crates as the extraction target before inventing
  new library APIs.
- Treat reviewer-found hygiene issues as worth one bounded retry when they are
  cheap and local: lost formal labels, drift-prone duplicated constants, missing
  imports, or stale helper comments.
- Do not throw away slow workers only for being slow. Cull only for scope
  drift, weak verification, or a bad patch.
- Close stale explorer/audit agents before spawning reviewers if you hit the
  live-agent cap. Reviewer slots are usually more valuable than keeping old
  read-only agents open.
- When one packet is already dirty on the integration branch, diff it against
  the reviewed worktree version before merging. Normalize to the reviewed
  version instead of guessing.

## Review Standard

Run review on every kept packet before integration.

Review for:

- bugs or behavioral regressions
- loss of math-code correspondence
- accidental abstraction widening
- output-path or checkpoint compatibility risk
- whether the extracted helper is a real shared stage

If review finds only a bounded local issue, allow one retry. If review shows
the packet boundary was wrong, discard the packet instead of repairing a complex
refactor in place.

## Integration

Integrate accepted packets one by one into the integration worktree.

After each merge, or after a small batch of disjoint merges, rerun the packet
verification commands on the integration branch. Then run a final `git status`
check before asking Jörn about the result.

When the round is over:

- delete session-only scratch packet files if the durable knowledge moved into a
  skill or `TASKS.md`
- remove merged worktrees
- delete merged packet branches

## Stop

Stop and ask Jörn when:

- the best next packet is no longer shallow
- multiple packets want the same files and require a design decision
- the improvement pass wants to change library boundaries or public policy
- the remaining work is mostly "large, delicate, and low-payoff"

The point of this focus is safe progress now, not exhaustive cleanup.
