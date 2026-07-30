---
name: rust
description: Use for Rust work whose project-specific mathematical, numerical, observability, performance, or reusable-crate documentation contracts matter. Ordinary Rust style is left to the agent's judgment.
---

# Rust Project Contracts

Rust supports thesis computations and reusable geometry code. Choose local code
structure with ordinary Rust judgment; this skill owns only project-specific
contracts.

## Mathematics And Numerics

- Shape APIs around mathematical operations and experiment workflows. Keep
  exact, f64, experiment, and helper surfaces separate where their contracts
  differ. Exact arithmetic often serves as the executable reference for f64
  audits.
- Put context-dependent propositions on producer or consumer contracts rather
  than on data containers that do not establish them.
- Public mathematical and numerical APIs state input and output contracts.
  Distinguish conditions validated locally, assumed after a named validation
  boundary, valid mathematical non-success, and theorem-backed guarantees.
- Use explicit outcomes when callers must distinguish mathematical cases.
  State whether f64 values are approximations, bounded results, indeterminate
  results, or heuristic guesses.
- Cite formal labels, proof notes, or API targets where code/math
  correspondence is not evident. Proof-sized reasoning belongs in `formal/` or
  an owner-local note, with code naming the relevant proposition or source.
- For a nontrivial specification-to-code path, keep a direct/reference
  implementation or another semantic witness when practical, and review the
  implementation against the mathematical contract rather than relying only on
  compilation or surface tests.
- When exploring numerical error guarantees, make candidate definitions and
  assumptions explicit enough to compare them, and use exact/reference or
  adversarial checks to distinguish a valid bound from a merely plausible one.

## Runtime Evidence

- Measure or profile before investing in performance tradeoffs. Optimize the
  measured hotspot and check whether the change mattered.
- Use ordinary `tracing` spans and events when production observability is
  useful. Keep tracing opt-in, ignorable, and out of return values and stdout
  data paths such as JSONL.

## Reusable Crate Documentation

Use a consumer `README.md` and maintainer `DEVELOPMENT.md` once a reusable
crate's API or architecture decisions are nontrivial. README should support
ordinary use without opening `src/`; DEVELOPMENT should preserve current scope,
API rationale, edit locations, important rejected/deferred approaches, and the
meaning of verification witnesses. Before substantively reviewing or materially
updating a reusable crate's README or DEVELOPMENT documentation, read
`references/crate-documentation-review.md`. Ordinary implementation work need
not load it.
