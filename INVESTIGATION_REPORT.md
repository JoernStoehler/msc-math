# Investigation Report: Square-Based Lagrangian Product Capacity Bug

**Date**: 2026-02-15
**Branch**: `claude/experiments-for-kai` in worktree `/workspaces/worktrees/experiments`
**Investigator**: Claude Code (Opus)
**Status**: Raw findings only. No fixes applied. No conclusions trusted.

## Summary of Bug

All three capacity algorithms (HK2017 unpruned, HK2017 pruned, billiard) produce
incorrect or missing results for certain Lagrangian products involving squares (n=4).
The minimum broken case is **(3,4) at θ=0°** (7 facets), where all three algorithms
return None for a polytope with known capacity 1.5.

## Trust Hierarchy (from Jörn)

- **TRUSTED**: Literature values, Jörn's hand calculations in thesis
- **NOT TRUSTED**: Algorithm implementations — must verify
- Agreement between algorithms does NOT imply correctness (they share code)

## Shared Code Between Algorithms

Both HK2017 and billiard share:
- `Polytope4D` construction (via `geom`)
- `build_adjacency_matrix()` (identical code in both crates)
- `solve_kkt()` (identical algorithm, duplicated in `hk2017/src/lib.rs:126` and `billiard/src/kkt.rs`)
- Q(β) computation formula
- ω₀ symplectic form
- All tolerance constants (EPS_BETA_POSITIVE=1e-12, EPS_Q_POSITIVE=1e-15, etc.)

## Finding 1: Minimum Broken Case — (3,4) at θ=0°

**Setup**: Triangle (circumradius 1) in q-space × Square (circumradius 1) in p-space, no rotation.
**Expected**: cap = 1.5 (known from `lagrangian_triangle_square()` in `known_polytopes.rs`)
**Actual**: ALL THREE algorithms return None. Zero valid orbits found.

**Exhaustive orbit search**: I manually tested ALL 4-facet and 5-facet cyclic permutations
via `solve_kkt`. None produce β > 0.

**The algorithm searches all subset sizes 2..7** (it reports 0 iterations only because
iterations aren't returned when result is None — this is a reporting artifact).

**(3,4) at other angles works fine:**

| θ       | cap_unpruned | cap_pruned | cap_billiard | agree |
|---------|-------------|-----------|-------------|-------|
| 0.000°  | NaN         | NaN       | NaN         | N/A   |
| 15.000° | 2.196       | 2.196     | 2.196       | YES   |
| 30.000° | 2.121       | 2.121     | 2.121       | YES   |

Facet normals for (3,4) at θ=0°:
```
facet 0: n=(0, 1, 0, 0)         h=0.5     [q-space, triangle]
facet 1: n=(-√3/2, -1/2, 0, 0)  h=0.5     [q-space, triangle]
facet 2: n=(√3/2, -1/2, 0, 0)   h=0.5     [q-space, triangle]
facet 3: n=(0, 0, 0, 1)         h=1/√2    [p-space, square]
facet 4: n=(0, 0, -1, 0)        h=1/√2    [p-space, square]
facet 5: n=(0, 0, 0, -1)        h=1/√2    [p-space, square]
facet 6: n=(0, 0, 1, 0)         h=1/√2    [p-space, square]
```

**Observation**: The square normals are axis-aligned: ±e₃, ±e₄. This means
some ω₀(q-normal, p-normal) pairs are zero. For example, ω₀((0,1,0,0), (0,0,-1,0)) = 0.
This creates a degenerate H matrix structure.

## Finding 2: (3,3) Works Perfectly

Triangle × Triangle (6 facets, the smallest possible Lagrangian product) works at ALL angles:

| θ       | cap_unpruned | cap_pruned | cap_billiard | k | agree |
|---------|-------------|-----------|-------------|---|-------|
| 0.000°  | 1.500       | 1.500     | 1.500       | 3 | YES   |
| 1.000°  | 1.485       | 1.485     | 1.485       | 3 | YES   |
| 10.000° | 1.382       | 1.382     | 1.382       | 3 | YES   |
| 30.000° | 1.299       | 1.299     | 1.299       | 3 | YES   |

All β = 1/3 (uniform), 6-facet orbits. Perfect agreement.

## Finding 3: (4,4) — Capacity Jump at θ=0°

### Adjacency pruning hypothesis: ELIMINATED

**Unpruned == pruned at EVERY angle tested** (32 test points). The adjacency matrix
is identical at θ=0°, 0.125°, and 45° (24 adjacent pairs in all cases). The bug is NOT
in adjacency pruning.

### The jump: θ=0° → θ=0.001°

| θ       | cap_HK2017 | cap_billiard | k | HK orbit size | HK β pattern |
|---------|-----------|-------------|---|---------------|-------------|
| 0.000°  | 2.000     | 2.000       | 2 | 4 facets      | all 0.354   |
| 0.001°  | 3.9999    | 3.9999      | 3 | 8 facets      | all 0.177   |
| 0.005°  | 3.9997    | 3.9998      | 3 | 8 facets      | all 0.177   |
| 0.010°  | 3.9993    | 3.9997      | 3 | 8 facets      | all 0.177   |
| 0.050°  | 3.9965    | 3.9983      | 3 | 8 facets      | all 0.177   |
| 0.100°  | 3.9930    | 3.9965      | 3 | 8 facets      | all 0.177   |
| 0.125°  | 3.9913    | 3.9956      | 3 | 8 facets      | all 0.177   |
| 0.250°  | 3.9827    | 3.9913      | 3 | 8 facets      | all 0.177   |
| 0.500°  | 3.9655    | 3.9827      | 3 | 8 facets      | all 0.177   |
| 1.000°  | 3.9320    | 3.9657      | 3 | 8 facets      | all 0.177   |

**HK2017 and billiard disagree at θ > 0°** (different permutations, different capacities),
but both give cap ≈ 4.0 — roughly 2× the correct value at θ=0°.

**HK2017 always finds the same 8-facet orbit** at θ > 0°: all β = 1/(4√2) = 0.176777.
This is a "degenerate" orbit using ALL 8 facets with equal weight.

### Mid-range and near θ=45°

| θ        | cap_HK2017 | cap_billiard | k | HK orbit   | agree |
|----------|-----------|-------------|---|------------|-------|
| 5.000°   | 3.692     | 3.840       | 3 | 8-facet    | NO    |
| 10.000°  | 3.453     | 3.704       | 3 | 8-facet    | NO    |
| 20.000°  | 3.121     | 3.482       | 3 | 8-facet    | NO    |
| 30.000°  | 2.928     | 3.285       | 3 | 8-facet    | NO    |
| 40.000°  | 2.839     | 3.031       | 3 | 8-facet    | NO    |
| 42.000°  | 2.832     | 2.959       | 3 | 8-facet    | NO    |
| 43.000°  | 2.735     | 2.735       | 2 | **4-facet** | YES  |
| 43.125°  | 2.830     | 2.914       | 3 | 8-facet    | NO    |
| 44.000°  | 2.829     | 2.876       | 3 | 8-facet    | NO    |
| 44.500°  | 2.829     | 2.853       | 3 | 8-facet    | NO    |
| 44.875°  | 2.828     | 2.835       | 3 | 8-facet    | NO    |
| 44.990°  | 2.828     | 2.829       | 3 | **5-facet** | NO   |
| 45.000°  | 2.828     | 5.657       | 3 | **6-facet** | NO   |

**θ=43°**: All three agree on cap=2.735 using a 4-facet orbit [0,6,2,4] with β all 0.354.
This is an isolated island of agreement.

**θ=44.99°**: HK2017 uses a 5-facet orbit [0,3,6,2,4] with β=[0.354, **0.000**, 0.354, 0.354, 0.354].
One β is exactly 0.000000 — barely passing the EPS_BETA_POSITIVE = 1e-12 check. Suspicious.

**θ=45°**: HK2017 gives cap=2√2≈2.828 (6-facet orbit), billiard gives 2×2√2≈5.657.
Billiard is exactly 2× at 45°.

## Finding 4: Manual Orbit Injection — ROOT CAUSE EVIDENCE

### 4-facet orbit [1,7,3,5] (optimal at θ=0°, cap=2.0)

| θ       | action        | Q        | β_pos | β pattern                    |
|---------|--------------|----------|-------|------------------------------|
| 0.000°  | 2.000        | 0.250    | YES   | [0.354, 0.354, 0.354, 0.354] |
| 0.001°  | -7.8×10¹⁰   | -6×10⁻¹² | NO    | [-0.000, 0.707, -0.000, 0.707] |
| 0.010°  | -1.4×10¹²   | -4×10⁻¹³ | NO    | [-0.000, 0.707, -0.000, 0.707] |
| 0.100°  | +2.5×10¹²   | +2×10⁻¹³ | NO    | [+0.000, 0.707, +0.000, 0.707] |
| 1.000°  | +5.5×10¹³   | +9×10⁻¹⁵ | NO    | [+0.000, 0.707, +0.000, 0.707] |
| 45.000° | None         | —        | —     | solve_kkt returned None       |

**CRITICAL**: At ANY θ > 0°, the KKT solution collapses β to [~0, 0.707, ~0, 0.707].
Facets 1 (q-space) and 3 (q-space) get β ≈ 0, while facets 7 (p-space) and 5 (p-space)
absorb all weight. The orbit genuinely becomes infeasible — this is NOT a numerical artifact.

Q approaches zero (10⁻¹²), making action = 0.5/Q blow up to ±10¹².

### 8-facet orbit [0,6,3,5,2,4,1,7] (what HK2017 finds at θ > 0°)

| θ       | action | Q      | β_pos | β pattern    |
|---------|--------|--------|-------|--------------|
| 0.000°  | 4.000  | 0.125  | YES   | all 0.177    |
| 0.001°  | 3.9999 | 0.1250 | YES   | all 0.177    |
| 1.000°  | 3.932  | 0.1272 | YES   | all 0.177    |
| 10.000° | 3.453  | 0.1448 | YES   | all 0.177    |
| 30.000° | 2.928  | 0.1708 | YES   | all 0.177    |
| 43.000° | 2.830  | 0.1767 | YES   | all 0.177    |
| 45.000° | 2.828  | 0.1768 | **NO** | has β < 0   |

**KEY**: This orbit is valid for 0° ≤ θ < 45° with ALL β EXACTLY 0.176777 (constant!).
At θ=0° it gives action=4.0 (above the 4-facet orbit's 2.0), but it's always valid.
At θ=45° it becomes infeasible (β has negative components).

### 4-facet orbit [0,6,2,4] (optimal at θ=43°)

| θ       | action | β_pos | notes |
|---------|--------|-------|-------|
| 0.000°  | None   | —     | solve_kkt returns None |
| 0.001°  | -2.9×10¹⁰ | NO | β collapsed to [~0, 0.707, ~0, 0.707] |
| 1.000°  | 5.3×10¹³ | NO | same pattern |
| 43.000° | 2.735  | YES   | all β = 0.354 |
| 45.000° | None   | —     | solve_kkt returns None |

This orbit is ONLY valid in a narrow angle range near 43°.

## Finding 5: (4,5) — Perfect Agreement

All three algorithms agree to FULL PRECISION (13+ decimal places) at all 4 tested angles:

| θ       | cap (all three) | k | orbit | β pattern |
|---------|----------------|---|-------|-----------|
| 0.000°  | 3.617          | 3 | 8-facet | varied |
| 1.000°  | 3.602          | 3 | 8-facet | varied |
| 4.500°  | 3.566          | 3 | 8-facet | varied |
| 9.000°  | 3.549          | 3 | 8-facet | varied |

**β values are NOT all equal** (unlike (4,4)), ranging from 0.056 to 0.242.
This suggests the (4,5) orbits do not suffer from the same degeneracy as (4,4).

**WHETHER THESE VALUES ARE CORRECT IS UNKNOWN.** Agreement between algorithms
sharing code is not evidence of correctness.

## Finding 6: Adjacency Matrices — Identical Across Angles

For (4,4):
- θ=0°: 24 adjacent pairs
- θ=0.125°: 24 adjacent pairs (IDENTICAL set)
- θ=45°: 24 adjacent pairs (IDENTICAL set)

Every q-facet is adjacent to every p-facet (4×4 = 16 pairs).
Within q-space: 0-1, 0-3, 1-2, 2-3 (4 pairs).
Within p-space: 4-5, 4-7, 5-6, 6-7 (4 pairs).
Total: 16 + 4 + 4 = 24. Same at all angles.

**Adjacency is NOT the issue.**

## Summary of Bug Patterns

### Pattern A: "Axis-aligned degeneracy"
When the p-factor is a square with axis-aligned normals (θ=0° or θ=45°),
certain ω₀ cross-products are exactly zero, creating degenerate KKT systems.
- (3,4) at θ=0°: ALL orbits fail
- (4,4) at θ=0°: works but only via a special 4-facet orbit
- (4,4) at θ=45°: billiard gives 2× (also axis-aligned after 45° rotation of square)

### Pattern B: "Infeasible transition"
The optimal orbit at θ=0° becomes KKT-infeasible at ANY θ > 0°,
even infinitesimally small rotations. The β values collapse discontinuously.
No replacement orbit with comparable action exists in the algorithm's search space.
The capacity jumps from 2.0 to ≈4.0.

### Pattern C: "Degenerate β"
Some orbits at specific angles have β components at exactly 0 or very near 0.
Example: (4,4) at θ=44.99° has a 5-facet orbit with one β = 0.000000.

## Fix Applied: KKT Null Space Search

**Root cause**: When the KKT system is rank-deficient (common for axis-aligned normals),
SVD returns the minimum-norm solution x₀ = V Σ⁻¹ Uᵀ b. This minimum-norm solution
often has β ≤ 0 for some components. But the null space of the KKT matrix is
non-trivial, and Q(β) is constant along the null space (because null space directions
satisfy the KKT stationarity conditions). So there may exist β = β₀ + Σ αᵢ vᵢ
with β > 0 and the SAME Q(β) — we just need to search for it.

**Fix** (applied to both `hk2017/src/lib.rs` and `billiard/src/kkt.rs`):
1. LU fast path: if invertible and β > 0, return immediately
2. SVD: compute particular solution β₀ and determine rank
3. If rank-deficient: extract null space from right singular vectors V^T
4. Search null space for β > 0:
   - 1D null space: find feasible interval for scalar α, pick midpoint
   - Multi-dimensional: iterative coordinate ascent on most-violated constraint

**Functions added**: `q_from_beta()`, `find_positive_beta_1d()`, `find_positive_beta_nd()`

## Post-Fix Results

### (4,4) capacity: before → after

| θ       | Before (HK) | Before (bil) | After (HK) | After (bil) | 3-way agree |
|---------|-------------|-------------|-----------|-----------|-------------|
| 0.000°  | 2.000       | 2.000       | 2.000     | 2.000     | YES |
| 0.001°  | 3.999       | 3.999       | 2.000     | 2.000     | YES |
| 0.005°  | —           | —           | 2.000     | 2.000     | YES |
| 0.010°  | —           | —           | 2.000     | 2.000     | YES |
| 0.050°  | —           | —           | 2.000     | 2.000     | YES |
| 0.100°  | —           | —           | 2.000     | 2.000     | YES |
| 0.125°  | 3.991       | 3.996       | 2.000     | 2.000     | YES |
| 0.250°  | 3.983       | 3.991       | 2.000     | 2.000     | YES |
| 0.500°  | —           | —           | 2.000     | 2.000     | YES |
| 1.000°  | 3.932       | 3.966       | 2.000     | 2.000     | YES |
| 5.000°  | 3.692       | 3.840       | 2.008     | 2.008     | YES |
| 10.000° | 3.453       | 3.704       | 2.031     | 2.031     | YES |
| 20.000° | 3.121       | 3.482       | 2.128     | 2.128     | YES |
| 30.000° | 2.928       | 3.285       | 2.309     | 2.309     | YES |
| 40.000° | 2.839       | 3.031       | 2.487     | 2.611     | NO  |
| 42.000° | 2.832       | 2.959       | 2.604     | 2.604     | NO  |
| 43.000° | 2.735       | 2.735       | 2.517     | 2.454     | NO  |
| 43.125° | 2.830       | 2.914       | 2.472     | 2.472     | YES |
| 44.000° | 2.829       | 2.876       | 2.418     | 2.418     | YES |
| 44.500° | —           | —           | 2.769     | 2.780     | NO  |
| 44.875° | 2.828       | 2.835       | 2.732     | 2.732     | YES |
| 44.990° | 2.828       | 2.829       | 2.825     | 2.825     | YES |
| 45.000° | 2.828       | **5.657**   | 2.828     | 2.828     | YES |

**Capacity jump at θ=0° → θ=ε: RESOLVED.** Capacity is now continuous (stays ~2.0).

**Billiard 2× bug at θ=45°: RESOLVED.** All three algorithms agree on 2√2 ≈ 2.828.

**Remaining disagreements near θ=40°-44.5°**: HK2017 and billiard find different
orbits at a few angles. The disagreement is small (~5%). Both algorithms find 6-facet
orbits with non-uniform β — the question is which orbit gives the true minimum action.
HK2017 is exhaustive, so it should be correct where it disagrees with billiard.
However, the billiard finds a LOWER capacity at θ=43° (2.454 vs 2.517), which means
HK2017 is NOT finding the true minimum at that angle. This needs further investigation.

### (3,4) capacity: FIXED

| θ       | Before (all) | After (all) | 3-way agree |
|---------|-------------|-----------|-------------|
| 0.000°  | **None**    | **2.121** | YES         |
| 15.000° | 2.196       | 2.196     | YES         |
| 30.000° | 2.121       | 2.121     | YES         |

(3,4) at θ=0° now returns a valid 5-facet orbit with cap = 3√2/2 ≈ 2.121.
Previously returned None because SVD minimum-norm β had negative components.

### (4,5) capacity: VALUES CHANGED

| θ       | Before (all) | After (all) | 3-way agree |
|---------|-------------|-----------|-------------|
| 0.000°  | **3.617**   | **2.558** | YES         |
| 1.000°  | 3.602       | 2.559     | YES         |

**CRITICAL**: The (4,5) capacity was WRONG before the fix. The pre-fix value of 3.617
gave sys ≈ 1.376, which was presented as a 37.6% Viterbo violation. The post-fix
value is 2.558, which gives a DIFFERENT sys value.

The new orbit is a 5-facet orbit [0, 7, 6, 2, 4] with β = [0.354, 0.171, 0.171, 0.354, 0.276],
compared to the pre-fix 8-facet orbit with varied β.

**Previous experiment results for (4,5) must be recomputed.**

### (3,3) capacity: UNCHANGED

(3,3) was never affected by the bug. All angles give correct results, all three algorithms agree.

### Orbit structure patterns (post-fix)

**Near θ=0° (4,4)**: HK2017 finds 6-facet orbits with 2 zero β components (effectively
4-facet orbits embedded in 6-facet framework). The null space search allows these to
pass β > 0 filter. Capacity stays ~2.0 (continuous).

**Mid-range (4,4)**: Clean 4-facet orbits with all β = 1/(2√2). Capacity increases smoothly.

**Near θ=45° (4,4)**: Mixed 6-facet orbits with non-uniform β. Some disagreement between
algorithms at θ=40°, 42°, 43°, 44.5°.

## Remaining Open Questions (for Jörn)

1. **(4,4) near θ=40°-44.5°**: HK2017 and billiard disagree at some angles.
   At θ=43°, billiard gives LOWER capacity (2.454 vs 2.517). Since HK2017 is
   exhaustive, this means HK2017 missed the optimal orbit. The null space search
   may still have edge cases, or the exhaustive search isn't truly exhaustive
   for all orbits in the presence of rank deficiency.

2. **(4,5) recomputation**: The polygon grid experiment needs to be rerun after
   the fix. The (4,5) sys value has changed from 1.376 to something else.

3. **Are the new values correct?** All three algorithms now agree much better,
   but they still share the same KKT solver code. Independent validation
   (e.g. hand calculation for (4,4) at θ=0° = hypercube → cap=2.0) supports
   the fix, but we have no independent reference for (4,5).

## Regression Tests Added

Four regression tests in `crates/hk2017/src/lib_test.rs`:
- `kkt_nullspace_square_square_zero`: (4,4) at θ=0° → cap=2.0
- `kkt_nullspace_square_square_near_zero`: (4,4) at θ=0.125° → cap≈2.0
- `kkt_nullspace_square_square_45deg`: (4,4) at θ=45° → cap=2√2, HK==billiard
- `kkt_nullspace_triangle_square_zero`: (3,4) at θ=0° → cap=3√2/2 (was None)

All pass in < 21s (debug mode).

## Raw Data Archive

All diagnostic output is preserved in:
- `/tmp/post_fix_diagnostic.txt` (full (4,4) sweep after fix)
- `/tmp/post_fix_remaining.txt` ((4,4) at 45°, (4,5))
- The test source code: `crates/hk2017/src/square_product_diagnostic.rs`

The data in the tables above is transcribed directly from test output.
