# Extract From Old `$harness-engineering`

<!--
Working extraction from `.agents/skills/harness-engineering/SKILL.md`.
This is not active policy. Use it as a review surface for deciding what, if
anything, should survive into a fresh GPT-5.5 harness-engineering skill.
-->

## Candidate Scope

- Use a harness-engineering surface only for repo-local text whose purpose is
  to change future agent behavior.
- Do not route ordinary Rust comments, thesis prose, proof drafts, experiment
  writeups, datasets, or figures through harness engineering unless the edit
  changes instruction, routing, authority, review, handoff, or steering text.
- Treat prompt and harness text as instrumental. It has value only insofar as
  it helps future agents do thesis-project work better.

## Candidate Quality Criteria

- State the intended future behavior or task.
- State how success will be recognized, when that is not obvious from the task.
- Make success criteria concrete enough for GPT-5.5 to specialize locally
  without redefining the objective.
- Leave task-local implementation choices to the future agent when many paths
  can satisfy the objective.
- Avoid prescribing a plan before the objective, authority, success signal, and
  constraints require one.
- Mark path suggestions as suggestions when they are not binding.
- Make binding constraints part of the objective or success measurement, not
  incidental preference text.
- Record historical context as history. Do not promote it to a current
  constraint unless the constraint is now known to be necessary for this repo.
- Separate stable context from task-local details when mixing them would blur
  authority.

## Candidate Ambiguity Checks

- If a wrong reading would change future behavior, rewrite until the intended
  reading is concrete.
- Treat vague words as search triggers, not banned tokens.
- If ambiguity remains expensive and delegation is available, a fresh subagent
  can be asked what is hard to understand or act on; treat the result as
  evidence, not authority.
- For complex delegated or prompt-driven tasks, give an escape hatch for
  rejecting an unclear or unproductive objective.

## Candidate Evaluation Pattern

- When future-agent output is available, compare the output against the intended
  behavior, not only against the prompt text.
- Check evidence of success before trusting a completion summary.
- If a prompt or harness edit fails, decide whether the right response is:
  discard the attempt, repair the prompt, repair the output, escalate to Jörn,
  or accept residual risk.
- Generalize a lesson only after checking that it is not just task-local
  history.

## Candidate Authority And Review

- Durable harness material includes `AGENTS.md`, `.agents/skills/**`,
  `.codex/agents/*.toml`, `.codex/config.toml`, reusable reference notes, and
  other long-lived agent-facing steering text.
- Durable harness text has higher cost than task-local prompt text because it
  affects repeated sessions, adds maintenance burden, and can conflict with
  nearby authority surfaces.
- Durable harness edits need a current Jörn request and Jörn review before they
  are treated as settled policy.
- Task-local prompts and temporary packets can be created freely when failures
  stay observable, bounded, and reversible.
- When replacing the purpose of an active durable surface, create a fresh
  replacement or move the old file out of the active surface instead of mutating
  the old file into a different-purpose instruction file.
- Before deleting, moving, or replacing an active path, inspect its git status.
  Stop if it is dirty, untracked, or ownership is unclear.

## Candidate Dependency Rules

- Use the skill-creation validation path when creating, deleting, renaming, or
  behaviorally changing a skill.
- Use current official OpenAI documentation before durable edits that depend on
  current OpenAI or Codex product behavior.
- Do not copy external provider guidance into repo prompt material. Translate
  only the repo-owned consequence, if one is chosen.

## Candidate Validation

- Run `git diff --check` for touched harness text.
- For touched skill folders, run:

```bash
uv run --with pyyaml python /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/<skill-name>
```

## Likely Presentation-Only Or Deletion Candidates

- The long proxy-measurement discussion in the old skill looks like
  presentation. The candidate content is the smaller set of objective,
  authority, ambiguity, evaluation, and validation commitments above.
- The detailed "after the future agent finishes" section may belong in review
  or post-mortem surfaces, not necessarily in a core harness-engineering skill.
- The old references to `agent-project-delta.md` and external provider surveys
  look like migration evidence or historical background, not active
  dependencies for ordinary harness edits.
