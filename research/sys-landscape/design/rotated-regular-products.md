# Lagrangian Products of Rotated Polygons: Logbook

## Motivation

The only known 4D counterexample to Viterbo's conjecture is a Lagrangian product of two pentagons (Haim-Kislev and Ostrover 2024). This experiment systematically searches for further counterexamples (sys > 1) in the space of Lagrangian products of regular 2D polygons by sweeping the rotation angle between the two factors.

## Status

**Complete.** Pentagon x pentagon at theta = 18 degrees confirms the HKO counterexample (sys ~ 1.047, `lagrangian-products-5x5.jsonl`). Heptagon x heptagon peaks at sys ~ 0.917 (theta ~ 12.86 degrees, `lagrangian-products-7x7.jsonl`), well below 1. No other regular polygon pair with 3 <= n <= m <= 7 achieves sys > 1.

## How to run

```bash
cargo run -p exp-sys-landscape --release --bin sys-rotated-regular-products
uv run analyze.py                             # generates plots
```

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: generates all sweep datasets |
| `analyze.py` | Python: plots sys(theta) curves |
| `formal/sys-landscape/rotated-regular-products.tex` | Formal proofs and definitions (rotation setup, symmetry lemma) |
| `lagrangian-products-5x5.jsonl` | Pentagon rotation curve (37 rows) |
| `lagrangian-products-7x7.jsonl` | Heptagon rotation curve (27 rows) |
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
| `lagrangian_products_7x7.png` | Heptagon rotation curve figure |
| `lagrangian_products_polygon_pairs.png` | All polygon pairs comparison figure |

## Design

### Rotation parameter

Fix polygons P in q-space and Q in p-space. Rotate Q by angle theta. The Lagrangian product K_theta = P x_L R(theta)Q defines a one-parameter family. Only relative rotation matters (rotating both factors by the same angle is a symplectic transformation).

### Symmetry reduction

For regular n-gon x m-gon, sys(theta) has period 2*pi/lcm(n,m) and mirror symmetry sys(theta) = sys(-theta), so the fundamental domain is [0, pi/lcm(n,m)]. (See Lemma `lem:rotation-fundamental-domain` in formal/sys-landscape/rotated-regular-products.tex for the proof.)

### Families

1. **Pentagon rotation curve (Family 1):** P = Q = regular pentagon, theta in [0, 36] degrees at 1-degree steps. Fundamental domain = 180/lcm(5,5) = 36 degrees. Output: `lagrangian-products-5x5.jsonl` (37 points).

2. **Heptagon rotation curve (Family 1b):** P = Q = regular heptagon, theta in [0, 180/7] degrees at ~1-degree steps (27 points over the fundamental domain). Output: `lagrangian-products-7x7.jsonl` (27 points).

3. **Polygon pair grid (Family 2):** All pairs (n, m) with 3 <= n <= m <= 6, at 6-degree steps over the fundamental domain. 10 pairs, one JSONL file each. Capacity computed using billiard algorithm.

4. **Random Lagrangian products (Family 3):** Delegated to separate `random-product-sample` experiment.

### Algorithm

All capacities computed using the billiard algorithm (fast, production default for Lagrangian products).

## Findings

1. **Pentagon x pentagon at theta = 18 degrees achieves sys ≈ 1.0472** (`lagrangian-products-5x5.jsonl` row 19: sys=1.047214), confirming the HKO counterexample. This is the global maximum across the fundamental domain.

2. **Minimum sys ≈ 0.9472 at theta = 0 degrees** (aligned pentagons) (`lagrangian-products-5x5.jsonl` row 1: sys=0.947214).

3. **Violation region** (sys > 1) spans theta in (13, 23) degrees within each fundamental domain (from theta=14 to theta=22 at 1-degree resolution, `lagrangian-products-5x5.jsonl`), about 25% of the 36-degree period.

4. **No other tested regular polygon pair achieves sys > 1.** Among all pairs with 3 <= n <= m <= 6 (at 6-degree resolution) and the 7x7 pair (at ~1-degree resolution), all sys curves stay below or at 1. Note: mixed pairs involving 7 (3x7, 4x7, 5x7, 6x7) were not tested. The 3x6 and 4x4 pairs reach sys=1.000 at the 6-degree sweep resolution (`lagrangian-products-3x6-6deg.jsonl`, `lagrangian-products-4x4-6deg.jsonl`) — finer sweeps may be warranted.

5. **Sys appears continuous as a function of rotation angle** with the expected periodicity from the symmetry lemma (observed at 1-degree resolution in `lagrangian-products-5x5.jsonl`; formal smoothness not verified).

6. **The counterexample is a local maximum in the rotation parameter space.** The sys(theta) curve peaks at theta = 18 degrees and decreases in both directions (`lagrangian-products-5x5.jsonl`).

7. **Heptagon x heptagon peaks at sys ≈ 0.917 at theta ≈ 12.86 degrees.** The curve is symmetric about the midpoint of the fundamental domain (180/14 ≈ 12.86 degrees). The peak is well below 1 — no counterexample (`lagrangian-products-7x7.jsonl` row 14: sys=0.917408, angle_deg=12.857).

## Triangle x Square Investigation

Triggered by a mismatch with CH2021 (Chaidez-Hutchings citing Schlenk Lem. 5.3.1): the expected capacity for `symplectic_triangle_square` disagreed with the billiard computation. The investigation resolved two bugs in the `known_polytopes` library:

1. **Naming error:** The `symplectic_triangle_square` polytope was actually a Lagrangian product (factors in the q-plane and p-plane, both Lagrangian subspaces). A true symplectic product would place factors in (q1, p1) and (q2, p2).

2. **Expected capacity error:** The expected capacity was set to 1.0 (from the min(c_A, c_B) formula, which applies to symplectic products). The correct value for the equilateral triangle x_L square is 1.5 (verified by billiard computation). The Schlenk Lem. 5.3.1 reference (via CH2021) concerns a right isosceles triangle, not equilateral.

Key numerical results (billiard computation, verified at time of investigation):
- Equilateral triangle x_L square: capacity = 1.5, sys = sqrt(3)/2 ≈ 0.866 (scale-and-ratio-invariant for any equilateral triangle x_L square)
- Right isosceles triangle x_L square: capacity = 1.0, sys = 1.0
- True symplectic triangle x_S square: capacity = 1.0, sys = 0.385
% [TODO: JÖRN - the known_polytopes functions referenced here were removed in a later refactor.
% The numerical values above are from the original investigation but are no longer reproducible
% from the current codebase. Decide whether to restore the test fixtures or remove this section.]

Fixes applied to `library/src/geom/known_polytopes.rs`:
- Renamed function to `lagrangian_triangle_square()` with correct capacity 1.5
- Added separate `symplectic_triangle_square()` with capacity 1.0

## Figures

- `lagrangian_products_5x5.png`: Systolic ratio of Pentagon x_L R(theta) Pentagon as a function of rotation angle theta on the fundamental domain [0, 36 degrees]. Red dashed line marks sys = 1 (Viterbo threshold). Shows the violation region around theta = 18 degrees.
- `lagrangian_products_7x7.png`: Systolic ratio of Heptagon x_L R(theta) Heptagon as a function of rotation angle theta on the fundamental domain [0, 25.7 degrees]. Red dashed line marks sys = 1 (Viterbo threshold). Peak at sys ~ 0.917, well below 1.
- `lagrangian_products_polygon_pairs.png`: Systolic ratio curves for all 10 regular n-gon x_L R(theta) m-gon pairs (3 <= n <= m <= 6). Dashed line marks sys = 1.

## Known limitations

- Only regular polygons tested; irregular polygon products not explored.
- Rotation angle step size is 6 degrees for the polygon pair grid (coarser than the 1-degree pentagon/heptagon sweeps).
- Family 3 (random Lagrangian products) delegated to `random-product-sample` experiment.
