# Test Coverage Matrix

## Overview

This document tracks test coverage across the crates, focusing on mathematical properties that ensure correctness.

## Coverage Matrix

| Component | Property | Test Type | Test Location | Status |
|-----------|----------|-----------|---------------|--------|
| **hk2017** |
| ehz_capacity | Correctness on simplex | Unit | lib_test.rs:58 | ✓ |
| ehz_capacity | Correctness on hypercube | Unit | lib_test.rs:71 | ✓ |
| ehz_capacity | Correctness on triangle product | Unit | lib_test.rs:84 | ✓ |
| ehz_capacity_pruned | Matches unpruned (hypercube) | Unit | lib_test.rs:104 | ✓ |
| ehz_capacity_pruned | Matches unpruned (random) | Proptest | lib_test.rs (NEW) | ✓ |
| ehz_capacity | Scaling law c_EHZ(λK) = λ²·c_EHZ(K) | Proptest | lib_test.rs (NEW) | ✓ |
| solve_kkt | Two facets | Unit | lib_test.rs:129 | ✓ |
| solve_kkt | Four facets symplectic | Unit | lib_test.rs:157 | ✓ |
| solve_kkt | Rank-deficient system | Unit | lib_test.rs:198 | ✓ |
| solve_kkt | Degenerate system | Unit | lib_test.rs:224 | ✓ |
| combinations | Basic combinatorics | Unit | lib_test.rs:97 | ✓ |
| **geom/volume** |
| simplex_volume_5 | Correctness on standard simplex | Unit | volume_test.rs:6 | ✓ |
| volume | Hypercube | Unit | volume_test.rs:24 | ✓ |
| volume | Simplex polytope | Unit | volume_test.rs:46 | ✓ |
| volume | Crosspolytope | Unit | volume_test.rs:133 | ✓ |
| volume | Scaling law vol(λK) = λ⁴·vol(K) | Unit | volume_test.rs:90 | ✓ |
| volume | Scaling law (random λ) | Proptest | volume_test.rs:166 | ✓ |
| volume | Positivity | Unit | volume_test.rs:105 | ✓ |
| **datasets/validation** |
| Polytope4D::new | Duplicate halfspaces rejected | Unit | validation_test.rs:54 | ✓ |
| Polytope4D::new | Antiparallel normals accepted | Unit | validation_test.rs:68 | ✓ |
| check_bounded | Simplex | Unit | validation_test.rs:77 | ✓ |
| check_bounded | Hypercube | Unit | validation_test.rs:83 | ✓ |
| check_bounded | Unbounded rejected | Unit | validation_test.rs:89 | ✓ |
| Polytope4D::vertices | Simplex vertex count | Unit | validation_test.rs:104 | ✓ |
| Polytope4D::vertices | Hypercube vertex count | Unit | validation_test.rs:116 | ✓ |
| Polytope4D::vertices | Constraint satisfaction | Unit | validation_test.rs:128 | ✓ |
| check_irredundant | Simplex | Unit | validation_test.rs:146 | ✓ |
| check_irredundant | Hypercube | Unit | validation_test.rs:153 | ✓ |
| check_irredundant | Redundant facet detected | Unit | validation_test.rs:160 | ✓ |
| validate_polytope | Full pipeline (simplex) | Unit | validation_test.rs:195 | ✓ |
| validate_polytope | Full pipeline (hypercube) | Unit | validation_test.rs:202 | ✓ |
| cross_product_4d | Perpendicularity | Unit | validation_test.rs:211 | ✓ |
| affine_rank | Single point | Unit | validation_test.rs:226 | ✓ |
| affine_rank | Collinear | Unit | validation_test.rs:232 | ✓ |
| affine_rank | 3D | Unit | validation_test.rs:242 | ✓ |
| **datasets/random** |
| sample_random_polytope | Completeness | Proptest | random_test.rs (NEW) | ✓ |

## Gap Analysis

### P1 (Critical) — Addressed by this PR

1. **pruned=unpruned for random polytopes** ✓
   - Status: ADDED (proptest in lib_test.rs)
   - Risk: pruned algorithm may compute wrong capacity on edge cases
   - Test: Generate random polytopes, verify pruned and unpruned match

2. **Capacity scaling law c_EHZ(λK) = λ²·c_EHZ(K)** ✓
   - Status: ADDED (proptest in lib_test.rs)
   - Risk: Capacity computation may fail to scale correctly
   - Test: Random scale factors, verify c_EHZ(λK) / λ² ≈ c_EHZ(K)

3. **Random polytope validation completeness** ✓
   - Status: ADDED (proptest in random_test.rs)
   - Risk: sample_random_polytope may accept invalid polytopes
   - Test: Every accepted polytope passes all validation checks

### P2 (Important) — Future work

4. **Volume-capacity relationship**
   - Status: NOT TESTED
   - Risk: No cross-crate consistency check
   - Test: For known polytopes, verify sys = c_EHZ² / (2·vol) matches expected values
   - Blocker: Needs sys computation and expected values

5. **Adjacency matrix correctness**
   - Status: ONLY INDIRECTLY TESTED (via pruned=unpruned)
   - Risk: build_adjacency_matrix may misidentify adjacent facets
   - Test: Known polytopes with explicit adjacency structure
   - Note: Current indirect coverage is acceptable, but explicit test would be better

6. **Vertex enumeration exhaustiveness**
   - Status: ONLY INDIRECTLY TESTED (via irredundancy checks)
   - Risk: Qhull may miss vertices on degenerate polytopes
   - Test: Known polytopes with explicit vertex lists
   - Note: Current indirect coverage is acceptable for random polytopes

### P3 (Nice-to-have) — Future work

7. **Billiard algorithm (when implemented)**
   - Test: Matches hk2017 on Lagrangian products

8. **Tube algorithm (when implemented)**
   - Test: Matches hk2017 on polytopes without Lagrangian 2-faces

9. **Edge cases in solve_kkt**
   - Nearly-degenerate systems (tested, but could expand)
   - High-dimensional kernels

10. **Numerical stability**
    - Large/small height ratios
    - Nearly-parallel normals
    - Extreme scaling factors

## Test Additions (This PR)

### 1. pruned_matches_unpruned_random (hk2017/lib_test.rs)

**Property**: For any valid polytope, ehz_capacity and ehz_capacity_pruned return the same capacity.

**Rationale**: Pruning by adjacency is an optimization that should not change the result. This is Corollary 5.3 in the thesis.

**Implementation**:
- Generate random polytopes with 5-8 facets (manageable search space)
- Seeds 0-99 for reproducibility
- Compare capacities with tolerance 1e-6

### 2. capacity_scales_quadratically (hk2017/lib_test.rs)

**Property**: c_EHZ(λK) = λ²·c_EHZ(K)

**Rationale**: The capacity is defined as minimum action, which scales quadratically with linear scaling of the polytope. This follows from the definition of the action functional and is a fundamental property.

**Implementation**:
- Use hypercube (fast, deterministic)
- Test scale factors 0.5, 2.0, 3.0
- Verify c_EHZ(λK) / λ² ≈ c_EHZ(K) with relative error < 1e-4

### 3. random_polytopes_pass_validation (datasets/random_test.rs)

**Property**: Every polytope accepted by sample_random_polytope passes full validation.

**Rationale**: Validates that the rejection sampling loop doesn't have bugs that let invalid polytopes through.

**Implementation**:
- Generate random parameters (facet_count, h_min, h_max, seed)
- Call sample_random_polytope
- If Ok, verify validate_polytope also returns Ok
- If Err, accept (rejection sampling is allowed to reject)

## Running Tests

```bash
cd /workspaces/worktrees/kai-demo-experiments/crates
cargo test                    # all tests
cargo test --test proptest    # proptests only
cargo test hk2017            # hk2017 only
```

## Coverage Statistics

- **Total properties tested**: 35
- **Unit tests**: 32
- **Proptests**: 3 (NEW)
- **Cross-crate tests**: 0 (P2 gap)

## Notes

- Proptests use deterministic RNG (ChaCha8Rng) for reproducibility
- Random polytope generation is relatively slow (rejection sampling with qhull)
- Test cases limited to 5-8 facets for hk2017 (exponential cost)
- Volume tests cover up to 16-facet polytopes (crosspolytope)
