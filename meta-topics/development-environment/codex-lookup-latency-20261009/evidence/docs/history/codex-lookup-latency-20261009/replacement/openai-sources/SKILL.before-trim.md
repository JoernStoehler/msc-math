---
name: openai-sources
description: Find authoritative sources for OpenAI products, APIs, models, and Codex operation, setup or troubleshooting. Use for questions about this agent when they concern Codex. Covers documentation lookup and implementation evidence; generic software work merely mentioning OpenAI does not need this skill.
---

# OpenAI sources

Find the evidence the actual question needs and carry the parent task forward. Source authority and execution order are separate: run independent retrievals in parallel; sequence work when an actual dependency requires it. Reuse adequate session evidence rather than fetching it again.

## Source triage without flooding the requesting thread

For unfamiliar or broad topics, prefer a bounded **Luna** triage delegate (`gpt-6-luna`) when delegation is authorized and model selection is available. Give it the exact question, relevant context, known sources, owned output and stopping point; do not send unrelated conversation history. Ask it to locate and inspect relevant sources, returning the answer or source route, supporting links, necessary excerpts, version/date limits and unresolved gaps. Bring only what matters back to the parent. Triage does not transfer the parent's responsibility for consequential reasoning or verification.

A narrow question with a known source can be answered directly; do not add a delegation round merely to use Luna. If Luna or delegation is unavailable, use bounded direct retrieval and state a material limitation. Never silently substitute another model for an explicitly requested model.

## Choose evidence by the claim

- **Public product, API, model or pricing claims:** inspect current official documentation or Help Center pages. Start with a relevant index when the page is unknown; search exact official topics when an index omits them. Open the supporting page, not just a search snippet. Preserve the user's selected model.
- **Local Codex operation or debugging:** inspect relevant local official implementation source or direct runtime evidence. Use the [official Codex repository](https://github.com/openai/codex) and [releases](https://github.com/openai/codex/releases) when needed. Match the installed revision when the answer depends on it; reading source does not verify live behavior. Documentation can supply product intent or fill gaps without being a prerequisite to independent source inspection.
- **Private account or workspace state:** use the authorized live connector, configuration or runtime. Public pages cannot establish the user's access, billing balance or private data. Documentation lookup does not authorize account writes or API execution.

## Discovery entry points

- [Developer hub index](https://developers.openai.com/llms.txt): routes API, Codex, Apps/Plugins, Agents and other developer topics.
- [API index](https://developers.openai.com/api/docs/llms.txt): API guides and reference.
- [ChatGPT/Codex index](https://learn.chatgpt.com/docs/llms.txt): product and Codex documentation.
- [Help Center](https://help.openai.com/): product and account guidance outside those indexes, including [ChatGPT release notes](https://help.openai.com/en/articles/6825453-chatgpt-release-notes) and [Codex with a ChatGPT plan](https://help.openai.com/en/articles/11369540-using-codex-with-your-chatgpt-plan).

Indexes are discovery aids, not exhaustive inventories. Follow redirects and inspect dates; omitted pages are not evidence that a capability is absent. Search official sites for uncovered topics. Do not load a full documentation export or broad manual merely because it exists.

## Return and stop

Return the supported answer with precise sources and material caveats. For parent handoffs, include only evidence and uncertainty needed for its next decision or action. Stop retrieval when the question is adequately supported; do further research for a specific unresolved claim. If evidence is missing or conflicting, say what remains unknown rather than converting silence in documentation into a product limitation.
