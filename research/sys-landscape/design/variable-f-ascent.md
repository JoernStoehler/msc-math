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

**Active.** Initial implementation.

## How to run

```bash
cd crates/ && cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent           # resume
cd crates/ && cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent -- --fresh # rerun
cd crates/exp-sys-landscape/variable-f-ascent/ && uv run analyze.py            # figures
```

## Files

| File | Role |
|------|------|
| run.rs | Binary: variable-F gradient ascent (RQ1 + RQ2) |
| analyze.py | Figures + summary statistics |
| variable-f-ascent.jsonl | Per-trial results |
| logbook.md | This file |

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

## Findings (2026-04-04)

Total runtime: 1986s (~33 min), 90 trials. Cached rerun: 111s.

### RQ1: 43/50 (86%) improved over F=10 local max

F=11 gradient ascent starting from barely-perturbed F=10 local maxima **consistently improves sys**. All added facets remained non-redundant (active) at the end of optimization — the F+1 polytope genuinely uses the extra degree of freedom.

| Source | src_sys | Improved | Best final | Best Δ |
|--------|---------|----------|------------|--------|
| general_0 | 0.7763 | 5/5 | 0.8231 | +0.047 |
| general_1 | 0.8321 | 2/5 | 0.8515 | +0.019 |
| general_2 | 0.8887 | 4/5 | 0.8945 | +0.006 |
| general_3 | 0.8551 | 5/5 | 0.8835 | +0.028 |
| general_4 | 0.7617 | 4/5 | 0.8191 | +0.057 |
| general_5 | 0.8748 | 5/5 | 0.8927 | +0.018 |
| general_6 | 0.7888 | 5/5 | 0.8219 | +0.033 |
| general_7 | 0.9005 | 3/5 | 0.9035 | +0.003 |
| general_8 | 0.8324 | 5/5 | 0.8752 | +0.043 |
| general_9 | 0.7151 | 5/5 | 0.7593 | +0.044 |

**Improvement rate correlates inversely with src_sys:** lower local maxima improve more reliably (general_0 at 0.776: 5/5) than higher ones (general_7 at 0.901: 3/5). The higher the F=10 local max, the closer it already is to the F=11 local max above it.

Mean Δ across all 50 trials: +0.016. Max Δ: +0.057 (variable-f-ascent.jsonl, rq1_general_4_p3).

HKO2024 testing (cut + ascent in F=11 space) moved to `exp-hko-local-maximum/cut-and-ascent/`.

### RQ2: Four-way comparison from random F=10 starts

Four paths from the same 10 random F=10 starting polytopes (seed 43):

| Path | Description | Mean | Median | Max | Min |
|------|-------------|------|--------|-----|-----|
| **D: F=10→F=11** | F=10 ascent → add facet → F=11 ascent | **0.828** | **0.852** | **0.901** | 0.698 |
| C: random F=11 | fresh random F=11 → ascent | 0.806 | 0.805 | 0.871 | 0.723 |
| A: F=10 ascent | F=10 → ascent | 0.795 | 0.821 | 0.879 | 0.686 |
| B: add+F=11 | add facet → F=11 ascent | 0.707 | 0.770 | 0.860 | 0.203 |

Paired comparisons:
- D wins **10/10** vs A. Mean(D-A) = +0.033.
- B wins **3/10** vs A. Mean(B-A) = -0.088.
- B wins **1/10** vs D. Mean(B-D) = -0.121.

**Ordering: D > C > A > B.** Optimize first, then expand F is strictly best. Adding a thin-sliver facet before optimization (Path B) hurts — the optimizer struggles with the pathological starting geometry. Random F=11 (Path C) slightly outperforms F=10 (Path A), showing more facets help when the geometry is natural.

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

- `gradient-ascent-general/` — fixed-F=10 ascent, provides RQ1 starting points (best sys=0.9005)
- `facet-splitting/` (exp-hko-local-maximum) — tested F=10→F=11 cuts on HKO2024 without subsequent ascent; all 536 cuts decreased sys
- `boundary-characterization/` (exp-combinatorial-cells) — combinatorial boundary types and density
