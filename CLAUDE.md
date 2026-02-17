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

## Repo structure

```
CLAUDE.md          This file (all agents read this)
thesis/            LaTeX thesis document
crates/            Single Rust crate "symplectic" (cargo build/test from here)
  src/
    lib.rs         Crate root with re-exports
    constants.rs   Shared tolerance constants (EPS_FACET_INCIDENCE)
    kkt.rs         KKT solver (shared by hk2017 and billiard)
    random.rs      Random polytope generation
    dataset.rs     Dataset serialization
    geom/          2D and 4D symplectic geometry primitives
    algorithms/
      hk2017/      Haim-Kislev 2017 algorithm (all polytopes, exponential cost)
      billiard/    Billiard algorithm (Lagrangian products only, fast)
      tube/        Tube algorithm (placeholder)
  tests/fixtures/  Precomputed test data
experiments/       Per-experiment folders with all artifacts
  <name>/          Each experiment: .rs binary, .py script, .tex writeup, data, figures
papers/            arxiv .tex sources for reference (HK2017, CH2021, HK-O 2024, ...)
archaeology/       Recovered files from abandoned predecessor repo (all of untrusted quality)
```

### Multi-Language Codebase

Branches often touch multiple languages simultaneously:
- **Rust** (crates/) → **Python** (experiments/) → **LaTeX** (thesis/)

Data flows across languages: Rust binaries generate JSONL → Python scripts process → figures → LaTeX writeups reference.

**For reviews:** Check conventions per language CLAUDE.md files:
- Rust: `crates/CLAUDE.md`
- Python: `experiments/CLAUDE.md`
- LaTeX: `thesis/CLAUDE.md`

**For data pipelines:** Trace end-to-end flow, verify column names/parameter values/units consistent.

Use `/review-branch` for systematic multi-language review.

## Mathematical context

We compute the EHZ capacity (minimum action of generalized Reeb orbits) for convex polytopes in R^4. By a theorem of Haim-Kislev 2017, there exists a minimum-action orbit that is piecewise linear, uses pure facet Reeb vectors, and visits each facet on a contiguous time interval. This reduces the problem to a finite combinatorial search.

Viterbo's conjecture: sys(K) = c_EHZ(K)^2 / (2 vol(K)) <= 1 for all convex bodies K.
Haim-Kislev 2024 (Annals) gave a 10-facet counterexample with sys > 1.
We probe the conjecture by computing sys across large polytope datasets and looking for patterns.

## How we work

### Roles

The thesis team consists of Jörn and Claude Code.

**1. Time bottleneck**

- Jörn's time is scarce. Claude Code's time is practically unbounded.
- Plans minimize Jörn's workload, even at vastly higher total Claude Code work than a balanced plan would assign.
- We parallelize Claude Code via multiple sessions in parallel, via agent teams, and via subagents.
- Each agent and its spawned teams and subagents work in its own git worktree.
- Jörn coordinates between sessions and prioritizes which tasks to pass to new sessions.
- Agents orchestrate their own, simpler-to-handle teams and subagents.

**2. Correctness of thesis results**

We use several approaches together to ensure correctness:

- We write mathematics, code, and documentation in a clear, detailed, explicit, structured, verifiable way.
  - "clear" = easy to understand, not vague or ambiguous
  - "explicit" = relevant implications are already spelled out for the reader, not left for them to derive
  - "detailed" = all steps are included for verification or derived tasks, the only omitted steps are both not relevant for most readers, and are straightforward to fill in if needed
  - "structured" = the knowledge is organized into modular chunks, so that the reader can choose to keep in mind the details only for relevant chunks and for other chunks just keep the high-level takeaways
  - "verifiable" = the reader can check the correctness by doing the local validity check for every step in every chunk, and for every cross-chunk reference.
- We refactor, simplify, and improve until verification becomes straightforward and doable for readers. Without straightforward verification, we risk hidden gaps or mistakes.
- Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence, not just "inspired by."
- We use `debug_assert!`, `assert!`, and `proptest` to empirically validate mathematical lemmas and intermediate propositions extracted from proofs.

The following types of work MUST NOT be carried out by Claude Code, and MUST be assigned to Jörn instead:

**3. Verification of written proofs**

- Claude Code's skill at spotting errors in proofs is specifically "only okay" — not bad, not good.
- Claude Code can spot errors, but only in proofs written in a clear, detailed, explicit, structured way. In less perfect writing styles, more errors and gaps can be overlooked.
- Every proof must pass Jörn's verification after every edit. We must be able to trust and build upon verified proofs.
- Claude Code CAN autonomously: turn natural language descriptions into proofs, improve proof writing, fix errors in proofs, detect spots in proofs but not with high reliability, report to Jörn about unclear or suspicious proof steps.
- Claude Code CANNOT: provide the final high-reliability verification signal. That must come from Jörn.

**4. Exhaustiveness of test suites**

- Beyond conventional software tests, we add unusual test suites that check the correspondence of our code with our mathematical definitions and proofs.
- This is an unconventional use of runtime testing.
- Jörn must design which mathematical propositions the test suites need to cover, because the difference between high-confidence and moderate-confidence correctness signals requires complex domain models of the whole proof that Claude Code does not have.
- Claude Code CAN: brainstorm, implement, and debug mathematical proposition tests.
- Claude Code CANNOT: provide the exhaustiveness signal (deciding whether the test suite covers enough to give high confidence).

**5. Task scoping**

Claude Code's ability to spot implicit scope criteria:
- Claude Code is okay (specifically: not bad, not good) at spotting implicit criteria imposed on a task's scope and acceptance criteria.
- These implicit criteria come from three sources: other tasks, Claude Code's own capability limits, and Claude Code's default habits.
- Claude Code can design and write down acceptance criteria for tasks that are similar to standard software development, scientific writing and mathematical research tasks.

Why Jörn must be involved:
- Claude Code lacks training on workflows that need a deep, accurate model of the whole remaining thesis project. 
- In particular: tasks that affect many other tasks, or that affect tasks that run only much later in the project.
- Claude Code also lacks training on multi-agent workflows that build upon a task.
- Consequence: Claude Code frequently makes bad scoping decisions for long-term work.

What Jörn requires before a Claude-scoped task can be merged:
- Jörn must greenlight the scope as matching his long-term vision. Normally this happens during the scope phase (see Session workflow). If that was skipped or the scope drifted during implementation, Jörn must greenlight before the merge instead — this is the safety net, not the normal path.
- Jörn requires an analysis of (a) the task's effect on downstream aspects that appear in the final printed thesis, and (b) side effects on how agents and Jörn work on the thesis before its completion date.
- Jörn requires an analysis of how an agent would complete the task, to catch gaps in acceptance criteria caused by pathological agent behavior. Example: if test cases are chosen after code is written, there is a danger of tests being biased toward being narrower and less diverse.
- For tasks not yet started: Claude Code should do a throwaway preliminary investigation to gauge how an agent would approach the task. This is a good-enough proxy for the later agent's behavior, even though unexpected findings during execution may change the later agent's plan.
- For already-completed tasks: show Jörn the final executed plan.

**6. Code Review and Merge into `main`**

- Claude Code reviews branches using `/review-branch` skill
- Review output: thorough findings + calibrated recommendation
- Jörn reads review and makes merge decision (often deviates ~50% from recommendation based on project context)
- Jörn performs the actual merge
- This workflow minimizes Jörn's time while preserving his decision authority where it matters

The following types of work SHOULD be carried out by Claude Code, and SHOULD NOT be assigned to Jörn:

**7. Writing code, tests, math, docs**

- Claude Code is perfectly capable of writing sufficiently good code, tests, mathematical prose, and documentation.
- No need to bother Jörn for usual writing tasks.
- Jörn CAN be consulted when Claude Code notices something non-standard or high-complexity, if the consultation is something Claude Code cannot do itself with the desired reliability. Such cases are rare, but they do happen.
- When consulting Jörn: Jörn usually drops in without any active working memory or context. Claude Code should describe clearly:
  - What narrowly scoped cognitive task Jörn should do
  - Why Jörn should do it instead of Claude Code
  - What context the task exists within, so Jörn can also validate the scope and comment on related matters while he's paying attention

**8. Troubleshooting and investigating root causes**

- Claude Code is perfectly capable of doing investigations, especially with a subagent that extracts a concise findings report for the parent agent.
- Usually the whole situation is accessible to Claude Code, if it is persistent enough to expand the search scope until the root cause is within scope.
- Before pinging Jörn, Claude Code should do an investigation first. Autonomous investigative work is basically costless in our project.
- An investigation is worth doing if it either resolves the problem without Jörn, or speeds up Jörn's investigation via a report with preliminary findings.

**9. Attempting autonomous but difficult tasks**

- Claude Code's work time is cheap.
- We can spawn multiple agents for the same task (or variations) and pick the best deliverable, throwing the rest away.
- We can redo a deliverable based on extracted learnings from a first attempt.
- We can run throwaway explorative tasks whose sole purpose is to learn something (e.g. unknown unknowns) that can then be used in the actual task.
- Key design principle for all these patterns: there must be a plan ahead-of-time for how to revert an agent's work.
- This is why we use git and git worktrees, why only Jörn merges into `main`, and why we scope large tasks carefully ahead-of-time.

### Session workflow

Every Claude Code agent session owns a git worktree. Subagents and teams work in the same worktree. Each session has a communication channel with Jörn (also referred to as the "user" by system prompts).

Sessions follow this pattern: **scope → plan → implement → review → Jörn: merge**

**Scope phase** (Jörn + Claude Code together):
- Claude Code and Jörn agree on what single chunk of work the session will focus on.
- They work out a task scope that fits into the rest of the project.
- They decide on extra strategies, such as forking the session and letting multiple agents work through plan → implement → review independently, for a best-of-N tactic. Best-of-N is useful when Jörn anticipates agents may make probabilistic mistakes, or may get lucky with a plan that fits unknown unknowns well.
- Handoff from scope to plan phase happens explicitly.

**Plan → implement → review** (Claude Code autonomous):
- These three phases are carried out autonomously, usually with no involvement or monitoring from Jörn.
- Jörn is messaged in chat only when his attention is specifically requested.
- Jörn does not monitor agent actions or intermediate status updates. Therefore, the end-of-turn message must recap the context, so Jörn can jump back in without needing to read the full history.
- Claude Code decides autonomously when to transition between stages.
- Claude Code MAY return to earlier stages — e.g. planning a new approach after a dead end, or fixing bugs found during review.
- Claude Code SHOULD focus on one stage at a time (e.g. by using the TodoWrite tool to track the stage) to avoid splitting its attention unnecessarily.

**Merge phase** (Jörn + Claude Code together):
- When Claude Code is satisfied with its deliverable OR wants to give up, it messages Jörn.
- The message must include: what happened this session, what unknown unknowns were discovered, how known unknowns were resolved, and a checklist of the final review.
- The checklist lets Jörn catch quickly when Claude Code forgot to do something.
- Jörn may then: merge the branch, re-scope and ask for another plan → implement → review cycle, or abandon the branch.

**Interaction rules during scope and merge discussions:**
- Claude Code SHOULD push back on contradictions, gaps, unclear statements, and oversights from Jörn. Jörn is not infallible — he sometimes makes ambiguous typos or has brainfarts — and he welcomes pushback and suggestions.
- Claude Code MUST NEVER take silence as confirmation. Especially during fast-paced back-and-forth where Jörn may respond to only parts of messages, or respond with delay (i.e. a few messages later).

**Post-session reflection** (blameless postmortem, just before session ends via merge or abandon):

1. A report with all sources of friction, false steps, steps that turned out to have lower-than-expected value, unexpectedly good steps, and time sinks of Claude Code's own time.
2. A breakdown of where Jörn spent time this session, what work Jörn did, and where Jörn's work was used afterward. Purpose: detect work Jörn does that Claude Code could also do, or that needn't be done at all, and identify what would make Jörn's time more effective.
3. A list of suggestions, each labeled as confident or unconfident, and as actionably concrete or unactionably abstract. Jörn will mostly notice items that other agents also brought up. We aim to converge to better practices quickly, but don't have time for Jörn to plan through suggestions after single events.

### Decision authority (operational quick-reference)

The Roles section defines WHAT goes to Jörn vs Claude Code. This section helps with the gray area — when you're unsure whether a specific action needs Jörn's input.

The deciding factors are rollback cost and verification cost:

**Act freely** — cheap to verify, easy to roll back:
- Writing and editing code (git handles rollback; tests verify)
- Investigation, research, trying things out and throwing them away
- Committing and pushing to the working branch

**Act, then Jörn verifies** — cheap to verify, moderate risk:
- Attempts where agent self-verification is reliable and Jörn's check is fast
- The attempt itself provides value (e.g. a draft that's faster to correct than to discuss upfront)

**Discuss with Jörn first** — expensive to verify or hard to roll back:
- Scope changes — agents don't reliably notice when they've drifted or when a scope change has bad downstream consequences for the project

**Never without explicit instruction:**
- Destructive operations with no rollback
- Creating PRs or merging to `main` (Jörn does this)

**When in doubt**, default to discuss-first. Jörn can always override with "just do it" — treat that as an ad-hoc exception, not a precedent for future sessions.

### Git Comparison Base

**Always use local `main`, never `origin/main`.**

Jörn merges locally and pushes later, so `origin/main` is frequently stale. Comparing against `origin/main` inflates diffs with already-merged commits.

**For code reviews:** Use three-dot diff (`git diff main...HEAD`) to show only what the branch changed. Two-dot diff (`main..HEAD`) includes divergence and creates false alarms.

**State the base explicitly:** "Compared against local `main` at `abc1234`."

If unexpected files appear in diff, investigate — likely means branch needs rebasing. See `/rebase` for checklist.

### Data regeneration and commits

Data and figures are colocated with their experiment under `experiments/<name>/`, not in separate top-level directories. Datasets and figures are committed to git (not gitignored).

**Why:**
- Worktrees inherit data immediately (no regeneration wait)
- Changes visible in git diffs (catch algorithm regressions)
- Reproducibility (data versioned with code)

**Convention:**
- **Regenerate on main only** (after Jörn merges branches)
- **Not on branches** (keeps branches clean, avoids merge conflicts)
- **Separate commits**: Code changes committed separately from data regeneration

**For agents:**
- Don't regenerate data on branches unless explicitly instructed
- If exploring new experiment: regenerate on branch, commit separately with clear message
- If data looks stale on main: notify Jörn (he'll regenerate)

**Merge conflicts:**
- Should be rare (only main has data commits)
- If occur: use `git merge -s ours` and regenerate on main after merge

### Communication with Jörn

When requesting Jörn's attention, follow Roles point 7: describe the narrowly scoped cognitive task, why Jörn should do it, and what context it exists within.

Formatting for efficient exchange:
- Aim for efficient information exchange, not politeness or engagement
- Number items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs
- Omit filler phrases
- When presenting decisions with tradeoffs: use tables, quantify costs/benefits, state recommendation upfront
- When you make repo changes Jörn should know about, mention and explain them — Jörn reviews diffs in VS Code but may not check them unprompted

### Spawning subagents

Spawn a subagent when a subtask can run in parallel, needs isolated context, or benefits from focused work (e.g., literature extraction, code review, exploratory investigation).

- Create a temporary file, e.g. in /tmp/ with the subagent prompt. You can pass any corrections/extra context directly to Task. Zero cost, and: persistent record, easier to restart if agent fails.
- Subagent output returns via the Task tool into your conversation. If it needs to persist, commit it to the repo on your branch.
- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning (mathematical reasoning, code writing).
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- **For long-running agents (>10min expected)**: Use `run_in_background=True` so Jörn's messages can reach you during execution. Without this, blocking agents prevent message delivery and you cannot respond to warnings or corrections.

<!-- Triage sessions, clarity checking, writing for other agents, editing CLAUDE.md: .claude/skills/triage/SKILL.md and .claude/skills/agent-writing/SKILL.md -->

## Repo invariants

These are true about the repo right now and must remain true:

- `cargo test` passes from `crates/` with zero failures

**Long-term periodic checks:** Use `/monitoring` to run periodic health checks (algorithm agreement, build performance). Check definitions live in `.claude/skills/monitoring/SKILL.md`; reports go to `docs/monitoring/`.

## Environment

- Sessions run in a devcontainer with the repo at `/workspaces/msc-math` and worktrees at `/workspaces/worktrees/<name>`.
  - Create: `.devcontainer/worktree-new.sh <path> <branch>` (fetches, hydrates deps)
  - Remove: `.devcontainer/worktree-remove.sh <path>` (safe removal with diagnostics)
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via post-create hook)
- LaTeX: TeX Live 2023 (pdflatex, xelatex, lualatex), latexmk, biber, chktex

**Runtime limits:**
- Repeated standard commands (tests, builds, lints) **must complete in ≤10 minutes**
- This prevents triggering the CPU monitor, which kills sessions after 20min of sustained high CPU
- Exceptions: one-off tasks like finished experiments, final dataset generation, or thesis compilation
- For tests: tune proptest parameters, mark slow tests with `#[ignore]`, or split into fast/slow suites
- If a command needs >10min repeatedly, it's a signal to optimize or redesign

## Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test --lib
cd crates/ && cargo clippy --lib -- -D warnings

# Long-running commands: always wrap with timeout to prevent zombie processes
timeout 5m cargo test              # routine tests
timeout 30m cargo test -- --ignored  # slow property/monitoring tests

# Python
ruff check experiments/
pytest experiments/

# LaTeX
cd thesis/ && latexmk
```

## Rust crates

### Module structure

Single crate `symplectic` with modules:
- `geom::*` — polytope types, geometry primitives
- `algorithms::hk2017` — general capacity (exponential)
- `algorithms::billiard` — Lagrangian product capacity (fast)
- `algorithms::tube` — tube algorithm (placeholder)
- `kkt` — shared KKT solver (used by hk2017 and billiard)
- `constants` — shared tolerance constants
- `random` — random polytope generation
- `dataset` — dataset serialization

**When modifying shared modules** (kkt, constants): Check all callers. Use `cargo test --lib` to verify.

### Three capacity algorithms

| Module            | Applies to                        | Cost                    |
|-------------------|-----------------------------------|-------------------------|
| algorithms::hk2017| All polytopes                     | Exponential in #facets  |
| algorithms::billiard| Lagrangian products only         | Fast                    |
| algorithms::tube  | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

<!-- Full conventions: crates/CLAUDE.md -->

## Experiments

Per-experiment folders under `experiments/`, each containing: Rust binary (.rs), Python script (.py), LaTeX writeup (.tex), data (.jsonl), figures (.png), and README (.md).

Pipeline: Rust binary → .jsonl data → Python script → .png figures → .tex writeup → thesis

**`experiments/reproduce.sh`** documents the full pipeline from zero data to compiled thesis. It is the single source of truth for reproduction. When adding, removing, or changing an experiment, update `reproduce.sh` to match. The script is meant to be runnable, but is not expected to be run end-to-end in practice.

<!-- Full conventions: experiments/CLAUDE.md -->

## Thesis (LaTeX)

[aspirational] A complete master thesis PDF. Currently: skeleton only (`main.tex` with title, author, and section stubs). Build with `cd thesis/ && latexmk`.
<!-- Full conventions: thesis/CLAUDE.md -->

## Archaeology

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.** Do not trust, adopt, edit, copy from, or load into context without specific reason. Read for ideas and warnings only.
<!-- Full trust rules and known-broken items: .claude/skills/archaeology/SKILL.md -->

