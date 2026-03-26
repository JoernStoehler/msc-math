# Gradient Correctness: Logbook

## Motivation

The library provides analytical gradients ∂sys/∂a_k via `capacity_derivatives_a` and `volume_derivatives_a` (envelope theorem for capacity, chain rule through h=1/|a|, n=a/|a| for volume). These are used by multiple optimization and analysis experiments. This experiment validates the gradients under increasingly adversarial conditions to understand where they're reliable and where they break down.

The envelope theorem derivation assumes a unique action-minimizing orbit with strict complementarity. Real polytopes can violate or nearly violate these assumptions.

## Status

**Not started.** Logbook scaffolded 2026-03-26.

## Research questions

1. **Generic polytopes:** Does the analytical gradient match finite differences across directions in R^{4F}? What sampling strategy is needed — along gradient, random directions, coordinate-aligned? How does accuracy depend on FD step size and polytope dimension?

2. **Non-generic geometry:** Is the gradient correct for Lagrangian products (which have symmetry-degenerate orbits) and other polytopes with symmetry groups? What about polytopes where multiple orbits achieve near-identical action?

3. **Near-degeneracy:** What happens when the gap between the best and second-best orbit action is small? The envelope theorem requires a unique minimizer — how does the gradient behave as this assumption is approached?

4. **Redundant halfspaces:** If we introduce a halfspace that barely cuts the polytope (or doesn't cut it at all), the gradient with respect to that facet should be zero (or nearly so). Is it?

## Design notes

- A shared validation harness (analytical vs FD comparison, error metrics) should serve all phases.
- Phase 3 (near-degeneracy) likely needs instrumentation beyond just "different polytopes" — e.g., logging the action gap, tracking which orbit the solver picks on each FD perturbation.
- The existing `capacity_derivatives_a_fd` and `volume_derivatives_a_fd` in `crates/src/derivatives.rs` provide FD baselines.
- Existing FD validation in `derivatives.rs` tests only the hypercube. This experiment should cover a much broader range of polytopes.

## Predecessor experiments

This experiment supersedes the gradient validation aspects of:
- **sys-optimization** Phases 1, 2, 4 (sensitivity analysis, single steps, validity testing)
- Parts of **correctness** (which tests capacity axioms, not gradient correctness)

The optimization aspects of sys-optimization (Phase 3) move to a separate experiment.
