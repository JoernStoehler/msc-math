---
name: rust-conventions
description: Rust conventions for `library/**/*.rs` and `experiments/**/*.rs`, including coordinate order, mathematical invariants, formal label references, algorithm boundaries, tests, error handling, and performance claims. Use before editing or reviewing Rust code.
---

# Rust Conventions

## Coordinate convention

(q₁, q₂, p₁, p₂) — components [0,1] = q-space, [2,3] = p-space, [0,2] and [1,3] = symplectic planes. Defined in `library/src/geom/symplectic_form.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂).

## Math-code correspondence

Types, function signatures, and function bodies have 1:1 structural correspondence to mathematical definitions. Not "inspired by" — literal correspondence.

- Doc comment formulas must match the code's actual computation
- Invariants stated in doc comments are enforced by types/constructors/assert!
- Properties stated in doc comments have corresponding tests
- Types encode mathematical invariants, validated in `::new()`

## Cross-references to formal math

Format: `[lem:label]`, `[thm:label]`, `[def:label]` -- matching `\label{}` in `formal/**/*.tex`.

- Include a one-line English description of the referenced result
- Never duplicate proofs -- formal math is the single maintained source of truth
- Never invent labels -- use `// TODO: add [lem:...] to formal math` if the lemma isn't written
- In source code, never use rendered numbers like "Lemma 3.2" -- always use the label
- Every non-trivial code block must map to a formal statement

Read the matching formal file before editing non-trivial `.rs` files:
- `library/src/geom/**` -> `formal/library/geom.tex`
- `library/src/kkt/**` -> `formal/library/kkt.tex`
- `library/src/algorithms/**` -> `formal/library/algorithms.tex`
- `experiments/<topic>/<experiment>/**` -> `formal/<topic>/*.tex` when a formal file exists

Load `$formal-math-conventions` when editing a formal label, adding a new reference, or changing a mathematical algorithm.

## Algorithms

Three capacity algorithms: `hk2017` (general, exponential), `billiard` (Lagrangian products, fast), `tube` (no Lagrangian 2-faces). Where domains overlap, algorithms must agree on computed capacity.

No rayon inside algorithms — parallelism is at the dataset level (each polytope independently).

## Helper boundaries

Extract a helper or subfunction only when there is a clean boundary that makes
the surrounding code easier to understand or modify.

- Prefer inline code when the logic is tightly coupled to one call site and the
  extracted helper would force readers to jump away only to recover the same
  local context.
- Prefer a context-specialized helper over a generic one when there is only one
  caller and the specialized signature is simpler to read than a "future-proof"
  abstraction.
- Extract shared logic when multiple callers would otherwise duplicate the same
  mathematically meaningful stage or when one boundary lets later refactors
  change one side without re-reading the full caller body.
- When deciding whether to extract, optimize for reader/refactor cost, not for
  minimizing raw line count:
  - how often will a future agent need to understand the helper body?
  - how often will a future agent need to refactor both caller and callee
    together?
  - does the helper hide a real algorithm stage, or only move local glue into a
    second file/function?

Bad smell: two helpers that are only used once each, have nearly the same
shape, and force callers to reconstruct local invariants that were clearer
inline.

## Tests

Library tests should give fast, live feedback while editing Rust code. Prefer small deterministic examples, named known polytopes, exact invariants, and narrowly scoped regression cases.

Do not add broad generated datasets, expensive random sweeps, or cached validation fixtures to `library/` to make default tests pass quickly. Put slow mathematical validation, edge-case searches, and generated evidence datasets under `experiments/`, then keep only small smoke/regression checks in the crate.

## Magic numbers

Empirically chosen constants: document rationale, motivating data point, limitations, and what to re-validate if changed. All in a comment on the constant definition.

## Performance claims

Never state performance without an inline benchmark citation. "~1ms" is a claim. "1.5-2.0ms for F=5-16 (criterion bench 2026-03-23)" is measured.

## Error handling

Standard Rust error handling, plus:

- When math is violated, panic. Don't try to recover gracefully — the math needs to be fixed, not worked around.
- Don't use `Option<T>` in math code. `None` has no canonical mathematical meaning.
- In math code, use enums instead of errors or panics to classify cases (e.g. invertible vs singular, feasible vs infeasible). Each variant is a mathematical proposition.
- Callers of math code must match on all variants and handle each case locally. Don't propagate with `?`. If a case is proven or conjectured to not occur, `assert!` on it.

## Experiment binaries

Only stable, validated code lives in `library/`. Don't modify the library for experiment-specific behavior.

Within an experiment package (`experiments/<group>/`), shared helpers belong in `src/lib.rs` when multiple binaries need the same function. This avoids copy-paste duplication and lets improvements propagate. Per-binary helpers that only one experiment uses stay in that binary's `main.rs`.

Cargo binary entrypoints in `experiments/**` carry machine-readable crate docs with
`Input Artifacts:` and `Output Artifacts:`. Use repo-relative artifact paths when the
binary owns concrete files in the repo, and `None` when the binary only prints to
stdout or only operates on ad-hoc CLI paths outside the maintained artifact set.
