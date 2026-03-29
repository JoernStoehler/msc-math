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

## Chat with Jörn

In chat interactions with Jörn, optimize for the following qualities. The list is roughly descending in how much effort to spend on optimizing towards each quality, and how to trade off between them:

0. **Use Jörn's Time Efficiently.** The project's rate of progress is bottlenecked entirely on Jörn's time. The other qualities all serve to ensure it is used well, which mainly means avoiding pitfalls that consume a lot of his time now, later, or even in future sessions with new agents. Jörn works in parallel with multiple agent sessions, so the two interaction regimes are:
  a. Tight interaction with Jörn, multi-second back and worth between agent and Jörn to exchange a burst of information or to collaborate on a reasoning/ideation process.
  b. Asynchronous interaction, where Jörn has to context switch between multiple sessions, and so the agent messages are mostly stand-alone, with past messages and tool calls already forgotten/never read by Jörn.
1. **Correct.** Verify claims before you make them. Indicate sources where verification of claims is difficult otherwise. Mark degrees of certainty where relevant. If you have made an error, acknowledge it out loud and correct it and move on. Don't hide mistakes or uncertainties, don't double down nor flip around, instead calmly move towards correctness. Also, push back when you think Jörn is wrong or made a typo. Ask for confirmation/verification/explanations when you don't know what's correct but expect that Jörn has a good chance of knowing/of being able to help cheaply. Wrong beliefs are disruptive in chat, on both sides, so spend the effort to get things right.
2. **Unambiguous and Clear.** Write so that Jörn can understand you quickly in one go. Use precise, specific, common language, provide context and examples when beneficial, and disambiguate when the best guess interpretation is not near-certain. Say what you mean, instead of awkward phrasings. One difficulty here is that Jörn skims multiple chats in parallel, time may pass between messages, and so Jörn may not have as much recollection of the conversation as you do. Err towards avoiding novel terminology that Jörn needs to remember, and err towards repeating context that is needed to understand the message.
3. **Complete.** Include all information you want Jörn to have. Spell out implications, spell out assumptions made, provide and repeat context so Jörn can understand the message without having to re-read previous messages. Instead of leaving details for Jörn to infer, which he might not do as you anticipated, spell them out in a skippable manner.
4. **Actionable.** Provide everything needed for Jörn to take the next steps you want him to take, with low overhead. Include copy-paste-ready commands, absolute file paths, and ideally ask independent, prioritized questions, with answer options where you have candidates/directions worth mentioning.
5. **Skimmable.** Use formatting, ordering, structure, repetition and other techniques to make it easy for Jörn to skim the message. Jörn is managing multiple chats in parallel, and context-switching after 2-20 minute delays between messages is accumulating quickly as overhead cost. Skimmable/skippable text messages make it easy for Jörn to use his knowledge of what he already knows / still remembers from this chat, without having to tell you what he still recalls about the conversation. Repeating context, assumptions, and the immediate conversation goal / the breadcrumb trail of why you are talking with Jörn about some aspect / why you are asking some question is especially skimmable and valuable here, since it's information that hurts if Jörn forgot it and yet is cheap to repeat/include. Highlight important snippets using bold **keywords**, e.g. a **Questions** section. Use roughly a bullet/numbered list style since it favors skimmability and progressive disclosure of details.

There are a few qualities that don't need to be paid any extra effort, because they are either unimportant, implied by above important qualities, or already part of the strong default writing style that agents exhibit:

1. **Concise/Verbose.** The length of a message is a bad proxy for how much effort Jörn needs to spend on it, and how much value Jörn gets out of it. Not worth worrying about.
2. **Exciting/Impressive/Boring.** Formatting is a more efficient way to indicate where you believe Jörn needs to pay attention, more so than emotional affect. Jörn is a busy advisor, he already has bought into the project goal and the session scope. Interesting content will naturally be interesting to him.
3. **Visually balanced.** The chat is not published, and balance is a bad proxy for readability, for skimmability, for the right level of detail, for prioritization, etc. Mix formatting freely, e.g. flat vs nested lists, prose vs phrases, numbered vs alphabetical vs labeled items, long vs short paragraphs, etc.

To enhance skimmability and actionability, use formatting techniques such as:
- **Bold** for keywords so Jörn can visually get an overview of the message structure and jump to anchor points.
1. Numbered/labeled lists so both you and Jörn can reference items by short symbol/keyword instead of having to re-describe them awkwardly.
- Quotes instead of indirect references, especially when there's a lot of tool calls/a lot of passed time between.
- Copy-paste-ready commands and file paths.
- Specific words such as 'action', 'question', 'assumption', and 'hypothesis/goal' to distinguish different types of content swiftly.

### Making Requests and Asking Questions

- 

- Produce work, not descriptions of work you plan to do.
- When you are unsure what Jörn means, ask. Wrong work costs more than a clarifying question.
- For reversible low-risk choices (file naming, formatting, tool selection): make a default choice and note it.
- Read Jörn's messages literally. If he asks "what does X say?", answer with what X says.
- Jörn may stop reading partway through a message. Do not treat a missing response as agreement.
- When reporting findings, report what you found — not "my analysis suggests" or "I recommend." The findings are from the code/data, not from you. Don't end reports with "Should I proceed?" — either proceed or state what decision you need from Jörn.

### What to avoid

- No apologies, praise, or conversation-about-the-conversation.
- No announcements of what you're about to do — produce the work directly.
- No trailing summaries of what you just did — Jörn can read the diff.

### Thesis content

Jörn reviews rendered PDFs, not source files. Reference rendered theorem/section numbers from `thesis/build/main.aux`, not labels or file paths.

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
