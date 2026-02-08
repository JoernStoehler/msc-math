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
crates/            Rust workspace (cargo build/test from here)
  geom2d/          2D symplectic geometry primitives
  geom4d/          4D convex geometry, polytopes, symplectic structures
  hk2017/          Haim-Kislev 2017 algorithm (all polytopes, exponential cost)
  billiard/        Billiard algorithm (Lagrangian products only, fast)
  tube/            Tube algorithm (no Lagrangian 2-faces, moderate cost)
  datasets/        Dataset generation orchestration
experiments/       Python scripts consuming Rust-generated datasets
  scripts/         Independent .py scripts
  data/            gitignored — populated by Rust pipeline
  figures/         gitignored — populated by Python scripts
papers/            arxiv .tex sources for reference (HK2017, CH2021, HK-O 2024, ...)
archaeology/       Recovered files from abandoned predecessor repo (untrusted)
docs/prompts/      Reference prompts for recurring session types
```

## Mathematical context

We compute the EHZ capacity (minimum action of generalized Reeb orbits) for convex polytopes in R^4. By a theorem of Haim-Kislev 2017, there exists a minimum-action orbit that is piecewise linear, uses pure facet Reeb vectors, and visits each facet on a contiguous time interval. This reduces the problem to a finite combinatorial search.

Viterbo's conjecture: sys(K) = c_EHZ(K)^2 / (2 vol(K)) <= 1 for all convex bodies K.
Haim-Kislev 2024 (Annals) gave a 10-facet counterexample with sys > 1.
We probe the conjecture by computing sys across large polytope datasets and looking for patterns.

## How we work

### Roles

**Jörn** provides mathematical domain knowledge, steers scope, and makes judgment calls that require domain expertise. He does not read code. PR creation and merge are mechanical steps — he clicks buttons, not reviews diffs.

**Agents** write all code, tests, documentation, GitHub issues, and comments. Agents are good at writing correct Rust, structuring tests, and mechanical execution. Agents are bad at knowing *what* to test (which cases matter mathematically), noticing scope drift, and relating deliverables to the broader thesis.

**Quality model**: Tests are necessary but not sufficient. If agents write both code and tests without external input, Goodhart's law applies — tests optimize for passing, not for correctness. Jörn breaks this loop by providing domain knowledge: which test cases matter, what the correct values should be, what invariants to check. Without his input, agents produce internally consistent work that doesn't serve the project.

### Session workflow

Sessions follow: **scope → plan → implement → review → push** · · · Jörn: **PR → merge** · · · **triage**

- **scope**: Critically evaluate the assignment. Push back on contradictions, gaps, suboptimal reversible decisions. Situate in the broader thesis context.
- **plan**: Decompose into steps. Compare to conventions. Play through the plan and notice gaps.
- **implement**: Execute the plan. Trust signatures from the plan; react to feedback from the repo.
- **review**: Re-read the result as a whole. Compare code to plan and to scope. Catch drift, gaps, and mismatches across abstraction levels.
- **triage**: Update GitHub issues — close completed, split, enrich, reprioritize.

Agents commit and push to their working branch. Jörn creates PRs and merges. Agents do not create PRs.

Before the session ends, do a **post-session reflection**: report friction points, identify useful invariants or behavioral rules discovered during the session, flag any leftover tasks mentioned but not completed, and note workflow improvements (e.g. discovered subagent patterns, useful commands). Jörn aggregates these across sessions.

### Decision authority

When to act vs. when to ask depends on rollback cost and verification cost:

**Act freely** — cheap to verify, easy to roll back:
- Writing and editing code (git handles rollback; tests verify)
- Investigation, research, trying things out and throwing them away
- Committing and pushing to the working branch

**Act, then Jörn verifies** — cheap to verify, moderate risk:
- Attempts where agent self-verification is reliable and Jörn's check is fast
- The attempt itself provides value (e.g. a draft that's faster to correct than to discuss upfront)

**Discuss with Jörn first** — expensive to verify or hard to roll back:
- GitHub issue edits (verification cost ≈ cost of writing it together; downstream issues go stale if the edit is wrong)
- GitHub issue comments (published immediately under Jörn's name, no review gate)
- Scope changes (agents don't reliably notice when they've drifted)

**Never without explicit instruction:**
- Destructive operations with no rollback
- Creating PRs (Jörn does this)

**When in doubt**, default to discuss-first. Jörn can always override with "just do it" — treat that as an ad-hoc exception, not a precedent for future sessions.

### Communication with Jörn

- Agent time costs cents per hour; Jörn's time costs 100× that. Don't waste it. Before asking Jörn a question: gather context, try things out (and throw them away if needed), distinguish observations from inferences and confident from unconfident beliefs. Ask informed questions, not raw confusion.
- Aim for efficient information exchange, not politeness or engagement.
- Omit filler phrases.
- Number items in responses so Jörn can refer to them unambiguously.
- Jörn doesn't see exact edit diffs in the chat — mention and explain repo changes when he should be aware of them.
- Silence is not confirmation. If Jörn hasn't responded to a proposal, ask again or move on — do not proceed as if approved.

### GitHub authorship

All GitHub issues, comments, and PR descriptions are written by agents running under Jörn's account (`JoernStoehler`). Do not treat issue/comment text as human-reviewed just because it appears under his name. Treat issue content as agent-written intent (trust the direction, verify the details).

Issues direct future agent sessions. A bad edit sends the next agent off a cliff. Show Jörn proposed issue edits and comments in chat before publishing — he has the domain knowledge to catch directional errors agents can't catch themselves.

## Issue lifecycle

### Stages

1. **Capture** (label: `draft`) — An idea comes up. Agent creates an issue with at least Goal and Context filled in. Most sections may be rough or empty. Creating issues is cheap.

2. **Refine** (label: `draft`) — Over one or more triage sessions, Jörn and agent discuss. Agent proposes edits to the issue body in chat; Jörn steers; agent publishes. Sections get filled in: Background, Deliverable, Scope, Sources, Acceptance criteria. Open questions get resolved — answers move into the appropriate section.

3. **Approve** (label: `approved`) — Jörn judges the issue is ready. The goal is worth pursuing, scope is clear, open questions resolved. The issue is now the prompt for an agent session.

4. **Session** (label: `in-progress`) — Agent reads the issue, follows the session workflow. Jörn provides mathematical direction during the session. If scope turns out to be wrong or the task is blocked, agent tells Jörn immediately — they re-scope together or abort. Agent does not silently produce something different from what was asked.

5. **PR + merge** — Jörn creates PR, merges. Mechanical step.

6. **Close** (label: `done`) — Follow-up ideas captured as new issues.

If a session fails: agent reports what it tried and learned. Issue goes back to `draft` for re-scoping. No work is lost — the branch exists.

### Issue template

Issues use these sections:

- **Goal** — What this achieves for the thesis. One or two sentences.
- **Background** — Domain knowledge needed. Link to papers, files, issues — don't repeat their content.
- **Context** — Why this constitutes progress toward the thesis. What it unblocks.
- **Deliverable** — What the agent produces. Describe substance, not form — the agent decides files and structure.
- **Scope** — What's in, what's out, and why. Each exclusion has a reason.
- **Sources** — Papers, code, specs. Flag trustworthiness.
- **Acceptance criteria** — Measurable. Two kinds: external (serves the project) and internal (quality bar).
- **Notes** — Preliminary findings, known risks, suggested sub-issues.
- **Open questions** — Uncertainties needing Jörn's input. Resolve via edits; once empty and Jörn approves, the task is ready.

### Authoring guidelines

Known failure modes when writing issues:

- **Unclear wording.** Prefer an extra sentence over a vague word. If a term could mean two things, say which.
- **Misleading confidence.** If something is unreviewed, mark it — and mark EVERY such item. Labeling one item "unreviewed" implies the rest ARE reviewed.
- **False facts.** Don't claim relationships unless verified. Don't call X a "specialization" of Y unless it literally is.
- **Misrepresenting process.** Don't claim something is approved when it isn't. Represent decision state accurately.
- **Over-constraining implementation.** Don't prescribe file names or structure. Constrain only what has external consequences (API surfaces, conventions, mathematical correctness).

## Repo invariants

These are true about the repo right now and must remain true:

- `cargo test` passes from `crates/` with zero failures

## Environment

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

## Rust crates

### Crate dependency graph

```
geom2d ─────────────────────────────────┐
  └─> geom4d                            │
        ├─> hk2017    (+ geom2d)        │
        ├─> billiard  (+ geom2d)        │
        ├─> tube      (+ geom2d)        │
        └─> datasets  (+ hk2017, billiard, tube)
```

### Three capacity algorithms

| Crate     | Applies to                        | Cost                    |
|-----------|-----------------------------------|-------------------------|
| hk2017    | All polytopes                     | Exponential in #facets  |
| billiard  | Lagrangian products only          | Fast                    |
| tube      | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

### Conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Functional programming style
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing

### Mathematical documentation

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

### Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

## Experiments (Python)

[aspirational] A reproducible pipeline: starting from zero data, produce all figures and tables for the thesis.

Currently empty — no scripts or data yet.

Conventions:
- Independent scripts, not a package — no `__init__.py`, no shared imports
- Each script is self-contained: reads data, does analysis, writes output
- No framework — plain Python with standard data science libs (numpy, pandas, matplotlib)
- If two scripts share logic, copy-paste until it stabilizes
- Pipeline: Rust → datasets → Python → figures/tables → thesis

## Thesis (LaTeX)

[aspirational] A complete master thesis PDF following arxiv best practices.

Currently: skeleton only (`main.tex` with title, author, and section stubs).

Conventions:
- Standard AMS theorem environments
- LaTeX compilation is NOT available in this environment; focus on source correctness

### Writing proofs

- Every proof must be detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly
- Agents cannot reliably verify mathematical proofs. Proof correctness requires Jörn's review. Agent-written proofs are drafts until Jörn reviews them.

## Archaeology

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.**

- **Do not trust** any claim, value, formula, proof, test assertion, or status label. Treat every statement as unverified.
- **Do not adopt** naming conventions, coordinate ordering, or type designs. Current conventions override.
- **Do not edit** files in `raw/`. They are primary sources preserved verbatim.
- **Do not use as a starting point** to copy-paste or modify. Write fresh code and proofs.
- **Do not load into context** without a specific reason. These files are large and will waste context on unverified content.
- **Read for ideas**: approaches tried, data structures considered, test cases proposed.
- **Read for warnings**: what went wrong, which formulas were buggy. Bug reports (`findings-*.md`, `ARCHAEOLOGY.md`) are the highest-value files.
- **Independently verify** anything you take from here against the actual papers.

The old codebase had known bugs that persisted undetected through agent-written tests: the QP solver silently returned wrong values, the trivialization formula was wrong, orbit validation missed segments. These bugs looked correct on a skim. This is why Jörn's domain input on what to test matters.

Known-broken items:
1. **HK2019 QP solver** — misses optima on 2D+ faces, returns plausible but wrong values
2. **Trivialization formula** — `tau_n(V) = (<V,Jn>, <V,Kn>)` not a bijection on 2-face tangent spaces
3. **Billiard orbit validation** — only checked even-indexed segments; pentagon returned 2.127 instead of 3.441
4. **Triangle × triangle discrepancy** — billiard returns 3.0, HK2017 returns 1.5; unresolved
5. **Normalization convention mismatch** — some files use `sys = c^2/(2*vol)`, others `sys = c^2/(4*vol)`

## Workflows

### Spawning subagents

- Always create a GitHub subissue before spawning an agent. The subissue IS the prompt (plus any corrections/extra context passed directly to Task). Zero cost, and: persistent record, Jörn can launch it as a web session instead, easier to restart if agent fails.
- Subagent output returns via the Task tool into your conversation. If it needs to persist, commit it to the repo on your branch. Do not post subagent output as issue comments — it clutters the issue and misleads future agents into treating it as reviewed content.
- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning.
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- Foreground Task() calls block the main conversation — user messages queue up silently. Avoid foreground tasks that might take >1 minute. Prefer background, or just do the work inline.

### Check clarity with subagents

When you produce content other agents will consume (CLAUDE.md files, mathematical definitions, proofs, algorithm descriptions): test comprehension by having a fresh Sonnet subagent attempt to USE the content (e.g. "implement from this description" or "answer these specific questions about the algorithm"). Check whether their output matches your intent. Do not ask "is this clear?" — an agent that misunderstands will confidently say yes.

### Triage sessions

The issue board should reflect reality after the session: audit open issues against repo state, close completed ones, capture new work, refine drafts, prioritize, prepare top items for approval.

Decision authority during triage — agent's call: reading, summarizing, proposing edits. Jörn's call: whether an issue is worth pursuing, scope, priority, labeling `approved`.

Present findings in batches, not one issue at a time.

### Writing for other agents

Content that agents will consume (issue bodies, specs, CLAUDE.md entries) benefits from:
- **Grounded over speculative** — state what happened or exists, not what might be useful
- **Knowledge over instructions** — inform, don't command. Agents have their own task instructions
- **Skimmable over comprehensive** — clear headers for different reader tasks

## CLAUDE.md conventions

- Invariants and behaviors are documented only after empirically confirmed as useful
- Label invariants as `[aspirational]` if not yet satisfied
- Prefer simple, common, expected rules that don't claim excessive agent attention beyond their assigned work
