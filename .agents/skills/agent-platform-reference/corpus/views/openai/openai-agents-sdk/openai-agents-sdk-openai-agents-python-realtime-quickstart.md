# Reading view: openai-agents-sdk-openai-agents-python-realtime-quickstart

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `openai-agents-sdk-openai-agents-python-realtime-quickstart`
- Source URL: https://openai.github.io/openai-agents-python/realtime/quickstart/
- Resolved URL: https://openai.github.io/openai-agents-python/realtime/quickstart/
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:openai-agents-sdk-openai-agents-python-realtime-quickstart -->

# Quickstart

Realtime agents in the Python SDK are server-side, low-latency agents built on the OpenAI Realtime API over WebSocket transport.

Python SDK boundary

The Python SDK does **not** provide a browser WebRTC transport. This page only covers Python-managed realtime sessions over server-side WebSockets. Use this SDK for server-side orchestration, tools, approvals, and telephony integrations. See also [Realtime transport](https://openai.github.io/openai-agents-python/realtime/transport/).

## Prerequisites

* Python 3.10 or higher
* OpenAI API key
* Basic familiarity with the OpenAI Agents SDK

## Installation

If you haven't already, install the OpenAI Agents SDK:

```
pip install openai-agents
```

## Create a server-side realtime session

### 1. Import the realtime components

```
import asyncio

from agents.realtime import RealtimeAgent, RealtimeRunner
```

### 2. Define the starting agent

```
agent = RealtimeAgent(
    name="Assistant",
    instructions="You are a helpful voice assistant. Keep responses short and conversational.",
)
```

### 3. Configure the runner

Prefer the nested `audio.input` / `audio.output` session settings shape for new code. For new realtime agents, start with `gpt-realtime-2.1`.

```
runner = RealtimeRunner(
    starting_agent=agent,
    config={
        "model_settings": {
            "model_name": "gpt-realtime-2.1",
            "audio": {
                "input": {
                    "format": "pcm16",
                    "transcription": {"model": "gpt-4o-mini-transcribe"},
                    "turn_detection": {
                        "type": "semantic_vad",
                        "interrupt_response": True,
                    },
                },
                "output": {
                    "format": "pcm16",
                    "voice": "ash",
                },
            },
        }
    },
)
```

### 4. Start the session and send input

`runner.run()` returns a `RealtimeSession`. The connection is opened when you enter the session context.

```
async def main() -> None:
    session = await runner.run()

    async with session:
        await session.send_message("Say hello in one short sentence.")

        async for event in session:
            if event.type == "audio":
                # Forward or play event.audio.data.
                pass
            elif event.type == "history_added":
                print(event.item)
            elif event.type == "agent_end":
                # One assistant turn finished.
                break
            elif event.type == "error":
                print(f"Error: {event.error}")


if __name__ == "__main__":
    asyncio.run(main())
```

`session.send_message()` accepts either a plain string or a structured realtime message. For raw audio chunks, use [`session.send_audio()`](https://openai.github.io/openai-agents-python/ref/realtime/session/#agents.realtime.session.RealtimeSession.send_audio "send_audio


  
      async
  ").

## What this quickstart does not include

* Microphone capture and speaker playback code. See the realtime examples in [`examples/realtime`](https://github.com/openai/openai-agents-python/tree/main/examples/realtime).
* SIP / telephony attach flows. See [Realtime transport](https://openai.github.io/openai-agents-python/realtime/transport/) and the [SIP section](https://openai.github.io/openai-agents-python/realtime/guide/#sip-and-telephony).

## Key settings

Once the basic session works, the settings most people reach for next are:

* `model_name`
* `audio.input.format`, `audio.output.format`
* `audio.input.transcription`
* `audio.input.noise_reduction`
* `audio.input.turn_detection` for automatic turn detection
* `audio.output.voice`
* `tool_choice`, `prompt`, `tracing`
* `async_tool_calls`, `tool_execution.pre_approval_tool_input_guardrails`, `guardrails_settings.debounce_text_length`, `tool_error_formatter`

The older flat aliases such as `input_audio_format`, `output_audio_format`, `input_audio_transcription`, and `turn_detection` still work, but nested `audio` settings are preferred for new code.

For manual turn control, use the low-level `session.update` / `input_audio_buffer.commit` / `response.create` flow described in the [Realtime agents guide](https://openai.github.io/openai-agents-python/realtime/guide/#manual-response-control).

For the full schema, see [`RealtimeRunConfig`](https://openai.github.io/openai-agents-python/ref/realtime/config/#agents.realtime.config.RealtimeRunConfig) and [`RealtimeSessionModelSettings`](https://openai.github.io/openai-agents-python/ref/realtime/config/#agents.realtime.config.RealtimeSessionModelSettings).

## Connection options

Set your API key in the environment:

```
export OPENAI_API_KEY="your-api-key-here"
```

Or pass it directly when starting the session:

```
session = await runner.run(model_config={"api_key": "your-api-key"})
```

`model_config` also supports:

* `url`: Custom WebSocket endpoint
* `headers`: Custom request headers
* `call_id`: Attach to an existing realtime call. In this repo, the documented attach flow is SIP.
* `playback_tracker`: Report how much audio the user has actually heard

If you pass `headers` explicitly, the SDK will **not** inject an `Authorization` header for you.

When connecting to Azure OpenAI, set `model_config["url"]` to a GA Realtime endpoint URL and pass headers explicitly. Avoid the legacy beta path (`/openai/realtime?api-version=...`) with realtime agents. See the [Realtime agents guide](https://openai.github.io/openai-agents-python/realtime/guide/#low-level-access-and-custom-endpoints) for details.

## Next steps

* Read [Realtime transport](https://openai.github.io/openai-agents-python/realtime/transport/) to choose between server-side WebSocket and SIP.
* Read the [Realtime agents guide](https://openai.github.io/openai-agents-python/realtime/guide/) for lifecycle, structured input, approvals, handoffs, guardrails, and low-level control.
* Browse the examples in [`examples/realtime`](https://github.com/openai/openai-agents-python/tree/main/examples/realtime).
