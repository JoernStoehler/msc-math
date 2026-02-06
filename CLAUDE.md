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

Sessions follow: **scope → plan → implement → review → PR → [compare →] merge → triage**

- **scope**: Critically evaluate the assignment. Push back on contradictions, gaps, suboptimal reversible decisions. Situate in the broader thesis context.
- **plan**: Decompose into steps. Compare to conventions. Play through the plan and notice gaps.
- **implement**: Execute the plan. Trust signatures from the plan; react to feedback from the repo.
- **review**: Compare code to plan and to scope. Look for mismatches across abstraction levels.
- **triage**: Update GitHub issues — close completed, split, enrich, reprioritize.

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

- Attempt before escalating to Jörn. If you fail, present what you learned.
- Keep files < 500 lines. Split proactively.
- When verifying proofs: flag spots you're least confident in, even if you found no error. Never declare a proof "verified" — declare what you checked and what remains.
- Agents cannot reliably verify mathematical proofs. Proof correctness requires Jörn's review. Agent-written proofs are drafts until Jörn reviews them.
- Write proofs with enough annotation to be easily verifiable. Never handwave or gloss over gaps. This protects against subtly flawed proofs that look correct on a skim but hide errors in glossed-over steps.
