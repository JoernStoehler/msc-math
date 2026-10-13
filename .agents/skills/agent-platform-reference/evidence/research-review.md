# Reference research review

**Reviewed:** 2026-10-13. Scope: `references/api-agents.md` (added), `references/chatgpt.md`, `claude-chat.md`, `codex.md`, `claude-code.md`, `custom-instructions.md`, `corrections.md`, and `skill-deployment.md`. I also read `HANDOFF.md`, the Agent Skill root file, `README.md`, and every other reference: `references/INDEX.md`, `acquisition-evidence.md`, `architecture.md`, and `maintenance-workflows.md`. The `harness-engineering-v2` skill was read from `/root/.codex/skills/remote-skills/skill-6aae282713f88191b595a35c6ea29a74/SKILL.md`; its source-ownership, scope, evidence, and uncertainty guidance informed this review.

## Result

The assigned references and new API agent overview distinguish documentation claims from observed runtime behavior, include source URLs and local mirror paths, and avoid account-specific or universal claims where rollout, version, plan, or session scope matters. Corrected old statements that all mirrors were pending; the final manifest count is 636 `ok` and 0 pending/error out of 636 entries. This review cites only source paths represented by acquired entries.

Key local sources reviewed:

- ChatGPT: `corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md`, `chatgpt-projects.md`, `chatgpt-gpts.md`, `chatgpt-memory.md`, `chatgpt-work-codex.md`, `chatgpt-skills.md`, `chatgpt-plugins.md`, `chatgpt-model-selection.md`.
- Claude chat: `corpus/articles/anthropic/claude-chat/claude-personalization.md`, `claude-chat-projects.md`, `claude-chat-cowork-merge.md`, `claude-chat-cloud-cowork.md`, `claude-chat-cowork-computer-use.md`, `claude-chat-skills.md`, `claude-chat-memory.md`, `claude-chat-projects-beta.md`.
- Codex: `corpus/articles/openai/codex/codex-skills.md`, `codex-agents-md.md`, `codex-prompt-rethink.md`; benchmark records `evidence/benchmark-details.json` and `evidence/benchmark-original.tsv`.
- Claude Code: `corpus/articles/anthropic/claude-code/claude-code-memory.md`, `claude-code-skills.md`, `claude-code-debug.md`, `claude-code-index.md`, `claude-code-features.md`, `claude-code-goal.md`.
- API agents/SDKs: `corpus/articles/openai/openai-api/openai-api-api-docs-guides-agents-sdk.md`, `corpus/articles/openai/openai-api/openai-api-api-docs-guides-agents-api-overview.md`, `corpus/articles/openai/openai-api/openai-api-api-docs-guides-agents-api-architecture.md`, `corpus/articles/anthropic/claude-api/claude-api-docs-en-agents-and-tools-tool-use-overview.md`, `corpus/articles/anthropic/claude-api/claude-api-docs-en-agents-and-tools-tool-use-handle-tool-calls.md`, `corpus/articles/anthropic/claude-code/claude-code-docs-en-agent-sdk-quickstart.md`, and `corpus/articles/anthropic/claude-code/claude-code-docs-en-agent-sdk-agent-loop.md`. Shared skill mechanics: `corpus/articles/standard/agent-skills/agent-skills-spec.md` and `corpus/articles/openai/openai-api/openai-api-api-docs-guides-tools-skills.md`.

## Notable corrections and caution

- The previous Codex note said acquisition cost had not been measured. It now reports the benchmark and limitations. The benchmark verification separates 17.494s (archive), 10.983s (shallow clone), and 21.753s (sparse checkout); transfer bytes were not instrumented. The compressed archive is 17,186,781 logical bytes; the original TSV’s 17,186,816 value is its 1-KiB-rounded `du -sk` allocation. The verified archive tree has 89,296,863 regular-file bytes plus 7 symlink-path bytes and 112,951,296 allocated bytes under lstat accounting. Earlier stat-based figures followed the symlink and counted target contents again. These are distinct measurements, not a content mismatch; see `evidence/benchmark-verification.md` for definitions and method-by-method counts.
- Claude Code `AGENTS.md` behavior is documented as version- and configuration-dependent; neither `claude --version` nor an effective `/context` session was available here. The references identify documented thresholds and instruct users to verify.
- ChatGPT and Claude UI/settings instructions are vendor-documented routes only. No account-settings tool was exposed and no authenticated product UI was inspected, so no user setting was changed or account capability verified.
- Custom Instructions text is 1,371 characters by Python `len()` over the paste block, excluding headings; Unicode characters count as Python code points, which may differ from how a product UI counts characters. The vendor Help mirror documents plan-specific caps.
- Source snapshots establish the content retrieved at manifest `last_success_utc`, not ongoing truth. Time-sensitive rollout and plan statements remain explicitly provisional.

## Checks and gaps

- Confirmed assigned reference links resolve to local paths and source URLs represented in the manifest. Final `scripts/validate.py` passed its structural and source-hash checks for all 636 mirrors. The four Codex mirrors resolve to hosted docs, while the repository benchmark is separately tagged; documentation and code provenance remain distinct.
- Inspected relevant excerpts in Custom Instructions, Projects, GPTs, Memory, Claude personalization/Projects/Cowork, Codex Skills/AGENTS.md, Claude Code memory/Skills/debug, Agent Skills spec, and Responses API Skills.
- `python scripts/validate.py`: PASS, 636 sources mirrored, 12 references. `python -m pytest -q tests`: 20 passed. The Custom Instructions paste block is 1,371 characters. No scripts or manifest were edited by this review. No product runtime claims were upgraded based on documentation alone.
- Remaining evidence gaps: no ChatGPT/Claude/Codex product runtime behavior was probed in this review; current plan/account UI availability is unverified; Codex Git wire-transfer bytes are absent. API/SDK claims describe documentation snapshots, and the local corpus has no acquired Anthropic Managed Agents reference. These limits are also stated in the reviewed references where relevant.
