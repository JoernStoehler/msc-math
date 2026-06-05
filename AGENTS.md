# AGENTS.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: finished PDF to Kai by 9.6.2026; official submission facts still need
current-source refresh before final handin.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Implicit Objectives
Unless stated otherwise:
- Agents must contextualize their work, including task scope and review criteria, as instrumental for thesis success.
- Agents must escalate early and push back if their task is nonsense or sub-optimally set.
- Agents must minimize the amount of time Jörn has to spend (this is the one bottleneck for the tgesis timeline)
- Concretely, agents should not ask questions they know the answer to, make requests they can carry out, end a turn without good reason, split a questionaire into multiple messages, ask questions that Jörn cannot cheaply answer, withhold/skip gathering information that Jörn has to rederive on his own time then.
- Agents must own their task even if informal and undefined and cannot hand it off without approval from Jörn, including shifting responsibility and leadership and decisions to Jörn. Jörn is just another expert they can consult via explicit requests in chat.
- Everyone has to keep Main in a blocker-free state where new parallel agents with independent tasks can spawn and merge worktrees at any time.

## Chat Rules

When interacting with Jörn in chat:
- Write plain. Use zero metaphors and zero analogies and zero new terminology.
- Number/label everything so Jörn can reference it without ambiguity.
- Use progressive disclosure.
- Do not iterate complex messages in chat, such as plans and questionnaires.
  Draft and iterate in scratch, then copy the polished message to chat.
- Final summaries should list review passes performed, including review
  subagents used or intentionally not used.
- Use LessWrong-style clear communication norms: Explicit belief strength
  as probabilities, value and cost amounts as dollars.
- Estimate whole-project value and cost, not only the local cost of the
  immediate action. Use tactics like value of information, maintenance effort,
  attention cost, downside/upside risks, reference classes, and cruxes when
  useful.
- Only use tactics where they are useful, e.g. omit explicit belief strength
  commentary when it is not relevant, such as during babble-and-prune.
- Use as a rough anchor: 1h Jörn labor = $300, 1h codex labor = $30.

## Files

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

- `AGENTS.md`: root instruction map. This repo does not use nested `AGENTS.md`.
- `FACTSHEET.md`: Jörn-confirmed project facts. Use it for project context
  that agents can rely on without re-asking or second-guessing unless newer
  Jörn/Kai/source truth contradicts it.
- `CAPABILITY_CLAIM_MAP.md`: non-authoritative cache of high-level repo-capability
  claims, their scope, support, caveats, and refresh triggers. Source files,
  tests, data, research notes, task progress files, and thesis text overrule it.
- `Cargo.toml`, `**/Cargo.toml`: Rust workspace and package manifests.
- `**/README.md`: consumer-facing entry point for normal use.
- `**/DEVELOPMENT.md`: maintainer-facing notes for changing internals.
- `thesis/`: publishable thesis. Self-contained, assets and text are copied
  deliberately instead of linking to `experiments/`, `formal/`, etc.
  `thesis/main.tex` inputs the active thesis scaffold. `thesis/legacy/`
  contains source material only, not active thesis text.
- `crates/`: internal Rust crates with stable code shared across experiments.
- `formal/`: formalization and proofs for development, not for publication.
- `experiments/`: Rust/Python experiment packages. Execution code, data,
  reports, and figures are next to their producer.
- `research/`: notes with ideas, design, interpretations for development.
- `papers/<abbreviationYear>/`: raw sources of cited papers.
- `tasks/`: live task model, current steering, submission/admin source files,
  and final thesis-done checks. Start at `tasks/README.md`.
- `.agents/skills/`: repo-local skill surface.
- `.codex/agents/`: repo-local subagent templates (optional).
- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly asks for a harness edit.
- `.worktrees/`: ignored local worktrees for independent agent sessions.
- `.devcontainer/`: local devcontainer with documentation.
- `scripts/`: small repo helper commands.
- `/tmp/`: scratch space for subagent prompts, iterative drafts, and
  disposable chat artifacts; not durable project state.

## Map Files

The `MAP.md` files are navigation caches. They index, summarize and
structure the folder content for quick navigation.
They are not authoritative sources, and can be regenerated via subagent.

- `CAPABILITY_CLAIM_MAP.md`: high-level repo-capability claim cache. It is not a folder
  inventory. Use it for "what can this repo currently rely on?" questions and
  refresh affected rows from source truth when source behavior changes.
- `research/INDEX.md`: research questions and current status.
- `crates/<crate>/MAP.md`: api and architecture.
- `experiments/MAP.md`: tree of experiments and current status.
- `thesis/MAP.md`: chapter structure and current status.

## README.md Files

- `README.md` files are entry points for agents working on a project slice. They
  reference further reading material so relevant files are discoverable without
  opening the whole folder.
- `README.md` files may contain summaries, overviews, factual notes, plans,
  roadmaps, important considerations, conventions, and workflows when this
  reduces future agent cost.
- Such knowledge must be maintainable. Point to source truth when source truth
  exists. When no source truth exists, include enough context or reasoning for
  future agents to reconstruct and re-evaluate the claim.
- Most knowledge should live exactly where it is needed: text, code, comments,
  TeX, experiment artifacts, research notes, task files, generated outputs, or
  other local file content.
- Process knowledge was often written by gpt-5.5 Codex agents during local
  work. It is based on contextual assumptions that may need to be questioned or
  overridden. Jörn is available to help with context and maintenance.
- The basic decision calculus is to weigh the value of a piece of knowledge for
  future agents against the cost for agents who read it and the cost for agents
  who maintain it. Keep documentation lean; prefer clear, precise, trustworthy
  source truth over large secondary explanations.
- Delete or demote historical notes when their current value is lower than
  their verification and maintenance cost. Git history is enough for obsolete
  planning material.

## Environments

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with
  Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.

## Commands

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
cd thesis/ && latexmk && ./check-build.sh # output: `thesis/build/main.pdf`

# Formal math
cd formal/ && latexmk
```
