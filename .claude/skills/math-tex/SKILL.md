---
name: math-tex
description: Conventions for math.tex files — LaTeX documents containing lemma statements, proofs, and derivations. Crate math lives in crates/src/<module>/math.tex, compiled via crates/src/math.tex. Experiment math lives in experiments/<name>/math.tex. Load when reading or writing math.tex files, or when Rust/experiment code references mathematical results.
---

# math.tex Conventions

math.tex files contain lemma statements, proofs, definitions, and derivations. They are the single source of mathematical truth for the code they sit next to.

## Locations and build

**Crate modules:** `crates/src/<module>/math.tex` (e.g. `geom/math.tex`, `kkt/math.tex`, `algorithms/math.tex`). These are `\input`'d by `crates/src/math.tex`, which compiles to a single PDF.

- Build: `cd crates/src && pdflatex math.tex`
- Shared preamble: `crates/src/math-preamble.tex` (packages, theorem environments)
- Per-module files are pure content — no `\documentclass` or `\begin{document}`. New packages or theorem environments go in `math-preamble.tex`.
- Cross-references between modules work (e.g. `algorithms/math.tex` can `\ref{lem:kkt}` from `kkt/math.tex`).

**Experiments:** `experiments/<name>/math.tex` — these are fragments `\input`'d into `thesis/experiments.tex`. No `\documentclass` or `\begin{document}`. They contain narrative, tables, figures, and subsections in addition to lemmas/proofs — appropriate for their role as thesis content.

## Purpose

| math.tex | thesis .tex |
|----------|-------------|
| Development-time proofs for code correctness | Final narrative for readers |
| Audience: Jörn (review) + agents (required reading) | Audience: examiner, motivated student |
| Written during development, alongside code | Written during final assembly (~last week) |
| Flat list of definitions/lemmas/proofs | Chapters with story arc and flow |
| Referenced by .rs doc comments via `[lem:label]` | References nothing (final output) |

The thesis draws from math.tex files and experiment logbooks during final assembly. Code and math.tex files never reference `thesis/`.

## What goes in math.tex

- Lemma and theorem statements with `\label{}`s
- Proofs
- Definitions introduced or used by the colocated code
- Formal derivations (e.g. gradient formulas, error bounds)
- Formal verification of claims made in experiment logbooks

## What does NOT go in math.tex

- Prose motivation, interpretation, discussion (→ logbook.md for experiments, doc comments for crates)
- Code documentation (→ .rs doc comments)
- Thesis narrative (→ thesis .tex, during final assembly)

## Label conventions

Labels use the format `\label{<type>:<name>}` where `<type>` is one of:
- Mathematical: `lem`, `thm`, `def`, `alg`, `cor`, `rem`, `prop`
- Equations: `eq` (for individual equations within larger structures)
- Computational facts: `fact` (numerical/literature citations, e.g. Golub & Van Loan)
- Experiment narrative: `sec`, `tab`, `fig` (subsections, tables, figures in experiment math.tex)
- Aliases: `\phantomsection\label{...}` for cross-reference targets without their own environment

Labels must be unique across all math.tex files in the repo (since the combined `crates/src/math.tex` and thesis may `\input` multiple files).

## Notation conventions

- KKT system uses the **symmetric** matrix form: `[H, A, 1; A^T, 0, 0; 1^T, 0, 0]` where A collects the dual vertices for facets in the support
- Dual-vertex parameterization: `K = {x : a_i^T x ≤ 1}`, Reeb vector `R_i = 2 J_0 a_i`
- Lagrange multipliers: μ (closure), ξ (normalization)
- β ∈ R^S (facet-indexed): β_i is the weight for facet i ∈ S, accessed via β_{σ(i)} for position i in ordering σ

## Agent rules

- **Required reading:** Read the module's math.tex before editing .rs files in that module.
- **Never invent labels.** If a lemma isn't written yet, add `// TODO: add [lem:...] to math.tex` in the .rs file.
- **Cross-references from .rs:** Use `[lem:label]` format with a one-line English description. See `rust-conventions` skill for details.
- **Jörn verifies math.** Agent-written proofs are drafts. Mark unverified content with `% [TODO: JÖRN -` or `% [GAP -`.
- **Every lemma must have a proof.** A statement-only stub (no `\begin{proof}...\end{proof}`) means the code's correctness is unverified. Flag statement-only entries as incomplete.
- **Every non-trivial code function must have a math.tex entry.** If a function in the module's .rs files implements mathematical logic and has no corresponding lemma/proposition in math.tex, flag it. The code→math.tex mapping is the mechanism that enforces the core rule for algorithmic correctness.

## Format conventions

Same comment conventions as thesis .tex files (see `tex-format` skill for `% Jörn:`, `% QC:`, `% [TODO:`, `% [GAP:` markers). Same proof structure (Assumptions -> Claim -> Overview -> Steps -> Conclusion). Same core rule: never write a factual claim without verifying it.

## The Core Rule

Never write a factual claim without verifying it against evidence in the same session. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`.
