# Gradient Ascent Products: Logbook

## Motivation

Can gradient-based optimization push sys past 1 for Lagrangian products? Previous experiments suggest Lagrangian products reach higher sys than general polytopes. This experiment performs projected gradient ascent on the Lagrangian product submanifold, preserving the product structure throughout optimization.

## Status

**Split from boundary-crossing-search (2026-04-04).** Fresh data generated.

## Predecessor

Split from `boundary-crossing-search/` (2026-04-04). That experiment combined general and Lagrangian product gradient ascent in one binary. The general polytope part is now in `gradient-ascent-general/`.

## How to run

```
cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-products           # resume from existing data
cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-products -- --fresh # rerun from scratch
cd crates/exp-sys-landscape/gradient-ascent-products/ && uv run analyze.py            # generate figures + summary
```

### Files

| File | Role |
|------|------|
| run.rs | Binary: generate Lagrangian products + projected gradient ascent + overshoot + wiggle |
| analyze.py | Summary table, 5 figures |
| gradient-ascent-products.jsonl | Per-seed summary (one row per polytope) |
| gradient-ascent-products-trace.jsonl | Per-iteration trace (diagnostic) |

## Algorithm

Projected gradient ascent on the Lagrangian product submanifold using billiard capacity backend. Gradient direction is projected to preserve Lagrangian product structure: q-facets keep zero p-components ([2],[3] zeroed), p-facets keep zero q-components ([0],[1] zeroed).

Per seed:
1. Gradient ascent with integrated overshoot (at each iteration, tries within-bound steps AND overshoot multipliers 1.5x, 2x, 3x beyond step bound)
2. On convergence: escape rounds (up to 3 rounds x 5 wiggles per round). Each wiggle perturbs dual vertices by ~5% Gaussian noise, then re-runs gradient ascent.

## Findings from boundary-crossing-search (predecessor, 2026-03-26)

Lagrangian products: 12 seeds, mean sys 0.821, max sys 0.933. Best overall sys=0.933 (lagrangian_3x7_1). Wiggle dominated overshoot. No sys > 1 found.

## Data status

Fresh data generated 2026-04-04 after warm-start removal and split from boundary-crossing-search. 12 fresh Lagrangian products (3 splits x 4 per bucket), no warm starts (warm-start source `large-scale-descent` was deleted as superseded). Polytopes use standard master seed (42), low attempt numbers, and benefit from database caching for initial capacity/volume computation.
