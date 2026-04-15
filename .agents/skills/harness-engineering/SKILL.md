---
name: harness-engineering
description: "Guidance for Jörn-approved edits to this repo's agent harness: `AGENTS.md`, `.agents/skills/**`, `.codex/agents/*.toml`, Codex config notes, onboarding wording, and subagent/review prompts. Use only when Jörn explicitly asks to revise onboarding, convention routing, skills, subagents, prompt behavior, or agent workflow documentation."
---

# Harness Engineering

## Goal

Edit the agent harness after Jörn has asked for harness edits. If a normal task session exposes a harness issue, propose the change at the end of the session or use `$post-mortem` when Jörn invokes it; do not edit harness files opportunistically.

The harness is:
- `AGENTS.md`: always-loaded project map and global invariants.
- `.agents/skills/**`: triggerable convention and workflow bodies.
- `.codex/agents/*.toml`: narrow subagent role prompts.
- `.codex/config.toml` and `.codex/reference/**`: Codex CLI configuration and reference material.
- `.devcontainer/**`: runtime-environment setup for local devcontainer and Codex web sessions.

## Design Rules

- Keep `AGENTS.md` short and task-facing. It is always loaded, so every sentence competes with the task context.
- Put detailed conventions, editing rationale, and workflow procedure in skills.
- Put "when to use this" trigger text in the skill description, not in `AGENTS.md` and not only in the skill body.
- Do not maintain a skill inventory or routing table in `AGENTS.md`; skill names and descriptions are already visible through the skill system, and duplicated triggers drift.
- Do not rely on nested settings or nested `AGENTS.md` files for required project behavior. Root-launched sessions may not load them.
- When a harness decision depends on Codex mechanics, compare Jörn's proposed model with current official OpenAI docs. If they differ, state the difference explicitly and explain whether the docs describe a hard product constraint, a recommendation, or a default that local experience may override.
- Use one subagent role with loaded checklists when the role is stable and only the review surface changes.
- Split a subagent only when the role, permissions, or output contract differs.
- Do not edit harness files during unrelated task work. Harness edits require a direct Jörn request in the current turn.
- Use `$post-mortem` for advisory reflection requested by Jörn. It suggests changes but does not execute them.

## Editing Workflow

1. Confirm Jörn asked for harness edits, not only reflection or normal task work.
2. Identify whether the change affects always-loaded context, skill routing, skill body procedure, subagent role, or runtime setup.
3. Remove obsolete text instead of preserving it as another path.
4. If editing a skill, follow `$skill-creator`: frontmatter has only `name` and `description`; the description carries trigger conditions.
5. If moving content out of `AGENTS.md`, add it to the skill whose description should trigger for that work.
6. Check for stale path assumptions with `rg`, especially `crates/`, `.agents/rules`, `math.tex`, `logbook.md`, and old review-agent names.
7. For Codex product behavior claims, cite the official OpenAI source or say the claim is based on local observed behavior.
8. Run validation:

```bash
uv run --with pyyaml python /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/<skill-name>
git diff --check
```

## Post-Session Reflection

`$post-mortem` is explicit-only and advisory. It suggests future harness changes but does not execute them.

Use this skill, not `$post-mortem`, when Jörn asks to actually edit the harness.
