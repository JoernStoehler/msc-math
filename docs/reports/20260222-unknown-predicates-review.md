# Review: Unknown predicates experiment — UNKNOWN admissibility survey

**Branch:** `unknown-predicates-b30a`
**Base:** local `main` at `9ca403a`
**Date:** 2026-02-22

## Build Verification

| Check | Status |
|-------|--------|
| `cargo test --lib` (crates/) | 180 passed, 28 ignored, 0 failed |
| `cargo clippy --lib -- -D warnings` | Clean |
| `cargo build --release` (experiments/) | Clean |
| `ruff check experiments/unknown-predicates/` | Clean |
| Python script runs and produces correct output | Verified |

## What Changed

3 commits, 713 lines added across 7 new files. No deletions, no modifications to existing code.

**New experiment:** `experiments/unknown-predicates/` — a survey checking whether the EHZ capacity algorithm ever produces UNKNOWN (inconclusive) admissibility verdicts on existing polytope datasets.

Files added:
- `unknown_predicates.rs` — Rust binary regenerating random-sweep (70 polytopes) + lagrangian-products (92 polytopes) with certified/uncertain capacity tracking
- `unknown_predicates.py` — Python analysis script with summary stats and beta_min histogram
- `unknown-predicates.tex` — LaTeX writeup documenting the negative result
- `unknown-predicates.jsonl` — 162 rows of results data
- `unknown_predicates_beta_min.png` — histogram figure
- `Cargo.toml` entry + `reproduce.sh` entries

## Commit Quality

| Commit | Message | Atomicity |
|--------|---------|-----------|
| `edd6bdd` | Add unknown-predicates experiment: Phase 1 UNKNOWN predicate survey | Code only (Rust + Python) |
| `f2df255` | Add Phase 1 data: zero UNKNOWNs across 162 polytopes | Data + figure only |
| `d3dcd3d` | Add .tex writeup and reproduce.sh entry | Documentation + pipeline |

All commits have Co-Authored-By. Messages describe "why" well. Code and data are in separate commits. Clean working tree.

## Code Quality

### Rust binary (unknown_predicates.rs)

**Parameter matching verified:** All constants (seed=42, h_min/max, facet plan, pentagon sweep angles, pair list, pair step) match the original `random_sweep.rs` and `lagrangian_sweep.rs` exactly. Comments explicitly state "must match [...] exactly".

**Numerical gap computation:** For EHZ results, uses `result.numerical_gap()` (= `capacity - capacity_uncertain`). For billiard results, computes the same inline: `result.capacity - result.capacity_uncertain`. Semantically identical. Correct.

**Systolic ratio:** `sys = capacity² / (2·vol)` — matches the mathematical definition. Verified: `pair_3x6_0deg` (triangle × hexagon at 0°) gives sys ≈ 1.0 as expected (known Viterbo optimizer).

**beta_min computation:** `result.best_beta.iter().cloned().fold(f64::INFINITY, f64::min)` — correct minimum over the dwell-time vector.

**Minor observation:** Algorithm field in JSONL says `"ehz_pruned"` but the function is now called `ehz_capacity` (after the rename in 9ca403a). Functionally irrelevant for data analysis. The name `ehz_pruned` is arguably more descriptive for JSONL readers.

### Python script (unknown_predicates.py)

Clean, well-structured. Proper error handling for missing data file. Histogram uses log scale for beta_min — appropriate given the 8-order-of-magnitude range. Division-by-zero protected with `max(n, 1)`.

### LaTeX writeup (unknown-predicates.tex)

**All numerical claims verified against data:**
- 162 polytopes total: 70 random + 92 lagrangian ✓
- Zero numerical gaps ✓
- Random β_min range: 6.7e-4 to 1.2e-1, median 4.5e-2 ✓
- Lagrangian β_min range: 6.2e-12 to 3.5e-1, median 1.7e-1 ✓
- Smallest β_min = 6.2e-12 for pair_4x4_18deg ✓ (verified: 6.166e-12)
- "Just 6× above threshold": 6.17e-12 / 1e-12 ≈ 6.2 ✓
- "8-order-of-magnitude gap": log₁₀(6.7e-4 / 6.2e-12) ≈ 8.0 ✓

Well-structured (Experiment → Results → Conclusion). Correct LaTeX math.

**Not yet wired into thesis** (`\input` not added to thesis/main.tex). This is expected — wiring in is a separate step.

## Data Pipeline

Pipeline is complete and consistent:
- Rust binary → `unknown-predicates.jsonl` (162 rows)
- Python script → `unknown_predicates_beta_min.png` (histogram)
- `.tex` writeup references both data and figure
- `reproduce.sh` updated with both steps in correct order (Step 1: Rust binary, Step 2: Python script)

## Figure Quality

The histogram clearly shows:
- Left panel (random-sweep): β_min concentrated in [1e-2, 1e-1], well above threshold
- Right panel (lagrangian-products): bimodal with mass near 0.17 (pentagon pairs) and one outlier near 1e-11 (pair_4x4_18deg near-miss)
- Red dashed line at EPS_BETA_POSITIVE = 1e-12 clearly visible

Minor: The lagrangian-products panel is visually dominated by the ~90 points in one bin, making the near-miss outlier barely visible. Not incorrect, but a zoom inset or log-y axis could improve readability. Low priority.

## Strengths

1. **Correct parameter matching** — careful reproduction of existing datasets
2. **Complete pipeline** — Rust → JSONL → Python → figure → .tex, all working
3. **Good commit hygiene** — atomic commits, code/data separated, good messages
4. **Verified numerical claims** — every number in the .tex traces back to data
5. **Meaningful scientific result** — the near-miss at pair_4x4_18deg (6× above threshold) is a genuine finding worth documenting

## Issues

None blocking. Two minor observations:

1. **Figure readability** (cosmetic): The lagrangian-products histogram bin structure doesn't highlight the near-miss outlier well. A log-y axis or annotation could help. Not blocking.

2. **No README.md** in experiment folder. Other experiments may or may not have one. Very minor.

## Executive Summary

**Summary of findings:**
1. Clean experiment: all builds pass, all data verified, all .tex claims match data
2. Parameters match original experiments exactly — correct reproduction
3. Complete pipeline with good commit structure (3 atomic commits)
4. No issues found. Two cosmetic observations (figure readability, missing README)

**Recommendation:** Merge as-is. Clean, well-structured experiment with verified results.

**Time investment:** ~25min review
