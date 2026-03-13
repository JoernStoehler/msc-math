# HK-O pentagon perturbations

Goal: perturb the standard HK-O pentagon counterexample and study the distribution of systolic ratios.

## Dataset

- Generator: `experiments/pentagon-perturb/pentagon_perturb.rs`
- Output JSONL: `experiments/pentagon-perturb/pentagon-perturb.jsonl`
- Base polytope: `known_polytopes::hko_pentagon()` (10 facets)
- Perturbation: uniform noise per component
  - Normals: each component in [-0.01, 0.01], then renormalized to unit length
  - Heights: each component in [-0.01, 0.01]
- Samples: 100 perturbed + 1 unperturbed baseline
- Algorithm: HK2017 pruned only

## Outputs

- Script: `experiments/pentagon-perturb/pentagon_perturb.py`
- Histogram: `experiments/pentagon-perturb/pentagon_perturb_sys_hist.png`
- Stats table (markdown): `experiments/pentagon-perturb/pentagon_perturb_stats.md`
- Stats table (LaTeX): `experiments/pentagon-perturb/pentagon_perturb_stats.tex`
- PCA table (markdown): `experiments/pentagon-perturb/pentagon_perturb_pca.md`
- PCA table (LaTeX): `experiments/pentagon-perturb/pentagon_perturb_pca.tex`

The script prints the stats and PCA tables to stdout for quick copy into notes or LaTeX.

## PCA

We build a 50D perturbation vector per sample using $(\Delta n, \Delta h) \in \mathbb{R}^5$
for each facet (10 facets total). PCA is run on the centered perturbation matrix from
the 100 perturbed samples, and the table lists the top components with explained
variance ratio as the strength column.

## Ideas

### Larger perturbation amplitudes
Current perturbation is ε=0.01 per component. Larger amplitudes (ε=0.05, 0.1) would
explore further from HKO2024 in the Lagrangian-product ambient space, at the cost of
some perturbations breaking the polytope structure.

### Directed perturbations along sys-gradient
sys-optimization computes ∂sys/∂h and ∂sys/∂n analytically. Instead of uniform random
perturbations, perturb along the gradient (and orthogonal to it) to map the sys landscape
directionally. Would show whether HKO2024 is a local max in the LP(Fq=5,Fp=5) ambient space.

### Facet-splitting: HKO2024 as degenerate F=11+ polytope
HKO2024 (10 facets as Lagrangian product) can also be viewed as a degenerate member of
the F=11 or F=12 general polytope space — with some facets "collapsed" (height → 0 relative
to neighbors). Splitting one facet into two and perturbing explores a direction invisible
in the LP ambient space. This tests whether HKO2024 is a local max in the larger ambient
space of general polytopes. Requires: constructing the degenerate embedding, then perturbing
the new facet's normal and height away from degeneracy.

### Dense sampling for boundary mapping
100 samples gives a histogram but not a boundary. For a 2D slice through perturbation
space (e.g., two PCA directions), dense sampling (1000–10000 points) could map the
sys=1 level set, showing the shape of the counterexample region.

## Run

```bash
cd experiments/ && cargo run --bin pentagon_perturb --release
python experiments/pentagon-perturb/pentagon_perturb.py
```
