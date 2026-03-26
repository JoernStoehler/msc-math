# Sys Search: Logbook

## Motivation

Can gradient-based optimization push sys past 1? Previous gradient experiments found best non-HKO sys ≈ 0.905 but were limited by the step-bound barrier (gradient ascent converges within each combinatorial cell but cannot cross boundaries). This experiment combines within-cell ascent with boundary-crossing strategies.

## Status

**Not started.** Logbook scaffolded 2026-03-26.

## Research questions

1. **Single-step characterization:** Given a gradient direction, what step sizes work? How conservative is the step bound? What happens at various fractions and multiples of t_max?

2. **Multi-step search:** From diverse starting points, run gradient ascent with boundary-crossing. Does any trajectory reach sys > 1? What's the distribution of final sys values?

3. **Strategy comparison:** How do different boundary-crossing approaches compare — overshoot (various multipliers), wiggle (random perturbation), facet-splitting? What success rates and improvement rates does each achieve?

4. **General vs Lagrangian:** Do Lagrangian products behave differently from general polytopes under optimization? (Previous experiments suggest Lagrangian products reach higher sys.)

## Design notes

- Starting points should include both random polytopes and converged points from within-cell ascent (the latter are where boundary-crossing matters).
- The single-step phase and multi-step phase share infrastructure (gradient computation, step-bound, polytope construction). Natural to have both in one binary.
- Previous experiments used F≤10 due to HK2017 cost. Consider whether billiard-only runs on Lagrangian products at higher F are worth including.

## Predecessor experiments

Supersedes:
- **gradient-descent** — gradient ascent on 1001 F=10 polytopes, within-cell only
- **sys-optimization** Phase 3 — iterative ascent on 140 polytopes
- **gradient-search** — overshoot + wiggle on 20 seeds (smoke test)

## Related experiments

- **gradient-correctness** — validates the gradient this experiment relies on
- **combinatorial-boundaries** — characterizes the boundaries this experiment crosses
