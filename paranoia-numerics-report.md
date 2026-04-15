# Paranoia: Numerical Claims Cross-Check Report

Generated 2026-04-07 by review-claims agents (sonnet) across 29 experiment logbooks.

## Executive Summary

- **29 logbooks** checked against JSONL data, run.rs, and analyze.py
- **6 clean** (no mismatches found)
- **8 minor** (rounding, single values slightly off)
- **7 moderate** (specific wrong values)
- **5 significant** (multiple stale numbers, wrong by meaningful amounts)
- **3 severe** (completely stale or factually wrong data descriptions)
- **3 code bugs** found incidentally (not numerical claims, but broken paths/labels)

## Closure Pass 2026-04-15

This pass reconciles the 2026-04-07 findings against the post-migration `research/**/design/*.md` notes, current experiment code, and committed generated artifacts. It does not regenerate JSONL.

Live follow-up shortlist:

1. **`experiments/verification/orbit-recovery/` — live follow-up, Jörn decision on dataset scope.** `research/verification/design/orbit-recovery.md` still claims 112 rows, 112/112 pass, old error magnitudes, and 0.025ms/4.7ms timing; `TASKS.md` still repeats `112/112 orbit-recovery polytopes pass` in the historical capacity-axiom summary; `experiments/verification/orbit-recovery/main.rs` still writes `solution_dim: 0`; committed `polytopes.jsonl` has 170 rows. Next packet: choose 170 cached polytopes vs 112 historical polytopes vs a smaller validation set, compute real `solution_dim`, then regenerate `orbit-recovery.jsonl`, the figure, and the design note/task summary.
2. **`experiments/hko-local-maximum/perturbation-neighborhood/` — live follow-up.** The design note correctly documents the new smoke/LICCA file layout and dual-vertex perturbation method, but its Findings section still reports the historical `pentagon-perturb.jsonl` N=101 result while the current analyzer reads `data/{smoke,licca}-eps-*.jsonl`; `TASKS.md` still repeats the old min/max values. Next packet: split historical findings from current smoke/LICCA findings, then update the task summary.
3. **`experiments/combinatorial-cells/convexity/` — live follow-up.** The old "zero product polytopes" finding is obsolete: current JSONL has product and random rows. Current docs are still numerically stale: committed data has 2800 rows, 2661 successful midpoint constructions, 1558/1558 product transition failures, and 0/1103 random transition failures, while `research/combinatorial-cells/design/convexity.md` reports 2711 successful constructions, 1561/1561 products, and 1/1150 random failures. Next packet: refresh the design-note counts and rerun `uv run analyze.py` if the figure needs the same numbers.
4. **`research/numerics/design/error-bounds.md` — live follow-up.** The code bug is closed (`experiments/numerics/error-bounds/tests.rs` uses `error-bounds/testdata/`), but the design note still contains historical/currently confusing text: the old M1 retained-eigenvalue mismatch and `make smoke`/`make full` commands. Next packet: mark the M1 paragraph historical or replace it with the current status, and replace the Makefile commands with the current run commands.
5. **`research/crosspolytope/design/main.md` — live follow-up, minor.** The reduction-factor wording was fixed to `~20-30x`, but elapsed time still disagrees with committed JSONL: the design note says 1112.8s, while `experiments/crosspolytope/main/crosspolytope.jsonl` records `time_capacity_ms = 1095119.632531` (1095.1s). Next packet: update the timing line or state that the 1112.8s table sum is historical console output.
6. **`research/hko-local-maximum/design/cut-and-ascent.md` — live follow-up, minor.** The scale-up estimate still says `~10s per trial`; the old audit measured about 16s/trial. Next packet: update or delete the stale estimate when the cut-and-ascent plan is next touched.

Closed by current evidence:

- **Profiling:** closed as a data-pipeline break. `experiments/verification/algorithm-comparison/profiling/logbook.jsonl` still contains the bad 2026-04-04 zero-duration row, but also has a usable 2026-04-15 `f5d4ba18` row; `profile.jsonl` has 15 nonzero/zero per-test rows from the refreshed candidate list. Treat 2026-04-15 as the first usable post-fixture-removal baseline.
- **Combinatorial cells except convexity:** `cell-widths`, `omega-hypothesis`, `boundary-characterization`, `gradient-discontinuity`, and `multiple-crossings` design notes now match the corrected current numbers found in the pass.
- **Verification algorithm-comparison benchmark/ablation:** current design notes contain the corrected growth rates, speedups, and `lag triangle x square = 3*sqrt(2)/2` expected capacity; ablation documents A0 as the agreement baseline.
- **Numerics except error-bounds:** `unknown-predicates`, `kkt-inertia`, `correctness`, and `q-error` current notes/code match the corrected counts, paths, or magnitudes relevant to the old flags.
- **HKO local-maximum except perturbation/cut-and-ascent:** `lagrangian-boundary` and `second-order` contain the corrected min-sys cross-reference and current curvature/random-direction findings.
- **Sys-landscape and rotated products:** `rejection-calibration`, `random-sample`, and `rotated-regular-products` match the corrected numbers/paths; the rotated-products output path code now writes under `rotated-regular-products/`.

---

## Severity: SEVERE

### 1. `dev-capacity-validation/orbit-recovery/logbook.md`
Nearly every number is wrong against current JSONL:
- Row count: 112 claimed → 108 actual (4 known polytopes missing from data)
- Error magnitudes 2–7 orders of magnitude smaller than stated (e.g., F=10 action error: 1.83e-6 claimed → 8.88e-14 actual)
- `solution_dim` hardcoded to 0 in run.rs (never computed), but logbook claims 4 exceptions with dim > 0
- Timing: mean 0.025ms → actual 0.010ms; mean capacity 4.7ms → actual 3.2ms
- **Root cause**: JSONL regenerated under different algorithm/conditions; logbook never updated

### 2. `exp-combinatorial-cells/convexity/logbook.md`
Product/random breakdown is completely wrong:
- Logbook claims "1565/1565 products fail, 0/1156 random fail"
- JSONL contains **zero** product polytopes — all 2711 rows are random, 57.6% have transition failures
- Smaller stale mismatches: ok-count (2721→2711), percentages (~0.1pp each), F≥8 rate (65%→66–67%)
- **Root cause**: Data regenerated with different polytope database (products removed)

### 3. `exp-hko-local-maximum/perturbation-neighborhood/logbook.md`
Multiple factual errors:
- Summary stats all wrong: min 1.0022→1.0142, max 1.0333→1.0385, mean 1.0205→1.0287, std 0.0064→0.0048
- **PCA dimensionality wrong**: logbook says 50D (5 components × 10 facets), actual is 40D (4 components × 10 facets)
- **Perturbation method description wrong**: logbook describes (normal, height) decomposition with renormalization; code jitters 4D dual vertices directly
- PCA explained variance values all wrong (6.51% vs 5.95% for PC1, etc.)
- **Root cause**: Logbook describes a different perturbation scheme than what the code implements

---

## Severity: SIGNIFICANT

### 4. `exp-combinatorial-cells/cell-widths/logbook.md`
Every computed statistic is stale (7 mismatches):
- Orbit probe count: 8387→8389
- Orbit median t_max: 0.258→0.257; non-orbit: 0.124→0.121
- Median anisotropy: 8.3x→8.4x
- **Max anisotropy: 9051x→7040x** (22% off)
- Event type split: 69.5%/30.5% → 69.0%/30.9%

### 5. `exp-combinatorial-cells/omega-hypothesis/logbook.md`
3 statistical values wrong (qualitative conclusions still hold):
- Finding 1: rho = -0.22→-0.20, p = 8e-12→6e-10
- Finding 2: rho = -0.02→-0.008, p = 0.61→0.82
- Finding 4: median = +0.0006→+0.0002, frac negative = 49.5%→49.75%

### 6. `dev-numerical-analysis/unknown-predicates/logbook.md`
Multiple mismatches:
- UNKNOWN count: 29→30
- Gap upper bound: 4.93e-12→2.66e-15 (off by ~1800x)
- All 6 beta_min statistics wrong (medians, mins, maxs for both datasets)
- Latent bug: run.rs now writes `dataset: "random-sample"` but JSONL has `"random-sweep"` — re-running would break analyze.py

### 7. `dev-algorithm-comparison/benchmark/logbook.md`
Timing model parameters stale:
- Lagrangian pruned growth rate: b=3.55→4.46 (25% off, machine-independent)
- Billiard growth rate: b=3.74→4.32 (15% off)
- Billiard speedup understated: "2-3x faster"→actually 3-6x (ratio 0.17–0.33, not 0.32–0.50)
- Various smaller timing discrepancies

### 8. `dev-algorithm-comparison/ablation/logbook.md`
- Speedups overstated: ~133x at F=8 (actual ~82x), ~1078x at F=10 (actual ~579x)
- Lagrangian speedup: ~33x claimed (actual ~64x time-based)
- **Wrong expected capacity**: lag △×□ = 1.5 claimed, actual = 2.121320 (= 3√2/2)
- A0 column uses theoretical permutation counts, not JSONL values, without documenting this

---

## Severity: MODERATE

### 9. `dev-algorithm-comparison/profiling/logbook.md`
All performance numbers UNVERIFIABLE:
- logbook.jsonl contains only 1 entry (2026-04-04), not the 3 historical runs described
- That entry has all per-test durations = 0.0 (parsing failure)
- The numbers in the logbook (21s wall, 165s CPU, 317 tests, specific test durations) have no backing data

### 10. `dev-numerical-analysis/kkt-inertia/logbook.md`
- **Internal inconsistency**: "Data regeneration" note claims 8 mismatches, but captured output and Status section both say 5
- Minor: file line count 67 vs actual 68

### 11. `dev-capacity-validation/correctness/logbook.md`
- Perturbation magnitude consistently wrong: every mention of "1% height perturbation" should be 0.5% (code uses delta in [-0.5, 0.5], not [0, 1])

### 12. `dev-numerical-analysis/error-bounds/logbook.md`
Most claims UNVERIFIABLE (data from prior pipeline not preserved). Verifiable issues:
- Wrong line number for sign fix (cited line 93, actual line 113)
- `make smoke`/`make full` Makefile targets no longer exist
- **Bug**: tests.rs path hardcoded as `verify-numerics/testdata/`, should be `error-bounds/testdata/`
- M1 audit description stale (code since fixed to use ALL eigenvalues)

### 13. `exp-hko-local-maximum/lagrangian-boundary/logbook.md`
- Cross-reference: min sys for pentagon-perturb = 1.002→1.0142
- R² label mismatch: "3.8% R²" on scaling direction actually belongs to joint scaling+rotation model (scaling alone = 1.3%)
- Hessian model R² values UNVERIFIABLE (no committed analysis code)

### 14. `exp-hko-local-maximum/second-order/logbook.md`
- CV values systematically ~8% lower than current data (all 15 directions)
- All curvature values verified; Phase 3 stats all verified

### 15. `exp-combinatorial-cells/boundary-characterization/logbook.md`
4 stale mismatches:
- Event type counts: 556/424→578/402 (incidence/omega flips)
- Max |delta_sys|: 2.91e-4→8.69e-5 (3x off)
- Orbit switch count: 26→28

### 16. `exp-combinatorial-cells/gradient-discontinuity/logbook.md`
- Row count: 873→882
- Correlation: r=0.518→0.452

---

## Severity: MINOR

### 17. `exp-sys-landscape/rejection-calibration/logbook.md`
3 off-by-one values in acceptance rate table:
- F=7 [0.5,2.0]: 0.333→0.334
- F=9 [0.5,2.0]: 0.488→0.489
- F=10 [0.1,5.0]: 0.282→0.283

### 18. `exp-sys-landscape/random-sample/logbook.md`
- Spearman rho: 0.52→0.53 (rounding: 0.5253 rounds to 0.53, not 0.52)

### 19. `exp-combinatorial-cells/multiple-crossings/logbook.md`
- Failure rate: 36%→35.7% (rounded up)
- Failure mode split: 84%/16%→83.5%/16.5%
- Gradient improvement: 71%→73% (2pp off)
- 1 UNVERIFIABLE claim (orbit-switch data not in JSONL schema)

### 20. `crosspolytope/logbook.md`
- Elapsed time: table sums to 1112.8s, JSONL records 1095.1s (1.6% discrepancy)
- Reduction factor: "~27-30x" but m=2 has 20x (outside stated range)

### 21. `exp-hko-local-maximum/cut-and-ascent/logbook.md`
- Timing estimate in Ideas section: ~10s/trial→actual ~16s/trial

### 22. `dev-numerical-analysis/q-error/logbook.md`
- E_math upper bound: "1e-28"→actual max 2.6e-28 (loose order-of-magnitude)

### 23. `exp-sys-landscape/rotated-regular-products/logbook.md`
- All numerical claims verified
- **Code bug**: run.rs output paths point to `lagrangian-products/` but data lives in `rotated-regular-products/`

---

## Clean (no mismatches)

- `exp-sys-landscape/random-product-sample/logbook.md`
- `visualization/logbook.md`
- `exp-sys-landscape/gradient-ascent-general/logbook.md`
- `exp-sys-landscape/gradient-ascent-products/logbook.md`
- `exp-sys-landscape/variable-f-ascent/logbook.md`
- `dev-gradient-ascent/strategy-comparison/logbook.md`

---

## Code Bugs Found (not numerical claims)

1. **rotated-regular-products/run.rs**: output paths use `lagrangian-products/` subdirectory, but data files live in `rotated-regular-products/`. Running run.rs would fail.

2. **error-bounds/tests.rs line 45**: hardcoded path `verify-numerics/testdata/` — should be `error-bounds/testdata/`. All tests fail with "No such file or directory".

3. **unknown-predicates/run.rs line 134**: writes `dataset: "random-sample"` but JSONL has `"random-sweep"` and analyze.py filters on `"random-sweep"`. Re-running would produce data that analyze.py cannot read.

---

## Methodology Notes

- Each logbook was checked by an independent review-claims agent (sonnet model)
- Agents verified against JSONL data files, run.rs source, and analyze.py logic
- "UNVERIFIABLE" means the data source doesn't exist (deleted, gitignored, or from prior pipeline) — not necessarily wrong
- Agents can be overconfident; for any specific mismatch you want to act on, spot-check the data directly
- Most "stale data" mismatches follow a common pattern: JSONL was regenerated after the logbook was written
