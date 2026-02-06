# Rust Crates

## Build and test

```bash
cd crates/
cargo build
cargo test
```

All tests must pass before committing.

## Crate dependency graph

```
geom2d ─────────────────────────────────┐
  └─> geom4d                            │
        ├─> hk2017    (+ geom2d)        │
        ├─> billiard  (+ geom2d)        │
        ├─> tube      (+ geom2d)        │
        └─> datasets  (+ hk2017, billiard, tube)
```

## Three capacity algorithms

| Crate     | Applies to                        | Cost                    |
|-----------|-----------------------------------|-------------------------|
| hk2017    | All polytopes                     | Exponential in #facets  |
| billiard  | Lagrangian products only          | Fast                    |
| tube      | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

## Conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`) — see testing philosophy below
- Single-purpose files, < 500 lines each
- Functional programming style
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing

## Mathematical documentation

- Definitions, lemmas, and proofs live in the Rust crates as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream and synced independently
- Quality bar: specific, correct, detailed, and clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing function bodies

## Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.
