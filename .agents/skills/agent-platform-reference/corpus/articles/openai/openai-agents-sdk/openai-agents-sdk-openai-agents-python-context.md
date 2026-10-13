# Context management

Context is an overloaded term. There are two main classes of context you might care about:

1. Context available locally to your code: this is data and dependencies you might need when tool functions run, during callbacks like `on_handoff`, in lifecycle hooks, etc.
2. Context available to LLMs: this is data the LLM sees when generating a response.

## Local context

This is represented via the [`RunContextWrapper`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper "RunContextWrapper


  
      dataclass
  ") class and the [`context`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.context "context


  
      instance-attribute
  ") property within it. The way this works is:

1. You create any Python object you want. A common pattern is to use a dataclass or a Pydantic object.
2. You pass that object to the various run methods (e.g. `Runner.run(..., context=whatever)`).
3. All your tool calls, lifecycle hooks etc will be passed a wrapper object, `RunContextWrapper[T]`, where `T` represents the type of your context object; the object itself is available via `wrapper.context`.

For some runtime-specific callbacks, the SDK may pass a more specialized subclass of `RunContextWrapper[T]`. For example, lifecycle hooks for `FunctionTool` instances typically receive `ToolContext`, which also exposes tool-call metadata like `tool_call_id`, `tool_name`, and `tool_arguments`.

The **most important** thing to be aware of: every agent, tool function, lifecycle etc for a given agent run must use the same *type* of context.

You can use the context for things like:

* Contextual data for your run (e.g. things like a username/uid or other information about the user)
* Dependencies (e.g. logger objects, data fetchers, etc)
* Helper functions

Note

The context object is **not** sent to the LLM. It is purely a local object that you can read from, write to and call methods on it.

Within a single run, derived wrappers share the same underlying app context, approval state, and usage tracking. Nested [`Agent.as_tool()`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.as_tool "as_tool") runs may attach a different `tool_input`, but they do not get an isolated copy of your app state by default.

### Use local context for capability visibility

When function tools, MCP tools, and handoffs depend on the same request policy, keep the policy inputs or helper on your application context. Each SDK surface exposes the current run context through its own callback:

* [`FunctionTool.is_enabled`](https://openai.github.io/openai-agents-python/ref/tool/#agents.tool.FunctionTool.is_enabled "is_enabled


    
        class-attribute
        instance-attribute
    ") receives a `RunContextWrapper`.
* [`Handoff.is_enabled`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.is_enabled "is_enabled


    
        class-attribute
        instance-attribute
    ") receives a `RunContextWrapper`.
* An MCP [`tool_filter`](https://openai.github.io/openai-agents-python/mcp/#dynamic-tool-filtering) receives a [`ToolFilterContext`](https://openai.github.io/openai-agents-python/ref/mcp/util/#agents.mcp.util.ToolFilterContext "ToolFilterContext


    
        dataclass
    "), whose `run_context` property contains the current `RunContextWrapper`.

Adapt the shared application policy to these callbacks instead of maintaining separate capability lists. The callbacks control which capabilities the SDK exposes for the current run; they cannot authorize a model-generated argument or resource selection. For function tools, enforce those decisions inside the tool implementation or with [tool input guardrails](https://openai.github.io/openai-agents-python/guardrails/#tool-guardrails) and [approvals](https://openai.github.io/openai-agents-python/human_in_the_loop/) when appropriate. MCP servers must authorize their own protected operations. For a handoff with `input_type`, check the parsed input at the start of `on_handoff`, before application side effects, and raise instead of returning when authorization fails. Tool input guardrails do not run for handoffs. See [handoff inputs](https://openai.github.io/openai-agents-python/handoffs/#handoff-inputs) for the callback lifecycle.

### What `RunContextWrapper` exposes

[`RunContextWrapper`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper "RunContextWrapper


  
      dataclass
  ") is a wrapper around your app-defined context object. In practice you will most often use:

* [`wrapper.context`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.context "context


    
        instance-attribute
    ") for your own mutable app state and dependencies.
* [`wrapper.usage`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.usage "usage


    
        class-attribute
        instance-attribute
    ") for aggregated request and token usage across the current run.
* [`wrapper.tool_input`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.tool_input "tool_input


    
        class-attribute
        instance-attribute
    ") for structured input when the current run is executing inside [`Agent.as_tool()`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.as_tool "as_tool").
* [`wrapper.approve_tool(...)`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.approve_tool "approve_tool") / [`wrapper.reject_tool(...)`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.reject_tool "reject_tool") when you need to update approval state programmatically.

Only `wrapper.context` is your app-defined object. The other fields are runtime metadata managed by the SDK.

If you later serialize a [`RunState`](https://openai.github.io/openai-agents-python/ref/run_state/#agents.run_state.RunState "RunState


  
      dataclass
  ") for human-in-the-loop or durable job workflows, that runtime metadata is saved with the state. Avoid putting secrets in [`RunContextWrapper.context`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.context "context


  
      instance-attribute
  ") if you intend to persist or transmit serialized state.

Conversation state is a separate concern. Use `result.to_input_list()`, `session`, `conversation_id`, or `previous_response_id` depending on how you want to carry turns forward. See [results](https://openai.github.io/openai-agents-python/results/), [running agents](https://openai.github.io/openai-agents-python/running_agents/), and [sessions](https://openai.github.io/openai-agents-python/sessions/) for that decision.

```
import asyncio
from dataclasses import dataclass

from agents import Agent, RunContextWrapper, Runner
from agents.decorators import tool


@dataclass
class UserInfo:  # (1)!
    name: str
    uid: int


@tool
async def fetch_user_age(wrapper: RunContextWrapper[UserInfo]) -> str:  # (2)!
    """Fetch the age of the user. Call this function to get user's age information."""
    return f"The user {wrapper.context.name} is 47 years old"


async def main():
    user_info = UserInfo(name="John", uid=123)

    agent = Agent[UserInfo](  # (3)!
        name="Assistant",
        tools=[fetch_user_age],
    )

    result = await Runner.run(  # (4)!
        starting_agent=agent,
        input="What is the age of the user?",
        context=user_info,
    )

    print(result.final_output)  # (5)!
    # The user John is 47 years old.


if __name__ == "__main__":
    asyncio.run(main())
```

1. This is the context object. We've used a dataclass here, but you can use any type.
2. This is a tool. You can see it takes a `RunContextWrapper[UserInfo]`. The tool implementation reads from the context.
3. We mark the agent with the generic `UserInfo`, so that the typechecker can catch errors (for example, if we tried to pass a tool that took a different context type).
4. The context is passed to the `run` function.
5. The agent correctly calls the tool and gets the age.

---

### Advanced: `ToolContext`

In some cases, you might want to access extra metadata about the tool being executed — such as its name, call ID, or raw argument string.  
For this, you can use the [`ToolContext`](https://openai.github.io/openai-agents-python/ref/tool_context/#agents.tool_context.ToolContext "ToolContext


  
      dataclass
  ") class, which extends `RunContextWrapper`.

```
from typing import Annotated
from pydantic import BaseModel, Field
from agents import Agent
from agents.decorators import tool
from agents.tool_context import ToolContext


class WeatherContext(BaseModel):
    user_id: str


class Weather(BaseModel):
    city: str = Field(description="The city name")
    temperature_range: str = Field(description="The temperature range in Celsius")
    conditions: str = Field(description="The weather conditions")


@tool
def get_weather(
    ctx: ToolContext[WeatherContext], city: Annotated[str, "The city to get the weather for"]
) -> Weather:
    print(
        f"[debug] Tool context: (name: {ctx.tool_name}, call_id: {ctx.tool_call_id}, args: {ctx.tool_arguments})"
    )
    return Weather(city=city, temperature_range="14-20C", conditions="Sunny with wind.")


agent = Agent(
    name="Weather Agent",
    instructions="You are a helpful agent that can tell the weather of a given city.",
    tools=[get_weather],
)
```

`ToolContext` provides the same `.context` property as `RunContextWrapper`,  
plus additional fields specific to the current tool call:

* `tool_name` – the name of the tool being invoked
* `tool_call_id` – a unique identifier for this tool call
* `tool_arguments` – the raw argument string passed to the tool
* `tool_namespace` – the Responses namespace for the tool call, when the tool was loaded through `tool_namespace()` or another namespaced surface
* `qualified_tool_name` – the tool name qualified with the namespace when one is available

Use `ToolContext` when you need tool-level metadata during execution.  
For general context sharing between agents and tools, `RunContextWrapper` remains sufficient. Because `ToolContext` extends `RunContextWrapper`, it can also expose `.tool_input` when a nested `Agent.as_tool()` run supplied structured input.

---

## Agent/LLM context

When an LLM is called, the **only** data it can see is from the conversation history. This means that if you want to make some new data available to the LLM, you must do it in a way that makes it available in that history. There are a few ways to do this:

1. You can add it to the Agent `instructions`. This is also known as a "system prompt" or "developer message". System prompts can be static strings, or they can be dynamic functions that receive the context and output a string. This is a common tactic for information that is always useful (for example, the user's name or the current date).
2. Add it to the `input` when calling the `Runner.run` functions. This is similar to the `instructions` tactic, but allows you to have messages that are lower in the [chain of command](https://cdn.openai.com/spec/model-spec-2024-05-08.html#follow-the-chain-of-command).
3. Expose it through `FunctionTool` instances. This is useful for *on-demand* context - the LLM decides when it needs some data, and can call the tool to fetch that data.
4. Use retrieval or web search. These are special tools that are able to fetch relevant data from files or databases (retrieval), or from the web (web search). This is useful for "grounding" the response in relevant contextual data.
