# Lagrangian Search: Logbook

## Motivation

HKO2024 is the only known polytope with sys > 1. It's a Lagrangian product of two regular pentagons at θ=18°. How special is this? Is the sys>1 region a tiny needle in Lagrangian product space, or is it accessible by random sampling or gradient optimization? And: is HKO the only sys>1 Lagrangian product, or are there others?

## Status

**Phase 1 complete** (dense perturbation sweep). Phases 2-3 deferred — Phase 1 results reshape the approach. 2026-03-27.

## Research questions

### Phase 1: Feasibility estimate

What fraction of random Lagrangian products have sys > 1?

Existing data points:
- LP(5,5) rotation sweep (lagrangian-products experiment): ~25% of rotations of *regular* pentagons have sys > 1 (9/36 at 1° resolution; θ ∈ 14°–22° of the 36° fundamental domain, `lagrangian-products-5x5.jsonl`).
- random-product-sweep: 100 random products with 3≤k≤m≤6, max sys = 0.794. None near 1.

These suggest regularity of the pentagon matters a lot, not just the polygon count. The question: what's the volume ratio of {sys > 1} to the full LP parameter space? If it's < 1e-4, brute-force sampling won't find it. If it's ~ 1%, a few thousand samples might.

Sample random LP(n,m) products (random polygon shapes, random rotation) for various (n,m). Focus on LP(5,5) but also test neighboring pairs.

### Phase 2: Guided search (contingent on Phase 1)

Does gradient ascent from random Lagrangian products find trajectories to the sys > 1 region? How large is HKO's basin of attraction under gradient ascent?

gradient-descent logbook reports best sys = 0.905 from Lagrangian starts (within-cell only). % [TODO: JÖRN - gradient-descent.jsonl appears truncated (612 polytopes, missing 4x6 and 5x5 splits). The 0.905 claim comes from the logbook, not verified against current JSONL.] With boundary crossing, maybe some trajectories reach HKO's basin. But if Phase 1 shows the sys > 1 region is extremely small, this may not be worth running at scale.

### Phase 3: Novelty check (contingent on Phase 2)

If Phase 2 (or Phase 1) finds a sys > 1 polytope: is it related to HKO2024 by a symplectomorphism? Check symmetry group, orbit structure, dual vertex geometry. A genuinely different sys > 1 Lagrangian product would be immediately publishable.

## Design notes

- Billiard algorithm for all evaluations (fast for Lagrangian products).
- Phase 1 is cheap: single sys evaluation per sample, ~25ms at F=10.
- Need to think about the probability measure on "random Lagrangian products." What distribution on polygon vertices? Uniform on vertex positions? Uniform on edge lengths? This affects what "volume ratio" means.
- Phase 2 depends on sys-search infrastructure (gradient ascent + boundary crossing). Could share code or wait for that experiment to mature.

## Predecessor experiments

- **lagrangian-products** — systematic rotation sweep of regular polygon pairs. The 1D version of Phase 1.
- **random-product-sweep** — 100 random products, max sys = 0.794. A small-scale version of Phase 1 with random (not regular) polygons.
- **pentagon-perturb** — 100 perturbations of HKO2024 at ε=0.01, all retain sys > 1. Shows the sys > 1 region has nonzero volume around HKO.
- **gradient-descent** — 501 Lagrangian products optimized, best sys = 0.905.

## How to run

```bash
cd experiments/
cargo run --release --bin lagrangian_search
python3 lagrangian-search/analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: dense perturbation sweep of HKO2024 dual vertices |
| `analyze.py` | Python: fraction-vs-epsilon curve, strip plot, summary table |
| `lagrangian-search.jsonl` | Per-sample data (6501 rows: 1 base + 500 per ε level × 13 levels) |
| `lagrangian-search-levels.jsonl` | Per-level summary (13 rows) |
| `lagrangian_search_fraction.png` | Figure: fraction with sys > 1 vs perturbation ε |
| `lagrangian_search_sys_vs_eps.png` | Figure: sys distribution per ε level (strip plot) |

## Phase 1: Dense perturbation sweep (2026-03-27)

### Design

Instead of random sampling across the full LP parameter space (already done by random-product-sweep with 0 hits), we measure the sys > 1 region directly by sweeping perturbation magnitude around the known counterexample.

- **Base:** HKO2024 dual vertices (10 facets in R⁴, Lagrangian product structure)
- **Perturbation:** Uniform[-ε, ε] on the 2 nonzero Lagrangian components of each dual vertex (20 independent coordinates). Only the nonzero components are perturbed, preserving Lagrangian product structure so billiard algorithm remains valid.
- **ε levels:** 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.10, 0.15, 0.20, 0.50, 1.00 — dense in the transition zone [0.02, 0.10]
- **Samples:** 500 valid polytopes per ε level via rejection sampling (max 100K attempts)
- **Algorithm:** billiard (native for Lagrangian products)
- **Seed:** 42

**Perturbation space geometry:** The base dual vertices have magnitude |a_i| ≈ 1.24. The perturbation space is 20-dimensional (2 components × 10 facets). At ε per component, the L2 norm of the perturbation vector concentrates around ε√(20/3) ≈ 2.58ε with ~10% relative spread (concentration of measure in 20D).

### Findings

All verified against `lagrangian-search-levels.jsonl`.

**1. The sys > 1 region has a characteristic per-component radius ε* ≈ 0.035.**

| ε | accepted/attempts | sys > 1 fraction | sys range |
|---|---|---|---|
| 0.01 | 500/500 (100%) | 500/500 (100.0%) | [1.014, 1.042] |
| 0.02 | 500/500 (100%) | 492/500 (98.4%) | [0.993, 1.037] |
| 0.03 | 500/500 (100%) | 367/500 (73.4%) | [0.970, 1.033] |
| 0.04 | 500/500 (100%) | 172/500 (34.4%) | [0.944, 1.030] |
| 0.05 | 500/500 (100%) | 79/500 (15.8%) | [0.912, 1.025] |
| 0.06 | 500/500 (100%) | 24/500 (4.8%) | [0.908, 1.027] |
| 0.07 | 500/500 (100%) | 8/500 (1.6%) | [0.873, 1.016] |
| 0.08 | 500/500 (100%) | 1/500 (0.2%) | [0.862, 1.002] |
| 0.10 | 500/500 (100%) | 1/500 (0.2%) | [0.814, 1.002] |
| 0.15 | 500/500 (100%) | 0/500 (0.0%) | [0.707, 0.971] |
| 0.20 | 500/500 (100%) | 0/500 (0.0%) | [0.605, 0.957] |
| 0.50 | 500/521 (96.0%) | 0/500 (0.0%) | [0.322, 0.905] |
| 1.00 | 500/2350 (21.3%) | 0/500 (0.0%) | [0.006, 0.839] |

The fraction with sys > 1 follows a sigmoidal decay from 100% at ε=0.01 to 0% at ε≥0.15. The 50% crossing is between ε=0.03 and ε=0.04. In relative terms, ε*=0.035 is ~2.8% of the base dual vertex magnitude.

**2. Acceptance rate stays near 100% until ε ≈ 0.5.**

Polytope validity (bounded, irredundant, vertex-enumerable) is not the binding constraint — the perturbation destroys sys > 1 long before it destroys polytope structure. Only at ε=1.00 (80% relative perturbation per component) does rejection become significant (21.3% acceptance).

**3. The sys distribution shifts and widens monotonically with ε.**

At ε=0.01: mean sys = 1.033 ± 0.004 (tight cluster above 1). At ε=0.10: mean sys = 0.920 ± 0.035 (below 1 with rare exceptions). At ε=1.00: mean sys = 0.425 ± 0.180 (converging toward the random-product-sweep regime).

**4. Consistency with pentagon-perturb.** Pentagon-perturb used ε=0.01 per component (all 4 components of each dual vertex, not just the 2 Lagrangian ones) and found 100/100 above sys > 1 with min sys = 1.002. Our ε=0.01 level (Lagrangian-only perturbation) finds 500/500 above with min sys = 1.014, consistent and slightly tighter (fewer perturbation dimensions, so smaller effective distortion).

### Interpretation

The sys > 1 region around HKO2024 is a **ball of modest radius** in Lagrangian product space — not a measure-zero point, but not a large region either. The ~2.8% relative radius means that polygons perturbed by more than ~3% per dual-vertex component almost certainly drop below sys = 1.

This explains the predecessor results:
- **pentagon-perturb** (ε=0.01): deep inside the ball → 100% retention
- **random-product-sweep** (fully random shapes): far outside the ball → 0% hits
- **gradient-descent** (random starts, within-cell optimization): reaches sys ≈ 0.9 but cannot cross combinatorial boundaries to reach HKO's basin

The transition is smooth (no sharp boundary), the std of sys grows with ε (expected: larger perturbations explore a larger volume and hit the boundary at varying angles).

### Open questions

1. **Anisotropy:** Is the sys > 1 boundary roughly spherical in the 20D perturbation space, or highly elongated/irregular? The per-sample perturbation vectors (stored in `delta_2d`) enable post-hoc directional analysis (PCA of above/below partition, directional hit rates, etc.). Not yet analyzed.

2. **Is the region star-shaped?** The random perturbation approach implicitly assumes the boundary is "roughly convex" for the fraction-vs-ε curve to be meaningful. If the region has tentacles or holes, the fraction curve is an average over directions. Directional analysis would reveal this.

3. **What fraction of the full LP(5,5) parameter space is sys > 1?** Our sweep measures the local radius but not the global fraction. The full LP(5,5) space is ~17-20 dimensional (depending on quotient), and the sys > 1 ball has radius ~0.035 around a point at distance ~1.24 from the origin. A rough volume fraction estimate: (0.035/1.24)^20 ≈ 10^{-31} — vanishingly small. Even in 1D (rotation only), the fraction is ~25% (regular pentagons, 9/36 in `lagrangian-products-5x5.jsonl`), but shape variation adds ~18 more dimensions in which the region is narrow.

4. **Why does regularity matter?** The rotation sweep (lagrangian-products) shows 25% of rotations of *regular* pentagons exceed sys > 1. Random pentagons never come close. The transition from regular to irregular appears to happen at ~3% perturbation. Understanding *which* deformations from regularity are most harmful would clarify why HKO works.

5. **Higher polygon pairs:** LP(7,7) regular peaks at 0.917. Would a dense perturbation sweep around the LP(7,7) optimum show a sys > 1 region (latent in the deformation), or is it strictly below 1 for all nearby shapes?
