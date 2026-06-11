# Numerics Content Notes

Status: section-local content companion for `thesis/numerics.tex`. Not source
truth.

Purpose: gather the high-level numerical reliability story and its boundaries.

Overruled by: `crates/`, `experiments/`, exact verification artifacts, formal
proofs, task files, and Jörn/Kai review.

Lifecycle: keep while the numerics section is being assembled. After the
section is stable, delete this file or reduce it to a short maintenance note.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Pacing

- Kai preference: numerics is interesting for about one high-level paragraph in
  the main text, not more.
- Detailed proofs and intermediate bounds belong in the appendix.
- Do not mix numerical-analysis language into symplectic definitions or exact
  algorithm proofs.
- Treat numerics as a support layer after the exact mathematical computation
  story.

## Content Inventory

- Exact arithmetic path: rational or algebraic data is used to implement
  mathematically meaningful helper operations slowly but without numerical
  error.
- These helpers are separated so later computations can reuse them instead of
  re-encoding the same mathematics.
- Floating-point fast path: the same mathematical algorithms are mapped to
  `f64` linear algebra where practical.
- Discontinuous predicates are treated as trinary `true`, `false`, or
  `indeterminate` decisions with error margins.
- `indeterminate` means the numerical evidence is not strong enough to decide
  the mathematical predicate; this differs from invalid input errors and
  unrecovered assertion failures.
- Logical use of indeterminate values: use cancellations such as
  `false and indeterminate = false`, and simplify searches only from decided
  values.
- Do not claim a relational abstract interpreter unless a retained proof adds
  it. Relations such as two individually indeterminate predicates whose
  disjunction is forced true are outside the current method.
- Include empirical error measurements and exact comparisons where the f64 path
  is used to rerun experiments.
- Include proven error bounds only at the strength needed by retained thesis
  claims.
