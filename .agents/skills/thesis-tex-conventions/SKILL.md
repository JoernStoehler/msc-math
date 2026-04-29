---
name: thesis-tex-conventions
description: Thesis LaTeX conventions for `thesis/**/*.tex`, including self-contained thesis prose, approval markers, theorem environments, labels, bibliography, figures, tables, and build checks. Use before editing or reviewing thesis sources.
---

# Thesis LaTeX Format

## Comment markers

- `% Jörn: <level> approved — <scope>` — review status (structure < math < text). One per scope. Agent edits within scope MUST delete the marker.
- `% [TODO: JÖRN - ...]` — needs Jörn's verification
- `% [GAP - <what's uncertain>]` — above-ambient-risk spot needing attention

## File headers

Every .tex file starts with a `%` block:
1. Identity: `% filename.tex — \input'd from parent.tex`
2. Sources: where the content comes from
3. Structure: outline of sections

No review status in headers — use `% Jörn:` markers in the body.

## Mathematical Environments

Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof`, `\remark`, `\example`.
For mathematical content, keep the statement and setup inside the relevant
environment except for minimal connective text.
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
