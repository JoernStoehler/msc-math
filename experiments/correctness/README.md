# Correctness Verification Experiment

Establish confidence in the HK2017 capacity implementation by verifying mathematical properties on a curated dataset.

## Status
Complete

## Files

| File | Purpose |
|------|---------|
| `correctness.rs` | Rust binary: dataset generator + tests |
| `correctness.jsonl` | Dataset (47 polytopes, 71 capacity values) |
| `correctness.tex` | Thesis writeup |

## Design

### Motivation

Before using computed capacities for scientific conclusions, we need high confidence that the implementation is correct. Standard unit tests are insufficient—we need to verify that the code satisfies the mathematical axioms that define EHZ capacity.

### Strategy

Generate a curated dataset where we know what the correct answers MUST be, then verify:

1. **Algorithm agreement:** Pruned vs unpruned HK2017, and HK2017 vs billiard on their shared domain
2. **Literature values:** Known polytopes with published capacity values
3. **Mathematical axioms:** Conformality, symplectic invariance, monotonicity, continuity

### Dataset Structure

47 polytopes organized into 5 groups:

| Group | Count | Description | Purpose |
|-------|-------|-------------|---------|
| Base | 10 | 5 random generic + 5 random Lagrangian products | Foundation for tests 1, 3-6 |
| Literature | 7 | Known polytopes with published capacity values | Test 2 (literature agreement) |
| Scaled | 10 | Base polytopes scaled by random α ∈ [0.5, 2.0] | Test 3 (conformality) |
| Transformed | 10 | Base polytopes transformed by random M ∈ Sp(4) | Test 4 (symplectic invariance) |
| Perturbed | 10 | Base polytopes with 1% height perturbation | Test 5 (continuity) |

**Key insight:** Tests 3-5 REUSE the same 10 base polytopes, creating derived versions. This allows us to test properties like c(αK) = α²c(K) by comparing the base and scaled versions.

### Capacity Computations

71 capacity values computed across 3 algorithm variants:

| Algorithm | Base | Literature | Scaled | Transformed | Perturbed | Total |
|-----------|------|------------|--------|-------------|-----------|-------|
| Pruned | 10 | 7 | 10 | 10 | 10 | **47** |
| Unpruned | 10 | — | — | — | — | **10** |
| Billiard | 5 | 4 | 5 | — | — | **14** |

**Total: 71 capacity computations** (no redundancy—each value is needed by at least one test).

Billiard is only computed for Lagrangian products:
- 5 of 10 base polytopes (the Lagrangian products)
- 4 of 7 literature polytopes (hypercube, pentagon, lag △×△, lag △×□)
- 5 of 10 scaled polytopes (those derived from Lagrangian base polytopes)

## Tests

All 6 tests read from `experiments/correctness/correctness.jsonl` (dataset must be regenerated if algorithm changes).

### Test 1: Direct Comparison

**Property:** All algorithm variants agree on their shared domain.

**Verification:**
- Pruned vs unpruned: all 10 base polytopes
- Pruned vs billiard: 5 Lagrangian products among base polytopes
- Asserts exactly 5 billiard values exist (prevents silent failures)

### Test 2: Literature Agreement

**Property:** Computed values match published capacity values from literature.

**Verification:**
- 7 known polytopes (simplex, hypercube, pentagon, 4 products)
- Pruned vs published value
- Billiard vs published value (for 4 Lagrangian products)
- Asserts exactly 4 billiard values exist

### Test 3: Conformality

**Property:** c(αK) = α²c(K) for scaling factor α.

**Verification:**
- 10 scaled polytopes (each with random α ∈ [0.5, 2.0])
- Compare c(αK) against α²c(base)
- Also verify billiard satisfies conformality (5 Lagrangian products)
- Asserts exactly 5 billiard values exist

### Test 4: Symplectic Invariance

**Property:** c(MK) = c(K) for M ∈ Sp(4).

**Verification:**
- 10 transformed polytopes (each with random M ∈ Sp(4))
- M generated via Cayley transform: M = (I - A)(I + A)⁻¹ where A ∈ sp(4)
- Compare c(MK) against c(base)

### Test 5: Continuity

**Property:** Small perturbations produce small capacity changes.

**Verification:**
- 10 perturbed polytopes (1% random height perturbation)
- Assert relative capacity change < 10%

### Test 6: Monotonicity

**Property:** K₁ ⊂ K₂ ⇒ c(K₁) ≤ c(K₂).

**Verification:**
- All pairs (K₁, K₂) from the full 47-polytope dataset
- Find max α such that αK₁ ⊂ K₂ (via vertex containment)
- Verify α²c(K₁) ≤ c(K₂) (combining conformality + monotonicity)
- Only test pairs with non-trivial containment (α > 0.1)
- Asserts at least 20 pairs tested (prevents vacuous success)

## Runtime

- **Dataset generation:** ~5.3 seconds (47 polytopes, 71 capacity values)
- **Test execution:** ~9.7 seconds (6 tests, reads dataset 6 times)

## Key findings

All tests pass with tolerance 10⁻⁶ relative error:
- ✓ 10 base polytopes: pruned = unpruned = billiard
- ✓ 7 literature polytopes: computed = published
- ✓ 10 scaled: c(αK) = α²c(K)
- ✓ 10 transformed: c(MK) = c(K)
- ✓ 10 perturbed: small Δh → small Δc
- ✓ 20+ pairs: αK₁ ⊂ K₂ ⇒ α²c(K₁) ≤ c(K₂)

## Data Flow to Thesis

The thesis section (`correctness.tex`) includes:

1. **Table of algorithm variants** (Pruned, Unpruned, Billiard)
2. **Table of capacity computation counts** (47+10+14 = 71 total)
3. **Enumeration of 6 test propositions** with mathematical formulas
4. **Table of literature polytopes** with F, c_EHZ, vol, sys, and source

The writeup is mathematical, not implementation-focused. It describes WHAT properties are verified, not HOW the Rust code works.

## Known limitations

- **Test coverage:** Tests verify properties, but don't prove exhaustiveness of the orbit search. Literature agreement provides empirical evidence that pruning doesn't discard optimal orbits.
- **Random sampling:** Base polytopes use fixed seed (42) for reproducibility. Different seeds may expose edge cases.
- **Tolerance:** 10⁻⁶ relative error is chosen empirically. Tighter tolerance may fail due to floating-point accumulation.

## Run

```bash
cd experiments/
cargo run --bin correctness --release   # Generates correctness/correctness.jsonl
cargo test --bin correctness --release  # Verifies all 6 properties
```

If tests fail after algorithm changes, investigate before merging. Failures indicate either:
1. Bug in new code (fix it)
2. Test assumption violated (update test + document why)
3. Numerical precision issue (adjust tolerance + document)
