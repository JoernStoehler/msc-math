---
name: meta-folder-layout
description: Conventions for project directory structure and where to place different kinds of knowledge. Load when deciding whether something belongs in CLAUDE.md, a skill, a reference doc, code comments, TASKS.md, or MEMORY.md. Provides a decision tree for knowledge placement. Does NOT cover how to write CLAUDE.md (see meta-claudemd) or how to write skills (see meta-skills).
---

# Project Directory Structure and Knowledge Placement

Where to put knowledge so agents find it when they need it. For foundational analysis of *why* placement matters, load the `meta-documentation` skill.

## Related skills

- `meta-documentation` — foundational analysis: knowledge taxonomy, instruction focus, why placement matters
- `meta-claudemd` — how to write CLAUDE.md content once you've decided something belongs there
- `meta-skills` — how to write SKILL.md content once you've decided something belongs in a skill

## Decision tree: where does this knowledge go?

**Is it tied to a specific file or folder?**
-> Code comment, file header, or doc comment. Agents naturally look at these from training.

**Does every agent need it?**
-> CLAUDE.md. But keep it lean — every agent pays the context-window cost.

**Does a subset of agents need it for a specific topic?**
-> SKILL.md. Name + description are always visible; body is loaded on demand.

**Is it too detailed or large for a skill body?**
-> Reference doc (`references/` inside the skill). Agents discover these via the skill body.

**Is it project management state (tasks, experiments, deferred work)?**
-> TASKS.md. Grows stale — agents don't habitually update it. Other agents only need to know it exists.

**Is it a session learning or communication behavior note?**
-> MEMORY.md. Occasionally clean and migrate stable entries to CLAUDE.md or standard locations.

**Is it ephemeral context for a subagent?**
-> Subagent prompt. Don't persist it.

## Location tiers — full rationale

**1. CLAUDE.md** — always in context. Cannot be skipped, but can be ignored despite being read (see "Instruction focus" in `meta-documentation`). Cost: every agent pays the context-window cost. Put knowledge here when useful for a majority of agents, or when it has been forgotten too often in other locations.

**2. SKILL.md files** — name + description always visible, body loaded on demand. Progressive disclosure for minority use cases. Limitation: agents sometimes run ahead without loading skills. Reminding agents to read skills is important. Subagents can be force-told to read a skill in their prompt or via the `skills:` field in agent definitions.

**3. Reference docs** (`references/` inside skills) — loaded by agents within a skill context. Not visible in system prompt — agents discover them via the skill body. Good for: detailed procedures, detection rules, checklists, examples, how-to guides. Too large or specific for the skill body itself.

**4. Standard locations** — code comments, doc comments, file headers, README.md, config files. Agents naturally look at these from training. Prefer these over SKILL.md when knowledge is tied to a specific file or folder. Anti-pattern: agents sometimes dump too much into README.md, treating them as catch-all dumps.

**5. TASKS.md** — project management knowledge (features, experiment ideas, deferred tasks, external constraints). Grows stale — agents don't habitually update it. Other agents only need to know it exists.

**6. MEMORY.md** — catch-all for session learnings, communication behavior. Occasionally clean and migrate stable entries to CLAUDE.md or standard locations.

## When to use each meta-layer component

| Component | Purpose | Example |
|-----------|---------|---------|
| CLAUDE.md | Universal agent instructions | "Run `cargo test` before committing" |
| Skill SKILL.md | Topic-specific conventions | Review methodology, TeX formatting rules |
| Reference doc | Detailed procedures, checklists | Detection rules for a review concern |
| Agent definition | Subagent capabilities (tools, model, skills) | Review agent with specific skill set |
| Code comment | File-scoped knowledge | "This function assumes sorted input" |
| TASKS.md | Project management state | "TODO: refactor X after Y ships" |
| MEMORY.md | Session learnings, communication | "Jorn prefers X over Y" |
