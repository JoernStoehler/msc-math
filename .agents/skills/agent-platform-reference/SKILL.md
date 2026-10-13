---
name: agent-platform-reference
description: Technical reference and corrective expertise for OpenAI/Anthropic products, model behavior and agent harnesses. Use to diagnose platform failures, repository handoffs, instructions/skills, user questions, goal loops, research execution bottlenecks or traced-session behavior, and choose source-grounded maintained fixes. Keep measured behavior and hypotheses distinct.
---

# Agent-platform reference

This skill is a **local-first operational reference**, not an instruction to replace testing with documentation. For a known failure, go directly to the relevant reference below, or run `python scripts/consult.py handoff|goal|questions|repair` with one topic. Read [references/INDEX.md](references/INDEX.md) only when the product or owning reference is unclear. Consult its operational notes and corrections before using a vendor claim as guidance.

## After a failure

Recover the actual task, failed action, effective product/surface/version, relevant configuration and user cost before choosing a fix. Distinguish the user's reported observation from independent reproduction. Consult one relevant reference first, then primary docs or tagged source for the disputed mechanism. Don't mistake passing storage tests for successful agent behavior.

- Repository handoff or machine-dependent scripts: read [repository-handoff.md](references/repository-handoff.md).
- Questions lost during steering or a goal loop that won't yield: read [user-input-lifecycle.md](references/user-input-lifecycle.md) and [goal-loops.md](references/goal-loops.md).
- Slow research, context pressure or missing checks: read [research-execution.md](references/research-execution.md); for measured runtime attribution use [session-traces.md](references/session-traces.md).
- Model-behavior claims and our evolving observations: read [model-evidence.md](references/model-evidence.md); keep reports, executed findings and hypotheses distinct.
- Choosing an instruction/configuration intervention: read [corrective-maintenance.md](references/corrective-maintenance.md).

Report the mechanism, evidence, proposed owning surface, expected benefit and remaining uncertainty. Cite source paths actually inspected; label paraphrases and copy any claimed quote exactly. When authorized to implement, change that owner, test a representative continuation, and preserve the user's objective. Keep activation separate from a proposed correction; don't silently alter live settings or enable goal loops.

## For product questions

1. Disambiguate product and surface (ChatGPT ordinary chat vs Work vs Codex; Claude chat/Cowork vs Claude Code CLI/Desktop/cloud vs Agent SDK; API-hosted vs self-hosted). Do not assume features transfer between surfaces.
2. Search the relevant `references/*.md` first. If the task environment has shell access and the downloaded corpus exists, search correction-enriched `corpus/views` with `rg -n -i 'TERM' references corpus/views`; use `corpus/articles` for normalized vendor text and `corpus/snapshots` for immutable response bytes.
3. Follow source URLs and version/date annotations. Documentation text is evidence, not automatically a specification of current runtime behavior.
4. Consult the relevant correction in [references/corrections.md](references/corrections.md) before asserting a contested limitation or precedence rule; loading the full register is unnecessary when the focused reference already includes it. Use an executable probe for implementation-dependent or contested behavior where feasible.
5. Refresh or browse when material is missing, outside its version/plan scope, superseded, or plausibly changed. Never claim stale local material is sufficient simply to avoid a web request.

## For maintenance tasks

Follow [references/maintenance-workflows.md](references/maintenance-workflows.md). This contains actual fetch/convert/validate commands. Keep generated vendor mirrors separate from hand-edited corrections.

## Deployment caveat

Ordinary ChatGPT conversations do **not** thereby gain access to arbitrary local files or a shell. Use ChatGPT project sources, an eligible installed ChatGPT Skill/plugin, or Work with permitted file access. Global Custom Instructions affect behavior but are not a file retrieval or tool installation mechanism; see [references/chatgpt.md](references/chatgpt.md).
