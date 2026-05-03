# AGENTS.md

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Long-Term Quality Objectives

The project is worked on by many agents over a long time. To avoid accumulation of technical debt and errors, we optimize more strongly for the following instrumental objectives than we do already for publication quality, and for short-term success at tasks.

- **Verifiability**: We stick to true claims, and distinguish strength of evidence, empirical versus theoretical support, observation from inference under potential overlooked hypotheses, aspirations from historical from current state, and so on. We also make it easy to check claims by linking them to their source of truth, and to evidence that previous checks were done. The main guidance here is to notice when a check was annoyingly hard, and to then add more signposting, cached reasoning results, full reasoning traces, references to all evidence, timestamped comments/markers that something was checked and by whom, and so on. The opposite pressure comes from avoiding staleness and reducing complexity, so we don't bother to record the full reasoning, but instead aim for the sweet spot where future agents can reproduce the steps between source of truth and the final claim they are checking.
- **Reproducibility**: Relatedly, everything should be reproducible from source truth, albeit we of course supply structure to speed up the process. This includes data, interpretation, writeup, but also code features, test cases, planned tasks and conventions.
- **Navigability**: We minimize the risk of future agents not finding relevant information, or being drowned in irrelevant material. This includes longer than usual speaking filenames, use of predictable standard terminology that can be grepped for, and cross-referencing between files. Navigation mostly works on a file-level, so we keep files single-concern.
- **Clarity**: Most code, math and text is read many times, so we optimize both code we created and we inherited from previous agents for readability and maintainability. This mainly includes using plain, specific, unambiguous descriptions, standard terminology, examples instead of analogies, and iteration to remove sources of complexity once a better alternative is found. Sentences should be broken down when they entangle multiple concerns, to be easier to edit.
- **Tracking**: We track tasks, progress, todos in the repo instead of external tools. Git tracks the history for us, to keep the current state of the repo more simple and focused on active and future work.

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
  preamble.tex
  bibliography.bib
  *.tex
experiments/
  MAP.md
  figure_config.py
  <topic>/
    Cargo.toml
    src/
      lib.rs
      *.rs
    <experiment>/
      *.rs
      *.py
      *.jsonl
      figures/
    <nested-package>/
      Cargo.toml
  verification/sage/
research/
  INDEX.md
  *.md
  sys-landscape-datascience/
papers/<abbreviationYear>/
/tmp/  (outside repo)
```

- `thesis/` is publishable and self-contained. It owns or copies publication
  assets and does not `\input` files from `formal/`, `experiments/`, or
  `crates/`.
- `crates/` contains durable Rust crates that are reusable beyond the thesis.
  No external users or dependencies exist until after submission.
- `formal/` contains developer-facing mathematics named by formalized objects,
  theorem clusters, and proof clusters. We prove that our algorithms work in
  addition to empirical testing.
- `experiments/` contains Rust/Python experiment packages grouped by topic. Data
  and figures live next to the experiment that produced them. Use
  `experiments/MAP.md` and local manifests to find each package's binaries,
  analysis scripts, and artifacts.
- `research/` contains interpreted analysis, decision history, proof-route
  state, topic summaries, and the `research/INDEX.md` navigation cache.
- `papers/<abbreviationYear>/` contains downloaded arXiv paper sources.
- `/tmp/` is the place for ephemeral prompt snippets, one-off reports, and
  generated artifacts that should not become repo source truth. Promote durable
  outcomes into the relevant repo surface instead of leaving them in `/tmp/`.

Domain map files:
- `research/INDEX.md`: interpretation notes, proof-route state, research-result
  caches, and topic-summary routing.
- `crates/MAP.md`: durable crate boundaries, API tiers, and core entities.
- `experiments/MAP.md`: experiment topic packages, helper crates, data patterns,
  and provenance.
- `thesis/submission/README.md`: university forms, submission mechanics, and
  preservation actions.

Map and index files are navigation caches over source truth. Their file-local
HTML comments say how to check or refresh them.

## Harness Map

```text
.agents/
  skills/<skill>/
    SKILL.md
    references/
      *.md
    scripts/
      *.sh
      *.py
.codex/
  worktrees/
  agents/
    <agent>.toml
  config.toml
ROADMAP.md
tasks/
  README.md
  verify-thesis-done.md
  <group>.md
.devcontainer/
  README.md
  codex-cloud.md
  devcontainer.json
  Dockerfile
  *.sh
scripts/
  codex-worktree.sh
  toc.sh
/tmp/  (outside repo)
```

- `.agents/skills/` contains repo-local skills discoverable by name and
  description. Do not treat `AGENTS.md` as the complete skill index.
- `.codex/worktrees/` contains repo-local worktrees for isolated Codex sessions.
- `ROADMAP.md` and `tasks/` route work, cache task state, and describe
  objectives; domain details usually live in `research/` or the relevant domain
  surface.
- `.devcontainer/` documents and configures the local devcontainer and Codex web
  environment, including setup, smoke, and cache-warmup scripts.
- `/tmp/` is the default place for temporary agent prompts, worker packets,
  draft reports, and artifacts to inspect or show Jörn. Do not rely on `/tmp/`
  for terminal verdicts or durable project state.
- `scripts/codex-worktree.sh` creates repo-local Codex worktrees.
- `scripts/toc.sh` prints the headings with line numbers for a given file. TODO: this could be a single-line bash command tbh if we don't use ranges but just linenumber:linecontent.
- This repo does not use nested `AGENTS.md` instruction maps; use root
  `AGENTS.md`, discoverable skills, and descriptive `MAP.md` files instead.

Harness map files:
- `ROADMAP.md`: thesis closeout streams, current phase, and task routing.
- `tasks/README.md`: task-bundle status/cache conventions for editing
  `tasks/*.md`. TODO: move to a skill.
- `tasks/<group>.md`: topic and cross-cutting mini-roadmaps with cached task
  state.
- `tasks/verify-thesis-done.md`: final thesis-done gate.

Roadmap and task files own work routing, not domain proof or experiment truth.
Use their file-local comments and `tasks/README.md` for refresh rules.

## Environment

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
cargo test -p algebraic-numbers --release --lib
cargo clippy -p algebraic-numbers --lib -- -D warnings

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

## Conventions

All conventions serve the long-term quality objectives, the final publication objectives, and short-term task success within a single agent session. We don't document in `AGENTS.md` what serves what, often multiple benefits apply.

### General

**Navigation and Exploration**
- use long descriptive names for files and folders
- use predictable code symbols, keywords, latex labels; grep to quickly find definitions and uses
- cross-reference other files, avoid unstable line numbers
- TODO: map regeneration skill

**Clarity**
- write plainly, don't use metaphors or analogies
- focus on information transfer to future agents
- use standard terminology
- be specific, neither over- nor under-inclusive
- break down sentences that entangle multiple concerns
- avoid vague terms
- don't abstract prematurely

**Verification and Tracking**
- link claims to their source of truth, except where obvious
- record enough arguments and intermediate steps to enable agents to easily check whether some reasoning result is true and detect when the underlying source of truth has changed
- note that often it's kinda obvious what arguments support a conclusion, and the real work was elevating the hypothesis
- explicitly track epistemic status of claims, such as empirical versus theoretical evidence, strong versus weak support, potential unknown unknowns i.e. overlooked hypotheses, diverse versus correlated arguments
- track task states, external decisions from Jörn, and results of expensive tests/checks to allow future agents to deem checks unnecessary/unchanged
- move unnecessary claims into the git history i.e. delete them, since they are expensive to verify

### Chat With Jörn
- use chat and `/tmp` artifacts to make Jörn's feedback efficient; keep durable
  repo files optimized for future agents and source truth
- gather repo evidence and do preliminary reasoning before asking Jörn for decisions
- ask Jörn for thesis scope, mathematical judgment, advisor-facing framing,
  taste, external-world actions, and design pivots; do not ask him to do
  agent-checkable grep, inventory, comparison, or first-pass drafting
- present long math to Jörn via pdf, not latex source; use chat for short math.

### Tasks
- `ROADMAP.md` is an overview of `tasks/`, not a source of truth.
- `tasks/` documents what remains to be done, but defers to `research/` and the other domain files for domain-specific details. It tracks what needs to be done, with what dependencies.
- write plainly, focus on the outcomes and how to measure success
- don't prematurely prescribe the method, don't prematurely make decisions, don't prematurely abstract and generalize, don't prematurely define tasks more thoroughly, nor prematurely promote mere ideas to accepted todos.
- don't invent unnecessary structure, agents just read entire task group files

### Rust Code
- functional programming style to avoid bugs from mutable state
- simple data types and function signatures to avoid complex abstractions
- explicit error types with `Result<>` instead of ambiguous `Option<>`
- we must have algorithms we trust, so we must correspond to formalized mathematics. use code comments to reference latex lemmas and their proofs, explain how math and code symbols correspond where not obvious, and track invariants/propositions in code comments where not obvious.
- use `clippy` and fast smoke tests that catch various classes of programming bugs
- use slow correctness test suites that aim to falsify empirically our mathematical work
- profile and use benchmarks to identify the few hotspots where performance or memory matters at all
- add regression tests to learn from past bugs
- don't mix multiple concerns into one function, test, or api
- don't prematurely abstract, don't prematurely generalize, don't prematurely optimize performance, don't prematurely add features (YAGNI)
- reduce indirection, define input/output data structures close to the function(s) that use them
- repo status: don't treat the public api as settled
- duplicate code when the context is genuinely different, abstract only when the contexts are a genuine family and the abstraction is simpler to understand than having multiple specialized versions (anti-DRY)
- use simple, predictable standard patterns for the job

### Python Code
- similar to Rust: write plainly, avoid abstraction, be predictable, and so on
- we mostly script/orchestrate with rust, so imperative style and little typing is fine
- stick to a "data science" style for rapid development
- use `Path(__file__).resolve().parent` for paths relative to the script
- use `experiments/figure_config.py` for figure styling when relevant
- figure captions should state observations before interpretation

### LaTeX in formal/
- agents are the most frequent readers, Jörn reviews for correctness and clarity, not style
- write plainly, be specific, neither over- nor under-inclusive, break down sentences that entangle multiple concerns, avoid vague terms, avoid analogies and metaphors
- do not invent new terminology, disambiguate terminology with extra adjectives and long names. the agent readers are familiar with most mathematical literature and naming/notation conventions.
- the focus is rigorously formalized mathematics that allows us to catch any wrong statements and edge cases.
- clearly track the verification status of mathematical writing, such as whether Jörn reviewed a formalization or proof, what gaps remain and why those look closeable, whether the proof idea is trusted and notation troubles are the obstacle, whether generic/main cases are trusted and what edge cases cause trouble, and so on
- use comments to track the "why" behind the current definitions/statements/proof methods, don't discuss historical attempts beyond what matters for the current state and for anticipated future work.
- use grep-able latex labels and reference them
- be fully rigorous in what conditions and guarantees lemmas claim, and in what inputs and outputs algorithms provide.
- new agent-written mathematics is unapproved unless it is mechanical or
  explicitly approved by Jörn
- don't hardcode theorem numbers; use labels and check references

### LaTeX in thesis/
- the audience for which we write is Kai, Elizabeth, and the hypothetical master students who build upon this thesis in the future.
- Jörn reviews for correctness, clarity, and presentation style.
- We target a professional, publication-ready, pure mathematics style when we write about symplectic geometry from a pure mathematician's perspective, and a more applied/data-science style when we write about experiments.
- Software engineers are not part of the audience, so we don't focus on code.
- Formatting of figures, including fonts and size and colors, are owned by the python code. Latex simply includes the images/pdfs.
- `thesis/` is self-contained and does not `\input` files from `formal/`, `experiments/`, or `crates/`. We deliberately copy assets into `thesis/` when we need them for publication.

### Research notes
- the audience is future agents, and indirectly (via chat) Jörn
- write plainly, focus on content, make reasoning traceable by providing arguments and intermediate steps instead of just conclusions whenever the elevated hypothesis alone is not obviously true already
- track the epistemic status of claims
- use `research/INDEX.md` to cache interpreted research results, proof-route state, and topic summaries, and link to the relevant domain files for details
- split experiments when it becomes hard to achieve multiple purposes/answer multiple questions in one experiment, copy and edit code cheaply
- track carefully the current prioritized subquestions/subgoals, in particular distinguish exploring the feasability of an idea, strengthening the evidence of a weak result, aiming to falsify, aiming to distinguish between hypotheses, producing evidence that is more legible even though it contains no new/additional information, refactoring/cleaning the experiment for long-term maintainability, and so on. Often multiple subgoals can be pursued at once - but not always all of them.
- experiments should be reproducible from scratch given all related research notes
- repo state: we now have the main and side results nailed down, and each experiment supports only one line of inquiry

### Papers
- prefer latex over markdown over pdf due to friction for agents during reads
- arxiv offers latex sources
- transcribe pdfs into markdown, then fix once, so future agents can rely on the more readable format
- we don't edit the sources, at most we add documentation
- compile latex sources to get the published numbering

### Experiments
- the research notes describe what the experiments are for, and interpret their results.
- sibling experiments should be mostly independent from each other, to faciliate rapid development
- data is located next to the producer
- do not patch-edit generated `.jsonl`, `.csv`, or figure outputs; regenerate
  them or document the needed refresh
- if tracked generated data changes unexpectedly, stop and report the file and
  command
- use script-like python and rust binaries, make the pipeline simple and reproducible and documented
- for development, provide smoke paths (smoke input data, smoke output data, smoke parameter settings)
- for large datasets, provide a slurm job script to be run on LICCA
- shared code is owned by the parent of the experiments that use it
- we use jsonl for data, because agents can manipulate it easily, and it's flexible enough for the rust row types we have

### Cluster and external execution
- agents do not have LICCA SSH access; prepare scripts, binaries, resource
  choices, and retrieval instructions for Jörn instead
- Jörn submits cluster jobs and retrieves external results unless the files are
  already present locally
- resource choices need a short justification

### Using Subagents
- subagents are for bounded first-pass labor, bounded verification, and
  independent checks; the top-level session owns integration and final claims
- delegate output is untrusted evidence until checked
- every subagent prompt needs a required cwd, scope, ownership, success check,
  output format, reserved decisions, and stop condition
- don't prematurely prescribe the approach, focus on the outcome and how to measure success
- use `gpt-5.3-codex-spark` for super-fast low-intelligence tasks such as text
  refactoring without a need for scientific understanding or reasoning

### Git, Worktrees, Merging
- inspect git status before deleting, moving, or replacing active paths in main
- use worktrees for isolated or parallel overlapping work
- destructive git operations require explicit approval
- merge conflicts are resolved by semantic truth and current repo state, not by
  timestamp, branch side, author, or task ownership
- merge-to-main requires approval from Jörn, and a thorough review of the branch

### Post Mortem
- after sessions, reflect on what was necessary for success, and what was wasted effort
- report a blameless post-mortem in chat, don't follow-up with high-risk actions
- brainstorm, triage and present potential repo changes that affect future agents positively
