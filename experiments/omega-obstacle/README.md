# Omega-Obstacle Experiment

Do near-Lagrangian 2-faces (small |omega_0(n_i, n_j)| between adjacent facets) act as "obstacles" that help create high systolic ratios?

## Status
Complete

**Mechanism tested**: Q(β) = Σ β_i β_j ω₀(...), capacity = 1/(2·max Q), sys = c²/(2V). If ω values are small, Q is smaller → capacity is larger → sys could be larger.

## Dataset

- 950 random polytopes: 200 each for F∈{5,6,7,8}, 100 for F=9, 50 for F=10
- Seed 42, h ∈ [0.8, 1.2], ChaCha8Rng
- Plus 3 known polytopes: HKO pentagon (sys=1.047), simplex (sys=0.75), hypercube (sys=0.5)
- Total: 953 polytopes, generated in ~8s (release mode)

## Key findings

### 1. Ridge min|ω| vs sys: weak negative correlation (rho=-0.22)

Spearman rho = -0.22, p = 8e-12. Polytopes with a smaller global min|ω| tend to have higher sys. BUT:
- The scatter is very wide — many polytopes with min|ω| ≈ 0 also have low sys
- Likely confounded by facet count: more facets → more ridge pairs → more chances for a small |ω| by chance, AND more facets → higher sys (rho=+0.37)
- All three known polytopes sit at min|ω| = 0 (all have at least one Lagrangian ridge)

### 2. Orbit-specific ω features: NO correlation with sys

- orbit_omega_min vs sys: rho = -0.02, p = 0.61 (not significant)
- orbit_omega_mean vs sys: rho = +0.00, p = 0.99 (not significant)
- The ω values on the optimal orbit do NOT predict sys

### 3. Orbit prefers LARGE ω transitions (opposite of hypothesis)

Orbit ridges |ω|: median = 0.54, mean = 0.52
Non-orbit ridges |ω|: median = 0.36, mean = 0.40

The orbit preferentially uses facet transitions with LARGE |ω|. This makes sense: the orbit maximizes Q = Σ β_i β_j ω_{ij}, so it seeks large ω values.

### 4. Gradient analysis: no directional signal

⟨∇_{n_k} sys, ∇_{n_k} ω(n_k, n_i)⟩ for orbit facets:
- Median = +0.0006, fraction negative = 49.5%
- Essentially symmetric around zero — no evidence that sys-increasing directions coincide with ω-decreasing directions

The |ω| vs dot product scatter shows a "trumpet" shape: variance increases at small |ω|, but the distribution remains centered on zero.

### 5. Known polytopes

| Polytope | F | sys | orbit_omega_min | ridge_omega_abs_min | orbit_len |
|----------|---|-----|-----------------|---------------------|-----------|
| HKO pentagon | 10 | 1.047 | 0.000 | 0.000 | 6 |
| Simplex | 5 | 0.750 | 0.000 | 0.000 | 5 |
| Hypercube | 8 | 0.500 | 1.000 | 0.000 | 4 |

All three have ridge_omega_abs_min = 0 (at least one Lagrangian ridge). But the hypercube has orbit_omega_min = 1.0 (orbit avoids Lagrangian ridges) yet sys = 0.5 < 1.

## Interpretation

The hypothesis "small ω between adjacent facets → high sys" is **not confirmed** by this data:

1. The weak global correlation (rho = -0.22) is likely a confound with facet count
2. Orbit-specific omega features show zero correlation
3. The orbit actually prefers LARGE omega transitions (to maximize Q)
4. The gradient analysis shows no directional relationship

The HKO counterexample has Lagrangian ridges (ω = 0) on its orbit, but this appears to be a property of its construction as a Lagrangian product perturbation, not a general mechanism.

## Files

| File | Purpose |
|------|---------|
| `omega_obstacle.rs` | Rust binary: generates JSONL dataset |
| `omega_obstacle.py` | Python: analysis and plots |
| `omega-obstacle.jsonl` | Dataset (953 rows) |
| `omega_obstacle_*.png` | Figures (7 plots) |

## Run

```bash
cd experiments/
cargo run --bin omega_obstacle --release
python3 omega-obstacle/omega_obstacle.py
```

## Known limitations

- Correlation analysis is observational; confounds with facet count not fully controlled for
- Only 3 known polytopes included as reference points
- No causal analysis (gradient-based or interventional)
