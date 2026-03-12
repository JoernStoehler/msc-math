# Meta-Documentation Architecture Rationale

Why the meta-documentation (CLAUDE.md, skills, agents, memory) is structured as it is.
Written March 2026 after an audit and refactor session.

## The problem

Agents need instructions to produce correct work, but:
1. Instructions loaded too early get forgotten during long sessions (context decay)
2. Instructions loaded too late miss the planning phase (agents already committed to an approach)
3. Too many instructions at once → agents ignore some ("instruction overload")
4. Duplicated instructions across files → maintenance burden, drift, contradictions
5. Agents don't know what they don't know — they won't search for instructions they don't realize exist

## Why skills over rules

`.claude/rules/` files are path-triggered: they load when Claude reads a file matching a glob pattern (e.g. `**/*.tex`). `.claude/skills/` files have their name+description always visible in the system prompt, with the full body loaded on demand.

We chose skills because:

**Rules load too late.** Rules trigger on file reads, which happen *after* the agent has already decided what to do. By the time the rule about "geometric definitions first" loads, the agent may have already written a coordinate-first definition. Skills are visible during planning because the agent sees the description and can load the body before committing.

**Rules don't load for new files.** When an agent creates a new `.tex` file (rather than editing an existing one), the rule may never trigger because no matching file was read. Skills are independent of file I/O.

**Skills support progressive disclosure.** The name+description (always visible, ~10 tokens each) tells the agent "this exists and is about X." The agent loads the body only when relevant. This is the right tradeoff for a project where most agents need a subset of conventions, not all of them.

**Anthropic best practices moved away from rules.** The Claude Code best practices page (2026) documents skills extensively and doesn't mention rules. Rules may be deprecated.

**Skills can be preloaded into subagents.** Agent definitions have a `skills:` field that forces skill bodies into the subagent's context at startup. Rules don't have an equivalent mechanism — they depend on the subagent reading matching files.

## Why one generic review agent instead of 12 specialized ones

The old architecture had 12 review agents (review-tex-style, review-rust-tests, etc.), each ~80 lines with inline checklists. We replaced them with 1 generic review agent + 1 review skill + 8 checklist reference docs.

**Maintenance cost.** 12 agents × 80 lines = 1,030 lines of agent definitions. Many contained duplicated methodology (sequential checklist approach, output format). A convention change required updating the rule file AND the review agent AND sometimes CLAUDE.md. Now: conventions live in skills, detection rules live in checklist reference docs, methodology lives in the review skill. Each piece has one canonical home.

**Composability.** The old agents were monolithic: "review-tex-style" combined build checks, environment checks, comment checks, figure checks, and anti-pattern checks into one agent. You couldn't run just the anti-pattern checks. The new architecture: spawn a subagent with any combination of concern + files + checklists.

**Context efficiency.** The old 12 agents all loaded the same convention content via rules auto-loading (~2-3k tokens each). The new agent preloads all skills once. Subagents read only the checklist reference docs they need.

**The generic agent tested successfully.** 4 parallel review runs on the actual repo found real issues with zero false positives. The methodology (sequential checklist) is what makes reviews work, not the agent specialization. The methodology is in the review skill; it works regardless of which concern is being reviewed.

## Architecture layers

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

**Why this layering:**
- CLAUDE.md → every agent sees it → only put what every agent needs
- Skills → agents self-select → put topic-specific conventions here
- Reference docs → agents load within a skill → put detailed procedures, examples, detection rules here (these are too large/specific for the skill body itself)
- Agent definitions → define capabilities, not behavior → behavior comes from skills

## Why detection rules are separate from conventions

Convention skills say *what's correct* (e.g. "use `\begin{algorithm}` environments"). Detection rules say *how to find violations* (e.g. "grep for `\noindent\textbf{Algorithm}`"). These are separated because:

1. **Different audiences.** Writing agents need conventions. Review agents need detection rules. Loading detection rules into a writing agent wastes context and causes confusion.
2. **Different update frequency.** Conventions change when Jörn decides a new rule. Detection rules change when we discover a new violation pattern. These evolve independently.
3. **Self-service model.** The review SKILL.md lists which checklist doc applies to which concern. The subagent reads the checklist it needs. No coordination from the main agent required.

## Decisions deferred or known-incomplete

- **`writing-conventions` skill** (292 lines) is the largest skill and could be split. Not done yet because it's rarely loaded and works as-is.
- **`thesis-pre-review` skill** overlaps with the review workflow. May consolidate later.
- **MEMORY.md** has stale entries from earlier sessions. Not cleaned up — low priority.
- **Agent teams** are experimental and untested for reviews. Documented as an alternative but not the default.
