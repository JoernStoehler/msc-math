# Second-Order Analysis: Logbook

## Status

**Active.** Data generated, figures produced. All 15 flat directions have negative curvature — numerical evidence supports local maximality.

## Motivation

The first-order analysis (Phase C LP, `../subdifferential-lp/`) confirmed 0 ∈ conv(per-orbit ∇sys), but found flat directions where the directional derivative is zero for all active orbits. First-order stationarity is necessary but not sufficient for local maximality. This experiment determines the second-order behavior along each flat direction via finite differences.

Additionally, the Phase C LP script is broken after the a_i migration (reads `normals`/`heights` fields that no longer exist). This experiment replaces it with a clean a_i-space computation.

TASKS.md `hko-local-maximality` next step #1: "Second-order analysis of flat directions — compute sys along each flat direction via finite differences."

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-second-order
cd crates/exp-hko-local-maximum/second-order && python3 analyze.py
```

Total runtime: ~20 seconds.

## Files

| File | Role |
|---|---|
| `run.rs` | Rust binary: gradient matrix, SVD, FD curves |
| `analyze.py` | Phase C LP replacement + curvature analysis + figures |
| `second-order-base.jsonl` | SVD data, flat directions, gradient matrix (1 row) |
| `second-order-curves.jsonl` | sys(ε) for each flat direction (420 rows) |
| `second_order_curves.png` | sys(ε) − sys(0) curves for all 15 directions |
| `second_order_curvatures.png` | Bar chart of curvatures by direction |
| `second_order_curvatures.tex` | LaTeX table of curvature values |
| `math.tex` | Second-order sufficiency conditions |

## Methodology

### Phase 1: Gradient matrix and flat directions

1. Run instrumented capacity on HKO2024 → 150 near-optimal orbits (all with gap < 1.3e-15)
2. For each orbit: re-solve KKT to get multiplier μ, then compute ∇_{a_i} sys via `capacity_derivatives_a()` and `volume_derivatives_a()`
3. Stack into gradient matrix G (150 × 40)
4. SVD → rank 25, so 15 flat directions (null space of G)

The a_i parameterization (K = {x : a_i · x ≤ 1}) gives a clean R^40 space with no gauge directions, unlike the old (n,h) parameterization which had 10 gauge (radial) directions and produced 16 flat directions in R^40 effective space.

### Phase 2: Finite-difference curves

For each of the 15 flat directions d_j (unit vectors in R^40):
- Perturb a_k → a_k + ε·d_{j,k} for 14 positive ε values mirrored to negative (28 total)
- ε grid: 5e-5 to 4e-2 (fine near base, coarse near characteristic radius ~0.035)
- Compute sys = c²/(2·vol) at each perturbed polytope via standard `ehz_capacity()`

### Curvature measurement

Symmetric curvature ratio: r(ε) = (sys(+ε) + sys(-ε) − 2·sys(0)) / ε²

For a C² function, r(ε) = f''(0) + O(ε²). For piecewise-smooth sys (orbit switching), r(ε) varies with ε — we report the median over the fine+medium range (ε ≤ 5e-3).

## Results

### Phase C LP (a_i space)

- **0 ∈ conv(per-orbit ∇sys): FEASIBLE** (residual norm 5.8e-9)
- 26 active orbits (λ > 1e-12) out of 150
- 0 on boundary of conv (not interior) → flat directions exist
- Gradient rank: **25** (of 40) → **15 flat directions**
- Sharp rank boundary: σ[24] = 0.152, σ[25] = 1.57e-8

Compared to old (n,h)-space result: rank 24 in R^50 with 10 gauge → 16 flat in R^40 effective. The a_i result gives rank 25 → 15 flat. One flat direction was a gauge artifact.

### Curvature results

**All 15 curvatures are negative** (second-order-curves.jsonl, analyze.py output):

| Direction | Curvature | CV (fine range) |
|-----------|-----------|-----------------|
| 0 | −1.85e-2 | 2.22 |
| 1 | −2.36e-1 | 1.88 |
| 2 | −2.81e-1 | 1.07 |
| 3 | −9.28e-2 | 0.70 |
| 4 | −2.78e-1 | 1.47 |
| 5 | −3.09e-1 | 1.34 |
| 6 | −2.33e-1 | 0.82 |
| 7 | −1.78e-1 | 1.27 |
| 8 | −1.87e-1 | 0.47 |
| 9 | −1.19e-1 | 1.25 |
| 10 | −5.68e-2 | 0.33 |
| 11 | −1.60e-1 | 0.77 |
| 12 | −1.90e-1 | 0.20 |
| 13 | −7.49e-2 | 0.22 |
| 14 | −1.31e-1 | 0.36 |

CV (coefficient of variation of r(ε) over ε ≤ 5e-3) > 1 in several directions indicates orbit switching — the curvature ratio is not constant because different orbits dominate at different ε. The sign is consistently negative regardless.

Direction 0 has the smallest curvature magnitude (−0.018), about 17× smaller than the largest (direction 5, −0.309). Still clearly negative, not ambiguous.

### Negative definiteness check (Phase 3, 2026-04-04)

Measuring curvature along 15 orthonormal basis vectors of ker(G) proves the diagonal of the Hessian is negative in that basis, but does NOT prove the Hessian is negative definite — off-diagonal entries could produce positive eigenvalues.

Phase 3 samples 100 random unit directions in the 15D flat subspace (uniform on S^14, seed 42) and measures curvature at each via FD at ε ∈ {1e-4, 5e-4, 1e-3, 5e-3}. Results (second-order-random.jsonl):
- **100/100 negative**, 0 ambiguous, 0 positive
- Worst (most positive) curvature: −0.060
- Mean: −0.167, std: 0.052
- Runtime: ~41s (800 capacity evaluations)

Combined with the 15 basis directions, this gives strong numerical evidence for negative definiteness.

### Symmetry decomposition (2026-04-04)

The symplectic symmetry group G_symp = ⟨Δ₇₂°, φ⟩ ≅ C₅ × Z₂ (order 10, abelian) acts on R^40 by permuting dual vertices. HKO2024 is a fixed point. The flat subspace ker(G) is invariant under this action.

**Irreducible decomposition of the 15D flat subspace:**

| Sector | dim | Geometric meaning |
|--------|-----|-------------------|
| Δ=1, φ=+1 | 3 | Fully symmetric (includes uniform scaling) |
| Δ=1, φ=-1 | 2 | Differential q/p (breaks q↔p symmetry) |
| Δ=e^{±72°i} | 4 | Breaks C₅ rotational symmetry (2 copies of 2D irrep) |
| Δ=e^{±144°i} | 6 | Breaks C₅ rotational symmetry (3 copies of 2D irrep) |

**Uniform scaling** (a_i → λa_i for all i) lies entirely in the flat subspace: sys = c²/(2vol) is scale-invariant because capacity is degree-2 homogeneous and volume is degree-4 in R^4.

**Directions up to symmetry:** ≤10 distinct curvature classes. Within each 2D C₅-irrep, the rotation mixes all directions → one curvature value. The 3D and 2D trivial sectors have independently varying curvatures.

### Interpretation

HKO2024 satisfies:
1. **First-order necessary condition:** 0 ∈ conv(subdifferential) in R^40 (LP feasible)
2. **Second-order condition:** negative curvature along all 15 basis directions AND all 100 random directions in the flat subspace (worst: −0.060)

This constitutes strong numerical evidence for negative definiteness of the generalized Hessian on the 15D flat subspace, supporting that HKO2024 is a strict local maximum of sys among F=10 polytopes.

### Limitations

- Finite-difference curvatures have no rigorous error bounds
- Only tests flat directions in the fixed-F=10 parameter space — not facet-splitting (F=11) or convex-body perturbations
- Piecewise smoothness of sys means the "second derivative" is the curvature of the min-envelope, not a classical Hessian eigenvalue
- Random sampling on S^14 provides probabilistic, not exhaustive, coverage
