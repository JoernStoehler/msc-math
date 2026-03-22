---
name: meta-create-workflow
description: Workflow for designing and writing down workflows. Load when you need to create a new workflow, decide whether it should be a skill or a subagent, or write an agent definition. Covers workflow structure, when to isolate into a subagent, and agent/hook format. For creating conventions instead, see meta-create-conventions. For creating review subagents specifically, see meta-create-review-subagent.
---

# Creating Workflows

A workflow for designing, writing down, and implementing workflows. Workflows are execution properties — they prescribe a sequence of actions to achieve a goal.

## Reference documents

- `references/agent-format.md` — agent definition YAML format, built-in subagent types, checklist
- `references/hook-format.md` — hook script conventions and current hooks

## Related skills

- `meta-foundations` — conceptual foundation (load first if you haven't)
- `meta-create-conventions` — for creating conventions instead of workflows
- `meta-create-review-subagent` — specialized workflow for creating review subagents

## Skill vs subagent vs hook

A workflow can be implemented as:

- **Skill** — the agent reads instructions and follows them in its own context. Use when: the workflow needs the agent's full context (conversation history, current plan), or the workflow is interactive (involves Jorn).
- **Subagent** — the workflow runs in an isolated context with focused instructions. Use when: the workflow benefits from isolation (see reasons below), or the workflow is a well-scoped subtask that can run in parallel.
- **Hook** — the workflow runs as a shell script triggered by a Claude Code event. Use when: the workflow is fully automated (no agent judgment needed), deterministic, and tied to a specific event (session start, worktree creation).

### When to isolate a workflow into a subagent

Subagents are valuable when:
1. **Instruction overload** — the parent agent has too many active concerns; delegating one concern to a focused subagent prevents instruction overload
2. **Parallel execution** — multiple independent subtasks can run simultaneously
3. **Context protection** — the subtask would pollute the parent's context window with irrelevant detail (e.g., reading 50 files for a search)
4. **Failure isolation** — if the subtask fails or goes wrong, the parent can discard the result without damage
5. **Focused expertise** — the subtask benefits from preloaded skills the parent doesn't need

Subagents are NOT valuable when:
- The subtask needs the parent's conversation context
- The subtask is trivial (overhead of delegation > benefit)
- The result needs Jorn's interactive input

## Designing a workflow

### 1. Define the goal

What state should the world be in after the workflow completes? This is the convention the workflow achieves.

### 2. Identify the steps

What sequence of actions achieves the goal? Be specific — "validate the output" is not a step; "run `cargo test --lib` and check for zero failures" is.

### 3. Identify decision points

Where does the workflow branch? What information determines the branch? Make decision criteria explicit.

### 4. Identify failure modes

What can go wrong at each step? Check `meta-foundations/references/agent-failure-modes.md` for common patterns. Design the workflow to account for them structurally (mandatory steps, verification gates), not by hoping agents will remember.

### 5. Choose the implementation

Use the skill/subagent/hook decision above. Then:
- For a skill: write the SKILL.md following `meta-create-conventions/references/skill-format.md`
- For a subagent: write the agent definition following `references/agent-format.md`, put methodology in a skill, preload it via `skills:` field
- For a hook: write the script following `references/hook-format.md`, register in settings.json

### 6. Test by use

Don't ask "is this clear?" — test by having an agent USE the workflow. Check whether the output matches intent.

## Description-as-contract

For both skill and agent descriptions: state what the workflow does AND what it does not guarantee. The parent agent plans as if the description is true. An overpromising description creates a silent gap in the parent's plan.

## Keep agent definitions minimal

Agent definitions define **capabilities** (tools, model, skills), not behavior. If you're writing inline instructions in the agent definition, that content belongs in a skill. The agent definition points to skills; skills contain the methodology.
