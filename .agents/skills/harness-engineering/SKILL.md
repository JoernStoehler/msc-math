---
name: harness-engineering
description: "Use when changing or evaluating Jörn's durable work harness: Codex source, configuration, tools, hooks, prompts, AGENTS.md, skills, custom agents and reusable prompts; documentation, navigation, repository layout, knowledge placement, helper scripts, APIs, or conventions whose primary objectives include shaping work; explicit Jörn work habits; model migrations; or behavior and cost evaluations. Skill edits also require skill-creator; durable bounded-subagent prompts also require subagent-prompting. Skip ordinary Codex use, one-off prompts or delegation, domain work, and refactoring with no intended work-system effect."
---

# Harness Engineering

The harness is every durable, changeable surface that shapes how Jörn and agents
find, understand, decide, execute, verify, and resume work. Optimize the whole
system for expected thesis success and for context a capable current model or
Jörn cannot otherwise recover cheaply.

Harness surfaces include:

- the Codex product, app-server, configuration, hooks, tools, and runtime;
- model-facing prompts, instructions, `AGENTS.md`, skills, custom roles,
  reusable prompts, and subagent protocols;
- repository information architecture: filenames, folder layout, opening
  paragraphs, maps, indexes, READMEs, knowledge bases, status and evidence
  documents, and other documentation when they shape discovery or
  interpretation;
- APIs, code organization, style, and simplification when they change how
  reliably or cheaply humans or agents can work;
- deterministic scripts, checks, templates, and retained evaluation evidence;
- explicit aids for Jörn's own behavior and habits, such as `JOERN.md`.

An artifact can be domain content, harness, or both. Classify it by its intended
effects rather than by filename or format. When the work also produces or
judges domain content, use the matching domain skill as well; this skill owns
the work-system intervention, not the domain judgment.

## Establish the active surface

1. State the desired change in work, the actors and work stages it should
   affect, and the surface through which it could do so. For Codex product work,
   identify the affected runtime surface (core/app-server) and any affected
   client: Codex TUI, ChatGPT Desktop, Paseo, or several of them.
2. Inspect the active owner files, applicable instruction and configuration
   layers, skill and custom-agent discovery, hooks and trust state, installed
   build, relevant Git history, and focused session evidence. Do not print
   credentials.
3. For a Codex upgrade or current product claim, inspect the installed state
   with `codex --version` and the relevant `codex doctor`, `codex debug`,
   `codex features`, or app-server schema command. Compare against the live
   schema/manual and the source revision matching the build. Read
   `references/official-openai-sources.md`; use `$openai-docs` when current
   OpenAI documentation affects the decision. Before changing `../codex`, read
   its applicable `AGENTS.md`, `LOCAL_DEVELOPMENT.md`, and focused Git history.
4. Distinguish upstream or accepted baseline, currently active local behavior,
   proposed behavior, observed evidence, and inference. A lexical miss or a
   remembered default is not evidence that a surface does not exist.

For local Codex session logs, use `$codex-session-log-parsing` when it is
available. The pre-GPT-5.6 harness at commit
`c457c78efdc340d8838f39274b387201f0ba8e04` and old logs preserve historical
failure controls and capability priors, not current policy.

## Choose the intervention layer

Choose the smallest reversible intervention with sufficient reach and the
lowest expected total cost, including implementation, maintenance, context,
attention, repeated-work, and rollback costs. Use the narrowest durable owner
matching that reach:

- A supported hook or small external supervisor owns event reactions it can
  express cleanly through stable events or APIs.
- Codex source is an eligible harness layer for core/app-server behavior, tool
  schemas, and defaults that should apply independently of Jörn's local
  configuration. Do not treat it as immutable, but do not patch it when a
  prompt, configuration, hook, or script expresses the same intended scope.
- Tracked project `.codex/config.toml` owns deliberate msc-math runtime policy;
  user config owns genuinely cross-repository or machine-local state such as
  trust bootstrap, UI/history preferences, trusted-hook state, and absolute
  machine paths.
- `AGENTS.md` owns project-wide invariants; skills own conditional workflows;
  routed references own detail; custom agents own recurring role postures; and
  topic-specific facts belong beside their source owner.
- Navigation and documentation own discoverability, vocabulary, evidence
  boundaries, and costly orientation when those are the intended intervention.
  Prefer improving the source or its local explanation over adding another
  overlapping map.
- Repository layout, filenames, APIs, and code simplification own recurring
  friction created by the territory itself rather than by a missing
  instruction.
- Scripts belong to the instruction package whose procedure consumes them, or
  to repository `scripts/` when they are general deterministic helpers.
- A glanceable human aid such as `JOERN.md` owns an explicitly chosen Jörn habit;
  do not disguise stakeholder preference or private expertise as agent policy.

Do not duplicate a setting or instruction across layers without documenting the
intended precedence. Add a custom role only for a recurring posture that the
built-in roles and a bounded prompt do not supply reliably. A reference is not
active merely because it exists.

Classify mixed material block by block. Preserve project facts, source maps,
accepted architecture, user stories, and quality decompositions independently
of model-contingent controls, generic advice, and history.

When reviewing a reusable or cold-start prompt, keep accepted constraints,
mutable observations, provisional diagnoses, and proposed strategy
distinguishable wherever conflating them could suppress useful reassessment.

## Review before applying

Draft candidates in `/tmp` and inspect read-only surfaces without approval. Do
not apply changes to prompt-bearing harness material until Jörn has reviewed
the exact candidate.

When prompt-bearing material has distinct meaningful base, current, and
proposed versions, default to a literal three-way rendering of each changed
hunk. Use these markers in order: `<<<<<<< CURRENT`, `||||||| BASE`, `=======`,
and `>>>>>>> PROPOSED`; place the corresponding exact text between them. Use
another exact review view when it is materially clearer or cheaper.

State the identity of all versions and which is active. Never replace exact
text with a conceptual summary. If no distinct baseline exists, show an exact
ordinary diff instead of inventing one. If the candidate changes after
approval, present the changed candidate again.

Jörn's approval of an exact candidate authorizes applying and committing it.
Complete any integration required by the applicable repository instructions
without inventing another approval gate. Ask again only if Jörn limits the
approval, assigns integration elsewhere, or the candidate changes. Preserve
unrelated changes.

## Validate and evaluate

Validate syntax, links, discovery, trigger boundaries, configuration
precedence, and fresh-process effective behavior in proportion to risk. A
configuration or discovery failure invalidates later behavioral probes.

For behavior evaluation, state the prediction, plausible side effects, smallest
discriminating observation, and stopping rule. Use representative tasks and
fresh agents when their unprimed interpretation is evidence; expose only
surfaces a real agent would receive and do not reveal the intended answer.
Compare decisions and artifacts rather than instruction recitation. Preserve
the exact prompt, raw output, evaluation verdict, and designer interpretation
as separate layers. One successful probe establishes only that case.

For a reusable bounded-subagent prompt, also use `$subagent-prompting`. This
skill owns durable placement, discovery, integration, and evaluation;
`$subagent-prompting` owns the assignment model, fresh-recipient contract,
prompt artifact, and return contract. Also use it whenever harness evaluation
delegates consequential bounded review or production work, even if the prompt
is one-off. When creating or updating any skill, also use `$skill-creator`.

For a static model catalog, treat it as a full replacement rather than an
overlay. Generate it from current effective remote metadata, record provenance
and the exact transformation, validate the round trip, and reassess it after
Codex or catalog changes. For Jörn's Luna V2 override, run
`scripts/generate_luna_v2_catalog.py`.

Report the changed surface, affected clients, validation, unresolved evidence,
rollback route, and any decision Jörn must make. Separate current upstream facts
from Jörn's local policy.
