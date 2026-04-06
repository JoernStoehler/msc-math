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

High-level structure:

- `crates/` 
  - `Cargo.toml`: Workspace
  - `library/`: Rust library (the core) with stable-enough features
  - `exp-<group>/`: Research experiments, grouped by their main research question
    - `<subdir>`: Self-contained simple, straight attempt to get partial answers using some method
  - `dev-<group>/`: In-development features that aren't stable enough yet even for quick experiments
    - `<subdir>`: One direction of development, e.g. empirical analysis of numerical error

- `thesis/`: the publishable master thesis; self-contained
  - `assets/`: Figures and tables are copied deliberately from `crates/` into `thesis/assets/`
  - `main.tex`
  - `bibliography.bib`
- `papers/`:
  - `<abbreviationYear>/`: downloaded paper sources

- `TASKS.md`, `IDEAS.md`: list of ideas, todos, ongoing tasks in this project

- `CLAUDE.md`: Onboarding document read by every agent
- `.claude`: Claude Code files
  - `worktrees/`: Independent git worktrees to avoid conflicts
  - `settings.json`, `hooks/`, `skills/`, `agents/`
  - `rules/`: Autoloaded when an agent interacts with a matching path for the first time.
- `feedback/*.md`: Dump for incident reports about mistakes and friction that regularly lead to infrastructure improvements.

- `.devcontainer`: Explicit, reproducible development environment

## General Conventions

- **Headers**: Every source file has a comment block header explaining purpose, context, and quick takeaways. Module-level files additionally document the architecture.
- **Self-Contained Thesis**: The thesis folder is self-contained, and copies instead of linking to other folders. This avoids silent updates to figures or text.
- **Feature Lifecycle**: New features are first developed in a `dev-<group>/`, potentially with feedback based on one or more experiments that relate to the feature. Once the code is stable and approved, it migrates into `library/`, and validation experiments either become test suites or remain in the `dev-<group>` permanently.
- **Merge gating**: Never merge to `main` without Jörn's instruction. Never perform destructive operations (delete branches, force-push, reset) without asking.
- **Agent time is free, Jörn's time is expensive.** When choosing between spending more agent time (exploring alternatives, reading code, running experiments, rolling back failed attempts) and spending Jörn's time (asking questions, presenting incomplete work, leaving problems for him to catch) — spend agent time.
- **Mathematical Theory**: We 1:1 match rust code to math, using both the type system and pedantically written `math.tex` files that contain formalizations and proofs of the theory the code depends on and is inspired by. Basically any rust algorithm is accompanied by a correctness proof of its input-output contract. Math and code often are developed together. Jörn reviews the resulting `crates/main.pdf` file that includes all the `math.tex` files, and he checks both whether formalizations are meaningful and useful, and whether their proofs are correct and readible. Note: the `crates/**/math.tex` files are for development, while the `thesis/main.tex` file is for publication and uses different lemmas, proofs, and formulations, with a focus on thesis advisors as readers instead of development agents.

## Git Conventions

- Always use local `main`, never `origin/main`.
- Before committing: `cd crates/library/ && cargo test --release --lib` passes, `cargo clippy --lib -- -D warnings` is clean.
- Work in a worktree (separate branch) unless Jörn says otherwise. This keeps `main` clean and lets multiple sessions run in parallel without conflicts.
- **Git LFS** tracks `.jsonl` files (configured in `.gitattributes`). This is transparent — `git add`, `git commit`, `git push` work normally. Limits on GitHub free plan ([docs](https://docs.github.com/en/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)): 2 GB per file, 10 GiB storage, 10 GiB bandwidth/month. If an experiment binary produces output >2 GB, compress (gzip) or split into multiple files before committing. A pre-commit hook (`scripts/pre-commit`, symlinked into `.git/hooks/`) blocks files >10 MB that aren't LFS-tracked — if it fires, either add the file pattern to `.gitattributes` via `git lfs track` or add to `.gitignore`.

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
cd crates/ && cargo build --workspace --release           # build all (library + all experiment groups)

# Thesis
cd thesis/ && latexmk && ./check-build.sh                 # build + check

# Math (all proofs — crate + experiments)
cd crates/ && pdflatex math.tex && pdflatex math.tex      # includes all crates/**/math.tex files
```

## Terminology About Agents

We stick to the same terminology that Anthropic uses, and just are more specific/unambiguous.

- **Orchestration agent**: the agent running a chat session with Jörn. Decomposes tasks and delegates via Agent(). Loaded via `/orchestrate`.
- **Agent**: a Claude instance spawned via Agent() to do leaf work. Cannot spawn more agents. Returns a single message to the orchestration agent, plus whatever side-effects it had. Gets CLAUDE.md, MEMORY.md, rules, and skills automatically.
- **Delegation**: orchestration agent spawning an agent via Agent().

## Text that agents read

Optimize for these qualities (descending effort priority) when writing files, comments, or messages that other agents read:

1. **Correct, corrigible.** Verify claims against code or data. When text will inevitably be wrong, make errors findable and fixable by future agents — cite sources, state assumptions explicitly, include enough context to tell correct from incorrect.
2. **Verifiable, observable, measurable.** State things the reader can check. Write "the code matches lem:foo — both compute X by doing Y" not "the code is correct." Write "returns the smallest eigenvalue of M" not "returns the appropriate eigenvalue."
3. **Unambiguous, clear, specific.** Each sentence should have one reading. Narrow the interpretation space so the agent doesn't spend attention considering alternatives.
4. **Complete.** Include what the reader needs to understand and act. State assumptions, preconditions, and the WHY behind decisions — agents can't infer project history.
5. **Actionable, low-overhead.** The reader should know what to do after reading. Provide concrete next steps, not just observations.
6. **Simple, concrete, standard.** Familiar patterns, concrete examples, no unnecessary terminology. Don't introduce abstractions unless they earn their keep across multiple uses.

**Vague-word ban:** Do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" without specifying *what* makes it so.
