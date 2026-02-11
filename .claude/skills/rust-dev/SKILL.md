---
name: rust-dev
description: Use when writing or modifying Rust code in crates/. Contains coding conventions, test file naming, mathematical documentation rules, and testing philosophy.
---

# Rust crate conventions

## Coding conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Functional programming style
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing

## Mathematical documentation

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

## Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

## Property-based testing

Use proptest for universal quantification: "∀ λ > 0: vol(λK) = λ⁴·vol(K)" → proptest. Not for single examples.

## Performance claims require measurement

Never state performance without benchmark. "~1ms" is claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured. Add benchmark if claim exists without measurement.

## Thesis constraints

Polytope4D: 5-16 facets typical. Research code, March 2026 deadline. Correctness > performance.

Don't suggest: Theoretical numerical analysis, O(n²) documentation when n ≤ 16, production features unlikely to matter.

Do suggest: Critical path tests, benchmarks for claims, robustness fixes (timeouts, limits).

## Commit checklist

Before final report:
- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Critical paths have tests
- [ ] Performance claims have benchmarks
- [ ] All changes committed
