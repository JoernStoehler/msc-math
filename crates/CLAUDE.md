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
geom2d
  └─> geom4d
        ├─> hk2017
        ├─> billiard
        ├─> tube
        └─> datasets (also depends on hk2017, billiard, tube)
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
- Single-purpose files, < 500 lines each
- Functional programming style
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing

## Testing philosophy

- proptest generators approximate mathematical quantifiers: "∀ polytopes K", "∀ A ∈ Sp(4)", etc.
- Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity)
- Standard bug-finding tests per Rust best practices for correctness-critical code
