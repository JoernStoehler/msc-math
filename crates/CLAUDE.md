# Rust crate conventions

## Coding conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Prefer iterator chains over `for` loops. Minimize mutable state. Use `map`, `filter`, `flat_map` for transformations.
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

## Test suites

Property tests load from a cached JSON fixture so they run fast (<1s) while still verifying mathematical properties. Non-default suites accept staleness risk in exchange for CPU savings — agents decide which to run based on what code they changed.

| Suite | Command | When to run | Time (2026-02-12) |
|-------|---------|-------------|-------------------|
| **Default** | `cargo test --lib` | Every iteration | ~54s wall, ~98s CPU |
| Regenerate capacity fixture | `cargo test -p hk2017 regenerate_test_dataset -- --ignored` | After changes to `ehz_capacity()` | ~2-3 min |
| Pruned vs unpruned agreement | `cargo test -p hk2017 pruned_matches_unpruned -- --ignored` | After changes to adjacency pruning | ~30s |
| Boundedness cross-check | `cargo test -p geom -- --ignored` | Monitoring, or after qhull/boundedness changes | ~3s |
| Expensive capacity (pentagon, crosspolytope) | `cargo test -p hk2017 pentagon_capacity -- --ignored` | Rare, specific investigations | 2-5 min |
| All ignored tests | `cargo test -- --ignored` | Full validation | ~10 min |

Target: default suite <3 min single-threaded. Times measured 2026-02-12 — may drift as codebase grows.

**Fixture location:** `hk2017/tests/fixtures/capacity_dataset.json` (committed, 27 polytopes with precomputed capacities).

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
- [ ] Working tree clean (no uncommitted changes)
