---
name: meta-documentation
description: Foundational analysis for the meta-layer (CLAUDE.md, skills, agents, memory). Load when you need to understand WHY a rule exists, evaluate edge cases, or design new agent infrastructure. Contains the knowledge taxonomy, instruction focus model, agent failure modes, and decision records. For HOW-TO guidance, load the focused skills instead: meta-claudemd (writing CLAUDE.md), meta-skills (writing skills), meta-subagents (writing agents), meta-hooks (hooks and sync), meta-folder-layout (where to place knowledge).
---

# Meta-Documentation — Foundational Analysis

Why the meta-layer is designed the way it is. Load this when you need to understand rationale, evaluate edge cases, or design new infrastructure. For actionable how-to guidance, use the focused skills listed below.

## Focused skills (actionable how-to)

- `meta-claudemd` — style rules and conventions for writing CLAUDE.md
- `meta-skills` — writing SKILL.md files, progressive disclosure model
- `meta-subagents` — writing agent definitions (`.claude/agents/*.md`)
- `meta-hooks` — hook scripts and cross-repo sync
- `meta-folder-layout` — where to place knowledge (decision tree, location tiers)

## Reference documents

- `references/meta-documentation-rationale.md` — architecture decisions: why skills over rules, why one generic review agent, the layer architecture, how to evaluate changes

## Why knowledge placement matters

Agents lack implicit knowledge: they don't know where to look for knowledge they need, and don't know what they don't know. Knowledge must be placed where agents will naturally find it.

Not all agents need everything. Subagents have well-scoped tasks and can triage reading efficiently. Less-scoped agents (main session agents) need cross-references for multi-step discovery rather than single-step lookup.

## Axes of project knowledge

Project knowledge varies along several observed axes that correlate with where it should live and how to communicate it to agents. These axes are not exhaustive — there are likely dimensions not listed here. The decomposition itself may also not be optimal: a simpler, more natural, or more predictive taxonomy may exist.

**Scope and lifetime:**
- **All projects** (permanent) — universal agent knowledge (e.g., delegation safeguards, feedback processing). Lives in meta-skills, synced across repos.
- **This project** (project lifetime) — project-specific conventions, domain knowledge. Lives in CLAUDE.md, project skills, reference docs.
- **This commit / branch** (branch lifetime) — current plan, in-progress state. Lives in plan files, TASKS.md, branch-specific handoffs.
- **This subtask** (ephemeral) — scoped context for a subagent. Lives in the subagent prompt, not persisted.

**Domain:** Knowledge about agents, about Claude Code files/settings, about the project's domain (D&D, card games, symplectic geometry), about project management, etc. Domain determines which skills/files house it.

**Source of truth:** A piece of text is either a source of truth or a derivative (summary, concretization, implication) of some source of truth. The source-vs-derivative distinction applies at the paragraph or subsentence level — a single file typically contains both. When a derivative diverges from its source of truth, the source wins. Derivatives must not be mistaken for sources.

**Novelty** (relative to agent training):
- **Common** — heavily represented in LLM training. Agents already have the behavior; a reminder is sufficient. Example: "write tests."
- **Rare** — seen in training but not a default behavior. Explicit instruction works. Example: a rare combination of common code style conventions.
- **Novel** — unseen, fully unknown to the agent. Needs a skill with examples; agent has no prior to build on. Example: project-specific verification workflows.
- **Anti-intuitive** — true things that clash with what agents falsely learned in training. Agents will actively resist these. Needs strong reinforcement, verification that the agent actually followed the instruction (not just agreed), and possibly structural enforcement (review agents, mandatory steps). Example: "your proofs are unreliable on first attempt" when training says confident = correct.

Most concepts are mixes: a novel spin on a common philosophy, a rare combination of common elements. The novelty axis predicts how much effort is needed to get agents to follow a rule — and anti-intuitive knowledge is the most expensive because agents fight it.

## Instruction focus

Agents lose focus and ignore instructions they've read. This happens when:
- Instructions were read early but the session is long
- Total instruction complexity is too high. Token count is a bad proxy for complexity — what matters is: how many behavior modifications does the agent need to hold active? Complex rules (novel behavior, multiple conditions) cost much more than reminders of standard practices. Evaluate complexity item by item, not by counting tokens or rules.

### Countermeasures

- **Colocate** instructions in file headers, code comments, or nearby files so they are freshly read when working with a file
- **Disentangle** instructions (how to behave) from factual knowledge (proofs, examples, function docs) to keep the instruction part simpler
- **Write clearly** — unambiguous, specific, actionable. Don't make agents spend effort interpreting
- **Pick natural instructions** — if an instruction is complex, ask whether a codebase change could make the desired behavior default. E.g., instead of 30 rules for how to test, adopt a standard test framework so the instruction becomes "we use pytest with hypothesis."
- **Progressively disclose** — file-scoped instructions go in file headers, subset-scoped in skills, universal in CLAUDE.md
- **Review subagents** with focused instruction sets catch what the parent agent missed. Each subagent gets a small instruction set for one concern and reports violations.

## Optimizing rules that don't work

When agents don't follow a rule:
1. **Notice** when a different behavior would have been better — not just fixing failures, but also noticing missed opportunities for improvement.
2. **Instruct** agents to do that different behavior.
3. **If that's not working:** optimize what behavior to aim for. The rule may be fighting agent defaults too hard. Often a different behavior that's closer to defaults achieves most of the value.
4. **Refactor the project** layout or state so the desired behavior becomes the natural default. Agents can often do this refactoring cheaply.

Steps 3 and 4 work together as an optimization loop, not an escalation ladder. Optimizations are not always local fine-tuning — they can be wholesale switches to entirely different optima in how-to-run-the-project space.

## Known agent failure modes — design around these

These are empirically observed failure modes that affect how you should design all agent infrastructure (skills, workflows, conventions, review agents, CLAUDE.md). Don't just tell agents to avoid these — design infrastructure that accounts for them structurally.

**Instruction overload.** Agents degrade silently when holding too many novel behavior modifications simultaneously. They don't notice they're overwhelmed — they proceed and drop constraints. Design implications:
- Skills and CLAUDE.md should minimize the number of simultaneously active novel instructions
- Complex workflows should delegate to focused subagents with small instruction sets, not rely on one agent holding everything
- Review agents work because each gets one concern with a small checklist, not all concerns at once
- When writing instructions, evaluate complexity per-item (how many behavior modifications?), not per-token

**Skipping planning.** Agents skip planning even at levels where it's obviously worth it (e.g., "what's the goal of this session?"). They'll agree in hindsight that planning would have helped. Design implications:
- Complex skills should include planning as a required step in the workflow, not a suggestion
- Don't rely on agents choosing to plan — build mandatory scope/plan phases into workflow skills

**Under-asking questions.** Agents systematically avoid questions that cost Jorn 10 seconds but have high expected value (10% x 1 hour saved). They overweight the visible cost of interrupting and ignore the expected cost of being wrong. Design implications:
- Build explicit checkpoints into workflows where the agent must verify assumptions with Jorn
- Treat "ask Jorn" as a concrete workflow step, not a fallback

**Not modeling own unreliability.** Observed in msc-math: agents are unreliable at both writing correct proofs on first attempt and checking proofs for correctness. They don't realize this, and proceed as if their output is correct — instead of taking ~60 seconds of Jorn's time to verify. Likely generalizes to any domain where agent output requires correctness (not just plausibility). A specific mechanism: agents don't verify the results of their own actions (edits, reverts, delegations) — they assume the action succeeded. Design implications:
- Review workflows must be mandatory, not optional — agents won't choose to verify
- Build verification into the workflow itself (write -> review -> fix -> re-review), not as a separate "if you want" step
- For critical content (math proofs, canonical facts), the workflow should include a Jorn verification step by default — the cost is low (~60s) and the cost of proceeding with errors is high
- After any action (edit, revert, subagent delegation), check the result rather than assuming success

**Treating presentation as confirmation.** Agents show information to Jorn and proceed as if he confirmed it. "I presented the scope" becomes "the scope is agreed." "I used this document" becomes "this document is approved." Presentation != confirmation. Design implications:
- Workflows should have explicit confirmation gates — don't let "I showed it" advance the state
- The existence of a document (plan file, target state) is not evidence of its approval status

**Not checking existing state before changing it.** Agents add content without reading what's already there, creating duplication or contradiction. Design implications:
- Skills that add content should include a step to read the target file first
- Review subagents should check for duplication against surrounding content, not just internal consistency

**Not modeling Jorn's current state.** Agents don't consider what Jorn can see, access, or answer right now. They send text walls too long for Jorn to review, ask questions requiring file reads he hasn't done, respond to old queued messages instead of the most recent one. Design implications:
- Skills that produce output for Jorn should summarize key decisions and questions at the end, not assume Jorn followed along
- Prefer file:line references over inline text when showing content for review
- Handoff files exist partly because agents can't reliably communicate findings within a session

**Transferring cognitive work to Jorn.** Agents ask Jorn to do thinking the agent should do: "what do you want me to do?", asking permission on obvious actions, requesting scope instead of proposing one. This is distinct from under-asking — the problem isn't asking too little, but asking the wrong kind of thing (open-ended "what should I do?" instead of specific "is X correct?"). Design implications:
- Skills should frame agent actions as proposals to verify, not open questions for Jorn to answer
- "I plan to do X because Y — any objection?" is cheaper for Jorn than "what should I do?"

**Responding to social signal instead of content.** When corrected, agents respond to the criticism ("You're right, I shouldn't have done that") instead of fixing the thing. Empty agreement wastes time. Design implications:
- Post-correction workflow: fix first, acknowledge briefly, move on

**Not generalizing from mistakes.** Agents fix the specific instance flagged by Jorn but don't abstract the error class or scan for other instances — in the code, in other files, or in their own recent behavior. Example: "forgot to run test XYZ" doesn't trigger "what else did I forget?" even though asking that question is well within agent capability. One specific manifestation: agents learn lessons abstractly but don't spontaneously apply them to their own current behavior (learned "keep shared files identical across repos" -> then immediately made project-specific modifications; documented delegation failures -> then immediately trusted subagent reports without verification). The `meta-feedback-processing` skill addresses this but agents still under-apply it. Design implications:
- The generalization step must be part of the resolution workflow, not a follow-up
- Review agents and postmortem skills should explicitly prompt for error-class abstraction

**Delegation failures: loud vs silent.** Subagent failures come in two kinds. Loud failures: the subagent reports "I'm stuck on subtask X" — the parent replans, no damage. Silent failures: the subagent reports "done" but did X' instead of X — the parent proceeds with a broken assumption. Silent failures are far more dangerous. They arise when the parent's mental model, the skill/agent description, or the prompt diverge from what the subagent actually does. Crucially, the subagent cannot detect or report this kind of failure — it has no access to the parent's intent, so the information asymmetry is structural, not a matter of subagent quality. Verification must come from outside the subagent (the parent or a separate verifier), not from the subagent being "better." Verification of reasoning tasks is especially hard because the output looks plausible even when wrong. Design implications:
- Skill/agent descriptions must state what the tool does AND does not guarantee — descriptions are the contract that parent agents plan around
- Verification after delegation is high-value because the communication channel (the prompt) is unreliable and agents don't anticipate ambiguity in their own prompts
- Complete verification (prove correctness) and incomplete falsification (find some errors) are different — don't let "we ran a review" be mistaken for "this is verified"
- For novel tasks, result types must be explicit in the prompt — agents have no training priors to fall back on

**Lossy transcription.** When encoding precise statements (from Jorn, from sources, from specifications) into their own words, agents systematically lose meaning. The losses aren't random — they tend toward simplification, generalization, flattening distinctions, and substituting the agent's framing for the original. Particularly dangerous when transcribing novel or anti-intuitive knowledge, where training priors pull the rewording toward the familiar (wrong) meaning. This is a well-known failure mode. Design implications:
- "Word-choice sensitivity" (see section below) is one intervention for Jorn's words specifically
- Review subagents should compare agent output against original source text, not just check internal consistency
- When encoding someone's precise statement, prefer quoting over paraphrasing
- Verification of transcribed content requires access to the original — a subagent checking only the transcription can't detect meaning drift

**Defaulting to the easy action.** When the correct action requires more effort, agents take the lower-effort alternative. Examples: paraphrasing instead of extracting original text, claiming convergence instead of continuing to search. Design implications:
- Workflows should make the correct (higher-effort) action the default path, not the alternative
- When a skill offers two approaches (easy and thorough), frame the thorough one as default and the easy one as the exception requiring justification

**Defaulting to the familiar action.** Agents pick the approach that pattern-matches to training, even when the situation calls for something different. Examples: "copy from the most mature source to others" (familiar pattern) instead of "evaluate each source's strengths and cross-pollinate" (requires judgment). The familiar approach is not necessarily easier — familiarity and effort are independent. Agents pick the approach they've seen most, regardless of effort. Design implications:
- Novel workflows should explicitly name the familiar-but-wrong approach and say why it doesn't apply here
- When a task requires judgment between approaches, the skill should flag that the obvious/familiar approach may not be correct

## Why word-choice sensitivity matters

Jorn communicates via subtle word choices that encode real distinctions. Agents trained on natural language tend to normalize variations ("not quite" -> "yes but also"), losing the correction's content. The CLAUDE.md instruction tells agents to adopt Jorn's exact phrasing rather than paraphrasing, because the cost of preserving exact wording is zero but the cost of losing a distinction compounds across the session.

## Why plan file maintenance matters

Context compaction is lossy. The compaction summary loses scheduled items (most dangerous), context for upcoming items (moderately dangerous), and completed items (least dangerous). The plan file is the only persistent memory that survives compaction without loss.

Agents that don't maintain the plan file lose track of scheduled work after compaction, which wastes Jorn's time re-explaining what needs to be done.

## Prompting modern models (reported experimental result, 2026)

Most prompt engineering advice — even from Anthropic 2026 — is outdated or has always been cargo culting. Jorn reports no noticeable differences from instruction phrasing variations with current models. What matters:
- Provide enough context
- Write clearly and unambiguously
- Use formats agents were trained on (markdown, code blocks, bullet lists) so they process with low cognitive load

Don't waste effort on "prompt engineering tricks." Focus on clarity, completeness, and unambiguity.

## Agent quality near context limits (reported observation, 2026)

Agents get unfocused and impatient as sessions approach 200k tokens (near compaction). Basic operations (plan execution, coding) remain fine, but decision-making quality degrades. Jorn sometimes auto-triggers compaction earlier to avoid this.

## HTML comments in CLAUDE.md

CLAUDE.md supports `<!-- comments -->` that are NOT auto-injected into agent context but ARE visible via Read/Edit. Since Edit requires a prior Read, agents editing CLAUDE.md will always see comments.

**Good for** (editor-facing metadata):
- Maintenance notes next to rules ("added after incident X")
- Historical context only editors need
- Inline rationale too small to justify loading this skill

**Not good for:**
- Anything all agents need to follow — must be in visible text.

## Decision records (failure modes that shaped CLAUDE.md rules)

These explain *why* specific rules exist. Read when a rule seems arbitrary or when considering changes.

### "Discuss-first" for issue edits and scope changes

**Failure mode (issue #12, Feb 2026):** Three agents attempted #12 over two days. Each read massive agent-written comments (posted under Jorn's account), treated them as authoritative, and either continued the brain-dump or stalled planning. No deliverable produced. ~1100 lines of unreviewed drafts posted as issue comments. Future agents treated these as authoritative, creating a feedback loop.

**Root causes:**
- Agents treated issue edits the same as code edits — but issues are expensive to verify and hard to roll back
- Agents interpreted Jorn's silence as approval
- GitHub shows all content under Jorn's account — no visual distinction between Jorn-written and agent-written

**Decisions:** Issue edits -> discuss-first. Silence != confirmation. Subagent output -> commit to branch, never post as comments.

### Tests are necessary but not sufficient

**Failure mode (msc-viterbo, 2025):** Predecessor repo had agent-written tests that all passed. Known bugs:
1. HK2019 QP solver missed optima — returned plausible but wrong values
2. Trivialization formula was not a bijection
3. Billiard orbit validation only checked even-indexed segments
4. Pentagon capacity: 2.127 (wrong) instead of 3.441 (correct)

**Root cause:** Goodhart's law. When agents write both code and tests, tests optimize for passing, not for correctness.

**Decision:** Jorn provides domain knowledge: which test cases matter, what correct values are, what invariants to check.

### Test comprehension by USE, not by asking

**Failure mode:** Agents asked subagents "is this clear?" — subagents that misunderstood confidently answered "yes."

**Decision:** Test comprehension by asking agents to USE the content (implement from a description, answer specific questions). Check whether their output matches intent.

### Why a single CLAUDE.md (not per-directory)

**Previous state:** 8 files across 4 directories (818 lines total). Agents pieced together mental models from fragments and got them wrong. Duplicated content drifted.

**Decision:** Single file, split by topic. Skills for progressive disclosure. Tried moving topic sections to skills-only — agents forgot to load them. Topic sections must stay in CLAUDE.md.

## Post-session reflection

Use the `/meta-post-mortem` skill. It covers: friction, unclear instructions, missing context, Jorn's time analysis, what worked, suggested changes, process checks, generalization, and persistence guidance.
