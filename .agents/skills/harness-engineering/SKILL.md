---
name: harness-engineering
description: "Use when editing or reviewing repo-local material whose purpose is to affect future agent behavior: `AGENTS.md`, `.agents/skills/**`, `.codex/agents/*.toml`, Codex config/reference notes, onboarding wording, subagent/review prompts, task packets, and agent-facing roadmap/task instructions."
---

# Harness Engineering

This active skill is intentionally erased during the GPT-5.5 harness migration.
Do not treat the pre-migration `$harness-engineering` body as current policy.

For current migration work, load `$harness-engineering-gpt-55` and follow the
user's current instructions. The extraction from the old skill lives at
`../harness-engineering-gpt-55/references/harness-engineering-extract.draft.md`
as review material, not policy.

Until a fresh target skill replaces this stub:

- keep harness edits tied to the current Jörn request;
- treat old harness files as legacy input, not settled policy;
- validate touched skills with the skill validation command;
- run `git diff --check` before committing.
