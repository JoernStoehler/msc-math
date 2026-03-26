# Lagrangian Search: Logbook

## Motivation

HKO2024 is the only known polytope with sys > 1. It's a Lagrangian product of two regular pentagons at θ=18°. How special is this? Is the sys>1 region a tiny needle in Lagrangian product space, or is it accessible by random sampling or gradient optimization? And: is HKO the only sys>1 Lagrangian product, or are there others?

## Status

**Not started.** Logbook scaffolded 2026-03-26.

## Research questions

### Phase 1: Feasibility estimate

What fraction of random Lagrangian products have sys > 1?

Existing data points:
- LP(5,5) rotation sweep (lagrangian-products experiment): ~28% of rotations of *regular* pentagons have sys > 1 (θ ∈ ~13°–23° of the 36° fundamental domain).
- random-product-sweep: 100 random products with 3≤k≤m≤6, max sys = 0.794. None near 1.

These suggest regularity of the pentagon matters a lot, not just the polygon count. The question: what's the volume ratio of {sys > 1} to the full LP parameter space? If it's < 1e-4, brute-force sampling won't find it. If it's ~ 1%, a few thousand samples might.

Sample random LP(n,m) products (random polygon shapes, random rotation) for various (n,m). Focus on LP(5,5) but also test neighboring pairs.

### Phase 2: Guided search (contingent on Phase 1)

Does gradient ascent from random Lagrangian products find trajectories to the sys > 1 region? How large is HKO's basin of attraction under gradient ascent?

gradient-descent found best sys = 0.905 from 501 Lagrangian starts (within-cell only). With boundary crossing, maybe some trajectories reach HKO's basin. But if Phase 1 shows the sys > 1 region is extremely small, this may not be worth running at scale.

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
- **pentagon-perturb** — 100 perturbations of HKO2024, all retain sys > 1. Shows the sys > 1 region has nonzero volume around HKO.
- **gradient-descent** — 501 Lagrangian products optimized, best sys = 0.905.
