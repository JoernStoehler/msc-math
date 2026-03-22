# Meta-Documentation: How to Make Good Decisions

This document teaches you how to write and modify meta-documentation (CLAUDE.md, skills, agents, memory, reference docs) at the same quality level as the current version. Read this before making structural changes to the meta-layer.

## The constraints you're working with

Agents need instructions to produce correct work, but:
1. Instructions loaded too early get forgotten during long sessions (context decay)
2. Instructions loaded too late miss the planning phase (agents already committed to an approach)
3. Too many instructions at once → agents ignore some ("instruction overload")
4. Duplicated instructions across files → maintenance burden, drift, contradictions
5. Agents don't know what they don't know — they won't search for instructions they don't realize exist

When agents get confused or slowed down, it's usually because the meta-layer violates one of these constraints. The fix is cleanup, not more instructions.

## Decision framework: where does this knowledge go?

When you have a new piece of knowledge to place, ask these questions in order:

**1. Who needs it?**
- Every agent → CLAUDE.md (but keep it lean — every token here costs every agent)
- Agents working on a specific topic → Skill SKILL.md
- Only review agents checking a specific concern → Reference doc under `review/references/`
- Only agents that open this specific file → Code comment or file header
- Only when editing the meta-layer itself → meta-skills (`meta-claudemd`, `meta-skills`, `meta-subagents`, `meta-hooks`, `meta-folder-layout`, `meta-documentation`)

**2. When do they need it?**
- During planning (before committing to approach) → Must be visible early. Skills work (name+description always visible). CLAUDE.md works. Reference docs don't — they require the agent to already know to look.
- During execution → Skills or reference docs. The skill body tells the agent which reference docs exist.
- During review → Review checklist reference docs. Subagent reads the checklist matching its concern.

**3. How stable is it?**
- Stable convention → Skill SKILL.md
- Evolving detection heuristic → Reference doc (cheaper to update)
- Project state that decays → MEMORY.md or TASKS.md
- One-off context → Don't persist it. Let it live in the conversation.

## Architecture layers and their roles

```
CLAUDE.md                        Always loaded. Project context, workflow, communication.
                                 Names which skills each topic needs. Kept lean.
    ↓
Skills (SKILL.md)                Name+description always visible. Body on demand.
                                 One per topic. Contains conventions (what's correct).
    ↓
Reference docs (references/)     Loaded by agents when needed. Detection rules,
                                 checklists, examples, how-to guides.
                                 Not visible in system prompt — agents discover
                                 them via skill body which mentions them.
    ↓
Agent definitions (.claude/agents/)  Defines which tools, model, and skills a
                                     subagent type gets. Minimal prompt.
```

**Key invariant:** Each piece of knowledge has exactly one canonical home. If a convention is in a skill, the review checklist references the skill's rule — it doesn't restate it. If a detection rule is in a checklist, the skill mentions the checklist exists — it doesn't inline the detection rule.

**Why this layering works:**
- CLAUDE.md → every agent pays the context cost → only put what every agent needs
- Skills → agents self-select by reading descriptions → topic-specific conventions
- Reference docs → agents load when inside a skill → detailed procedures too large for the skill body
- Agent definitions → define capabilities (tools, model, skills), not behavior → behavior comes from skills

## Key design decisions and what makes them work

### Skills over rules

`.claude/rules/` are path-triggered (load when Claude reads a matching file). `.claude/skills/` have name+description always visible, body on demand.

Skills win because:
- **Rules load too late.** They trigger on file reads, after the agent has already decided what to do. Skills are visible during planning.
- **Rules don't load for new files.** Creating a new `.tex` file may never trigger the tex rule. Skills are independent of file I/O.
- **Skills can be preloaded into subagents** via the `skills:` field in agent definitions. Rules depend on the subagent reading matching files.
- **Anthropic's platform direction** (as of 2026) documents skills extensively and doesn't mention rules.

### One generic review agent, not 12 specialized ones

What makes reviews work is the **methodology** (sequential checklist, one item at a time, record immediately), not agent specialization. That methodology lives in the review skill. Any generic agent following it produces good reviews.

The old 12-agent architecture had:
- 1,030 lines of agent definitions with duplicated methodology
- Convention changes requiring updates in 3 places (rule + agent + sometimes CLAUDE.md)
- No composability (couldn't run just anti-pattern checks without the full tex-style review)

The new architecture separates concerns:
- **What's correct** → convention skills (one canonical source)
- **How to detect violations** → checklist reference docs (one per review concern)
- **How to do a review** → review skill (methodology, output format, phase ordering)
- **What tools/model a reviewer gets** → agent definition (minimal, just capabilities)

### Detection rules separate from conventions

Convention skills say *what's correct*. Checklist reference docs say *how to find violations*. Separated because:
- **Different audiences.** Writing agents need conventions. Review agents need detection rules.
- **Different update frequency.** Conventions change when Jörn decides a rule. Detection rules change when we discover a new violation pattern.
- **Self-service.** The review skill lists which checklist applies to which concern. Subagents read what they need.

## Relationship to Anthropic best practices

Anthropic's guides are a good starting point but have gaps:
- They don't cover when to use skills vs rules vs agents vs reference docs in combination. We filled that gap empirically (see "Decision framework" above).
- They overstress prompt engineering and underweight structural decisions (where to put knowledge matters more than how to phrase it).
- They target a general audience. This project does academic pure math research — a niche with specific needs (mathematical verification workflows, proof review, notation consistency) that Anthropic's guides don't address.

**Rule of thumb:** Anthropic gives a good first guess. Compare with Jörn's guidance. If Jörn's guidance isn't written down for something you need, ask him.

## How to evaluate your changes

After modifying the meta-layer, check:
1. **No duplication.** Is the knowledge in exactly one place? Or did you create a second copy?
2. **Right layer.** Is it at the right visibility level for its audience?
3. **Discoverable.** Can an agent that needs this knowledge find it? (Follow the chain: CLAUDE.md → skill description → skill body → reference doc.)
4. **Lean CLAUDE.md.** Did you add to CLAUDE.md? Could it go in a skill instead?
5. **No instruction overload.** Did you increase the total instruction complexity for agents that don't need this knowledge?

## Known-incomplete areas

- **`meta-documentation` skill** has been split into focused skills: `meta-claudemd`, `meta-skills`, `meta-subagents`, `meta-hooks`, `meta-folder-layout`. The remaining `meta-documentation` is foundational analysis (taxonomy, failure modes, decision records).
- **`thesis-pre-review` skill** overlaps with the review workflow. May consolidate later.
- **MEMORY.md** has stale entries from earlier sessions. Not cleaned up — low priority.
- **Agent teams** are experimental and untested for reviews. Documented as an alternative but not the default.
