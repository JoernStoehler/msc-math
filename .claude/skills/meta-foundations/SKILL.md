---
name: meta-foundations
description: Shared foundation for all meta-layer work — the three repo layers (artifact, convention-and-workflow, meta), conventions vs workflows, the three work phases (plan, implement, review), progressive disclosure, instruction focus, and the optimization loop. Load before writing or editing skills, CLAUDE.md, or agent definitions. Does NOT contain how-to guidance — see meta-create-conventions, meta-create-workflow, meta-create-review-subagent for that.
---

# Meta-Layer Conventions

The conceptual foundation for all meta-layer work. Load this first; load the creation workflows when you need to act.

## Creation workflows (actionable how-to)

- `meta-create-conventions` — workflow for designing and writing down conventions
- `meta-create-workflow` — workflow for designing and writing down workflows
- `meta-create-review-subagent` — workflow for creating review subagents (specialized form of workflow)

## Reference documents

- `references/agent-failure-modes.md` — catalog of observed agent failure modes with design implications
- `references/decision-records.md` — why specific rules and architecture choices exist

## The three layers

This repo has three layers. Each layer's conventions and workflows govern the layer below it.

1. **Artifact layer** — The work product: Rust code, LaTeX thesis, Python experiments, datasets. What gets built.
2. **Convention and workflow layer** — Conventions and workflows for producing artifacts. Coding style, review sequences, session phases, agent orchestration, communication norms. Lives in CLAUDE.md, skills, rules, and agent definitions.
3. **Meta layer** — Conventions and workflows for selecting and communicating conventions and workflows to agents. How to write skills, structure CLAUDE.md, define agents, decide where knowledge goes, sync shared files across repos. Lives in the `meta-*` skills.

The meta layer and the convention and workflow layer are the same kind of thing — both are conventions and workflows. They differ only in subject: layer 2 governs artifact production, layer 3 governs layer 2 itself.

In `.claude/skills/`, the `meta-` prefix identifies meta-layer content. Skills without the prefix are convention-and-workflow-layer content. Artifact-layer content doesn't live in `.claude/` at all.

## Conventions vs workflows

Two fundamentally different kinds of knowledge, requiring different design approaches:

- **Convention** — a target state property. "Achieve Y to enable Z." Constrains the space of acceptable states. Examples: "code has doc comments," "skills have description-as-contract," "meta-* skills are identical across repos."
- **Workflow** — an execution property. "Do X to achieve Y." Prescribes a sequence of actions. Examples: "before presenting .tex, run review subagents," "when feedback comes in: fix → abstract → scan → record."

Many conventions have a companion workflow that achieves or maintains the convention. When designing either, ask: is this about what the world should look like (convention), or what I should do (workflow)?

## The three work phases

Agents oscillate between three phases: **plan → implement → review**. Conventions and workflows must work across all three:

- **Planning** — the agent decides what to do. Conventions should be predictable here, ideally aligning with common practices agents learned in training.
- **Execution** — the agent produces artifacts. Conventions should be actionable and measurable — the agent can tell whether it's following them as it works.
- **Review** — the agent (or a subagent) verifies the result. Conventions should be verifiable or near-completely falsifiable.

A good convention carries through all three phases. This is why good conventions make separate review checklists redundant — the convention IS the review specification. Checklists only add value for error detection patterns that aren't conventions (e.g., "look for unargued claims in proofs").

## Progressive disclosure

Controls context window cost. Each level is visible to fewer agents:

1. **CLAUDE.md** — always loaded. Every agent pays this cost. Only put what every agent needs.
2. **Skill frontmatter** (name + description) — always visible in system prompt. Agents self-select by reading descriptions.
3. **Skill body** (SKILL.md) — loaded on demand. Topic-specific conventions and workflows.
4. **Reference docs** (`references/` inside skills) — loaded by agents within a skill context. Detailed procedures, checklists, detection rules. Agents discover them via the skill body.
5. **Agent definitions** (`.claude/agents/`) — define capabilities (tools, model, skills), not behavior. Behavior comes from preloaded skills.

**Key invariant:** Each piece of knowledge has exactly one canonical home. If a convention is in a skill, the review checklist references the skill — it doesn't restate it. Duplication causes drift.

## Source of truth

A piece of text is either a source of truth or a derivative (summary, concretization, implication). The distinction applies at the paragraph or subsentence level — a single file typically contains both. When a derivative diverges from its source of truth, the source wins. Derivatives must not be mistaken for sources.

## Instruction focus

Agents lose focus and ignore instructions they've read. Token count is a bad proxy for complexity — what matters is: how many behavior modifications does the agent need to hold active? Complex rules (novel behavior, multiple conditions) cost much more than reminders of standard practices.

**Novelty** (relative to agent training) predicts enforcement effort:
- **Common** — agents already do this; a reminder suffices. Example: "write tests."
- **Rare** — seen in training but not default; explicit instruction works.
- **Novel** — unseen; needs a skill with examples; agent has no prior.
- **Anti-intuitive** — clashes with training; agents actively resist. Needs structural enforcement (review agents, mandatory steps), not just instruction.

**Countermeasures for focus loss:**
- **Colocate** instructions near the files they govern (file headers, code comments)
- **Disentangle** instructions from factual knowledge to keep the instruction part simpler
- **Pick natural instructions** — if a rule is complex, ask whether a codebase change could make the desired behavior default
- **Review subagents** with focused instruction sets catch what the parent agent missed

## Optimizing rules that don't work

When agents don't follow a rule:
1. **Notice** when a different behavior would have been better
2. **Instruct** agents to do that behavior
3. **Optimize** what behavior to aim for — the rule may fight agent defaults too hard; a different behavior closer to defaults often achieves most of the value
4. **Refactor the project** so the desired behavior becomes the natural default

Steps 3 and 4 work together as an optimization loop, not an escalation ladder. Optimizations can be wholesale switches to entirely different optima, not just local fine-tuning.

**When a rule isn't working**, check the failure modes catalog (`references/agent-failure-modes.md`) for a matching pattern — many rule failures are instances of known failure modes with known structural countermeasures.
