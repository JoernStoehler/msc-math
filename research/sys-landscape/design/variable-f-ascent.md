# Variable-F Gradient Ascent: Logbook

## Motivation

Fixed-F gradient ascent optimizes within R^{4F} but the F-polytope space is a strict subset of the space of all polytopes (and of convex bodies). A local maximum in F-space might not be a local maximum in (F+1)-space. This experiment tests whether allowing facet count to grow (F → F+1) unlocks higher sys values.

Two research questions:

**RQ1: Can F-local maxima be improved by embedding into (F+1)-space?**
Take an F=10 local maximum, add a barely-non-redundant facet (embedding into F=11 space near the original polytope), then run gradient ascent. Does sys improve beyond the F=10 local max?

**RQ2: Is (F+1)-ascent better than F-ascent when started early?**
From the same random F=10 start, compare four paths:
- **Path A**: F=10 gradient ascent
- **Path B**: add facet → F=11 gradient ascent
- **Path C**: fresh random F=11 gradient ascent (baseline)
- **Path D**: F=10 ascent → add facet → F=11 ascent (optimize-then-expand)

The four-way comparison separates "structured entry from F-space helps" from "more facets help", and tests whether optimizing before expanding (D) beats expanding before optimizing (B).

## Status

**Active.** Initial implementation. The tracked
`variable-f-ascent.jsonl` artifact was refreshed on 2026-04-18 after the
fixed-`F` source packets were regenerated.

## How to run

```bash
cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent           # resume
cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent -- --fresh # rerun
cd experiments/sys-landscape/variable-f-ascent/ && uv run analyze.py            # figures
```

## Files

| File | Role |
|------|------|
| main.rs | Binary: variable-F gradient ascent (RQ1 + RQ2) |
| analyze.py | Figures + summary statistics |
| variable-f-ascent.jsonl | Per-trial results |
| variable-f-ascent/cache.jsonl | Local exact-geometry cache for touched start, added, endpoint, and intermediate polytopes |
| research/sys-landscape/design/variable-f-ascent.md | This file |

### Output contract note

As of 2026-04-18, `variable-f-ascent.jsonl` is no longer just a thin endpoint
summary. Each row also stores:

- `source_name` and `lineage_id` for grouping related paths or placements
- `direct_parent_trial` when the parent row exists in the same dataset
- exact rational dual vertices for the ascent start, the post-addition state
  when present, and the final endpoint

The local `cache.jsonl` now also persists `volume`, `volume_err`,
`capacity_err`, and best-sigma metadata when those values were already computed
during the run. It still does **not** try to encode experiment-specific
provenance in the shared `source` field, because the cache contains many
intermediate gradient-step polytopes from mixed lineages.

## Methodology

### Facet addition

To embed an F-polytope P into (F+1)-space: add a dual vertex a_{F+1} = n / (h_K(n) - ε) where h_K(n) = max_v ⟨n,v⟩ is the support function and ε > 0 is a small depth parameter. This creates a barely-non-redundant facet that shaves a thin sliver off P. When ε → 0, the (F+1)-polytope approaches P.

Direction n is sampled uniformly on S³. Depth ε = 1e-3 (same as facet-splitting experiment).

### Gradient ascent

Same algorithm as gradient-ascent-general: line search over step fractions of t_max + overshoot multipliers, wiggle escape strategy. Copied into this binary for self-containment.

### RQ1 setup

Starting points: 10 F=10 local maxima from gradient-ascent-general (final_dual_vertices from gradient-ascent-general.jsonl). For each: 5 random facet placements → (F+1) gradient ascent.

### RQ2 setup

10 random F=10 starting polytopes (fresh, master seed 43 to avoid overlap with gradient-ascent-general's seed 42). For each, all four paths run from the same starting polytope.

## Findings (2026-04-18 refreshed packet)

Current packet: `90` completed trials (`50` for RQ1, `40` for RQ2).

### RQ1: 45/50 (90%) improved over the source F=10 local maximum

F=11 gradient ascent starting from a barely-perturbed F=10 local maximum still
improves `sys` in the large majority of placements, and the added facet
remained active in `100%` of the refreshed packet.

| Source | src_sys | Improved | Best final | Best Δ |
|--------|---------|----------|------------|--------|
| general_0 | 0.7691 | 5/5 | 0.8211 | +0.0520 |
| general_1 | 0.7540 | 5/5 | 0.8046 | +0.0507 |
| general_2 | 0.8310 | 4/5 | 0.8579 | +0.0269 |
| general_3 | 0.7905 | 5/5 | 0.8283 | +0.0379 |
| general_4 | 0.8750 | 5/5 | 0.8759 | +0.0009 |
| general_5 | 0.6909 | 5/5 | 0.8172 | +0.1263 |
| general_6 | 0.7506 | 5/5 | 0.7997 | +0.0491 |
| general_7 | 0.8582 | 4/5 | 0.8856 | +0.0274 |
| general_8 | 0.7167 | 4/5 | 0.7590 | +0.0423 |
| general_9 | 0.9030 | 3/5 | 0.9063 | +0.0034 |

The refreshed packet strengthens the same qualitative split: weaker F=10 local
maxima can move substantially in F=11, while the strongest fixed-F seed
`general_9` only improves in `3/5` placements and only up to `0.9063`. Mean
`Δ` across all `50` RQ1 trials is `+0.0257`; the largest gain is
`+0.1263` at `rq1_general_5_p2`.

HKO2024-near testing stays out of this packet and lives in
`experiments/hko-local-maximum/cut-and-ascent/`.

### RQ2: Four-way comparison from random F=10 starts

Four paths from the same `10` random F=10 starting polytopes (seed `43`):

| Path | Description | Mean | Median | Max | Min |
|------|-------------|------|--------|-----|-----|
| **D: F=10→F=11** | F=10 ascent → add facet → F=11 ascent | **0.8422** | **0.8371** | **0.8868** | 0.8037 |
| A: F=10 ascent | F=10 → ascent | 0.8293 | 0.8206 | 0.8861 | 0.7744 |
| C: random F=11 | fresh random F=11 → ascent | 0.8180 | 0.8226 | 0.9032 | 0.7224 |
| B: add+F=11 | add facet → F=11 ascent | 0.5265 | 0.5304 | 0.8818 | 0.1205 |

Paired comparisons:
- D wins **10/10** vs A. Mean(D-A) = `+0.0129`.
- B wins **2/10** vs A. Mean(B-A) = `-0.3028`.
- B wins **1/10** vs D. Mean(B-D) = `-0.3157`.

The refreshed ordering is **D > A > C > B** by mean final `sys`. The main
conclusion survives: optimize in F=10 first, then expand to F=11. The penalty
for adding a thin facet before any optimization is much stronger in the
refreshed packet than in the 2026-04-04 run.

### Figures

| Figure | Description |
|--------|-------------|
| variable-f-rq1.png | Scatter: F=10 local max sys vs F=11 ascent final sys. Points above diagonal = improved. |
| variable-f-rq2.png | Box plot: four-way comparison of final sys by path. |

## Open questions

1. Does iterating (F→F+1→F+2→...) keep improving, or does it plateau after one step?
2. Would larger ε (deeper cuts) or gradient-informed placement (not random) improve RQ1 success rate?
3. Does the improvement rate depend on F? (Currently only tested F=10→F=11.)

## Related experiments

- `gradient-ascent-general/` — fixed-F=10 ascent, provides RQ1 starting points (current bounded packet best sys=0.9030)
- `experiments/hko-local-maximum/facet-splitting/` — tested F=10→F=11 cuts on HKO2024 without subsequent ascent; all 536 cuts decreased sys
- `boundary-characterization/` (exp-combinatorial-cells) — combinatorial boundary types and density
