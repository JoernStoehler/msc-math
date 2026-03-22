---
name: meta-claudemd
description: Style rules and conventions for writing CLAUDE.md. Load when editing CLAUDE.md or evaluating whether a rule belongs there. Covers structure, phrasing, and what does/doesn't work empirically. Does NOT cover where knowledge should live (see meta-folder-layout) or foundational analysis (see meta-documentation).
---

# Writing and Editing CLAUDE.md

How to write CLAUDE.md so agents actually follow it. For the rationale behind these rules and the foundational knowledge model, load the `meta-documentation` skill.

## Related skills

- `meta-documentation` — foundational analysis: knowledge taxonomy, instruction focus, failure modes
- `meta-folder-layout` — deciding whether something belongs in CLAUDE.md vs a skill vs a reference doc

## The ideal CLAUDE.md

- Reminders about standard practices, highlighting their importance for this project
- Simple, actionable behavior modifications
- Reinforcements of default behavior where forgetting has been observed

Prefer reminders of well-known best practices over teaching novel practices. Novel practices are more expensive for agents to follow — they must override default behavior.

## Organize by agent lifecycle, not by taxonomy

Structure sections around the moments an agent faces a decision: "when you produce knowledge", "when you need knowledge", "when you edit code". Not around abstract categories ("location tiers", "knowledge types"). The agent hits a moment and immediately finds the relevant instruction — no interpretation needed.

## No meta-sentences in CLAUDE.md

Don't explain *why* a section exists ("Agents lack implicit knowledge..."). That's meta-knowledge about the section — it belongs in the `meta-documentation` skill, not in CLAUDE.md. CLAUDE.md goes straight to what to do.

## Decision trees, not prose

"Tied to a file? -> code comment. Applies to most agents? -> CLAUDE.md." gives a concrete action. "Knowledge should be placed appropriately" gives nothing.

## Rationale lives in the skill

The "why" behind CLAUDE.md rules lives in the `meta-documentation` skill and its reference docs. CLAUDE.md is pure action. When an agent needs to understand *why* a rule exists — e.g., to decide whether an edge case should follow the rule — they load that skill.

## One claim per bullet

Dense prose packs multiple claims that get lost on rewrite. Each bullet states one claim.

## Qualifier preservation

Every adjective narrows meaning. "Clear, detailed, explicit, structured, verifiable" is not a synonym list:
- "clear" = easy to understand, not vague or ambiguous
- "detailed" = all steps included for verification
- "explicit" = implications spelled out, not left for the reader
- "structured" = modular chunks the reader can selectively zoom into
- "verifiable" = local validity check possible for every step

When rewriting: does this preserve all constraints the original imposed?

## Concrete over abstract

- "Run `cargo test` from `crates/`" not "run the tests"
- "Only Jorn merges to main" not "merges require human approval"

## Priority ordering

Clarity & unambiguousness > correctness > maintainability >>> tokens (nearly unimportant at our context window scale).

Using 50 extra words to prevent a misunderstanding is always worth it.

## What worked and what didn't (assessed March 2026)

Empirical observations from ~5 months of use. Input to the optimization loop.

**Worked well:**
- **Mathematical context** — useful across many agent tasks
- **Multi-Language Codebase** — worked great; possibly slightly verbose but not worth risking degradation by trimming
- **Git conventions** — fixed bad behavior (stale origin/main, wrong diff syntax)
- **Subagent usage** — explicit instruction of novel behavior works
- **Topic sections in main CLAUDE.md** — must stay in CLAUDE.md; skills-only didn't work
- **Review workflow (short)** — short enough to keep in CLAUDE.md; risk of agents forgetting to look up a review skill is too high
- **Environment** — same as topic sections: didn't work as skill-only

**Caused problems:**
- **Roles section** — taxonomic format doesn't help agents at decision moments. Needs to be distributed into per-topic behavioral modifications.
- **Decision authority** — same problem as Roles; needs redistribution
- **Session Workflow scope/plan separation** — agents jump into plan too fast, rarely do scope phase separately

**Better as SKILL.md than CLAUDE.md:**
- **Post-session reflection** — better as a skill loaded at session end
- **Plan workflow** — useful but most agents already use it somewhat; might work as skill
- **Meta-rules meta-knowledge** — meta-knowledge itself can move to skill; actionable instructions stay in CLAUDE.md
