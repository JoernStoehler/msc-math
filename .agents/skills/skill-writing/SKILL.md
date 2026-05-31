---
name: skill-writing
description: Use when Codex drafts, updates, reviews, or proposes repo-local skill material such as `.agents/skills/*/SKILL.md`, skill references, or skill metadata. Use together with the built-in skill-creator for mechanics. Focuses on the observed failure or repeated workflow motivating the skill, trigger boundaries, source truth, existing repo surfaces, likely side effects, Jörn review gates, and whether a skill is the right live surface. Do not use merely because another skill is loaded or mentioned.
---

# Skill Writing

Use `$skill-creator` for skill mechanics: frontmatter, trigger description,
resource layout, validation, and optional metadata. This skill is for the
repo-specific question: what should the skill say so future agents do useful
work without new failure modes?

Do not start by drafting the `SKILL.md`. First write a short scratch preflight
in `/tmp`.

The preflight should answer:

- what observed failure or repeated workflow motivates this skill
- what future behavior the skill should make more likely
- what tasks should trigger the skill
- what nearby tasks should not trigger the skill
- what existing repo files already own nearby guidance or source truth
- what evidence supports the proposed skill content
- what bad extra work the skill could accidentally encourage
- what concrete task or artifact will be used to review the draft

If those answers are not available from the repo or the current chat, ask Jörn
before drafting. Do not fill the gap with generic skill advice.

## Drafting

Keep the live skill body short and operational. A useful repo-local skill
usually transfers one of these:

- a decision rule future agents repeatedly need
- a source-truth map for a recurring task
- a review lens for a known failure mode
- a workflow that is easy to forget or easy to do in the wrong order

Prefer plain bullets over balanced sections. Do not optimize for visual
symmetry. Do not use a template unless the task itself needs a fixed shape.

Point to existing task, research, map, or reference files instead of copying
their content into the skill. If detailed background is useful but not needed
on every trigger, put it in `references/` and say when to read it.

## Review Before Showing Jörn

Review the draft against the scratch preflight, not against whether it sounds
like a plausible skill.

Check:

- the description triggers on the intended future tasks and avoids nearby
  false positives
- the body names the source truth or says when source truth is missing
- the skill does not duplicate a task file, map file, research note, or
  another skill
- the skill does not invite agents to do extra harness, planning, or review
  work during unrelated tasks
- the skill preserves Jörn review gates instead of replacing them with agent
  judgment
- the guidance would have helped on the motivating failure or workflow
- the guidance is concrete enough that a future agent can act on it without
  inventing a new interpretation layer

Run the skill validator before handing back:

```bash
uv run --with pyyaml --script /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/<skill-name>
```

Do not commit repo-local skill changes without Jörn's explicit approval.

## Long Notes

`references/harness-engineering-notes.md` contains the old long-form
`$harness-engineering` material. Read it only when the preflight or review is
still stuck after checking the concrete motivating failure, existing repo
surfaces, and `$skill-creator`.
