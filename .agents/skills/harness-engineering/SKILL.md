---
name: harness-engineering
description: "Guidance for Jörn-approved edits to this repo's agent harness: `AGENTS.md`, `.agents/skills/**`, `.codex/agents/*.toml`, Codex config notes, onboarding wording, and subagent/review prompts. Use only when Jörn explicitly asks to revise onboarding, convention routing, skills, subagents, prompt behavior, or agent workflow documentation."
---

# Harness Engineering

## Goal

Edit the agent harness after Jörn has asked for harness edits. If a normal task session exposes a harness issue, propose the change at the end of the session or use `$post-mortem` when Jörn invokes it; do not edit harness files opportunistically.

When the harness change is really about what "bad complexity" means for
agent-heavy projects, read
[`references/agent-project-delta.md`](references/agent-project-delta.md).
That note is about the delta from ordinary human-team heuristics; do not load
it for routine harness edits that do not touch simplification, architecture, or
agent-facing readability.

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
- Name top-level session-scope skills `*-focus`, not `role-*` or `*-scope`. "Focus" says what the session keeps active and reasons about directly; "role" is vague, and "scope" is overloaded with file/write scopes and sounds like a restriction. Prefer concrete focus names such as `project-management-focus` over abstract labels such as `coordination-focus`.
- For a focus skill, state the cognitive labor the top-level session owns, the artifact it keeps current, what may move to subagents, and which decisions stay with Jörn.
- Keep focus skills, workflow skills, and subagent roles separate. Focus skills define what the main session keeps active; workflow skills define reusable procedures inside a focus; subagent roles define separate-context output contracts.
- Write skill bodies positive-first: operating model, action checklist or template, Jörn gates, and stop conditions. Add negative examples only when they prevent an observed or high-cost failure mode.
- Review skill drafts for actionability. Ask reviewers which sentences would change behavior on a real task, which sentences are dense, and which examples or guardrails are redundant.
- When a harness decision depends on Codex mechanics, compare Jörn's proposed model with current official OpenAI docs. If they differ, state the difference explicitly and explain whether the docs describe a hard product constraint, a recommendation, or a default that local experience may override.
- Use one subagent role with loaded checklists when the role is stable and only the review surface changes.
- Split a subagent only when the role, permissions, or output contract differs.
- Do not edit harness files during unrelated task work. Harness edits require a direct Jörn request in the current turn.
- Use `$post-mortem` for advisory reflection requested by Jörn. It suggests changes but does not execute them.
- When removing stale top-level artifacts, prefer deletion or relocation over adding warnings around them.
- Before deleting a tracked file or directory, check that git has captured the current state. If the path is untracked or has uncommitted edits, stop and either commit the state first or ask Jörn. Rollback should be possible through git.

## Editing Workflow

1. Confirm Jörn asked for harness edits, not only reflection or normal task work.
2. Identify whether the change affects always-loaded context, skill routing, skill body procedure, subagent role, or runtime setup.
3. For a new or renamed focus skill, first state the labor and artifact it owns; choose the name after that sentence is clear.
4. Keep a short decision ledger in the conversation before editing: decision, rejected alternative, and affected files.
5. Remove obsolete text instead of preserving it as another path.
6. If editing a skill, follow `$skill-creator`: frontmatter has only `name` and `description`; the description carries trigger conditions.
7. If moving content out of `AGENTS.md`, add it to the skill whose description should trigger for that work.
8. Check for stale path assumptions with `rg`, especially `crates/`, `.agents/rules`, `math.tex`, `logbook.md`, and old review-agent names.
9. For Codex product behavior claims, cite the official OpenAI source or say the claim is based on local observed behavior.
10. Run validation:

```bash
uv run --with pyyaml python /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/<skill-name>
git diff --check
```

## Post-Session Reflection

`$post-mortem` is explicit-only and advisory. It suggests future harness changes but does not execute them.

Use this skill, not `$post-mortem`, when Jörn asks to actually edit the harness.
