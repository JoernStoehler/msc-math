# Task: Continue procedural layer rewrite

## Context

Session on 2026-03-24 simplified the meta layer, created the agent-design skill, and began rewriting all procedural files from scratch. The approach: move old content to `/tmp/old-*`, write fresh files based on what agents actually need, not anchored to old content.

## What's done

1. **agent-design skill** — complete, Jörn-reviewed. `.claude/skills/agent-design/SKILL.md` with expert model, experience, 7-step workflow, references (system prompt repo, skills guide).
2. **CLAUDE.md** — rewritten from scratch with project context + layout only. Jörn edited it to fix errors. Remaining sections (core rule, Jörn-gating, session workflow, environment, quick commands) need to be migrated from `/tmp/old-CLAUDE.md` — but only after Jörn confirms each one.
3. **TASKS.md** — deadline fixed (mid-April), stale entries removed, self-documenting conventions header added.
4. **Post-mortem skill** — updated to route feedback to `feedback/<skill-name>.md`.

## What's NOT done

All domain skills need fresh rewrites. Old versions in `/tmp/old-skills/`. The approach from the agent-design workflow:
- Think about what labor agents do → what they need beyond training → write the minimal skill
- Don't anchor to old content

**Phase 1 (math.tex + Rust):** `math-tex`, `rust-conventions`, `rust-tests`
**Phase 2 (thesis):** `tex-content`, `tex-format`, `tex-build`
**Phase 3 (experiments):** `experiment-conventions`, `python-conventions`
**Phase 4 (coordination):** `collaboration`, `session-handoff`, `review`, `git-conventions`, `communication` (new — extracted from old CLAUDE.md)
**Phase 5 (specialized):** `data-pipeline`, `slurm`
**Agents:** 6 agent .md files also in `/tmp/old-agents/`, not yet touched.

Also not done: CLAUDE.md needs remaining sections added (core rule, Jörn-gating, session workflow, environment, quick commands). Do this incrementally as skills are written.

## Key decisions from this session

- **Communication with Jörn** → extract to a skill, not CLAUDE.md (subagents don't need it)
- **Skill list** → don't put in CLAUDE.md (goes stale). Agents discover skills from descriptions.
- **Thesis topic** → don't put in CLAUDE.md. Progressively disclosed via .tex files.
- **TASKS.md conventions** → colocated in TASKS.md itself, not a separate skill
- **HTML comments** → don't rely on them in CLAUDE.md or SKILL.md (unknown whether visible)
- **llms.txt** → referenced only from agent-design skill workflow step 4, not from CLAUDE.md
- **Feedback collection** → `feedback/<skill-name>.md`, not `.claude/feedback/`

## Key files

- `/tmp/old-CLAUDE.md` — old CLAUDE.md for reference
- `/tmp/old-skills/` — old skill content for reference (do NOT anchor to these)
- `/tmp/old-agents/` — old agent content for reference
- `.claude/skills/agent-design/SKILL.md` — the workflow for writing procedural files
- `TASKS.md` — master task list
- `~/.claude/projects/-workspaces-msc-math/memory/agent-cognition.md` — Jörn's expert model summary

## Caution

Context was degraded toward end of session — I introduced errors in CLAUDE.md that Jörn had to fix. Fresh context will help.
