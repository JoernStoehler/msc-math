# HK-O Pentagon Perturbations

Perturb the HK-O pentagon counterexample and study the distribution of systolic ratios in its neighborhood.

## Status
Complete

## Design

- Base polytope: `known_polytopes::hko_pentagon()` (10 facets)
- Perturbation: uniform noise per component
  - Normals: each component in [-0.01, 0.01], then renormalized to unit length
  - Heights: each component in [-0.01, 0.01]
- Samples: 100 perturbed + 1 unperturbed baseline
- Algorithm: HK2017 pruned only

## Key findings

- **All 100 perturbations retain sys > 1** (min 1.002, max 1.033, mean 1.021)
- Base (unperturbed) sys = 1.047 is highest, confirming HKO2024 is a local maximum
- Standard deviation of sys across perturbations is 0.006 (tight cluster)
- PCA on the 50D perturbation vector shows no dominant direction; top 5 components each explain ~5% of variance

## Files

| File | Purpose |
|------|---------|
| `pentagon_perturb.rs` | Rust binary: generates perturbed polytopes and computes sys |
| `pentagon_perturb.py` | Python: histogram of sys values, stats and PCA tables |
| `pentagon-perturb.jsonl` | Dataset (101 rows: 100 perturbed + 1 baseline) |
| `pentagon-perturb.tex` | Thesis writeup |
| `pentagon_perturb_sys_hist.png` | Figure: histogram of systolic ratios |
| `pentagon_perturb_stats.md` | Summary statistics table (markdown) |
| `pentagon_perturb_stats.tex` | Summary statistics table (LaTeX) |
| `pentagon_perturb_pca.md` | PCA component table (markdown) |
| `pentagon_perturb_pca.tex` | PCA component table (LaTeX) |

## Run

```bash
cd experiments/ && cargo run --bin pentagon_perturb --release
python experiments/pentagon-perturb/pentagon_perturb.py
```

## Known limitations

- Perturbation amplitude is small (epsilon=0.01); larger amplitudes are untested
- Random perturbations only; directed perturbations along sys-gradient not implemented
- Does not test facet-splitting (F=11+ directions)

## Ideas

### Larger perturbation amplitudes
Current perturbation is epsilon=0.01 per component. Larger amplitudes (epsilon=0.05, 0.1) would
explore further from HKO2024 in the Lagrangian-product ambient space, at the cost of
some perturbations breaking the polytope structure.

### Directed perturbations along sys-gradient
sys-optimization computes d(sys)/dh and d(sys)/dn analytically. Instead of uniform random
perturbations, perturb along the gradient (and orthogonal to it) to map the sys landscape
directionally.

### Facet-splitting: HKO2024 as degenerate F=11+ polytope
Splitting one facet into two and perturbing explores a direction invisible
in the LP ambient space. This tests whether HKO2024 is a local max in the larger ambient
space of general polytopes.

### Dense sampling for boundary mapping
For a 2D slice through perturbation space (e.g., two PCA directions), dense sampling
(1000-10000 points) could map the sys=1 level set.
