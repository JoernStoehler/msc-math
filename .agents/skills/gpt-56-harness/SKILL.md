---
name: gpt-56-harness
description: Use when designing, editing, reviewing, diagnosing, or evaluating agent-facing instructions for GPT-5.6, including AGENTS.md, repo-local skills, Codex configuration ownership, custom agents, reusable subagent/reviewer prompts, cold-start prompts intended to replace missing session context for a new autonomous session, and model-family migrations. Do not use for ordinary task prompts, ordinary delegation, or domain work merely because Codex performs it.
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
- Keep user/IDE model, effort, verbosity, and concurrency settings in
  `~/.codex/config.toml`. Do not add repo overrides that make later IDE GUI
  writes ineffective.
- Codex already supplies `default`, `worker`, and `explorer`. Add a custom role
  only for a recurring project-specific need.

For current product/model claims, read
`references/official-openai-sources.md` and fetch the live pages. Treat copied
claims as dated caches.

For behavior evaluation, state the prediction and possible side effects, then
choose checks in dependency order. Configuration/discovery failures invalidate
later behavioral probes. Use representative tasks and fresh agents when their
unprimed interpretation is evidence; constrain them to surfaces a real agent
would receive and do not reveal the intended answer. Compare final decisions
and artifacts, not instruction recitation. One successful probe establishes
only that case.

When writing a reusable review or evaluation prompt, name the target and source
material, downstream use or reader, and priority lenses. Priorities are not a
closed finding whitelist unless explicitly stated. When unprimed interpretation
matters, ask what the reviewer actually understood, inferred, missed, or found
ambiguous. Preserve the exact prompt, raw output, evaluation verdict, and
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
