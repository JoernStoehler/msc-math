# Review: sys-optimization experiment (Phase 1–2)

**Branch:** `/workspaces/worktrees/sys-optimization`
**Base:** local `main` at `9ca403a`
**Date:** 2026-02-22

## Build Verification

| Check | Status |
|-------|--------|
| Library tests (`cargo test --lib`) | PASS (all 180+28 ignored) |
| Library clippy | PASS (zero warnings) |
| Experiment binary build | PASS |
| Experiment clippy | 3 warnings (see Issues) |
| Python lint (`ruff check`) | PASS |
| Working tree | Clean |

## Commit Quality

6 commits with clean logical separation:

1. `071cc52` — Rust binary (code)
2. `908c21b` — Generated data (JSONL)
3. `6a796f1` — Python analysis script
4. `47ef0ce` — Generated figures and stats table
5. `dba00b4` — LaTeX writeup
6. `a65f192` — reproduce.sh update

Good progression: code → data → analysis → figures → writeup → pipeline. All authored by Jörn.

## Library Code Verification

The binary copies ~400 lines of library internals (KKT solver, combinatorics, adjacency). All verified against library source:

| Item | Library location | Match? |
|------|-----------------|--------|
| 6 numeric constants | kkt.rs, constants.rs | Exact |
| `omega0_local` | geom/symplectic.rs:28 | Exact |
| `q_from_beta` | kkt.rs:59-69 | Exact |
| `find_positive_beta_1d` | kkt.rs:79-120 | Exact |
| `find_positive_beta_nd` | kkt.rs:125-170 | Exact |
| `build_kkt_system` | kkt.rs:184-223 | Exact |
| `solve_kkt_svd_path` | kkt.rs:233-359 | Exact |
| `solve_kkt_full` | kkt.rs:380-407 | Exact |
| `combinations` | hk2017/mod.rs:157-180 | Exact |
| `for_each_cyclic_permutation` | hk2017/permutations.rs:22-35 | Exact |
| `heap_perms_buf` | hk2017/permutations.rs:38-57 | Exact |
| `build_adjacency_matrix` | hk2017/mod.rs:184-204 | Exact |
| `build_directed_adjacency_matrix` | hk2017/mod.rs:216-229 | Exact |
| `is_adjacent_cycle` | hk2017/mod.rs:232-235 | Exact |

Follows convention: experiment binary copies library internals, documented with source references.

## Mathematical Correctness

**Chain rule:** Code computes `d(sys)/d(h_k) = (1/vol)[c · dc/dh_k - sys · dvol/dh_k]`. Verified correct derivation from `sys = c²/(2·vol)`.

**Capacity derivatives:** Re-solves KKT for top orbits (within 5% of best action, minimum 20) with ±1e-7 height perturbation. The orbit set is fixed; only heights change. For perturbations this small, the optimal orbit is overwhelmingly likely to remain in the tracked set. Sound approach.

**Cross-check:** Instrumented capacity is asserted to match library `ehz_capacity` within 1e-8 for every polytope. This catches any copy/divergence bugs.

**Step bound:** For simple vertices (4 incident facets), exact computation via N^{-1}·g. For non-simple vertices (>4), conservative bound via `slack/max_g`. Safe but potentially very conservative for Lagrangian products (common non-simple vertices). Acknowledged in code comments.

## Data Pipeline Verification

| Claim | Verified |
|-------|----------|
| 140 sensitivity rows | 140 (60 random-sweep + 80 random-product-sweep) |
| 700 step rows | 700 (140 polytopes × 5 fractions) |
| 1120 total facets | 1120 (matches LaTeX figure caption) |
| 94% favorable facets | 94.6% (1059/1120) |
| 140/140 with non-zero gradient | Confirmed |
| 128 improved (91%) | 128 (128/140 = 91.4%) |
| Best sys before: 0.7579 | Confirmed |
| Best sys after: 0.8369 | Confirmed (from random_4x5_1, old_sys=0.7349) |
| Mean Δsys: 0.0327 | Confirmed |
| Median Δsys: 0.0135 | Confirmed |
| reproduce.sh updated | Confirmed (step 1 + step 2) |

All LaTeX stats table values match recomputed values from raw data. All LaTeX prose claims match data.

**Minor discrepancy in prose:** LaTeX says best sys "starting from a polytope with sys = 0.735". The actual old_sys for the best-after polytope (random_4x5_1) is 0.7349, which rounds to 0.735. OK. But the max sys *before any step* is 0.7579 (a different polytope). The prose is correct — it describes the starting sys of the polytope that achieved the best *after-step* sys, not the highest starting sys overall.

## Strengths

1. **Clean experiment structure** following all conventions (colocated artifacts, pipeline documented)
2. **Proper cross-validation** — instrumented capacity asserted against library for every polytope
3. **Well-separated commits** — code, data, analysis, writeup each separate
4. **Correct math** — chain rule, finite differences, step bounds all verified
5. **Good writeup** — clear method description, honest discussion of limitations
6. **Figures are informative** — scatter plot clearly shows improvement structure, gradient histogram shows distribution

## Issues

### 1. f64::INFINITY serialized as JSON null (low severity)

`runner_up_action` and `runner_up_gap` are `f64` fields in `SensitivityRow`. When a polytope has only 1 valid orbit, these are set to `f64::INFINITY`, which serde_json serializes as `null`. Affects 8/140 rows.

**Impact:** None currently — Python analysis doesn't use these fields. But the serialization behavior is version-dependent and could break on serde_json updates. Better practice: use `Option<f64>` and set to `None` when there's no runner-up.

**Suggested fix:** Change `runner_up_action: f64` and `runner_up_gap: f64` to `Option<f64>` in `SensitivityRow`, and set them explicitly to `None` when `orbits.len() < 2`.

### 2. Clippy warnings (low severity)

Three warnings on the experiment binary:
- `InstrumentedResult.capacity_uncertain` — field never read
- `InstrumentedResult.iterations` — field never read
- `k % 2 == 0` → suggest `k.is_multiple_of(2)` in copied Heap's algorithm

**Impact:** Cosmetic. The unused fields appear to be for future use (Phase 3). The `is_multiple_of` is in copied library code.

### 3. LaTeX not yet wired into thesis (informational)

The writeup references `\ref{sec:random-sweep}` and `\ref{sec:random-product-sweep}` which require thesis integration. This is expected — thesis wiring is a separate step.

### 4. README.md not updated (low severity)

Pre-existing `README.md` (from ideation phase, status "Ideation") not updated to reflect that Phase 1–2 are now implemented.

### 5. One "not improved" outlier at high sys (informational, for Jörn)

The improvement scatter shows a red point near (0.76, 0.73): a polytope where the gradient step *decreased* sys. This is the polytope with the highest starting sys in the random-sweep dataset. Possibly worth investigating — the gradient direction may be less reliable near the dataset's extremes, or the step bound may overshoot.

## Executive Summary

**Summary of findings:**
1. All data verified correct. LaTeX claims match recomputed stats from raw JSONL (140 polytopes, 128 improved, best sys 0.84).
2. Copied library code (14 functions + 6 constants) verified exact match against library source.
3. Three minor clippy warnings (2 unused fields, 1 style); f64::INFINITY→null serialization is fragile but doesn't affect current results.
4. Chain rule, finite differences, and step bound math are correct.

**Recommendation:** Merge. Clean, correct experiment with proper cross-validation. Minor issues (clippy warnings, serialization fragility, README staleness) are low-priority and don't affect correctness or results. The experiment provides genuine new insights for the thesis.

**Time investment:** ~40min review
