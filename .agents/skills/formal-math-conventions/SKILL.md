---
name: formal-math-conventions
description: Developer-facing formal mathematics conventions for `formal/**/*.tex` and Rust/math correspondence. Use when editing or reviewing formal proofs, definitions, labels, formal build files, or Rust code/comments that cite `[lem:...]`, `[thm:...]`, `[def:...]`, or other formal labels.
---

# Formal Math Conventions

## Scope

Use `formal/` as the developer-facing mathematical source for code and experiments. Thesis prose in `thesis/` follows `$thesis-tex-conventions` instead.

Current layout:
- `formal/main.tex`: full formal build.
- `formal/preamble.tex`: shared theorem environments, packages, and notation helpers.
- `formal/bibliography.bib`: bibliography for formal builds.
- `formal/*.tex`: content files named by the formal object, theorem cluster, or
  proof cluster they define, not by the crate or experiment that consumes them.

## Before Editing

1. Identify the formal label, theorem cluster, or proof object the code or
   experiment depends on.
2. Grep for the label or nearby labels, then read the matching root-level
   `formal/*.tex` file.
3. For Rust-linked work, also load `$rust-conventions` and grep for the cited labels:

```bash
rg -n "\[(lem|thm|def|prop|cor|rem|eq):" crates experiments
rg -n '\\label\{(lem|thm|def|prop|cor|rem|eq):' formal
```

## File Structure

- Content files under `formal/` do not declare `\documentclass`.
- Shared notation and environments belong in `formal/preamble.tex`.
- Keep experiment motivation and empirical interpretation outside formal math unless the statement is a formal definition, lemma, theorem, proposition, proof, or calculation.
- Use bare graphic filenames in formal content only when the build context sets the graphics path.

## Labels And Cross-References

- Label format: `\label{<type>:<name>}` where `<type>` is one of `lem`, `thm`, `def`, `alg`, `cor`, `rem`, `prop`, `eq`, `fact`, `sec`, `tab`, or `fig`.
- Labels are unique across all `formal/**/*.tex` files.
- Rust code cites labels with `[lem:label]`, `[thm:label]`, `[def:label]`, and similar bracketed forms. Do not use rendered theorem numbers in code comments.
- Rust citations follow the `$rust-conventions` proof-burden rule: cite the
  formal label when code correctness depends on the result; do not force labels
  onto pure orchestration or obvious plumbing.
- If code needs a label that does not exist, add a TODO in code rather than inventing a false reference.

## Proof Standards

- Every lemma, theorem, proposition, and corollary has a proof unless Jörn explicitly marks the statement as a deferred conjectural placeholder.
- Mark uncertainty directly above the affected statement or proof:
  - `% [TODO: JÖRN - ...]` for a point requiring Jörn's judgment.
  - `% [GAP - ...]` for an above-background-risk gap.
- Do not duplicate proofs in Rust doc comments. Code comments should cite formal
  labels and summarize what the cited result gives.

## Build Checks

Use the smallest build that covers the changed files:

```bash
cd formal/ && latexmk
```

After building, use `.aux` files to locate rendered labels when reporting to Jörn:

```bash
rg 'lem:label-name' formal/**/*.aux
```
