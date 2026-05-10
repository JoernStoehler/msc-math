# AGENTS.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of May 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Files

```text
.
|-- AGENTS.md
|-- Cargo.toml
|-- thesis/
|   |-- main.tex
|   |-- *.tex
|   |-- bibliography.bib
|   |-- build/main.pdf
|-- crates/
|   |-- MAP.md
|   |-- symplectic/
|   |   |-- README.md
|   |   |-- Cargo.toml
|   |   |-- src/**/*.rs
|   |   |-- benches/*.rs
|   |   `-- tests/*.rs
|   `-- algebraic-numbers/
|       |-- README.md
|       |-- DEVELOPMENT.md
|       |-- Cargo.toml
|       |-- examples/*.rs
|       |-- src/*.rs
|       `-- tests/*.rs
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
|   `-- sys-landscape-datascience/
|-- papers/<abbreviationYear>/
|-- tasks/
|   |-- MAP.md
|   |-- README.md
|   |-- submit-thesis/
|   |   |-- *.md
|   |   `-- *.pdf
|   |-- references/*.md
|   |-- submit-thesis.md
|   |-- verify-thesis-done.md
|   `-- <group>.md
|-- .agents/skills/<skill>/
|   |-- SKILL.md
|   |-- agents/openai.yaml
|   |-- references/*.md
|   `-- scripts/
|-- .codex/
|   |-- agents/<agent>.toml
|   |-- config.toml
|   `-- worktrees/
|-- .devcontainer/
|   |-- README.md
|   |-- codex-cloud.md
|   |-- devcontainer.json
|   |-- Dockerfile
|   `-- *.sh
|-- scripts/
|   |-- codex-worktree.sh
|   `-- toc.sh
`-- /tmp/  (outside repo)
```

- `AGENTS.md`: root instruction map. This repo does not use nested `AGENTS.md`.
- `Cargo.toml`, `**/Cargo.toml`: Rust workspace and package manifests.
- `**/README.md`: consumer-facing entry point for normal use.
- `**/DEVELOPMENT.md`: maintainer-facing notes for changing internals.
- `thesis/`: publishable thesis. Self-contained, assets and text are copied
  deliberately instead of linking to `experiments/`, `formal/`, etc.
- `crates/`: internal Rust crates with stable code shared across experiments.
- `formal/`: formalization and proofs for development, not for publication.
- `experiments/`: Rust/Python experiment packages. Execution code, data,
  reports, and figures are next to their producer.
- `research/`: notes with ideas, design, interpretations for development.
- `papers/<abbreviationYear>/`: raw sources of cited papers.
- `tasks/`: durable task tracking, current steering, submission/admin source
  files, and final thesis-done checks.
- `.agents/skills/`: repo-local skill surface.
- `.codex/agents/`: repo-local subagent templates (optional).
- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly asks for a harness edit.
- `.codex/worktrees/`: isolated worktrees for independent agent sessions.
- `.devcontainer/`: local devcontainer with documentation.
- `scripts/`: small repo helper commands.
- `/tmp/`: scratch space for subagent prompts, iterative drafts, and
  disposable chat artifacts; not durable project state.

## Map Files

The `MAP.md` files are navigation caches. They index, summarize and
structure the folder content for quick navigation.
They are not authoritative sources, and can be regenerated via subagent.

- `tasks/MAP.md`: dependency map of upcoming tasks and current status.
- `research/INDEX.md`: research questions and current status.
- `crates/<crate>/MAP.md`: api and architecture.
- `experiments/MAP.md`: tree of experiments and current status.
- `thesis/MAP.md`: chapter structure and current status.

## Review

Final summaries should list review passes performed, including review subagents
used or intentionally not used.

## Commands

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with
  Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.
- Codex web environment: lower-complexity environment for web sessions. See
  `.devcontainer/codex-cloud.md`; TeX is intentionally out of scope there.

Quick commands:

```bash
# Harness and maps
git diff --check
bash scripts/toc.sh AGENTS.md MAP_OR_TASK_FILE.md

# Rust crates
cargo test -p symplectic --release --lib
cargo clippy -p symplectic --lib -- -D warnings
cargo test -p symplectic --release -- --ignored
cargo test -p algebraic-numbers --release
cargo clippy -p algebraic-numbers --all-targets -- -D warnings

# Rust workspace and experiments
cargo build --workspace --release
cargo check -p PACKAGE_NAME
cargo build -p PACKAGE_NAME --release

# Thesis
cd thesis/ && latexmk && ./check-build.sh
perl -ne 'if (/\\newlabel\{LABEL_NAME\}\{\{([^}]*)\}\{([^}]*)\}/) { print "number=$1 page=$2\n" }' thesis/build/main.aux

# Formal math
cd formal/ && latexmk
rg -n -A 10 -F '\label{LABEL_NAME}' formal/*.tex
```
