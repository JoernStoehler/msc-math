---
name: project-management-partner
description: Project management partner session. Review TASKS.md, assess what's blocked vs actionable, produce session prompts for parallel work streams. Use when Jörn asks about project status, priorities, what to work on next, or wants to spin up work sessions.
user-invocable: true
---

# Project Management Partner

Interactive session with Jörn to review project state, identify actionable work, and produce prompts for parallel sessions.

## What this session does

1. Read TASKS.md for current task state
2. Check recent git history for what's changed since last update
3. Read any relevant handoffs
4. Discuss with Jörn: what's blocked, what's actionable, what to prioritize
5. Produce copy-pasteable session prompts for the agreed work streams
6. Update TASKS.md with any new information from the discussion

## Session prompts

The main output is session prompts Jörn pastes into separate Claude Code sessions. Each prompt should:

- State the problem context and WHY the work matters (not step-by-step instructions)
- Reference the files to read for full context
- Name relevant skills to use (e.g., /experiment-design)
- State the deliverable: what file(s) should exist when the session is done
- If the work corresponds to a TASKS.md task, name it so the session can update its status
- Be self-contained — the target session has CLAUDE.md and rules but no conversation history
- Use fenced code blocks (```) so Jörn can copy them easily

Do NOT prescribe implementation steps. Write context and goals. The target session's agent figures out the how — it has the same skills, rules, and CLAUDE.md.

Every prompt must include "Work in a worktree" unless Jörn explicitly says otherwise. Sessions that edit main directly can collide with each other and with Jörn's work.

## What to avoid

- Do not suggest "decide the thesis story arc" or similar high-level scoping that Jörn already knows about
- Do not propose more streams than Jörn can review the output of
- Do not conflate independent tasks into one stream — each stream should have one clear deliverable
- Do not describe what you would do — do it (read files, check git, assess blockers)
- Do not give time estimates

