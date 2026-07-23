# AGENTS.md

## Objective

This repository supports Jörn Stöhler's master thesis, *Probing Viterbo's
Conjecture*, supervised by Kai Cieliebak and Elizabeth Gaar.

Every session should improve one of the three deliverables:

1. the printed-quality thesis at `thesis/build/main.pdf`;
2. the durable Rust crates under `crates/`;
3. the reproducible experiment pipeline and retained evidence under
   `experiments/`.

If the connection to those outcomes is unclear, ask before expanding the work.

## Start here

- `README.md`: project overview and first entry points.
- `ARCHITECTURE.md`: stable repository domains, ownership, and search routes.
- `docs/project-status.md`: current milestones and unresolved gates.
- `docs/project-facts.md`: Jörn-confirmed project and external facts.
- `thesis/README.md`, `formal/README.md`, `experiments/README.md`, and
  `crates/README.md`: owner-specific entry points.
- `submit/README.md`: submission sources and official-form cache.

Use ordinary text and symbol search after choosing an owner. Read the named
source before relying on a summary.

## Authority

Current source files, tests, data, producer outputs, owner-local notes,
accepted Jörn/Kai decisions, and active thesis text overrule summaries.

- `docs/project-facts.md` records still-current Jörn-confirmed facts.
- `docs/project-status.md` records project state, not mathematical truth.
- `docs/capabilities.md` is a cross-owner view, not independent evidence.
- `README.md` files are entry points. `DEVELOPMENT.md` files are maintainer
  notes.
- Generated artifacts must be regenerated from their producer; do not
  hand-edit them.
- Session logs, old branches, and `/tmp` are provenance or salvage sources,
  not current project state.

Absence from a README or search result does not establish absence from the
project. Before declaring a project-wide proof, experiment, or implementation
gap, search the relevant owners and report the searched scope.

## Repository boundaries

- `thesis/` is publication text. `thesis/main.tex` defines the active PDF.
  Content companions support writing but are not mathematical source truth.
- `formal/` is proof development and may contain stronger, weaker, or
  superseded routes not used by the thesis.
- `experiments/` keeps producer code, inputs, outputs, interpretation, and
  reproduction instructions with the question they serve.
- `crates/` contains reusable Rust libraries. Follow normal Cargo layout.
- `papers/` contains source papers and paper-specific notes.
- `submit/` contains submission/admin sources.
- `.worktrees/` contains isolated local worktrees and is not project content.
- `/tmp/` is disposable scratch.

Across the project, four-dimensional coordinates use `(q1, q2, p1, p2)`.
Prefer coordinate-free notation when the order is irrelevant.

## Working rules

- Main is read-only unless Jörn explicitly requests that exact Main edit.
  Ordinary changes use a worktree and reach Main only after review and Jörn's
  merge approval.
- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly requests harness work.
- Preserve unrelated user changes in dirty worktrees.
- Do not ask Jörn to do accessible local work. Ask for mathematical or
  stakeholder cruxes, private context, LICCA access, mail, or admin actions.
- Continue until the assigned outcome is complete, explicitly paused, or
  concretely blocked.
- Use bounded exploration or review agents when their separate context repays
  the handoff. Main owns target choice, integration, and final judgment.
- Keep communication plain and low-friction. Ask one crux at a time and put
  questions needing Jörn's answer in the final message.
- For a repository or harness diff needing Jörn's review, place the exact diff
  under `/tmp/joern/` and name its base and candidate.

## Documentation

Put durable knowledge with the narrowest owner that future work will inspect.
Document repository-specific facts, decisions, evidence, status, source paths,
sharp edges, and expensive checks. Do not duplicate generic knowledge.

Prefer conventional filenames and stable, grep-able terms. Keep active and
superseded material visibly distinct. When a result's scope or rationale is not
recoverable from the artifact itself, state it beside the artifact.

Navigation views must say what they cover. A view is not evidence, and an
incomplete view must not imply a complete inventory.

## Baseline commands

```bash
# Worktree
GIT_LFS_SKIP_SMUDGE=1 git worktree add .worktrees/<name> -b <branch> main

# Rust
cargo fmt --check
cargo test -p symplectic --release --lib
cargo test -p algebraic-numbers --release
cargo test -p euclidean-polytopes
cargo build --workspace --release

# Thesis
cd thesis && latexmk && ./check-build.sh

# Formal proof-development document
cd formal && latexmk
```

Experiment READMEs own their producer commands and output-safety rules. Read
the local README before running a command that may overwrite tracked evidence.
