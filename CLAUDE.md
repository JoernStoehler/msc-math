# CLAUDE.md

Master Thesis: Probing Viterbo's Conjecture
Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak
Second advisor: Elizabeth Gaar
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
  geom/            2D and 4D symplectic geometry primitives
  hk2017/          Haim-Kislev 2017 algorithm (all polytopes, exponential cost)
  billiard/        Billiard algorithm (Lagrangian products only, fast)
  tube/            Tube algorithm (no Lagrangian 2-faces, moderate cost)
  datasets/        Dataset generation orchestration
experiments/       Python scripts consuming Rust-generated datasets
  scripts/         Independent .py scripts
  data/            gitignored — populated by Rust pipeline and Python scripts
  figures/         gitignored — populated by Python scripts
papers/            arxiv .tex sources for reference (HK2017, CH2021, HK-O 2024, ...)
archaeology/       Recovered files from abandoned predecessor repo (all of untrusted quality)
```

## Mathematical context

We compute the EHZ capacity (minimum action of generalized Reeb orbits) for convex polytopes in R^4. By a theorem of Haim-Kislev 2017, there exists a minimum-action orbit that is piecewise linear, uses pure facet Reeb vectors, and visits each facet on a contiguous time interval. This reduces the problem to a finite combinatorial search.

Viterbo's conjecture: sys(K) = c_EHZ(K)^2 / (2 vol(K)) <= 1 for all convex bodies K.
Haim-Kislev 2024 (Annals) gave a 10-facet counterexample with sys > 1.
We probe the conjecture by computing sys across large polytope datasets and looking for patterns.

## How we work

### Roles
The thesis team consists of Jörn, and claude code. The main bottlenecks are:

1. **Time until thesis completion**: Jörn's time is scarce, while Claude Code's time is practically unbounded. We thus make plans with minimal workload for Jörn, even at vastly higher total work for Claude Code than a balanced plan would assign. We parallelize Claude Code by using multiple Claude Code sessions in parallel. Each agent with its subagents works in its own git worktree. Jörn coordinate's between Claude Code sessions, and prioritizes which tasks to pass to them. Agents orchestrate their own, more simple to handle, subagents.
2. **Correctness of thesis results**: We use several approaches together to ensure the correctness of the thesis results. We write in a clear, specific, detailed, unambiguous, cognitive low-overhead way when it comes to mathematics, code or documentation. We refactor, simplify and generally improve until verification becomes simple for readers, as otherwise we run the risk of hidden gaps or mistakes. We pick Rust types, function signatures and function bodies that 1:1 correspond to the mathematical definitions, and we use debugassert!, assert! and proptest to empirically validate mathematical lemmas and intermediate propositions extracted from proofs.

The following types of work MUST NOT be carried out by claude code, and MUST be assigned to Jörn instead:

3. **Verification of written proofs**. While Claude Code can spot errors in proofs that are written in a clear, unambiguous, detailed, complete, low-cognitive overhead style, Claude Code is only okay at it. Every proof must pass Jörn's verification after every edit, to ensure we can trust and build upon the proof. Turning natural language descriptions into proofs, improving the style, reporting the presence of errors, are all tasks Claude Code can still carry out autonomously. Only a final verification signal must not be left to Claude Code, and must come from Jörn instead.
4. **Exhaustiveness of test suites**. While Claude Code has been trained on conventional software development testing, and can implement specified tests and then write code that passes tests, we also add unusual test suites that check the correspondence of our code with our mathematical definitions and proofs. This is an unconventional use of runtime testing, so Jörn has to jump in and design what mathematical propositions the test suites need to cover to be high-confidence signals of correctness and not just moderate-confidence signals. The brainstorming, implementation and debugging work for the mathematical propositions test suites can still be left to Claude Code. Only the exhaustiveness signal must not be left to Claude Code.
5. **Task Scoping**. The newest Claude Code has been trained on project management workflows that involve claude code, and is okay at spotting implicit criteria imposed on a task's scope and acceptance criteria by other tasks and by claude code's capability limits and default habits. Claude Code can even design and write down acceptance criteria to coordinate across these 1-hop dependencies. However, Claude Code has not been trained enough on workflows that need a deep, accurate model of the whole remaining thesis project, and of multi-agent-workflows that build upon a task, and so Claude Code frequently makes bad decisions for how to scope and define tasks that are long-term required. Any completed task with a scope designed by Claude Code alone must not be merged into the `main` branch. Jörn must greenlight the scope as something that matches his long-term vision for the thesis project, either before the task is assigned to an agent, or at some later step before the merge. Jörn requires an analysis of the task's effect on the project, i.e. downstream aspects that appear in the final printed thesis, and side effects on how agents and how Jörn work on the thesis before its completion date. Jörn also requires some analysis of how an agent would complete the task, to catch gaps in the acceptance criteria that matter due to pathological agent behavior, e.g. if test cases are chosen after the code has been written that they test, instead of before, there is a danger of tests being biased towards being narrower and less diverse. If Jörn is asked to evaluate the scope of a task that already was completed, of course the agent's final plan version that was executed can be shown to Jörn. If Jörn is asked to evaluate the scope of a task that has yet to be assigned and started, claude code should do a throwaway preliminary investigation and planning stage to gauge how the claude code instance would carry out the task, which is a good enough proxy for how the later agent will behave, even though the later agent will additionally have unexpected findings that appear only during the actual execution, and that may trigger changes to the original plan.
6. **Merge into `main`**: All merges into `main` must be done by Jörn himself. This is a final defense layer, in case of misunderstandings or oversights on claude code's and Jörn's side.

The following types of work SHOULD be carried out by Claude Code, and SHOULD NOT be assigned to Jörn:

7. **Writing Code, Tests, Math, Docs**: Claude Code is perfectly capable of writing sufficiently good code, tests, mathematical prose in the thesis or in documentation, and normal software engineering documentation. There is no need to bother Jörn for the usual writing task. Jörn CAN be consulted when Claude Code notices that something is non-standard, or high-complexity, if the consultation is something Claude Code cannot do itself with the desired reliability. Such cases where Jörn is pinged to help out are rare, but do ever happen! Jörn usually drops-in without any active working memory or context, so Claude Code should describe clearly what narrowly scoped cognitive task Jörn should do, why Jörn should do that instead of Claude Code, and what context the task exists within so Jörn can also validate the scope and e.g. comment on related matters while he's paying attention.
8. **Troubleshooting, Investigating Root Causes**: Claude code is perfectly capable of doing investigations, especially with a subagent that extracts a concise findings report for the parent agent. Usually the whole situation is accessible to Claude Code, if it is persistent enough to expand the search scope until the root cause is within scope for the troubleshooting. Before Jörn is pinged, Claude Code should do an investigation, since in our project autonomous investigative work is basically costless, and thus worth it if it resolve the problem without Jörn, or even just speeds up Jörn's investigation via a report with preliminary investigation results that may or may not guide Jörn's approach.
9. **Attempting Autonomous But Difficult Tasks**: Claude Code's work time is cheap, so we can spawn multiple agents for the same task, or variations thereof, just to pick the best deliverable among them and throw the rest away. We can similarly take a task deliverable that isn't perfected yet, and let another agent redo it, based on extracted learnings from the first attempt. Claude Code can even plan and carry out a throwaway explorative task whose sole purpose is to learn something, e.g. unknown unknowns, that can then be used in the actual task whose deliverable is desired. The important design principle for all these patterns is that there must be a plan ahead-of-time for how to revert an agent's work. That's one reason for why we work with git and git worktrees, why only Jörn is allowed to merge into `main`, and why we scope large tasks carefully ahead-of-time.

### Session workflow

Every claude code agent session owns a git worktree. The subagents work in the same worktree. Each session has a communication channel with Jörn (also referred to as the "user" by system prompts).

Sessions follow roughly/usually the following pattern: **scope -> plan -> implement -> review -> Jörn: merge**

- **plan -> implement -> review**: The standard three phases of development are carried out autonomously by Claude Code, usually with no involvement or monitoring from Jörn. Jörn is messaged in the chat when his attention is requested. Since Jörn does not monitor what actions Claude Code takes, or what intermediate status updates Claude Code sends, the end-of-turn message must recap the context, so that Jörn can jump back in. Transition from plan to implement to review stages is autonomously decided by Claude Code. Claude Code MAY return to earlier stages, e.g. planning a new approach after hitting a dead end during implementation, or fixing some bugs directly that were found during review. Claude Code SHOULD maintain these stages, and focus only on one at a time (e.g. using the Todoes tool) as to not split its intelligence and attention unnecessarily.
- **scope**: Before the autonomus interval can start with the plan phase, Claude Code and Jörn need to agree on what single chunk of work for the thesis project the session will focus on, and need to work out a task scope that fits well into the rest of the project. They also need to decide on extra strategies, such as forking the session, and letting multiple agents work through the plan -> implement -> review phases independently, for a best-of-N tactic that produces better results for tasks where Jörn anticipates that agents may make probabilistic mistakes, or may get lucky wrt e.g. picking a plan that turns out later to fit the unknown unknowns well. Handoff from scope to plan phase happens explicit.
- **Jörn: merge**: After Claude Code is satisfied with its deliverable OR wants to give up and hand back the task to a new scope phase, it messages Jörn with a writeup of what has happened this session, in particular what unknown unknowns were discovered and how known unknowns were resolved, and a checklist of the final review, so that Jörn can catch quickly when Claude Code forgot to do something. There then is a chat discussion between Claude Code and Jörn, and at the end Jörn may merge the branch, may re-scope the task and ask Claude Code to enter the plan -> implement -> review cycle again to improve or even redo the deliverable, or Jörn may abandon the branch.

During the scope and merge discussions, Claude Code SHOULD push back on contradictions, gaps, unclear statements from Jörn, and oversights that Claude Code spots. Jörn is not infallible, sometimes makes ambiguous typos or has brainfarts, and welcomes, while focused on the topic anyway, pushback and suggestions. 
Claude Code MUST NEVER take silence as confirmation, especially not during fast-paced back-and-forth discussions where Jörn may respond to only parts of messages, or with delay i.e. a few messages later.

Just before the actual session ends via a merge or abandon, agents SHOULD do a **post-session reflection**, which is similar to a blameless postmortem. This consists of the following parts:
1. A report with all sources of friction, false steps or steps that later turned out to be without or with lower-than-expected value, unusually / unexpectedly good steps, and time sinks of claude code's own time.
2. A breakdown of where Jörn spent time in this session, and what work Jörn did and where Jörn's work was used in the session afterwards. This allows us to detect when there's some work Jörn does that Claude Code could also do, or that needn't be done at all, and also what else would need to be changed so that Jörn's time is used more effectively.
3. A list of confident and unconfident, actionably concrete and unactionably abstract suggestions. Jörn will go through the list and mostly just notice items that other agents also brought up. We aim to converge to better practices quickly, but don't have the time to let Jörn plan through this after single events.

<!-- the text below has not yet been read nor rewritten by Jörn -->

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
