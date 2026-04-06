---
paths:
  - "**/math.tex"
---

# math.tex Conventions

math.tex files are the single source of mathematical truth for colocated code.

## Locations and build

**Root:** `crates/main.tex` compiles ALL crate + experiment math into one PDF.
Build: `cd crates/ && latexmk` (produces `main.pdf`).
This is the authoritative build — cross-references between experiments and crate lemmas resolve here.

**Crate modules:** `crates/library/src/<module>/math.tex`, `\input`'d by both root `main.tex` and `crates/library/src/math.tex`.
Preamble: `crates/library/src/math-preamble.tex` (packages, environments). Per-module files are pure content — no `\documentclass`.

**Experiments:** `crates/exp-<group>/<subdir>/math.tex` — content files `\input`'d by root `main.tex`. No `\documentclass`. Use bare filenames for `\includegraphics` (e.g., `foo.png`, not `../crates/exp-<group>/<subdir>/foo.png`); the compile context sets `\graphicspath` per section.

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

## Agent rules

- Read math.tex before editing .rs files in the same module
- Never invent labels — use `// TODO: add [lem:...] to math.tex` in .rs
- Mark unverified content: `% [TODO: JÖRN - ...]` (needs Jörn's verification) or `% [GAP - <what's uncertain>]` (above-ambient-risk spot)
- Every non-trivial code function needs a corresponding math.tex entry
