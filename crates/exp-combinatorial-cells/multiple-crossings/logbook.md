# Combinatorial Sweep: Logbook

Split from the original `combinatorial-structure/` experiment (Pass 4: multi-boundary sweep).

## Motivation

Multi-boundary traversal: walk along a direction for a distance budget, iteratively stepping past each combinatorial boundary. Tracks sys at each step to measure whether the gradient direction maintains improvement across multiple boundary crossings. Answers: how many boundaries does a typical gradient step cross?

## Status

**Complete (2026-03-27).** 140 polytopes, 560 sweep rows (4 directions per polytope). Split into standalone experiment.

## How to run

```bash
cd crates/ && cargo run -p exp-combinatorial-cells --release --bin cell-multiple-crossings
uv run analyze.py
```

## Results (from combinatorial-structure, 2026-03-27)

### Multi-boundary sweeps

Walk along a direction for distance 1.0. 560 sweeps (140 polytopes x 4 directions: gradient, neg-gradient, 2 dense random).

| F | Median boundaries | Mean | Max |
|---|-------------------|------|-----|
| 5 | 1 | 2.1 | 6 |
| 6 | 3 | 3.2 | 13 |
| 7 | 4 | 5.5 | 17 |
| 8 | 7 | 8.0 | 36 |
| 9 | 10 | 10.8 | 33 |
| 10 | 14 | 14.4 | 45 |

(sweep JSONL, budget=1.0)

**A typical gradient step crosses ~6 boundaries** (median across all F and directions). Scales roughly linearly with F.

**Event type distribution in sweeps:** incidence flips 75.6%, omega_0 flips 24.4%.

**36% of sweeps end by construction failure** before exhausting the distance budget. Failure modes: 84% "unbounded" (lost positive spanning), 16% "facet redundant".

**sys increases along gradient sweeps despite boundary crossings.** 71% of gradient sweeps end with higher sys than they started (median improvement 65%). 0% of neg-gradient sweeps increase sys (sanity check). The path is not monotonic (only 28% non-decreasing) -- sys oscillates across boundaries.

**Orbit switches in the gradient direction tend to increase sys.** At single-boundary crossings: 4/5 gradient-direction orbit switches had positive delta_sys.

## Open questions

1. **Construction failure after multi-boundary crossing:** 36% of sweeps fail. Sys-search should detect and backtrack.
