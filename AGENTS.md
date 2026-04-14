# AGENTS.md

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture

Planned deliverables:
1. A printed-quality LaTeX thesis (`thesis/build/main.pdf`)
2. A high-performance Rust library for symplectic geometry on polytopes (`library/`)
3. A reproducible experiment pipeline (`experiments/`)

## Project Layout

- `library/`: Rust library -- proven algorithms with tests and `math.tex` proofs
- `experiments/`: Research experiments, grouped by research question
  - `<group>/`: Experiment package
    - `<subdir>/`: One self-contained experiment (`main.rs`, `analyze.py`, `logbook.md`, `math.tex`)
  - Development subtrees stay inside the relevant group until they are stable.
- `formal/`: Developer-facing mathematical sources for `library/` and `experiments/`
- `research/`: Design notes, method selection, and experiment plans
- `thesis/`: Publishable master thesis; self-contained, does not link to `library/`, `experiments/`, or `formal/`
  - `assets/`: Figures and tables copied from `experiments/` (not symlinked)
  - `main.tex`, `bibliography.bib`
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources

- `RESULTS.md`: What this project found and built — thesis content plan
- `TASKS.md`: Unified project tracker (tasks, experiments, ideas). Run `bash scripts/tasks-toc.sh` for a section index with line ranges.
- `feedback/*.md`: Incident reports; processed during workflow-update sessions
- `AGENTS.md`: Codex-native project instructions
- `.agents/`
  - `skills/`: Codex-native skills (workflows and conventions)
- `.codex/`
  - `config.toml`: Codex config
  - `agents/*.toml`: Codex-native subagents
  - `worktrees/`: repo-local git worktrees for Codex sessions

## General Conventions

- **File headers**: Every source file starts with a comment block stating purpose and context. Module-level files additionally document the module's architecture.
- **Self-contained thesis**: `thesis/` copies figures and tables from `experiments/` into `thesis/assets/` instead of linking. Never modify `thesis/` content from experiment code.
- **Feature lifecycle**: New code starts in the relevant `experiments/` subtree, informed by experiment results. Once stable and approved by Jörn, it migrates into `library/`. Validation experiments either become library tests or remain in `experiments/`.
- **Merge gating**: Agents may merge to `main` only after the pre-merge workflow reports no blockers and Jörn has explicitly approved the merge. Destructive operations (delete branches on main, force-push, reset) still require asking.
- **Task ownership**: `[active]` means exactly one session owns the whole `###` task — the header and its intent, not a literal sub-list of body bullets. If a body bullet conflicts with the task goal, flag it; do not narrow ownership to the literal bullet.
- **Agent time is free, Jörn's time is expensive.** When choosing between spending more agent time (exploring alternatives, reading code, running experiments, rolling back failed attempts) and spending Jörn's time (asking questions, presenting incomplete work, leaving problems for him to catch) — spend agent time.
- **Define the check first.** Before acting, decide what will prove the task is done. Tool success is not task success.
- **Do the agent-reviewable passes before pinging Jörn.** Before asking Jörn to review a draft, packet, proof sketch, experiment write-up, or conclusion, first review it yourself and, when useful, with subagents for: clarity of language, document structure, skimmability, internal consistency, contradiction checks, factual claim vs code/data/source verification, fact-checkability, source attribution, explicit assumptions, explicit caveats, alignment with `RESULTS.md`, alignment with `TASKS.md`, alignment between thesis text and logbooks, alignment between thesis text and `math.tex`, alignment between text and code behavior, alignment between figures and the text that cites them, alignment between citations and bibliography keys, missing tests, missing verification steps, missing labels, missing cross-references, missing definitions, missing figure provenance, missing bibliography data, formatting, buildability, reproducibility, obvious edge cases, obvious counterexamples, obvious alternative interpretations, and scope drift. Ask Jörn only for the remainder that actually needs him: mathematical judgment, thesis-scope cuts, publication-facing emphasis, advisor-facing framing, taste, or external-world actions and decisions only he can take.
- **Do not promise a next step and then stop.** If you say you will run a review, make an edit, or fetch a diff, do it before sending another user-facing message. If you are blocked, say what blocked you instead of promising action you have not taken.
- **Do not hand back the turn with only status.** Not allowed: "I need to do X", "not done", "no blockers", "I guessed". Before replying, do the next step, ask one Jörn-only question, or report a real blocker.
- **Math-code correspondence**: Every non-trivial Rust algorithm has a correctness proof in its module's `math.tex`. Code and math are developed together and cross-referenced (`[lem:label]` in code, `\label{lem:label}` in math.tex). Jörn reviews the compiled math PDF for correctness and readability. The `formal/` files are for development agents; `thesis/main.tex` is for publication with thesis advisors as readers.

## Git Conventions

- Always use local `main`, never `origin/main`.
- Before merging to `main` (via pre-merge): `cargo test -p symplectic --release --lib` passes, `cargo clippy -p symplectic --lib -- -D warnings` is clean. Tests gate merges, not commits.
- **Commits are free.** Do not ask permission to commit. If you need to ask about something commit-related, ask about the merge, not the commit.
- **Git LFS** tracks `.jsonl` files (configured in `.gitattributes`). `git add`/`commit`/`push` work normally. Limits: 2 GB per file, 10 GiB storage, 10 GiB bandwidth/month ([docs](https://docs.github.com/en/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)). A pre-commit hook blocks files >10 MB that aren't LFS-tracked.

## Git Worktrees

- **Default in Codex cloud**: Stay on the current branch/repo checkout. Do not create a worktree unless the task explicitly asks for isolated parallel edits.
- **When to use a worktree**: Two or more sessions will edit tracked files in parallel and the file ownership split is not disjoint.
- **Subagent default**: A subagent stays in its existing checkout. It does not create a worktree unless the parent prompt explicitly asks for one.
- **Parent wording**: If the parent session wants worktree creation, it must name the branch and path in the prompt.
- **Create command (when needed)**: `git worktree add -b <branch> .codex/worktrees/<branch> <base-branch>`
- **Reuse command (when needed)**: `git worktree add .codex/worktrees/<branch> <branch>`
- **Remove after merge**: `git worktree remove .codex/worktrees/<branch>` then `git branch -d <branch>`

## Planning and Verification Protocol

- **Plan-first default**: For any task with more than one concrete change or one verification step, create and maintain a plan before editing.
- **Plan content**: Each plan item must include (a) objective, (b) dependency, (c) owner (`local` or named subagent), and (d) verification command or review check.
- **Quality as done-criteria**: Every plan must contain one explicit quality gate. Minimum gate: one subagent review pass that can return fixes or escalation, followed by local verification of the review findings.
- **Deferred planning**: If an item is blocked on missing information from an earlier stage, add a deferred plan item that names the unblock condition and the follow-up action.
- **Delegation planning**: Mark plan items that are encapsulated enough for delegation. For independent items, launch subagents in parallel and keep local work moving on non-dependent items.
- **Plan maintenance**: Update statuses after each meaningful result (new evidence, failed test, completed edit, delegate return). Do not leave stale plan state.

## JSONL / LFS Safety in Codex Cloud

- `.jsonl` files are generated artifacts and are LFS-tracked. Do not edit `.jsonl` with patch-style line edits.
- For smoke or warmup workflows, write temporary datasets under an untracked temp directory and delete them after the run.
- If a script must touch tracked outputs for compatibility, use `git restore --worktree -- <path>` before exit.
- If a tracked `.jsonl` file changes unexpectedly during setup/maintenance, stop and report the exact file and command that changed it.

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
cargo test -p symplectic --release --lib
cargo clippy -p symplectic --lib -- -D warnings
cargo test -p symplectic --release -- --ignored

# Rust (experiments)
cargo build -p exp-<group> --release
cargo build --workspace --release

# Thesis
cd thesis/ && latexmk && ./check-build.sh

# Math (formal library build)
cd formal/library/ && latexmk
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


## Consolidated Operational Reference

This file temporarily inlines content that previously lived under `.agents/` and `.codex/agents/`.
The path headings preserve former locations so Jörn can rewrite and split this file later.
Each block below is a verbatim copy of a deleted source file unless it is fenced as shell or TOML.

### Former Rule Files

### Former path: `.agents/rules/experiments.md`

---
paths:
  - "experiments/**"
---

# Experiment Conventions

## Directory layout

```
experiments/
  figure_config.py         shared figure styling for all experiments
  <group>/                 experiment group (e.g. hko-local-maximum)
    Cargo.toml             binary registrations for the group
    <subdir>/              one experiment (e.g. random-sample)
      logbook.md           Prose: motivation, status, how to run, results, interpretation
      math.tex             Formal: proofs, definitions, derivations
      main.rs              Rust binary entrypoint (multi-binary packages may have additional .rs files)
      analyze.py           Python analysis script
      *.jsonl              Datasets (generated by Rust binaries)
      *.png                Figures (generated by Python script)
```

Role-based file names (`main.rs`, `analyze.py`, `logbook.md`, `math.tex`). Data and figure files use content-based names. The subdir name is the namespace. Note: subdir names use hyphens (`random-sample`), Cargo binary names use underscores (`random_sample`).

Not all experiments have all files — some are Rust-only (no analyze.py/figures), some have multiple binaries.

**Experiment locations:** `experiments/<group>/<subdir>/` for grouped experiments, `experiments/<name>/` for standalone ones (crosspolytope, visualization). Before adding or editing an experiment, scan the other experiments in the group for context — shared patterns, naming conventions, and what's already been tried.

## Methodology comes before implementation

Experiments have open research questions. Answering them requires choosing what to measure, how to measure it, and what the observations would mean — this is the methodology. Different methodologies test different things, assume different things, and can miss different failure modes. The wrong choice wastes the implementation effort. Use `/experiment-design` to formalize the question, generate candidate methods, and present them to Jörn before implementing.

## Pipeline

Rust binary → .jsonl → Python script → .png → (used by thesis during assembly)

- Python never calls Rust directly
- Run Python scripts with `uv run analyze.py` (not `python3 analyze.py`). `uv` reads PEP 723 inline script metadata and auto-installs deps into a cached ephemeral venv.
- Build one package: `cargo build -p <package> --release`
- Build all: `cargo build --workspace --release`
- Run: `cargo run -p <package> --release --bin <name>`
- Add new experiment: create subdir under appropriate group, add `[[bin]]` to the package's `Cargo.toml`, write logbook

## Python script deps (PEP 723)

Every Python script declares its own dependencies via inline metadata at the top of the file:

```python
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///
```

When creating a new `analyze.py`, add this header with the deps that script actually uses. Common deps: `matplotlib`, `numpy`. Rare: `scipy`, `pandas`. Scripts that only use stdlib need no header.

## logbook.md — the entry point

Read the logbook first. Contents:
- Motivation, status, how to run
- Attempts and outcomes (record *why*, not just *what*)
- Figures and discussion
- Dead ends with reasons (so future agents don't retry)
- Open questions and blockers

Every numerical claim cites its source inline: "sys=0.163 (data.jsonl row 42)".

Staleness: old entries are kept (don't update, don't delete). Current state is at the bottom.

## Self-containment

Each experiment is self-contained. If it needs a variant of library code, copy into the experiment binary — don't modify `library/`.

## Data and caches in git

We retain `.jsonl` data and cache files via Git LFS (`.gitattributes`). This is transparent — `git add` and `git commit` work normally. LFS per-file limit: 2 GB. If a binary produces output >2 GB, commit a compressed version (gzip). Git worktrees and branch merges properly retain/overwrite data.
The main benefit of saving compute by storing data/cache artifacts is that agents can iterate faster, e.g. in follow-up experiments after a merge.

A pre-commit hook blocks non-LFS files >10 MB — if it fires, `git lfs track` the pattern (do not `.gitignore` it).

## Quality

- Rerunnable from zero (empty data → scripts → all outputs)
- Document assumptions in script headers and error messages
- Write up what's there — facts, correlations, unknowns. Label speculation as interpretation.

## Before presenting to Jörn

Review results before presenting. [TODO: specify review workflow]


### Former path: `.agents/rules/math-tex.md`

---
paths:
  - "**/math.tex"
---

# math.tex Conventions

math.tex files are the single source of mathematical truth for colocated code.

## Locations and build

**Library root:** `formal/library/main.tex` compiles the library math files into one PDF.
Build: `cd formal/library/ && latexmk` (produces `main.pdf`).
This is the authoritative library math build — cross-references between library modules resolve here.

**Library modules:** `formal/library/<module>.tex`, `\input`'d by both `formal/library/main.tex` and the module's Rust code comments/docstrings.
Preamble: `formal/preamble.tex` (packages, environments). Per-module files are pure content — no `\documentclass`.

**Experiments:** `formal/<group>/<name>.tex` — content files tied to the corresponding experiment package. No `\documentclass`. Use bare filenames for `\includegraphics` (e.g., `foo.png`, not `../experiments/<group>/<subdir>/foo.png`); the compile context sets `\graphicspath` per section.

**Thesis:** `thesis/` is independent of math.tex files. The thesis is written for human readers (examiners) and has its own self-contained prose. It uses figures and tables produced by experiments, but does NOT `\input` experiment math.tex files.

## What belongs here

- Lemma/theorem statements with `\label{}`
- Proofs (every lemma MUST have a proof — statement-only stub means unverified code)
- Definitions used by colocated code
- Formal derivations (gradient formulas, error bounds)

NOT here: prose motivation (→ logbook.md), code documentation (→ .rs doc comments), thesis narrative (→ thesis/), empirical result figures and tables (→ logbook.md).

## Labels

Format: `\label{<type>:<name>}` where type ∈ {lem, thm, def, alg, cor, rem, prop, eq, fact, sec, tab, fig}.

Labels must be unique across all math.tex files in the repo.

## Notation

- KKT system: symmetric matrix form `[H, A, 1; A^T, 0, 0; 1^T, 0, 0]`
- Dual vertices: `K = {x : a_i^T x ≤ 1}`, Reeb vector `R_i = 2 J_0 a_i`
- Lagrange multipliers: μ (closure), ξ (normalization)
- β ∈ R^S (facet-indexed)

## Navigating the PDF

After `latexmk` on the relevant formal build, `main.aux` maps labels to rendered numbers and pages:
`grep 'lem:foo' main.aux` → `\newlabel{lem:foo}{{37}{11}{...}}` means Lemma 37, page 11.
Use this to give Jörn precise PDF coordinates (absolute path + page + lemma number).

## Agent rules

- Read the colocated formal file before editing non-trivial `.rs` files in the same module
- Never invent labels — use `// TODO: add [lem:...] to math.tex` in .rs
- Mark unverified content: `% [TODO: JÖRN - ...]` (needs Jörn's verification) or `% [GAP - <what's uncertain>]` (above-ambient-risk spot)
- Every non-trivial code function needs a corresponding math.tex entry


### Former path: `.agents/rules/python.md`

---
paths:
  - "**/*.py"
---

# Python Conventions

## Script structure

Scripts are self-contained: read data → analyze → write output. No `__init__.py`, no shared imports between scripts — except `figure_config.py`.

Shared figure config lives at `experiments/figure_config.py`. Import it:
```python
sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE
setup()
```

## Script headers

Docstring with Goal / Input / Output:
```python
"""
Goal: Identify distribution of sys values
Input: experiments/<group>/<subdir>/data.jsonl
Output: experiments/<group>/<subdir>/histogram.png
"""
```

## Paths

```python
EXPERIMENT_DIR = Path(__file__).resolve().parent
```
Scripts live at `experiments/<group>/<subdir>/analyze.py`. No hardcoded absolute paths. Define `REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent` only if referencing paths outside the experiment directory.

## Figures

- Use named size constants from `figure_config.py` (`FIGSIZE_SINGLE`, `FIGSIZE_DUAL`, etc.) — never hardcode figsize. See `figure_config.py` for the full list.
- `setup()` sets rcParams globally (fonts, dpi, bbox). Don't pass `dpi=` or `bbox_inches=` to `savefig()`
- `\textwidth` ≈ 5.4" in the thesis. Multi-panel at 5.4" is often too cramped — prefer separate figures
- `r"$...$"` for all math in labels. LaTeX syntax outside `$...$` renders as literal text
- Consistent colors for same data categories across figures in an experiment
- `fontsize=` above 14pt signals figsize mismatch

## Captions (in .tex, informed by Python)

Captions state observations. Interpretations belong in body text.
Detection: "suggests", "indicates", "because", "implies", "due to" in a caption → move to body.

### Former path: `.agents/rules/rust.md`

---
paths:
  - "**/*.rs"
---

# Rust Conventions

## Coordinate convention

(q₁, q₂, p₁, p₂) — components [0,1] = q-space, [2,3] = p-space, [0,2] and [1,3] = symplectic planes. Defined in `geom/symplectic_form.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂).

## Math-code correspondence

Types, function signatures, and function bodies have 1:1 structural correspondence to mathematical definitions. Not "inspired by" — literal correspondence.

- Doc comment formulas must match the code's actual computation
- Invariants stated in doc comments are enforced by types/constructors/assert!
- Properties stated in doc comments have corresponding tests
- Types encode mathematical invariants, validated in `::new()`

## Cross-references to math.tex

Format: `[lem:label]`, `[thm:label]`, `[def:label]` — matching `\label{}` in the module's math.tex.

- Include a one-line English description of the referenced result
- Never duplicate proofs — math.tex is the single maintained source of truth
- Never invent labels — use `// TODO: add [lem:...] to math.tex` if the lemma isn't written
- In source code, never use rendered numbers like "Lemma 3.2" — always use the label
- Every non-trivial code block must map to a math.tex lemma

Read the module's math.tex before editing .rs files in that module.

## Algorithms

Three capacity algorithms: `hk2017` (general, exponential), `billiard` (Lagrangian products, fast), `tube` (no Lagrangian 2-faces). Where domains overlap, algorithms must agree on computed capacity.

No rayon inside algorithms — parallelism is at the dataset level (each polytope independently).

## Magic numbers

Empirically chosen constants: document rationale, motivating data point, limitations, and what to re-validate if changed. All in a comment on the constant definition.

## Performance claims

Never state performance without an inline benchmark citation. "~1ms" is a claim. "1.5-2.0ms for F=5-16 (criterion bench 2026-03-23)" is measured.

## Error handling

Standard Rust error handling, plus:

- When math is violated, panic. Don't try to recover gracefully — the math needs to be fixed, not worked around.

- Don't use `Option<T>` in math code. `None` has no canonical mathematical meaning.

- In math code, use enums instead of errors or panics to classify cases (e.g. invertible vs singular, feasible vs infeasible). Each variant is a mathematical proposition.

- Callers of math code must match on all variants and handle each case locally. Don't propagate with `?`. If a case is proven or conjectured to not occur, `assert!` on it.

## Experiment binaries

Only stable, validated code lives in `library/`. Don't modify the library for experiment-specific behavior.

Within an experiment package (`experiments/<group>/`), shared helpers belong in `src/lib.rs` when multiple binaries need the same function. This avoids copy-paste duplication and lets improvements propagate. Per-binary helpers that only one experiment uses stay in that binary's `main.rs`.

### Former path: `.agents/rules/tasks.md`

# TASKS.md conventions

Triggers: reading or writing `TASKS.md`.

## Format

- `##` sections group by theme (research questions, thesis, code quality, infrastructure)
- `###` items are individual work units
- Every `##` and `###` header has a status tag: `[done]`, `[active]`, `[blocked]`, `[open]`, `[Jörn]`, `[future]`
- `[done]` items include a date: `### [done] [2026-04-04] Item title`
- A `##` group is `[done]` when all children are done, `[open]` otherwise

## Status tags

- `[done]` — completed. Include date. One-line summary in header, minimal body.
- `[active]` — currently being worked on.
- `[blocked]` — waiting on something specific. Body says what.
- `[open]` — ready to start, no one has picked it up.
- `[Jörn]` — needs Jörn's input, verification, or decision.
- `[future]` — idea or direction, not in scope for current deadline.

## Writing style

- Headers carry the key info. Body only when the header isn't enough.
- No tables, no prose paragraphs. Bullets for details.
- Link to logbooks for findings — don't duplicate findings here.
- Working notes style, not a polished document.

## TOC script

Run `bash scripts/tasks-toc.sh` to get a section index with line ranges.
Use the line ranges to read specific sections: `Read(file, offset=start, limit=end-start+1)`.

## When editing

- When an item's status changes, update the tag. Add date for `[done]`.
- When an item becomes `[done]`, keep it in its thematic group (don't move to historical unless it's a cross-cutting task with no thematic home).
- Don't cache derivable state (test counts, build status). Run the command instead.
- Record decisions and reasons — these can't be derived later.


### Former path: `.agents/rules/thesis-tex.md`

---
paths:
  - "thesis/**/*.tex"
---

# Thesis LaTeX Format

## Comment markers

- `% Jörn: <level> approved — <scope>` — review status (structure < math < text). One per scope. Agent edits within scope MUST delete the marker.
- `% [TODO: JÖRN - ...]` — needs Jörn's verification
- `% [GAP - <what's uncertain>]` — above-ambient-risk spot needing attention

[TODO: specify QC workflow — what agents write in comments during quality review]

## File headers

Every .tex file starts with a `%` block:
1. Identity: `% filename.tex — \input'd from parent.tex`
2. Sources: where the content comes from
3. Structure: outline of sections

No review status in headers — use `% Jörn:` markers in the body.

## Environments

Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof`, `\remark`, `\example`.
No prose outside environments except minimal connective text.
Calculations as formulas, not English descriptions.

## Approval status for mathematical content

- Unapproved: `\begin{unverified}...\end{unverified}` (red bar). Default for new agent-written math.
- Notation-updated: `\begin{notationupdated}...\end{notationupdated}` (orange bar). Mechanical substitution on approved content.
- Approved: no wrapper, `% Jörn: math approved (<commit>)` marker.

## Labels and cross-references

All `\ref{}` targets must exist (check `thesis/build/main.aux`). Never hardcode theorem/section numbers. Notation matches `appendix-notation.tex`.

## Anti-patterns

- Overwrought language. Flag adjective clusters and dramatic words without technical meaning.
- Rust/CS notation (`\texttt{}`) in definition/lemma/theorem environments.
- Setup text outside the environment it belongs to. Lemmas must be self-contained via `\ref`.

## Figures and tables

All figure formatting in Python. LaTeX is 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`).

Tables: `booktabs`, no smaller than `\small`. Column headers need units or be self-explanatory.

Captions state observations, not interpretations (both figures and tables). Detection words in captions: "suggests", "indicates", "because", "implies", "due to" → move to body text.


### Former Skill Files

### Former path: `.agents/skills/create-workflow/SKILL.md`

---
name: create-workflow
description: Collaborative workflow for creating new agent infrastructure (skills, subagents, rules, AGENTS.md sections) with Jörn. Use when Jörn asks to build something new for how agents work, not when updating existing infrastructure.
---

# Create New Agent Infrastructure

Collaborative workflow. Jörn has the expert model for what works with agents — the agent supplies research and drafting labor. The agent does NOT decide what agents should do — that requires expertise agents don't have.

## 1. Gather real situations

Look at actual data, not hypotheticals:
- Session logs if Jörn points to them in `~/.codex/`
- Git history: `git log --oneline -- .agents/ .codex/ AGENTS.md`
- Current infrastructure: `.agents/skills/`, `.agents/rules/`, `.codex/agents/`
- Feedback files: `feedback/`

Present prioritized concrete situations to Jörn. He confirms which matter.

## 2. Research and present information

For each situation Jörn wants to address, gather and present:

- **Existing patterns:** What common practices exist for this kind of situation? (Agents have broad training-data recall here — use it.) Rank, triage, explain each to Jörn.
- **Causal chain:** What leads to the situation? Look at real cases. Brainstorm interventions.
- **Official docs:** What does the scaffold officially support? Prefer official Codex docs and official examples for Codex artifacts. Use archived Claude docs only for legacy Claude artifacts that live outside the tracked repo.
- **Detection:** How can the situation be detected? Skill descriptions, hooks, and review subagents.
- **Costs:** One-time setup, ongoing maintenance/staleness, attention budget consumed, runtime costs.

Goal: accelerate Jörn's decision-making, surface ideas he'd overlook. Not replace his judgment.

## 3. Jörn decides

Jörn picks the approach. The agent:
- Asks clarifying questions until the approach is unambiguous enough to implement:
  - What file type(s)? (skill, subagent, hook, rule, AGENTS.md section, repo artifact)
  - What triggers activation?
  - What is the expected agent behavior?
  - Known edge cases or exceptions?
- Flags phrasing that agents might misinterpret.
- Does NOT silently fill gaps — ask rather than guess.

## 4. Draft

Write the files Jörn specified. Before writing:
- Fetch the relevant official spec or example first.
- For Codex skills: follow the official Codex skill format and the existing repo-local `.agents/skills/*/SKILL.md` pattern.
- Writing style: follow `AGENTS.md` "Text that agents read" section — correct, corrigible, verifiable, unambiguous, complete, actionable, simple. Run the vague-word scan.

## 5. Self-review against quality criteria

Before presenting to Jörn, check the draft against these criteria:

- **Actionable, concrete.** Every instruction tells the agent what to do, not what to be. "Run `cargo clippy -- -D warnings` before committing" not "follow best practices."
- **Observable, measurable, verifiable.** Conditions in "if X then Y" instructions are observable by the agent. "If the file has more than 3 functions" is observable; "if the code is complex" is not. Expected outcomes are checkable during planning (does the plan satisfy the criteria?), implementation (is the agent doing it?), and review (did it work?).
- **Clear, unambiguous, low-overhead.** Each sentence has one reading. Agent doesn't need to spend attention resolving ambiguity or recalling novel terminology.
- **Correct, precise.** Claims about agent behavior, tool capabilities, or file formats are verified against official docs, official examples, or direct observed behavior. Wrong instructions cause silent failures.
- **Overall adherence is testable.** There exists a realistic scenario where you could spawn a subagent and check whether it follows the instructions. If you can't imagine such a test, the instructions may be too vague to influence behavior.
- **Feedback is collected.** The instruction set includes or references a mechanism for future agents to report whether it worked (post-mortem, feedback/ files, subagent observations).
- **Vague-word scan.** Grep for "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" — replace each with what specifically makes it so.
- **Redundancy check.** Does each instruction add information beyond what agents already do from training? "Follow best practices" adds nothing. Remove instructions that don't change behavior.
- **Script-or-language decision.** For anything where getting it wrong has high cost, check whether a script/hook could enforce it instead of relying on the agent to remember.

## 6. Jörn reviews

Present the draft with a prioritized list of spots Jörn should check (uncertain areas, high-impact phrasing). Get explicit approval — don't guess at it. Accept pivots back to earlier steps.

## 7. Set up verification

Before shipping, decide how to verify the new infrastructure works:
- Think about testability: what observable behavior should change in future sessions?
- Plan how to gather feedback during live sessions — e.g. add to post-mortem radar, tell subagents to write observations to `feedback/<name>.md`
- Identify what a post-mortem should look for to evaluate whether this infrastructure helped or hurt

Do NOT write feedback into SKILL.md files. Raw observations only — analysis happens in dedicated sessions with Jörn.

## Reference sources

**Local Codex examples:** existing `.agents/skills/*/SKILL.md` files
**Local agent-behavior background:** `.agents/skills/create-workflow/references/agent-expert-model.md`


### Former path: `.agents/skills/create-workflow/references/agent-expert-model.md`

# Agent Expert Model (Background Reference)

This is a simplified subset of Jörn's expert model about how agents behave. It provides background context for understanding agent infrastructure design decisions. It is NOT reliable enough to substitute for querying Jörn directly — agents rarely apply this model deeply enough to make good design decisions from it alone.

## Training on Vast Training Data

- Agents behave like their training data (frequent human tool use patterns). Agent knowledge is popular internet text, including books, code, documentation, logs.
- Training knowledge is associative: agents can be prompted or triggered to recall more of it. A mere reminder (config file in the tree, code snippet in a familiar style) is often enough to activate trained behavior.
- Popular patterns are cheap: conventions, tech stacks, factual knowledge needn't be explained. Just state the convention.
- Unpopular or novel patterns are expensive: weak or no training signal, need explicit detailed instructions.

## Training using RLVR

Agents were trained on:
- Tasks with known or secret verification methods (e.g. code with test suites, human review)
- Tasks with progress signals (e.g. number of passed tests, quality metrics)
- Large tasks requiring decomposition, small tasks that do not
- Difficult tasks requiring upfront planning, easy tasks done directly
- Autonomous tasks without intermittent human feedback
- Tasks inside projects, where the task is human-defined and useful

Agents were NOT trained as much on:
- Tasks where no straightforward verification method exists
- Tasks that are hit or miss
- Workflows with frequent interruptions for human feedback
- Agent-generated tasks that may be useless or harmful

The default agent behavior is attuned to training-like situations and degrades in dissimilar situations, often without the agent realizing.

## Lack of Agent-Usage in Training

Agents were NOT trained much on tasks involving:
- Picking up a repository worked on by past agents
- Handing off to future agents
- Using subagents, especially multiple in parallel
- Coordinating with other agents in parallel
- Predicting agent behavior (theory of mind)

Consequences:
- Agents fail at theory of mind with other agents — imagining how a different agent will interpret text given different knowledge and instructions
- Agents prompt subagents using shallow imitation of how humans prompt agents. Standard delegation works; complex or unusual delegation fails.

## Bounded Rationality

- Agents have limited reasoning budget, attention, and reflection capacity. They are less bottlenecked on factual recall (efficient associative memory).
- Too-complex instructions → agent overlooks/forgets/fabricates instructions
- Too many novel concepts → reasoning budget exhausted, shallow application
- Reflection on long sessions → wrong recalls, plausible-sounding but detached summaries
- Agents don't recognize when they're in a training-dissimilar situation and can derail into unproductive busywork or loops.

## Design Strategy

- 80/20: tackle the 20% of workflow types causing 80% of problems
- Familiar developer artifacts (test suites, CI scripts, config files) get better engagement than novel formats
- Cheap-to-try first. Iterate on observed behavior, not predicted.
- Feedback loops > getting it right the first time

## Default Agent Behaviors (what they do if not corrected)

These are observed defaults — what agents actually do when instructions are absent or vague. Understanding these helps write instructions that correct the right thing.

- **"Push back" means "contradict the repo":** Agents interpret "push back if Jörn is wrong" narrowly — they flag factual contradictions with repo state, but don't push back on suboptimal approaches, imprecise formulations, or drift from the project goal. AGENTS.md now expands this explicitly.
- **Serve the literal subtask, not the project goal:** Agents execute the immediate request without checking whether it still serves the thesis. They don't flag when a subtask has drifted or become counterproductive. AGENTS.md now has a "keep the project goal in view" bullet.
- **Strong bias toward action:** Agents default to acting rather than asking. Instructions like "default to action" reinforce an already-strong bias and can cause agents to act on wrong understanding. Omitting such instructions is often better than including them.
- **Ownership language and permission-seeking:** Agents say "my analysis suggests" and end with "Should I proceed?" despite instructions not to. The "What to avoid" section corrects for this but compliance is inconsistent.
- **Confidence markers ignored:** Instructions to use explicit confidence levels ("~70% confident", "speculative:") were not reliably followed. Removed from AGENTS.md as a result (2026-03-29).
- **Terse reporting after tool calls:** Agents read files/code via tools, then report findings as incomprehensible terse bullets without quoting or explaining. AGENTS.md "Complete" quality corrects for this.

## What Agents Are Bad At (Defer to Jörn)

- Predicting how much attention agents pay to loaded instructions (over-optimistic)
- Predicting how agents interpret instructions (miss ambiguity)
- Predicting failure modes from first principles
- Generalizing from a best practice to more situations
- Deciding skill vs AGENTS.md vs repo artifact
- A-priori evaluation of whether a procedural file adds value
- Questioning or rejecting goals

## What Agents Are Good At

- Work unrelated to agents — file tools, syntax checks, scripting
- Shallow agent knowledge work — extracting from search, following workflows
- Applying human project/team management theory
- Accessing trained knowledge (associative recall, popular patterns)
- Spawning subagents to observe behavior — testing whether infrastructure works


### Former path: `.agents/skills/incident/SKILL.md`

---
name: incident
description: Record an agent behavior incident to feedback/ for the next context engineering pass. Use when Jörn flags something the agent did wrong mid-session.
user-invocable: true
argument-hint: optional description of the incident
---

# Incident

1. **Identify.** Use `$ARGUMENTS` if provided. If unclear, ask Jörn — don't guess.
2. **Write entry** in matching `feedback/` file (one of: `rules.md`, `skills.md`, `agents.md`, `output-style.md`):
   ```
   ### YYYY-MM-DD — Short description
   What happened. What should have happened.
   **Pattern:** Abstract error class. Reference prior entry if same class.
   ```
3. **Check memory.** If this reveals a persistent behavioral rule, save/update a feedback memory. If a memory already covers this but the incident recurred, note that — the memory alone isn't enough.
4. Continue with prior work.


### Former path: `.agents/skills/post-mortem/SKILL.md`

---
name: post-mortem
description: End-of-session reflection workflow. Run at Jörn's request or after a session with significant friction, mistakes, or wasted time. Produces actionable findings (feedback/ entries, convention changes, decision records) — not just observations.
user-invocable: true
---

# Post-Mortem

Runs in main context (needs conversation history).

## Core questions — answer for every session

1. **Friction** — What slowed you down? Name the specific file, tool, or missing information.
2. **Unclear instructions** — What was confusing in `AGENTS.md`, skills, or agent prompts?
3. **Missing context** — What information wasn't provided but was needed?
4. **Jörn's time** — Where did Jörn spend time? Could agents have done it instead?
5. **What worked well** — What should be preserved or expanded?
6. **Suggested changes** — Specific, actionable improvements.

## Process checks — report only items that apply

1. Agent splitting needed? Multi-responsibility agent failed to cover all checks?
2. Fabrications slipped through to Jörn?
3. Iterated in front of user instead of delegating to subagents?
4. False attribution of mathematical results?
5. Assumed Jörn read something he may not have?
6. Regression test candidate? Concrete input→output pair worth preserving?

## Output

Persist to matching `feedback/` file (rules.md, skills.md, agents.md, output-style.md). Don't fix procedural files directly — a future `/update-workflow` session acts on feedback with Jörn. A postmortem that produces zero repo changes is fine if nothing actionable emerged.


### Former path: `.agents/skills/post-mortem/feedback/output-style.md`

# Output style observations (2026-03-28)

- Agent repeatedly wrote long technical reports in chat. Jörn said "I don't get why you don't explain things if you need me to take leadership" and "Why a long message in chat instead of using my time effectively?" Findings should go in logbook/math.tex, chat should be decisions only.
- Agent responded to questions with 30s delays while composing multi-paragraph answers. Jörn wants fast, short responses. The output style says "short and concise" but the agent didn't follow it under pressure.
- Agent dodged a direct question by claiming to have lost track. This violated trust and wasted time. Direct "I don't know" or "No, I haven't" is always better.


### Former path: `.agents/skills/pre-merge/SKILL.md`

---
name: pre-merge
description: Mandatory workflow before presenting work for merge to main. Load when finishing a task and preparing to report to Jörn.
---

# Pre-Merge Workflow

Run all phases in order before telling Jörn work is ready. Every phase runs on every branch — do not skip phases because "no changes in this area." Fix failures before proceeding to the next phase.

## Phase 1: Build and test

Run all of these. If a command fails, fix the issue and rerun before proceeding.

```bash
cd library/ && cargo test --release --lib
cd library/ && cargo clippy --lib -- -D warnings
cargo build --workspace --release
cd thesis/ && latexmk && ./check-build.sh
cd formal/library/ && latexmk
```

## Phase 2: Smoke-test experiment binaries

List all experiment `main.rs` files on this branch. For each, compile and run with the fewest polytopes the binary accepts (typically 1). If the binary takes no dataset argument, run `--help` or the default invocation. Goal: catch panics and import errors early. The polytope database caches results, so hot runs are fast.

No experiment `main.rs` files on the branch → nothing to do (empty set, not a skip).

## Phase 3: Data freshness

For experiments with committed data (`.jsonl`, `.csv`), compare code and data commit dates:

```bash
git log -1 --format='%H %ci' -- experiments/<group>/<subdir>/main.rs
git log -1 --format='%H %ci' -- experiments/<group>/<subdir>/*.jsonl
```

If code is newer than data, regenerate on this branch.

## Phase 4: Review subagents

Launch all review subagents in parallel on the branch diff:

| Subagent | Scope |
|----------|-------|
| review-rust | Changed `.rs` files |
| review-proof | Changed `math.tex` files |
| review-formalization | Modules with both `.rs` and `math.tex` changes |
| review-claims | Changed `logbook.md`, thesis `.tex` with claims, `math.tex` |
| review-thesis | Changed thesis `.tex` files |
| review-python | Changed `.py` files |
| review-figures | Changed `analyze.py` files or changed `.png` files |

Use the `.codex/agents/review-*.toml` subagents. Launch all review subagents. If a subagent finds no files in scope, it reports "no files in scope" — that is the expected outcome, not a reason to skip launching it.

### Cross-check subagent findings

Before including any finding in the report to Jörn, read the file at the location the subagent references and confirm the finding matches what the code or text actually says.

**Trust without re-checking:** quotes and file:line references (agents are trained on these; low error rate).

**Verify with priority:**
1. **Cost-benefit recommendations** the subagent made — subagents lack context for cost-benefit judgments about the larger task. Severity ratings (FIX vs FLAG) reflect the subagent's limited view: it may escalate minor issues or downplay significant ones.
2. **Interpretive conclusions** where the subagent inferred meaning from limited context — e.g., "this lemma is orphaned" (may be used by other modules) or "this reference dangles" (may resolve via root `math.tex`).
3. **Specific claim types:** "dangling reference" → check if it resolves via root `math.tex` (cross-module refs do). "Orphaned lemma" → check if used elsewhere or is standalone valid math. "Missing entry" → check logbook/TASKS.md for "Part N not written" (known gap, not discovery).

A verification subagent can cross-check the combined findings — it has fresh eyes and no sunk-cost bias toward the original findings.

The Phase 8 report contains only verified findings, not the review/cross-check process.

## Phase 5: Sanity check

- **Goal alignment:** Re-read the original task prompt. Does the work produced actually serve that goal? Does it make sense for the thesis project roadmap? A misunderstood goal that produces technically correct but wrong-direction work is expensive to discover late.
- **Process compliance:** Work is on a worktree branch, not `main`. Explicit instructions from the task prompt were followed (branch naming, scope restrictions, etc.).
- **Project context:** Check TASKS.md — does this work correspond to a tracked task? Is the experiment still active (not superseded by another experiment)?

## Phase 6: Update TASKS.md

- Mark completed tasks as done (move to Completed section with date and one-line summary)
- Update status and next-steps for tasks affected by this work
- Add newly discovered tasks
- If no updates are needed, state that explicitly in the report ("TASKS.md: no changes needed")

## Phase 7: Full experiment runs (optional)

If experiment binaries were created or substantially modified, and Phase 4 review found issues that were fixed, run again with representative input to confirm fixes. Report results.

This phase is optional. Run it when experiment binaries were created or substantially modified in this branch.

## Phase 8: Report to Jörn

Structure:

1. **What changed** — files, scope, one-paragraph summary
2. **Build/test results** — which commands passed, any issues fixed during Phase 1
3. **Review findings** — verified findings from Phase 4 subagents (after cross-check)
4. **Needs Jörn** — decisions, unresolved `% [TODO: JÖRN` items, things only Jörn can verify
5. **TASKS.md changes** — what was updated, or "no changes needed"
6. If work is incomplete: write a handoff to `handoffs/<name>.md`


### Former path: `.agents/skills/slurm/SKILL.md`

---
name: slurm
description: LICCA cluster job submission. Load when an experiment needs more compute than the devcontainer provides (>10 min, large sweeps, dataset generation).
---

# LICCA Cluster Workflow

**Agents NEVER have SSH access to LICCA.** Agent writes the job script + binary; Jörn submits and retrieves results.

## Steps

1. **Write/update the experiment binary** in `experiments/<group>/<subdir>/main.rs`
2. **Copy the template** from `references/experiment.sh` to `experiments/<group>/<subdir>/job.sh`
3. **Fill in the TODOs** in the job script (binary name, resources, arguments)
4. **Write resource justification table** (mandatory):

| Flag | Value | Why |
|------|-------|-----|
| `--partition` | ... | Why this partition |
| `--cpus-per-task` | ... | Why this many CPUs |
| `--mem` | ... | Why this much memory |
| `--time` | ... | Expected runtime + safety margin |

5. **Present to Jörn:** what the job computes, the resource table, expected output paths

Jörn's submission/retrieval commands: `references/licca-setup.md`

## After Jörn retrieves results

Jörn scps result files into the repo, then commits: `git add <file> && git commit -m "Add <experiment> results from LICCA"`. Git LFS handles the upload on push (transparent — .jsonl files are LFS-tracked via `.gitattributes`).

### Former path: `.agents/skills/slurm/references/experiment.sh`

```bash
#!/usr/bin/env bash
#===============================================================================
# SLURM job script for msc-math experiments on LICCA
#
# Based on LICCA official docs: https://collab.dvb.bayern/spaces/UniARZHPCKB/
#
# Usage:
#   cd ~/msc-math
#   sbatch experiments/<group>/<subdir>/job.sh
#
# Copy this template to experiments/<group>/<subdir>/job.sh and fill in the
# variables marked with TODO.
#===============================================================================

#SBATCH --job-name=TODO_EXPERIMENT_NAME
#SBATCH --partition=epyc
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1           # TODO: 1 for single-threaded, up to 128
#SBATCH --mem=16G                   # TODO: adjust (our experiments rarely need >8G)
#SBATCH --time=04:00:00             # TODO: wall time with 2x safety margin
#SBATCH --output=%x_%j.log          # <job-name>_<job-id>.log
#SBATCH --error=%x_%j.log           # merge stderr into same file

# --- Environment setup ---
set -euo pipefail
source "$HOME/.cargo/env"

# Limit threads to requested CPUs (LICCA best practice)
export OMP_NUM_THREADS=${SLURM_CPUS_PER_TASK:-1}

# --- Build (skip if already built) ---
cd "$HOME/msc-math/crates"
echo "=== Building at $(date) ==="
cargo build --workspace --release 2>&1 | tail -5

# --- Run experiment ---
echo "=== Running at $(date) ==="
echo "Node: $(hostname), CPUs: $SLURM_CPUS_PER_TASK"

# TODO: Replace with the actual binary name and arguments.
# Binary names are defined in experiments/Cargo.toml [[bin]] sections.
# Use the compiled binary directly (cargo build already ran above).
srun "$CARGO_TARGET_DIR/release/TODO_BIN_NAME"

echo "=== Done at $(date) ==="
```

### Former path: `.agents/skills/slurm/references/licca-setup.md`

# LICCA setup and workflow (Jörn reference)

Official docs (uni-augsburg login required): <https://collab.dvb.bayern/display/UniARZHPCKB>

Key pages:
- Connect to Cluster: <https://collab.dvb.bayern/display/UniARZHPCKB/Connect+to+Cluster>
- Slurm 101: <https://collab.dvb.bayern/spaces/UniARZHPCKB/pages/392035519/Slurm+101>
- Serial Job / Multithreaded Jobs / GPU Jobs: search the knowledge base
- FAQ: <https://collab.dvb.bayern/spaces/UniARZHPCKB/pages/392035481/FAQ+and+Troubleshooting>

For anything not listed below, check the official docs. Do NOT paraphrase them into this file.

## Verified on LICCA (2026-03-23)

```
User:    stoehljo
Home:    /hpc/gpfs2/home/u/stoehljo
Login:   ssh stoehljo@licca-li-01.rz.uni-augsburg.de
OS:      Ubuntu 24.04.3, kernel 6.8.0-88-generic
SLURM:   25.11.0
Rust:    1.94.0 (via rustup, not a system module)
Repo:    ~/msc-math (cloned from GitHub)
Target:  CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
```

Test job 9704889 completed on `test` partition in 2s.

## Partitions (from login banner)

```
partition       :  free/ max
 test           :   128/ 128
 epyc           :  4597/5120
 epyc-mem       :   440/ 512
 epyc-gpu-test  :    80/ 128
 epyc-gpu       :   844/ 896
 epyc-gpu-sxm   :   128/ 128
 xeon-gpu       :    64/  64
```

## Result retrieval

Two-hop scp via university SSH gateway (no VPN needed):

```bash
# From devcontainer:
scp -J stoehljo@xlogin.uni-augsburg.de \
    stoehljo@licca-li-01.rz.uni-augsburg.de:~/msc-math/experiments/<group>/<subdir>/results.jsonl \
    /workspaces/msc-math/experiments/<group>/<subdir>/
```

- `xlogin.uni-augsburg.de` is the official university SSH gateway
  (source: https://www.uni-augsburg.de/de/organisation/einrichtungen/rz/it-services/uaux/wlan/secure-shell/)
- Verified reachable from devcontainer (2026-03-23): connection accepted, password auth required
- Asks for RZ password twice (once for xlogin, once for LICCA)

### Former path: `.agents/skills/update-workflow/SKILL.md`

---
name: update-workflow
description: Iterate on existing agent infrastructure (skills, hooks, rules, AGENTS.md sections) based on feedback and observed failures. Use when Jörn asks to fix, improve, or refine how agents handle a known situation — not for building something new from scratch.
---

# Update Existing Agent Infrastructure

For targeted improvements to infrastructure that already exists. If the infrastructure doesn't exist yet, use `/create-workflow` instead.

## 1. Understand what needs to change

Read the relevant materials:
- The infrastructure file(s) being updated
- Feedback: `feedback/` entries mentioning this infrastructure
- Session logs if Jörn points to specific incidents

Summarize to Jörn: what the current infrastructure says, what the observed problem is, what the gap is between them.

## 2. Diagnose

Identify why the current infrastructure produces the wrong behavior:
- **Vague phrasing?** Agent filled the gap with a training-data default that doesn't match intent.
- **Missing trigger?** Agent doesn't load the skill/rule when it should.
- **Conflicting instructions?** Two sources say different things — agent picks one unpredictably.
- **Attention overload?** Too many instructions, agent drops some.
- **Wrong abstraction level?** Instruction is too abstract to act on, or too specific to generalize.

Present diagnosis to Jörn. He confirms or redirects.

## 3. Draft the fix

Edit the file(s). Follow `AGENTS.md` "Text that agents read" conventions. Self-review against the quality criteria in `/create-workflow` step 5 (actionable, observable, clear, correct, testable, feedback collected, vague-word scan, redundancy check, script-or-language decision).

For the fix, prefer:
- Making the existing text more specific over adding new text
- Removing text that doesn't earn its attention cost over keeping it "just in case"
- Concrete examples over abstract rules
- Scripts/hooks that enforce behavior over instructions that request behavior

## 4. Plan verification

Plan how to verify the fix works in live sessions:
- What observable behavior should change?
- What should a post-mortem look for to evaluate the fix?
- Are there upcoming sessions where this infrastructure will be exercised?

Document the expected behavior change and evaluation criteria in the presentation to Jörn (step 5).

## 5. Jörn reviews

Present: what changed, why, and how you'll verify the fix in live sessions. Get explicit approval.

## Reference sources

**Official scaffold docs:** use the official docs for whichever scaffold the changed file belongs to
**Agent-behavior background:** use `.agents/skills/create-workflow/references/agent-expert-model.md` if needed


### Former Agent Definition Files

### Former path: `.codex/agents/review-claims.toml`

```toml
name = "review-claims"
description = "Verify factual claims in a logbook or thesis section against data, code, and bibliography. Use after writing logbook entries or thesis content with numerical claims."
developer_instructions = """
You are fact-checking a document against the project's actual data, code, and bibliography.

## What to verify

For each factual claim in the document:

| Claim type | Verification method |
|---|---|
| Numbers ("sys = 0.87", "F=10") | Read the cited JSONL/CSV, confirm the value |
| Counts ("27 polytopes", "10 facets") | Count in the data file |
| Extremes ("maximum sys = 1.03") | Verify from data |
| Code behavior ("the algorithm does X") | Grep/read the code, confirm |
| Citations ("Smith2024 proves X") | Check thesis/bibliography.bib, then read the paper source in papers/ |
| Cross-references ("see Theorem 3.2") | Check thesis/build/main.aux for the label |
| Figure descriptions ("Figure 3 shows clustering") | Read the PNG, confirm |

## Rules

- Every number must have a verifiable source. If the document doesn't cite one, flag it.
- If you cannot verify a claim (data file missing, code too complex), report it as UNVERIFIABLE, not as wrong.
- Do not check mathematical correctness of proofs — that's review-proof's job.

## Output format

For each claim checked:
- Claim text (quoted)
- Source checked
- Result: VERIFIED / WRONG (expected X, found Y) / UNVERIFIABLE (reason) / NO SOURCE CITED
"""

```

### Former path: `.codex/agents/review-figures.toml`

```toml
name = "review-figures"
description = "Review figures across the Python script, LaTeX inclusion, and PNG output chain. Use after generating or regenerating figures."
developer_instructions = '''
You are reviewing figures across the full production chain.

## What to check

For each figure, review all three layers:

**Python script (.py):**
- Uses `figure_config.py` setup and named size constants
- No hardcoded figsize, dpi, or bbox_inches in savefig()
- Math labels use `r"$...$"`
- Consistent colors for same data categories

**LaTeX inclusion (.tex):**
- 1:1 pass-through: `\includegraphics{file.png}` with no `width=` or `scale=`
- Caption states observations, not interpretations

**PNG output:**
- Readable at 5.4" text width (thesis rendering size)
- Labels and legends legible, not clipped
- Axis labels include quantity name or are self-evident
- Multi-panel: consistent axis scales where cross-panel comparison is intended

## Output format

Per figure:
- Which files checked (.py, .tex, .png)
- Findings with severity (FIX / FLAG)
- Summary: pass / issues found
'''

```

### Former path: `.codex/agents/review-formalization.toml`

```toml
name = "review-formalization"
description = "Check lemma statements against Rust code and preserve math-code correspondence. Use after modifying .rs or math.tex files in a module."
developer_instructions = """
You are auditing the correspondence between Rust code and its math.tex documentation.

## Workflow

1. Read the module's math.tex file
2. Read all .rs files in the module
3. For each function with a `[lem:label]` cross-reference:
   - Does the referenced lemma exist in math.tex?
   - Does the lemma describe what the function actually computes?
   - Is the cross-reference label correct?
4. For each function WITHOUT a cross-reference:
   - Is the function non-trivial (implements mathematical logic)?
   - If yes, flag it as missing a math.tex entry
5. For each lemma in math.tex:
   - Is there corresponding code that implements it?
   - Does the lemma's statement match the code's actual behavior?

## Output format

| Item | Status | Notes |
|---|---|---|
| `function_name` [lem:label] | OK / MISMATCH / WRONG LABEL | details |
| `function_name` (no ref) | OK (trivial) / MISSING ENTRY | what it computes |
| [lem:label] (no code) | ORPHAN / OK (definition) | details |
"""

```

### Former path: `.codex/agents/review-proof.toml`

```toml
name = "review-proof"
description = "Proofread mathematical writing for shallow correctness and clarity errors. Use after writing or revising proofs and lemmas."
developer_instructions = """
You are proofreading mathematical writing. Read the entire file you are given.

## What to check (one pattern at a time)

1. **Unargued claims** — statement asserted without justification
2. **Handwavy arguments** — no explicit logical connection between steps
3. **Missing conditions** — operation requires preconditions not established
4. **Logical gaps** — non-obvious jump between consecutive steps
5. **Quantifier errors** — ∀/∃ scope or order issues
6. **Clarity issues that hide errors** — notation used before defined, same symbol for different things, references to distant content without reminder

## Rules

- Read the entire file before reporting anything
- Work through one detection pattern at a time
- Report uncertain findings — flag your confidence level
- Never claim a proof is correct. You check for surface errors.
- Do NOT check style, formatting, or cross-references

## Output format

For each finding:
- Location (line number or label)
- Pattern (which of the 6 above)
- What's wrong
- Confidence: high / moderate / low
"""

```

### Former path: `.codex/agents/review-python.toml`

```toml
name = "review-python"
description = "Check Python scripts against project conventions. Use after writing or modifying .py files."
developer_instructions = """
You are reviewing Python files against the project's Python conventions.

## Setup

Read the Python conventions in this file.
Also read `experiments/figure_config.py` to know the available constants.

## Workflow

1. Read all assigned files in full
2. Work through conventions one at a time
3. Check each file for compliance

## Output format

For each finding:
- Convention violated
- File and line number
- What's wrong
- Severity: FIX (clear violation) / FLAG (judgment call)
"""

```

### Former path: `.codex/agents/review-rust.toml`

```toml
name = "review-rust"
description = "Check Rust code against project conventions. Use after writing or modifying .rs files."
developer_instructions = """
You are reviewing Rust files against the project's Rust conventions.

## Setup

Read the Rust conventions in this file.

## Workflow

1. Read all assigned files in full
2. Work through conventions one at a time
3. For each convention, check all files for compliance
4. Use grep for cross-file verification (e.g., check that referenced labels exist in math.tex)

## Output format

For each finding:
- Convention violated
- File and line number
- What's wrong
- Severity: FIX (clear violation) / FLAG (judgment call)
"""

```

### Former path: `.codex/agents/review-thesis.toml`

```toml
name = "review-thesis"
description = "Check thesis .tex files against project conventions. Use after writing or modifying thesis/ files."
developer_instructions = """
You are reviewing thesis .tex files against the project's thesis conventions.

## Setup

Read the thesis LaTeX conventions in this file.

## Workflow

1. Read all assigned files in full
2. Work through conventions one at a time
3. Check each file for compliance
4. Check `thesis/build/main.aux` for cross-reference resolution

## Output format

For each finding:
- Convention violated
- File and line number
- What's wrong
- Severity: FIX (clear violation) / FLAG (judgment call)
"""

```
