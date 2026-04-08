# Combinatorial Profiling: Logbook

Split from the original `combinatorial-structure/` experiment (Pass 1: per-facet profiling).

## Motivation

Per-facet cell width measurement in dual-vertex space. For each facet k, probe N_FACET_DIRS random S^3 directions in R^4_k to measure how far one can move a single dual vertex before the combinatorial type changes. No EHZ computation needed -- only boundary detection.

## Status

**Complete (2026-03-27).** 140 polytopes, 11200 profiling rows. Split into standalone experiment.

## How to run

```bash
cd crates/ && cargo run -p exp-combinatorial-cells --release --bin cell-widths
uv run analyze.py
```

## Results (from combinatorial-structure, 2026-03-27)

### Per-facet cell profiling

10 random S^3 directions per facet per polytope. 11200 profiling rows total.

| Metric | Orbit facets | Non-orbit facets |
|--------|-------------|-----------------|
| Probes | 8389 | 2810 |
| Median t_max | 0.257 | 0.121 |

(profiling JSONL, t_max < 100, cell_orbit_vs_nonorbit.png)

**Orbit facets have 2x wider cells than non-orbit facets.** Non-orbit facets are not constrained by the optimal Reeb orbit and can be closer to degeneracy.

**Cells are highly anisotropic:** median max/min t_max ratio within a facet's R^4 is 8.4x, with extreme outliers up to 7040x (cell_anisotropy.png).

**Event types in per-facet probes:** incidence flips dominate (69.1%), omega_0 flips 30.9% (profiling_event_types.png). Per-facet directions move only one dual vertex, so incidence flips are more common than in global probes.

**Cell width decreases with F** for both orbit and non-orbit facets (cell_width_by_F.png).

## Open questions

1. **Anisotropy structure:** What determines the anisotropy directions within each facet's R^4? Deferred unless sys-search needs anisotropic steps.
