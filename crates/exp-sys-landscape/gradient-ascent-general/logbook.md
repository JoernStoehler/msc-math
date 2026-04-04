# Gradient Ascent General: Logbook

## Motivation

Can gradient-based optimization push sys past 1 for general (non-Lagrangian) polytopes? Previous gradient experiments found best non-HKO sys ~ 0.905 but were limited by the step-bound barrier (gradient ascent converges within each combinatorial cell but cannot cross boundaries). This experiment combines within-cell ascent with boundary-crossing strategies on general polytopes.

## Status

**Split from boundary-crossing-search (2026-04-04).** Data needs regeneration.

## Predecessor

Split from `boundary-crossing-search/` (2026-04-04). That experiment combined general and Lagrangian product gradient ascent in one binary. The Lagrangian product part is now in `gradient-ascent-products/`.

## How to run

```
cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-general           # resume from existing data
cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-general -- --fresh # rerun from scratch
cd crates/exp-sys-landscape/gradient-ascent-general/ && python3 analyze.py            # generate figures + summary
```

### Files

| File | Role |
|------|------|
| run.rs | Binary: generate general polytopes + free gradient ascent + overshoot + wiggle |
| analyze.py | Summary table, 5 figures |
| gradient-ascent-general.jsonl | Per-seed summary (one row per polytope) |
| gradient-ascent-general-trace.jsonl | Per-iteration trace (diagnostic) |
| sys-search.jsonl | Legacy data from boundary-crossing-search (mixed general+product rows, needs regeneration) |
| sys-search-trace.jsonl | Legacy trace data from boundary-crossing-search (needs regeneration) |
| sys_search_*.png | Figures (from legacy data) |

## Algorithm

Free gradient ascent in R^{4F} on general polytopes using HK2017 capacity backend. No gradient projection (unlike the Lagrangian product variant in gradient-ascent-products/).

Per seed:
1. Gradient ascent with integrated overshoot (at each iteration, tries within-bound steps AND overshoot multipliers 1.5x, 2x, 3x beyond step bound)
2. On convergence: escape rounds (up to 3 rounds x 5 wiggles per round). Each wiggle perturbs dual vertices by ~5% Gaussian noise, then re-runs gradient ascent.

## Findings from boundary-crossing-search (predecessor, 2026-03-26)

General polytopes: 10 seeds, mean sys 0.823, max sys 0.901. Warm starts from gradient-descent: mean delta 0.065. Wiggle dominated overshoot (41/42 seeds across all categories). No sys > 1 found.

## Data status

Legacy data (`sys-search.jsonl`, `sys-search-trace.jsonl`) contains mixed general+Lagrangian rows from the predecessor experiment. Needs regeneration with the split binary.
