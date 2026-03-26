# Gradient Correctness: Logbook

## Motivation

The library provides analytical gradients ∂sys/∂a_k via `capacity_derivatives_a` and `volume_derivatives_a`. These are used by multiple optimization and analysis experiments. This experiment validates the gradients under increasingly adversarial conditions to understand where they're reliable and where they break down.

## Status

**Not started.** Logbook scaffolded 2026-03-26.

## Research questions

1. **Generic polytopes:** Does the analytical gradient match finite differences? What sampling strategy works in R^{4F} — along gradient, random directions, coordinate-aligned? How does accuracy depend on FD step size and polytope dimension?

2. **Non-generic geometry:** Is the gradient correct for Lagrangian products (which have symmetry-degenerate orbits) and other polytopes with symmetry groups? What about polytopes where multiple orbits achieve near-identical action?

3. **Near-degeneracy:** What happens when the gap between the best and second-best orbit action is small? How does the gradient behave as the minimizer becomes non-unique?

4. **Redundant halfspaces:** If we introduce a halfspace that barely cuts the polytope, what happens to the gradient for that facet?

## Design notes

- A shared validation harness (analytical vs FD comparison, error metrics) should serve all phases.
- Question 3 likely needs instrumentation beyond just "different polytopes" — e.g., logging the action gap, tracking which orbit the solver picks on each FD perturbation.
- Existing `capacity_derivatives_a_fd` and `volume_derivatives_a_fd` in `crates/src/derivatives.rs` provide FD baselines. Currently tested only on the hypercube.
- Check the derivative lemmas in `experiments/sys-optimization/math.tex` and `crates/src/derivatives.rs` for what assumptions the formulas rely on.

## Predecessor experiments

This experiment supersedes the gradient validation aspects of sys-optimization (Phases 1, 2, 4). The optimization aspects of sys-optimization (Phase 3) move to a separate experiment.
