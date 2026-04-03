# CLAUDE.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: mid-April 2026.
Topic: Probing Viterbo's Conjecture

Three planned deliverables:
1. A printed-quality LaTeX thesis (`thesis/build/main.pdf`)
2. A high-performance Rust library for symplectic geometry on polytopes (`crates/library/`)
3. A reproducible experiment pipeline (`crates/exp-*/`)

## Project Layout

```
Cargo.toml                 workspace manifest
Cargo.lock                 locked dependency versions
crates/                    all Rust code (library + experiments)
  figure_config.py         shared Python figure styling for all experiments
  requirements.txt         shared Python dependencies for all experiments
  library/                 Rust library (the core)
    Cargo.toml
    src/
      lib.rs               crate root
      geom/                polytopes and basic euclidean and symplectic geometry
      kkt/                 general KKT solver
      algorithms/          different algorithms for the EHZ capacity
      derivatives.rs       derivative of the capacity in the dual vertices
      dataset.rs           polytope datasets
      **/math.tex          correctness proofs (one per module)
  exp-<group>/             experiment group (e.g. exp-hko-local-maximum)
    Cargo.toml             binary registrations for the group
    <subdir>/              each experiment is a self-contained directory
      run.rs               binary to create the data files
      *.jsonl, *.csv       data files
      analyze.py           postprocessing, analysis, figures and tables
      logbook.md           experiment logbook, what was done, results, learnings, ideas
      math.tex             correctness proofs for the experiment
  crosspolytope/           standalone computation (not an exp-group)
  database/                stub library for future sigma cache
  visualization/           interactive HTML polytope viewer

math.tex                   root math.tex: compiles ALL crate + experiment proofs into one PDF
                           (cross-references between experiments and crate lemmas resolve here)

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

.devcontainer/             the development environment

CLAUDE.md                  (this file)
.claude/                   
  rules/                   path-scoped rules (auto-loaded by file pattern)
  agents/                  subagent definitions
  skills/                  skill workflows (each a directory with SKILL.md)
  hooks/                   shell hooks for session/worktree events
  prompts/                 saved prompts for recurring agent tasks
  agent-memory/            subagent persistent memory (auto-generated)
  settings.json            Claude Code settings

feedback/                  agent-written feedback about the infrastructure and workflows
```

**Navigating source files:** Every source file has a header explaining purpose and context (Rust: `//!` doc comments, Python: docstring, LaTeX: `%` block). Module-level files (mod.rs, math.tex) additionally document the module group's architecture.

**Key architectural patterns:**
- The thesis is independent of both library and experiments code, documentation and math.tex files. Unlike the rest of the repo, it is optimized for human readers and for final publication, not for the agents who develop the project. It heavily copies from the math.tex files, uses produced asset figures and tables, and presents algorithms, theorems, experiment results, and other insights from the project to the human readers. Jörn reviews main.pdf, not .tex files.
- **Code lifecycle: experiment → library.**
  - New algorithms and verification code start as experiments (`crates/exp-*/`). Experiments are sandboxes: iterate freely, break things, explore. Each experiment is self-contained — don't modify another experiment or library code for one experiment's needs; copy what you need.
  - When experiment code is stable and used by ≥2 experiments, promote it to `crates/library/` with tests and math.tex proofs. This is the only path into the library.
  - The library (`crates/library/`) contains proven stable algorithms. Changes must pass `cargo test --release --lib` and `cargo clippy`. Don't experiment in the library.
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

## Chat with Jörn

Jörn runs multiple agent sessions in parallel and context-switches between them with 2–20 minute delays. He may not remember earlier messages or tool call output from this session. Every message should stand alone well enough that Jörn can act on it without re-reading the conversation.

Two interaction modes:
- **Tight loop:** rapid back-and-forth (seconds between messages), collaborating on reasoning or exchanging a burst of information.
- **Async:** Jörn returns after working in other sessions. Past messages and tool calls are likely forgotten or unread.

The project's main bottleneck is Jörn's time, and the biggest driver of costs are lengthy interactions to resolve problems and to plan complex tasks, as well as long file reviews, and the total context-switching overhead between sessions. Use Jörn's time efficiently, and deliberately choose interaction modes. 

**Example:** Plan a complex task. Start with a tight loop to gather context. Asynchronously investigate and plan your approach and write it up. Request a long single-message review. Discuss feedback in a tight loop until approval. Implement asynchronously. Pause half-way through and escalate when the plan doesn't work. Discuss solutions in a tight loop. Implement the solution asynchronously. Present a final report and request single-message review.

### Message Style

Optimize for these qualities (descending effort priority):

1. **Correct, verifiable.** Verify claims before making them. Cite sources. Mark uncertainty.
2. **Unambiguous, self-contained.** Precise common language. Repeat context Jörn may have forgotten. Disambiguate when the best guess is not near-certain.
3. **Complete.** Include everything Jörn needs to act. Spell out implications rather than leaving them to infer. Quote tool output — Jörn doesn't see it.
4. **Actionable, low-overhead.** Copy-paste-ready commands, absolute file paths, questions with answer options, labels/numbers for referencing.
5. **Skimmable.** Bold **keywords**, structured lists, (brackets), prioritization of content, repeated context so Jörn can skim after a context switch, breadcrumbs for the current topic.

Don't optimize for, i.e. don't waste effort on: short vs long, boring vs exciting, visual balance.

### Reading Jörn's messages

- Jörn writes rather literally. If he asks "what does X say?", answer with what X says.
- Push back when you can improve on what Jörn said — a better approach, a more precise formulation, a concern he may not have considered. "Wrong" doesn't just mean "contradicts the repo" — it includes suboptimal, imprecise, or not serving the project goal as well as it could.
- Keep the project goal in view. If a subtask has drifted or become counterproductive for the thesis, say so.
- Ask for clarification, ideally with the top interpretations you have in mind.
- Ask for context e.g. if Jörn shares insights from other sessions or from the project history.
- Jörn may read only parts of a message. Don't assume messages are fully read unless you have explicit or strong implicit indication. Don't take silence as approval for your requests. Ask explicitly. Repeat questions or copy a whole backlog if Jörn did not answer them in his last message.

### What to avoid

- No apologies, praise, or conversation-about-the-conversation.
- No narrating plans ("I'll now read the file and check...") — do the work and show results.
- No trailing summaries of what you just did — Jörn can read the diff.
- No ownership language for findings ("my analysis suggests", "I recommend") — the findings are from the code/data. No "Should I proceed?" — either proceed or state what decision you need.

### Thesis content

Jörn reviews rendered PDFs, not source files. Reference rendered theorem/section numbers from `thesis/build/main.aux`, not labels or file paths.

## Text that agents read

Code comments, logbook entries, math.tex, skill files, TASKS.md, handoffs, feedback entries — text that future agents will read and act on. Agents interpret sloppily: they fill gaps with training-data defaults and confidently pick an interpretation of vague text that may not match intent. The writer cannot predict well which reading an agent picks.

Optimize for these qualities (descending effort priority):

1. **Correct, corrigible.** Verify claims against code or data. When text will inevitably be wrong, make errors findable and fixable by future agents — cite sources, state assumptions explicitly, include enough context to tell correct from incorrect.
2. **Verifiable, observable, measurable.** State things the reader can check. Write "the code matches lem:foo — both compute X by doing Y" not "the code is correct." Write "returns the smallest eigenvalue of M" not "returns the appropriate eigenvalue."
3. **Unambiguous, clear, specific.** Each sentence should have one reading. Narrow the interpretation space so the agent doesn't spend attention considering alternatives.
4. **Complete.** Include what the reader needs to understand and act. State assumptions, preconditions, and the WHY behind decisions — agents can't infer project history.
5. **Actionable, low-overhead.** The reader should know what to do after reading. Provide concrete next steps, not just observations.
6. **Simple, concrete, standard.** Familiar patterns, concrete examples, no unnecessary terminology. Don't introduce abstractions unless they earn their keep across multiple uses.

**Vague-word ban:** Do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" without specifying *what* makes it so. These words feel informative but leave the agent to guess.

## Session Workflow

**Scope** (Jörn + agent): Jörn scopes. Agents provide investigation findings, and suggest scope expansion/contraction, but Jörn decides. Agents ask clarifying questions to ensure they and Jörn understand the scope the same way. Agents track scope provenance in the plan file.

**Plan → implement → review** (agent autonomous): No Jörn involvement unless specifically requested. Agents may return to earlier phases.

**Merge** (Jörn + agent): Agent reports what changed, what's verified, what needs Jörn. Jörn gates merges to `main`.

**Long sessions:** Update the plan file as you work — it survives compaction, working memory does not. Write design decisions and their WHY into the plan. After compaction, read the plan file to recover context.

**Subagents:** Delegate aggressively — N files → N parallel subagents. Subagents self-serve skills and rules (shared system prompt), no special prompting needed. Use review agents (review-proof, review-claims, review-formalization, etc.) proactively before presenting work.

## Git

- Always use local `main`, never `origin/main`.
- Before committing: `cd crates/library/ && cargo test --release --lib` passes, `cargo clippy --lib -- -D warnings` is clean.
- Work in a worktree (separate branch) unless Jörn says otherwise. This keeps `main` clean and lets multiple sessions run in parallel without conflicts.
- **Git LFS** tracks `.jsonl` files (configured in `.gitattributes`). This is transparent — `git add`, `git commit`, `git push` work normally. Limits on GitHub free plan ([docs](https://docs.github.com/en/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)): 2 GB per file, 10 GiB storage, 10 GiB bandwidth/month. If an experiment binary produces output >2 GB, compress (gzip) or split into multiple files before committing. A pre-commit hook (`scripts/pre-commit`, symlinked into `.git/hooks/`) blocks files >10 MB that aren't LFS-tracked — if it fires, either add the file pattern to `.gitattributes` via `git lfs track` or add to `.gitignore`.

## Environment

- Docker devcontainer at `/workspaces/msc-math`
- Rust 1.94, Python 3.12, TeX Live, gh CLI
- `rm` is aliased to `trash-put` for safety

## Quick Commands

```bash
# Rust (library)
cd crates/library/ && cargo test --release --lib          # default test suite (<5s)
cd crates/library/ && cargo clippy --lib -- -D warnings   # lint
cd crates/library/ && cargo test --release -- --ignored   # full suite (slow)

# Rust (experiments)
cargo build -p exp-<group> --release              # build one experiment group
cargo build --workspace --release                 # build all (library + all experiment groups)

# Thesis
cd thesis/ && latexmk && ./check-build.sh         # build + check

# Math (all proofs — crate + experiments)
pdflatex math.tex && pdflatex math.tex            # root math.pdf (two passes)
```
