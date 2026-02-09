---
name: thesis
description: Use when working on files in thesis/. Contains LaTeX conventions, proof writing rules, and the quality bar for mathematical documentation.
---

# Thesis conventions

[aspirational] A complete master thesis PDF following arxiv best practices.

Currently: skeleton only (`main.tex` with title, author, and section stubs).

## LaTeX conventions

- Standard AMS theorem environments
- LaTeX compilation is NOT available in this environment; focus on source correctness

## Writing proofs

- Every proof must be detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly
- Agents cannot reliably verify mathematical proofs. Proof correctness requires Jörn's review. Agent-written proofs are drafts until Jörn reviews them.
