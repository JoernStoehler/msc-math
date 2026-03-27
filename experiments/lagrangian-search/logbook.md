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

1. **What fraction of the full LP(5,5) parameter space is sys > 1?** Our sweep measures the local radius but not the global fraction. The full LP(5,5) space is ~17-20 dimensional (depending on quotient), and the sys > 1 region has characteristic radius ~0.035 around a point at distance ~1.24 from the origin. A rough volume fraction estimate: (0.035/1.24)^20 ≈ 10^{-31} — vanishingly small. Even in 1D (rotation only), the fraction is ~25% (regular pentagons, 9/36 in `lagrangian-products-5x5.jsonl`), but shape variation adds ~18 more dimensions in which the region is narrow.

2. **Why does regularity matter?** The rotation sweep (lagrangian-products) shows ~25% of rotations of *regular* pentagons exceed sys > 1. Random pentagons never come close. The transition from regular to irregular appears to happen at ~3% perturbation. Understanding *which* deformations from regularity are most harmful would clarify why HKO works.

3. **What does the sys > 1 region look like?** See analysis attempt below.

## Shape of the sys > 1 region (2026-03-27)

### Failed approaches

**Linear model on δ:** Fitting sys ~ δ (20 features) gives R²=0.02. Expected: at a critical point the linear term vanishes, and the dominant dependence is higher-order. The R²≈0 confirms gradient ≈ 0 (this doesn't require C²).

**Isotropic quadratic (L2 norm):** Fitting Δsys ~ ‖δ‖² gives R²=0.48 in the transition zone (ε=0.03-0.06, n=2000). Pooling all ε levels, sys ~ L2 gives R²=0.85, but this is trivially driven by the ε-level structure.

**Anisotropic quadratic (Hessian):** Fitting Δsys ~ δᵀHδ (210 features) gives R²=0.78, a 0.30 improvement over isotropic. The fitted H has eigenvalues from -8.2 to +0.7 (16 negative, 4 positive), suggesting ~9× aspect ratio.

**Why the quadratic model is wrong:** sys is not C² at HKO2024. HKO has degenerate minimum-action orbits (multiple orbits tie for the capacity), so sys = c²/2vol is locally the pointwise minimum of several smooth sheets. The Hessian of a pointwise minimum is not the Hessian of any individual sheet. The 4 positive eigenvalues likely reflect directions where one sheet rises while another (lower) sheet falls — the quadratic form can't represent this. The fitted "eigenvalues" are artifacts of projecting a non-smooth surface onto a smooth model.

### The clean question

We have f: R²⁰ → R (sys as a function of the perturbation δ). Properties:
- f(0) ≈ 1.047 (local max, or near one)
- f is Lipschitz but not C² (piecewise smooth: f = min_k f_k where each f_k is smooth, corresponding to a particular optimal orbit)
- 6500 point evaluations at random locations in L∞ boxes of varying size

We want to characterize S = {δ : f(δ) > 1}.

**Approaches that respect the structure:**

1. **Directional radius estimation.** For random directions u ∈ S¹⁹, binary-search for r(u) where f(r·u) = 1. The distribution of r(u) characterizes S without smoothness assumptions. Requires ~200 new evaluations (20 directions × 10 bisection steps), ~5 seconds. Assumes S is star-shaped w.r.t. the origin.

2. **Piecewise-linear model.** The correct local model is f(δ) ≈ f(0) + min_k ⟨g_k, δ⟩, where g_k are the gradients of the smooth sheets meeting at HKO. Fitting this is a min-of-affine-functions regression. The level set {min_k ⟨g_k, δ⟩ = c} is a polytope in δ-space, which is the right geometric object for piecewise-smooth f. The number of sheets k can be estimated from the data by the number of distinct orbit types appearing near HKO.

3. **Support function / convex hull.** If S is convex, compute the convex hull of the above-threshold samples. Its principal widths characterize the shape without smoothness. Requires convexity of S (plausible for the superlevel set of a concave piecewise-linear f, but not guaranteed).

Method 1 is cheapest and most informative per evaluation. Method 2 is the correct theoretical model but requires identifying the orbit sheets.

### Ball test from existing data

**The region is not a ball.** Direct test: if S were an L2 ball of radius R, every above-1 sample would have L2 < R and every below-1 sample L2 > R. But:
- Furthest above-1 sample: L2 = 0.230
- Closest below-1 sample: L2 = 0.050

966 above-1 samples are further from HKO than the closest below-1 sample. The boundary radius varies by at least 4.6× across directions. Per-ε breakdown:

| ε | frac>1 | L2 of furthest above-1 | L2 of closest below-1 |
|---|---|---|---|
| 0.02 | 98.4% | 0.065 | 0.050 |
| 0.03 | 73.4% | 0.096 | 0.060 |
| 0.04 | 34.4% | 0.124 | 0.073 |
| 0.05 | 15.8% | 0.166 | 0.088 |
| 0.06 | 4.8% | 0.197 | 0.112 |

The boundary radius r(u) varies from ~0.05 (steepest directions) to ~0.20 (flattest). The fraction-vs-ε curve averages over all directions and reports the mean behavior.

## Directional boundary probing (2026-03-27)

The L∞-box sweep measures the average size of the sys > 1 region but can't resolve its shape (L2 concentration at fixed ε prevents directional discrimination). To measure the shape directly, we binary-search for the boundary radius r(u) along 500 random directions u ∈ S¹⁹ from HKO2024.

### How to run

```bash
cd experiments/
cargo run --release --bin lagrangian_probe
python3 lagrangian-search/analyze.py
```

### Files

| File | Role |
|------|------|
| `probe.rs` | Rust binary: directional binary search along 500 random rays |
| `lagrangian-probe.jsonl` | Per-direction data (500 rows: direction vector, boundary radius) |
| `lagrangian_probe_radii.png` | Histogram of boundary radii |

### Findings

All 500 directions probed successfully (bisection to tolerance 1e-4). Verified against `lagrangian-probe.jsonl`.

**1. The boundary is anisotropic with 7× aspect ratio.**

| Stat | Value |
|---|---|
| Mean radius | 0.0941 |
| Median radius | 0.0901 |
| Std | 0.0290 |
| CV | 0.309 |
| Min | 0.0432 |
| Max | 0.3033 |
| Ratio max/min | 7.01 |
| Per-component (mean/√(20/3)) | 0.0364 |

The per-component radius 0.036 matches ε* ≈ 0.035 from the fraction-vs-ε curve — the two measurements are consistent.

**2. The distribution is right-skewed.** Most directions have radii in [0.06, 0.11] (P10-P75), with a long tail extending to 0.30. The region is compact in most directions but has a few "peninsulas" where the boundary extends 3× further than the median.

| Percentile | Radius |
|---|---|
| P5 | 0.059 |
| P25 | 0.074 |
| P50 | 0.090 |
| P75 | 0.106 |
| P95 | 0.150 |

**3. The anisotropy is not attributable to known geometric directions.**

- Scaling direction: r = 0.114 correlation with radius (3.8% R²)
- Joint rotation direction: r = 0.151 correlation (marginal effect)
- All 20 direction components together: R² = 0.066

Neither scaling, rotation, nor any linear combination of perturbation components explains the directional variation. The radius function r(u) appears "unstructured" — the variation comes from the combinatorial orbit structure (which facet sequence is optimal changes along different rays), not from any smooth geometric feature.

### Interpretation

The sys > 1 region around HKO2024 is a **compact, anisotropic body** in 20D Lagrangian perturbation space:
- Median radius 0.09 in L2 (per-component ~0.035)
- Right-skewed: compact in most directions, with sparse long-range extensions (up to 0.30)
- 7× aspect ratio, but no interpretable dominant direction
- Shape likely determined by the combinatorial structure of degenerate minimum-action orbits at HKO2024, not by smooth geometry
