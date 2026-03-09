# CLAUDE.md

## Knowledge Placement

**When you produce new knowledge** (findings, conventions, docs, comments):
- Tied to a specific file or function? → code comment, doc comment, or file header. This is the natural location agents look at when working with that code.
- Applies to most agents? → CLAUDE.md.
- Applies to a minority of agents? → `.claude/skills/*/SKILL.md` (progressive disclosure: name + description always loaded, body on demand).
- Project management (tasks, ideas, deferred work, constraints)? → `TASKS.md` (root). Grows stale; that's fine.
- Session learning or cross-session state? → `MEMORY.md`. Migrate stable entries to CLAUDE.md or standard locations.
- Don't dump unrelated knowledge into README.md files. Each README covers its own directory's purpose.

**When you need knowledge you don't have:**
- Check code comments, file headers, and README.md in the relevant directory first.
- Check CLAUDE.md (you already have it in context — search for keywords).
- Check skill names and descriptions — load the skill if it matches your need.
- Check `TASKS.md` for project-level context (what's planned, what's deferred, why).
- Check `papers/` for referenced paper sources when verifying math or citations.
- Check `.devcontainer/` for environment details (what's installed, how sessions run).

**When editing CLAUDE.md, SKILL.md, or agent prompt files:**
- Load the `writing-conventions` skill first. It contains the rationale, style rules, and cross-reference tag system.
- Editing CLAUDE.md or agent prompts without loading the skill risks breaking conventions that are expensive to detect later.

**Agent prompt architecture:** Subagent definitions in `.claude/agents/*.md` 1:1 copy relevant CLAUDE.md sections into their prompt body. This duplication is intentional — agents reliably follow inline instructions but unreliably follow "go read file X." Cross-reference tags (`<copied-to>` in CLAUDE.md, `<copied-from>` in agent files) track which copies need updating. Details in the `writing-conventions` skill.
