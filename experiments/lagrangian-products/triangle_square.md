# Triangle x Square Investigation

## Summary

The algorithm correctly computes capacity = 1.5 for the `symplectic_triangle_square` polytope. The discrepancy with the expected value of 1.0 has two independent root causes:

1. **Naming/construction error**: The polytope is a *Lagrangian* product (factors in the (q1,q2) and (p1,p2) planes), not a *symplectic* product (which would require factors in (q1,p1) and (q2,p2) planes). The capacity formula `c(A x_S B) = min(c(A), c(B))` applies only to symplectic products, not Lagrangian products.

2. **Literature misattribution**: The comment cites Schlenk Lem. 5.3.1 (via CH2021) which concerns a Lagrangian product of a **right isosceles triangle** and a square having systolic ratio 1. Our construction uses an **equilateral** triangle, which gives systolic ratio sqrt(3)/2 ~ 0.866, not 1.

## Polytope Construction Analysis

### Current construction (`symplectic_triangle_square`)

- **Triangle**: equilateral with circumradius 1 (inradius 0.5) in the (q1, q2) plane
  - Normals: `(cos theta, sin theta, 0, 0)` for theta = pi/2 + 2*pi*k/3
  - Heights: 0.5 (= inradius)
- **Square**: unit square [-0.5, 0.5]^2 in the (p1, p2) plane
  - Normals: `(0, 0, +/-1, 0)` and `(0, 0, 0, +/-1)`
  - Heights: 0.5
- **7 facets total**, 12 vertices

### Subspace classification

| Subspace | Coordinates | omega_0 restricted | Classification |
|----------|-------------|-------------------|----------------|
| (q1, q2) | (*, *, 0, 0) | omega_0(e_q1, e_q2) = 0 | **Lagrangian** |
| (p1, p2) | (0, 0, *, *) | omega_0(e_p1, e_p2) = 0 | **Lagrangian** |
| (q1, p1) | (*, 0, *, 0) | omega_0(e_q1, e_p1) = 1 | **Symplectic** |
| (q2, p2) | (0, *, 0, *) | omega_0(e_q2, e_p2) = 1 | **Symplectic** |

The triangle is in the Lagrangian subspace (q1, q2) and the square is in the Lagrangian subspace (p1, p2). This makes the product a **Lagrangian product**, despite the function being named `symplectic_triangle_square`.

A true symplectic product would place the triangle in (q1, p1) and the square in (q2, p2).

### Verified areas

- Triangle area: 3*sqrt(3)/4 ~ 1.299
- Square area: 1.0
- 4D volume (product of areas): 1.299 (confirmed by volume computation)

## Algorithm Output Analysis

The algorithm finds the minimum action orbit with **5 facets** (permutation [0, 6, 1, 2, 5]):

| Position | Facet | Normal | Height | Beta |
|----------|-------|--------|--------|------|
| sigma(0) | F0 | (0, +1, 0, 0) | 0.5 | 1/3 |
| sigma(1) | F6 | (0, 0, 0, -1) | 0.5 | 1/2 |
| sigma(2) | F1 | (-0.866, -0.5, 0, 0) | 0.5 | 1/3 |
| sigma(3) | F2 | (+0.866, -0.5, 0, 0) | 0.5 | 1/3 |
| sigma(4) | F5 | (0, 0, 0, +1) | 0.5 | 1/2 |

- Q(beta) = 1/3
- Action = 0.5 / Q(beta) = 1.5
- Constraints satisfied: N^T beta = 0, eta^T beta = 1

The orbit visits all 3 triangle facets (with beta = 1/3 each) and 2 of the 4 square facets (the p2-direction pair, with beta = 1/2 each).

### Billiard interpretation

For a Lagrangian product K x_L T, the EHZ capacity equals the minimum T-billiard T-dual-length in K (Theorem 2.13 in Artstein-Avidan--Ostrover, Theorem 1 in Rudolf).

With K = equilateral triangle (circumradius 1) and T = [-0.5, 0.5]^2:
- Support function: h_T(u) = 0.5|u1| + 0.5|u2|
- Altitude trajectory (vertex to opposite side midpoint): T-dual-length = 1.5
- This matches the algorithm output exactly.

### Formula derivation

For equilateral triangle with inradius r and square with half-side s:
- c = 6sr (altitude trajectory: height = 3r, h_T(step) = 3sr, round trip = 6sr)
- vol = 3*sqrt(3)*r^2 * 4s^2 = 12*sqrt(3)*r^2*s^2
- sys = c^2 / (2*vol) = 36*s^2*r^2 / (24*sqrt(3)*r^2*s^2) = sqrt(3)/2 ~ 0.866

This is scale- and ratio-invariant: sys = sqrt(3)/2 for ANY equilateral triangle x_L square.

## Literature Review

### CH2021 (Chaidez-Hutchings) reference

> "Apparently the previous minimum number of vertices of a known example with systolic ratio 1 was 12, given by the Lagrangian product of a triangle and a square [Lem. 5.3.1]{Schlenk}."

This says "a triangle and a square" -- it does NOT specify "equilateral." The Schlenk reference (Embedding Problems in Symplectic Geometry, 2005) discusses specific triangles that achieve sys = 1.

### Verification: right isosceles triangle

A right isosceles triangle (legs = 1) x_L unit square gives:
- capacity = 1.0
- volume = 0.5
- sys = 1.0^2 / (2 * 0.5) = 1.0

This confirms the Schlenk reference uses a right isosceles triangle, not equilateral.

### Symplectic product formula

For a *true* symplectic product (factors in symplectically orthogonal symplectic subspaces), the formula c(A x_S B) = min(c(A), c(B)) holds. Verified computationally:

Triangle in (q1, p1) x Square in (q2, p2):
- capacity = 1.0 = min(area_tri, area_sq) = min(1.299, 1.0)
- Best orbit uses only the 4 square facets (subset [3,4,5,6])

## Conclusion

**Hypothesis H1 is closest to correct, but more precisely:**

The construction is a Lagrangian product (not symplectic), AND the expected value formula was also misapplied:
- The `min(area)` formula applies to symplectic products, not Lagrangian products.
- The Schlenk sys=1 result applies to a right isosceles triangle, not equilateral.
- The algorithm's output of 1.5 is correct for the polytope as constructed.

| Component | Status |
|-----------|--------|
| Algorithm (hk2017) | **Correct** -- verified by billiard calculation |
| Polytope normals/heights | **Correct** -- polytope is validly constructed |
| Function name | **Wrong** -- says "symplectic" but is actually Lagrangian |
| Expected capacity (1.0) | **Wrong** -- correct value is 1.5 |
| Literature citation | **Misleading** -- equilateral triangle gives sys=0.866, not 1.0 |

## Recommendation

Fix the code with the following changes (option C from investigation):

1. **Rename** `symplectic_triangle_square` to clarify it is a Lagrangian product.
2. **Fix expected capacity** to 1.5 and update the source comment.
3. **Optionally add** a true symplectic product (triangle in (q1,p1), square in (q2,p2)) with expected capacity 1.0, which would genuinely test the `min(c_A, c_B)` formula.
4. **Optionally add** the right isosceles triangle x_L square with sys=1 as a separate known polytope, matching the Schlenk reference.

The test in `lib_test.rs` already uses the correct computed value (1.5) rather than the wrong literature value (1.0), so it passes. But the `known_polytopes.rs` constructor still sets `capacity: 1.0`.

## Changes Made

Investigation binaries created (throwaway; removed during repo restructure):
- `investigate_tri_sq.rs`
- `investigate_tri_sq_v2.rs`
- `investigate_tri_sq_v3.rs`

All three mandatory recommended fixes were applied to `crates/src/geom/known_polytopes.rs`:

1. **Rename**: `lagrangian_triangle_square()` added as the renamed, correctly-labelled function (capacity 1.5)
2. **Fix expected capacity**: capacity set to 1.5 in `lagrangian_triangle_square()`
3. **True symplectic product added**: `symplectic_triangle_square()` added with capacity 1.0 (formula min(c_A, c_B))

The optional fix (adding right isosceles triangle ×_L square with sys=1 matching Schlenk Lem. 5.3.1) was not applied.

## Appendix: Key Numerical Results

```
Equilateral triangle x_L square:
  capacity = 1.500000
  volume   = 1.299038
  sys      = 0.866025 (= sqrt(3)/2)

Right isosceles triangle x_L square:
  capacity = 1.000000
  volume   = 0.500000
  sys      = 1.000000

True symplectic triangle x_S square:
  capacity = 1.000000
  volume   = 1.299038
  sys      = 0.384900

Triangle x_L triangle (for comparison):
  capacity = 1.500000
  volume   = 1.687500
  sys      = 0.666667
```
