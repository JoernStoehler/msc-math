---
name: openai-sources
description: Find relevant official documentation or implementation evidence for OpenAI products, APIs, models and Codex, including questions about this agent.
---

# OpenAI sources

Use these entry points as needed; indexes are not exhaustive:

- [Developer index](https://developers.openai.com/llms.txt)
- [API index](https://developers.openai.com/api/docs/llms.txt)
- [ChatGPT/Codex index](https://learn.chatgpt.com/docs/llms.txt)
- [Official Codex source and releases](https://github.com/openai/codex)
- Outside those indexes: [Help Center](https://help.openai.com/), including [ChatGPT release notes](https://help.openai.com/en/articles/6825453-chatgpt-release-notes) and [Codex plan/usage guidance](https://help.openai.com/en/articles/11369540-using-codex-with-your-chatgpt-plan).

For unfamiliar or broad questions, consider using a bounded Luna (`gpt-6-luna`) triage delegate. Give it the question and relevant context; ask for the answer, supporting links, necessary excerpts and unresolved gaps. Bring those findings back to the parent instead of loading broad guides there. A known-source question can be answered directly. Independent retrievals can run in parallel.
