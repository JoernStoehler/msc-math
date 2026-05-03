---
name: harness
description: Use when editing or reviewing repo-local agent-facing harness material such as AGENTS.md, skills, agent prompts, subagent definitions, task-routing instructions, or Codex config/reference text.
---

# Harness

This skill is a fresh GPT-5.5 target surface. It is intentionally terse and
incomplete while the harness is being rebuilt.

Use it for repo-local text whose purpose is to affect future agent behavior.
Do not use it for ordinary domain prose or code comments unless the edit changes
agent routing, authority, validation, or handoff behavior.

TODO: decide which harness objectives deserve separate reference files.
TODO: decide which deleted old-harness commitments should be restored here.

## Navigation

Objective: agents should quickly find the right repo surface and the right
harness surface without rereading the whole repository or guessing from generic
repo priors.

Conventions:

- Use speaking names for skills and harness files. Prefer names that state the
  surface or objective directly.
- `AGENTS.md` is the always-loaded repo map. It should help all agents explore
  the repo more swiftly, while also carrying broadly useful context such as the
  project objective, quality objectives, and quick commands.
- Do not put detailed workflows into `AGENTS.md` unless every agent is likely
  to need them.
- Put surface-specific conventions in surface skills, not in hidden prose or
  generic references.
- Put migration-only discussion in draft files under
  `.agents/skills/harness-engineering-gpt-55/references/`.
- Use `/tmp/` for temporary prompt snippets, review packets, and reports that
  are not durable repo state.

Measure this objective by checking whether a fresh agent can answer:

- What kind of repo is this?
- Which source surface owns the thing I need?
- Which harness surface, if any, should I load?
- Which files are current policy, draft material, historical extraction, or raw
  git-history fallback?

TODO: add examples of good and bad skill names after rebuilding more skills.
TODO: decide whether map-maintenance belongs here or in a future objective
reference about repo explorability.

## Authority

- Current Jörn instructions override this skill.
- Draft files are review material, not policy, unless a current task explicitly
  asks to use them as the source for an edit.
- Deleted old harness files are raw material in git history, not active
  guidance.
- The extraction packet
  `.agents/skills/harness-engineering-gpt-55/references/old-harness-extract.draft.md`
  is a compact index of candidate content, not a replacement harness.

TODO: define how fresh target skills become promoted from draft to active.

## Editing

- Prefer objective, authority, measurement, and validation language over
  process choreography.
- Mark suggestions as suggestions when they are not binding.
- Mark uncertainty with `TODO` rather than pretending a list is complete.
- Do not repair references inside old surfaces that are being deleted.
- When deleting active harness files, first preserve a usable extraction unless
  Jörn explicitly says git history is enough.

Validation:

```bash
uv run --with pyyaml python /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/harness
git diff --check
```
