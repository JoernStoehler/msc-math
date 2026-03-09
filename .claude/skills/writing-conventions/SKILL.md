---
name: writing-conventions
description: Load before writing or editing CLAUDE.md, SKILL.md, or agent prompt files. Contains knowledge placement rationale, instruction design principles, and style rules.
---

# Writing Conventions for CLAUDE.md, SKILL.md, and Agent Prompts

## Why knowledge placement matters

Agents lack implicit knowledge: they don't know where to look for knowledge they need, and don't know what they don't know. Knowledge must be placed where agents will naturally find it.

Not all agents need everything. Subagents have well-scoped tasks and can triage reading efficiently. Less-scoped agents (main session agents) need cross-references for multi-step discovery rather than single-step lookup.

## Location tiers — rationale

**1. CLAUDE.md** — always in context. Cannot be skipped, but can be ignored despite being read (see "Instruction focus" below). Cost: every agent pays the context-window cost. Put knowledge here when useful for a majority of agents, or when it has been forgotten too often in other locations.

**2. SKILL.md files** — name + description always visible, body loaded on demand. Progressive disclosure for minority use cases. Limitation: agents sometimes run ahead without loading skills. Reminding agents to read skills is important. Subagents can be force-told to read a skill in their prompt.

**3. Standard locations** — code comments, doc comments, file headers, README.md, config files. Agents naturally look at these from training. Prefer these over SKILL.md when knowledge is tied to a specific file or folder. Cost to non-interested agents is low — agents habitually skip irrelevant code comments. Anti-pattern: agents sometimes dump too much into README.md, treating them as catch-all dumps.

**4. TASKS.md** — project management knowledge (features, experiment ideas, deferred tasks, external constraints). A single .md file + git log is more agent-ergonomic than GitHub issues or kanban boards. Grows stale — agents don't habitually update it. Other agents only need to know it exists.

**5. MEMORY.md** — catch-all for session learnings, communication behavior. Occasionally clean and migrate stable entries to CLAUDE.md or standard locations.

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
- **Review subagents** with focused instruction sets catch what the parent agent missed. Each subagent gets a small instruction set for one concern and reports violations. This catches both minor issues (half-assed code comments) and major issues (fundamental design violations requiring rollback)

## Style rules for CLAUDE.md

The ideal CLAUDE.md is:
- Reminders about standard practices, highlighting their importance for this project
- Simple, actionable behavior modifications
- Reinforcements of default behavior where forgetting has been observed

Prefer reminders of well-known best practices over teaching novel practices. Novel practices are more expensive for agents to follow — they must override default behavior.

### Organize by agent lifecycle, not by taxonomy

Structure sections around the moments an agent faces a decision: "when you produce knowledge", "when you need knowledge", "when you edit code". Not around abstract categories ("location tiers", "knowledge types"). The agent hits a moment and immediately finds the relevant instruction — no interpretation needed.

### No meta-sentences in CLAUDE.md

Don't explain *why* a section exists ("Agents lack implicit knowledge..."). That's meta-knowledge about the section — it belongs in this skill, not in CLAUDE.md. CLAUDE.md goes straight to what to do.

### Decision trees, not prose

"Tied to a file? → code comment. Applies to most agents? → CLAUDE.md." gives a concrete action. "Knowledge should be placed appropriately" gives nothing.

### Rationale lives in the skill

The "why" behind CLAUDE.md rules lives here (in the writing-conventions skill). CLAUDE.md is pure action. When an agent needs to understand *why* a rule exists — e.g., to decide whether an edge case should follow the rule — they load this skill.

### One claim per bullet

Dense prose packs multiple claims that get lost on rewrite. Each bullet states one claim.

Bad: "Claude Code is okay at spotting implicit criteria imposed on a task's scope and acceptance criteria by other tasks and by Claude Code's capability limits and default habits."

Good: Break into atomic bullets where each claim is independently visible.

### Qualifier preservation

Every adjective narrows meaning. "Clear, detailed, explicit, structured, verifiable" is not a synonym list:
- "clear" = easy to understand, not vague or ambiguous
- "detailed" = all steps included for verification
- "explicit" = implications spelled out, not left for the reader
- "structured" = modular chunks the reader can selectively zoom into
- "verifiable" = local validity check possible for every step

When rewriting: does this preserve all constraints the original imposed?

### Concrete over abstract

- "Run `cargo test` from `crates/`" not "run the tests"
- "Only Jörn merges to main" not "merges require human approval"

### Decision trees over prose principles

When behavior depends on conditions, use if-then structure instead of prose principles that invite interpretation variance.

### Priority ordering

Clarity & unambiguousness > correctness > maintainability >>> tokens (nearly unimportant at our context window scale).

Using 50 extra words to prevent a misunderstanding is always worth it.

## Writing SKILL.md files

- Frontmatter: `name` and `description` are always loaded. Make the description specific enough that agents can decide whether to load the skill.
- Body: loaded on demand. Organize for the agent who loaded the skill — they already know they need it.
- SKILL.md writing/editing is usually initiated or approved by Jörn — there's a natural moment to load this skill.

## Optimizing rules that don't work

When agents don't follow a rule:
1. **Notice** when a different behavior would have been better — not just fixing failures, but also noticing missed opportunities for improvement.
2. **Instruct** agents to do that different behavior.
3. **If that's not working:** optimize what behavior to aim for. The rule may be fighting agent defaults too hard. Often a different behavior that's closer to defaults achieves most of the value.
4. **Refactor the project** layout or state so the desired behavior becomes the natural default. Agents can often do this refactoring cheaply.

Steps 3 and 4 work together as an optimization loop, not an escalation ladder. Three dimensions are optimized jointly:
- (a) The original target behavior
- (b) Related workflows that interact with this behavior
- (c) Project layout and state

Optimizations are not always local fine-tuning — they can be wholesale switches to entirely different optima in how-to-run-the-project space. No specific optimization algorithm is recommended; trial and error combined with detailed feedback/postmortems works.

## Why word-choice sensitivity matters

Jörn communicates via subtle word choices that encode real distinctions. Agents trained on natural language tend to normalize variations ("not quite" → "yes but also"), losing the correction's content. This is a known failure mode in human-agent communication: the human's correction gets paraphrased back into the agent's original framing, and the distinction is lost.

The CLAUDE.md instruction tells agents to adopt Jörn's exact phrasing rather than paraphrasing, because the cost of preserving exact wording is zero but the cost of losing a distinction compounds across the session.

## Why plan file maintenance matters

Context compaction is lossy. The compaction summary loses scheduled items (most dangerous), context for upcoming items (moderately dangerous), and completed items (least dangerous). The plan file is the only persistent memory that survives compaction without loss.

This is a rule, not a suggestion — Jörn has told multiple agents about this. Agents that don't maintain the plan file lose track of scheduled work after compaction, which wastes Jörn's time re-explaining what needs to be done.

The danger ranking (scheduled > context > completed) reflects the asymmetry: completed items are already done and only matter for final reporting, while forgotten scheduled items never get done at all.

## Writing agent prompts (.claude/agents/*.md)

- Agent prompts 1:1 copy relevant CLAUDE.md sections (not summaries, not references)
- Agent-specific content (task description, output format, detection rules) goes at the top
- CLAUDE.md copies go below, labeled with source
- Cross-reference tags for maintainability:
  - In CLAUDE.md: `<copied-to>agent1, agent2</copied-to>` after section headers
  - In agent prompts: `<copied-from>CLAUDE.md § Section Name</copied-from>` before copied blocks
  - When editing either side, check the tags and update the other side
