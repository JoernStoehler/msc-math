---
name: meta-create-conventions
description: Workflow for designing and writing down project conventions. Load when you need to create a new convention, refine an existing one, or decide where a convention should live. Covers what makes a good convention, the refinement loop, and knowledge placement. For creating workflows instead, see meta-create-workflow. For the conceptual foundation, see meta-foundations.
---

# Creating Conventions

A workflow for designing, refining, and writing down conventions. Conventions are target state properties — they describe what the world should look like, not what to do.

## Reference documents

- `references/claudemd-format.md` — CLAUDE.md structure and style rules
- `references/skill-format.md` — SKILL.md frontmatter, body, and reference doc format
- `references/knowledge-placement.md` — decision tree for where knowledge goes
- `references/anthropic-skill-guide.md` — Anthropic's skill-building guide (good starting point, has gaps)

## Related skills

- `meta-foundations` — conceptual foundation (load first if you haven't)
- `meta-create-workflow` — for creating workflows instead of conventions

## What makes a good convention

A convention constrains the space of acceptable states. A good convention works across all three work phases (see `meta-foundations` § "The three work phases"): predictable in planning, actionable in execution, verifiable in review.

Additionally:
- **Justified** — the convention exists for a reason. State the reason (or point to it) so agents can handle edge cases. A convention without a reason gets dropped when it's inconvenient.
- **Scoped** — the convention says who it applies to and when. "Rust code in crates/" is scoped. "All code" is usually too broad.

## Workflow

### 1. Identify the need

Something is wrong or inconsistent across artifacts. You've noticed a pattern of mistakes, or Jorn has flagged an issue.

### 2. Check if a convention already exists

Search CLAUDE.md, relevant skills, and code comments. Don't create a duplicate — update the existing convention instead.

### 3. Draft the convention

State: what the target state is, who it applies to, and why. Keep it concrete — see `references/claudemd-format.md` for style rules.

### 4. Decide where it lives

Use the decision tree in `references/knowledge-placement.md`:
- Every agent needs it → CLAUDE.md
- Subset of agents, specific topic → skill
- Too detailed for skill body → reference doc
- Tied to a specific file → code comment or file header

### 5. Write it in the right format

- For CLAUDE.md: see `references/claudemd-format.md`
- For a skill: see `references/skill-format.md`
- For a code comment: just write it where agents will see it

### 6. Verify no duplication

Check that the convention is stated in exactly one place. Other locations should reference it, not restate it.

## Refinement loop

Conventions need refinement when agents don't follow them (see "Optimizing rules that don't work" in `meta-foundations`):

1. Is the convention observable? If agents can't tell whether they're following it, make it more concrete.
2. Is it at the right novelty level? Anti-intuitive conventions need structural enforcement (review subagents), not just instruction.
3. Is it in the right place? A convention that agents keep missing may need to move closer to where they work (e.g., from a skill to CLAUDE.md, or from CLAUDE.md to a file header).
