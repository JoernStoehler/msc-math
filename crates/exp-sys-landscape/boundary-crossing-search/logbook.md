# Sys Search: Logbook

## Motivation

Can gradient-based optimization push sys past 1? Previous gradient experiments found best non-HKO sys ≈ 0.905 but were limited by the step-bound barrier (gradient ascent converges within each combinatorial cell but cannot cross boundaries). This experiment combines within-cell ascent with boundary-crossing strategies.

## Status

**Development run complete (2026-03-26).** Pipeline works end-to-end. 42 seeds (10 general + 12 Lagrangian + 20 warm starts from gradient-descent). Best sys=0.933. No sys > 1 found. Ready for scale-up.

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

## How to run

```
cd crates/exp-sys-landscape/boundary-crossing-search/
cargo run --release --bin opt-boundary-crossing           # resume from existing data
cargo run --release --bin opt-boundary-crossing -- --fresh # rerun from scratch
python3 analyze.py                                         # generate figures + summary
```

### Files

| File | Role |
|------|------|
| run.rs | Single binary: generate polytopes + gradient ascent + overshoot + wiggle |
| analyze.py | Summary table, 5 figures |
| sys-search.jsonl | Per-seed summary (one row per polytope) |
| sys-search-trace.jsonl | Per-iteration trace (diagnostic) |
| sys_search_*.png | Figures |

## Design (2026-03-26)

Single binary combining gradient-descent's proven within-cell ascent with gradient-search's boundary-crossing strategies. Uses the dual-vertex `_a` API throughout.

**Seeds:** Three categories, analyzed separately:
- Fresh general random F=10 polytopes (HK2017 backend)
- Fresh Lagrangian products F=10, splits (3,7), (4,6), (5,5) (billiard backend)
- Warm starts: top-sys converged points from gradient-descent (tests boundary-crossing on already-optimized polytopes)

**Algorithm per seed:**
1. Gradient ascent with integrated overshoot (at each iteration, tries within-bound h-only + (h,n) steps AND overshoot multipliers 1.5x, 2x, 3x beyond step bound)
2. On convergence: escape rounds (up to 3 rounds × 5 wiggles per round). Each wiggle perturbs heights by ~5% Gaussian noise, then re-runs gradient ascent.
3. Track which strategy (within_cell / overshoot / wiggle) found the best sys.

**Parameterization:** Everything in dual-vertex (a-space). Gradient d(sys)/d(a_k) computed via library's `capacity_derivatives_a` and `volume_derivatives_a`, then step directly: a_k(t) = a_k + t * d_k. Step bound is linearized (vertex velocity via A_D^{-1}). For Lagrangian products, the gradient direction is projected to preserve subspace structure (q-facets keep zero p-components, p-facets keep zero q-components). Wiggle perturbs dual vertices directly; `Polytope4D::from_f64` rejects invalid results.

**Deferred:** Facet-splitting strategy (no code exists; would need combinatorial-boundaries results). Math.tex (correctness proofs belong in sibling experiments).

## Findings (development run, 2026-03-26)

### Summary statistics

| Category | N | Mean sys | Max sys | P90 sys | Mean Δ | Escapes |
|----------|---|----------|---------|---------|--------|---------|
| general | 10 | 0.823 | 0.901 | 0.890 | 0.437 | 10/10 |
| lagrangian | 12 | 0.821 | 0.933 | 0.880 | 0.422 | 12/12 |
| warm | 20 | 0.856 | 0.914 | 0.893 | 0.065 | 19/20 |

### Key observations

1. **Best sys = 0.933** (lagrangian_3x7_1, sys-search.jsonl). No sys > 1 found — the barrier remains unbreached across all 42 seeds.

2. **Wiggle dominates overshoot.** 41/42 seeds have wiggle as the winning strategy. Overshoot never won. Random perturbation followed by re-optimization outperforms stepping in the gradient direction beyond t_max.

3. **Warm starts show meaningful improvement.** Starting from gradient-descent's converged points (sys 0.76–0.87), boundary-crossing achieves mean Δ=0.065, max Δ=0.136.

4. **4 seeds exceed sys > 0.9** (sys-search.jsonl): lagrangian_3x7_1 (0.933), warm_10_general_171 (0.914), warm_11_lagrangian_3x7_52 (0.908), general_7 (0.901).

5. **Zero panics.** The a-space wiggle (perturbing dual vertices directly, validated by `Polytope4D::from_f64`) avoids the near-degenerate polytopes that caused KKT solver panics in the previous (h,n)-based version.

### Figures

- `sys_search_distribution.png`: Final sys histogram by category.
- `sys_search_improvement.png`: Starting vs final sys scatter. All points above y=x.
- `sys_search_strategy.png`: Box plot of final sys by winning strategy.
- `sys_search_escape.png`: Strategy bar chart. Wiggle wins 41/42 seeds.
- `sys_search_convergence.png`: Iteration count by category.

## Landscape observations (2026-03-26)

Analysis of trace data from the dev run:

- **5% wiggle drops sys by median 4.4%** (e.g. 0.82 → 0.78). 62% of wiggles produce net improvement after re-ascent. This is NOT saddle-point escape (too large) — it's closer to basin hopping.
- **Convergence is fast** (~5-10 iterations). The landscape is smooth within each combinatorial cell.
- **The step bound, not gradient vanishing, limits ascent.** Local optima are artifacts of the combinatorial structure (which orbit is optimal changes at cell boundaries), not of the smooth geometry within a cell.
- **The gradient direction is uninformative past t_max.** Overshoot (continuing in the gradient direction beyond the combinatorial boundary) never helps.
- **The 5% wiggle strength is unjustified.** Copied from gradient-search. We don't know whether it's optimal, and we don't know the autocorrelation length of the sys landscape that would set a principled perturbation scale.

## Open questions

The dev run answered "does the pipeline work?" (yes) but opened deeper questions about the sys landscape and search strategy:

1. **What does the sys landscape look like?** How many local optima, how large are their basins, is there funnel structure? This determines which search strategy is appropriate.
2. **Why does overshoot never win?** Is the gradient direction past t_max always bad, or does our linearized step bound just land in bad places?
3. **What's the right perturbation scale?** The 5% is arbitrary. The autocorrelation length of sys along random walks would set a principled choice.
4. **Does the optimal orbit change after wiggle?** If yes, wiggles cross between basins (basin hopping). If no, they re-enter the same basin from a different angle (expensive saddle escape). The current trace doesn't record orbit identity — easy to add.
5. **Are there better search strategies than wiggle?** Basin hopping, CMA-ES, simulated annealing, or methods designed for combinatorially-structured landscapes?
6. **Does F matter?** All runs use F=10. Higher F means more combinatorial cells, possibly different landscape structure.

## Known limitations

- Development scale only (42 seeds). Not statistically significant.
- F=10 only.
- Gradient correctness not independently validated (relies on gradient-correctness experiment).
- Step bound is linearized (first-order). For overshoot, the bound may be inaccurate — but `Polytope4D::from_f64` validates results.
- Wiggle strength (5%) is unjustified. No landscape characterization to inform this choice.
