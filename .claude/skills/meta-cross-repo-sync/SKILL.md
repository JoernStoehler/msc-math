---
name: meta-cross-repo-sync
description: How to keep shared infrastructure synchronized across repos. Load when editing any meta-* skill, behavior norms, or hook scripts. All meta-* skills are byte-for-byte identical across repos. Does NOT cover project-specific content (CLAUDE.md sections, domain skills, agents).
---

# Cross-Repo Sync

Three repos share identical meta-layer infrastructure: msc-math, dnd-claude-code, xrisk-pause-game.

## What is shared (must be identical across repos)

- All `meta-*` skills (meta-documentation, meta-claudemd, meta-skills, meta-subagents, meta-hooks, meta-folder-layout, meta-collaboration, meta-feedback-processing, meta-session-handoff, meta-post-mortem, meta-cross-repo-sync)
- Behavior norms section in CLAUDE.md (### Agent Behavior Norms)
- Hook scripts (`.claude/hooks/`)

## What is NOT shared (project-specific)

- CLAUDE.md sections other than behavior norms
- Project-specific skills (e.g., dnd's hook-conventions, msc-math's rust-conventions)
- Agent definitions (`.claude/agents/`)
- TASKS.md, PROGRESS.md
- Handoff files

## Sync workflow

When editing any shared file:

1. Make the change in one repo
2. Copy the file verbatim to the other two repos
3. Commit in all three repos

Word-for-word identical content enables mechanical diffing to detect drift.

## Sync strategy

Copy files manually. No shared repo — not worth the indirection for 3 projects. The cost is manual copying; the benefit is zero coupling between repos.
