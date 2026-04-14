# AGENTS.md

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
    - `<subdir>/`: One self-contained experiment (`run.rs`, `analyze.py`, `logbook.md`, `math.tex`)
  - `dev-<group>/`: Unstable features not yet ready for library or experiments
    - `<subdir>/`: One development direction, e.g. numerical analysis (`run.rs`, `analyze.py`, `logbook.md`, `math.tex`)

- `thesis/`: Publishable master thesis; self-contained, does not link to `crates/`
  - `assets/`: Figures and tables copied from `crates/` (not symlinked)
  - `main.tex`, `bibliography.bib`
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources

- `RESULTS.md`: What this project found and built — thesis content plan
- `TASKS.md`: Unified project tracker (tasks, experiments, ideas). Run `bash scripts/tasks-toc.sh` for a section index with line ranges.
- `feedback/*.md`: Incident reports; processed during workflow-update sessions
- `AGENTS.md`: Codex-native project instructions
- `.agents/`: Codex-native skills and rules
- `.codex/`: Codex config and subagents
- `.codex/worktrees/`: repo-local git worktrees for Codex sessions

## General Conventions

- **File headers**: Every source file starts with a comment block stating purpose and context. Module-level files additionally document the module's architecture.
- **Self-contained thesis**: `thesis/` copies figures and tables from `crates/` into `thesis/assets/` instead of linking. Never modify `thesis/` content from experiment code.
- **Feature lifecycle**: New code starts in `dev-<group>/`, informed by experiment results. Once stable and approved by Jörn, it migrates into `library/`. Validation experiments either become library tests or remain in `dev-<group>/`.
- **Merge gating**: Agents may merge to `main` only after the pre-merge workflow reports no blockers and Jörn has explicitly approved the merge. Destructive operations (delete branches on main, force-push, reset) still require asking.
- **Task ownership**: `[active]` means exactly one session owns the whole `###` task — the header and its intent, not a literal sub-list of body bullets. If a body bullet conflicts with the task goal, flag it; do not narrow ownership to the literal bullet.
- **Agent time is free, Jörn's time is expensive.** When choosing between spending more agent time (exploring alternatives, reading code, running experiments, rolling back failed attempts) and spending Jörn's time (asking questions, presenting incomplete work, leaving problems for him to catch) — spend agent time.
- **Define the check first.** Before acting, decide what will prove the task is done. Tool success is not task success.
- **Do the agent-reviewable passes before pinging Jörn.** Before asking Jörn to review a draft, packet, proof sketch, experiment write-up, or conclusion, first review it yourself and, when useful, with subagents for: clarity of language, document structure, skimmability, internal consistency, contradiction checks, factual claim vs code/data/source verification, fact-checkability, source attribution, explicit assumptions, explicit caveats, alignment with `RESULTS.md`, alignment with `TASKS.md`, alignment between thesis text and logbooks, alignment between thesis text and `math.tex`, alignment between text and code behavior, alignment between figures and the text that cites them, alignment between citations and bibliography keys, missing tests, missing verification steps, missing labels, missing cross-references, missing definitions, missing figure provenance, missing bibliography data, formatting, buildability, reproducibility, obvious edge cases, obvious counterexamples, obvious alternative interpretations, and scope drift. Ask Jörn only for the remainder that actually needs him: mathematical judgment, thesis-scope cuts, publication-facing emphasis, advisor-facing framing, taste, or external-world actions and decisions only he can take.
- **Do not promise a next step and then stop.** If you say you will run a review, make an edit, or fetch a diff, do it before sending another user-facing message. If you are blocked, say what blocked you instead of promising action you have not taken.
- **Do not hand back the turn with only status.** Not allowed: "I need to do X", "not done", "no blockers", "I guessed". Before replying, do the next step, ask one Jörn-only question, or report a real blocker.
- **Math-code correspondence**: Every non-trivial Rust algorithm has a correctness proof in its module's `math.tex`. Code and math are developed together and cross-referenced (`[lem:label]` in code, `\label{lem:label}` in math.tex). Jörn reviews `crates/main.pdf` for correctness and readability. The `crates/**/math.tex` files are for development agents; `thesis/main.tex` is for publication with thesis advisors as readers.

## Git Conventions

- Always use local `main`, never `origin/main`.
- Before merging to `main` (via pre-merge): `cd crates/library/ && cargo test --release --lib` passes, `cargo clippy --lib -- -D warnings` is clean. Tests gate merges, not commits.
- **Commits are free.** Do not ask permission to commit. If you need to ask about something commit-related, ask about the merge, not the commit.
- Work in a worktree (separate branch) unless Jörn says otherwise.
- **Git LFS** tracks `.jsonl` files (configured in `.gitattributes`). `git add`/`commit`/`push` work normally. Limits: 2 GB per file, 10 GiB storage, 10 GiB bandwidth/month ([docs](https://docs.github.com/en/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)). A pre-commit hook blocks files >10 MB that aren't LFS-tracked.

## Git Worktrees

- **Worktree default**: If you need to edit any tracked file outside `TASKS.md`, `AGENTS.md`, `.agents/`, `.codex/`, and `feedback/`, create a fresh worktree first.
- **Subagent default**: A subagent keeps using the repository copy it already has. It does not create a worktree unless the parent asks. It does not merge branches unless the parent asks.
- **Parent wording**: If the parent session wants a subagent to create a worktree or merge a branch, it must say that explicitly. Otherwise, the subagent should not do either.
- **Create command**: `git worktree add -b <branch> .codex/worktrees/<branch> main`
- **Reuse command**: `git worktree add .codex/worktrees/<branch> <branch>`
- **Enter a worktree**: `cd /workspaces/msc-math/.codex/worktrees/<branch>`
- **Remove a worktree after merge**: `git worktree remove .codex/worktrees/<branch>` then `git branch -d <branch>`
- **Branch base**: New worktree branches start from local `main`, not `origin/main`.

## Environment

Two supported environments exist:

- **Local devcontainer**: full baseline environment. See `.devcontainer/`.
- **Codex cloud**: lower-complexity travel/mobile environment for code work.
  See `codex-cloud.md`.

Local devcontainer baseline:

- Docker devcontainer at `/workspaces/msc-math`
- Rust 1.94, Python 3.12, TeX Live, gh CLI

Codex cloud v1 baseline:

- Default Codex `universal` image plus `bash scripts/codex-cloud-setup.sh`
- Rust build/test/clippy must work
- Python analysis must work on smoke-generated or otherwise hydrated inputs
- `git-lfs` is installed, but committed LFS files may still be pointer files in cloud
- TeX is intentionally out of scope in cloud v1

## Quick Commands

```bash
# Rust (library)
cd crates/library/ && cargo test --release --lib
cd crates/library/ && cargo clippy --lib -- -D warnings
cd crates/library/ && cargo test --release -- --ignored

# Rust (experiments)
cd crates/ && cargo build -p exp-<group> --release
cd crates/ && cargo build --workspace --release

# Thesis
cd thesis/ && latexmk && ./check-build.sh

# Math (all proofs — crate + experiments)
cd crates/ && latexmk
```

## Terminology

- **Top-level session**: the top-level agent session that talks with Jörn and coordinates or executes the current task as needed.
- **Subagent**: a Codex subagent declared under `.codex/agents/` and invoked through Codex delegation tools.
- **Delegation**: top-level session spawning a subagent or worker to do leaf work.

## Text that agents read

Optimize for these qualities (descending effort priority) when writing files, comments, or messages that other agents read:

1. **Correct, corrigible.** Verify claims against code or data. When text will inevitably be wrong, make errors findable and fixable — cite sources, state assumptions, include enough context to tell correct from incorrect.
2. **Verifiable, observable, measurable.** State things the reader can check. Write "the code matches lem:foo — both compute X by doing Y" not "the code is correct."
3. **Unambiguous, clear, specific.** Each sentence should have one reading.
4. **Complete.** Include what the reader needs to understand and act. State assumptions, preconditions, and the WHY behind decisions.
5. **Actionable, low-overhead.** The reader should know what to do after reading.
6. **Simple, concrete, standard.** Familiar patterns, concrete examples, no unnecessary abstractions.
7. **Literal wording.** Use precise terms with stable meanings. Do not use metaphors, slogans, or invented labels unless you define them and they remove ambiguity.

**Vague-word ban:** Do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" without specifying what makes it so.
