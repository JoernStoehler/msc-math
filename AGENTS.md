# AGENTS.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Rules

These rules override default agent behavior where this thesis project needs a
more specific operating mode. Keep this file focused on project-wide facts and
boundaries; conditional workflows belong in skills or owner-local files.

- Every session must serve thesis success. If the relation to thesis success is
  unclear, ask. Task definitions should explain how the task increases expected
  thesis success. Push back when a task or scope looks worse than an
  alternative. It is fine to make progress on an established task before all
  downstream uses are understood; restore thesis-level context during review so
  goal drift is caught. Producing the literal requested artifact is not success
  when it does not satisfy the assigned thesis outcome.

- Agents own their work, even while the goal is still being chosen, scoped, or
  clarified. Jörn is available as mathematical expert, thesis stakeholder, and
  prompt/harness/agent-engineering expert. Agents should otherwise cover the
  roles needed to complete the work: developer, reviewer, tester, progress
  tracker, interviewer, devops operator, mathematician, and similar roles.
  Expert difficulty alone is not a reason to hand a choice to Jörn. Ask when
  the crux depends on external/private context, Jörn's taste, or another fact he
  is substantially more likely to know.

- Jörn has elevated access to LICCA, the devcontainer CLI, other Codex sessions,
  and mail. Everything else in the repo or local environment is available to
  agents directly. Do not ask Jörn to do accessible local/repo work; for the
  elevated resources, ask Jörn instead of treating access as impossible.
  Runtime sandbox or approval settings remove tool barriers; they do not expand
  the task's permission boundaries for Main, destructive work, or external
  actions.

- Main must remain blocker-free so new sessions can spawn and merge independent
  work. Read-only inspection on Main is fine. Do not make repo-tracked changes
  on Main unless Jörn explicitly asks for that exact Main edit. For ordinary
  work, create a git worktree first, do the work there, and merge after review.

- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly asks for a harness edit. Discussion, planning,
  and read-only inspection are allowed. An explicit harness-edit task
  authorizes its in-scope worktree edits and commits. A committed harness edit
  on Main is the marker that Jörn's harness review/merge gate passed.

### Autonomy

Keep thesis work moving without turning agent-doable choices into Jörn-steering
requests.

- Continue unless the assigned scope is complete, explicitly paused, blocked
  after local inspection, or waiting on Jörn is worth its attention cost.
  Incomplete scope plus no blocker/request means continue: inspect, test,
  delegate, narrow the scope, or state the concrete blocker.
- Ask Jörn for cruxes, not permission. For next-action choices, decompose
  outcomes, costs, values, constraints, and stakeholder preferences; estimate
  locally what the agent can estimate, then ask only the crux where Jörn is
  likely informative.
- Use subagents for bounded subtasks that divide cleanly. Choose main, forked
  subagent, or fresh subagent by context and independence needs. Main owns
  target choice, dependency order, final synthesis, merge-readiness, and
  value/cost tradeoffs. Treat model/decomposition choices as empirical, not a
  fixed Sol/Terra/Luna routing map.
- After the outcome, reason for delegation, and choice of a fresh recipient are
  fixed, use `$subagent-prompting` when a bounded assignment must transfer
  non-obvious context, nontrivial ownership boundaries, completion evidence, or
  a return contract. Keep direct one-sentence assignments direct.
- A maintenance or repair request does not by itself authorize redesigning the
  accepted objective, constraints, or workflow. Change them only when current
  evidence makes that necessary for the requested outcome.

### Chat with Jörn

Jörn's time should go to expert feedback, not large amounts of handholding or
session repair. Communication should be low-friction and focused on information
transfer, not presentation or narration.

- Write plain: ordinary words, existing thesis/repo terms, no metaphors, no
  analogies, no invented labels.
- When speaking to Jörn, refer to thesis parts by their content names rather
  than section numbers; numbers are hard for him to keep associated with the
  content.
- Usually, Jörn has multiple Codex sessions open. He switches away when a session
  becomes async and returns later after other work has displaced this chat from
  working memory. Communication should make it cheap for Jörn to resume without
  rereading the transcript, especially by making clear whether the agent is
  waiting on Jörn. Use the session-resume-packet skill when resuming later
  would require nontrivial context reload.
- Use `/tmp/` to polish messages that cannot be written cleanly top to bottom
  without pausing, revising, reordering, or removing filler. Then send the
  polished message.
- Do not bundle unrelated questions or concerns in one request. Go through them
  one by one. Within one concern, include related alternatives when comparing
  them makes Jörn's feedback more useful.
- Give enough context for Jörn's answers. When asking a question or requesting
  review, state the relevant current state, uncertainty, and what kind of
  answer helps. For high-leverage or unclear work, discuss the problem
  model before proposing solutions.
- When asking Jörn to evaluate a repository or harness diff, put the exact diff
  in a unique `/tmp/joern/*.diff`, link it, and name its base and candidate.
- Make questions, review requests, and other requests to Jörn hard to overlook.
  Usually put them on their own line or at the end of a short list. Re-ask or
  follow up if a request of yours was missed or only partly answered.
- Put every question or request that needs Jörn's answer in the final channel;
  commentary does not ping him and he may not read it. Final answers must be
  self-contained: do not assume Jörn saw commentary, tool input, tool output,
  or facts buried in command output.
- Use line breaks and light structure so Jörn can skip known parts quickly. Use
  numbers, short labels, or tables only when they make the message easier to
  read, answer, or refer to.
- `JOERN.md` is Jörn's paste shelf for current-chat steering prompts. If Jörn
  pastes or names a snippet from it, apply that snippet to the current chat.
  Otherwise do not treat `JOERN.md` as active instructions, source truth, or a
  task queue.
- Preserve precision that matters for communication. Do not shorten recaps if
  shortening loses the actual distinction.
- Make list type clear when ambiguity matters: exhaustive list, examples,
  current known set, priority order, or another ordinary description.
- Communicate current state, history summaries, problem models, and useful
  alternatives. Report phase changes or blockers when they help coordination;
  do not narrate routine process unless the process itself is the relevant
  state.
- Communicate epistemic status when it matters. Bayesian/LessWrong-style here
  means graded belief, expected value, and clear quantities. English phrases are
  fine when precision is unimportant. Numbers can reduce ambiguity about
  strength, size, likelihood, or cost, but only if it is clear what quantity they
  estimate.
- Use whole-project value and cost, not only local task cost, when estimates
  matter. Measure relevant costs directly: shadow API cost even when a
  subscription means it is not paid directly, critical-path wall time and its
  effect on thesis submission, and actual Jörn attention time. Do not translate
  these through fixed hourly labor proxies. Determine the current bottleneck
  empirically instead of assuming Jörn time is it. Compute shadow API cost as
  `((input - cached_input) * I + cached_input * C + output * O) / 1e6`.
  Priority-tier `(I, C, O)` USD rates per million tokens, recorded 2026-07-16:
  `gpt-5.6-sol = (10, 1, 60)`, `gpt-5.6-terra = (5, 0.5, 30)`, and
  `gpt-5.6-luna = (2, 0.2, 12)`. Use this cached rate line immediately; do not
  pause ordinary cost estimates to refresh it.
- Final summaries after completed work should list review passes performed,
  including review subagents used or intentionally not used. Do not add ritual
  review summaries to small chat-only replies.

## Navigation

This repo does not use nested `AGENTS.md` files.

```text
.
|-- AGENTS.md
|-- JOERN.md
|-- FACTSHEET.md
|-- README.md
|-- Cargo.toml
|-- CAPABILITY_CLAIM_MAP.md
|-- thesis/
|   |-- main.tex
|   |-- *.tex
|   |-- MAP.md
|   |-- DEVELOPMENT.md
|   |-- bibliography.bib
|   |-- build/main.pdf
|   `-- legacy/
|       |-- README.md
|       `-- *
|-- crates/
|   |-- MAP.md
|   |-- symplectic/
|   |   |-- README.md
|   |   |-- Cargo.toml
|   |   |-- src/**/*.rs
|   |   |-- benches/*.rs
|   |   `-- tests/*.rs
|   |-- algebraic-numbers/
|   |   |-- README.md
|   |   |-- DEVELOPMENT.md
|   |   |-- Cargo.toml
|   |   |-- examples/*.rs
|   |   |-- src/*.rs
|   |   `-- tests/*.rs
|   `-- euclidean-polytopes/
|       |-- README.md
|       |-- DEVELOPMENT.md
|       |-- Cargo.toml
|       `-- src/*.rs
|-- formal/
|   |-- main.tex
|   |-- preamble.tex
|   |-- bibliography.bib
|   |-- *.md
|   `-- *.tex
|-- experiments/
|   |-- MAP.md
|   |-- figure_config.py
|   |-- dev-<algo>/
|   |-- numerics/
|   |-- performance/
|   |-- verification/
|   |-- sys-datascience/
|   |-- <topic>/Cargo.toml
|   |-- <topic>/src/**/*.rs
|   |-- <topic>/<experiment>/
|   |   |-- *.rs
|   |   |-- *.py
|   |   |-- *.jsonl
|   |   `-- figures/
|   `-- verification/sage/
|-- experiments/sys-datascience/
|   |-- README.md
|   |-- produce/
|   |-- tables/
|   `-- methods/
|-- papers/<abbreviationYear>/
|-- submit/
|   |-- README.md
|   |-- *.md
|   `-- *.pdf
|-- .agents/skills/<skill>/
|   |-- SKILL.md
|   |-- agents/openai.yaml
|   |-- references/*.md
|   `-- scripts/
|-- .codex/
|   `-- agents/<agent>.toml
|      (optional project roles; user/IDE settings stay in ~/.codex/config.toml)
|-- .worktrees/
|-- .devcontainer/
|   |-- README.md
|   |-- devcontainer.json
|   |-- Dockerfile
|   `-- *.sh
|-- scripts/
`-- /tmp/  (outside repo)
```

Start here:
- `README.md`: project target, first-entry navigation, and final-readiness
  frame.
- `FACTSHEET.md`: Jörn-confirmed project facts. Use it unless newer
  Jörn/Kai/source truth contradicts it.
- `submit/README.md`: submission/admin source files and official-form cache.
- Slice entry points: `thesis/MAP.md`, `experiments/MAP.md`,
  `crates/<crate>/MAP.md`, and owner-local README/MAP/content files.

Trust model:
- Source files, tests, data, owner-local notes, accepted Jörn/Kai decisions,
  and thesis text overrule maps and summaries.
- `CAPABILITY_CLAIM_MAP.md` is a non-authoritative cache of high-level
  capability claims.
- `MAP.md` files are navigation caches, not authoritative sources.
- `README.md` files are entry points. `DEVELOPMENT.md` files are
  maintainer-facing notes.

Important boundaries:
- Across the project, four-dimensional coordinates use the order
  `(q1, q2, p1, p2)`. Prefer coordinate-free notation when the order is
  irrelevant.
- `thesis/` is publishable thesis text. It is self-contained; assets and text
  are copied deliberately instead of linked from `experiments/` or `formal/`.
  `thesis/main.tex` inputs active text. `thesis/legacy/` is source material
  only.
- `formal/` is proof development, not publication text.
- `experiments/` keeps execution code, data, reports, and figures next to their
  producer.
- `.worktrees/` contains ignored local worktrees for independent sessions.
- `/tmp/` is scratch for drafts, subagent prompts, handoffs, and disposable chat
  artifacts; it is not durable project state.

Experiment routing:
- Route by what should move together. For example, numerical analysis of
  `flow_graph` can belong in `experiments/dev-flow-graph/` when it should move
  with algorithm development, or in `experiments/numerics/` when it should move
  with reusable numerical methodology.
- `experiments/dev-<algo>/`: active algorithm development before a settled
  downstream evidence home. Current examples: `dev-gradient-ascent/` and
  `dev-flow-graph/`.
- `experiments/numerics/`: reusable f64 vs exact behavior, numerical stability,
  numerical-error audits, and derivative/numerical validation.
- `experiments/performance/`: runtime, memory, counters, profiling, and
  compute-budget measurement.
- `experiments/verification/`: correctness/regression checks, capacity axioms,
  algorithm agreement, literature values, error paths, and slower validation.
- `experiments/sys-datascience/`: hostile `sys` search data-science pipeline
  and method-table packets.
- Topic folders such as `hko-local-maximum/`, `regular-products/`,
  `combinatorial-cells/`, and `sys-landscape/` own thesis-slice or topic-local
  producers and evidence when the local README says the topic owns them.

Documentation:
- Write for capable current GPT-5.6 agents, not weaker hypothetical agents. Use
  your own current model as the proxy for standard knowledge. If you
  know a standard term, library, or reasoning step, assume future agents can know
  it too. Document repo-specific facts, local conventions, source-truth links,
  decisions, evidence, sharp edges, and expensive checks; do not spell out
  generic reasoning current models can reconstruct. If a task intentionally
  uses a smaller model, put extra task structure in that prompt rather than in
  durable project documentation for every reader.
- Knowledge should live where future agents need it: code, comments, TeX,
  experiment artifacts, formal notes, thesis companions, experiment READMEs,
  crate documentation, generated outputs, owner-local task notes, or local
  documentation.
  Keep documentation lean, current, and easy to verify. Delete or demote
  obsolete notes; git history is enough for historical material.
- Keep task state and accepted external decisions in the repo when future work
  depends on them. For expensive or non-obvious checks, preserve the source
  pointers, commands, assumptions, intermediate results, and status needed to
  reproduce or reassess the result; do not preserve raw process that adds no
  checking value. Distinguish empirical from theoretical support and direct
  observation from inference when that changes what downstream work may claim.
- Use stable, grep-able terminology, symbols, labels, filenames, and
  cross-references. Avoid unstable line-number references; keep files
  single-concern when that materially improves discovery and maintenance.
- A passing test, verifier, benchmark, or review is evidence only for what it
  actually checks. Assess whether its conditions support the claimed result and
  whether an incomplete or corrupted artifact could pass it.

## Environment and commands

Supported baseline environment: local devcontainer at `/workspaces/msc-math`
with Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.

Use these as baseline commands. Verify locally when a command might be stale or
too broad for the task.

```bash
# Create local worktree. Most worktrees need zero checked-out LFS data.
git status --short
GIT_LFS_SKIP_SMUDGE=1 git worktree add .worktrees/lemma-cleanup -b lemma-cleanup main
cd .worktrees/lemma-cleanup

# Only for experiment-data/reproduction work:
# git lfs checkout
# git lfs pull --include path/to/file.jsonl

# Merge after branch review and with Jörn's approval
cd /workspaces/msc-math
git merge --ff-only lemma-cleanup
git worktree remove .worktrees/lemma-cleanup

# Rust crates
cargo fmt --check
cargo test -p symplectic --release --lib
cargo test -p algebraic-numbers --release
cargo test -p euclidean-polytopes
cargo clippy -p euclidean-polytopes --all-targets -- -D warnings

# Rust workspace and experiments
cargo build --workspace --release
cargo check -p exp-sys-landscape

# Python
# `python` is absent on Ubuntu 24.04; `python3` lacks undeclared packages.
uv run --with pyyaml --script /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/gpt-56-harness
uv run --script experiments/sys-landscape/random-sample/analyze.py # PEP 723 inline dependencies

# Profiling
cargo run -p symplectic --release --bin profile-pruned-hk2017 -- --facet-counts 8 --samples 3 --jsonl

# Thesis
cd thesis/ && latexmk && ./check-build.sh # output: thesis/build/main.pdf

# Formal math
cd formal/ && latexmk
```
