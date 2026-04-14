# AGENTS.new.rules.md

## Rules And Conventions

### Working style

- Keep the project goal in view: thesis PDF, library, and reproducible experiments.
- Define the check first. Decide what evidence will show the task is done before editing.
- Spend agent time, not Jörn's time. Explore, verify, rerun, and self-review before asking.
- Do not hand back the turn with only status. Before replying, do the next step, ask one Jörn-only question, or report a real blocker.
- Do not promise a next step and then stop. If you say you will run something, run it before the next user-facing message.
- Verify claims against code, data, or builds. State assumptions and cite exact files, labels, rows, or commands when possible.
- Use literal, concrete wording. Avoid vague words like `appropriate`, `properly`, `good`, `reasonable`, `robust` unless you define the criterion.

### Git and workspace

- Use local `main`, never `origin/main`.
- Commits are free. Do not ask permission to commit. Ask only about merge decisions.
- Work in a fresh worktree before editing tracked files outside `TASKS.md`, `AGENTS.md`, `.agents/`, `.codex/`, and `feedback/`, unless Jörn explicitly says to work on `main`.
- Do not merge to `main` without Jörn's explicit approval.
- Do not use destructive git commands unless Jörn explicitly asks.

### Repo conventions

- Every source file starts with a header comment describing purpose and context. Module-level files also describe architecture.
- `thesis/` is self-contained. Copy figures and tables into `thesis/assets/`; do not wire thesis builds directly to experiment code.
- New unstable ideas start in `crates/dev-*`. Stable validated code moves into `crates/library/`.
- `TASKS.md` is the tracker. Update status tags when work changes the actual task state.
- `logbook.md` is the entry point for experiments. Record motivation, attempts, outcomes, and exact sources for numerical claims.
- Keep experiment pipelines reproducible: Rust binary -> data file -> Python analysis -> figure.

### Rust and math

- Read the colocated `math.tex` before editing non-trivial `.rs` files in the same module.
- Non-trivial mathematical code needs a matching result in `math.tex`.
- Use cross-references like `[lem:label]`, `[thm:label]`, `[def:label]`; never hardcode rendered theorem numbers.
- Never invent labels. If the math is missing, leave a TODO pointing to `math.tex`.
- Doc comments, formulas, invariants, tests, and code must match literally, not approximately.
- In math code, prefer mathematical enums over `Option<T>` for case distinctions.
- If the math is violated, panic rather than silently recovering.

### `math.tex` and thesis text

- `math.tex` is the single source of truth for proofs, formal definitions, and derivations tied to code.
- Put motivation and empirical interpretation in `logbook.md`, not `math.tex`.
- Use unique labels of the form `\label{type:name}`.
- Mark uncertain content with `% [TODO: JÖRN - ...]` or `% [GAP - ...]`.
- Thesis `.tex` is publication prose for humans, not a dump of development math.
- Captions state observations, not interpretations.

### Python and figures

- Experiment Python scripts are self-contained and run with `uv run analyze.py`.
- Keep dependencies in PEP 723 inline metadata when needed.
- Use `crates/figure_config.py` for figure setup and size constants.
- Do not hardcode absolute paths, `dpi=`, `bbox_inches=`, or ad hoc figsize values.
- Use `r"$...$"` for math labels.
- Keep figure styling consistent within an experiment.

## Quick Commands

```bash
# Search
rg "pattern"
rg --files

# Worktree
git worktree add -b <branch> .codex/worktrees/<branch> main
cd .codex/worktrees/<branch>

# TASKS index
bash scripts/tasks-toc.sh

# Rust library
cd crates/library/ && cargo test --release --lib
cd crates/library/ && cargo clippy --lib -- -D warnings
cd crates/library/ && cargo test --release -- --ignored

# Workspace / experiments
cd crates/ && cargo build --workspace --release
cd crates/ && cargo build -p exp-<group> --release
cd crates/ && cargo run -p exp-<group> --release --bin <name>

# Python analysis
cd crates/exp-<group>/<subdir>/ && uv run analyze.py

# Thesis and math
cd thesis/ && latexmk && ./check-build.sh
cd crates/ && latexmk

# Label lookup
cd crates/ && grep 'lem:<label>' main.aux
cd thesis/ && grep '<label>' build/main.aux
```

## Quality Gates

Run the gates that match the files you changed before presenting work to Jörn.

- Re-read the original task and check that the work actually serves that goal.
- Review every changed file for rule adherence. Use a review subagent per changed file family if available; otherwise do the same checks manually.
- For changed Rust library code: run `cargo test --release --lib` and `cargo clippy --lib -- -D warnings`.
- For changed workspace code: run `cd crates/ && cargo build --workspace --release`.
- For changed experiment binaries: compile and smoke-run the binary with the smallest valid input or `--help`.
- For changed `math.tex`: run `cd crates/ && latexmk`.
- For changed thesis files: run `cd thesis/ && latexmk && ./check-build.sh`.
- For changed Python analysis: run the script with `uv run analyze.py` and inspect the generated outputs.
- For changed claims, figures, proofs, or cross-references: verify them against data, code, generated artifacts, and `.aux` files.
- If code changed after committed data was generated, regenerate the affected data or mark the mismatch explicitly.
- Update `TASKS.md` if task status, blockers, or follow-up work changed.
