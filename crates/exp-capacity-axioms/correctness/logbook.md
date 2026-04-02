# Correctness Verification: Logbook

## Motivation

Before using computed capacities for scientific conclusions, we need high confidence that the implementation is correct. Standard unit tests are insufficient -- we need to verify that the code satisfies the mathematical axioms that define EHZ capacity (conformality, symplectic invariance, monotonicity, continuity) and agrees with published literature values.

## Status

**Complete.** All 6 test propositions pass. Jörn approved the thesis writeup (2026-02-16).

## How to run

```bash
cd crates/exp-capacity-axioms/correctness/
cargo run --bin axioms-correctness --release   # Generates correctness.jsonl
cargo test --bin axioms-correctness --release   # Verifies all 6 properties
```

If tests fail after algorithm changes, investigate before merging. Failure triage: (1) implementation bug, (2) test assumption violated, (3) numeric precision issue.

Different seeds may expose edge cases not covered by seed 42.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: dataset generator + test harness (6 property tests) |
| `math.tex` | Formal writeup: algorithm variants, test propositions, literature table |
| `correctness.jsonl` | Dataset: 47 polytopes, 71 capacity values across 3 algorithms |

## Design

### Strategy

Generate a curated dataset where we know what the correct answers MUST be, then verify algorithm agreement, literature values, and mathematical axioms.

### Dataset structure

47 polytopes organized into 5 groups:

| Group | Count | Description | Purpose |
|-------|-------|-------------|---------|
| Base | 10 | 5 random generic + 5 random Lagrangian products | Foundation for tests 1, 3-6 |
| Literature | 7 | Known polytopes with published capacity values | Test 2 |
| Scaled | 10 | Base polytopes scaled by random alpha in [0.5, 2.0] | Test 3 (conformality) |
| Transformed | 10 | Base polytopes transformed by random M in Sp(4) | Test 4 (symplectic invariance) |
| Perturbed | 10 | Base polytopes with 1% height perturbation | Test 5 (continuity) |

### Capacity computations

71 capacity values across 3 algorithm variants (47 pruned + 10 unpruned + 14 billiard). Tests 3-5 reuse the 10 base polytopes (creating derived versions), so no redundant computation.

### The 6 tests

1. **Direct comparison**: Pruned vs unpruned on 10 base polytopes; pruned vs billiard on 5 Lagrangian products. Each test asserts an exact expected billiard count (5, 4, 5 respectively) to prevent silent vacuous passes.
2. **Literature agreement**: 7 known polytopes (simplex, hypercube, HK-O pentagon, 4 products) against published values. Billiard computed for 4 of the 7 (hypercube, pentagon, lag triangle x triangle, lag triangle x square).
3. **Conformality**: c(alpha*K) = alpha^2 * c(K) for 10 scaled polytopes.
4. **Symplectic invariance**: c(M*K) = c(K) for 10 transformed polytopes (M via Cayley transform).
5. **Continuity**: 1% height perturbation produces < 10% relative capacity change.
6. **Monotonicity**: For all pairs (K1, K2) with non-trivial containment (alpha > 0.1), verify alpha^2 * c(K1) <= c(K2). Asserts >= 20 pairs tested.

## Findings

1. All 6 tests pass with tolerance 1e-6 relative error.
2. 10 base polytopes: pruned = unpruned = billiard (on shared domain).
3. 7 literature polytopes: computed = published.
4. 10 scaled: c(alpha*K) = alpha^2 * c(K) confirmed.
5. 10 transformed: c(M*K) = c(K) confirmed.
6. 10 perturbed: small delta_h produces small delta_c.
7. 20+ monotonicity pairs verified.

## Known limitations

- Tests verify properties but don't prove exhaustiveness of the orbit search. Literature agreement provides empirical evidence that pruning doesn't discard optimal orbits.
- Fixed seed (42) for reproducibility.
- Tolerance 1e-6 is empirically chosen; tighter tolerance may fail due to floating-point accumulation.
- Runtime: ~5.3s for dataset generation, ~9.7s for tests (reads dataset 6 times).

## Open questions

1. **math.tex says "facet normals" but code perturbs heights.** Test 5 (continuity) applies a 1% height perturbation (`h * (1 + 0.01 * uniform)`) — confirmed in run.rs. But math.tex (Jörn-approved 2026-02-16) describes it as "a small random perturbation of the facet normals." One source is wrong. Needs Jörn.

## Related experiments

- **ablation**: Detailed analysis of pruning correctness (agreement across 4 variants on 54 polytopes).
- **benchmark**: Validates pruned vs unpruned and pruned vs billiard agreement on a different dataset.
