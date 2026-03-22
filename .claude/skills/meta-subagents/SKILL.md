---
name: meta-subagents
description: Conventions for creating and editing agent definitions (.claude/agents/*.md). Load when writing a new agent or modifying an existing one. Covers the capabilities-not-behavior principle, description-as-contract, and minimal agent definitions. Does NOT cover skill writing (see meta-skills) or CLAUDE.md style (see meta-claudemd).
---

# Writing Agent Definitions

How to write `.claude/agents/*.md` files so parent agents plan around them correctly and subagents behave as expected.

## Reference documents

- `references/agent-yaml-reference.md` — YAML frontmatter fields, built-in subagent types, checklist

## Related skills

- `meta-skills` — writing SKILL.md files (skills are where behavior lives)
- `meta-documentation` — foundational analysis: failure modes (especially "delegation failures: loud vs silent")

## Capabilities, not behavior

Agent definitions define **capabilities** (tools, model, skills), not behavior. Behavior comes from the skills preloaded via the `skills:` field.

## Agent definition contents

- YAML frontmatter: `name`, `description`, `model`, `tools`, `skills`
- A brief task description (what the agent does)
- Pointers to where the agent should look for its methodology

## Description-as-contract

The `description` field determines how parent agents plan around this agent. The parent reads the description, decides to spawn the agent, and then plans as if the agent will do what the description says.

If the description is vague or overpromises, the parent's plan has a gap equal to the delta between what it expected and what the agent actually does — and this gap is hard to catch because verifying a reasoning subagent's output is nearly as hard as doing the task yourself.

Descriptions must clearly state:
- **What the agent does** — its guaranteed outputs
- **What the agent does not guarantee** — limits the parent should not rely on

This is the same principle as skill descriptions (see `meta-skills`).

## Keep definitions minimal

If you find yourself writing inline checklists or detailed instructions in the agent definition, that content belongs in a skill or reference doc instead. The agent definition points to skills; the skills contain the methodology.

## Preloading skills

Use the `skills:` field to force-load skills into subagents. This is more reliable than hoping the subagent will discover and load the right skills on its own. Agents sometimes run ahead without loading skills — preloading removes that failure mode.
