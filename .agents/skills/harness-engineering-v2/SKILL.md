---
name: harness-engineering-v2
description: Design, revise, supersede, and evaluate Codex harness guidance. Use for AGENTS.md files, skills, prompts, agent definitions, or other maintained interventions intended to change future agent behavior; do not use for ordinary domain writing or application implementation.
---

# Harness engineering

## Intended effect and evaluation

**Outcome:** Improve agents' expected ability to complete intended work well by making the right knowledge, capabilities, and decision guidance available when needed.
**Evaluation:** Prefer the intervention with the greatest expected net benefit across result quality, reliability, Jörn's effort, context and tool costs, latency, maintenance burden, and harmful side effects. Smaller is preferable when the relevant benefits are otherwise similar; it is not the primary objective.
**Rule:** Preserve explicit user choices, scope, and existing authority boundaries. Harness guidance must not silently choose a different product, expand an assignment, mutate unrelated configuration, or treat approval for one action as authority for others.

## Write guidance Jörn can maintain

### Put review attention where it matters

**Suggestion:** Order sections, and items within them, by importance to Jörn's likely review and decisions. Put consequential outcomes, risks, uncertain diagnoses, and disputed choices before routine mechanics and reference catalogs, except when a dependency must be understood first.
**Reason:** Review attention is limited, and early items are easier to notice and challenge.
**Does not mean:** ordering has a proven effect on agent behavior. Treat that as unknown unless there is relevant evidence.

### Make the structure legible

**Suggestion:** Use headings for real nested topics. Within a topic, use a small set of bold inline labels such as **Rule**, **Suggestion**, **Reason**, **Prevents**, **Does not mean**, **Clarifying context**, **Example**, and **Completion** when the label changes how a statement should be interpreted.
**Prevents:** rules and suggestions being confused, relevant context becoming orphaned, and maintainers having to infer relationships from proximity or formatting.
**Does not mean:** use every label, force every topic into one schema, flatten meaningful hierarchy into a wall of labeled paragraphs, repeat facts under several labels, or add labels that do not help interpretation.

**Rule:** Keep related labeled paragraphs adjacent with no blank lines. Use one blank line between distinct arguments. Preserve the blank lines needed around headings, lists, tables, examples, and code blocks.
**Prevents:** related meaning looking fragmented and distinct arguments looking accidentally connected.

### Preserve meaning while rewriting

**Rule:** Before a substantial rewrite, recover the governing outcomes, accepted user choices, concrete failures or risks, reasons, boundaries, uncertainty, discovery path, and maintained callers. Preserve enough of each non-obvious constraint for a later maintainer to decide whether it still applies.
**Suggestion:** Use Git as the recoverable baseline and keep unsettled drafts outside loaded instruction paths. Compare the source and candidate before activation. Account for meaningful content that was removed, merged, weakened, or generalized; a sentence-by-sentence ledger is unnecessary when the disposition is clear.
**Rule:** Keep semantic revision distinct from activation, caller migration, distribution, and deletion. Obtain any review or approval required by the owner before those later actions.
**Prevents:** information being silently lost during cleanup, tentative drafts changing live behavior, and approval of prose being mistaken for approval of rollout.

### Keep future maintenance cheap

**Suggestion:** Prefer plain language, stable concepts, local grouping, and links to canonical owners. Route time-sensitive product knowledge to current authoritative documentation instead of copying volatile details without need.
**Rule:** Prune, merge, rewrite, or remove guidance when that improves clarity, while preserving the intent, concrete mistake or risk, and boundary that justify any remaining constraint.
**Prevents:** successive corrections accumulating into broad, overlapping, contradictory, or orphaned rituals.

## Choose the intervention

### Establish what should change

**Rule:** Establish the behavior or decision to affect, the evidence for it, the intended audience, and the narrowest scope in which the guidance is true before choosing a surface. Distinguish accepted requirements and observed facts from provisional diagnoses, preferences, and proposed strategies when confusing them could constrain later work.
**Prevents:** a vivid example, isolated failure, or tentative explanation becoming a universal ritual.
**Does not mean:** every useful capability needs a prior incident. A realistic intended request and a clear expected benefit can justify guidance.

**Rule:** Assume the agent is capable. Include guidance that changes decisions, supplies unavailable context, establishes an invariant, or defines observable completion. Match specificity to the cost of deviation: state outcomes and criteria when several approaches work; prescribe exact steps or parameters when variation creates a concrete failure.
**Prevents:** generic advice consuming context, brittle procedures replacing judgment, and examples becoming accidental mandates.

### Select the owning surface

**Rule:** Put guidance where its consumers encounter it:

- scoped `AGENTS.md` for instructions that should apply throughout that directory scope;
- a skill for conditional, reusable expertise or workflow;
- a prompt for one run or reusable user-invoked starting text;
- an agent definition for one durable role;
- a script for repeated deterministic mechanics;
- references for substantial detail needed only in some cases;
- assets for material copied or transformed into outputs;
- domain documentation or code when the knowledge is product truth rather than agent process.

**Suggestion:** Prefer the nearest durable owner. Promote guidance only when its consumers and truth genuinely span the wider scope.
**Prevents:** irrelevant instructions loading into ordinary work, competing owners, and project-local knowledge silently becoming universal policy.

## Know how Codex loads guidance

### `AGENTS.md`

**Technical context:** Codex loads instruction files before doing work, normally once for a launched session. It first looks in `CODEX_HOME` for `AGENTS.override.md`, then `AGENTS.md`, and uses the first non-empty file. For project guidance it walks from the project root to the current working directory, checking `AGENTS.override.md`, `AGENTS.md`, then configured fallback names in each directory. It includes at most one file per directory; nearer files load later and override broader guidance.
**Technical context:** Empty files are skipped. The combined project-instruction limit defaults to 32 KiB. Changing a loaded file does not imply that an already-running session has reloaded it.
**Design consequence:** Put broad invariants high and narrow exceptions near their consumers. Avoid duplicating the same rule at several levels merely for visibility.

### Skills

**Technical context:** Codex initially exposes each discovered skill's name, description, and path, then reads `SKILL.md` when the skill is selected, and reads supporting resources only as needed. Repository skills are discovered from `.agents/skills` directories between the current working directory and repository root. Same-named skills are separate discoverable entries; they do not merge.
**Technical context:** The initial skill catalog has a context budget, so long descriptions may be shortened and some skills may be omitted when the catalog is large. Automatic invocation is the default unless supported metadata changes it.
**Design consequence:** Make the description discriminating: state the capability, when it applies, and only exclusions needed to prevent likely misrouting. Keep normally co-needed guidance together; move substantial conditional detail into references only when progressive loading has a real benefit.

### Prompts and agent definitions

**Technical context:** A prompt or agent definition affects a run because it is selected or passed to that run; merely storing the file does not activate it. Inherited context, available tools, model choice, reasoning controls, and environment are separate concerns. Prompt wording must not imply that it changed a control the execution surface did not provide.

## Read relevant external guidance

**Rule:** When current product, model, or harness behavior could change the design, read the relevant current source before finalizing the intervention. Reuse a source already read in the current work when it remains adequate. These routes are selected by relevance, not a checklist that must be opened for every edit.

### OpenAI: authoritative for Codex mechanics

- [Build with agent skills](https://learn.chatgpt.com/docs/build-skills.md)
- [Custom instructions with AGENTS.md](https://learn.chatgpt.com/docs/agent-configuration/agents-md.md)
- [GPT-5.6 prompting guide](https://developers.openai.com/api/docs/guides/prompt-guidance-gpt-5p6.md)
- [GPT-6 Astra guide](https://developers.openai.com/api/docs/guides/latest-model/gpt-6-astra.md)
- [Evaluation best practices](https://developers.openai.com/api/docs/guides/evaluation-best-practices.md)

### Anthropic: useful cross-vendor design evidence

- [Building effective agents](https://www.anthropic.com/engineering/building-effective-agents)
- [Effective context engineering for AI agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents)
- [Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- [Equipping agents for the real world with agent skills](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills)
- [Writing tools for agents](https://www.anthropic.com/engineering/writing-tools-for-agents)
- [Demystifying evals for AI agents](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents)

**Boundary:** Treat OpenAI documentation as the owner of Codex product mechanics. Treat cross-vendor material as design evidence, not proof of Codex behavior. Preserve an explicitly selected model, and distinguish model selection, reasoning controls, and product execution modes.

## Create or revise the artifact

### Skill packages

**Rule:** Every skill needs `SKILL.md` with YAML frontmatter containing `name` and `description`. Use lowercase letters, digits, and hyphens for names, keep names under 64 characters, and match the folder name. Add `agents/openai.yaml`, `scripts/`, `references/`, or `assets/` only for an actual consumer: optional interface or invocation metadata, reusable executable mechanics, conditional detail, or output material.
**Suggestion:** Keep guidance normally needed together in one `SKILL.md`. Split material when it has a distinct consumer or lifecycle, or when progressive loading meaningfully reduces context cost without hiding required guidance.
**Rule:** When creating a skill, use the bundled initializer when available, request only needed resource directories, replace scaffold placeholders, and preserve supported metadata when revising an existing package. Automatic invocation is the default; change it only when explicitly intended. Preserve unrelated policy or dependency fields in retained metadata.

### Prompts and agent definitions

**Rule:** Start with the least text that preserves the outcome, necessary evidence and context, constraints, autonomy boundaries, completion bar, and required return. Add model-specific guidance only where current official guidance or representative behavior shows it helps.
**Suggestion:** State facts, user choices, diagnoses, hypotheses, and recommendations distinctly when their status affects the recipient's decisions. Include exact mechanics only where they are not reliably discoverable or where variation is costly.

## Validate, evaluate, and supersede

### Check structure and discovery

**Rule:** Validate a completed skill with:

```bash
python3 "${CODEX_HOME:-$HOME/.codex}/skills/.system/skill-creator/scripts/quick_validate.py" <skill-directory>
```

Inspect discovery wording, callers, links, unfinished placeholders, invocation metadata, and changed scripts. Structural validation does not establish useful behavior.

### Test the intended behavior

**Suggestion:** Use realistic independent forward tests when behavioral uncertainty and consequence justify their cost. Test the surfaces a real recipient receives and evaluate observable decisions or artifacts, including important nearby or negative cases when useful.
**Rule:** Keep distinct: non-reproduction of the original incident, syntax validity, owner approval, and evidence that behavior improved. None proves all the others.

### Supersede or distribute deliberately

**Rule:** Before superseding or distributing guidance, inspect every maintained discovery scope and caller that can reach it. Migrate or deliberately retain consumers, remove or disable obsolete owners when authorized, and do not distribute a candidate its owner has marked as unsettled or unapproved.
**Does not mean:** same-named repository copies must converge. Inspect their contracts and ownership separately.
**Completion:** The intervention has positive expected net benefit; the owning surface is discoverable at the intended scope; required owner review is complete; meaningful source content has been preserved or deliberately retired; obsolete callers are migrated or explicitly retained; syntax, links, and executable resources validate; and the handoff distinguishes structural checks from behavioral evidence.
