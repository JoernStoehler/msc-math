# Handoff: Dense 2D Slice Experiment

Map the sys≈1 level set around HKO2024 in a 2D parameter slice. Visually compelling thesis figure; strengthens the "HKO is isolated" story.

## Goal

Produce a heatmap/contour plot of sys(θ₁, θ₂) in a 2D slice through HKO2024, showing where sys crosses 1. The 1D precedent (lagrangian-products rotation sweep, 37 points) already shows a clear sys>1 region around θ=18° — extending to 2D would reveal the shape of that region.

## Design question (needs decision before implementing)

**Which 2D slice?** Options:

1. **(θ, shape parameter) in LP(5,5)** — rotation angle θ (as in lagrangian-products) × one shape deformation of the pentagon (e.g., vertex displacement magnitude). Most interpretable. Extends the 1D rotation sweep naturally.
2. **Two PCA directions from pentagon-perturb data** — data-driven, captures most variance. But pentagon-perturb found no dominant direction (top PCA components only 2.5-3× uniform), so may not be informative.
3. **Two dual-vertex components** — simplest to implement (perturb a₁[0] and a₁[1]), but hard to interpret geometrically.

Draft recommendation: option 1. The rotation angle is already well-understood from lagrangian-products, and adding a shape parameter would reveal whether the counterexample region is a narrow ridge or a broad plateau.

## Draft plan

1. **Read** lagrangian-products/run.rs and pentagon-perturb/run.rs to understand existing parameterization
2. **Design** the 2D parameterization — define the two axes precisely, decide grid resolution
3. **Implement** run.rs: nested loop over 2D grid, evaluate sys at each point, write JSONL
4. **Implement** analyze.py: contour plot of sys, mark sys=1 level set, mark HKO2024 location
5. **Run** at 50×50 resolution first (~2500 evals, ~1-2 min at ~25ms/eval for LP products)
6. **Assess** whether finer resolution or different slice is needed

## Draft verification

- sys at HKO2024 grid point should match known value (≈1.0472)
- Slice along θ axis (fixing shape=0) should reproduce lagrangian-products 1D sweep
- Grid points far from HKO should have sys < 1 (consistent with pentagon-perturb findings)

## Compute estimate

- 50×50 grid: ~2500 evals × 25ms ≈ 1 min (billiard algorithm on LP products)
- 200×200 grid: ~40k evals × 25ms ≈ 17 min
- Both fine locally, no LICCA needed.

## Risk

The interesting structure might not be visible in the chosen 2D slice. If the sys>1 region is very narrow in one direction, a coarse grid might miss it. Start coarse, refine adaptively if needed.
