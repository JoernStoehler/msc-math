# Research Questions

Central index mapping research questions to experiments.
Answers live in experiment logbooks, not here.

## 1. Is HKO2024 a local maximum?

HKO2024 is the only known counterexample to Viterbo's conjecture (sys ≈ 1.0472).
Is it a local maximum of sys in various polytope spaces?

- **1a.** First-order improving direction in F=10 (n,h)-space?
  → [gradient-analysis](crates/exp-hko-local-maximum/gradient-analysis/) (analytical gradients, sensitivity)
- **1b.** Can adding a facet (F=10 → F=11) improve sys?
  → [facet-splitting](crates/exp-hko-local-maximum/facet-splitting/)
- **1c.** LP test: 0 ∈ conv(subdifferential) in 40D parameter space?
  → [subdifferential-lp](crates/exp-hko-local-maximum/subdifferential-lp/)
- **1d.** Shape of the sys>1 region around HKO2024 in LP space?
  → [lagrangian-boundary](crates/exp-hko-local-maximum/lagrangian-boundary/)
- **1e.** Second-order behavior along 16 flat directions?
  → [NOT YET TESTED]

## 2. Are there novel sys>1 polytopes beyond HKO2024's neighborhood?

The sys=1 threshold matters — this is about Viterbo's conjecture specifically.

- **2a.** Which Lagrangian products of regular polygons achieve sys>1?
  → [rotation-sweep](crates/exp-sys-landscape/rotation-sweep/)
- **2b.** Can gradient ascent from random starts reach sys>1?
  → [boundary-crossing-search](crates/exp-sys-landscape/boundary-crossing-search/)
- **2c.** Structurally different sys>1 polytope near HKO2024?
  → [perturbation-neighborhood](crates/exp-sys-landscape/perturbation-neighborhood/)

## 3. How does sys behave as a function on polytope space?

No privileged threshold — sys is studied as a continuous function.

- **3a.** Structural features correlated with high sys?
  → [omega-hypothesis](crates/exp-sys-landscape/omega-hypothesis/) (falsified: small symplectic area not the mechanism)
- **3b.** Behavior across combinatorial type boundaries?
  → [combinatorial-profiling](crates/exp-sys-landscape/combinatorial-profiling/),
    [combinatorial-anatomy](crates/exp-sys-landscape/combinatorial-anatomy/),
    [combinatorial-convexity](crates/exp-sys-landscape/combinatorial-convexity/),
    [combinatorial-sweep](crates/exp-sys-landscape/combinatorial-sweep/)
- **3c.** sys distribution for random polytopes?
  → [random-sweep](crates/exp-sys-landscape/random-sweep/),
    [random-product-sweep](crates/exp-sys-landscape/random-product-sweep/),
    [rejection-sampling](crates/exp-sys-landscape/rejection-sampling/)
- **3d.** Following sys gradients?
  → library `derivatives.rs`; validated by [basic-validation](crates/dev-gradient-validation/basic-validation/),
    [edge-cases](crates/dev-gradient-validation/edge-cases/),
    [subdifferential](crates/dev-gradient-validation/subdifferential/)

## 4. How do we compute capacity better?

Speed, accuracy, trustworthiness, introspection of the computational tools.
Instrument development — results promote to `crates/library/`.

- **4a.** Which algorithm variant is fastest?
  → [ablation](crates/dev-algorithm-comparison/ablation/),
    [benchmark](crates/dev-algorithm-comparison/benchmark/),
    [profiling](crates/dev-algorithm-comparison/profiling/)
- **4b.** Numerical error bounds?
  → [error-bounds](crates/dev-numerical-analysis/error-bounds/),
    [q-error](crates/dev-numerical-analysis/q-error/),
    [kkt-inertia](crates/dev-numerical-analysis/kkt-inertia/),
    [unknown-predicates](crates/dev-numerical-analysis/unknown-predicates/)
- **4c.** Capacity satisfies known axioms?
  → [correctness](crates/dev-capacity-validation/correctness/),
    [orbit-recovery](crates/dev-capacity-validation/orbit-recovery/)
- **4d.** Analytical gradients match finite differences?
  → [basic-validation](crates/dev-gradient-validation/basic-validation/),
    [edge-cases](crates/dev-gradient-validation/edge-cases/),
    [subdifferential](crates/dev-gradient-validation/subdifferential/)
