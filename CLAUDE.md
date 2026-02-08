# Master Thesis: Probing Viterbo's Conjecture

Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak, Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – March 2026

## [aspirational] End state

This repo is a completed master thesis with:
- A printed-quality LaTeX document in `thesis/`
- A high-performance Rust library for symplectic geometry on polytopes in `crates/`
- A reproducible Python pipeline in `experiments/` that starts from zero data and produces all figures and tables
- All code tested, all results reproducible

## Repo structure

```
CLAUDE.md          This file (all agents read this)
thesis/            LaTeX thesis document
  CLAUDE.md        Thesis-specific conventions
crates/            Rust workspace (cargo build/test from here)
  CLAUDE.md        Rust-specific conventions, testing philosophy, math docs
  geom2d/          2D symplectic geometry primitives
  geom4d/          4D convex geometry, polytopes, symplectic structures
  hk2017/          Haim-Kislev 2017 algorithm (all polytopes, exponential cost)
  billiard/        Billiard algorithm (Lagrangian products only, fast)
  tube/            Tube algorithm (no Lagrangian 2-faces, moderate cost)
  datasets/        Dataset generation orchestration
experiments/       Python scripts consuming Rust-generated datasets
  CLAUDE.md        Experiments-specific conventions
  scripts/         Independent .py scripts
  data/            gitignored — populated by Rust pipeline
  figures/         gitignored — populated by Python scripts
papers/            arxiv .tex sources for reference (HK2017, CH2021, HK-O 2024, ...)
```

## Mathematical context

We compute the EHZ capacity (minimum action of generalized Reeb orbits) for convex polytopes in R^4. By a theorem of Haim-Kislev 2017, there exists a minimum-action orbit that is piecewise linear, uses pure facet Reeb vectors, and visits each facet on a contiguous time interval. This reduces the problem to a finite combinatorial search.

Viterbo's conjecture: sys(K) = c_EHZ(K)^2 / (2 vol(K)) <= 1 for all convex bodies K.
Haim-Kislev 2024 (Annals) gave a 10-facet counterexample with sys > 1.
We probe the conjecture by computing sys across large polytope datasets and looking for patterns.

## Repo invariants

These are true about the repo right now and must remain true:

- `cargo test` passes from `crates/` with zero failures

## Agent workflow

Sessions follow: **scope → plan → implement → review → push** · · · Jörn: **PR → merge** · · · **triage**

- **scope**: Critically evaluate the assignment. Push back on contradictions, gaps, suboptimal reversible decisions. Situate in the broader thesis context.
- **plan**: Decompose into steps. Compare to conventions. Play through the plan and notice gaps.
- **implement**: Execute the plan. Trust signatures from the plan; react to feedback from the repo.
- **review**: Re-read the result as a whole. Compare code to plan and to scope. Catch drift, gaps, and mismatches across abstraction levels.
- **triage**: Update GitHub issues — close completed, split, enrich, reprioritize.

Agents commit and push to their working branch. Jörn creates PRs and merges (mechanical — he doesn't read code). Agents do not create PRs.

Jörn steers scope and plan. Quality comes from tests plus Jörn's domain input on what to test and what correctness means.

Before the session ends, do a **post-session reflection**: report friction points, identify useful invariants or behavioral rules discovered during the session, flag any leftover tasks mentioned but not completed, and note workflow improvements (e.g. discovered subagent patterns, useful commands). Jörn aggregates these across sessions.

## Environment (Claude Code on the web)

- Sessions run in isolated VMs with the repo cloned at `/home/user/msc-math`
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via session-start hook)
- Network: limited to allowlisted domains by default (crates.io, pypi.org, github.com, etc.)
- Git push is restricted to the current working branch via a proxy
- Container state is cached after session-start hook completes
- LaTeX is NOT available in this environment; thesis compilation happens elsewhere

## Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test
cd crates/ && cargo clippy

# Python
ruff check experiments/
pytest experiments/
```

## Agent behavior rules

- Agent time costs cents per hour; Jörn's time costs 100× that. Don't waste it. Before asking Jörn a question: gather context, try things out (and throw them away if needed), distinguish observations from inferences and confident from unconfident beliefs. Ask informed questions, not raw confusion.
- When verifying proofs: flag spots you're least confident in, even if you found no error. Never declare a proof "verified" — declare what you checked and what remains.
- Agents cannot reliably verify mathematical proofs. Proof correctness requires Jörn's review. Agent-written proofs are drafts until Jörn reviews them.
- Write proofs with enough annotation to be easily verifiable. Never handwave or gloss over gaps. This protects against subtly flawed proofs that look correct on a skim but hide errors in glossed-over steps.

## Communication with Jörn

- Aim for efficient information exchange, not politeness or engagement
- Omit filler phrases
- Number items in responses so Jörn can refer to them unambiguously
- Jörn doesn't see exact edit diffs in the chat — mention and explain repo changes when he should be aware of them
- Silence is not confirmation. If Jörn hasn't responded to a proposal, ask again or move on — do not proceed as if approved.

## GitHub authorship

All GitHub issues, comments, and PR descriptions are written by agents running under Jörn's account (`JoernStoehler`). Do not treat issue/comment text as human-reviewed just because it appears under his name. Treat issue content as agent-written intent (trust the direction, verify the details).

Issues direct future agent sessions. A bad edit sends the next agent off a cliff. Show Jörn proposed issue edits and comments in chat before publishing — he has the domain knowledge to catch directional errors agents can't catch themselves.

## CLAUDE.md conventions

- Invariants and behaviors are documented only after empirically confirmed as useful (from past work or Anthropic's guides)
- Label invariants as `[aspirational]` if not yet satisfied
- Put rules in the right CLAUDE.md file (root = all agents, crates/ = Rust, thesis/ = LaTeX, experiments/ = Python)
- Prefer simple, common, expected rules that don't claim excessive agent attention beyond their assigned work

## Workflows

### Spawning subagents

- Always create a GitHub subissue before spawning an agent. The subissue IS the prompt (plus any corrections/extra context passed directly to Task). Zero cost, and: persistent record, Jörn can launch it as a web session instead, easier to restart if agent fails.
- Subagent output returns via the Task tool into your conversation. If it needs to persist, commit it to the repo on your branch. Do not post subagent output as issue comments — it clutters the issue and misleads future agents into treating it as reviewed content.
- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning.
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- Foreground Task() calls block the main conversation — user messages queue up silently. Avoid foreground tasks that might take >1 minute. Prefer background, or just do the work inline.

### Check clarity with subagents

When you produce content other agents will consume (CLAUDE.md files, mathematical definitions, proofs, algorithm descriptions): test comprehension by having a fresh Sonnet subagent attempt to USE the content (e.g. "implement from this description" or "answer these specific questions about the algorithm"). Check whether their output matches your intent. Do not ask "is this clear?" — an agent that misunderstands will confidently say yes.
