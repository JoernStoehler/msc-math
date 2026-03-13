# Experiment Ideas

Ongoing thoughts, ideas, edge cases, preliminary findings.

## Symplectic classification of simplices (Fickel, Cieliebak)

**Source:** https://github.com/konstantinfickel/symplecticsimplices

Bachelor thesis (Konstantin Fickel, supervised by Cieliebak at Augsburg). Main result: two 2n-simplices are equivalent under affine symplectomorphism iff their 2-dimensional subsimplices have identical symplectic areas. Generalizes a prior n=2 result.

**Why interesting:**
- Lemmas and the main result may be directly useful for experiments that reduce polytopes modulo Sp(4) action
- The symplectic classification via 2-face areas is a computationally tractable invariant
- Could enable equivalence-class reduction: instead of sweeping all polytopes, sweep Sp(4)-orbits

**Status:** Not yet read in detail. Worth a close look for reusable lemmas.
**Update (2026-03-13):** Jörn notes that for random sampling, equivalence classes and raw
polytopes provide roughly the same diversity — the probability distributions differ but not
enough to matter. Deprioritized unless a specific use case arises.

## HKO2024 local neighborhood (research direction)

**Goal:** Determine whether HKO2024 is a local maximum of the systolic ratio. This would be
a major, potentially publishable result.

**Key insight:** HKO2024 lives in multiple ambient spaces simultaneously:
- Lagrangian products LP(Fq=5, Fp=5) — pentagon-perturb explores this
- Lagrangian products LP(Fq=6, Fp=5) — HKO2024 as degenerate (one q-facet collapsed)
- General polytopes F=10 — sys-optimization gradients cover this
- General polytopes F=13 — HKO2024 as degenerate (3 facets collapsed)
- Convex bodies — polytopes as discretization of smooth boundary

Each embedding gives different perturbation directions. Local max in one space ≠ local max
in a larger space. See pentagon-perturb/README.md and sys-optimization/NOTES.md for
experiment-specific ideas.

**Existing data to audit first:**
- sys-optimization Phase 1: does ∇sys point inward at HKO2024?
- pentagon-perturb: does sys decrease in all sampled directions?
- sys-optimization Phase 3: does iterative ascent starting from HKO2024 improve sys?

**New experiments needed:**
- Facet-splitting perturbations (F=11 ambient space)
- F-refinement convergence (increasing F as smooth approximation)
- Dense 2D slice through perturbation space for boundary mapping

**What the evidence would inform:** How to structure a proof — where tight bounds come from,
what equalities/symmetries must be handled exactly vs. via inequalities, what shape
perturbation theory on polytopes takes empirically.
