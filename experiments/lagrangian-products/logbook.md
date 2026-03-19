# Lagrangian Products of Rotated Polygons: Logbook

## Motivation

The only known 4D counterexample to Viterbo's conjecture is a Lagrangian product of two pentagons (Haim-Kislev and Ostrover 2024). This experiment systematically searches for further counterexamples (sys > 1) in the space of Lagrangian products of regular 2D polygons by sweeping the rotation angle between the two factors.

## Status

**Complete.** Pentagon x pentagon at theta = 18 degrees confirms the HKO counterexample (sys ~ 1.047). No other regular polygon pair with 3 <= n <= m <= 6 achieves sys > 1 (at 6-degree resolution).

## How to run

```bash
cd experiments/
cargo run --bin lagrangian_products --release       # generates all JSONL datasets
python3 lagrangian-products/analyze.py           # generates plots
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: generates all sweep datasets |
| `analyze.py` | Python: plots sys(theta) curves |
| `math.tex` | Formal proofs and definitions (symmetry lemma, rotation curves, polygon pair grid) |
| `lagrangian-products-5x5.jsonl` | Pentagon rotation curve (37 rows) |
| `lagrangian-products-3x3-6deg.jsonl` | Triangle x triangle sweep (11 rows) |
| `lagrangian-products-3x4-6deg.jsonl` | Triangle x square sweep (4 rows) |
| `lagrangian-products-3x5-6deg.jsonl` | Triangle x pentagon sweep (3 rows) |
| `lagrangian-products-3x6-6deg.jsonl` | Triangle x hexagon sweep (6 rows) |
| `lagrangian-products-4x4-6deg.jsonl` | Square x square sweep (9 rows) |
| `lagrangian-products-4x5-6deg.jsonl` | Square x pentagon sweep (3 rows) |
| `lagrangian-products-4x6-6deg.jsonl` | Square x hexagon sweep (4 rows) |
| `lagrangian-products-5x5-6deg.jsonl` | Pentagon x pentagon sweep at 6-deg (7 rows) |
| `lagrangian-products-5x6-6deg.jsonl` | Pentagon x hexagon sweep (2 rows) |
| `lagrangian-products-6x6-6deg.jsonl` | Hexagon x hexagon sweep (6 rows) |
| `lagrangian_products_5x5.png` | Pentagon rotation curve figure |
| `lagrangian_products_polygon_pairs.png` | All polygon pairs comparison figure |

## Design

### Rotation parameter

Fix polygons P in q-space and Q in p-space. Rotate Q by angle theta. The Lagrangian product K_theta = P x_L R(theta)Q defines a one-parameter family. Only relative rotation matters (rotating both factors by the same angle is a symplectic transformation).

### Symmetry reduction

For regular n-gon x m-gon, sys(theta) has period 2*pi/lcm(n,m) and mirror symmetry sys(theta) = sys(-theta), so the fundamental domain is [0, pi/lcm(n,m)]. (See Lemma `lem:rotation-fundamental-domain` in math.tex for the proof.)

### Three families

1. **Pentagon rotation curve (Family 1):** P = Q = regular pentagon, theta in [0, 36] degrees at 1-degree steps. Fundamental domain = 180/lcm(5,5) = 36 degrees. Output: `lagrangian-products-5x5.jsonl` (37 points).

2. **Polygon pair grid (Family 2):** All pairs (n, m) with 3 <= n <= m <= 6, at 6-degree steps over the fundamental domain. 10 pairs, one JSONL file each. Capacity computed using billiard algorithm.

3. **Random Lagrangian products (Family 3):** Delegated to separate `random-product-sweep` experiment.

### Algorithm

All capacities computed using the billiard algorithm (fast, production default for Lagrangian products).

## Findings

1. **Pentagon x pentagon at theta = 18 degrees achieves sys ≈ 1.0472**, confirming the HKO counterexample. This is the global maximum across the fundamental domain.

2. **Minimum sys ≈ 0.9472 at theta = 0 degrees** (aligned pentagons).

3. **Violation region** (sys > 1) spans approximately theta in (13.5, 22.5) degrees within each fundamental domain, about 25% of the period.

4. **No other regular polygon pair (3 <= n <= m <= 6) achieves sys > 1** at 6-degree resolution. All 10 polygon pair curves stay below 1.

5. **Sys is a smooth function of rotation angle** with the expected periodicity from the symmetry lemma.

6. **The counterexample is a local maximum in the rotation parameter space.** The sys(theta) curve peaks at theta = 18 degrees and decreases in both directions.

## Triangle x Square Investigation

Triggered by a mismatch with CH2021 (Chaidez-Hutchings citing Schlenk Lem. 5.3.1): the expected capacity for `symplectic_triangle_square` disagreed with the billiard computation. The investigation resolved two bugs in the `known_polytopes` library:

1. **Naming error:** The `symplectic_triangle_square` polytope was actually a Lagrangian product (factors in the q-plane and p-plane, both Lagrangian subspaces). A true symplectic product would place factors in (q1, p1) and (q2, p2).

2. **Expected capacity error:** The expected capacity was set to 1.0 (from the min(c_A, c_B) formula, which applies to symplectic products). The correct value for the equilateral triangle x_L square is 1.5 (verified by billiard computation). The Schlenk Lem. 5.3.1 reference (via CH2021) concerns a right isosceles triangle, not equilateral.

Key numerical results:
- Equilateral triangle x_L square: capacity = 1.5, sys = sqrt(3)/2 ≈ 0.866 (scale-and-ratio-invariant for any equilateral triangle x_L square)
- Right isosceles triangle x_L square: capacity = 1.0, sys = 1.0
- True symplectic triangle x_S square: capacity = 1.0, sys = 0.385

Fixes applied to `crates/src/geom/known_polytopes.rs`:
- Renamed function to `lagrangian_triangle_square()` with correct capacity 1.5
- Added separate `symplectic_triangle_square()` with capacity 1.0

## Known limitations

- Only regular polygons tested; irregular polygon products not explored.
- Rotation angle step size is 6 degrees for the polygon pair grid (coarser than the 1-degree pentagon sweep).
- Family 3 (random Lagrangian products) delegated to `random-product-sweep` experiment.
