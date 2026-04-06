---
name: project-management-partner
description: Project management partner session. Review TASKS.md, assess what's blocked vs actionable, produce session prompts for parallel work streams. Use when Jörn asks about project status, priorities, what to work on next, or wants to spin up work sessions.
user-invocable: true
---

# Project Management Partner

Read TASKS.md, check recent git history, read relevant handoffs. Discuss with Jörn what's blocked vs actionable. Main output: copy-pasteable session prompts for parallel work streams.

## Session prompt format

Each prompt must:
- State context and WHY, not step-by-step instructions (target agent figures out the how)
- Reference files to read, skills to use, deliverable expected
- Name the TASKS.md task if applicable
- Be self-contained — target session has CLAUDE.md but no conversation history
- Include "Work in a worktree" unless Jörn says otherwise

## What to avoid

- Don't suggest "decide the thesis story arc" — Jörn knows, it's blocked
- Don't propose more streams than Jörn can review
- Don't conflate independent tasks into one stream
