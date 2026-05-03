<!--
Purpose: historical inventory of execution constraints for the repo
maintainability / architecture program.
Context: verify all facts against current maps, tasks, skills, and code before
reuse.
-->

# Execution Constraints Inventory

> Historical snapshot. Do not treat this note as current instruction or live
> architecture state; verify facts against current maps, tasks, and code before
> reuse.

## Status

- Discovery packet: D5.
- Date: 2026-04-16.
- Scope: operational constraints for later maintainability work packets.
- Nature: observed repo/process rules, not architecture decisions.

## Method

Checked:

- `AGENTS.md`
- `.agents/skills/subagent-delegation/SKILL.md`
- `.agents/skills/roadmap-maintenance/SKILL.md`
- current `ROADMAP.md` / `tasks/*.md` planning style and maintainability-program note

Key evidence commands used during this pass:

```bash
sed -n '60,170p' AGENTS.md
sed -n '1,240p' .agents/skills/subagent-delegation/SKILL.md
rg -n 'Worktrees|Planning and Verification|JSONL / LFS Safety|Git Conventions' AGENTS.md
```

## Constraint Inventory

### 1. Worktree And CWD Discipline

- Work only in the assigned cwd.
- Treat the tool default cwd as untrusted until confirmed.
- Use `/workspaces/msc-math` on `main` only when the task deliberately targets
  the root checkout or Jörn explicitly grants main-checkout work.
- Create a worktree when a task asks for isolated edits or when parallel
  sessions would edit overlapping tracked files.
- Base new worktrees on local `main` unless Jörn names a different base.
- Every subagent prompt must name the required cwd because `spawn_agent` cannot
  set cwd automatically.

Implication for later packets:

- Discovery notes can land in the current root checkout because the write scope
  is isolated under `.codex/reference/repo-maintainability/`.
- Broad implementation packets should default to dedicated worktrees once they
  edit shared library or experiment files.

### 2. JSONL / LFS Safety

- `.jsonl` files are generated artifacts and LFS-tracked.
- Do not edit `.jsonl` with patch-style line edits.
- If a smoke or warmup run needs data, write it to an untracked temp path and
  delete it after use.
- If a script touches tracked outputs only for compatibility, restore those
  paths before finishing.
- If a tracked `.jsonl` changes unexpectedly, stop and report the exact file
  and command.

Implication for later packets:

- Discovery packets may inspect `.jsonl` paths and hashes, but should not
  rewrite them.
- Any data-policy cleanup packet must distinguish `document mirror policy` from
  `regenerate tracked data`.

### 3. Planning And Verification Requirements

- For tasks with more than one concrete change or one verification step, keep a
  plan with objective, dependency, owner, and verification command or review
  check.
- Include a quality gate in the plan.
- Update the plan after meaningful results.
- Before asking Jörn to review a draft or conclusion, first run the checks that
  agents can run: buildability, internal consistency, source attribution,
  figure/text alignment, claim/data alignment, label/cross-reference
  resolution, missing tests, and scope drift.

Implication for later packets:

- Every maintainability packet in the future DAG should name one concrete
  verification command or review check, even for doc-only changes.
- The eventual DAG should include explicit review/verification nodes, not only
  implementation nodes.

### 4. Delegation Rules

- Subagents are for bounded first-pass labor after the active surface is clear.
- The top-level session keeps integration and correctness ownership.
- Parallel delegation is only for independent subtasks with disjoint write
  scopes or read-only questions.
- Delegate output is evidence, not a final fact, until locally checked.
- Delegation should not silently change task ownership, thesis direction, or
  merge readiness.

Implication for later packets:

- Discovery and note-writing can be parallelized by file.
- Shared design decisions, overlapping code edits, and public API choices
  should not be parallelized as independent worker edits.
- Later implementation packets should separate `design approved` from
  `execution delegated`.

### 5. Git / Merge Constraints

- Use local `main` as the base, not `origin/main`.
- Agents may commit without asking, but merge approval still requires Jörn.
- Before merging to `main`, run the `pre-merge` skill and get explicit approval
  from Jörn.
- Destructive operations such as force-push, branch deletion on `main`,
  `git reset --hard`, and checkout-based reverts require explicit approval.

Implication for later packets:

- The maintainability program may generate many PR-sized branches, but merge
  sequencing remains a separate reviewed step.
- The future DAG should track integration order independently from commit order.

### 6. Current Root-Checkout Risk

- Historical root checkout at the time of this inventory already had unrelated
  changes:
  - modified legacy `TASKS.md`
  - modified final-verification surfaces
  - durable `.codex/reference/repo-maintainability/` notes

Implication for later packets:

- Root-checkout discovery work should stay confined to the new
  `.codex/reference/repo-maintainability/` area plus the already-touched tracker
  file. Refresh the tracker path against current `ROADMAP.md` / `tasks/*.md`
  routing before reusing this historical inventory.
- Implementation packets that touch shared code should prefer worktrees to
  avoid coupling with this planning state.

## Reusable Packet Rules

Later packet writers should always include:

- required cwd
- exact write scope
- verification command or review check
- stop condition
- which decisions stay with Jörn or the top-level session
- whether the packet is root-checkout-safe or requires a worktree

## Open Questions

- Whether some pure-doc packets should still prefer worktrees for cleanliness,
  or whether root-checkout doc packets are acceptable during the planning
  phase.
- Whether the eventual implementation DAG should reserve dedicated reviewer
  packets after each major branch or only at bundle boundaries.

## Next Safe Resume Point

- Reuse this inventory when converting discovery results into execution
  packets.
- Do not duplicate these rules into each later note unless a packet needs a
  stricter local rule.
