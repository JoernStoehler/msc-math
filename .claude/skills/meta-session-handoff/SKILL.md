---
name: meta-session-handoff
description: End-of-session checklist. Persists decisions to files, updates task tracking, writes handoff files. Optional review subagent checks structural integrity (broken references, TASKS.md contradictions) but NOT content correctness or conversation completeness.
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

If a future session will continue this work, write a handoff file to `handoffs/`. See the `meta-collaboration` skill for the handoff file format (Context, Scope, Out of scope, Key files, Prior findings, Success criteria, Dependencies).

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

Spawn a review subagent that reads ONLY the persistent artifacts (handoff files, TASKS.md, updated CLAUDE.md, changed files) — not the conversation.

**What this step checks:**
1. Do handoff file pointers reference files that actually exist?
2. Does TASKS.md contradict the actual file state (e.g., marks something done that isn't)?
3. Are there obvious gaps — a handoff file that says "see the updated config" but no config was changed?
4. Is the handoff file self-contained enough that an agent reading only it and the repo could start working?

**What this step does NOT check:**
- Correctness of content (mathematical claims, lore consistency, code logic)
- Completeness of what was discussed in conversation — the subagent can't see the conversation, so it can only check what's written, not what's missing
- Quality of decisions made during the session

This is a structural/mechanical check, not a content review. It catches broken references and obvious contradictions, not subtle omissions or wrong claims.

**When to skip:** Trivial sessions (single small edit, no continuation needed).

## Common mistakes

- Leaving important context only in conversation (the next agent can't see it)
- Not writing a handoff file when work will continue
- Assuming the next agent knows what you know
- Handoff files that summarize instead of pointing to files
- Uncommitted work
