# IDEAS

Research directions and experiment ideas. See individual experiment logbooks for experiment-specific follow-ups.

**Other idea sources:**
- `crates/exp-algorithm-comparison/ablation/ideas-future.md` — pruning extensions (A1 vs F scaling, non-simple polytopes, UNKNOWN predicates, face lattice approach, exact skeleton via perturbation)
- Individual experiment logbook "Known limitations" sections

---

## HKO2024 local maximality

**Goal:** Show or strongly suggest that HKO2024 is a local maximum of the systolic ratio in the space of convex bodies. Would be a publishable result.

**Key insight:** HKO2024 lives in multiple ambient spaces simultaneously:
- Lagrangian products LP(5, 5) — pentagon-perturb explores this
- Lagrangian products LP(6, 5) — HKO2024 as degenerate (one q-facet collapsed)
- General polytopes F=10 — sys-optimization gradients cover this
- General polytopes F=13 — HKO2024 as degenerate (3 facets collapsed)
- Convex bodies — polytopes as discretization of smooth boundary

Each embedding gives different perturbation directions. Local max in one space ≠ local max in a larger space.

**Existing data to audit first:**
- sys-optimization Phase 1: does ∇sys point inward at HKO2024?
- pentagon-perturb: does sys decrease in all sampled directions?
- sys-optimization Phase 3: does iterative ascent starting from HKO2024 improve sys?

**New experiments needed:**
- Facet-splitting perturbations (F=11 ambient space)
- F-refinement convergence (increasing F as smooth approximation)
- Dense 2D slice through perturbation space for boundary mapping

---

## Regular Lagrangian product analysis

Dense (n, m, θ) sweep across wide range. Fit sys(n, m, θ) formula. Key question: does the fitted formula predict sys>1 only for P_5 ×_L R(θ) P_5?

**Sub-ideas:**

1. **Fit to sys(n-gon × rotated m-gon) data.**

2. **Analytical proof of the fitted formula for 5-gon × rotated 5-gon.** Approach: first discuss which orbit(s) can have the minimum action and give formulas for each, and solve, and then prove which exact (θ-parameterized) value is the smallest. Guess that motivates why this is easy: there should be 1 combinatorics (+symmetries) that is minimum for θ ≠ 0 mod symmetry.

---

## Massive random search on LICCA

**Sub-ideas:**

1. **Massive random search on LICCA for random polytopes** with gradient descent and combinatoric-changing step sizes / non-GD-algorithms.

2. **Massive random search but for Lagrangian products** (where we know that the density of getting close to HKO2024 is no longer negligible).

3. **Random search in the neighborhood of HKO2024**, of n-gon × m-gon products with sys = 1.

---

## sys(K) landscape analysis

Analyze the sys(K) landscape. K is a high-dimensional (stratified in F) space and sys is non-smooth, but we can randomly sample and look at gradients and follow gradient steps.

**Example questions:**
- Whether gradient flow takes two polytopes K₁, K₂ and moves them together or not
- Whether local maxima exist at sys < 1
- Whether all our flow methods get super slow at some points and why and whether random noise helps
- etc.

I doubt that symmetry considerations matter, even Sp(4) or symplectic-product-preserving non-linear symplectomorphisms is low-dimensional relative to the K space.

---

## Subdifferential analysis of sys(HKO2024)

Correctly analyze the subdifferentials of sys(HKO2024). Sample along any directions with first order equal zero to see what sign the second order has in that direction.

Sadly ∂²sys(K)/∂K² would be too high-dimensional and require new methods, but ∂ₓ∂ₓ sys(K) in one direction x rather two directions x₁, x₂ may be tractable and not require much new theory (i.e. it is still done by looking at the second-order derivatives of every orbit with minimum action).

---

## Symplectic classification of simplices

**Source:** Fickel (supervised by Cieliebak, Augsburg). Two 2n-simplices are equivalent under affine symplectomorphism iff their 2-dimensional subsimplices have identical symplectic areas.

**Why interesting:**
- May be useful for experiments that reduce polytopes modulo Sp(4) action
- Could enable equivalence-class reduction for sweeps

**Status:** Deprioritized (2026-03-13). Jörn notes that for random sampling, equivalence classes and raw polytopes provide roughly the same diversity.

---

## Experiment findings summary (2026-03-22)

Reference for what results exist. See individual logbooks for full details.

| Experiment | Finding |
|---|---|
| **crosspolytope** | First computed c_EHZ for 4D crosspolytope: c=4.0 (same as hypercube, its dual), sys=0.75 |
| **hko-neighborhood** | Evidence HKO2024 is local max: gradient ascent converges in 1 step, all facet-splitting cuts decrease sys |
| **lagrangian-products** | HKO is only sys>1 among regular polygon pairs (3≤n,m≤6, 6° resolution) |
| **pentagon-perturb** | All 100 random perturbations retain sys>1 (min 1.002, max 1.033). HKO at 1.047 is highest |
| **gradient-descent** | 995 polytopes, none reach sys>1. Step-bound barrier at combinatorial type boundary |
| **sys-optimization** | Best sys=0.878 from 140 starts. Combinatorial type boundary is binding constraint |
| **omega-obstacle** | Hypothesis that small abs(ω₀) increases sys **falsified**. Zero correlation (ρ=-0.02) |
| **random-sweep** | 70 random polytopes (F=5-12), max sys=0.578. Random polytopes stay far from violation |
| **random-product-sweep** | Random Lagrangian products max sys=0.794 (6×6). HKO requires specific rotation angle |
| **ablation** | All 4 pruning variants agree. A2 speedup exponential (~1078x at F=10) |
| **correctness** | All 6 mathematical axioms pass |
| **kkt-inertia** | Eigenvalue inertia formula holds for 6/7 polytopes. 5 mismatches are threshold artifacts |
| **orbit-recovery** | 112/112 polytopes pass. Errors <1e-6 for F≤9 |
| **q-error** | 1.13M nodes, worst E=2.9e-11. Empirically exact at f64 |
| **unknown-predicates** | 29 UNKNOWNs, all Lagrangian products, all f64 noise |
| **benchmark** | Construction dominates for F≤10 (80-92%). Practical limit F≤12 |

**Blockers for new experiment work:**
- Library has no derivative API (∂c_EHZ/∂a_i) — see TASKS.md `dual-vertex-parameterization`
- Experiment gradient code is in (n, h) space, needs rewriting after a_i-only KKT formulation
