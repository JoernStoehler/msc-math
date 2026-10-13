# Reading view: openai-agents-sdk-openai-agents-python-repl

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `openai-agents-sdk-openai-agents-python-repl`
- Source URL: https://openai.github.io/openai-agents-python/repl/
- Resolved URL: https://openai.github.io/openai-agents-python/repl/
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:openai-agents-sdk-openai-agents-python-repl -->

# REPL utility

The SDK provides `run_demo_loop` for quick, interactive testing of an agent's behavior directly in your terminal.

```
import asyncio
from agents import Agent, run_demo_loop


async def main() -> None:
    agent = Agent(name="Assistant", instructions="You are a helpful assistant.")
    await run_demo_loop(agent)


if __name__ == "__main__":
    asyncio.run(main())
```

`run_demo_loop` prompts for user input in a loop, keeping the conversation history between turns. By default, it streams model output as it is produced. When you run the example above, run\_demo\_loop starts an interactive chat session. It continuously asks for your input, remembers the entire conversation history between turns (so your agent knows what's been discussed) and automatically streams the agent's responses to you in real-time as they are generated.

To end this chat session, simply type `quit` or `exit` (and press Enter) or use the `Ctrl-D` keyboard shortcut.
