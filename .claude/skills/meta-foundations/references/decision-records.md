# Decision Records

Why specific rules and architecture choices exist. Read when a rule seems arbitrary, when considering changes, or when evaluating whether a proposed change conflicts with lessons already learned.

## Skills over rules

`.claude/rules/` are path-triggered (load when Claude reads a matching file). `.claude/skills/` have name+description always visible, body on demand.

Skills win because:
- **Rules load too late.** They trigger on file reads, after the agent has already decided what to do. Skills are visible during planning.
- **Rules don't load for new files.** Creating a new `.tex` file may never trigger the tex rule. Skills are independent of file I/O.
- **Skills can be preloaded into subagents** via the `skills:` field in agent definitions. Rules depend on the subagent reading matching files.
- **Anthropic's platform direction** (as of 2026) documents skills extensively and doesn't mention rules.

## One generic review agent, not 12 specialized ones

What makes reviews work is the **methodology** (sequential checklist, one item at a time, record immediately), not agent specialization. That methodology lives in the review skill. Any generic agent following it produces good reviews.

The old 12-agent architecture had:
- 1,030 lines of agent definitions with duplicated methodology
- Convention changes requiring updates in 3 places (rule + agent + sometimes CLAUDE.md)
- No composability (couldn't run just anti-pattern checks without the full tex-style review)

The current architecture separates concerns:
- **What's correct** → convention skills (one canonical source, including detection patterns)
- **How to do a review** → review skill (orchestration, phase ordering, spawn mapping)
- **What tools/model a reviewer gets** → agent definition (minimal, just capabilities)

Good conventions are verifiable — the convention IS the review specification. Separate checklists were removed because they restated conventions and drifted. Detection patterns that aren't conventions (e.g., "look for unargued claims") live inline in dedicated agents (e.g., `math-review`).

## "Discuss-first" for issue edits and scope changes

**Failure mode (issue #12, Feb 2026):** Three agents attempted #12 over two days. Each read massive agent-written comments (posted under Jorn's account), treated them as authoritative, and either continued the brain-dump or stalled planning. No deliverable produced. ~1100 lines of unreviewed drafts posted as issue comments. Future agents treated these as authoritative, creating a feedback loop.

**Root causes:**
- Agents treated issue edits the same as code edits — but issues are expensive to verify and hard to roll back
- Agents interpreted Jorn's silence as approval
- GitHub shows all content under Jorn's account — no visual distinction between Jorn-written and agent-written

**Decisions:** Issue edits -> discuss-first. Silence != confirmation. Subagent output -> commit to branch, never post as comments.

## Tests are necessary but not sufficient

**Failure mode (msc-viterbo, 2025):** Predecessor repo had agent-written tests that all passed. Known bugs:
1. HK2019 QP solver missed optima — returned plausible but wrong values
2. Trivialization formula was not a bijection
3. Billiard orbit validation only checked even-indexed segments
4. Pentagon capacity: 2.127 (wrong) instead of 3.441 (correct)

**Root cause:** Goodhart's law. When agents write both code and tests, tests optimize for passing, not for correctness.

**Decision:** Jorn provides domain knowledge: which test cases matter, what correct values are, what invariants to check.

## Test comprehension by USE, not by asking

**Failure mode:** Agents asked subagents "is this clear?" — subagents that misunderstood confidently answered "yes."

**Decision:** Test comprehension by asking agents to USE the content (implement from a description, answer specific questions). Check whether their output matches intent.

## Why a single CLAUDE.md (not per-directory)

**Previous state:** 8 files across 4 directories (818 lines total). Agents pieced together mental models from fragments and got them wrong. Duplicated content drifted.

**Decision:** Single file, split by topic. Skills for progressive disclosure. Tried moving topic sections to skills-only — agents forgot to load them. Topic sections must stay in CLAUDE.md.

## How to evaluate meta-layer changes

After modifying the meta layer, check:
1. **No duplication.** Is the knowledge in exactly one place? Or did you create a second copy?
2. **Right layer.** Is it at the right visibility level for its audience?
3. **Discoverable.** Can an agent that needs this knowledge find it? (Follow the chain: CLAUDE.md → skill description → skill body → reference doc.)
4. **Lean CLAUDE.md.** Did you add to CLAUDE.md? Could it go in a skill instead?
5. **No instruction overload.** Did you increase the total instruction complexity for agents that don't need this knowledge?

## HTML comments in CLAUDE.md

CLAUDE.md supports `<!-- comments -->` that are NOT auto-injected into agent context but ARE visible via Read/Edit. Since Edit requires a prior Read, agents editing CLAUDE.md will always see comments.

**Good for** (editor-facing metadata):
- Maintenance notes next to rules ("added after incident X")
- Historical context only editors need
- Inline rationale too small to justify loading a skill

**Not good for:**
- Anything all agents need to follow — must be in visible text.
