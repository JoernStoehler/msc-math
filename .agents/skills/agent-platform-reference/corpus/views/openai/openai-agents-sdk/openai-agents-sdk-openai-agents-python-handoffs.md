# Reading view: openai-agents-sdk-openai-agents-python-handoffs

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `openai-agents-sdk-openai-agents-python-handoffs`
- Source URL: https://openai.github.io/openai-agents-python/handoffs/
- Resolved URL: https://openai.github.io/openai-agents-python/handoffs/
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:openai-agents-sdk-openai-agents-python-handoffs -->

# Handoffs

Handoffs allow an agent to delegate tasks to another agent. This is particularly useful in scenarios where different agents specialize in distinct areas. For example, a customer support app might have agents that each specifically handle tasks like order status, refunds, FAQs, etc.

Handoffs are represented as tools to the LLM. So if there's a handoff to an agent named `Refund Agent`, the tool would be named `transfer_to_refund_agent`.

## Creating a handoff

All agents have a [`handoffs`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.handoffs "handoffs


  
      class-attribute
      instance-attribute
  ") param, which can either take an `Agent` directly, or a `Handoff` object that customizes the Handoff.

If you pass plain `Agent` instances, their [`handoff_description`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.handoff_description "handoff_description


  
      class-attribute
      instance-attribute
  ") (when set) is appended to the default tool description. Use it to hint when the model should pick that handoff without writing a full `handoff()` object.

You can create a handoff using the [`handoff()`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.handoff "handoff") function provided by the Agents SDK. This function allows you to specify the agent to hand off to, along with optional overrides and input filters.

### Basic usage

Here's how you can create a simple handoff:

```
from agents import Agent, handoff

billing_agent = Agent(name="Billing agent")
refund_agent = Agent(name="Refund agent")

# (1)!
triage_agent = Agent(name="Triage agent", handoffs=[billing_agent, handoff(refund_agent)])
```

1. You can use the agent directly (as in `billing_agent`), or you can use the `handoff()` function.

### Customizing handoffs via the `handoff()` function

The [`handoff()`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.handoff "handoff") function lets you customize things.

* `agent`: This is the agent to which things will be handed off.
* `tool_name_override`: By default, the `Handoff.default_tool_name()` function is used, which resolves to `transfer_to_<agent_name>`. You can override this.
* `tool_description_override`: Override the default tool description from `Handoff.default_tool_description()`
* `on_handoff`: A callback function executed when the handoff is invoked. This is useful for things like kicking off some data fetching as soon as you know a handoff is being invoked. This function receives the agent context, and can optionally also receive LLM generated input. The input data is controlled by the `input_type` param.
* `input_type`: The schema for the handoff tool-call arguments. When set, the parsed payload is passed to `on_handoff`.
* `input_filter`: This lets you filter the input received by the next agent. See below for more.
* `is_enabled`: Whether the handoff is enabled. This can be a boolean or a function that returns a boolean, allowing you to dynamically enable or disable the handoff at runtime.
* `nest_handoff_history`: Optional per-handoff override for the RunConfig-level `nest_handoff_history` setting. If `None`, the value defined in the active run configuration is used instead.

The [`handoff()`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.handoff "handoff") helper always transfers control to the specific `agent` you passed in. If you have multiple possible destinations, register one handoff per destination and let the model choose among them. Use a custom [`Handoff`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff "Handoff


  
      dataclass
  ") only when your own handoff code must decide which agent to return at invocation time.

```
from agents import Agent, handoff, RunContextWrapper


def on_handoff(ctx: RunContextWrapper[None]):
    print("Handoff called")


agent = Agent(name="My agent")

handoff_obj = handoff(
    agent=agent,
    on_handoff=on_handoff,
    tool_name_override="custom_handoff_tool",
    tool_description_override="Custom description",
)
```

## Handoff inputs

In certain situations, you want the LLM to provide some data when it calls a handoff. For example, imagine a handoff to an "Escalation agent". You might want the model to provide a reason so you can log it.

```
from pydantic import BaseModel

from agents import Agent, handoff, RunContextWrapper


class EscalationData(BaseModel):
    reason: str


async def on_handoff(ctx: RunContextWrapper[None], input_data: EscalationData):
    print(f"Escalation agent called with reason: {input_data.reason}")


agent = Agent(name="Escalation agent")

handoff_obj = handoff(
    agent=agent,
    on_handoff=on_handoff,
    input_type=EscalationData,
)
```

`input_type` describes the arguments for the handoff tool call itself. The SDK exposes that schema to the model as the handoff tool's `parameters`, validates the returned JSON locally, and passes the parsed value to `on_handoff`.

`is_enabled` is evaluated while the SDK prepares the available handoffs, before the model returns handoff arguments, so it cannot authorize values inside an argument-bearing handoff. When authorization depends on the parsed fields, perform the check at the start of `on_handoff`, before any application side effects. If authorization fails, raise instead of returning; the SDK continues the transfer after `on_handoff` returns successfully. Tool input guardrails apply to function tools, not handoffs.

It does not replace the next agent's main input, and it does not choose a different destination. The [`handoff()`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.handoff "handoff") helper still transfers to the specific agent you wrapped, and the receiving agent still sees the conversation history unless you change it with an [`input_filter`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.input_filter "input_filter


  
      class-attribute
      instance-attribute
  ") or nested handoff history settings.

`input_type` is also separate from [`RunContextWrapper.context`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.context "context


  
      instance-attribute
  "). Use `input_type` for metadata the model decides at handoff time, not for application state or dependencies you already have locally.

### When to use `input_type`

Use `input_type` when the handoff needs a small piece of model-generated metadata such as `reason`, `language`, `priority`, or `summary`. For example, a triage agent can hand off to a refund agent with `{ "reason": "duplicate_charge", "priority": "high" }`, and `on_handoff` can log or persist that metadata before the refund agent takes over.

Choose a different mechanism when the goal is different:

* Put existing application state and dependencies in [`RunContextWrapper.context`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper.context "context


    
        instance-attribute
    "). See the [context guide](https://openai.github.io/openai-agents-python/context/).
* Use [`input_filter`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.input_filter "input_filter


    
        class-attribute
        instance-attribute
    "), [`RunConfig.nest_handoff_history`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.nest_handoff_history "nest_handoff_history


    
        class-attribute
        instance-attribute
    "), or [`RunConfig.handoff_history_mapper`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.handoff_history_mapper "handoff_history_mapper


    
        class-attribute
        instance-attribute
    ") if you want to change what history the receiving agent sees.
* Register one handoff per destination if there are multiple possible specialists. `input_type` can add metadata to the chosen handoff, but it does not dispatch between destinations.
* If you want structured input for a nested specialist without transferring the conversation, prefer [`Agent.as_tool(parameters=...)`](https://openai.github.io/openai-agents-python/ref/agent/#agents.agent.Agent.as_tool "as_tool"). See [tools](https://openai.github.io/openai-agents-python/tools/#structured-input-for-tool-agents).

## Input filters

When a handoff occurs, it's as though the new agent takes over the conversation, and gets to see the entire previous conversation history. If you want to change this, you can set an [`input_filter`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.input_filter "input_filter


  
      class-attribute
      instance-attribute
  "). An input filter is a function that receives the existing input via a [`HandoffInputData`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.HandoffInputData "HandoffInputData


  
      dataclass
  "), and must return a new `HandoffInputData`.

[`HandoffInputData`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.HandoffInputData "HandoffInputData


  
      dataclass
  ") includes:

* `input_history`: the input history before `Runner.run(...)` started.
* `pre_handoff_items`: items generated before the agent turn where the handoff was invoked.
* `new_items`: items generated during the current turn, including the handoff call and handoff output items.
* `input_items`: optional items to forward to the next agent instead of `new_items`, allowing you to filter model input while keeping `new_items` intact for session history.
* `run_context`: the active [`RunContextWrapper`](https://openai.github.io/openai-agents-python/ref/run_context/#agents.run_context.RunContextWrapper "RunContextWrapper


    
        dataclass
    ") at the time the handoff was invoked.

Nested handoff history is available as an opt-in beta and is disabled by default while we stabilize it. When you enable [`RunConfig.nest_handoff_history`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.nest_handoff_history "nest_handoff_history


  
      class-attribute
      instance-attribute
  "), the runner compacts summarizable history into ordered assistant summary segments while preserving lossless message items in their original positions. Each generated summary segment uses the `<CONVERSATION HISTORY>` wrapper, and later handoffs flatten earlier generated segments before rebuilding the ordered transcript. Sessions, `RunState`, and `RunResult.to_input_list()` track exact message occurrences moved into this SDK-default history so those occurrences are not appended twice; separate identical messages are still preserved. You can provide your own mapping function via [`RunConfig.handoff_history_mapper`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.handoff_history_mapper "handoff_history_mapper


  
      class-attribute
      instance-attribute
  ") to return the exact list of input items for the next agent instead of using the built-in segmentation. The opt-in applies only when neither the handoff's `input_filter` nor the active run's `RunConfig.handoff_input_filter` is set, so existing code that already customizes the payload (including the examples in this repository) keeps its current behavior without changes. You can override the nesting behaviour for a single handoff by passing `nest_handoff_history=True` or `False` to [`handoff(...)`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.handoff "handoff"), which sets [`Handoff.nest_handoff_history`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.nest_handoff_history "nest_handoff_history


  
      class-attribute
      instance-attribute
  "). If you just need to change the wrapper text for generated summary segments, call [`set_conversation_history_wrappers`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.set_conversation_history_wrappers "set_conversation_history_wrappers") before running your agents. Call [`reset_conversation_history_wrappers`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.reset_conversation_history_wrappers "reset_conversation_history_wrappers") before a later run when you need to restore the default wrappers.

Nested handoff history changes how the transcript is represented; it does not redact sensitive data. Tool-call arguments and tool outputs can remain in the generated assistant summary even when the corresponding structured tool items are no longer forwarded separately. Treat the receiving agent and its model provider as recipients of the forwarded history.

For client-managed history, use an explicit [`input_filter`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.input_filter "input_filter


  
      class-attribute
      instance-attribute
  ") or [`RunConfig.handoff_input_filter`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.handoff_input_filter "handoff_input_filter


  
      class-attribute
      instance-attribute
  ") to select or redact the content that the receiving agent may see. If a custom filter also calls `nest_handoff_history`, sanitize `input_history`, `pre_handoff_items`, and `new_items` before that call. The helper builds nested history from those three fields and ignores any existing `input_items` override. Filtering only `input_items` can therefore leave excluded tool content in the generated summary.

If the filter must preserve the original `new_items` for session history, the filter can instead call `nest_handoff_history` and sanitize the returned `input_history` before returning the nested result. Clearing or replacing only `input_items` after nesting does not remove content already included in `input_history`.

Server-managed conversations (`conversation_id`, `previous_response_id`, or `auto_previous_response_id`) do not support handoff input filters; use a separate run with explicitly selected input when the receiving agent must not inherit that server-managed history. Do not reuse the original `conversation_id` or `previous_response_id` in that separate run.

If both the handoff and the active [`RunConfig.handoff_input_filter`](https://openai.github.io/openai-agents-python/ref/run/#agents.run.RunConfig.handoff_input_filter "handoff_input_filter


  
      class-attribute
      instance-attribute
  ") define a filter, the per-handoff [`input_filter`](https://openai.github.io/openai-agents-python/ref/handoffs/#agents.handoffs.Handoff.input_filter "input_filter


  
      class-attribute
      instance-attribute
  ") takes precedence for that specific handoff.

Note

Handoffs stay within a single run. Input guardrails still apply only to the first agent in the chain, and output guardrails only to the agent that produces the final output. Use tool guardrails when you need checks around each custom function-tool call inside the workflow.

There are some common patterns (for example removing all tool calls from the history), which are implemented for you in [`agents.extensions.handoff_filters`](https://openai.github.io/openai-agents-python/ref/extensions/handoff_filters/#agents.extensions.handoff_filters)

```
from agents import Agent, handoff
from agents.extensions import handoff_filters

agent = Agent(name="FAQ agent")

handoff_obj = handoff(
    agent=agent,
    input_filter=handoff_filters.remove_all_tools,  # (1)!
)
```

1. This will automatically remove all tool-related items from the history when `FAQ agent` is called.

`remove_all_tools` removes structured tool items. It does not redact tool arguments or results already copied into ordinary messages or nested-history summaries. Use a custom input filter to remove or redact those message contents when needed.

## Recommended prompts

To make sure that LLMs understand handoffs properly, we recommend including information about handoffs in your agents. We have a suggested prefix in [`agents.extensions.handoff_prompt.RECOMMENDED_PROMPT_PREFIX`](https://openai.github.io/openai-agents-python/ref/extensions/handoff_prompt/#agents.extensions.handoff_prompt.RECOMMENDED_PROMPT_PREFIX "RECOMMENDED_PROMPT_PREFIX


  
      module-attribute
  "), or you can call [`agents.extensions.handoff_prompt.prompt_with_handoff_instructions`](https://openai.github.io/openai-agents-python/ref/extensions/handoff_prompt/#agents.extensions.handoff_prompt.prompt_with_handoff_instructions "prompt_with_handoff_instructions") to automatically add recommended data to your prompts.

```
from agents import Agent
from agents.extensions.handoff_prompt import RECOMMENDED_PROMPT_PREFIX

billing_agent = Agent(
    name="Billing agent",
    instructions=f"""{RECOMMENDED_PROMPT_PREFIX}
    <Fill in the rest of your prompt here>.""",
)
```
