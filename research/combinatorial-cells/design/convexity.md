# Combinatorial Convexity: Logbook

Split from the original `combinatorial-structure/` experiment (Pass 3: convexity testing).

## Motivation

Tests whether combinatorial-type cells in dual-vertex space are convex. Samples pairs of boundary probes from per-facet profiling and checks if the midpoint of two interior points has the same combinatorial type. Three checks: incidence preservation, omega_0 sign preservation, transition matrix preservation.

## Status

**Complete (2026-03-27).** 140 polytopes, 2711 successful midpoint constructions out of 2800 tests. Split into standalone experiment.

## How to run

```bash
cd crates/ && cargo run -p exp-combinatorial-cells --release --bin cell-convexity
uv run analyze.py
```

## Results (from combinatorial-structure, 2026-03-27)

### Convexity testing

| Check | Failure rate |
|-------|-------------|
| Incidence change | 0.85% |
| omega_0 sign change | 52.6% |
| Transition matrix change | 57.6% |

(convexity JSONL, midpoint_construction_ok=true rows, cell_convexity.png)

**Cells are NOT convex.** 57.6% of midpoints have a different transition matrix. The transition matrix determines which cycles are feasible in HK2017.

Incidence is almost always preserved (99.15%) -- incidence boundaries are approximately hyperplanes.

**Failure rate increases sharply with F:** ~0% at F=5, ~50% at F=6-7, ~67% at F>=8.

**Non-convexity is entirely a product phenomenon.** Random polytopes: 1/1150 (0.1%) transition failures. Lagrangian products: 1561/1561 (100%). Products have special omega_0 relationships between cross-factor facet pairs (near-zero values that flip easily under perturbation).

**Implications for optimization:** Non-convexity means line searches cannot assume the combinatorial type is constant along a straight-line interpolation between two interior points.
