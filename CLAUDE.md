# CLAUDE.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: mid-April 2026.
Topic: Probing Viterbo's Conjecture

Three planned deliverables:
1. A printed-quality LaTeX thesis (`thesis/build/main.pdf`)
2. A high-performance Rust library for symplectic geometry on polytopes (`crates/`)
3. A reproducible experiment pipeline (`experiments/`)

## Project Layout

```
crates/                    Rust library (the core)
  Cargo.toml
  src/
    lib.rs                 crate root
    geom/                  polytopes and basic euclidean and symplectic geometry
    kkt/                   general KKT solver
    algorithms/            different algorithms for the EHZ capacity 
    derivatives.rs         derivative of the capacity in the dual vertices
    dataset.rs             polytope datasets
    **/math.tex            correctness proofs (one per module)

math.tex                     root math.tex: compiles ALL crate + experiment proofs into one PDF
                             (cross-references between experiments and crate lemmas resolve here)

experiments/               each experiment is a self-contained directory
  <name>/
    run.rs                 binary to create the data files
    *.jsonl, *.csv         data files
    analyze.py             postprocessing, analysis, figures and tables
    logbook.md             experiment logbook, what was done, results, learnings, ideas
    math.tex               correctness proofs for the experiment
    
thesis/
  main.tex                 master document
  *.tex                    chapter files
  bibliography.bib         citations
  build/                   latexmk output

papers/
  <abreviationYear>/
    *.tex                  arXiv paper sources for reading

handoffs/
  *.md                     temporary task handoff files for future sessions
TASKS.md                   master task list, project management
IDEAS.md                   research directions and experiment ideas

.devcontainer/             devcontainer config, access method docs

CLAUDE.md, .claude/        agent configuration
  rules/                   path-scoped rules (auto-loaded by file pattern)
  agents/                  subagent definitions
  skills/                  skill workflows (each a directory with SKILL.md)
  hooks/                   shell hooks for session/worktree events
  output-styles/           output style definitions
  memory/                  persistent cross-session memory
  settings.json            Claude Code settings

archaeology/               untrusted files from abandoned predecessor repo
feedback/                  raw agent-design observations (rules, skills, agents, output style)
```

**Navigating source files:** Every source file has a header explaining purpose and context (Rust: `//!` doc comments, Python: docstring, LaTeX: `%` block). Module-level files (mod.rs, main .tex includes) additionally document the module group's architecture.

**Key architectural patterns:**
- The thesis is independent of both library and experiments code, documentation and math.tex files. Unlike the rest of the repo, it is optimized for human readers and for final publication, not for the agents who develop the project. It heavily copies from the math.tex files, uses produced asset figures and tables, and presents algorithms, theorems, experiment results, and other insights from the project to the human readers. Jörn reviews main.pdf, not .tex files.
- **Code lifecycle: experiment → library.**
  - New algorithms and verification code start as experiments (`experiments/`). Experiments are sandboxes: iterate freely, break things, explore. Each experiment is self-contained — don't modify another experiment or library code for one experiment's needs; copy what you need.
  - When experiment code is stable and used by ≥2 experiments, promote it to `crates/` with tests and math.tex proofs. This is the only path into the library.
  - The library (`crates/`) contains proven stable algorithms. Changes must pass `cargo test --release --lib` and `cargo clippy`. Don't experiment in the library.
  - Jörn reviews math.pdf and logbook.md, not .tex, .rs, .py files.
- math.tex files live alongside code in the library and experiments, and are independent of thesis/. They prove the correctness of the code and of other mathematical claims, and they serve as documentation for developers about how the algorithm works on a mathematical level, and they ensure code is correct by formalizing claims and proving claims in LaTeX. Jörn reviews math.pdf, not math.tex files.
- Polished workflows and conventions and best practice tips are provided to the agents, so that they work effectively and minimize the use of Jörn's limited time. Agent time is priced at $0/h, due to the flatrate Anthropic Max $200/mo subscription, but Jörn's time is limited.

## Core Rule

Never write a factual claim without verifying it against evidence in the same session. "The code does X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` to track and assign it to Jörn for manual verification.

**Citation verification:** Never produce author names or paper titles from memory. Verify against `thesis/bibliography.bib` or `papers/`. Agents confidently produce wrong names (e.g. "Cieliebak-Hutchings" instead of the correct "Chaidez-Hutchings").

**External systems:** When documenting external systems (LICCA cluster, university services), link to official documentation — do not paraphrase it. Agent paraphrases go stale silently and are unverifiable.

## Decision Authority

| | Cheap to verify | Expensive to verify |
|---|---|---|
| **Easy rollback** | Act freely | Act, then Jörn verifies |
| **Hard rollback** | Discuss first | Discuss first |

Never without Jörn's instruction: destructive operations, merging to `main`, modifying `.claude/` procedural files.

## Session Workflow

**Scope** (Jörn + agent): Jörn scopes. Agents provide investigation findings, and suggest scope expansion/contraction, but Jörn decides. Agents ask clarifying questions to ensure they and Jörn understand the scope the same way. Agents track scope provenance in the plan file.

**Plan → implement → review** (agent autonomous): No Jörn involvement unless specifically requested. Agents may return to earlier phases.

**Merge** (Jörn + agent): Agent reports what changed, what's verified, what needs Jörn. Jörn gates merges to `main`.

**Long sessions:** Update the plan file as you work — it survives compaction, working memory does not. Write design decisions and their WHY into the plan. After compaction, read the plan file to recover context.

**Subagents:** Delegate aggressively — N files → N parallel subagents. Subagents self-serve skills and rules (shared system prompt), no special prompting needed. Use review agents (review-proof, review-claims, review-formalization, etc.) proactively before presenting work.

## Git

- Always use local `main`, never `origin/main`.
- Before committing: `cargo test --release --lib` passes, `cargo clippy --lib -- -D warnings` is clean.
- Work in a worktree (separate branch) unless Jörn says otherwise. This keeps `main` clean and lets multiple sessions run in parallel without conflicts.

## Environment

- Docker devcontainer at `/workspaces/msc-math`
- Rust 1.94, Python 3.12, TeX Live, gh CLI
- `rm` is aliased to `trash-put` for safety
- `archaeology/` is in the repo but untrusted — do not rely on its contents

## Quick Commands

```bash
# Rust
cd crates/ && cargo test --release --lib          # default test suite (<5s)
cd crates/ && cargo clippy --lib -- -D warnings   # lint
cd crates/ && cargo test --release -- --ignored   # full suite (slow)

# Thesis
cd thesis/ && latexmk && ./check-build.sh         # build + check

# Math (all proofs — crate + experiments)
pdflatex math.tex && pdflatex math.tex            # root math.pdf (two passes)

# Experiments
cd experiments/ && cargo build --release          # build experiment binaries
```
