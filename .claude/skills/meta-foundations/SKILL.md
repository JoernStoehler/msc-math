---
name: meta-foundations
description: Shared foundation for all meta-layer work — the three repo layers (artifact, convention-and-workflow, meta), conventions vs workflows vs principles, the three work phases (plan, implement, review), progressive disclosure, instruction focus, and the optimization loop. Load before writing or editing skills, CLAUDE.md, or agent definitions.
---

# Meta-Layer Conventions

This file explains the conceptual foundations of how to run a project with agents successfully.

The project can be divided into three layers:
- **Project Artifacts**: the work products we want to produce in order to achieve our project goal.
- **Procedural Project Knowledge**: the accumulated knowledge about how to go about producing the project artifacts.
- **Meta-layer Knowledge**: procedural knowledge about procedural knowledge - how to pick best practices, how to communicate them to the agents, and how to structure the project.

Concretely the layers contain:
- **Artifacts**: code, documents, datasets, experiments — the things we build and maintain. They are commonly found in similar form in projects that do not use agents at all.
- **Procedural knowledge**: conventions that describe what useful artifacts look like, and advice and workflows that help agents to produce them. This is the knowledge that agents directly consult and try to follow when they work. It lives in CLAUDE.md, skills, subagents, and scattered across per-file comments.
- **Meta-layer knowledge**: same files as procedural knowledge. We use a `meta-` prefix to indicate that a skill is about how to create procedural knowledge, rather than create artifacts. Most agents can ignore the meta-layer entirely and focus on producing the artifacts that their task requires, picking and using procedural knowledge as needed.

We loosely divide an agent's lifecycle into phases, which repeat on varous hierarchical levels of organization:
- **Plan**
- **Implement**
- **Review**

The review stage is crucial for agent-led projects, because during implementation the conventions can be both intentionally or unintentionally violated and need to be restored, and during planning the effort of ensuring that conventions are upheld is often too costly compared to scheduling a review phase to catch and correct violations. If the review finds any violations, the agent goes back to the planning phase and usually plans how to fix the violations, then implements the fixes, then reviews again until all is well. Alternatively, agents might privot and instead of fixing the current artifacts, they redo a large portion of the plan and try again from scratch, this time with more information about what unanticipated consequences their original plan had. Lastly, agents might escalate to the human project owner for help if planning for how to fix/how to fulfill the conventions and the task itself has become too difficult.

Without a review phase, agents have ended up overlooking convention violations for long stretches of work and across handoffs between agents, which leads to a large amount of rework and wasted effort, as work begins to build upon falsely trusted earlier work.

## Meta-Layer Files

The main files that constitute the meta-layer are:

- `meta-foundations/SKILL.md` — this file, the entry point for meta-layer design.
- `meta-create-conventions/SKILL.md` — meta-conventions that conventions should follow, and meta-workflows to choose good conventions.
- `meta-create-workflow/SKILL.md` — meta-conventions for workflows, and meta-workflows for designing workflows. Workflows usually live in skills or subagents, here we handle both.
- `meta-foundations/references/agent-failure-modes.md` — background reading on observed agent failure modes and how to design around them.
- `meta-foundations/references/decision-records.md` — background reading on why specific rules and architecture choices exist, to inform future decisions and optimizations.

## Progressive disclosure

Agents degrade as a too large instruction surface is loaded, and too many concerns require or are unintentionally paid attention. To manage this, we use progressive disclosure of knowledge across different files and levels of the project structure. Agents triage what artifacts, procedural knowledge to load for their given task and situation, and can on-the-fly read more as it becomes relevant. Agents delegate narrowly scoped tasks to subagents, so that the subagents can focus on a smaller instruction surface, and the main agent need not even load said instruction surface in some cases.

The project is structured to support this disclosure pattern, using the following special file types that are supported by the claude code agents.

1. **CLAUDE.md** — always loaded. Every agent pays this cost. Only put what every agent needs.
2. **Skill frontmatter** (name + description) — always visible in system prompt.
3. **Skill body** (SKILL.md) — loaded on demand by the agent. Topic-specific conventions and workflows.
4. **Reference docs** (`references/` inside skills) — loaded on demand by the agent. Agents discover them via the skill body.
5. **Agent frontmatter** (name + description) — always visible in system prompt.
6. **Agent body** (`.claude/agents/*.md`) — not loaded, instead visible to the subagent when it is spawned. Contains the workflow and instructions that the subagent follows.

To make progressive disclosure maintainable, we prefer to keep information in a single source of truth, and instead reference it from other places when needed, e.g. a subagent body may reference a skill with a workflow that the subagent will likely find useful. Sometimes we avoid the cost of indirection (the agent has to pay attention to a tool call, and may forget to do so) and duplicate information instead, e.g. as a short summary of what is found in another file, or a copy of a convention that is especially important to incorporate into planning or implementation phase already instead of just review.

## Instruction focus

Agents lose focus and ignore instructions they've read. Token count is a bad proxy for complexity — what matters is: how many behavior modifications does the agent need to hold active? Complex rules (novel behavior, multiple conditions) cost much more than reminders of standard practices.

**Novelty** (relative to agent training) predicts enforcement effort:
- **Common** — agents already do this; a reminder suffices. Example: "write tests."
- **Rare** — seen in training but far from a default; explicit instruction works.
- **Novel** — unseen; needs unambiguous, actionable instructions, ideally with examples to help the agent convert the reading into a behavior. Example: "Reference in the doccomments the mathematical lemma that shows the correctness of the specific algorithm that is implemented using the syntax `/// [file:latex-label]`. Don't write the proof inline into the codefile, instead use the adjacent math.tex files. Follow the usual conventions and workflows for lemma statements and proofs."
- **Anti-intuitive** — clashes with training; agents actively resist. Needs structural enforcement (review agents, mandatory steps), not just instruction. Example: "Don't accept your own mathematical proofs as correct, you are overconfident in your ability to spot gaps and errors in mathematical reasoning. Instead, iterate until you cannot spot any gaps and errors anymore, then instead of declaring the proof correct, hand it to Jörn for a final, trusted review. Only after Jörn explicitly confirms that the proof is correct, mark it as such."

**Countermeasures for focus loss:**
- **Colocate** instructions near the files they govern (file headers, code comments). This way they will be read at all, and will be read at the right time.
- **Disentangle** instructions from factual knowledge to keep the instruction part simpler. Factual knowledge is less costly than procedural knowledge, since it sits dormant until the agent uses it and does not pay attention at all times to it.
- **Pick natural instructions** — if a rule is complex, ask whether a larger change in rules, and a refactoring of the project, can replace the complex rule with a common, perhaps even default, behavior. Example: instead of mixing mathematical proofs with rust code, move the proofs into a separate latex file, and use standard conventions for writing proofs in latex.
- **Review subagents** with focused instruction sets catch what the parent agent missed

## Optimizing rules that don't work

When agents don't follow a rule:
1. **Notice** when a different behavior would have been better
2. **Instruct** agents to do that behavior
3. **Optimize** what behavior to aim for — the rule may fight agent defaults too hard; a different behavior closer to defaults often achieves most of the value
4. **Refactor the project** so the desired behavior becomes the natural default

Steps 3 and 4 work together as an optimization loop, not an escalation ladder. Optimizations can be wholesale switches to entirely different optima, not just local fine-tuning.

**When a rule isn't working**, check the failure modes catalog (`references/agent-failure-modes.md`) for a matching pattern — many rule failures are instances of known failure modes with known structural countermeasures.

Generally, Jörn will be useful and worth it to involve when changing agent behavior, but share the results of your own investigation and optimization attempts to save his time.

## Resource Constraints

The main resources to manage in any agent-led project are:
- **Jörn's time** — the most expensive and scarce resource. The main drain on Jörn's time is not repeating an important question that takes up 10 seconds of his time to answer, but when Jörn reviews deliverables that fail to meet the project conventions and quality standards. Reviews from Jörn should thus be postponed until the deliverable is likely to pass, and reviews should be scoped more narrowly to prioritize sections that are more likely to trigger a restart from scratch if they are wrong. Example:
  - a large feature task relies on some lemma being correct. The agent plans the following: write+review a proof in a loop until the lemma is, to the limits of the agent's ability, correct. Then implement the Rust code, and the Rust tests, to check whether the lemma proof correctly translated into an algorithm that solves the problem. Go back to the proof if the tests fail in ways that suggest the math is wrong. Only after both proof reviews and code tests pass, hand the proof to Jörn for verification.
  - an agent has several questions related to the task which it failed to find an answer to quickly during the planning stage. it decides to ask Jörn these questions now already, and batches them into one block as to reduce total overhead from context switching. The agent provides context about the plan-so-far, so that Jörn can understand the questions more easily and can deliver an answer that's more useful to the agent and might thus preempt follow up questions.
- **Jörn's waiting time** - Jörn parallelises his attention, but there are limits, and so even when agents work autonomously they should not waste wall-time carelessly. Example:
  - agents should avoid running very long simulations or test suites for no good reason.
  - agents should check in with Jörn instead of just starting a plan that they believe will likely take multiple hours until the first feedback on the plan's viability arrives. Jörn can forecast more competently than agents whether a plan is viable, so a minute of his time now can save hours of waiting time after.
- **Instruction Surface Complexity** — Opus 4.6 with 1 million token context window isn't limited by the number of tokens in practice, but by the instruction surface complexity. The main failure mode here is scope creep, when one task after the other is added to the session, instead of wrapping up the session and starting a new one for the new task. A second danger is a lack of delegation, when an agent reads up a lot of conventions itself instead of splitting the task and delegating to subagents that deliver a perhaps not unified, but at least directionally useful result.
- **Misleading formulations, wrong statements, unclear or overly complex artifacts** — agents work fastest when the texts they read can be trusted and they can recombine ideas and facts freely. Any text that causes conflicts in this creative step, or that ends up being relied on despite being false, causes a lot of errors as long as it remains in the repo. It's generally important to look for misleading statements and falsehoods in the artifacts, and to fix them as soon as possible, perhaps via background subagents, rather than deferring fixes to much later. This way agents can stay focused on replacing or expanding old features, instead of rechecking them for correctness and reliability.
- **Feedback loops**: Review agents are the backbone of ensuring the repo remains correct and reliable. Designing feedback loops, and having them available, is crucial to enable reviews at all. We use test driven development for code behavior feedback, mathematical proofs that are timeless once settled for idea correctness feedback, and blameless postmortems for agent process feedback.

