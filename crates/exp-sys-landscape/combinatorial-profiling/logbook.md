# Combinatorial Profiling: Logbook

Split from `combinatorial-structure/` (Pass 1). See that logbook for full history.

## Motivation

Per-facet cell width measurement in dual-vertex space. For each facet k, probe N_FACET_DIRS random S^3 directions in R^4_k to measure how far one can move a single dual vertex before the combinatorial type changes. No EHZ computation needed -- only boundary detection.

## Status

**Complete (2026-03-27).** 140 polytopes, 11200 profiling rows. Split into standalone experiment.

## How to run

```bash
cargo run -p exp-sys-landscape --release --bin sys-combinatorial-profiling
python3 analyze.py
```

## Results (from combinatorial-structure, 2026-03-27)

### Per-facet cell profiling

10 random S^3 directions per facet per polytope. 11200 profiling rows total.

| Metric | Orbit facets | Non-orbit facets |
|--------|-------------|-----------------|
| Probes | 8387 | 2810 |
| Median t_max | 0.258 | 0.124 |

(profiling JSONL, t_max < 100, cell_orbit_vs_nonorbit.png)

**Orbit facets have 2x wider cells than non-orbit facets.** Non-orbit facets are not constrained by the optimal Reeb orbit and can be closer to degeneracy.

**Cells are highly anisotropic:** median max/min t_max ratio within a facet's R^4 is 8.3x, with extreme outliers up to 9051x (cell_anisotropy.png).

**Event types in per-facet probes:** incidence flips dominate (69.5%), omega_0 flips 30.5% (profiling_event_types.png). Per-facet directions move only one dual vertex, so incidence flips are more common than in global probes.

**Cell width decreases with F** for both orbit and non-orbit facets (cell_width_by_F.png).

## Open questions

1. **Anisotropy structure:** What determines the anisotropy directions within each facet's R^4? Deferred unless sys-search needs anisotropic steps.
