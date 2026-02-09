# CLAUDE.md refactor — research and design log

Date: 2026-02-09
Session: Claude Code (Opus 4.6), local devcontainer
Branch: `claude/rewrite-claude-md`

## Problem statement

CLAUDE.md is 332 lines. Top half (lines 1-87) written/reviewed by Jörn, bottom half (88-331) agent-written and unreviewed. Agents editing the file have dropped adjectives and quantifiers during rewrites, treating word reduction as always positive. Example: "clear, specific, detailed, unambiguous, cognitive low-overhead" → "high-quality" — losing the operational meaning of each word.

## Research conducted

Three parallel Sonnet subagents researched current (late 2025 / early 2026) best practices. Older prompting advice was excluded — models changed enough that pre-2025 advice is often wrong for Opus 4.5/4.6 and GPT-5 series.

### Anthropic (Claude Opus 4.5/4.6) findings

Sources: [Prompting best practices - Claude API Docs](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices), [Best Practices for Claude Code](https://code.claude.com/docs/en/best-practices), [Introducing Claude Opus 4.6](https://www.anthropic.com/news/claude-opus-4-6), [Writing a good CLAUDE.md | HumanLayer](https://www.humanlayer.dev/blog/writing-a-good-claude-md)

1. **System prompt responsiveness**: Opus 4.5/4.6 are "more responsive to the system prompt than previous models." Aggressive language (MUST, CRITICAL, NEVER) that was needed for 3.x can now cause overtriggering. The fix from docs: "dial back any aggressive language."

2. **Instruction budget**: Research consensus is ~150-200 instructions before uniform degradation. Claude Code's system prompt already takes ~50 instructions, leaving ~100-150 for CLAUDE.md.

3. **What makes information stick**: XML tags help section parsing. Position matters (early = better retention). Specificity > abstraction ("Run pytest experiments/" > "Write tests"). Progressive disclosure — keep root CLAUDE.md minimal, reference separate files.

4. **Known Opus 4.6 failure modes**: (a) Overthinking / excessive exploration, (b) Over-engineering / overeagerness, (c) Destructive actions without confirmation, (d) Excessive subagent spawning, (e) Ignoring CLAUDE.md content it deems task-irrelevant.

5. **CLAUDE.md length recommendation**: Community consensus is under 300 lines. HumanLayer's own is <60 lines. Content Claude deems task-irrelevant gets actively ignored (system prompt tells it to disregard non-essential context).

6. **Known issue — training data overrides context**: GitHub issue #21119 documents Claude repeatedly ignoring CLAUDE.md instructions in favor of training data patterns. The core pattern: "Reading ≠ Following." Multiple duplicate issues (#20989, #19252, #18411) confirm this is systemic.

### OpenAI (GPT-5 series) findings

Sources: [GPT-5 Prompting Guide | OpenAI Cookbook](https://cookbook.openai.com/examples/gpt-5/gpt-5_prompting_guide), [GPT-5.1 Prompting Guide](https://cookbook.openai.com/examples/gpt-5/gpt-5-1_prompting_guide), [GPT-5.2 Prompting Guide](https://cookbook.openai.com/examples/gpt-5/gpt-5-2_prompting_guide)

1. **Models compress nuance under pressure**: Qualifiers disappear in paraphrase. A rule "use bullets only when the user explicitly asks for 'options,' 'list,' or 'checklist'" was misunderstood — model started bulleting everything. Fix: repeat the condition inside the formatting rule itself.

2. **Contradictions waste reasoning tokens**: GPT-5 docs state "poorly-constructed prompts containing contradictory or vague instructions can be more damaging to GPT-5 than to other models, as it expends reasoning tokens searching for a way to reconcile the contradictions."

3. **Decision trees > prose principles**: Instead of "be autonomous," prescribe: "If key information is missing, pause and ask 1-3 clarifying questions. For users who sound rushed, minimize questions." The if-then structure locks meaning.

4. **Embed examples within rules, not separately**: When tool usage rules included examples immediately after (not in a separate section), accuracy improved.

5. **Repeat critical constraints at execution points**: For coding agents: "Before any non-trivial code change, ensure the current plan has exactly one appropriate item marked in_progress." Restating at the decision point, not just in the overview, improves adherence.

6. **Making tradeoffs explicit prevents inconsistent behavior**: When an event-planning agent had conflicting guidance ("be concise" vs "err on completeness"), it oscillated. Solution: "make tradeoffs explicit" by stating when one principle overrides another.

7. **Specificity defeats abstraction**: "2-5 sentences" works, "be brief" doesn't. Named sections with headers > buried instructions. Numbered thresholds > vague frequency guidance.

8. **Use metaprompting to catch contradictions**: Ask the model to diagnose conflicting instructions, then ask it to patch them. Surfaces issues humans miss.

### CLAUDE.md specific practices

Sources: [Claude Code CLAUDE.md docs](https://code.claude.com/docs/en/memory), [builder.io CLAUDE.md guide](https://www.builder.io/blog/claude-md-guide), GitHub community examples

1. **Auto-loaded at session start**: CLAUDE.md is read automatically with no memory of previous sessions.

2. **`.claude/rules/` directory**: Files here are automatically included without explicit imports. Useful for splitting large instruction sets across topic-specific files.

3. **`@imports` syntax**: Reference external files with `@path/to/file` to keep main file lean.

4. **Do not auto-generate**: The `/init` command produces a starter file but should be manually refined. "The leverage flows both directions — poor CLAUDE.md amplifies downstream problems."

5. **Universal content only**: Since CLAUDE.md appears in every session, task-specific guidance belongs elsewhere.

## Key corrections from Jörn during design discussion

These corrections significantly changed the plan direction. Each is documented because a future agent might re-derive the wrong conclusions from the same research.

### 1. CLAUDE.md is user-context, not system prompt

The Anthropic finding about "dial back aggressive language for Opus 4.6" applies to system prompts. CLAUDE.md is injected as user-context, where model sensitivity is actually lower than the system prompt. This means:
- The instruction budget research (~150-200) doesn't directly constrain CLAUDE.md length
- Aggressive language (MUST NOT, NEVER) may actually be needed, not harmful
- The "overtriggering" concern doesn't apply

### 2. Density, not length, is the problem

The community recommendation to "keep CLAUDE.md under 300 lines" optimizes for the wrong thing. Token cost is nearly zero at ~100k context window scale. The real cost is information density — dense prose is easy to edit destructively.

Priority ordering: clarity & unambiguousness > correctness > maintainability >>> tokens

Consequences:
- File may grow longer (more bullets = less density = harder to accidentally erase info)
- Redundancy is a feature (same rule in two places is more robust)
- 50 extra words to prevent a misunderstanding is always worth it
- We are allowed to switch to different practices if they get us 90% of desired behavior but are actually clear and easier to write

### 3. No file splitting

CLAUDE.adr.md documents (lines 96-108) a previous decision to consolidate from 8 files to 1. The problems with multi-file setups were real: fragmented mental models, duplication, inconsistency after edits. The Plan subagent recommended extracting to separate files — this contradicts a deliberate, well-motivated decision.

### 4. "What, why, how" is a completeness checklist, not an organizational template

The research frames "what/why/how" as content pillars. Jörn clarified: use it to check coverage after writing a section, not to organize content into separate bins. The "how" also has a "why" — forcing separation creates artificial boundaries.

### 5. Within-section ordering

Jörn specified: prerequisites → rule → motivation → further material. This is a within-section structure that protects information during edits:
- An agent rewriting top-down hits the rule first (hardest to discard)
- The motivation explains why it matters (agent must consciously choose to drop it)
- Further material is cheapest to lose

### 6. Edit model uses git, not chat

Original plan: "agents propose edits in chat, Jörn approves." Jörn corrected: agents edit directly on their branch, Jörn reviews via git diff in VS Code. This is better because:
- A/B comparison in diff view
- Jörn can edit inline to show what he wants (instead of explaining abstractly)
- Line references are precise
- Less round-trip communication overhead

## Decisions made

1. **Style guide**: Written as a new section in CLAUDE.adr.md (7 rules). Covers: one claim per bullet, within-section ordering, qualifier preservation, priority ordering, concrete over abstract, decision trees over prose, completeness check.

2. **Claim inventory before rewrite**: Every atomic claim in current CLAUDE.md is listed (265 claims found), classified by source (Jörn-reviewed vs unreviewed), and flagged for issues. Jörn reviews the inventory before the rewrite begins. This catches wrong claims before they get polished into cleaner format.

3. **Full rewrite applying style guide**: Break dense paragraphs into atomic bullets, apply within-section ordering, add motivation where missing (especially unreviewed half), add meta-section about editing the file. File may grow longer.

4. **No content removal**: All 265 claims are preserved unless Jörn explicitly marks one for removal in the inventory review.

## Factual issues found in current CLAUDE.md

- Claim #90: "[aspirational — migrating to devcontainer]" — migration has happened
- Claim #172: `/home/user/msc-math` — now `/workspaces/msc-math`
- Claim #135: "Jörn doesn't see exact edit diffs in chat" — Jörn now reviews via VS Code diffs
- Claim #173: Rust 1.93 — needs verification against current environment
