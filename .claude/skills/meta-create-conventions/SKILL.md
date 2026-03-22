---
name: meta-create-conventions
description: Workflow for designing and writing down project conventions and procedural knowledge. Load when you need to create a new convention, refine an existing one, or decide where it should live. Covers what makes a good convention, the refinement loop, and knowledge placement. For creating subagent workflows specifically, see meta-create-workflow. For the conceptual foundation, see meta-foundations.
---

# Creating Conventions

A workflow for designing, refining, and writing down conventions. Conventions describe properties that artifacts should have. Both positive ("artifact must have X") and negative ("artifact must not have Y") framings are fine.

## Reference documents

- `references/claudemd-format.md` — CLAUDE.md structure and style rules
- `references/skill-format.md` — SKILL.md frontmatter, body, and reference doc format
- `references/knowledge-placement.md` — decision tree for where knowledge goes
- `references/anthropic-skill-guide.md` — Anthropic's skill-building guide (good starting point, has gaps)

## Related skills

- `meta-foundations` — conceptual foundation (load first if you haven't)
- `meta-create-workflow` — for creating workflows instead of conventions

## What makes a good convention

A convention constrains the space of acceptable artifact states. Four properties to check:

- **Observable** — can the agent tell whether the convention is met by inspecting the artifact? If not, the convention can't be reviewed.
- **Actionable** — does the agent have actions available, and know what those actions are, that cause the desired property? Note: observable but not actionable is possible ("tests must cover all edge cases" — you can verify a complete set but can't generate one from scratch). Actionable but not observable is also possible ("write readable code" — you know how but can't measure it). Both negative and positive conventions can be actionable.
- **Scoped** — the convention says who it applies to and when. "Rust code in crates/" is scoped. "All code" is usually too broad.
- **Justified** — the convention exists for a reason. State the reason (or point to it) so agents can handle edge cases. A convention without a reason gets dropped when it's inconvenient. Negative conventions are often justified by anecdote (a past failure).
- **Known** — ideally, the convention is familiar to agents from training, so that they don't incur as much attention or execution overhead from it. Novel conventions can be explained to agents, but they slow down and distract from the main task. See `meta-foundations:##Optimizing rules that don't work` for strategies to mitigate this.

## Workflow

### 1. Identify the need

Something is wrong or inconsistent across artifacts. You've noticed a pattern of mistakes, or Jorn has flagged an issue.

### 2. Check if a convention already exists

Search CLAUDE.md, relevant skills, and code comments. Don't create a duplicate — update the existing convention instead.

### 3. Draft the convention

State: what the target state is, who it applies to, and why. Keep it concrete — see `references/claudemd-format.md` for style rules.

### 4. Decide where it lives

Use the decision tree in `references/knowledge-placement.md`:
- Tied to a specific file → code comment or file header
- Subset of agents, specific topic → skill
- Too detailed for skill body → reference doc
- Every agent needs it → CLAUDE.md

### 5. Write it in the right format

- For CLAUDE.md: see `references/claudemd-format.md`
- For a skill: see `references/skill-format.md`
- For a code comment: just write it where agents will see it
- Always focus on clarity and unambiguity. The cost of verbosity is tiny, and the cost of confusion or cognitive overhead is big.

### 6. Test it

- Ask a subagent to evaluate whether the meta-conventions for conventions ("what makes a good convention") are met.
- Use concrete scenarios to test the subagent's understanding, and whether the convention is really actionable and observable. This can include a full, throwaway, low-cost mock task in a worktree.

## Refinement loop

Conventions need refinement when agents don't follow them (see "Optimizing rules that don't work" in `meta-foundations`):

1. Is the convention observable? If agents can't tell whether they're following it, make it more concrete.
2. Is it at the right novelty level? Anti-intuitive conventions need structural enforcement (review subagents), not just instruction.
3. Is it in the right place? A convention that agents keep missing may need to move closer to where they work (e.g., from a skill to CLAUDE.md, or from CLAUDE.md to a file header).
