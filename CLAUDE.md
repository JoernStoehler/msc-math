# CLAUDE.md

Master Thesis: Probing Viterbo's Conjecture
Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak
Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – March 2026

## [Aspirational] End state

This repo is making progress towards a completed master thesis with:
- A printed-quality LaTeX document `thesis/build/main.pdf`
- A high-performance stable Rust library for symplectic geometry on polytopes in `crates/`
- A reproducible experiment pipeline in `experiments/`

## Mathematical Context

The thesis is motivated by a paper from Haim-Kislev and Ostrover 2024, which disproved Viterbo's conjecture in dimension 4 via an explicit counterexample polytope. The conjecture was until then a famous open problem in symplectic geometry.

Viterbo's Conjecture (2000): For any convex body K in R^2n, including any polytope K in R^4, the systolic ratio `sys(K) = c_EHZ(K)^2 / (2 vol(K))` is at most 1, where `c_EHZ(K)` is the Ekeland-Hofer-Zehnder capacity of K.
Haim-Kislev and Ostrover (2024, Annals): Defines a 10-facet counterexample with `sys > 1`.

We follow Haim-Kislev 2017, Chaidez-Hutchings 2021 in extending the usual smooth symplectic geometry setting to polytopes in R^4. We extend the published algorithms for computing c_EHZ(K) by implementing them in Rust, adding optimizations that exploit known facts about the symplectic geometry of polytopes, and we verify the correctness of our code with excessive paranoia to avoid any errors even on large, or adversarially chosen, polytopes.

We then probe the conjecture by computing `sys` across large polytope datasets and looking for patterns.

## Multi-Language Codebase

Branches often touch multiple languages simultaneously:
- **Rust** (crates/, experiments/): most code that requires performance, or is correctness-critical, or just interacts with other rust code.
- **Python** (experiments/): for plotting, data processing, orchestration, and data science experiments where python is the more common and less cumbersome choice.
- **LaTeX** (thesis/, experiments/): for the thesis pdf, facing the real readers (Jörn, Kai, Elizabeth) and the imagined readers (a motivated MSc student with a background in symplectic geometry and optimization theory).
- **Markdown** (various): agent-facing writeups, including conventions, rules, workflows, documentation, takeaways, experiment ideas, data interpretation, reports and learnings, and much more.
- **Json/Jsonl/Csv** (experiments/): for datasets that are consumed by and produced by experiments. It's just a more convenient data format than binary formats, e.g. easier git diffs.

Each topic section below mentions its relevant review subagent(s) for focused checklists.

## Knowledge Placement

**When you produce new knowledge** (findings, conventions, docs, comments):
- Tied to a specific file or function? → code comment, doc comment, or file header. This is the natural location agents look at when working with that code.
- Convention for a specific file type or directory? → `.claude/rules/*.md` (auto-loaded by path pattern). This is where topic-specific conventions live.
- Applies to most agents regardless of file type? → CLAUDE.md.
- Applies to a minority of agents? → `.claude/skills/*/SKILL.md` (progressive disclosure: name + description always loaded, body on demand).
- Project management (tasks, ideas, deferred work, constraints)? → `TASKS.md` (root). Grows stale; that's fine.
- Session learning or cross-session state? → `MEMORY.md`. Migrate stable entries to CLAUDE.md or standard locations.
- Don't dump unrelated knowledge into README.md files. Each README covers its own directory's purpose.

**When you need knowledge you don't have:**
- Check code comments, file headers, and README.md in the relevant directory first.
- Check CLAUDE.md (you already have it in context — search for keywords).
- Check skill names and descriptions — load the skill if it matches your need.
- Check `TASKS.md` for project-level context (what's planned, what's deferred, why).
- Check `papers/` for referenced paper sources when verifying math or citations.
- Check `.devcontainer/` for environment details (what's installed, how sessions run).

**When editing CLAUDE.md or SKILL.md files:**
- Load the `meta-documentation` skill first for rationale and style rules.

**Convention enforcement architecture:**
- **CLAUDE.md** — project context, workflow, communication rules. Always loaded. Kept lean.
- **Skills** (`.claude/skills/`) — convention details per topic (e.g. `rust-conventions`, `tex-content`). Loaded on demand by main agents and explicitly by subagents. CLAUDE.md topic sections name which skills to load.
- **Review workflow** — `review` skill teaches review subagents the sequential checklist methodology. Main agent spawns generic subagents specifying concern + files + skills.

## Communication with Jörn

**Before requesting Jörn's attention:** Investigate first. Autonomous investigative work is basically costless. An investigation is worth doing if it either resolves the problem without Jörn, or speeds up Jörn's investigation via a report with preliminary findings.

**When requesting Jörn's attention:**
- Describe the narrowly scoped cognitive task Jörn should do
- Say why Jörn should do it instead of you
- Provide the context it exists within — Jörn usually drops in without working memory of your session
- After pauses in discussion, re-provide session context. Jörn switches between multiple agent sessions and does not monitor what agents do.

**Formatting for efficient exchange:**
- Number items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs
- Omit filler phrases — aim for efficient information exchange, not politeness
- When presenting decisions with tradeoffs: use tables, quantify costs/benefits, state recommendation upfront
- When you make repo changes Jörn should know about, mention and explain them — Jörn reviews diffs in VS Code but may not check them unprompted

**Interaction dynamics:**
- Push back on contradictions, gaps, unclear statements, and oversights. Jörn is not infallible — he sometimes makes ambiguous typos or has brainfarts — and he welcomes pushback.
- Never take silence as confirmation. Especially during fast-paced back-and-forth where Jörn may respond to only parts of messages, or respond with delay.
- **Word-choice sensitivity:** Jörn communicates distinctions via subtle word choices that agents tend to gloss over. When Jörn says "not quite" and corrects a nuance, the specific words he chose carry meaning. Don't paraphrase corrections back into your original framing — adopt his exact phrasing and check whether you lost a distinction.

**Processing feedback:** Load the `feedback-processing` skill when receiving corrections from Jörn. It describes the generalization loop: fix the instance, abstract the error class, scan for all instances, record durably.

## Staying Focused Across Long Sessions

**Plan file as persistent memory:** Update the plan file as you work — it survives context compaction, your working memory does not.
- After completing an item: mark it done, note any surprises or context future items need.
- Before starting a new item: record what you're about to do and why.
- When discovering context relevant to upcoming items: write it into the plan now, not "later."
- When you need something to survive a session boundary or compaction: put it in the plan file.

**What gets lost at compaction** (danger ranking, most to least dangerous):
1. **Scheduled items you haven't started** — you forget they exist and they never get done
2. **Context and considerations for upcoming items** — you redo them from scratch or miss nuances
3. **Completed items** — low cost, already done, only needed for final reporting

**Session recovery after compaction or handoff:**
- If you suspect you lost context: check the plan file first, then MEMORY.md.
- If you need details from the pre-compaction conversation: delegate JSONL transcript reading to a subagent. Never read the transcript yourself — it's too large and wastes your context window.
- Never guess about what happened pre-compaction — verify or say "I don't know."

**Plan + compact + continue (for multi-phase work within a single session):**

With 1M context windows, sessions run for hours and accumulate far more context than the compaction summary can preserve. The plan file is the critical bridge. The workflow:

1. **Work a phase.** Implement, test, commit. Update the plan file as you go (progress, decisions, surprises).
2. **Before compaction:** Write a complete handoff in the plan file — not just progress markers, but:
   - What was built and where (files, commits)
   - Design decisions made during implementation and WHY (these are the hardest to recover)
   - What's broken / known limitations
   - What to do next, in priority order
   - Key file paths the next phase will need
3. **Jörn triggers compaction.** The context is summarized; most working details are lost.
4. **After compaction:** The agent reads the plan file and picks up from "suggested next steps." The compaction summary provides rough continuity; the plan file provides precise instructions.

Why this matters more at 1M than at 200k: a 200k session accumulates ~2 hours of context; compaction loses moderate detail. A 1M session accumulates ~10 hours; compaction loses proportionally more. Design discussions, Jörn's feedback, numerical insights discovered during debugging — all of this must be in the plan file or it's gone.

**Anti-patterns:**
- Updating the plan file only at the end of a phase (too late if compaction happens mid-work)
- Writing "see code for details" instead of capturing the WHY in the plan (code shows WHAT, not WHY)
- Assuming the compaction summary preserves specific numbers, thresholds, or file paths (it doesn't)

## Session Workflow

Every agent session owns a git worktree. Subagents and teams work in the same worktree.

**Time economics:** Jörn's time is scarce; agent time is practically free ($0/h). Plans minimize Jörn's workload, even at vastly higher total agent work. We parallelize agents via multiple sessions, agent teams, and subagents.

### Session pattern: scope → plan → implement → review → merge

**Scope phase** (Jörn + agent together):
- Agree on a single chunk of work for this session.
- Jörn scopes the task within his long-term project vision. Agents cannot reliably do this — they lack deep models of how tasks affect downstream work or later sessions.
- Agents provide preliminary investigation findings to help Jörn scope faster.
- Handoff to plan phase happens explicitly.

**Plan → implement → review** (agent autonomous):
- These three phases are carried out autonomously, usually with no involvement from Jörn.
- Jörn is messaged in chat only when his attention is specifically requested.
- Jörn does not monitor agent actions or intermediate status. End-of-turn messages must recap context so Jörn can jump back in without reading the full history.
- Agents decide autonomously when to transition between phases and MAY return to earlier phases (e.g. replanning after a dead end).
- Focus on one phase at a time to avoid splitting attention.

**Merge phase** (Jörn + agent together):
- When the agent is satisfied with its deliverable OR wants to give up, it messages Jörn.
- Include: what happened this session, what unknown unknowns were discovered, how known unknowns were resolved, and a checklist of the final review.
- Only Jörn merges to `main`. Only Jörn creates PRs.

### What needs discussion vs. what doesn't

The deciding factors are rollback cost and verification cost:

**Act freely** — cheap to verify, easy to roll back:
- Writing and editing code (git handles rollback; tests verify)
- Investigation, research, trying things out and throwing them away
- Committing and pushing to the working branch

**Act, then Jörn verifies** — cheap to verify, moderate risk:
- Attempts where agent self-verification is reliable and Jörn's check is fast
- Drafts that are faster to correct than to discuss upfront

**Discuss with Jörn first** — expensive to verify or hard to roll back:
- Scope changes — agents don't reliably notice when they've drifted or when a scope change has bad downstream consequences

**Never without explicit instruction:**
- Destructive operations with no rollback
- Creating PRs or merging to `main`

**When in doubt**, default to discuss-first. Jörn can always override with "just do it."

### Autonomous difficult tasks

Agent time is cheap. Use it aggressively:
- Spawn multiple agents for the same task (or variations) and pick the best deliverable.
- Redo a deliverable based on learnings from a first attempt.
- Run throwaway exploratory tasks whose sole purpose is to learn unknowns.
- **Revert plan required:** For all these patterns, there must be a plan ahead-of-time for how to revert an agent's work. This is why we use git worktrees and why only Jörn merges to `main`.

### Plan workflow

Conventions for the plan phase (the `plan` subagent overrides default `/plan`):

**Save Jörn's time:**
- Obtain findings upfront — Jörn can decide faster with data than with armchair designs
- Present findings in a skimmable progressive-disclosure format
- Pre-empt follow-up investigations — avoid slow back-and-forth with minute-long interruptions
- Provide session context after pauses — Jörn switches between sessions and does not monitor agents
- Check scope against the time economics and scoping rules in this section before finalizing

**Track where task scope comes from:**
- The root terminal goal is thesis success
- Convergent instrumental goals (rule adherence, best practices, minimizing Jörn's time) are omnipresent
- Open-scope ideas floated during planning can expand the session scope
- Closed-scope goals concretize how to achieve some other goal
- Track why each plan element was picked over alternatives — needed to adapt the plan when feedback comes in

## Subagents & Review

Spawn a subagent when a subtask can run in parallel, needs isolated context, or benefits from focused work (e.g., literature extraction, code review, exploratory investigation).

- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning (mathematical reasoning, code writing).
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- **For long-running agents (>10min expected)**: Use `run_in_background=True` so Jörn's messages can reach you during execution.

### The core rule

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code and confirming the cross-check exists. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`. Violating this rule is the single most damaging failure mode — it spreads across the whole thesis when others rely on a false claim, and then wastes a lot of Jörn's time to identify downstream issues and redo work.

**Citation verification (core rule instance):** Never produce author names, paper titles, or literature attributions from memory. Always verify against `thesis/bibliography.bib` (for cited works) or the paper files in `papers/` (for author names and content). Agents confidently produce plausible-sounding but wrong author names from training data — e.g., "Cieliebak-Hutchings" instead of the correct "Chaidez-Hutchings" (CH2021). The authoritative sources are:
- `thesis/bibliography.bib` — all cited works with correct author fields
- `papers/<key>/` — local copies of referenced papers

### Review workflow

Load the `review` skill for the full workflow. Key points:

- Fix syntax/style before reviewing semantics/content
- Spawn generic review subagents in parallel — one per concern per file group
- Each subagent loads the relevant convention skill and works through a checklist item-by-item
- Err towards running too many subagents: agent time is free, especially parallelized
- Mandatory before presenting `.tex` deliverables to Jörn, recommended for all deliverables

## Git

Load `git-conventions` skill. Key: always use local `main` (not `origin/main`), three-dot diffs for reviews, commit checklist before reports.

## Thesis Writing

Load skills: `tex-build` (build commands, PDF review), `tex-format` (comments, headers, environments, figures), `tex-content` (four audiences, correctness, proofs, citations).

## Experiment Writing

Builds upon Thesis Writing. Load `experiment-conventions` skill. Key: "write up what's there — nothing more, nothing less", verification against JSONL data, pipeline Rust→JSONL→Python→figures→thesis.

## Rust Library

Single crate `symplectic` in `crates/`. Invariant: `cargo test` passes with zero failures. Load skills: `rust-conventions` (coding style, math-code correspondence, cross-refs), `rust-tests` (testing philosophy, fixtures, test organization).

## Experiments

Per-experiment folders under `experiments/`. Load skills: `experiment-conventions` (structure, pipeline, philosophy, quality), `python-conventions` (script headers, figure sizing, visual quality).

## Environment

- Sessions run in a devcontainer with the repo at `/workspaces/msc-math`.
  - Worktrees: use `--worktree` flag or `EnterWorktree` tool. Hooks in `.claude/hooks/` override defaults to branch from local `main`. Worktrees land at `.claude/worktrees/<name>/`.
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via post-create hook)
- LaTeX: TeX Live 2023 (pdflatex, xelatex, lualatex), latexmk, biber, chktex

**Runtime limits:**
- Repeated standard commands (tests, builds, lints) **must complete in ≤10 minutes**
- This prevents triggering the CPU monitor, which kills sessions after 20min of sustained high CPU
- Exceptions: one-off tasks like finished experiments, final dataset generation, or thesis compilation
- For tests: tune proptest parameters, mark slow tests with `#[ignore]`, or split into fast/slow suites
- If a command needs >10min repeatedly, it's a signal to optimize or redesign

## Quick Commands

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

## Archaeology

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.** Do not trust, adopt, edit, copy from, or load into context without specific reason. Read for ideas and warnings only.

## Working notes (redistribute later)

**Rollbacks are cheap.** Git handles rollback; agent time (1h = $0) is practically free. Commit your work regularly so rollbacks are possible and so context survives compaction. When you defer a question to keep working, write it down (plan file or TODO comment) so it doesn't get dropped. Deferred ≠ dropped. The worst case of deferring is wasted agent time — which is acceptable unless Jörn is actively waiting on you or important session context will be lost.

**Fix obvious bugs you find, even if another agent wrote the code.** Don't ignore problems just because they weren't your fault. Report what you found and fix it — or if the fix is risky/large, report it and explain why you didn't fix it.
