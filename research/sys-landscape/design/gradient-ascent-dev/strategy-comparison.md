# Strategy Comparison: Logbook

## Motivation

gradient-ascent-general/gradient-ascent-products (formerly boundary-crossing-search) found that wiggle (random perturbation after step) dominates overshoot (stepping past cell boundary) for improving sys. But the wiggle strength (5%) is unjustified — inherited from the deleted gradient-search experiment. This instrument systematically compares strategies on the same polytope set.

## Status

**Scaffolded.** Not yet implemented.

## How to run

```bash
cd crates/ && cargo run -p dev-gradient-ascent --release --bin dev_strategy_comparison
```

## Research questions

1. Overshoot vs wiggle vs additive noise vs random restart: which converges fastest?
2. What wiggle strength is optimal? (cell-widths experiment cell widths suggest 0.12-0.26 as scale)
3. Does mixing strategies (e.g. wiggle + occasional random restart) help?
4. How does strategy choice interact with polytope type (random vs Lagrangian product)?

## Related experiments

- `exp-sys-landscape/gradient-ascent-general/`, `exp-sys-landscape/gradient-ascent-products/` — found wiggle > overshoot, 5% strength
- `exp-combinatorial-cells/cell-widths/` — cell widths for calibrating wiggle strength
- `exp-combinatorial-cells/convexity/` — random cells convex, product cells not
