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
  build/main.pdf
crates/
  symplectic/src/
  symplectic/benches/
  algebraic-numbers/src/
  algebraic-numbers/benches/
  algebraic-numbers/tests/
formal/
  library/*.tex
  <topic>/*.tex
  main.tex
experiments/
  <topic>/Cargo.toml
  <topic>/<experiment>/main.rs
  <topic>/<experiment>/analyze.py
  verification/sage/
research/
contracts/
papers/<abbreviationYear>/
```

- `thesis/` is publishable and self-contained. It owns or copies publication
  assets and does not `\input` files from `formal/`, `experiments/`, or
  `crates/`.
- `crates/` contains durable Rust crates.
- `formal/` contains developer-facing mathematics for crates and experiments.
- `experiments/` contains Rust/Python experiment packages. Data and figures live
  next to the experiment that produced them.
- `research/` contains interpreted analysis, decision history, proof-route
  state, and topic summaries.
- `contracts/` contains canonical algorithm correspondence and verification
  contracts.
- `papers/<abbreviationYear>/` contains downloaded arXiv paper sources.

One-hop maps:

| Surface | Read when |
| --- | --- |
| `ROADMAP.md` | orienting on thesis closeout streams, current phase, or where a task belongs |
| `tasks/README.md` | editing `tasks/*.md` or interpreting task-bundle status/cache conventions |
| `research/INDEX.md` | looking for interpretation notes, proof-route state, or research-result caches |
| `crates/MAP.md` | navigating durable crate boundaries, API tiers, and core entities |
| `experiments/MAP.md` | navigating experiment topic packages, helper crates, data patterns, and provenance |
| `tasks/verify-thesis-done.md` | checking the once-run final thesis-done gate |
| `thesis/submission/README.md` | checking university forms, submission mechanics, or preservation actions |

Subtree `MAP.md` files are descriptive navigation caches, not always-loaded
instruction surfaces. Do not add nested `AGENTS.md` files as required
instruction maps; root-launched Codex sessions will not reliably load them.

## Domain Conventions

- **File headers:** Module-level source files start with a short purpose/context
  comment block. Small leaf files may rely on module docs and clear names.
  Detailed language-specific header rules live in the relevant convention
  skills.
- **Feature lifecycle:** New exploratory code starts in the relevant
  `experiments/` subtree. Stable, approved algorithms migrate into `crates/`.
  Validation experiments either become crate tests or remain in `experiments/`.
- **Test/validation boundary:** Crate tests are fast live checks for developer
  feedback and ordinary regressions. Slow mathematical validation, edge-case
  searches, broad random sweeps, and generated evidence datasets live in
  `experiments/`.
- **Math-code correspondence:** Rust code cross-references formal mathematics
  when correctness depends on a formal result. Use labels such as `[lem:label]`,
  `[thm:label]`, or `[def:label]`; pure orchestration does not need a label. The
  matching `\label{...}` lives in `formal/*.tex`.
- **Experiment paths:** Use semantic experiment paths. Do not force balanced
  subtrees when the semantics are asymmetric.
- **Research notes:** Put research-state notes, interpreted analysis, decision
  history, and next-step planning in `research/`. Keep only execution-facing
  packet docs under `experiments/`.
- **Data ownership:** Keep generated data with the producer that writes it.
  Avoid multiple binaries writing to the same tracked output.
- **Cross-file references:** Comments and notes should reference neighboring
  surfaces explicitly, e.g. `<file>.tex:\ref{label}`, `<file>.rs:symbol`, or
  `<file>.sage:symbol`.
- **JSONL / LFS safety:** `.jsonl` files are generated artifacts tracked by Git
  LFS. Trace figure, table, dataset, and experiment-artifact provenance with
  targeted `rg` and local source inspection. There is no repo-wide generated
  dataflow map; rebuild one only if repeated provenance work proves it is worth
  maintaining.

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
    repo-maintainability/design/
  worktrees/
.devcontainer/
scratch/
scripts/
```

- Use the discoverable skill whose name and description match the task; detailed
  conventions and validation live in the skill.
- `.codex/agents/` contains repo-local subagent definitions.
- `.codex/reference/harness/` contains reusable repo-local harness rules by
  concern.
- `.codex/reference/repo-maintainability/design/` contains durable
  maintainability design notes and inventories.
- `.codex/worktrees/` contains repo-local worktrees for isolated Codex sessions.
- `.devcontainer/` documents the local devcontainer and Codex web environment.
- `scratch/` is undocumented temporary material, not current convention text.
- `scripts/` contains helper scripts not tied to one runtime environment.

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
