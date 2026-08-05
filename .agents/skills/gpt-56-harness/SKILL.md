---
name: gpt-56-harness
description: "Use for durable GPT-5.6 harness work: AGENTS.md, repo-local skills, Codex configuration or agents, reusable or cold-start prompts, model migrations, and behavior evaluation. Skill edits also require skill-creator and current official guidance; durable bounded-subagent prompts also require subagent-prompting. Skip one-off prompts, ordinary delegation, and domain work."
---

# GPT-5.6 Harness

Design the active harness around expected thesis success and the context a
capable current model cannot otherwise recover cheaply.

- Inspect the active surface, relevant owner files, Git history, and focused
  session evidence before inferring why material exists.
- Classify mixed material block by block. Separate project/external facts,
  source maps, accepted architecture, user stories, and quality decompositions
  from model-contingent behavior controls, generic advice, and history.
- Put project-wide invariants in `AGENTS.md`, conditional task knowledge in
  skills, details in routed references, and topic-specific knowledge beside its
  owner. A reference is not active merely because it exists.
- Keep deliberate msc-math model, effort, permission, feature, agent, memory,
  shell-policy, and project-specific integration choices in tracked
  `.codex/config.toml`. Keep only machine/user state in
  `~/.codex/config.toml`, including trust bootstrap, UI/history preferences,
  trusted-hook state, other-project trust, and absolute machine paths. Do not
  duplicate a setting across layers without documenting the intended
  precedence.
- Codex already supplies `default`, `worker`, and `explorer`. Add a custom role
  only for a recurring project-specific need.

When creating or updating a skill, also use `$skill-creator`.

For current OpenAI/Codex product, model, configuration, agent, skill-discovery,
or prompting claims, also use `$openai-docs`, read
`references/official-openai-sources.md`, and fetch the live pages. Treat copied
claims as dated caches.

For behavior evaluation, state the prediction and possible side effects, then
choose checks in dependency order. Configuration/discovery failures invalidate
later behavioral probes. Use representative tasks and fresh agents when their
unprimed interpretation is evidence; constrain them to surfaces a real agent
would receive and do not reveal the intended answer. Compare final decisions
and artifacts, not instruction recitation. One successful probe establishes
only that case.

For a reusable bounded-subagent prompt, use both skills:
`$gpt-56-harness` owns durable placement, discovery, integration, and behavior
evaluation; `$subagent-prompting` owns the assignment model, fresh-recipient
contract, prompt artifact, and return contract. When a harness evaluation merely
delegates bounded review or production work, use `$subagent-prompting` for that
assignment. Preserve the exact prompt, raw output, evaluation verdict, and
designer interpretation as separate layers when they guide a durable harness
change.

When reviewing a reusable or cold-start prompt, check that accepted constraints,
mutable observations, provisional diagnoses, and proposed strategy remain
distinguishable where conflating them could suppress useful reassessment.

An explicit harness task authorizes worktree edits and commits. Jörn's approval
is required before a harness commit reaches Main; a worktree commit is not that
approval marker. Validate syntax, links, discovery, trigger boundaries, and
representative behavior in proportion to risk.

The pre-GPT-5.6 harness at commit `c457c78efdc340d8838f39274b387201f0ba8e04`
and local Codex session logs preserve GPT-5.5 failure controls and capability
priors. If GPT-5.6 repeats a behavior, inspect that evidence before adding a new
rule; do not treat it as current policy by default.
