# ALL ONBOARDING MATERIAL — MERGED FOR REWRITE
# Each section shows the source file path.
# Total: 7 files, ~500 lines.

# ============================================================
# SOURCE: CLAUDE.md (root)
# ============================================================

# Master Thesis: Probing Viterbo's Conjecture

Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak, Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – March 2026

## [aspirational] End state

This repo is a completed master thesis with:
- A printed-quality LaTeX document in `thesis/`
- A high-performance Rust library for symplectic geometry on polytopes in `crates/`
- A reproducible Python pipeline in `experiments/` that starts from zero data and produces all figures and tables
- All code tested, all results reproducible

## Repo structure

```
CLAUDE.md          This file (all agents read this)
thesis/            LaTeX thesis document
  CLAUDE.md        Thesis-specific conventions
crates/            Rust workspace (cargo build/test from here)
  CLAUDE.md        Rust-specific conventions, testing philosophy, math docs
  geom2d/          2D symplectic geometry primitives
  geom4d/          4D convex geometry, polytopes, symplectic structures
  hk2017/          Haim-Kislev 2017 algorithm (all polytopes, exponential cost)
  billiard/        Billiard algorithm (Lagrangian products only, fast)
  tube/            Tube algorithm (no Lagrangian 2-faces, moderate cost)
  datasets/        Dataset generation orchestration
experiments/       Python scripts consuming Rust-generated datasets
  CLAUDE.md        Experiments-specific conventions
  scripts/         Independent .py scripts
  data/            gitignored — populated by Rust pipeline
  figures/         gitignored — populated by Python scripts
papers/            arxiv .tex sources for reference (HK2017, CH2021, HK-O 2024, ...)
```

## Mathematical context

We compute the EHZ capacity (minimum action of generalized Reeb orbits) for convex polytopes in R^4. By a theorem of Haim-Kislev 2017, there exists a minimum-action orbit that is piecewise linear, uses pure facet Reeb vectors, and visits each facet on a contiguous time interval. This reduces the problem to a finite combinatorial search.

Viterbo's conjecture: sys(K) = c_EHZ(K)^2 / (2 vol(K)) <= 1 for all convex bodies K.
Haim-Kislev 2024 (Annals) gave a 10-facet counterexample with sys > 1.
We probe the conjecture by computing sys across large polytope datasets and looking for patterns.

## Repo invariants

These are true about the repo right now and must remain true:

- `cargo test` passes from `crates/` with zero failures

## Agent workflow

Sessions follow: **scope → plan → implement → review → push** · · · Jörn: **PR → merge** · · · **triage**

- **scope**: Critically evaluate the assignment. Push back on contradictions, gaps, suboptimal reversible decisions. Situate in the broader thesis context.
- **plan**: Decompose into steps. Compare to conventions. Play through the plan and notice gaps.
- **implement**: Execute the plan. Trust signatures from the plan; react to feedback from the repo.
- **review**: Re-read the result as a whole. Compare code to plan and to scope. Catch drift, gaps, and mismatches across abstraction levels.
- **triage**: Update GitHub issues — close completed, split, enrich, reprioritize.

Agents commit and push to their working branch. Jörn creates PRs and merges (mechanical — he doesn't read code). Agents do not create PRs.

Jörn steers scope and plan. Quality comes from tests plus Jörn's domain input on what to test and what correctness means.

Before the session ends, do a **post-session reflection**: report friction points, identify useful invariants or behavioral rules discovered during the session, flag any leftover tasks mentioned but not completed, and note workflow improvements (e.g. discovered subagent patterns, useful commands). Jörn aggregates these across sessions.

## Environment (Claude Code on the web)

- Sessions run in isolated VMs with the repo cloned at `/home/user/msc-math`
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via session-start hook)
- Network: limited to allowlisted domains by default (crates.io, pypi.org, github.com, etc.)
- Git push is restricted to the current working branch via a proxy
- Container state is cached after session-start hook completes
- LaTeX is NOT available in this environment; thesis compilation happens elsewhere

## Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test
cd crates/ && cargo clippy

# Python
ruff check experiments/
pytest experiments/
```

## Agent behavior rules

- Agent time costs cents per hour; Jörn's time costs 100× that. Don't waste it. Before asking Jörn a question: gather context, try things out (and throw them away if needed), distinguish observations from inferences and confident from unconfident beliefs. Ask informed questions, not raw confusion.
- When verifying proofs: flag spots you're least confident in, even if you found no error. Never declare a proof "verified" — declare what you checked and what remains.
- Agents cannot reliably verify mathematical proofs. Proof correctness requires Jörn's review. Agent-written proofs are drafts until Jörn reviews them.
- Write proofs with enough annotation to be easily verifiable. Never handwave or gloss over gaps. This protects against subtly flawed proofs that look correct on a skim but hide errors in glossed-over steps.

## Communication with Jörn

- Aim for efficient information exchange, not politeness or engagement
- Omit filler phrases
- Number items in responses so Jörn can refer to them unambiguously
- Jörn doesn't see exact edit diffs in the chat — mention and explain repo changes when he should be aware of them
- Silence is not confirmation. If Jörn hasn't responded to a proposal, ask again or move on — do not proceed as if approved.

## GitHub authorship

All GitHub issues, comments, and PR descriptions are written by agents running under Jörn's account (`JoernStoehler`). Do not treat issue/comment text as human-reviewed just because it appears under his name. Treat issue content as agent-written intent (trust the direction, verify the details).

Issues direct future agent sessions. A bad edit sends the next agent off a cliff. Show Jörn proposed issue edits and comments in chat before publishing — he has the domain knowledge to catch directional errors agents can't catch themselves.

## CLAUDE.md conventions

- Invariants and behaviors are documented only after empirically confirmed as useful (from past work or Anthropic's guides)
- Label invariants as `[aspirational]` if not yet satisfied
- Put rules in the right CLAUDE.md file (root = all agents, crates/ = Rust, thesis/ = LaTeX, experiments/ = Python)
- Prefer simple, common, expected rules that don't claim excessive agent attention beyond their assigned work

## Workflows

### Spawning subagents

- Always create a GitHub subissue before spawning an agent. The subissue IS the prompt (plus any corrections/extra context passed directly to Task). Zero cost, and: persistent record, Jörn can launch it as a web session instead, easier to restart if agent fails.
- Subagent output returns via the Task tool into your conversation. If it needs to persist, commit it to the repo on your branch. Do not post subagent output as issue comments — it clutters the issue and misleads future agents into treating it as reviewed content.
- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning.
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- Foreground Task() calls block the main conversation — user messages queue up silently. Avoid foreground tasks that might take >1 minute. Prefer background, or just do the work inline.

### Check clarity with subagents

When you produce content other agents will consume (CLAUDE.md files, mathematical definitions, proofs, algorithm descriptions): test comprehension by having a fresh Sonnet subagent attempt to USE the content (e.g. "implement from this description" or "answer these specific questions about the algorithm"). Check whether their output matches your intent. Do not ask "is this clear?" — an agent that misunderstands will confidently say yes.

# ============================================================
# SOURCE: crates/CLAUDE.md
# ============================================================

# Rust Crates

## Build and test

```bash
cd crates/
cargo build
cargo test
```

All tests must pass before committing.

## Crate dependency graph

```
geom2d ─────────────────────────────────┐
  └─> geom4d                            │
        ├─> hk2017    (+ geom2d)        │
        ├─> billiard  (+ geom2d)        │
        ├─> tube      (+ geom2d)        │
        └─> datasets  (+ hk2017, billiard, tube)
```

## Three capacity algorithms

| Crate     | Applies to                        | Cost                    |
|-----------|-----------------------------------|-------------------------|
| hk2017    | All polytopes                     | Exponential in #facets  |
| billiard  | Lagrangian products only          | Fast                    |
| tube      | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

## Conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`) — see testing philosophy below
- Functional programming style
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing

## Mathematical documentation

- Definitions, lemmas, and proofs live in the Rust crates as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream and synced independently
- Quality bar: specific, correct, detailed, and clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing function bodies

## Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

# ============================================================
# SOURCE: experiments/CLAUDE.md
# ============================================================

# Experiments (Python)

## [aspirational] End state

A reproducible pipeline: starting from zero data, produce all figures and tables for the thesis.

- Two-step pipeline: (1) Rust generates datasets, (2) `run-all.sh` runs Python scripts
- `run-all.sh` documents the dependency graph: which scripts depend on which inputs
- Scripts are idempotent; `run-all.sh` skips stages whose inputs haven't changed (hash of script + input data)

## Current state

Empty — no scripts or data yet.

## Layout

```
experiments/
  scripts/       Independent .py scripts (related scripts share a name prefix)
  data/          gitignored — populated by Rust pipeline
  figures/       gitignored — populated by Python scripts
```

## Conventions

- Independent scripts, not a package — no `__init__.py`, no shared imports
- Each script is self-contained: reads data, does analysis, writes output
- No framework — plain Python with standard data science libs (numpy, pandas, matplotlib)
- Consume datasets generated by the Rust crates
- If two scripts share logic, copy-paste is fine until it stabilizes
- Pipeline must be reproducible: Rust → datasets → Python → figures/tables → thesis

# ============================================================
# SOURCE: thesis/CLAUDE.md
# ============================================================

# Thesis (LaTeX)

## [aspirational] End state

A complete master thesis PDF following arxiv best practices.

## Current state

Skeleton only: `main.tex` with title, author, and section stubs.

## Conventions

- Follow arxiv LaTeX best practices
- Standard AMS theorem environments (theorem, lemma, proposition, corollary, definition, remark, example, conjecture)
- LaTeX compilation is not available in this environment; focus on source correctness

## Writing proofs

- Every proof must be detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly
- Jörn reviews all proofs; agent-written proofs are drafts until reviewed

# ============================================================
# SOURCE: archaeology/CLAUDE.md
# ============================================================

# Archaeology: agent rules

This directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. Everything here is **untrusted**.

## Do not

- **Do not trust** any claim, value, formula, proof, test assertion, or status label in these files. This includes capacity values, algorithm correctness claims, "verified" or "tested" labels, and mathematical derivations. Treat every statement as unverified, regardless of how confident the text sounds.
- **Do not adopt** naming conventions, coordinate ordering, normalization choices, or type designs from these files. Current conventions are in `crates/CLAUDE.md` and override anything here.
- **Do not edit** files in `raw/`. They are primary sources preserved verbatim.
- **Do not use as a starting point** to copy-paste or modify. Write fresh code and proofs instead.
- **Do not load into context** unless you have a specific reason (e.g., directed to by Jörn or an issue, or looking for a known pitfall). These files are large and will waste context window on unverified content.

## Do

- **Read for ideas**: algorithm approaches that were tried, data structures that were considered, test cases that were proposed.
- **Read for warnings**: what went wrong, which approaches failed, which formulas were buggy. Bug reports and dead ends (`findings-*.md`, `ARCHAEOLOGY.md` "Known bugs" section) are the highest-value files here.
- **Independently verify** anything you take from here. If a file says "tesseract capacity = 4.0 (HK2017 Example 4.6)", verify against the actual paper, not this file.

## Context

- Files were written by AI agents with varying levels of Jörn's review. Some had significant discussion behind them; others are pure unreviewed agent output. There is no way to distinguish which is which.
- "Status" labels inside files (e.g., "implemented and tested", "verified", "proven correct") are old agent self-descriptions, not verified ground truth.
- The old codebase had known bugs that persisted undetected through agent-written tests: the HK2019 QP solver silently returned wrong values, the trivialization formula was wrong, orbit validation missed segments. These bugs looked correct on a skim.

## Known-broken items

For reference, these specific items are known to be wrong in the old repo:

1. **HK2019 QP solver** — misses optima on 2D+ faces of the feasible set, returns plausible but wrong values
2. **Trivialization formula** — `tau_n(V) = (<V,Jn>, <V,Kn>)` is not a bijection on 2-face tangent spaces; was later fixed
3. **Billiard orbit validation** — only checked even-indexed segments, missed bounce transitions; pentagon returned 2.127 instead of 3.441
4. **Triangle x triangle discrepancy** — billiard returns 3.0, HK2017 returns 1.5; unresolved at time of archival
5. **Normalization convention mismatch** — some files use `sys = c^2/(2*vol)`, others use `sys = c^2/(4*vol)`

## Structure

- `raw/docs/` — 51 recovered documentation files (specs, thesis drafts, proofs, bug reports, literature summaries)
- `raw/code/` — 12 recovered Rust source files (algorithm implementations, flattened from three subdirectories)
- `raw/tests/` — 23 recovered Rust test files
- `raw/ARCHAEOLOGY.md` — index from the source branch with tables, provenance info, and known bugs
- `INDEX.md` — per-file metadata: type tag, origin, one-line description

# ============================================================
# SOURCE: docs/prompts/triage.md
# ============================================================

# Project board review ("triage")

Reference material for sessions where the task is to review and maintain the GitHub issue board.

## What a board review involves

The issue board should reflect reality after the session. Concretely:

1. **Audit** — Read all open issues. Check recent merges and closed issues. Compare what the board says vs. what the repo contains.
2. **Close** — Issues whose deliverable is already in main.
3. **Capture** — Work implied by the project goals or by recently completed work, but not yet tracked as an issue.
4. **Refine** — Issue bodies that are stale, incomplete, or don't match conventions. Rewrite using the issue template (`.github/ISSUE_TEMPLATE/task.md`).
5. **Prioritize** — Given dependencies and thesis timeline, what should be worked on next?
6. **Prepare** — Make top-priority issues session-ready: all template sections filled, open questions resolved, label → `approved`.

These steps don't need to happen in strict order — auditing often reveals things to capture or close, refining often surfaces open questions that change priority.

## Starting context

An agent doing board review needs broad, shallow context rather than deep knowledge of one topic:
- All open issues (titles, labels, bodies)
- Recently closed issues and merged PRs (what changed since last review)
- Current codebase state (what exists, what's a stub)
- The issue template and lifecycle doc (`docs/references/issue-lifecycle.md`)

## Decision authority

Agent's call:
- Reading and summarizing issue state
- Proposing closures, new issues, edits
- Writing/rewriting issue bodies to match conventions
- Running subagent clarity checks on refined issues

Jörn's call:
- Whether an issue is worth pursuing
- Scope boundaries (what's in, what's out)
- Priority order
- Labeling issues `approved`

## Workflow

Present findings in batches — a prioritized list of proposed actions that Jörn can approve, reject, or steer. Don't drip-feed one issue at a time; the overhead of context-switching between issues is lower when they're presented together.

For each issue being refined: check against the authoring guidelines in the issue template (false claims, over-constraining, misleading confidence, unclear wording, process misrepresentation).

When capturing new issues, a rough draft with just Goal and a few notes is fine — refinement happens iteratively across sessions, not all at once.

## Operational notes

Useful starting sequence: `gh issue list --state open`, `gh issue list --state closed --limit 10`, then read each open issue body. Comparing issue claims against actual repo state (what files exist, what's a stub) catches stale issues quickly.

Read issue bodies, not just titles — titles can be stale or misleading after edits.

Subagent clarity checks (Sonnet) work well for refined issues: a fresh agent reads the issue and answers targeted comprehension questions. Catches ambiguities that the author is blind to.

## Writing for other agents

Content that agents will consume (issue bodies, specs, CLAUDE.md entries) benefits from:
- **Grounded over speculative** — state what happened or what exists, not what might be useful
- **Knowledge over instructions** — inform, don't command. Agents have their own task instructions
- **Skimmable over comprehensive** — clear headers, so readers with different tasks can skip irrelevant sections
- **Escaped behavior rules** — a `## Workflow` header is enough signal that the content is contextual, not a directive

These principles apply to issue bodies too, not just reference docs.

## Creating new prompt files

Part of triage is noticing when a recurring session type would benefit from a reference prompt — the way this file exists for board review sessions. Examples: a proof-writing prompt, an implementation prompt, a literature-extraction prompt.

**How to produce a good prompt file:**

1. **Taboo the name.** Before writing, define the session type without its shorthand label. "Triage" → "review the project board against repo state, update it so top items are session-ready." This forces you to identify the concrete steps rather than writing vague instructions.

2. **Structure by reader questions.** Order sections by what a reader asks in sequence: "What is this?" → "What do I need to know?" → "What can I decide vs. what's Jörn's call?" → "How do we interact?" → "What goes wrong?" Each section answers one question.

3. **Write from experience, not imagination.** Every pitfall, operational tip, and workflow note should come from something that actually happened in a session. Speculative advice tends to be vague or wrong. If no sessions of this type have happened yet, keep the file short — just the step decomposition and decision authority — and extend it after the first session.

4. **Test with a subagent.** Have a Sonnet agent read the prompt and answer: "What would you do first? What decisions are yours? What's unclear?" This catches gaps the author is blind to.

5. **Aim for activation, not instruction.** Agents already have good habits from training. The prompt's job is to activate the right habits and provide project-specific context — not to teach the agent how to think. "Check against the authoring guidelines in the issue template" activates an existing skill; "Make sure every sentence is clear and unambiguous" is generic advice the agent already follows.

New prompt files go in `docs/prompts/`. Mention them in relevant CLAUDE.md files only if agents in that scope routinely need them.

## Known pitfalls from past sessions

- Claiming relationships between components without verifying (e.g. "X determines Y", "X is a specialization of Y"). Check before asserting.
- Revising an issue body many times before Jörn has seen any version. Write one complete draft, link it, get feedback.
- Presenting tool output as if Jörn can see it. He sees only assistant text messages — present substance in prose or link to GitHub URLs.
- Asking questions that assume Jörn already read something. Either tell him it's ready to review, or ask only questions needed before writing.

# ============================================================
# SOURCE: .github/ISSUE_TEMPLATE/task.md
# ============================================================

---
name: Task
about: A candidate task believed to constitute progress toward the thesis (#1)
---

<!--
LIFECYCLE — full description with examples: docs/references/issue-lifecycle.md

  created (label: draft)
    Issue captured from a spark — an idea that came up during triage, a session,
    or any conversation. Most sections may be empty or rough. That's fine.
    The issue exists so the idea isn't lost.

  → refined via edits (label: draft)
    Over one or more triage sessions, sections get filled in and sharpened.
    Open questions get resolved and their answers flow into other sections.
    Facts and claims get verified. Scope gets negotiated with Jörn.

  → approved (label: approved)
    Jörn reads the issue and labels it "approved". This means:
    the goal is worth pursuing, the scope is appropriate, the deliverable
    is clear, and the open questions are resolved (or non-blocking).
    From this point, the issue IS the prompt for an agent session.

  → agent session (label: in-progress)
    Agent + Jörn discuss scope, plan, implement, review, push.
    Agent works on a feature branch. See root CLAUDE.md for session workflow.

  → PR + merge
    Jörn creates PR, reviews, merges to main.

  → closed (label: done)
    Issue closed. Follow-up ideas captured as new issues during triage.

AUTHORING GUIDELINES

  These are the known failure modes when writing issues. Guard against them:

  - Unclear or ambiguous wording. Don't sacrifice clarity for brevity.
    Prefer an extra sentence over a vague word. If a term could mean
    two things, say which one you mean.

  - Misleading confidence signals. If something is unreviewed or uncertain,
    mark it explicitly — and do so for EVERY such item, not just some.
    Labeling one item "unreviewed" implies the others ARE reviewed.

  - False facts. Don't claim relationships between concepts unless verified.
    Don't state that X determines Y when actually X, Y, and Z all contribute.
    Don't call one thing a "specialization" of another unless it literally is.

  - Misrepresenting process. Don't omit stages the session agent will go
    through. Don't claim something is approved when it isn't. Represent
    the actual state of decisions accurately.

  - Over-constraining implementation. Don't prescribe file names, file counts,
    section structures, or other decisions the agent can trivially make
    during implementation. Constrain only what has external consequences
    (API surfaces, conventions from CLAUDE.md, mathematical correctness).
-->

## Goal

<!-- What this task achieves for the thesis. Short — a sentence or two.
     This is the "what" at the highest level. -->

## Background

<!-- Domain knowledge a reader needs for this issue to make sense.
     Concepts, definitions, theorems, prior work. Link to papers, files,
     issues for deeper reading — don't repeat their content here.
     This section answers: "what do I need to UNDERSTAND?" -->

## Context

<!-- Why this task constitutes progress toward the thesis (#1).
     How it connects to parent issues and the dependency graph.
     What completing this unblocks or improves.
     Desired benefits and anticipated risks to the project.
     This section answers: "why should we DO this?" -->

## Deliverable

<!-- What the agent produces. Describe the substance, not the form —
     the agent decides files, structure, and commits.
     What is the interaction surface with the rest of the project?
     What downstream agents or code will consume this deliverable? -->

## Scope

<!-- Agreed-upon boundaries. What's in, what's out, and why.
     Each exclusion should have a reason (out of scope because X,
     deferred to issue #Y, not needed because Z).
     This section prevents scope creep during implementation. -->

## Sources

<!-- Where to find information needed for implementation.
     Papers, code, specs, existing implementations.
     Flag trustworthiness: is this a reviewed paper, an untrusted
     archaeology file, a known-broken implementation?
     (Background = read to understand. Sources = read to implement.) -->

## Acceptance criteria

<!-- How to know the task is done. Each criterion should be:
     - Measurable: an agent or Jörn can unambiguously check it
     - Motivated: why this criterion matters (what goes wrong without it)

     Two kinds:
     External — the deliverable serves the project. Downstream agents
       and code can consume it. It integrates correctly.
     Internal — quality bar for long-term project health. Clarity,
       correctness, review requirements per crates/CLAUDE.md.
       E.g.: "proofs are drafts until Jörn reviews" is an internal
       criterion motivated by agents' inability to verify math reliably. -->

## Notes

<!-- Preliminary findings from scoping and triage sessions.
     Known risks inside scope. Suggested sub-issues worth considering.
     Anything discovered during refinement that the session agent
     should know about. Can be empty for fresh issues. -->

## Open questions

<!-- Uncertainties, decisions that need Jörn's input, dependencies
     on other issues. Resolve via edits as answers emerge — move
     answers into the appropriate section above.
     Once this section is empty and Jörn approves, the task is
     ready for assignment. -->
