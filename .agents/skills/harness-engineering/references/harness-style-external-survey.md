<!--
Purpose: provider-practice report for future harness-style baseline work.
Context: summarizes external practices gathered by delegated source-collection
packets; it does not recommend repo-specific changes.
-->

# Provider Practices For Current Agent Harnesses

## Scope

Report date: 2026-05-02.

Question: what are OpenAI, Anthropic, and Google/DeepMind doing or describing
with current models for harness, context, prompt, chat, multi-agent,
multi-task, long-running, and human-delegation patterns?

This report is provider-practice-only. It does not translate those practices
into `msc-math` recommendations.

Source adequacy: after the follow-up source audit and provider-specific source
collection, the source set is adequate for public provider practice. It is not
adequate for private/internal harness prompts, private review rubrics, or exact
internal project-management procedures; those are not public in the gathered
sources.

## Cross-Provider Picture

Across the gathered sources, the big-player pattern is not "write a better
prompt" in isolation. Current agent work is described as a system around the
model:

- current-model retuning rather than inherited prompt stacks;
- product-contract or outcome-first prompting;
- explicit tools, state, memory, approvals, traces, and evaluation;
- explicit multi-agent ownership patterns;
- context management and compaction for long work;
- isolated workspaces, sessions, or subagents for parallel work;
- human approval/checkpoints for consequential or blocked actions;
- evaluation of intermediate behavior, not only final answers.

## Current Models Are Retuning Events

OpenAI's [Introducing GPT-5.5](https://openai.com/index/introducing-gpt-5-5/),
published April 23, 2026, describes GPT-5.5 as carrying more work itself on
messy multi-part tasks: planning, using tools, checking work, navigating
ambiguity, and continuing until finished.

OpenAI's [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model)
guide says to treat GPT-5.5 as a new model family, not a drop-in replacement.
It recommends starting from the smallest prompt that preserves the product
contract, then tuning reasoning effort, verbosity, tool descriptions, and
output format against representative examples.

OpenAI's [GPT-5.5 model page](https://developers.openai.com/api/docs/models/gpt-5.5)
lists `gpt-5.5-2026-04-23` as the newest frontier reasoning model for complex
professional work, with support for tools including function calling, web/file
search, tool search, code interpreter, hosted shell, apply patch, skills,
computer use, and MCP.

Anthropic's [Introducing Claude Opus 4.7](https://www.anthropic.com/news/claude-opus-4-7),
published April 16, 2026, describes Opus 4.7 as improving over Opus 4.6 on
advanced software engineering, difficult long-running tasks, instruction
attention, and self-verification before reporting back.

Anthropic's [Introducing Claude Opus 4.6](https://www.anthropic.com/news/claude-opus-4-6),
published February 5, 2026, says Opus 4.6 plans more carefully, sustains
agentic tasks longer, works more reliably in larger codebases, improves review
and debugging, and offers a 1M-token context window in beta.

Google's [Gemini 3 Developer Guide](https://ai.google.dev/gemini-api/docs/gemini-3),
last updated April 28, 2026 UTC, positions Gemini 3.1 Pro for complex
multimodal reasoning and exposes large context windows for current Gemini 3
preview models. It says `thinking_level` controls latency, cost, and reasoning
depth.

## Prompting Is Framed Around Contracts

OpenAI's [GPT-5.5 prompt guidance](https://developers.openai.com/api/docs/guides/prompt-guidance#gpt-5.5-prompting-guide)
says shorter, outcome-first prompts usually work better than process-heavy
prompt stacks. It says prompts should describe what good looks like, what
constraints matter, what evidence is available, and what the final answer
should contain.

OpenAI's GPT-5.5 guidance also says coding workflows need explicit reuse,
subagent delegation, test expectations, acceptance criteria, and
continue-versus-ask rules.

Anthropic's [Define success criteria and build evaluations](https://platform.claude.com/docs/en/test-and-evaluate/develop-tests)
page says prompt engineering should start with clear success criteria and
evaluations, including task-specific evals, edge cases, automation where
possible, and detailed rubrics for LLM-based grading.

Google's [Gemini 3 Developer Guide](https://ai.google.dev/gemini-api/docs/gemini-3)
says Gemini 3 prompting should simplify older chain-of-thought-style prompt
engineering, use concise/direct instructions, keep Gemini 3 temperature at the
default `1.0`, and put specific questions after large data context.

## Harness Means Tools, State, Memory, And Observability

OpenAI's [Agents SDK](https://developers.openai.com/api/docs/guides/agents)
overview describes agents as applications that plan, call tools, collaborate
across specialists, and keep enough state for multi-step work. The SDK is
presented as the code-first path when the application owns orchestration, tool
execution, approvals, state, custom storage, runtime behavior, handoffs,
guardrails, and observability.

OpenAI's [Compaction](https://developers.openai.com/api/docs/guides/compaction),
[Integrations and observability](https://developers.openai.com/api/docs/guides/agents/integrations-observability),
and [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals)
docs describe long-running workflow support through compaction, traces of model
calls/tool calls/handoffs/guardrails/custom spans, trace grading, datasets, and
eval runs.

Anthropic's [Effective context engineering for AI agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents),
published September 29, 2025, defines context engineering as managing the whole
inference state: instructions, tools, MCP, external data, and message history.
It targets high-signal, minimal context.

Anthropic's [Best Practices for Claude Code](https://code.claude.com/docs/en/best-practices)
centers Claude Code practice on context as the scarce resource, explicit
verification criteria, explore-plan-code loops, precise prompts, hooks, skills,
subagents, checkpoints, resume, non-interactive mode, and parallel sessions.

Google ADK's [Agent Development Kit overview](https://adk.dev/get-started/about/)
defines agents, tools, callbacks, session/state, memory, artifacts, code
execution, planning, and model abstraction as first-class primitives.

Google ADK's [Sessions](https://adk.dev/sessions/) and
[Memory](https://adk.dev/sessions/memory/) docs distinguish the current
conversation thread, session scratchpad/task progress, and searchable
cross-session knowledge. Memory can ingest full sessions or incremental event
deltas during long-running sessions.

Google's [Context caching](https://ai.google.dev/gemini-api/docs/caching) docs
describe implicit and explicit caching for repeated large contexts and recommend
placing large common content at the beginning of similar requests to improve
cache hits.

## Coding Agents Use Workspaces, Progress State, And Verification Loops

OpenAI's [Codex](https://openai.com/codex/) page presents Codex as an
end-to-end coding agent for real engineering work, with built-in worktrees,
cloud environments for parallel workflows, Skills for repeatable team
standards, and Automations for always-on background work.

OpenAI's [Codex cloud](https://developers.openai.com/codex/cloud) docs describe
Codex cloud as an environment that can read, edit, and run code, work on
background and parallel tasks, connect to GitHub, and create pull requests.

Anthropic's [Building agents with the Claude Agent SDK](https://claude.com/blog/building-agents-with-the-claude-agent-sdk),
published September 29, 2025, describes the Claude Agent SDK loop as "gather
context -> take action -> verify work -> repeat," with filesystem/search,
subagents, compaction, and tools as primitives.

Anthropic's [Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents),
published November 26, 2025, describes a long-running pattern with an
initializer agent, structured feature list, incremental single-feature sessions,
progress files, git commits, startup orientation, and end-to-end tests.

Anthropic's [Harness design for long-running application development](https://www.anthropic.com/engineering/harness-design-long-running-apps),
published March 24, 2026, reports multi-hour autonomous app-building with
planner/generator/evaluator agents, structured sprint contracts,
Playwright-based evaluation, automatic compaction, and iterative harness
simplification.

Anthropic's [Building a C compiler with a team of parallel Claudes](https://www.anthropic.com/engineering/building-c-compiler),
published February 5, 2026, describes a research prototype using 16 parallel
agents, containers, git-backed task locks, tests, CI, progress docs, and
specialized roles. It also warns that autonomous code still needs human
verification.

## Multi-Agent Work Is Split By Ownership Pattern

OpenAI's [Orchestration and handoffs](https://developers.openai.com/api/docs/guides/agents/orchestration)
distinguishes two patterns:

- **Handoffs:** a specialist takes over a branch of the work.
- **Agents as tools:** a manager remains responsible for the final reply and
  calls specialists as bounded capabilities.

OpenAI says to add specialists only when they materially improve capability
isolation, policy isolation, prompt clarity, or trace legibility.

OpenAI's [Subagents](https://developers.openai.com/codex/subagents) docs
describe explicitly requested Codex subagent workflows where specialized agents
run in parallel, collect results into one response, and inherit sandbox and
approval policy from the parent session.

Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents),
published December 19, 2024, distinguishes workflows with predefined code paths
from agents that dynamically direct their own tool use. It names prompt
chaining, routing, parallelization, orchestrator-workers, and
evaluator-optimizer as different patterns with different fit conditions.

Anthropic's [Create custom subagents](https://code.claude.com/docs/en/sub-agents)
describes Claude Code subagents as specialized assistants with separate context
windows, custom prompts, model/tool/permission settings, scoped MCP access,
hooks, memory, background execution, and optional worktree isolation.

Anthropic's [Orchestrate teams of Claude Code sessions](https://code.claude.com/docs/en/agent-teams)
describes experimental agent teams with a team lead, shared task list,
inter-agent messaging, task claiming, hooks, and human-visible steering.

Google ADK's [Multi-agent systems](https://adk.dev/agents/multi-agents/)
describes parent/sub-agent hierarchies, workflow orchestrators, shared session
state, agent-as-tool delegation, fan-out/gather, hierarchical task
decomposition, generator-critic review, iterative refinement, and
human-in-the-loop patterns.

Google ADK's [Workflow agents](https://adk.dev/agents/workflow-agents/)
separates deterministic workflow agents from LLM agents. Sequential, loop, and
parallel workflow agents provide predictable execution patterns while subagents
keep LLM flexibility.

## Human Involvement Is Built Into The Workflow

OpenAI's [Introducing ChatGPT agent](https://openai.com/index/introducing-chatgpt-agent/),
published July 17, 2025, describes a virtual computer with visual browser, text
browser, terminal, API access, and connectors. It says ChatGPT agent preserves
task context across tools, allows interruption/takeover, asks permission for
consequential actions, and includes prompt-injection and high-impact-action
safeguards.

OpenAI's [Introducing workspace agents in ChatGPT](https://openai.com/index/introducing-workspace-agents-in-chatgpt/),
published April 22, 2026, describes Codex-powered shared agents for repeatable,
long-running team workflows in ChatGPT and Slack, with connected tools, memory,
approvals for sensitive steps, analytics, admin controls, Compliance API
visibility, and prompt-injection safeguards.

OpenAI's [Guardrails and human review](https://developers.openai.com/api/docs/guides/agents/guardrails-approvals)
docs describe guardrails that validate input, output, or tool behavior
automatically, and human review that pauses runs before sensitive side effects.
Approval interruptions return resumable state so the same run can continue after
approval or rejection.

OpenAI's [A practical guide to building agents](https://openai.com/business/guides-and-resources/a-practical-guide-to-building-ai-agents/)
emphasizes eval baselines, tools/connectors, handoffs, exit conditions,
guardrails, and human intervention for failures or high-risk actions.

Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
says agents should get ground truth from the environment at each step, pause for
human feedback at checkpoints or blockers, and use stopping conditions such as
iteration limits.

Google ADK's [Action confirmations](https://adk.dev/tools-custom/confirmation/)
and [Long running function tools](https://adk.dev/tools-custom/function-tools/)
docs describe experimental confirmation for human or supervising-system
approval before tool execution, and long-running tools that pause the run while
a client waits or continues.

## Tool Interfaces Are Designed For Agents

Anthropic's [Writing effective tools for agents](https://www.anthropic.com/engineering/writing-tools-for-agents),
published September 11, 2025, treats tools as agent-facing interfaces. It
recommends evaluation-driven tool design, clear boundaries, high-signal
responses, token-efficient pagination/filtering/truncation, and
prompt-engineered tool descriptions.

Anthropic's [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
Appendix 2 says tool definitions and interfaces deserve as much care as prompts.
It recommends natural formats for the model, reduced formatting overhead,
testing tool use, and interfaces that make mistakes harder. The appendix gives
an example where relative-path mistakes were fixed by requiring absolute file
paths.

Anthropic's [Define tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/define-tools)
documentation says tool descriptions should cover what the tool does, when to
use it, parameter meanings, behavior, caveats, and limitations.

Google's [Function calling with the Gemini API](https://ai.google.dev/gemini-api/docs/function-calling),
last updated December 18, 2025 UTC, describes function calling, parallel calls,
compositional calls, and combining built-in tools with custom functions through
tool context circulation. It says consequential tool calls should be validated
with the user before execution.

Google's [Thought Signatures](https://ai.google.dev/gemini-api/docs/thought-signatures)
docs describe encrypted thought signatures that preserve reasoning context
across multi-step interactions. SDK chat history handles this automatically,
while manual REST/history handling must return signatures exactly, especially
for function calling.

## Long-Running Research And Action Agents Are Productized

Google's [Deep Research Max](https://blog.google/innovation-and-ai/models-and-research/gemini-models/next-generation-gemini-deep-research/),
published April 21, 2026, describes Gemini 3.1 Pro-based autonomous research
agents for long-horizon research over the web, uploaded files, connected stores,
and MCP sources. The output includes reports, charts, and infographics that can
serve as context-gathering stages in larger pipelines.

Google's [Interactions API](https://ai.google.dev/gemini-api/docs/interactions)
is described as a beta unified interface for models and agents, simplifying
state management, tool orchestration, and long-running tasks. It supports
background agent execution, polling, stored interactions,
`previous_interaction_id`, tools, structured outputs, and MCP.

OpenAI's ChatGPT agent and workspace-agent sources describe action-taking chat
agents and shared long-running workplace agents with connected tools, memory,
permissions, and user/admin oversight.

## Evaluation Looks At Intermediate Behavior

OpenAI's [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals)
recommends trace grading while debugging behavior, then datasets and eval runs
when repeatability is needed.

Anthropic's [Demystifying evals for AI agents](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents),
published January 9, 2026, recommends multi-turn agent evals with code-based,
model-based, and human graders; capability evals for hill-climbing, regression
evals for drift, and stable task environments for coding agents.

Google ADK's [Why Evaluate Agents](https://adk.dev/evaluate/) treats agent eval
as final-response quality plus trajectory/tool-use quality. It supports
expected-vs-actual trajectory comparison, built-in metrics, eval sets, web UI
inspection, pytest integration, `adk eval`, and trace views.

Google's [Vertex AI agent evaluation](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/models/evaluation-agents)
adds trajectory evaluation for agents beyond final-response scoring.

The shared pattern is that final answers are not enough. Tool use, intermediate
responses, session state, handoffs, approvals, and traces are evaluation
targets.

## Public-Source Limits

The gathered provider sources do not expose private internal prompt libraries,
full internal harness repositories, exact private compaction or memory
algorithms, internal review rubrics, or complete internal project-management
procedures.

Some current surfaces are preview or experimental:

- OpenAI workspace agents were described as a research preview in April 2026.
- Anthropic agent teams are documented as experimental and disabled by default.
- Parts of Google ADK human-in-loop support are described as experimental or
  language-specific.

Some pages do not expose visible publication or update dates; those are marked
as date-not-visible in the source list.

## Source List

- OpenAI, [Introducing GPT-5.5](https://openai.com/index/introducing-gpt-5-5/),
  published April 23, 2026.
- OpenAI, [Using GPT-5.5](https://developers.openai.com/api/docs/guides/latest-model),
  date not visible in gathered source.
- OpenAI, [GPT-5.5 model page](https://developers.openai.com/api/docs/models/gpt-5.5),
  snapshot `gpt-5.5-2026-04-23`.
- OpenAI, [GPT-5.5 prompt guidance](https://developers.openai.com/api/docs/guides/prompt-guidance#gpt-5.5-prompting-guide),
  date not visible in gathered source.
- OpenAI, [Codex](https://openai.com/codex/), date not visible in gathered
  source.
- OpenAI, [Codex cloud](https://developers.openai.com/codex/cloud), date not
  visible in gathered source.
- OpenAI, [Subagents](https://developers.openai.com/codex/subagents), date not
  visible in gathered source.
- OpenAI, [Introducing workspace agents in ChatGPT](https://openai.com/index/introducing-workspace-agents-in-chatgpt/),
  published April 22, 2026.
- OpenAI, [Introducing ChatGPT agent](https://openai.com/index/introducing-chatgpt-agent/),
  published July 17, 2025.
- OpenAI, [Agents SDK](https://developers.openai.com/api/docs/guides/agents),
  date not visible in gathered source.
- OpenAI, [Orchestration and handoffs](https://developers.openai.com/api/docs/guides/agents/orchestration),
  date not visible in gathered source.
- OpenAI, [Guardrails and human review](https://developers.openai.com/api/docs/guides/agents/guardrails-approvals),
  date not visible in gathered source.
- OpenAI, [Compaction](https://developers.openai.com/api/docs/guides/compaction),
  date not visible in gathered source.
- OpenAI, [Integrations and observability](https://developers.openai.com/api/docs/guides/agents/integrations-observability),
  date not visible in gathered source.
- OpenAI, [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals),
  date not visible in gathered source.
- OpenAI, [A practical guide to building agents](https://openai.com/business/guides-and-resources/a-practical-guide-to-building-ai-agents/),
  date not visible in gathered source.
- Anthropic, [Introducing Claude Opus 4.7](https://www.anthropic.com/news/claude-opus-4-7),
  published April 16, 2026.
- Anthropic, [Introducing Claude Opus 4.6](https://www.anthropic.com/news/claude-opus-4-6),
  published February 5, 2026.
- Anthropic, [Best Practices for Claude Code](https://code.claude.com/docs/en/best-practices),
  date not visible in gathered source.
- Anthropic, [Building agents with the Claude Agent SDK](https://claude.com/blog/building-agents-with-the-claude-agent-sdk),
  published September 29, 2025.
- Anthropic, [Create custom subagents](https://code.claude.com/docs/en/sub-agents),
  date not visible in gathered source.
- Anthropic, [Orchestrate teams of Claude Code sessions](https://code.claude.com/docs/en/agent-teams),
  date not visible in gathered source.
- Anthropic, [Effective context engineering for AI agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents),
  published September 29, 2025.
- Anthropic, [Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents),
  published November 26, 2025.
- Anthropic, [Harness design for long-running application development](https://www.anthropic.com/engineering/harness-design-long-running-apps),
  published March 24, 2026.
- Anthropic, [Building a C compiler with a team of parallel Claudes](https://www.anthropic.com/engineering/building-c-compiler),
  published February 5, 2026.
- Anthropic, [Writing effective tools for agents](https://www.anthropic.com/engineering/writing-tools-for-agents),
  published September 11, 2025.
- Anthropic, [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents),
  published December 19, 2024.
- Anthropic, [Define tools](https://platform.claude.com/docs/en/agents-and-tools/tool-use/define-tools),
  date not visible in gathered source.
- Anthropic, [Define success criteria and build evaluations](https://platform.claude.com/docs/en/test-and-evaluate/develop-tests),
  date not visible in gathered source.
- Anthropic, [Demystifying evals for AI agents](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents),
  published January 9, 2026.
- Google, [Gemini 3 Developer Guide](https://ai.google.dev/gemini-api/docs/gemini-3),
  last updated April 28, 2026 UTC.
- Google, [Function calling with the Gemini API](https://ai.google.dev/gemini-api/docs/function-calling),
  last updated December 18, 2025 UTC.
- Google, [Thought Signatures](https://ai.google.dev/gemini-api/docs/thought-signatures),
  date not visible in gathered source.
- Google, [Context caching](https://ai.google.dev/gemini-api/docs/caching),
  date not visible in gathered source.
- Google, [Interactions API](https://ai.google.dev/gemini-api/docs/interactions),
  date not visible in gathered source.
- Google, [Deep Research Max](https://blog.google/innovation-and-ai/models-and-research/gemini-models/next-generation-gemini-deep-research/),
  published April 21, 2026.
- Google ADK, [Agent Development Kit overview](https://adk.dev/get-started/about/),
  date not visible in gathered source.
- Google ADK, [Sessions](https://adk.dev/sessions/), date not visible in
  gathered source.
- Google ADK, [Memory](https://adk.dev/sessions/memory/), date not visible in
  gathered source.
- Google ADK, [Multi-agent systems](https://adk.dev/agents/multi-agents/),
  date not visible in gathered source.
- Google ADK, [Workflow agents](https://adk.dev/agents/workflow-agents/),
  date not visible in gathered source.
- Google ADK, [Action confirmations](https://adk.dev/tools-custom/confirmation/),
  date not visible in gathered source.
- Google ADK, [Long running function tools](https://adk.dev/tools-custom/function-tools/),
  date not visible in gathered source.
- Google ADK, [Why Evaluate Agents](https://adk.dev/evaluate/), date not
  visible in gathered source.
- Google Cloud, [Vertex AI agent evaluation](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/models/evaluation-agents),
  date not visible in gathered source.
