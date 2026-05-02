<!--
Purpose: external evidence report for future harness-style baseline work.
Context: source facts and candidate applications only; this is not active repo
policy.
-->

# External Harness Style Evidence Report

## Scope

Report date: 2026-05-02.

This report is evidence input for the harness replacement plan in
`tasks/infrastructure.md`. It summarizes current provider and
research-engineering sources about agent harnesses, prompt style, orchestration,
recovery, evaluation, and model-current prompt migration.

Epistemic status:

- **Source fact:** what the cited source says.
- **Cross-source pattern:** synthesis across cited source facts.
- **Candidate application:** a possible application to this repo, not policy.
  Jorn should accept, revise, or reject it before active harness text changes.

This report does not claim external harness expertise. It organizes evidence for
the next design discussion.

## Current-Model Prompt Migration

**Source facts**

- OpenAI's [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model.md)
  guide identifies `gpt-5.5` as the latest model, says to treat it as a new
  model family, and recommends starting from the smallest prompt that preserves
  the product contract before tuning reasoning effort, verbosity, tool
  descriptions, and output format.
- OpenAI's [GPT-5.5 prompt guidance](https://developers.openai.com/api/docs/guides/prompt-guidance#gpt-5.5-prompting-guide)
  says shorter outcome-first prompts usually work better than process-heavy
  prompt stacks. It says prompts should describe what good looks like, what
  constraints matter, what evidence is available, and what the final answer
  should contain.
- OpenAI's GPT-5.5 guide says coding workflows need explicit reuse, subagent
  delegation, test expectations, acceptance criteria, and continue-versus-ask
  rules.
- Anthropic's [Introducing Claude Opus 4.7](https://www.anthropic.com/news/claude-opus-4-7),
  published 2026-04-16, says Opus 4.7 improves on Opus 4.6 for advanced
  software engineering and complex long-running tasks. It also says Opus 4.7
  follows instructions more literally, so prompts written for earlier models can
  produce unexpected results and users should retune prompts and harnesses.

**Cross-source pattern**

Current frontier coding and agent models should not inherit older prompt stacks
unchanged. The common pattern is model-current retuning around product contract,
success criteria, tool/interface descriptions, and observed behavior.

**Candidate applications**

- Audit inherited harness prose for stale GPT-4 through GPT-5.3 and
  older-Claude assumptions.
- Keep binding constraints explicit, but preserve process recipes only where the
  path matters or observed failures justify them.
- Treat GPT-5.5 and current Claude Opus behavior as separate from earlier model
  folklore. This report contains a current Opus 4.7 source; it does not
  independently survey Opus 4.6 docs.

## Orchestration And Delegation

**Source facts**

- OpenAI's [Agents SDK](https://developers.openai.com/api/docs/guides/agents)
  overview describes agents as applications that plan, call tools, collaborate
  across specialists, and keep enough state for multi-step work. It points to
  tracing, guardrails, human review, state, and orchestration as workflows grow.
- OpenAI's [Orchestration and handoffs](https://developers.openai.com/api/docs/guides/agents/orchestration)
  separates **handoffs**, where a specialist takes over a branch, from
  **agents as tools**, where a manager remains responsible for the final reply.
  It says to add specialists only when they improve capability isolation, policy
  isolation, prompt clarity, or trace legibility.
- Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents),
  published 2024-12-19, distinguishes workflows with predefined code paths from
  agents that dynamically direct their own tool use.
- The same Anthropic article distinguishes prompt chaining, routing,
  parallelization, orchestrator-workers, and evaluator-optimizer patterns.

**Cross-source pattern**

Delegation style depends on ownership and work shape. Handoff, manager-owned
specialist calls, parallelization, orchestrator-worker, and evaluator-optimizer
are different patterns.

**Candidate applications**

- Avoid vague ownership phrases. Name what the delegating session owns:
  synthesis, verification, integration, tracker updates, final answer,
  merge-readiness judgment, or policy decision.
- Use true handoff only when the task explicitly moves ownership. Otherwise
  describe subagents as bounded evidence, patch, review, or packet producers.
- Explain why the chosen delegation pattern matches the work shape.

## Chat Failures And Recovery

**Source facts**

- OpenAI's [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model.md)
  says long-running, tool-heavy, or evidence-gathering workflows need success
  criteria and stopping rules.
- OpenAI's [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals)
  says traces help debug workflow-level questions such as whether the agent
  picked the right tool, whether a handoff happened when it should, and whether
  a workflow violated an instruction or safety policy.
- Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
  says agents should get ground truth from the environment at each step, pause
  for human feedback at checkpoints or blockers, and include stopping
  conditions such as iteration limits.

**Cross-source pattern**

The sources do not provide a ready-made chat style for difficult Codex sessions.
They do support treating breakdowns as workflow state: inspect the current
trace/environment, compare behavior to success criteria, then stop, ask, or
recover according to explicit rules.

**Candidate applications**

- Classify difficult chat by concrete failure mode: unclear goal,
  interruption/state change, accumulated tool or interpretation errors,
  premature implementation, or communication collapse.
- Recovery should begin with current evidence: latest user request, cwd, git
  status, diffs, logs, tests, source citations, and changed files.
- A restart path may be appropriate when continuing the same conversation would
  carry too much confused state.

## Goal Clarification

**Source facts**

- Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
  describes agents as beginning with either a command or interactive discussion;
  once the task is clear, agents plan and operate independently.
- OpenAI's [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model.md)
  emphasizes success criteria and stopping rules for long-running and
  tool-heavy workflows.

**Cross-source pattern**

Clarification and execution are different phases. The sources do not specify
how Codex should implement that distinction, but they support a task-clear
checkpoint before execution begins.

**Candidate applications**

- Goal clarification mode should have a different success condition from normal
  coding mode: clarified objective, intended artifact, scope, non-goals,
  decision gates, and first executable step.
- The mode should prevent implementation-by-inertia unless Jorn explicitly asks
  for action during clarification.

## Flat Packet Orchestration

**Source facts**

- Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
  says parallelization is useful when subtasks can be divided and run in
  parallel or when multiple focused perspectives are useful.
- Google ADK's [Multi-agent systems](https://adk.dev/agents/multi-agents/)
  page describes a parallel fan-out/gather pattern where subagents write results
  to distinct shared-state keys and a later agent aggregates them.
- Google ADK's [Workflow agents](https://adk.dev/agents/workflow-agents/)
  separates deterministic workflow agents from LLM agents; sequential, loop,
  and parallel workflow agents provide predictable execution patterns while
  subagents keep LLM flexibility.

**Cross-source pattern**

Large flat work surfaces can be prepared deterministically even when each packet
uses model judgment locally. Parallel work needs distinct outputs and an
explicit aggregation step.

**Candidate applications**

- The data-science experiment group may need a packet schema before broad
  delegation: packet id, input artifact, method family, write scope, output
  artifact, validation, stop condition, and aggregation contract.
- Parallelism should follow packet independence and output isolation, not a
  general desire to go faster.

## Agent-Computer Interface

**Source facts**

- Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
  Appendix 2 argues that tool definitions and interfaces deserve as much care
  as prompts. It recommends natural formats for the model, reduced formatting
  overhead, testing tool use, and interfaces that make mistakes harder.
- The same appendix says a relative-path failure in an agent was fixed by
  requiring absolute file paths.
- Anthropic's [Define tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/define-tools)
  documentation says tool descriptions should cover what the tool does, when to
  use it, parameter meanings, behavior, caveats, and limitations.

**Cross-source pattern**

Some harness failures are interface failures rather than only prompt failures.
The agent needs unambiguous fields for actions, side effects, and constraints.

**Candidate applications**

- Treat cwd, worktree, write scope, dirty files, generated artifacts,
  validation commands, and escalation boundaries as packet/interface fields.
- Distinguish mandatory fields from background context that an agent can infer.

## Evaluation And Review

**Source facts**

- OpenAI's [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals)
  recommends trace grading while debugging behavior, then datasets and eval runs
  when repeatability is needed.
- Anthropic's [Define success criteria and build evaluations](https://platform.claude.com/docs/en/test-and-evaluate/develop-tests)
  says prompt engineering should start by defining success criteria and
  designing evaluations. It recommends task-specific evals with edge cases and
  automation when possible.
- Google ADK's [Why Evaluate Agents](https://adk.dev/evaluate/) describes
  evalsets that can include user queries, expected tool use, expected
  intermediate responses, reference responses, and initial session state.
- Google Cloud's [Scale your agents](https://docs.cloud.google.com/gemini-enterprise-agent-platform/scale)
  page, last updated 2026-05-01 UTC, surfaces sessions, memory, code execution,
  testing deployed agents, observability, and evaluation as separate scaling
  concerns.

**Cross-source pattern**

Agent behavior should be evaluated at workflow/trace level, not only by final
answer. Intermediate state and tool choices are first-class evidence.

**Candidate applications**

- Harness review should inspect intermediate behavior: tool choices,
  cwd/worktree behavior, delegation boundaries, validation, stop behavior,
  uncertainty marking, and interruption handling.
- Reusable eval/check machinery should wait until the failure category is stable
  enough to measure.

## Low-Signal Or Deferred Sources

- Generic prompt-tip pages were skipped when they did not address durable
  harness behavior, orchestration, recovery, or evaluation.
- Non-official summaries, podcasts, Reddit threads, and copied PDFs were not
  used because official provider sources already covered the main patterns.
- Adjacent academic work on coding-agent rules was deferred. It may be useful
  later, but this report focused on provider and research-engineering sources.

## Source List

- OpenAI, [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model.md),
  fetched 2026-05-02.
- OpenAI, [GPT-5.5 prompt guidance](https://developers.openai.com/api/docs/guides/prompt-guidance#gpt-5.5-prompting-guide),
  fetched 2026-05-02.
- OpenAI, [Agents SDK](https://developers.openai.com/api/docs/guides/agents),
  fetched 2026-05-02.
- OpenAI, [Orchestration and handoffs](https://developers.openai.com/api/docs/guides/agents/orchestration),
  fetched 2026-05-02.
- OpenAI, [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals),
  fetched 2026-05-02.
- Anthropic, [Introducing Claude Opus 4.7](https://www.anthropic.com/news/claude-opus-4-7),
  published 2026-04-16.
- Anthropic, [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents),
  published 2024-12-19.
- Anthropic, [Define tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/define-tools),
  checked 2026-05-02.
- Anthropic, [Define success criteria and build evaluations](https://platform.claude.com/docs/en/test-and-evaluate/develop-tests),
  checked 2026-05-02.
- Google ADK, [Multi-agent systems](https://adk.dev/agents/multi-agents/),
  checked 2026-05-02.
- Google ADK, [Workflow agents](https://adk.dev/agents/workflow-agents/),
  checked 2026-05-02.
- Google ADK, [Why Evaluate Agents](https://adk.dev/evaluate/),
  checked 2026-05-02.
- Google Cloud, [Scale your agents](https://docs.cloud.google.com/gemini-enterprise-agent-platform/scale),
  last updated 2026-05-01 UTC.
