---
paths:
  - "crates/**/*.rs"
---

# Rust Mathematical Documentation

## Math-Code Correspondence

Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence, not just "inspired by."

## Verification Criteria

- Doc comment formulas must match code's actual computation (not aspirational, not approximate)
- Invariants stated in doc comments must be enforced by types/constructors/assert!/debug_assert!
- Properties stated in doc comments must have corresponding tests

## Documentation Quality

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

## Cross-References to Thesis

Format: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` — matching LaTeX `\label{}` name exactly.

Rules:
1. Always include a one-line English description of what the referenced result says
2. Never duplicate proofs inline — comment says *what*, thesis says *why*
3. Never use rendered numbers like "Lemma 3.2" — use the label
4. Verification: grep `crates/src/` for occurrences, find the `.tex` `\label{...}`, check match

## Magic Numbers

Empirically chosen constants must have their rationale documented in code comments on the constant definition itself. Include: why that value, what data point motivated it, known limitations, and what must be re-validated if changed.

## Performance Claims

Never state performance without benchmark. "~1ms" is a claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured.

## The Core Rule (for Rust doc comments)

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with a TODO comment.
