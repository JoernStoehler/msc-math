# Near-Lagrangian 2-Faces and Systolic Ratio: Logbook

## Motivation

The HKO counterexample is a perturbed Lagrangian product whose optimal orbit traverses ridges with omega_0(n_i, n_j) = 0 (Lagrangian ridges). This motivates the hypothesis: do near-Lagrangian ridges (small |omega_0(n_i, n_j)| between adjacent facets) act as "obstacles" that help create high systolic ratios?

The mechanism: Q(beta) = sum beta_i beta_j omega_0(...), where Q is maximized subject to KKT constraints, capacity = 1/(2 max Q), sys = c^2/(2V). If omega values are small, Q is smaller, capacity is larger, and sys could be larger.

## Status

**Complete.** Hypothesis not confirmed. Four independent tests (ridge correlation, orbit-specific features, orbit vs non-orbit distribution, gradient analysis) all fail to support the hypothesis.

## How to run

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-omega   # generates omega-obstacle.jsonl
cd experiments/combinatorial-cells/omega-hypothesis/
uv run analyze.py                        # generates all figures
```

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: generates JSONL dataset with omega features and gradients |
| `analyze.py` | Python: correlation analysis and figure generation |
| `math.tex` | Formal writeup (hypothesis, Phase A/B, conclusion) |
| `omega-obstacle.jsonl` | Dataset (953 rows) |
| `omega_obstacle_ridge_min_vs_sys.png` | Ridge min\|omega\| vs sys scatter plot |
| `omega_obstacle_orbit_min_vs_sys.png` | Orbit min\|omega\| vs sys scatter plot |
| `omega_obstacle_orbit_mean_vs_sys.png` | Orbit mean\|omega\| vs sys scatter plot |
| `omega_obstacle_orbit_vs_nonorbit.png` | Orbit vs non-orbit \|omega\| distribution |
| `omega_obstacle_gradient_dots.png` | Gradient dot product distribution |
| `omega_obstacle_gradient_neighbor_split.png` | Gradient dot products split by neighbor type |
| `omega_obstacle_omega_vs_dot.png` | \|omega\| vs gradient dot product (trumpet shape) |

## Design

### Dataset

- 950 random polytopes: 200 each for F in {5, 6, 7, 8}, 100 for F = 9, 50 for F = 10.
- Seed 42, h in [0.8, 1.2], ChaCha8Rng.
- 3 known polytopes as reference: HKO pentagon (sys = 1.047), simplex (sys = 0.75), hypercube (sys = 0.5).
- Total: 953 polytopes, generated in ~8s (release mode).

### Features computed

For each polytope: c_EHZ via instrumented HK2017 (returns optimal beta, lambda, nu, Q from KKT solve), geometric skeleton (ridge adjacency via shared 2-faces), and omega_0(n_i, n_j) for every ridge pair.

### Two-phase analysis

- **Phase A (observational):** Correlate omega_0 features with sys across the dataset. Features: ridge min|omega|, orbit min|omega|, orbit mean|omega|, orbit vs non-orbit |omega| distributions.
- **Phase B (gradient):** Compute dot product of tangential gradients: d = <grad_{n_k} sys, grad_{n_k} omega_0(n_k, n_i)>. If d < 0 systematically, the sys-increasing direction decreases omega_0 (supporting hypothesis). Uses analytical derivatives from Corollary `cor:sys-derivative` and Lemmas `lem:vol-derivative-normal`--`lem:cap-derivative-normal`.

## Findings

1. **Ridge min|omega| vs sys: weak negative correlation (rho = -0.20, p = 6e-10).** Polytopes with smaller global min|omega| tend to have slightly higher sys. However, the scatter is very wide, and this is likely confounded by facet count: more facets produce more ridge pairs (more chances for small |omega| by chance), and facet count itself correlates positively with sys (rho = +0.37).

2. **Orbit-specific omega features: NO correlation with sys.** orbit_omega_min vs sys: rho = -0.01, p = 0.82 (not significant). orbit_omega_mean vs sys: rho = +0.00, p = 0.99 (not significant). The omega values on the optimal orbit do not predict sys.

3. **Orbit prefers LARGE omega transitions (opposite of hypothesis).** Orbit ridges |omega|: median = 0.54, mean = 0.52. Non-orbit ridges |omega|: median = 0.36, mean = 0.40. The orbit preferentially uses transitions with large |omega|, consistent with Q-maximization: the optimizer seeks large omega terms to maximize Q.

4. **Gradient analysis: no directional signal.** <grad_{n_k} sys, grad_{n_k} omega_0(n_k, n_i)> for orbit facets: median = +0.0002, fraction negative = 49.8%. Essentially symmetric around zero. The |omega| vs dot product scatter shows a "trumpet" shape (variance increases at small |omega|), but remains centered on zero.

5. **Why the hypothesis fails:** The KKT optimizer compensates — it adjusts beta* and selects orbits that maximize Q despite small individual omega_0 contributions. Small omega_0 values on individual ridges do not translate into small Q (or large capacity) because the optimizer redistributes weight across the orbit's transitions.

5. **Known polytopes:** All three have ridge min|omega| = 0 (at least one Lagrangian ridge). But the hypercube has orbit_omega_min = 1.0 (orbit avoids Lagrangian ridges) yet sys = 0.5. The HKO counterexample's Lagrangian orbit ridges are a consequence of its construction as a perturbed Lagrangian product, not a general mechanism.

## Known limitations

- Correlation analysis is observational; confounds with facet count not fully controlled for.
- Only 3 known polytopes as reference points.
- No causal/interventional analysis beyond the gradient dot-product test.

## Related experiments

- `lagrangian-products`: the Lagrangian product structure that motivates this hypothesis.
- `sys-optimization`: uses the same sensitivity infrastructure (gradient computations).
