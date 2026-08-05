# AGENTS.md

## Purpose

This repository exists to produce Jörn Stöhler's published master thesis,
*Probing Viterbo's Conjecture*, supervised by Kai Cieliebak and Elizabeth Gaar.
The terminal deliverable is the printed-quality thesis at
`thesis/build/main.pdf`. Code, formal work, experiments, and retained evidence
are durable sources for that thesis, not terminal value by themselves.

Prefer work by its expected contribution to the published thesis. Ask before
expanding work whose contribution is unclear.

## Find the relevant work

Use this intentionally incomplete map to choose a search domain, not as a
reading list:

```text
/workspaces/msc-math/
├── ARCHITECTURE.md    stable domain and authority map
├── docs/              project status, confirmed facts, cross-domain views
├── thesis/            publication text; main.tex controls the active PDF
├── formal/            statements, proofs, audits, unresolved obligations
├── experiments/       producers, evidence, analyses, interpretation
├── crates/            reusable Rust libraries and tests
├── papers/            source papers and paper-specific notes
├── submit/            submission and administrative sources
├── .agents/skills/    conditional task workflows
└── .worktrees/        isolated checkouts; not project content
```

Open a named source directly. Otherwise choose the smallest plausible domain,
then use its README only if orientation is needed. Search filenames, text,
symbols, and manifests; read sources before relying on summaries. A lexical
miss is weak evidence of absence because related work may use other terms.

Route thesis and formal work through `$thesis`, experiments through
`$empirical-research`, source-paper work through `$paper-conventions`, and Rust
work through `$rust` when its project-specific contracts matter. The active
skill catalog and skill frontmatter own exact trigger boundaries; read every
matching skill before acting.

## Evidence and authority

Current sources, tests, data, producer outputs, local notes, accepted Jörn/Kai
decisions, and active thesis text overrule summaries.

- `docs/project-facts.md` records still-current Jörn-confirmed facts.
- `docs/project-status.md` records project state, not mathematical truth.
- `docs/capabilities.md` is a cross-domain view, not independent evidence.
- `DEVELOPMENT.md` files contain maintainer notes; a README has only the
  authority of the sources behind its claims.
- Regenerate generated artifacts from their producer; do not hand-edit them.
- Session logs, old branches, and `/tmp` are provenance or salvage sources, not
  current project state. Use `$codex-session-log-parsing` for local Codex logs.

The repository deliberately retains negative results, alternatives, and
superseded routes. Before declaring a project-wide proof, experiment, or
implementation gap, broaden terminology, inspect plausible domain READMEs, and
report the searched scope.

Formal work may be stronger, weaker, or superseded relative to the active
thesis. Producer-generated datasets remain attributable to their producer;
consumers name the producer output or data contract they use.

Four-dimensional coordinates use `(q1, q2, p1, p2)`. Prefer coordinate-free
notation when their order is irrelevant.

## Keep the work moving

- Continue until the current scope is complete, explicitly paused, locally
  blocked, or waiting on Jörn is worth its attention cost. Incomplete scope
  without a blocker means continue.
- Do not ask what should be done when the agent can generate the options,
  predict consequences, and compare them against known goals. Choose locally.
  Ask only for a missing fact, option, prediction, constraint, or preference
  likely to change the choice.
- Treat incoming messages as information or coordination unless they clearly
  request action; do not infer a task. Feedback constrains the active
  work unless it explicitly replaces it. Interpret explicit requests to record
  something for later or interrupt the work literally.
- A repair request does not authorize redesigning accepted objectives,
  constraints, or workflow unless current evidence makes redesign necessary.
- Do accessible local work. Ask Jörn for mathematical or stakeholder cruxes,
  private context, LICCA access, mail, or administrative actions.

### Worktrees, Main, and harness changes

- Main is read-only unless Jörn requests that exact Main edit. Ordinary changes
  use a worktree. Do not merge an exact candidate into Main before Jörn approves
  it; after approval, perform the merge unless he assigns it elsewhere or asks
  to leave it unmerged.
- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are frozen
  unless Jörn requests harness work. Use `$gpt-56-harness`; also use
  `$skill-creator` when changing a skill.
- Preserve unrelated user changes in dirty worktrees.

### Subagents

- Delegate bounded work that divides cleanly, especially when parallelism cuts
  critical-path wall time. The root retains dependency order, synthesis,
  merge-readiness, and value/cost decisions.
- For every fresh subagent, set `fork_turns="none"` and explicitly choose its
  model and reasoning effort.
- If delegation needs non-obvious context, ownership boundaries, completion
  evidence, or a return contract, use `$subagent-prompting`; write and
  self-review its prompt in `/tmp/prompts/`. Keep a genuinely one-sentence
  assignment inline.
- Treat decomposition and model choice as empirical rather than a fixed routing
  table.

### Cross-session work

Perform a handoff or prepare an exact relay when a known person, root session,
or agent depends on the result; do not invent dependencies.

Root sessions handle routine coordination themselves: check known owners and
dependencies, route information, claim work, delegate bounded pieces, or record
work for later. Ask Jörn only when ownership depends on private priorities or
stakeholder judgment.

The current thesis-completion backlog is documented at
`/tmp/msc-math-thesis-backlog/README.md` and stored in
`/tmp/msc-math-thesis-backlog/backlog.jsonl`. Read its README before use. It is
only cross-session operational state: root sessions update it, subagents report
through their root, and it is not mathematical, empirical, or project source
truth. This rule becomes inert when either file is removed.

## Communicate with Jörn

- Write plainly in existing project terms. Transfer useful state, evidence,
  uncertainty, blockers, and decisions; omit routine narration.
- Do agent-doable option generation and comparison first. Ask only for a real
  crux, with enough context to answer it. Separate unrelated concerns, batch
  related questions, and make every request explicit in a self-contained final
  message.
- Do not end the turn while useful agent-doable work remains. When waiting is
  necessary, say exactly what depends on Jörn. Make cold resumption cheap.
- For review, link the complete artifact and an exact full-context diff in a
  unique `/tmp/joern/*.diff`; name its base and candidate.

## Documentation

Put durable repository-specific facts, decisions, evidence, status, sharp
edges, and expensive checks beside the code, artifact, question, or contract
that makes them interpretable. Use conventional layouts and stable,
grep-friendly names. Declare dependencies where consumed; do not imply that a
manually maintained producer-side consumer list is exhaustive.

Keep active and superseded material visibly distinct. Navigation views state
their coverage and support only the claims they establish.

Place a helper owned by a conditional workflow in
`.agents/skills/<owning-skill>/scripts/`, a standalone repository-wide
deterministic helper in repository-root `scripts/`, and a producer
transformation beside its producer. Keep one-off probes in `/tmp`; promote one
only after it has recurring value and a clear owner.

## Baseline commands

Before writing any command Jörn should run on, to, or from LICCA, use `$licca`.

```bash
# Worktree
GIT_LFS_SKIP_SMUDGE=1 git worktree add .worktrees/<name> -b <branch> main

# Python
uv run --script thesis/figures/foundations/generate.py
# Wrong: python3 thesis/figures/foundations/generate.py  # bare Python lacks declared dependencies

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

Producer and experiment READMEs own their commands and output-safety rules.
Read the local README before running anything that may overwrite tracked
evidence.
