---
name: math-tex
description: Conventions for math.tex files — standalone LaTeX documents containing lemma statements, proofs, and derivations. Found in crate module directories (crates/symplectic/src/kkt/math.tex) and experiment directories (experiments/<name>/math.tex). Load when reading or writing math.tex files, or when Rust/experiment code references mathematical results.
---

# math.tex Conventions

math.tex files are standalone LaTeX documents containing lemma statements, proofs, definitions, and derivations. They are the single source of mathematical truth for the code they sit next to.

## Locations

- **Crate modules:** `crates/symplectic/src/<module>/math.tex` (e.g. `geom/math.tex`, `kkt/math.tex`)
- **Experiments:** `experiments/<name>/math.tex`

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

## File conventions

- One math.tex per module/experiment directory. Split only if agents report the file is too large.
- Must compile standalone — Jörn reviews these as PDF.
- Uses standard LaTeX theorem environments (`\begin{lemma}`, `\begin{definition}`, `\begin{proof}`, etc.) with `\label{}`s.
- Living document: grows alongside the code/experiment. Not a polished thesis section.

## Label conventions

Labels use the format `\label{<type>:<name>}` where `<type>` is one of `lem`, `thm`, `def`, `alg`, `cor`, `rem`, `prop`.

Labels must be unique across all math.tex files in the repo (since the thesis may `\input` multiple math.tex files during final assembly).

## Agent rules

- **Required reading:** Read the module's math.tex before editing .rs files in that module.
- **Never invent labels.** If a lemma isn't written yet, add `// TODO: add [lem:...] to math.tex` in the .rs file.
- **Cross-references from .rs:** Use `[lem:label]` format with a one-line English description. See `rust-conventions` skill for details.
- **Jörn verifies math.** Agent-written proofs are drafts. Mark unverified content with `% [TODO: JÖRN -` or `% [GAP -`.

## Format conventions

Same comment conventions as thesis .tex files (see `tex-format` skill for `% Jörn:`, `% QC:`, `% [TODO:`, `% [GAP:` markers). Same proof structure (Assumptions -> Claim -> Overview -> Steps -> Conclusion). Same core rule: never write a factual claim without verifying it.

## The Core Rule

Never write a factual claim without verifying it against evidence in the same session. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`.
