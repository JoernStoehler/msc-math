---
name: experiment-design
description: Turn a research question into an actionable experiment. Use when starting a new experiment or tackling a new research question — not for bug fixes, data regeneration, or narrow implementation tasks.
user-invocable: true
---

# Experiment Design

## Steps

1. **Formalize the question** as a testable mathematical property. State what observation would confirm it and what would falsify it.
2. **Generate 2-4 candidate methods.** For each: what it measures, what it assumes, what failure modes it would miss.
3. **Present to Jörn.** One line per method. Jörn picks. Do not implement before this step.
4. **Track epistemic status** during implementation: observation (raw data), assumption (method requires), inference (data + assumptions), hypothesis (speculation beyond data).

## Common method families in this project

- **First-order prediction:** Perturb by td, check f(x+td) − f(x) − t·g·d = o(t). Tests defining property of (sub)gradient.
- **Known closed-form:** Verify on polytopes with analytic answer. Strong but limited to available forms.
- **Convergence rate:** Error decreases at predicted rate as parameter varies. Catches asymptotic bugs, misses constant factors.
- **Cross-implementation:** Two independent computations of same quantity. Catches coding bugs, not shared conceptual errors.
- **Property-based:** Test invariance, symmetry, monotonicity. Catches structural bugs, doesn't verify values.
