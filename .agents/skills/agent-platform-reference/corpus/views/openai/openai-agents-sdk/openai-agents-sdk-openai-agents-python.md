# Reading view: openai-agents-sdk-openai-agents-python

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `openai-agents-sdk-openai-agents-python`
- Source URL: https://openai.github.io/openai-agents-python/
- Resolved URL: https://openai.github.io/openai-agents-python/
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:openai-agents-sdk-openai-agents-python -->

# OpenAI Agents SDK

The [OpenAI Agents SDK](https://github.com/openai/openai-agents-python) enables you to build agentic AI apps in a lightweight, easy-to-use package with very few abstractions. It's a production-ready upgrade of our previous experimentation for agents, [Swarm](https://github.com/openai/swarm/tree/main). The Agents SDK has a very small set of primitives:

* **Agents**, which are LLMs equipped with instructions and tools
* **Agents as tools / Handoffs**, which allow agents to delegate to other agents for specific tasks
* **Guardrails**, which enable validation of agent inputs and outputs

In combination with Python, these primitives are powerful enough to express complex relationships between tools and agents, and allow you to build real-world applications without a steep learning curve. In addition, the SDK comes with built-in **tracing** that lets you visualize and debug your agentic flows, as well as evaluate them and even fine-tune models for your application.

## Why use the Agents SDK

The SDK has two driving design principles:

1. Enough features to be worth using, but few enough primitives to make it quick to learn.
2. Works great out of the box, but you can customize exactly what happens.

Here are the main features of the SDK:

* **Agents**: Build agents with instructions, tools, guardrails, handoffs, and a built-in loop that continues until the task is complete.
* **Sandbox agents**: Run specialists inside real isolated workspaces. Sandbox agents support manifest-defined files, sandbox client selection, and resumable sandbox sessions.
* **Realtime agents**: Build powerful voice agents with `gpt-realtime-2.1`, automatic interruption detection, context management, guardrails, and more.
* **Voice agents**: Build voice pipelines that combine speech-to-text, an agent workflow, and text-to-speech.
* **Python-first**: Use built-in language features to orchestrate and chain agents, rather than needing to learn new abstractions.
* **Agents as tools / Handoffs**: A powerful mechanism for coordinating and delegating work across multiple agents.
* **Guardrails**: Run input validation and safety checks in parallel with agent execution, and fail fast when checks do not pass.
* **Function tools**: Turn any Python function into a tool with automatic schema generation and Pydantic-powered validation.
* **MCP server tool calling**: Built-in integration that exposes remote MCP tools to agents alongside function tools.
* **Sessions**: A persistent memory layer for maintaining working context within an agent loop.
* **Human in the loop**: Built-in mechanisms for involving humans during agent runs.
* **Tracing**: Built-in tracing for visualizing, debugging, and monitoring workflows, with support for the OpenAI suite of evaluation, fine-tuning, and distillation tools.

## Agents SDK or Responses API?

The SDK uses the Responses API by default for OpenAI models, but it wraps model calls in a higher-level runtime.

Use the Responses API directly when:

* you want to own the loop, tool dispatch, and state handling yourself
* your workflow is short-lived and mainly about returning the model's response

Use the Agents SDK when:

* you want the runtime to manage turns, tool execution, guardrails, handoffs, or sessions
* your agent should produce artifacts or operate across multiple coordinated steps
* you need a real workspace or resumable execution through [Sandbox agents](https://openai.github.io/openai-agents-python/sandbox_agents/)

You do not need to choose one globally. Many applications use the SDK for managed workflows and call the Responses API directly for lower-level paths.

## Installation

```
pip install openai-agents
```

## Hello world example

```
from agents import Agent, Runner

agent = Agent(name="Assistant", instructions="You are a helpful assistant")

result = Runner.run_sync(agent, "Write a haiku about recursion in programming.")
print(result.final_output)

# Code within the code,
# Functions calling themselves,
# Infinite loop's dance.
```

(*If running this, ensure you set the `OPENAI_API_KEY` environment variable*)

```
export OPENAI_API_KEY=sk-...
```

## Start here

* Build your first text-based agent with the [Quickstart](https://openai.github.io/openai-agents-python/quickstart/).
* Then decide how you want to carry state across turns in [Running agents](https://openai.github.io/openai-agents-python/running_agents/#choose-a-memory-strategy).
* If the task depends on real files, repos, or isolated per-agent workspace state, read the [Sandbox agents quickstart](https://openai.github.io/openai-agents-python/sandbox_agents/).
* If you are deciding between handoffs and manager-style orchestration, read [Agent orchestration](https://openai.github.io/openai-agents-python/multi_agent/).

## Choose your path

Use this table when you know the job you want to do, but not which page explains it.

| Goal | Start here |
| --- | --- |
| Build the first text agent and see one complete run | [Quickstart](https://openai.github.io/openai-agents-python/quickstart/) |
| Add function tools, hosted tools, or agents as tools | [Tools](https://openai.github.io/openai-agents-python/tools/) |
| Run a coding, review, or document agent inside a real isolated workspace | [Sandbox agents quickstart](https://openai.github.io/openai-agents-python/sandbox_agents/) and [Sandbox clients](https://openai.github.io/openai-agents-python/sandbox/clients/) |
| Decide between handoffs and manager-style orchestration | [Agent orchestration](https://openai.github.io/openai-agents-python/multi_agent/) |
| Keep memory across turns | [Running agents](https://openai.github.io/openai-agents-python/running_agents/#choose-a-memory-strategy) and [Sessions](https://openai.github.io/openai-agents-python/sessions/) |
| Use OpenAI models, websocket transport, or non-OpenAI providers | [Models](https://openai.github.io/openai-agents-python/models/) |
| Review outputs, run items, interruptions, and resume state | [Results](https://openai.github.io/openai-agents-python/results/) |
| Build a low-latency voice agent with `gpt-realtime-2.1` | [Realtime agents quickstart](https://openai.github.io/openai-agents-python/realtime/quickstart/) and [Realtime transport](https://openai.github.io/openai-agents-python/realtime/transport/) |
| Build a speech-to-text / agent / text-to-speech pipeline | [Voice pipeline quickstart](https://openai.github.io/openai-agents-python/voice/quickstart/) |
