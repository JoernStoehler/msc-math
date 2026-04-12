# CLAUDE.md

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture

Planned deliverables:
1. A printed-quality LaTeX thesis (`thesis/build/main.pdf`)
2. A high-performance Rust library for symplectic geometry on polytopes (`crates/library/`)
3. A reproducible experiment pipeline (`crates/exp-*/`)

## Project Layout

- `crates/`
  - `Cargo.toml`: Workspace manifest
  - `main.tex`: Compiles all per-module `math.tex` files into `main.pdf`
  - `library/`: Rust library — proven algorithms with tests and math.tex proofs
  - `exp-<group>/`: Research experiments, grouped by research question
    - `<subdir>/`: One self-contained experiment (run.rs, analyze.py, logbook.md, math.tex)
  - `dev-<group>/`: Unstable features not yet ready for library or experiments
    - `<subdir>/`: One development direction, e.g. numerical analysis (run.rs, analyze.py, logbook.md, math.tex)

- `thesis/`: Publishable master thesis; self-contained, does not link to `crates/`
  - `assets/`: Figures and tables copied from `crates/` (not symlinked)
  - `main.tex`, `bibliography.bib`
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources

- `RESULTS.md`: What this project found and built — thesis content plan
- `TASKS.md`: Unified project tracker (tasks, experiments, ideas). Run `bash scripts/tasks-toc.sh` for a section index with line ranges.
- `feedback/*.md`: Incident reports; processed during `/update-workflow` sessions
- `CLAUDE.md`: This file — read by every agent
- `.claude/`: Settings, hooks, skills, agents, rules, worktrees

## General Conventions

- **File headers**: Every source file starts with a comment block stating purpose and context. Module-level files additionally document the module's architecture.
- **Self-contained thesis**: `thesis/` copies figures and tables from `crates/` into `thesis/assets/` instead of linking. Never modify `thesis/` content from experiment code.
- **Feature lifecycle**: New code starts in `dev-<group>/`, informed by experiment results. Once stable and approved by Jörn, it migrates into `library/`. Validation experiments either become library tests or remain in `dev-<group>/`.
- **Merge gating**: Agents may merge to `main` after a `/pre-merge` check reports no blockers. Destructive operations (delete branches on main, force-push, reset) still require asking.
- **Task ownership**: `[active]` means exactly one session owns the whole `###` task — the header and its intent, not a literal sub-list of body bullets. If a body bullet conflicts with the task goal, flag it; do not narrow ownership to the literal bullet.
- **Agent time is free, Jörn's time is expensive.** When choosing between spending more agent time (exploring alternatives, reading code, running experiments, rolling back failed attempts) and spending Jörn's time (asking questions, presenting incomplete work, leaving problems for him to catch) — spend agent time.
- **Math-code correspondence**: Every non-trivial Rust algorithm has a correctness proof in its module's `math.tex`. Code and math are developed together and cross-referenced (`[lem:label]` in code, `\label{lem:label}` in math.tex). Jörn reviews `crates/main.pdf` for correctness and readability. The `crates/**/math.tex` files are for development agents; `thesis/main.tex` is for publication with thesis advisors as readers.

## Git Conventions

- Always use local `main`, never `origin/main`.
- Before merging to `main` (via `/pre-merge`): `cd crates/library/ && cargo test --release --lib` passes, `cargo clippy --lib -- -D warnings` is clean. Tests gate merges, not commits.
- **Commits are free.** Do not ask permission to commit. If you need to ask about something commit-related, ask about the merge, not the commit.
- Work in a worktree (separate branch) unless Jörn says otherwise.
- **Git LFS** tracks `.jsonl` files (configured in `.gitattributes`). `git add`/`commit`/`push` work normally. Limits: 2 GB per file, 10 GiB storage, 10 GiB bandwidth/month ([docs](https://docs.github.com/en/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)). A pre-commit hook blocks files >10 MB that aren't LFS-tracked.

## Environment

- Docker devcontainer at `/workspaces/msc-math`
- Rust 1.94, Python 3.12, TeX Live, gh CLI

## Quick Commands

```bash
# Rust (library)
cd crates/library/ && cargo test --release --lib          # default test suite (<5s)
cd crates/library/ && cargo clippy --lib -- -D warnings   # lint
cd crates/library/ && cargo test --release -- --ignored   # full suite (slow)

# Rust (experiments)
cd crates/ && cargo build -p exp-<group> --release        # build one experiment group
cd crates/ && cargo build --workspace --release           # build all

# Thesis
cd thesis/ && latexmk && ./check-build.sh                 # build + check

# Math (all proofs — crate + experiments)
cd crates/ && latexmk                                     # builds main.pdf from main.tex
```

## Terminology

- **Orchestration agent**: the agent running a chat session with Jörn. Decomposes tasks and delegates via Agent(). Loaded via `/orchestrate`.
- **Agent**: a Claude instance spawned via Agent() to do leaf work. Cannot spawn further agents. Returns a single message plus file-system side effects. Gets CLAUDE.md, MEMORY.md, rules, and skills automatically.
- **Delegation**: orchestration agent spawning an agent via Agent().

## Text that agents read

Optimize for these qualities (descending effort priority) when writing files, comments, or messages that other agents read:

1. **Correct, corrigible.** Verify claims against code or data. When text will inevitably be wrong, make errors findable and fixable — cite sources, state assumptions, include enough context to tell correct from incorrect.
2. **Verifiable, observable, measurable.** State things the reader can check. Write "the code matches lem:foo — both compute X by doing Y" not "the code is correct."
3. **Unambiguous, clear, specific.** Each sentence should have one reading.
4. **Complete.** Include what the reader needs to understand and act. State assumptions, preconditions, and the WHY behind decisions.
5. **Actionable, low-overhead.** The reader should know what to do after reading.
6. **Simple, concrete, standard.** Familiar patterns, concrete examples, no unnecessary abstractions.

**Vague-word ban:** Do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" without specifying *what* makes it so.
