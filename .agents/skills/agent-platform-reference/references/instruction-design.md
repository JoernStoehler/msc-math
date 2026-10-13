# Instruction design: bootstrap, routing and corrective rules

Consult this page when deciding what belongs in AGENTS.md, skill descriptions,
SKILL.md or supporting documentation. It compares relevant guidance rather than
using one discovery guide as a complete design account. Loading mechanics and
behavioral recommendations can coexist in the same guide; keep their claims and
evidence distinguishable. Checked 13 October 2026.

## Jörn's advice, 13 October 2026

**Attribution:** The following is a paraphrase of Jörn's advice in the msc-math
conversation, not a vendor requirement or a measured behavioral result.

- AGENTS.md usually bootstraps each session agent and subagent, and states
  corrective rules for recurring unwanted behavior. Whether particular
  subagents receive it needs checking.
- Include the project, repository navigation, short pointers to relevant files
  that lack skill descriptions, and quick commands demonstrating the stack and
  expected work. Commands can prevent minor mistakes and avoid unnecessary
  specialized skills.
- Skill descriptions play a similar routing role: make the relevant SKILL.md
  discoverable. Avoid duplicating the whole skill catalog in AGENTS.md when its
  descriptions are already available to the consumer.
- A small additional read of relevant materials is often a reasonable cost.
  Independent reads can be batched with skills and project-document reads.
  Jörn regards this overhead as generally small; this is not a measured promise
  about every harness, network request or volume of returned text.
- Jörn's explanation that agents are trained to attend to quick commands is
  his working model, not a training fact established by the consulted guides.

**Use:** Consider this advice when designing guidance for Jörn's workflow.
Preserve the context and distinguish it from explicit instructions. It does not
authorize a mandatory full-repository read, new skills, account changes or
unrelated execution. Optimize useful completed work and total cost together;
byte count and tool-call count alone do not determine quality.

## Compare the relevant sources

| Source | What it contributes | Local mirror |
| --- | --- | --- |
| [AGENTS.md format guide](https://agents.md/) | Agent-facing project context, commands and conventions; nested files for specialized scopes. No universal byte target. | Not in the received corpus; read online 13 October. |
| [OpenAI AGENTS.md guide](https://learn.chatgpt.com/docs/agent-configuration/agents-md) | Discovery, precedence, global/project scopes and combined size limits. Primarily mechanics, not a complete design guide. | `corpus/articles/openai/codex/codex-agents-md.md` |
| [OpenAI: rethinking skills and prompts](https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra) | Contextual pointers, short discriminating descriptions, reassessing accumulated rules and model-specific boundaries. Avoid unconditional prerequisite stacks for tiny changes. | `corpus/articles/openai/codex/codex-prompt-rethink.md` |
| [OpenAI: build skills](https://learn.chatgpt.com/docs/build-skills) | Metadata first, full skill on selection; catalog budgets can shorten or omit entries. | `corpus/articles/openai/chatgpt-work/chatgpt-work-docs-build-skills.md` |
| [Anthropic: memory/instructions](https://code.claude.com/docs/en/memory) | Instruction scope and effective loading; target under 200 lines per CLAUDE.md, and imports still load content. Guidance differs from enforced configuration. | `corpus/articles/anthropic/claude-code/claude-code-memory.md` |
| [Anthropic: best practices](https://code.claude.com/docs/en/best-practices) | Include useful commands, non-default conventions, quirks and gotchas; link detailed material. Remove rules without a useful effect and keep information current. | `corpus/articles/anthropic/claude-code/claude-code-docs-en-best-practices.md` |
| [Anthropic: skills](https://code.claude.com/docs/en/skills) | Description/body separation and supporting files; availability and preload settings vary. | `corpus/articles/anthropic/claude-code/claude-code-skills.md` |

Jörn's project/navigation/command selection and conditional reading model align
with these sources. The guides do not establish that all reads merge
automatically, that their latency/token costs are negligible, or that a model
will obey a rule because it received it. Parallel calls can reduce read latency;
returned text still enters context. Corrective rules need a relevant trigger and
observable effect. For mechanically enforceable boundaries, use supported
permissions, scripts or hooks where appropriate; avoid turning every judgment
into a rigid runtime check.

## Subagents: receipt is surface-specific

- **Codex implementation:** inspected source at commit
  `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d` (rust-v0.162.1) passes inherited
  instruction snapshots to children; their config starts from the parent with
  the effective working directory, and the instruction manager loads project
  instructions. Full-history forks additionally preserve history. See
  [spawn](https://github.com/openai/codex/blob/092d3acd6bec3e3a14bdc7e7a2810ab628ab759d/codex-rs/core/src/agent/control/spawn.rs),
  [child config](https://github.com/openai/codex/blob/092d3acd6bec3e3a14bdc7e7a2810ab628ab759d/codex-rs/core/src/agent/child_config.rs)
  and [instruction manager](https://github.com/openai/codex/blob/092d3acd6bec3e3a14bdc7e7a2810ab628ab759d/codex-rs/core/src/agents_md_manager.rs).
  This is tagged source evidence, not a live assertion about every Codex/Work
  harness or successful adherence.
- **Claude Code:** current [subagent guidance](https://code.claude.com/docs/en/sub-agents#what-loads-at-startup)
  says ordinary children receive the instruction hierarchy, including AGENTS.md
  loaded as project instructions. Explore and Plan skip it; custom definitions
  can opt out through `omitClaudeMd`. Non-fork children do not receive the
  parent's conversation, previously read files or invoked skills. Listed
  preload skills supply their full content; a fork inherits the conversation.
- **Chat/Cowork and other hosted surfaces:** verify the effective harness.
  Claude Code documentation alone does not establish identical startup there.

## Retrieval and evidence limits

The received corpus already contains the vendor guides above. The earlier
msc-math answer consulted too narrow a subset; the format site's absence is a
specific coverage gap, not evidence that all instruction guidance was missing.
The previous skill index routed by product and failure but lacked this focused
design comparison. This page supplies that route; no improved behavior or
complete corpus coverage is claimed.

Search both terminology and decisions: AGENTS.md, CLAUDE.md, instructions,
bootstrap, navigation, progressive disclosure, skills, subagents, hooks and
context. Follow a mechanics question to product docs/source; follow a design
question to the design guides as well. One matched URL or the total source count
does not establish adequate coverage. Expand acquisition only for a consequential
missing source; keep mirrors generated and this interpretation hand-maintained.
