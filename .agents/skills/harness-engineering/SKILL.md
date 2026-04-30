---
name: harness-engineering
description: "Use when editing or reviewing repo-local material whose purpose is to affect future agent behavior: `AGENTS.md`, `.agents/skills/**`, `.codex/agents/*.toml`, Codex config/reference notes, onboarding wording, subagent/review prompts, task packets, handoffs, and agent-facing roadmap/task instructions."
---

# Harness Engineering

## Objective

This skill guides harness engineering: writing or reviewing prompt material so
future agents follow the author's intended behavior and achieve the author's
intended tasks.

Use this skill when you intend to change how future agents behave. GPT-5.5
already knows ordinary code and prose conventions from training; do not use this
skill for Rust comments, thesis prose, proof drafts, experiment write-ups,
datasets, or figures unless the edit changes instruction, routing, authority,
review, handoff, or other steering text.

Prompt material is purely instrumental. It succeeds if future agents succeed at
their tasks. The prompt itself has no thesis-project value.

## Success Measurement

Direct success is observed when future agents use the prompt material to achieve
the author's intended task. Before that outcome is observable, use the checks
below as proxy measurements. They are not necessary or sufficient in general;
adapt, combine, or skip them based on context, risk-benefit tradeoffs, prior
prompting attempts, and Jörn's harness-engineering expertise.

### While Writing Prompt Material

- The prompt specifies the intended task or behavior, including how to measure
  success.
- The specified success measure is concrete enough for the future agent to
  operationalize without redefining the objective.
  - Leave task-local implementation choices to the future agent: which commands
    to run, which parts of test suites matter, and whether or how to delegate.
  - Break a criterion down only until GPT-5.5 can specialize it quickly. Do not
    write every `rg` command when the criterion implies an obvious search.
- The prompt is clear, specific, and unambiguous where a wrong reading would
  change the task.
  - When ambiguity matters and delegation is authorized, ask a fresh non-fork
    subagent what is hard to understand or act on. Treat that as evidence, not
    authority.
  - A grep for words that often appear in vague phrases finds no true positives.
- The future agent receives a threshold and escape hatch for aborting or
  rejecting the task if the prompt is too ambiguous to carry out without likely
  damage.
- The prompt does not specify a plan or path prematurely.
  - Once the objective and success measure are explicit, GPT-5.5 can choose an
    ad-hoc, context-specific path.
  - Suggestions for a path are stated as non-binding suggestions.
  - Include enough context for the future agent to tell when suggestions apply
    and what tradeoffs they make.
- Binding constraints are specified as part of the objective and are measured for
  task success.
  - Include enough context for GPT-5.5 to know where constraints come from and
    whether they are binding.
  - Constraints that are known to be necessary are not worded as mere
    suggestions.
- Historic context, especially learnings from prior attempts, is recorded as
  history. Do not promote it to constraints unless the constraint is now known to
  be necessary, not merely helpful or maybe necessary.
- The future agent is not asked to both choose or operationalize a complex
  objective and achieve that objective.
  - This creates bad feedback loops where agents often pick objectives that are
    too easy, misunderstood, unproductive, incomplete, or messy.
  - If pick-and-implement is needed, split it into two stages with success
    measurement for the "pick" stage in between.
  - A handoff to a second agent is often cleaner.
  - If the objective or success criteria are mutable, state which components are
    open to interpretation and what the interpretation may be based on.
- The requested final response or deliverable records what was delivered and the
  success-measurement results, including missing or failed signals.
- For complex tasks, ask the future agent to reject nonsensical or unproductive
  objectives, even if they are achievable.
  - For simple tasks, this pulls in expensive context considerations for little
    gain.
  - For complex tasks, the future agent will consider that context anyway while
    disambiguating the objective, planning, executing, and measuring success.
- Put stable context before task-specific context when that separation changes
  how future agents interpret authority or task-local facts.
- Put dynamic handoff details near the end of a packet when that separates stable
  instructions from task-local facts.

### After The Future Agent Finishes

- Compare the deliverable against the intended task.
  - If the future agent provides evidence of success, check that first.
  - Ask an independent fresh subagent for a summary of the deliverable without
    knowing the prompt, then compare the summary against your intent rather than
    only against the prompt.
  - Rerun the success-measure checks when they are still relevant.
- For imperfect results, decide between escalation to Jörn or a parent agent,
  discarding the attempt and starting again with appended learnings, following
  up with a repair prompt, or accepting the imperfect result with visible
  residual risk.
  - Redo the whole prompt only when the learnings are substantial or reveal
    substantial wrong assumptions in how the prompt was formulated.
  - When the learning generalizes, recheck other parts of the prompt for similar
    issues.

## Durable Prompt Material

Durable prompt material includes `AGENTS.md`, `.agents/skills/**`,
`.codex/agents/*.toml`, `.codex/config.toml`, reusable reference notes, and other
long-lived text that future agents may treat as instructions.

Durable prompt material has higher stakes and different dynamics than a one-shot
prompt or handoff:

- It steers repeated future sessions.
- It adds a maintenance burden and increases complexity of the total harness
  surface.
- It can be edited gradually, and many agents provide indirect feedback on it
  over time.
- Usually it involves material that is broader, more general, or more
  multi-purpose in scope, so feedback is less attributable to specific items.
- It depends on other prompt material, including skill trigger descriptions,
  subagent role authority, loaded instruction surfaces, and nearby prompts or
  docs that may contradict the edit.

Durable harness edits require a current Jörn request and final Jörn review before
the change is treated as durable policy.

Jörn blanket-authorizes task-local prompts and temporary handoffs if their
failures stay observable, bounded, and reversible.

When replacing the purpose of durable prompt material, replace the old file or
move it out of the active surface and create a fresh file. Do not mutate an old
instruction file into a new-purpose file; that adds confusing complexity and
pulls the replacement toward the old file purpose.

Before deleting, moving, or replacing an active path, inspect
`git status --short -- <path>`. If the path is dirty, untracked, or ownership is
unclear, stop and ask Jörn before changing it.

## Required References

Use `$skill-creator` before edits that change a skill's behavior:

- creating, deleting, renaming, or replacing a skill
- changing a skill `description`
- adding or removing sections, procedures, validation gates, or trigger logic
- changing examples when the example teaches a rule

You do not need `$skill-creator` for review-only work, typo fixes, formatting
that does not change behavior, or stale-path cleanup outside a skill.

Use `$openai-docs` before edits that depend on current OpenAI or Codex behavior:

- GPT-5.5 prompt modernization or model migration
- model, reasoning effort, verbosity, tool-use, state, compaction, or hosted-tool
  guidance
- claims about Codex product behavior or OpenAI recommendations

Fetch https://developers.openai.com/api/docs/guides/latest-model.md and follow
the current `promptingGuide` and `migrationGuide` links returned there.

Do not copy summaries of OpenAI guidance into repo prompt material. Use the docs
to shape the edit, then encode the repo-owned convention or role contract.

When the change is about complexity, architecture, or agent-facing readability in
an agent-heavy project, read
[`references/agent-project-delta.md`](references/agent-project-delta.md).

Use `$post-mortem` when extracting learnings from a completed session or when
reaching a milestone, especially when evidence accumulates over multiple
sessions.

## Validation

Run validation for the touched surface:

```bash
git diff --check
```

For touched skill folders, also run:

```bash
uv run --with pyyaml python /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/<skill-name>
```
