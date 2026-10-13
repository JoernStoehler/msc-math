# Agent product reference index

Source review: 2026-10-13. All selected sources are locally acquired and hash validated. `evidence/inventory.json` records per-product counts, byte totals and duplicate content; `corpus/discovery.json` records index selections and exclusions. Read correction-enriched views under `corpus/views` or compare canonical vendor text under `corpus/articles`. Documentation establishes retrieved statements, not live account or product-runtime behavior.

## Product and harness mechanics

| Product/surface | Read first | Upstream evidence |
| --- | --- | --- |
| ChatGPT ordinary chat / custom instructions / memory | `chatgpt.md`, `corrections.md` | OpenAI Help Center: Custom Instructions, Projects, Memory, GPTs, models |
| ChatGPT Work, Codex remote/local distinction | `chatgpt.md` | OpenAI Help Center: ChatGPT Work and Codex |
| ChatGPT skills/plugins availability | `chatgpt.md`, `skill-deployment.md` | OpenAI Help Center: Skills in ChatGPT; OpenAI Plugins docs |
| Claude chat, global instructions, projects, Cowork | `claude-chat.md` | Anthropic Help: Personalization, projects, new Claude experience |
| Codex CLI, repository and instructions | `codex.md`, `skill-deployment.md` | openai/codex tag and Codex documentation |
| Claude Code sessions, skills, AGENTS.md and CLAUDE.md | `claude-code.md` | code.claude.com/docs/llms.txt + linked Markdown |
| Agent Skills syntax / conventions / limits | `skill-deployment.md` | agentskills.io/specification; OpenAI Skills API |
| OpenAI API / Agents SDK and Claude Agent SDK | `api-agents.md` | Official API and SDK guides in the local corpus |
| Source synchronization and acquisition choices | `maintenance-workflows.md`, `acquisition-evidence.md` | Public endpoints and benchmark commands |
| Custom Instructions ready-to-paste text | `custom-instructions.md` | Grounded in observed user-reported failure modes and current ChatGPT UI constraints |
| Architecture decisions and comparison | `architecture.md` | Product affordances and packaging constraints |

For a specific negative claim, require scope and a recent source. For a live account-specific capability, check the actual account/surface and use effective permission/tool discovery.

## Corrective expertise after failures

| Failure or maintenance task | Consult first |
| --- | --- |
| Move a repo between workstation and cloud or Claude chat/Cowork | `repository-handoff.md` |
| Questions disappear after steering; required answer missing | `user-input-lifecycle.md` |
| Goal activation, non-yielding loops, suppressed completion notifications | `goal-loops.md` |
| Choose between repo guidance, skill, catalog configuration and runtime change | `corrective-maintenance.md` |

For a known case, `python scripts/consult.py handoff`, `goal`, `questions` or `repair` reads just that page. See `REVIEW.md` for task demonstrations and negative results.

## Model behavior, evaluation and workflow design

These consultation routes overlap the mechanics routes above. A guide can mix
loading mechanics with recommendations about model behavior; label the claims
individually. Vendor prompting advice, user observations and causal hypotheses
retain those labels. Costly
model research is not a prerequisite for ordinary documented-mechanics queries.
`harness-engineering` remains the specialized intervention workflow; the pages
below supply evidence and interpretation when it needs them.

| Consultation | Reference |
| --- | --- |
| Research execution bottleneck / delegation/context/configuration | `research-execution.md` |
| Fully traced session-time attribution | `session-traces.md` |
| Model behavior, uncertainty and incremental findings | `model-evidence.md` |
| AGENTS.md/CLAUDE.md design, skill routing, subagent instruction receipt and Jörn's advice | `instruction-design.md` |
