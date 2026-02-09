# CLAUDE.md — Architecture Decision Record

Why the rules in CLAUDE.md exist. Read this when a rule seems arbitrary or when considering changes.

## Decision authority framework

### Why "discuss-first" for GitHub issues?

**Failure mode (issue #12, Feb 2026):** Three agents attempted #12 over two days. Each read massive agent-written comments (posted under Jörn's account), treated them as authoritative, and either continued the brain-dump or stalled planning. No deliverable was produced. Agents posted ~1100 lines of unreviewed drafts as issue comments, posted a "plan" nobody agreed to, and edited the issue body without asking.

**Root cause:** Agents treated issue operations the same as code operations — act first, ask later. But issue edits have different economics:
- **Code**: cheap to verify (tests), easy to roll back (git). Low risk.
- **Issue edits**: expensive to verify (Jörn reading the text ≈ cost of writing it together), hard to roll back cleanly (downstream issues may reference the content, GitHub edit history is clunky).

**Alternative considered:** "Never edit issues without permission" — rejected because agents legitimately edit issues during triage. The problem isn't editing, it's editing without Jörn's input to catch directional errors.

**Decision:** Issue edits go in the "discuss-first" category. Agent proposes content in chat, Jörn steers, agent publishes.

### Why "when in doubt, default to discuss-first"?

**Failure mode:** Agents systematically over-classify actions as "act freely." The system prompt encourages autonomy and proactivity, which conflicts with CLAUDE.md guardrails. Agents that are unsure which category applies tend to guess "act freely" because it's the path of least resistance.

**Decision:** Explicit default. Jörn can override ad-hoc ("just do it"), but the override doesn't generalize to future sessions. This means agents err on the side of communication rather than action.

## Quality model

### Why "tests are necessary but not sufficient"?

**Failure mode (msc-viterbo, 2025):** The predecessor repo had agent-written tests that all passed. Known bugs:
1. HK2019 QP solver missed optima — returned plausible but wrong values
2. Trivialization formula was not a bijection
3. Billiard orbit validation only checked even-indexed segments
4. Pentagon capacity: 2.127 (wrong) instead of 3.441 (correct)

These bugs were undetected because agents wrote tests that verified internal consistency, not mathematical correctness. The tests checked "does the code do what the code does?" not "does the code compute the right mathematical quantity?"

**Root cause:** Goodhart's law. When agents write both code and tests, tests optimize for passing, not for correctness. Without external domain knowledge about what the correct values are and which edge cases matter, agents produce internally consistent but mathematically wrong systems.

**Decision:** Jörn provides domain knowledge: which test cases matter, what the correct values should be, what invariants to check. This is his primary quality contribution — not code review.

## Communication rules

### Why "silence is not confirmation"?

**Failure mode (#12, Feb 2026):** An agent proposed a plan, Jörn didn't respond (connection issues / was thinking / was typing a correction), agent proceeded as if the plan were approved, and published changes that needed to be reverted.

**Root cause:** The system prompt encourages agents to be autonomous. Combined with queued messages in CC web, agents interpret non-response as implicit approval. CC web has latency and connection drops that make real-time back-and-forth unreliable.

**Decision:** Explicit rule. If Jörn hasn't responded, ask again or move on. Never proceed as if approved.

### Why "agent time is cheap, Jörn's time is expensive"?

This is the fundamental economic constraint of the project. Agent compute costs cents per hour. Jörn's time is the bottleneck — he's writing a thesis with a March 2026 deadline.

**Derived rules:**
- Agents prepare before asking (gather context, try things, distinguish observations from inferences)
- Agents don't waste Jörn's time on questions they could answer by investigation
- But agents DO involve Jörn on decisions he can't delegate (mathematical direction, scope, what to test)

### Why "omit filler phrases" and "number items"?

**Failure mode:** Agents write long, polite responses. Jörn has to parse prose to find the decision points. On CC web with cut-off lines and connection issues, this is especially painful.

**Decision:** Efficient formatting. Numbered items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs.

## GitHub authorship

### Why emphasize "agent-written"?

**Failure mode (#12, Feb 2026):** Agent read comments posted under `JoernStoehler`'s account, assumed they were human-reviewed, and treated 900 lines of speculative analysis as authoritative direction. In reality, a previous agent had posted the comments.

**Root cause:** GitHub shows all content under Jörn's account. There's no visual distinction between Jörn-written and agent-written content. Agents default to trusting content from the "project owner."

**Decision:** Explicit warning. All GitHub content is agent-written. Trust the direction, verify the details.

## Subagent output

### Why "commit to branch, don't post as comments"?

**Failure mode (#12, Feb 2026):** An earlier version of CLAUDE.md said "Post agent output as GitHub issue comments." Agents followed this rule and posted 1100+ lines of unreviewed drafts as comments on #12. Future agents then read these comments and treated them as authoritative context, creating a feedback loop of noise.

**Alternative considered:** "Post as comments with an [UNREVIEWED] tag" — rejected because agents still treat tagged content as directional.

**Decision:** Subagent output returns via the Task tool. If it needs to persist, commit to the branch (goes through PR). Never post as issue comments.

## Clarity testing

### Why "test by USE, not by asking"?

**Failure mode:** An earlier version of CLAUDE.md said "run a Sonnet subagent with a targeted questionnaire to test whether a fresh agent can reproduce the intended understanding. Ask both for comprehension answers and for what's unclear or ambiguous." Agents followed this and asked subagents "is this clear?" or "what's unclear?" — subagents that misunderstood the content confidently answered "yes, it's clear" or identified irrelevant nitpicks while missing fundamental misunderstandings.

**Root cause:** Asking "is this clear?" tests the agent's confidence, not its comprehension. An agent that misunderstands will be confidently wrong.

**Decision:** Test comprehension by asking agents to USE the content (implement from a description, answer specific questions about an algorithm). Check whether their output matches intent. This catches misunderstandings that confidence-based checks miss.

## File structure

### Why a single CLAUDE.md?

**Previous state:** 8 files across 4 directories — root CLAUDE.md, crates/CLAUDE.md, experiments/CLAUDE.md, thesis/CLAUDE.md, archaeology/CLAUDE.md, docs/prompts/triage.md, docs/references/issue-lifecycle.md, .github/ISSUE_TEMPLATE/task.md. Total: 818 lines.

**Problems:**
- Work model was scattered across root CLAUDE.md sections (Agent workflow, Agent behavior rules, Communication with Jörn, GitHub authorship) — no single section gave the full picture
- Agents pieced together a mental model from fragments and got it wrong
- Duplicated content (build commands in root and crates/, proof rules in root and thesis/)
- More files = more surface area for inconsistency after edits

**Decision:** Consolidate into single 310-line CLAUDE.md. YAGNI — split later if a section grows too large for agents to process effectively.

## Style guide for CLAUDE.md edits

### Why this style guide exists

**Failure mode (Feb 2026):** Agents editing CLAUDE.md dropped adjectives and quantifiers during rewrites, treating word reduction as always positive. For example, "clear, specific, detailed, unambiguous, cognitive low-overhead" might become "high-quality" — losing the operational meaning of each word. Each adjective names a different quality bar; removing any one of them silently weakens the requirement.

**Root cause:** Agents treat prose as compressible text, where shorter = better. In a ~100k token context window, the marginal cost of one extra word is near zero. The marginal value of a qualifier that steers agent behavior is high. The cost-benefit analysis that justifies compression doesn't apply here.

**Alternative considered:** "Agents propose edits in chat, Jörn publishes" — rejected because git diff provides better review than chat (A/B comparison, inline editing, line references).

**Decision:** Agents edit CLAUDE.md directly on their branch. Jörn reviews via git diff in VS Code. To protect against information-destroying edits, the file follows these structural rules:

### Rule 1: One claim per bullet

Dense prose packs multiple claims that get lost when an agent rewrites a sentence. Each bullet or sentence states one claim.

**Example of the problem.** This paragraph contains 5 distinct claims:

> "Claude Code is okay at spotting implicit criteria imposed on a task's scope and acceptance criteria by other tasks and by Claude Code's capability limits and default habits."

The claims are: (a) Claude can spot implicit criteria, (b) its skill level is specifically "okay" (not bad, not good), (c) these criteria come from other tasks, (d) they also come from Claude's own capability limits, (e) they also come from Claude's default habits. An agent rewriting this sentence to "Claude Code has limited scope-detection ability" loses claims (c), (d), (e) and miscalibrates (b).

**The fix:** Break into atomic bullets where each claim is independently visible.

### Rule 2: Within-section ordering

Each section presents content in this order:

1. **Prerequisites** — what the reader needs to know to parse the rule (definitions, context)
2. **Rule** — the actual constraint, claim, or decision
3. **Motivation** — why the rule exists, what goes wrong without it
4. **Further material** — references, examples, elaboration

This ordering protects information during edits: an agent rewriting top-down encounters the rule first and is less likely to discard it. The motivation explains why it matters, so the agent must make a conscious choice to drop it. Further material is the cheapest to lose.

### Rule 3: Qualifier preservation

Every adjective narrows meaning. "Clear, specific, detailed, unambiguous" is not a synonym list — each word names a different quality bar:

- "clear" → the reader can parse it without re-reading
- "specific" → no hand-waving or generalities
- "detailed" → all steps included, nothing left implicit
- "unambiguous" → two readers arrive at the same understanding

When rewriting, check: does this rewrite preserve all constraints the original imposed? If a word seems redundant, it probably distinguishes this claim from a weaker claim that was also considered and rejected.

### Rule 4: Priority ordering

Clarity (the reader understands what we say) & unambiguousness (no variance in understanding) > correctness (it's actually what we want) > maintainability (edits are easy) >>> tokens (nearly unimportant at our context window scale).

Consequences:

- It's better to be slightly imprecise but totally clear than precisely correct but ambiguous.
- Using 50 extra words to prevent a misunderstanding is always worth it.
- Redundancy is welcome — the same rule stated in two places is more robust than a single statement.
- We are allowed to switch to different practices if they get us 90% of the behavior we want but are actually clear and easier to write.

### Rule 5: Concrete over abstract

- "Run `cargo test` from `crates/`" not "run the tests"
- "Only Jörn merges to main" not "merges require human approval"
- "Claude Code is only okay at it" not "Claude Code has limited capability"

The concrete version resists misinterpretation. The abstract version invites the reader to substitute their own interpretation.

### Rule 6: Decision trees over prose principles

When behavior depends on conditions, use if-then structure. Prose principles require the reader to interpret them, which introduces variance.

**Example.** Instead of "Agents should balance autonomy with communication," write:
- Act freely: writing code, investigation, committing to the working branch
- Discuss first: GitHub issue edits, scope changes
- Never without instruction: destructive operations, merging to `main`

### Rule 7: Completeness check (what / why / how)

After writing or editing a section, verify:
- Did we say what this IS? (the thing, the rule, the fact)
- Did we say why it EXISTS? (what problem it solves, what goes wrong without it)
- Did we say how it's USED? (what an agent should concretely do)

This is a checklist for completeness, not an organizational template — the "how" can have its own "why" and they don't need to be separated.
