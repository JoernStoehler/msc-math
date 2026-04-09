# Results

<!-- How this file works:
Answers "what should the thesis say?" — not a repo inventory.
Sections group related takeaways by theme (methods, results, rejected, open, ...).
Within sections, nesting mixes support (evidence for a claim) with related sub-takeaways.
An item belongs if: (1) Kai needs it for "is thesis done?" (2) worth bragging about,
(3) impacts thesis structure, or (4) natural dependency cluster.
Stubs and aspirational items belong. Working-notes style. -->

Developed Methods:
- Capacity: three algorithms — hk2017 (general, exponential), billiard (Lagrangian products, polynomial), tube (symplectic, aspirational)
- Adjacency pruning: orders-of-magnitude speedup [cor:adjacency-pruning]; empirically equals full search on simple polytopes [cor:ridge-sufficiency, unverified]
- Orbit recovery from KKT dual [lem:base-point-recovery]
- Exact rational arithmetic; three-valued solver (certified/uncertain/infeasible)
- Analytical derivatives ∂c/∂a_k, ∂vol/∂a_k; gradient ascent on sys
- Random sampling: normals on S³ with rejection; Lagrangian products from random polygons
- Validation: capacity axioms, orbit recovery, literature matches pass; numerical errors negligible

Results:
- HKO2024 is a local maximum ...
  - Proven: ... for 10-facet polytopes (1st and 2nd order)
    - No improving direction exists [lem:first-order-necessary, lem:cone-equals-kernel]
    - All flat-direction curvatures strictly negative [prop:second-order-local-max] — awaiting Jörn verification
    - Random perturbations retain sys>1, none exceed sys ≈ 1.0472
  - Conjecture: ... for higher F
    - Facet-splitting (F→F+1): all decrease sys
    - Cut-and-ascent: converges back to HKO
    - Variable-F: most F=10 maxima improve at F=11, none reach sys>1
  - Speculation: ... for convex bodies (no evidence, but F-refinement consistent)
- No novel sys>1 examples found
  - Random sampling and gradient ascent: best results well below 1
  - sys>1 region: volume ~10^{-31} — random search hopeless
  - Among rotated regular products (3≤n≤m≤6, plus 7×7): only pentagon×pentagon exceeds 1
    - Pentagon×pentagon at θ=18° IS the HKO2024 polytope (sys ≈ 1.0472)
    - Violation region: θ ∈ (13°,23°), ~25% of fundamental domain
    - sys(θ) is periodic and symmetric [lem:rotation-fundamental-domain] — awaiting Jörn verification
    - Open: closed formula for sys(P₅ × R(θ)P₅) — Kai asked; symmetry reduces domain, formula open
    - 3×6, 4×4 reach sys≈1 at coarse resolution — finer sweeps warranted
    - Right isosceles triangle ×_L square: sys=1 exactly (Schlenk)
  - Bet: density 10^{-F} to 10^{-4F}, far beyond feasible
- Crosspolytope: c_EHZ = 4.0, sys = 3/4 — satisfies Viterbo (new: not previously computed)
  - Same capacity as hypercube — duality coincidence?
- Visualizing ∂K yielded no new insights

Rejected ideas:
- ML: gap between HKO and everything else suggests qualitatively different behavior
- Omega hypothesis falsified: no ω₀–sys correlation; orbit ridges prefer large ω₀ (opposite)
- Direction-filtered subdifferential: negative result
- Richer regression data: nothing to add
- More compute: density 10^{-F} or worse, far beyond feasible

Open Ideas:
- Analytical formula for sys(P₅ × R(θ)P₅) — Kai asked; symmetry lemma reduces domain, formula open
- Crosspolytope optimality proof (m≥14 cannot improve c_EHZ=4.0)
- F-refinement convergence (increasing F → convex-body sys?)
- Convex-body direction (Minkowski smoothing K + εB⁴)
- Structural explanation for 0 ∈ conv(gradients) at HKO (golden ratio? C₅×Z₂?)
- Massive random search on LICCA
- Systematic landscape analysis (gradient flow, local maxima distribution)
- Dimension scaling (max sys vs F)

Datasets:
- ~10^5 polytopes, up to F=16
- Seeds: generic, products, regular products (with rotation), literature
- Transformations: perturbations, gradient ascent (intermediate + final), extra facets

Quality:
- Proofs: algorithms rigorously proven
- Validation: axioms, literature, orbit recovery
- Numerics: error bounds proven, exact rational for combinatorial decisions

Project:
- Reproducibility: complete project reproducible from git
- Documentation: self-documenting with research notes
- Library: polished Rust library
- AI Agents: usage patterns documented
