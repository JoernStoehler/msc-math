# AGENTS.md

This file is the always-loaded repo map. It should help agents find the right
surface quickly and carry only context that is broadly useful across tasks.
Detailed workflows and conventions live in skills, topic maps, and task files.

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Domain Map

```text
thesis/
  main.tex
  *.tex
  bibliography.bib
  build/main.pdf
crates/
  MAP.md
  symplectic/src/
    lib.rs
    **/*.rs
    **/test_*.rs
  symplectic/benches/
    *.rs
  algebraic-numbers/src/
    lib.rs
    *.rs
    test_*.rs
  algebraic-numbers/benches/
    *.rs
  algebraic-numbers/tests/
    *.rs
formal/
  main.tex
  library/*.tex
  <topic>/*.tex
experiments/
  MAP.md
  figure_config.py
  combinatorial-cells/
  crosspolytope/
  hko-local-maximum/
  numerics/
  sys-landscape/
  verification/
  visualization/
  <topic>/Cargo.toml
  <topic>/src/lib.rs
  <topic>/<experiment>/*.rs
  <topic>/<experiment>/*.py
  <topic>/<nested-package>/Cargo.toml
  verification/sage/
research/
contracts/
  README.md
  registry.toml
  *.md
papers/<abbreviationYear>/
```

- `thesis/` is publishable and self-contained. It owns or copies publication
  assets and does not `\input` files from `formal/`, `experiments/`, or
  `crates/`.
- `crates/` contains durable Rust crates.
- `formal/` contains developer-facing mathematics for crates and experiments.
- `experiments/` contains Rust/Python experiment packages. Data and figures live
  next to the experiment that produced them. Entrypoint and analysis filenames
  vary by package; use `experiments/MAP.md` and local manifests before assuming
  `main.rs` or `analyze.py`.
- `research/` contains interpreted analysis, decision history, proof-route
  state, and topic summaries.
- `contracts/` contains canonical algorithm correspondence and verification
  contracts. It is a documentation/registry surface, not imported runtime code.
- `papers/<abbreviationYear>/` contains downloaded arXiv paper sources.

Domain one-hop maps:

| Surface | Read when |
| --- | --- |
| `research/INDEX.md` | looking for interpretation notes, proof-route state, or research-result caches |
| `crates/MAP.md` | navigating durable crate boundaries, API tiers, and core entities |
| `experiments/MAP.md` | navigating experiment topic packages, helper crates, data patterns, and provenance |
| `thesis/submission/README.md` | checking university forms, submission mechanics, or preservation actions |

Subtree `MAP.md` files are descriptive navigation caches, not always-loaded
instruction surfaces. Do not add nested `AGENTS.md` files as required
instruction maps; root-launched Codex sessions will not reliably load them.

## Harness Map

Open the relevant harness reference when a task touches session behavior, Git,
worktrees, planning, verification, or agent-facing text.

```text
.agents/
  skills/
.codex/
  agents/
  reference/
    harness/session-rules.md
    harness/worktrees-and-git.md
    harness/planning-and-verification.md
    harness/text-for-agents.md
    domain/conventions.md
    repo-maintainability/design/
  worktrees/
ROADMAP.md
tasks/
  README.md
  verify-thesis-done.md
.devcontainer/
scratch/
scripts/
```

- Use the discoverable skill whose name and description match the task; detailed
  conventions and validation live in the skill.
- `.codex/agents/` contains repo-local subagent definitions.
- `.codex/reference/harness/` contains reusable repo-local harness rules by
  concern.
- `.codex/reference/domain/conventions.md` contains broad domain conventions
  for agents. Language-specific details live in the matching skills.
- `.codex/reference/repo-maintainability/design/` contains durable
  maintainability design notes and inventories.
- `.codex/worktrees/` contains repo-local worktrees for isolated Codex sessions.
- `ROADMAP.md` and `tasks/` route work, cache task state, and describe
  objectives; domain details usually live in `research/` or the relevant domain
  surface.
- `.devcontainer/` documents the local devcontainer and Codex web environment.
- `scratch/` is undocumented temporary material, not current convention text.
- `scripts/` contains helper scripts not tied to one runtime environment.

Harness one-hop maps:

| Surface | Read when |
| --- | --- |
| `ROADMAP.md` | orienting on thesis closeout streams, current phase, or where a task belongs |
| `tasks/README.md` | editing `tasks/*.md` or interpreting task-bundle status/cache conventions |
| `tasks/verify-thesis-done.md` | checking the once-run final thesis-done gate |

## Environment

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with
  Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.
- Codex web environment: lower-complexity environment for web sessions. See
  `.devcontainer/codex-cloud.md`; TeX is intentionally out of scope there.

Quick commands:

```bash
# Rust crates
cargo test -p symplectic --release --lib
cargo clippy -p symplectic --lib -- -D warnings
cargo test -p symplectic --release -- --ignored

# Rust workspace and experiments
cargo build --workspace --release
cargo build -p exp-<topic> --release

# Thesis
cd thesis/ && latexmk && ./check-build.sh

# Formal math
cd formal/ && latexmk
cd formal/library/ && latexmk
```
