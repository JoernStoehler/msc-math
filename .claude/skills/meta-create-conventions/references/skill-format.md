# SKILL.md Format

How to write skills that agents discover, load, and follow correctly.

## Additional reference documents

- `anthropic-skill-guide.md` (in this directory) — Anthropic's skill-building guide
- `skill-yaml-reference.md` (in this directory) — YAML frontmatter fields, file structure, checklist

## The progressive disclosure model

Skills use a three-level system that controls context window cost:

1. **Frontmatter** (always loaded) — `name` and `description` appear in every agent's system prompt. This is the only part agents see without explicitly loading the skill. Cost: every agent pays this context cost.
2. **SKILL.md body** (loaded on demand) — the full skill content. Loaded when an agent decides the skill is relevant. Organize for the agent who loaded the skill — they already know they need it.
3. **Reference docs** (`references/` inside the skill directory) — loaded by agents within a skill context. Not visible in the system prompt — agents discover them via the skill body. Good for: detailed procedures, detection rules, checklists, examples, how-to guides. Too large or specific for the skill body itself.

The skill body should mention which reference docs exist so agents know to look.

## Frontmatter conventions

Required fields: `name` and `description`.

The `description` field is the most important part. It determines whether agents load the skill — a parent agent reads the description to decide whether to call a skill/subagent, then plans as if the skill does what the description says.

### Description-as-contract

If the description overpromises (e.g., "verify completeness" when it only checks file references), the parent's plan silently has a gap. Descriptions must state both:
- **What the skill does** — capabilities it provides
- **What it does not check** — limits of its scope

This applies equally to skill descriptions and agent descriptions.

### Description format

- Under 1024 characters
- Include WHAT the skill does and WHEN to load it (trigger conditions)
- Be specific: "Load when editing CLAUDE.md" not "Load for documentation tasks"
- Name related skills it does NOT cover, so agents don't load the wrong one

## Body conventions

- Organize for the agent who loaded the skill — they already know they need it
- Mention which reference docs exist in `references/`
- Keep SKILL.md under 5,000 words; move detailed content to `references/`

## Skill folder structure

```
skill-name/
  SKILL.md              # Required — main skill file
  references/           # Optional — detailed docs too large for skill body
    some-reference.md
```

- Folder names: kebab-case (`meta-skills`, not `meta_skills` or `MetaSkills`)
- No README.md inside skill folders
- All documentation goes in SKILL.md or `references/`
