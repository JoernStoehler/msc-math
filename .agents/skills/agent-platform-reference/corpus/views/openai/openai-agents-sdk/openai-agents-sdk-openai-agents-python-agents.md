# Reading view: openai-agents-sdk-openai-agents-python-agents

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `openai-agents-sdk-openai-agents-python-agents`
- Source URL: https://openai.github.io/openai-agents-python/agents/
- Resolved URL: https://openai.github.io/openai-agents-python/agents/
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:openai-agents-sdk-openai-agents-python-agents -->

# Agents

Agents are the core building block in your apps. An agent is a large language model (LLM) configured with instructions, tools, and optional runtime behavior such as handoffs, guardrails, and structured outputs.

Use this page when you want to define or customize a single base `Agent` rather than a `SandboxAgent`. If you are deciding how multiple agents should collaborate, read [Agent orchestration](https://openai.github.io/openai-agents-python/multi_agent/). If the agent should run inside an isolated workspace with manifest-defined files and sandbox-native capabilities, read [Sandbox agent concepts](https://openai.github.io/openai-agents-python/sandbox/guide/).

The SDK uses the Responses API by default for OpenAI models, but the distinction here is orchestration: `Agent` plus `Runner` lets the SDK manage turns, tools, guardrails, handoffs, and sessions for you. If you want to own that loop yourself, use the Responses API directly instead.

## Choose the next guide

Use this page as the hub for agent definition. Jump to the adjacent guide that matches the next decision you need to make.

| If you want to... | Read next |
| --- | --- |
| Choose a model or provider setup | [Models](https://openai.github.io/openai-agents-python/models/) |
| Add capabilities to the agent | [Tools](https://openai.github.io/openai-agents-python/tools/) |
| Run an agent against a real repo, document bundle, or isolated workspace | [Sandbox agents quickstart](https://openai.github.io/openai-agents-python/sandbox_agents/) |
| Decide between manager-style orchestration and handoffs | [Agent orchestration](https://openai.github.io/openai-agents-python/multi_agent/) |
| Configure handoff behavior | [Handoffs](https://openai.github.io/openai-agents-python/handoffs/) |
| Run turns, stream events, or manage conversation state | [Running agents](https://openai.github.io/openai-agents-python/running_agents/) |
| Inspect final output, run items, or resumable state | [Results](https://openai.github.io/openai-agents-python/results/) |
| Share local dependencies and runtime state | [Context management](https://openai.github.io/openai-agents-python/context/) |

## Basic configuration

The most common properties of an agent are:

| Property | Required | Description |
| --- | --- | --- |
| `name` | yes | Human-readable agent name. |
| `instructions` | no | System prompt or dynamic instructions callback. Strongly recommended. See [Dynamic instructions](https://openai.github.io/openai-agents-python/agents/#dynamic-instructions). |
| `prompt` | no | OpenAI Responses API prompt configuration. Accepts a static prompt object or a function. See [Prompt templates](https://openai.github.io/openai-agents-python/agents/#prompt-templates). |
| `handoff_description` | no | Short description exposed when this agent is offered as a handoff target. |
| `handoffs` | no | Delegate the conversation to specialist agents. See [handoffs](https://openai.github.io/openai-agents-python/handoffs/). |
| `model` | no | Which LLM to use. See [Models](https://openai.github.io/openai-agents-python/models/). |
| `model_settings` | no | Model tuning parameters such as `temperature`, `top_p`, and `tool_choice`. |
| `tools` | no | Tools the agent can call. See [Tools](https://openai.github.io/openai-agents-python/tools/). |
| `mcp_servers` | no | MCP servers that provide MCP-backed tools to the agent. See the [MCP guide](https://openai.github.io/openai-agents-python/mcp/). |
| `mcp_config` | no | Fine-tune how MCP tools are prepared, such as converting their schemas to strict mode and formatting MCP failures. See the [MCP guide](https://openai.github.io/openai-agents-python/mcp/#agent-level-mcp-configuration). |
| `input_guardrails` | no | Guardrails that run on the first user input for this agent chain. See [Guardrails](https://openai.github.io/openai-agents-python/guardrails/). |
| `output_guardrails` | no | Guardrails that run on the final output for this agent. See [Guardrails](https://openai.github.io/openai-agents-python/guardrails/). |
| `output_type` | no | Structured output type instead of plain text. See [Output types](https://openai.github.io/openai-agents-python/agents/#output-types). |
| `hooks` | no | Agent-scoped lifecycle callbacks. See [Lifecycle events (hooks)](https://openai.github.io/openai-agents-python/agents/#lifecycle-events-hooks). |
| `tool_use_behavior` | no | Control whether tool results loop back to the model or end the run. See [Tool use behavior](https://openai.github.io/openai-agents-python/agents/#tool-use-behavior). |
| `reset_tool_choice` | no | Reset `tool_choice` after a tool call (default: `True`) to avoid tool-use loops. See [Forcing tool use](https://openai.github.io/openai-agents-python/agents/#forcing-tool-use). |

```
from agents import Agent
from agents.decorators import tool


@tool
def get_weather(city: str) -> str:
    """returns weather info for the specified city."""
    return f"The weather in {city} is sunny"


agent = Agent(
    name="Haiku agent",
    instructions="Always respond in haiku form",
    model="gpt-5-nano",
    tools=[get_weather],
)
```

Everything in this section applies to `Agent`. `SandboxAgent` builds on the same ideas, then adds `default_manifest`, `base_instructions`, `capabilities`, and `run_as` for workspace-scoped runs. See [Sandbox agent concepts](https://openai.github.io/openai-agents-python/sandbox/guide/).

## Prompt templates

You can reference a prompt template created in the OpenAI platform by setting `prompt`. This works when OpenAI models are accessed through the Responses API.

To use it, please:

1. Go to https://platform.openai.com/playground/prompts
2. Create a new prompt variable, `poem_style`.
3. Create a system prompt with the content:

   ```
   Write a poem in {{poem_style}}
   ```
4. Run the example with the `--prompt-id` flag.

```
from agents import Agent

agent = Agent(
    name="Prompted assistant",
    prompt={
        "id": "pmpt_123",
        "version": "1",
        "variables": {"poem_style": "haiku"},
    },
)
```

You can also generate the prompt dynamically at run time:

```
from dataclasses import dataclass

from agents import Agent, GenerateDynamicPromptData, Runner


@dataclass
class PromptContext:
    prompt_id: str
    poem_style: str


async def build_prompt(data: GenerateDynamicPromptData):
    ctx: PromptContext = data.context.context
    return {
        "id": ctx.prompt_id,
        "version": "1",
        "variables": {"poem_style": ctx.poem_style},
    }


agent = Agent(name="Prompted assistant", prompt=build_prompt)
result = await Runner.run(
    agent,
    "Say hello",
    context=PromptContext(prompt_id="pmpt_123", poem_style="limerick"),
)
```

## Context

Agents are generic on their `context` type. Context is a dependency-injection tool: it's an object you create and pass to `Runner.run()`, that is passed to every agent, tool, handoff etc, and it serves as a grab bag of dependencies and state for the agent run. You can provide any Python object as the context.

Read the [context guide](https://openai.github.io/openai-agents-python/context/) for the full `RunContextWrapper` surface, shared usage tracking, nested `tool_input`, and serialization caveats.

```
from dataclasses import dataclass


@dataclass
class Purchase:
    id: str


@dataclass
class UserContext:
    name: str
    uid: str
    is_pro_user: bool

    async def fetch_purchases(self) -> list[Purchase]:
        # implement your logic here
        return []


agent = Agent[UserContext](
    ...,
)
```

## Output types

By default, agents produce plain text (i.e. `str`) outputs. If you want the agent to produce a particular type of output, you can use the `output_type` parameter. A common choice is to use [Pydantic](https://docs.pydantic.dev/) objects, but we support any type that can be wrapped in a Pydantic [TypeAdapter](https://docs.pydantic.dev/latest/api/type_adapter/) - dataclasses, lists, TypedDict, etc.

```
from pydantic import BaseModel
from agents import Agent


class CalendarEvent(BaseModel):
    name: str
    date: str
    participants: list[str]


agent = Agent(
    name="Calendar extractor",
    instructions="Extract calendar events from text",
    output_type=CalendarEvent,
)
```

Note

When you pass an `output_type`, that tells the model to use [structured outputs](https://platform.openai.com/docs/guides/structured-outputs) instead of regular plain text responses.

## Multi-agent system design patterns

There are many ways to design multi‑agent systems, but we commonly see two broadly applicable patterns:

1. Manager (agents as tools): A central manager/orchestrator invokes specialized sub‑agents as tools and retains control of the conversation.
2. Handoffs: Peer agents hand off control to a specialized agent that takes over the conversation. This is decentralized.

See [our practical guide to building agents](https://cdn.openai.com/business-guides-and-resources/a-practical-guide-to-building-agents.pdf) for more details.

### Manager (agents as tools)

The `customer_facing_agent` handles all user interaction and invokes specialized sub‑agents exposed as tools. Read more in the [tools](https://openai.github.io/openai-agents-python/tools/#agents-as-tools) documentation.

```
from agents import Agent

booking_agent = Agent(...)
refund_agent = Agent(...)

customer_facing_agent = Agent(
    name="Customer-facing agent",
    instructions=(
        "Handle all direct user communication. "
        "Call the relevant tools when specialized expertise is needed."
    ),
    tools=[
        booking_agent.as_tool(
            tool_name="booking_expert",
            tool_description="Handles booking questions and requests.",
        ),
        refund_agent.as_tool(
            tool_name="refund_expert",
            tool_description="Handles refund questions and requests.",
        ),
    ],
)
```

### Handoffs

Configured handoff targets are sub‑agents to which the agent can delegate. When a handoff occurs, the delegated agent receives the conversation history and takes over the conversation. This pattern enables modular, specialized agents that excel at a single task. Read more in the [handoffs](https://openai.github.io/openai-agents-python/handoffs/) documentation.

```
from agents import Agent

booking_agent = Agent(...)
refund_agent = Agent(...)

triage_agent = Agent(
    name="Triage agent",
    instructions=(
        "Help the user with their questions. "
        "If they ask about booking, hand off to the booking agent. "
        "If they ask about refunds, hand off to the refund agent."
    ),
    handoffs=[booking_agent, refund_agent],
)
```

## Dynamic instructions

In most cases, you can provide instructions when you create the agent. However, you can also provide dynamic instructions via a function. The function will receive the agent and context, and must return the prompt. Both regular and `async` functions are accepted.

```
from agents import Agent, RunContextWrapper


def dynamic_instructions(context: RunContextWrapper[UserContext], agent: Agent[UserContext]) -> str:
    return f"The user's name is {context.context.name}. Help them with their questions."


agent = Agent[UserContext](
    name="Triage agent",
    instructions=dynamic_instructions,
)
```

## Lifecycle events (hooks)

Sometimes, you want to observe the lifecycle of an agent. For example, you may want to log events, pre-fetch data, or record usage when certain events occur.

There are two hook scopes:

* [`RunHooks`](https://openai.github.io/openai-agents-python/ref/lifecycle/#agents.lifecycle.RunHooks "RunHooks


    
        module-attribute
    ") observe the entire `Runner.run(...)` invocation, including handoffs to other agents.
* [`AgentHooks`](https://openai.github.io/openai-agents-python/ref/lifecycle/#agents.lifecycle.AgentHooks "AgentHooks


    
        module-attribute
    ") are attached to a specific agent instance via `agent.hooks`.

The callback context also changes depending on the event:

* Agent start/end hooks receive [`AgentHookContext`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.AgentHookContext "AgentHookContext


    
        dataclass
    "), which wraps your original context and carries the shared run usage state.
* LLM, tool, and handoff hooks receive [`RunContextWrapper`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper "RunContextWrapper


    
        dataclass
    ").

Typical hook timing:

* `on_agent_start`: when a specific agent begins running; `on_agent_end`: when that agent finishes producing a final output.
* `on_llm_start` / `on_llm_end`: immediately around each model call.
* `on_tool_start` / `on_tool_end`: around each local tool invocation. For function tools, the hook `context` is typically a `ToolContext`, so you can inspect tool-call metadata such as `tool_call_id`.
* `on_handoff`: when control moves from one agent to another.

Use `RunHooks` when you want a single observer for the whole workflow, and `AgentHooks` when you want lifecycle callbacks scoped to a specific agent.

```
from agents import Agent, RunHooks, Runner


class LoggingHooks(RunHooks):
    async def on_agent_start(self, context, agent):
        print(f"Starting {agent.name}")

    async def on_llm_end(self, context, agent, response):
        print(f"{agent.name} produced {len(response.output)} output items")

    async def on_agent_end(self, context, agent, output):
        print(f"{agent.name} finished with usage: {context.usage}")


agent = Agent(name="Assistant", instructions="Be concise.")
result = await Runner.run(agent, "Explain quines", hooks=LoggingHooks())
print(result.final_output)
```

For the full callback surface, see the [Lifecycle API reference](https://openai.github.io/openai-agents-python/ref/lifecycle/).

## Guardrails

Guardrails allow you to run checks/validations on user input in parallel to the agent running, and on the agent's output once it is produced. For example, you could screen the user's input and agent's output for relevance. Read more in the [guardrails](https://openai.github.io/openai-agents-python/guardrails/) documentation.

## Cloning/copying agents

By using the `clone()` method on an agent, you can duplicate an Agent, and optionally change any properties you like.

```
pirate_agent = Agent(
    name="Pirate",
    instructions="Write like a pirate",
    model="gpt-5.6-sol",
)

robot_agent = pirate_agent.clone(
    name="Robot",
    instructions="Write like a robot",
)
```

`clone()` uses `dataclasses.replace`, so it performs a shallow copy. A list attribute that you do not override, such as `tools`, `handoffs`, `mcp_servers`, `input_guardrails`, or `output_guardrails`, remains the exact list held by the original agent. Mutating that list through either agent therefore affects both agents. To give the clone an independent list container, pass a new list, for example `pirate_agent.clone(tools=[*pirate_agent.tools, extra_tool])`. The entries copied into that new list remain the same tool or handoff objects unless you replace those entries too.

## Forcing tool use

Supplying a list of tools doesn't always mean the LLM will use a tool. You can force tool use by setting [`ModelSettings.tool_choice`](https://openai.github.io/openai-agents-python/ref/model_settings/#agents.model_settings.ModelSettings.tool_choice "tool_choice


  
      class-attribute
      instance-attribute
  "). Valid values are:

1. `auto`, which allows the LLM to decide whether or not to use a tool.
2. `required`, which requires the LLM to use a tool (but it can intelligently decide which tool).
3. `none`, which requires the LLM to *not* use a tool.
4. Setting a specific string e.g. `my_tool`, which requires the LLM to use that specific tool.

When you are using OpenAI Responses tool search, named tool choices are more limited: you cannot target bare namespace names or deferred-only tools with `tool_choice`, and `tool_choice="tool_search"` does not target [`ToolSearchTool`](https://openai.github.io/openai-agents-python/ref/tool/#agents.tool.ToolSearchTool "ToolSearchTool


  
      dataclass
  "). In those cases, prefer `auto` or `required`. See [Hosted tool search](https://openai.github.io/openai-agents-python/tools/#hosted-tool-search) for the Responses-specific constraints.

```
from agents import Agent, ModelSettings
from agents.decorators import tool


@tool
def get_weather(city: str) -> str:
    """Returns weather info for the specified city."""
    return f"The weather in {city} is sunny"


agent = Agent(
    name="Weather Agent",
    instructions="Retrieve weather details.",
    tools=[get_weather],
    model_settings=ModelSettings(tool_choice="get_weather"),
)
```

## Tool use behavior

The `tool_use_behavior` parameter in the `Agent` configuration controls how tool outputs are handled:

* `"run_llm_again"`: The default. Tools are run, and the LLM processes the results to produce a final response.
* `"stop_on_first_tool"`: The output of the first tool call is used as the final response, without further LLM processing.

```
from agents import Agent
from agents.decorators import tool


@tool
def get_weather(city: str) -> str:
    """Returns weather info for the specified city."""
    return f"The weather in {city} is sunny"


agent = Agent(
    name="Weather Agent",
    instructions="Retrieve weather details.",
    tools=[get_weather],
    tool_use_behavior="stop_on_first_tool",
)
```

* `StopAtTools(stop_at_tool_names=[...])`: Stops if any specified tool is called, using its output as the final response.

```
from agents import Agent
from agents.agent import StopAtTools
from agents.decorators import tool


@tool
def get_weather(city: str) -> str:
    """Returns weather info for the specified city."""
    return f"The weather in {city} is sunny"


@tool
def sum_numbers(a: int, b: int) -> int:
    """Adds two numbers."""
    return a + b


agent = Agent(
    name="Stop At Stock Agent",
    instructions="Get weather or sum numbers.",
    tools=[get_weather, sum_numbers],
    tool_use_behavior=StopAtTools(stop_at_tool_names=["get_weather"]),
)
```

* `ToolsToFinalOutputFunction`: A custom function that processes tool results and decides whether to end the run with a final output or continue processing with the LLM.

```
from agents import Agent, FunctionToolResult, RunContextWrapper
from agents.agent import ToolsToFinalOutputResult
from agents.decorators import tool
from typing import List, Any


@tool
def get_weather(city: str) -> str:
    """Returns weather info for the specified city."""
    return f"The weather in {city} is sunny"


def custom_tool_handler(
    context: RunContextWrapper[Any], tool_results: List[FunctionToolResult]
) -> ToolsToFinalOutputResult:
    """Processes tool results to decide final output."""
    for result in tool_results:
        if result.output and "sunny" in result.output:
            return ToolsToFinalOutputResult(
                is_final_output=True, final_output=f"Final weather: {result.output}"
            )
    return ToolsToFinalOutputResult(is_final_output=False, final_output=None)


agent = Agent(
    name="Weather Agent",
    instructions="Retrieve weather details.",
    tools=[get_weather],
    tool_use_behavior=custom_tool_handler,
)
```

Note

To prevent infinite loops, the framework automatically resets `tool_choice` to "auto" after a tool call. This behavior is configurable via [`agent.reset_tool_choice`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.reset_tool_choice "reset_tool_choice


  
      class-attribute
      instance-attribute
  "). The infinite loop is because tool results are sent to the LLM, which then generates another tool call because of `tool_choice`, ad infinitum.
