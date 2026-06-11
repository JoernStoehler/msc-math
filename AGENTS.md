# AGENTS.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: finished PDF to Kai by 16.6.2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Rules

These rules override default agent behavior where this thesis project needs a
more specific operating mode. They exist to fix common agent failures, not to
turn AGENTS.md into a full manual.

- Every session must serve thesis success. If the relation to thesis success is
  unclear, ask. Task definitions should explain how the task increases expected
  thesis success. Push back when a task or scope looks worse than an
  alternative. It is fine to make progress on an established task before all
  downstream uses are understood; restore thesis-level context during review so
  goal drift is caught.

- Agents own their work, even while the goal is still being chosen, scoped, or
  clarified. Jörn is available as mathematical expert, thesis stakeholder, and
  prompt/harness/agent-engineering expert. Agents should otherwise cover the
  roles needed to complete the work: developer, reviewer, tester, progress
  tracker, interviewer, devops operator, mathematician, and similar roles.

- Jörn has elevated access to LICCA, the devcontainer CLI, other Codex sessions,
  and mail. Everything else in the repo or local environment is available to
  agents directly. Do not ask Jörn to do accessible local/repo work; for the
  elevated resources, ask Jörn instead of treating access as impossible.

- Main must remain blocker-free so new sessions can spawn and merge independent
  work. Read-only inspection on Main is fine. Do not make repo-tracked changes
  on Main unless Jörn explicitly asks for that exact Main edit. For ordinary
  work, create a git worktree first, do the work there, and merge after review.

- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly asks for a harness edit. Discussion, planning,
  and read-only inspection are allowed.

### Chat with Jörn

Jörn's time should go to expert feedback, not large amounts of handholding or
session repair. Communication should be low-friction and focused on information
transfer, not presentation or narration.

- Write plain: ordinary words, existing thesis/repo terms, no metaphors, no
  analogies, no invented labels.
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
- Make questions, review requests, and other requests to Jörn hard to overlook.
  Usually put them on their own line or at the end of a short list. Re-ask or
  follow up if a request was missed or only partly answered.
- Use line breaks and light structure so Jörn can skip known parts quickly. Use
  numbers, short labels, or tables only when they make the message easier to
  read, answer, or refer to.
- Preserve precision that matters for communication. Do not shorten recaps if
  shortening loses the actual distinction.
- Make list type clear when ambiguity matters: exhaustive list, examples,
  current known set, priority order, or another ordinary description.
- Communicate current state, history summaries, problem models, and useful
  alternatives. Do not narrate process unless the process itself is the relevant
  state.
- Communicate epistemic status when it matters. Bayesian/LessWrong-style here
  means graded belief, expected value, and clear quantities. English phrases are
  fine when precision is unimportant. Numbers can reduce ambiguity about
  strength, size, likelihood, or cost, but only if it is clear what quantity they
  estimate.
- Use whole-project value and cost, not only local task cost, when estimates
  matter. Rough anchor: 1h Jörn labor = $300; 1h Codex labor = $30.
- Final summaries after completed work should list review passes performed,
  including review subagents used or intentionally not used. Do not add ritual
  review summaries to small chat-only replies.

## Navigation

This repo does not use nested `AGENTS.md` files.

```text
.
|-- AGENTS.md
|-- FACTSHEET.md
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
|   `-- *.tex
|-- experiments/
|   |-- MAP.md
|   |-- figure_config.py
|   |-- <topic>/Cargo.toml
|   |-- <topic>/src/**/*.rs
|   |-- <topic>/<experiment>/
|   |   |-- *.rs
|   |   |-- *.py
|   |   |-- *.jsonl
|   |   `-- figures/
|   `-- verification/sage/
|-- research/
|   |-- INDEX.md
|   |-- *.md
|-- experiments/sys-landscape/datascience/
|   |-- README.md
|   |-- dataset/
|   |-- produce/
|   |-- tables/
|   `-- methods/
|-- papers/<abbreviationYear>/
|-- tasks/
|   |-- README.md
|   |-- definition-of-success.md
|   |-- current-state.md
|   |-- planning-notes.md
|   |-- submit-thesis/
|   |   |-- README.md
|   |   |-- *.md
|   |   `-- *.pdf
|   |-- references/*.md
|-- .agents/skills/<skill>/
|   |-- SKILL.md
|   |-- agents/openai.yaml
|   |-- references/*.md
|   `-- scripts/
|-- .codex/
|   |-- agents/<agent>.toml
|   `-- config.toml
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
- `FACTSHEET.md`: Jörn-confirmed project facts. Use it unless newer
  Jörn/Kai/source truth contradicts it.
- `tasks/README.md`: live task model, steering, submission/admin source files,
  and final thesis-done checks.
- Slice entry points: `thesis/MAP.md`, `experiments/MAP.md`,
  `crates/<crate>/MAP.md`, and `research/INDEX.md`.

Trust model:
- Source files, tests, data, research notes, task files, and thesis text overrule
  maps and summaries.
- `CAPABILITY_CLAIM_MAP.md` is a non-authoritative cache of high-level
  capability claims.
- `MAP.md` files are navigation caches, not authoritative sources.
- `README.md` files are entry points. `DEVELOPMENT.md` files are
  maintainer-facing notes.

Important boundaries:
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

Documentation:
Knowledge should live where future agents need it: code, comments, TeX,
experiment artifacts, research notes, task files, generated outputs, or local
documentation. Keep documentation lean, current, and easy to verify. Delete or
demote obsolete notes; git history is enough for historical material.

## Environment and commands

Supported baseline environment: local devcontainer at `/workspaces/msc-math`
with Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.

Use these as baseline commands. Verify locally when a command might be stale or
too broad for the task.

```bash
# Create local worktree
git status --short
git worktree add .worktrees/lemma-cleanup lemma-cleanup
cd .worktrees/lemma-cleanup
git lfs checkout
git lfs pull --include path/to/file.jsonl # only if a required object is missing

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
uv run --with pyyaml --script /home/vscode/.codex/skills/.system/skill-creator/scripts/quick_validate.py .agents/skills/clarify-in-chat
uv run --script experiments/sys-landscape/random-sample/analyze.py # PEP 723 inline dependencies

# Profiling
cargo run -p symplectic --release --bin profile-pruned-hk2017 -- --facet-counts 8 --samples 3 --jsonl

# Thesis
cd thesis/ && latexmk && ./check-build.sh # output: thesis/build/main.pdf

# Formal math
cd formal/ && latexmk
```
