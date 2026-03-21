---
name: session-handoff
description: Use this checklist at the end of a working session before context is lost. Captures session state into persistent artifacts and optionally spawns a review subagent to verify completeness.
user-invocable: true
---

# Session Handoff

Use at the end of a working session before context is lost. Runs in main context (needs session history).

## When to use

- Before ending a session ("wrap up", "handoff", "done for now")
- Before context compaction warnings
- After completing a major piece of work

## Step 1: Persist decisions to files

Anything decided in conversation but not yet written to files will be lost.

- Decisions, corrections, or clarifications from Jörn → update affected files
- Brainstormed content not yet saved → add to appropriate location
- Conventions established this session → update CLAUDE.md or relevant skill
- Discovered tasks → add to TASKS.md

## Step 2: Update task tracking

- Update TASKS.md (or PROGRESS.md) with current state
- Mark completed items as done
- Add new items discovered during session
- Remove or correct outdated entries

## Step 3: Write handoff file (if work continues)

If a future session will continue this work, write a handoff file to `handoffs/`. See the `collaboration` skill for the handoff file format (Context, Scope, Out of scope, Key files, Prior findings, Success criteria, Dependencies).

Key principles:
- Pointers over summaries — point to files, don't describe them
- One task per handoff file
- Scope boundaries prevent drift — name what's out of scope
- Include findings the next agent would otherwise re-derive

## Step 4: Commit

- Stage changes (prefer specific files over `git add -A`)
- Write clear commit message summarizing session work
- Push if working on a branch

## Step 5: Subagent review (recommended for substantial sessions)

Spawn a review subagent that reads ONLY the persistent artifacts (handoff files, TASKS.md, updated CLAUDE.md, changed files) — not the conversation. The subagent answers:

1. Could a new agent reconstruct what to do next from files alone?
2. Are there decisions or context mentioned in the handoff that aren't backed by file changes?
3. Is anything in TASKS.md contradicted by the actual file state?
4. Are handoff file pointers valid (do the referenced files exist)?

The subagent simulates what the next session will experience. If it can't reconstruct the state, the handoff has gaps — fix them.

**When to skip:** Trivial sessions (single small edit, no continuation needed). When in doubt, run it — the cost is low and it catches gaps the main agent is blind to.

## Common mistakes

- Leaving important context only in conversation (the next agent can't see it)
- Not writing a handoff file when work will continue
- Assuming the next agent knows what you know
- Handoff files that summarize instead of pointing to files
- Uncommitted work
