---
name: harness-engineering
description: Design, revise, supersede, and evaluate Codex harness guidance. Use for AGENTS.md files, skills, prompts, agent definitions, or other maintained interventions intended to change future agent behavior; do not use for ordinary domain writing or application implementation.
metadata:
  version: "2"
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

### Establish useful behavior before optimizing its cost

**Suggestion:** For a new related workflow, start with one complete, well-sectioned skill. Establish that its content and organization support useful results before splitting runtime guidance into separate skills or references to reduce loading cost. Distinct section topics do not by themselves require distinct files; do not maintain parallel section and file trees without a useful consumer need.
**Reason:** A useful baseline makes it easier to preserve quality while reducing cost. Starting from an aggressively reduced context can make missing knowledge and lost capability harder to identify and restore. This is a design heuristic, not proof that a single file is always best.

**Rule:** Assess result quality and costs together, but do not accept a substantial loss of useful results for modest savings in context, calls or maintenance. A large saving or hard resource constraint can justify a meaningful tradeoff; state that tradeoff rather than treating less text or fewer calls as improvement by itself.
**Suggestion:** Once there is a useful baseline, compare concrete section/file/workflow decompositions against representative uses. Consider what the recipient must read and do, missed selection or reference hops, irrelevant loading, duplicated or scattered knowledge, and changes a maintainer must coordinate. Use expert judgment or bounded probes when they can materially change the choice.
**Rule:** Structural validity and technical accuracy do not establish low usage cost, low maintenance cost, good behavior or proximity to an optimum. Report which of those a review actually assessed and what remains unknown.
**Does not mean:** freeze cost improvements until an expensive benchmark passes, combine unrelated capabilities into one skill, or preserve every sentence. Use the available evidence at the scale the decision warrants, and remove content with no useful role.

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

### Use prior advice as contextual evidence

**Suggestion:** Earlier expert advice and incident interpretation can justify considering a candidate in a new situation. Recover the relevant old circumstances and rationale, compare them with the present decision, and evaluate what transfers before promoting, retaining or dropping the candidate. Do not treat attributed situational advice as a universal rule.
**Reason:** The expert's underlying judgment model may be only partly explicit. Relevant advice can remain useful without establishing that the same remedy is best now. New evidence reduces dependence on the endorsement only insofar as it covers the relevant grounds; avoid counting the two as independent confirmation.
**Boundary:** Explicit instructions and preferences retain their authority. Keep attribution and uncertainty even when further evaluation changes a candidate's evidential weight.

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
**Suggestion:** Keep normally co-needed runtime guidance in one `SKILL.md`, with useful section boundaries. Follow the staged design guidance above before cost-driven splitting; a distinct consumer or lifecycle can justify a separate owner, but a topical distinction alone does not. Preserve supported content and behavior when changing what readers must discover or load.
**Suggestion:** Use a stable capability name; keep a revision label in string-valued `metadata.version`. For cross-surface packages, use the common Agent Skills frontmatter rather than a top-level `version` field or platform-only extensions. Metadata records the revision; it does not make the runtime select or update that revision. Distinct names such as `skill-v1` and `skill-v2` are useful when independently selectable variants are intentionally being compared, not as automatic release naming.
**Rule:** When creating a skill, use the bundled initializer when available, request only needed resource directories, replace scaffold placeholders, and preserve supported metadata when revising an existing package. Automatic invocation is the default; change it only when explicitly intended. Preserve unrelated policy or dependency fields in retained metadata.

### Prompts and agent definitions

**Rule:** Start with the least text that preserves the outcome, necessary evidence and context, constraints, autonomy boundaries, completion bar, and required return. Add model-specific guidance only where current official guidance or representative behavior shows it helps.
**Suggestion:** State facts, user choices, diagnoses, hypotheses, and recommendations distinctly when their status affects the recipient's decisions. Include exact mechanics only where they are not reliably discoverable or where variation is costly.

### Keep skill-specific design memory

**Suggestion:** When future maintenance would otherwise lose useful context, keep an optional `references/design-notes.md` beside the skill. This is companion memory for agents designing, reviewing, or revising that individual skill, not extra instructions for every ordinary invocation. Read it before consequential revisions. Link it from `SKILL.md` with that encounter condition. A short skill with no useful design context needs no companion.
**Content:** Retain incident/evaluation references, non-obvious decisions and their reasons, alternatives still worth considering, unresolved questions, and what evidence would justify changing the design. Compact working notes are sufficient; do not require a formal decision record or complete writeup for each point. Distinguish observed evidence and accepted choices from hypotheses and tentative proposals, with dates or version context where needed.
**Boundary:** Keep instructions and constraints needed to perform the skill in `SKILL.md` or its execution references; do not hide them in design memory. Notes explain the design but do not silently establish new authority or policy. Link to raw evidence at its owner rather than copying transcripts. Update or prune the notes as the design changes instead of accumulating an activity log.
**Naming:** Prefer `references/design-notes.md`: it identifies the content without implying code comments, human-only commentary, or a formal ADR process. Preserve an existing equivalent name when changing it would only create migration work. For this skill's naming and scope rationale, see [design notes](references/design-notes.md) when maintaining this convention.

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
