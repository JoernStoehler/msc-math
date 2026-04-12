# HK-O Pentagon Perturbations: Logbook

## Motivation

Viterbo's conjecture was disproved by Haim-Kislev and Ostrover (2024) via an explicit 10-facet pentagon counterexample with sys > 1. This experiment probes the local stability of that counterexample: does sys remain above 1 under small geometric perturbations, or is the counterexample an isolated point?

## Status

**Complete.** 100 random perturbations generated and analyzed. All retain sys > 1.

## How to run

```bash
cd crates/ && cargo run -p exp-hko-local-maximum --release --bin hko-perturbation
python analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: generates perturbed polytopes and computes sys |
| `analyze.py` | Python: histogram of sys values, summary stats, and PCA tables |
| `math.tex` | Formal writeup (input'd from `thesis/experiments.tex`) |
| `pentagon-perturb.jsonl` | Dataset: 101 rows (100 perturbed + 1 baseline) |
| `pentagon_perturb_sys_hist.png` | Figure: histogram of systolic ratios |
| `pentagon_perturb_stats.tex` | LaTeX table: summary statistics |
| `pentagon_perturb_pca.tex` | LaTeX table: PCA components (commented out in math.tex) |

## Design

- **Base polytope:** `known_polytopes::hko_pentagon()` (10 facets, the HKO2024 counterexample)
- **Perturbation method:** Uniform noise per component of the 4D dual vertices
  - Each of the 4 components of each dual vertex a_i perturbed by delta ~ Uniform[-0.01, 0.01]
  - No renormalization; the perturbed vertices define the new polytope directly via `Polytope4D::from_f64`
- **Sample count:** 100 perturbed + 1 unperturbed baseline
- **Algorithm:** HK2017 pruned only
- **Seed:** 41
- **Max attempts per sample:** 2000 (to find perturbations yielding valid polytopes)

## Findings

All verified against `pentagon-perturb.jsonl` (101 rows).

1. **All 100 perturbations retain sys > 1.** The counterexample is robust under small perturbations.
2. **Summary statistics of perturbed sys:** min = 1.0142, max = 1.0385, mean = 1.0287, std = 0.0048.
3. **Base (unperturbed) sys = 1.0472 is highest**, indicating HKO2024 is a local maximum of sys in this perturbation space (among the 101 sampled points; not a proof of local maximality).
4. **PCA on the 40-dimensional perturbation vector** (4 components per facet x 10 facets) shows no dominant direction: top 5 components explain 6.51%, 6.07%, 5.79%, 5.40%, 4.96% of variance respectively. Uniform baseline would be 1/40 = 2.5% per component; the top components are 2-2.6x this, so there is mild anisotropy but no single dominant direction.

## Known limitations

- Perturbation amplitude is small (epsilon = 0.01 per component); larger amplitudes are untested.
- Only random perturbations; directed perturbations along the sys gradient are not tested here (see sys-optimization).
- Facet count is fixed at F = 10. Facet-splitting perturbations (F = 11+) are not explored.
- All perturbations preserve the Lagrangian product structure implicitly (normals stay near the original product structure). True general perturbations breaking LP structure are not tested.

## Dead ends / deferred directions

- **Larger perturbation amplitudes** (epsilon = 0.05, 0.1): would explore further from HKO2024 but risk breaking polytope structure. Deferred.
- **Directed perturbations along sys-gradient:** this idea evolved into the sys-optimization experiment instead.
- **Facet-splitting (F = 11+):** splitting one facet into two explores directions invisible in the LP ambient space. Tests whether HKO2024 is a local max in the larger ambient space. Deferred.
- **Dense 2D slice sampling:** 1000-10000 points in a 2D slice (e.g., two PCA directions) could map the sys = 1 level set. Not attempted.

## Related experiments

- **sys-optimization:** Computes analytical gradients of sys and takes directed steps. The "directed perturbation" idea from this experiment's ideation.
- **gradient-descent:** Scales up sys-optimization to ~1000 polytopes.
