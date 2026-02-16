# HK-O pentagon perturbations

Goal: perturb the standard HK-O pentagon counterexample and study the distribution of systolic ratios.

## Dataset

- Generator: `crates/datasets/src/bin/pentagon_perturb.rs`
- Output JSONL: `experiments/data/pentagon-perturb.jsonl`
- Base polytope: `known_polytopes::hko_pentagon()` (10 facets)
- Perturbation: uniform noise per component
  - Normals: each component in [-0.01, 0.01], then renormalized to unit length
  - Heights: each component in [-0.01, 0.01]
- Samples: 100 perturbed + 1 unperturbed baseline
- Algorithm: HK2017 pruned only

## Outputs

- Script: `experiments/scripts/pentagon_perturb.py`
- Histogram: `experiments/figures/pentagon_perturb_sys_hist.png`
- Stats table (markdown): `experiments/figures/pentagon_perturb_stats.md`
- Stats table (LaTeX): `experiments/figures/pentagon_perturb_stats.tex`
- PCA table (markdown): `experiments/figures/pentagon_perturb_pca.md`
- PCA table (LaTeX): `experiments/figures/pentagon_perturb_pca.tex`

The script prints the stats and PCA tables to stdout for quick copy into notes or LaTeX.

## PCA

We build a 50D perturbation vector per sample using $(\Delta n, \Delta h) \in \mathbb{R}^5$
for each facet (10 facets total). PCA is run on the centered perturbation matrix from
the 100 perturbed samples, and the table lists the top components with explained
variance ratio as the strength column.

## Run

```bash
cargo run --bin pentagon_perturb --release
python experiments/scripts/pentagon_perturb.py
```
