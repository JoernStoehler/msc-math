# Review: Extend ablation with F=9,10 generics, non-simple polytopes, and scaling analysis

**Branch:** `ablation-ext`
**Base:** local `main` at `9ca403a`
**Date:** 2026-02-22

---

## 1. Build Verification

| Check | Status |
|---|---|
| `cargo test --lib` (crates/) | 180 passed, 0 failed, 28 ignored |
| `cargo clippy --lib` (crates/) | Clean (0 warnings) |
| `cargo build` (experiments/) | OK |
| `ruff check experiments/` | 2 pre-existing errors in `benchmark.py` (not in branch scope) |
| `git status` | Clean working tree |

## 2. What Changed (3 commits, ~750 LOC changed across 6 files)

1. **c366ea0** — Rust: extend dataset to F=9,10 generic polytopes + non-simple polytopes (bipyramids, cut simplices). Extract `make_bipyramid()` and `make_cut_simplex()` helpers. Regenerate JSONL (39→54 polytopes, 156→216 entries).
2. **0bde1a7** — Python: add `fit_scaling_exponent()` and `print_nonsimple_table()` analysis functions. Regenerate timing figure.
3. **886a71a** — LaTeX: add scaling exponent paragraph, non-simple table, update prose/tables for extended dataset. Update `reproduce.sh` comment.

## 3. Commit Quality

- Messages are descriptive and include bullet-point summaries
- Co-Authored-By present on all commits
- Code and data colocated in same commit (CLAUDE.md prefers separate commits for code vs data, but this is minor for experiment binaries)
- Logical separation: Rust → Python → LaTeX across three commits

## 4. Code Review

### Rust (ablation.rs)

**New code: `make_bipyramid()` (lines 788–813)**
Construction is mathematically correct. For a 3D polytope with k faces:
- Bipyramid has 2k facets, each apex lies on k facets
- Normalization is correct: `norm4 = sqrt(nx² + ny² + nz² + (h/a)²)`
- Heights correctly divided by norm4
- Unit normals verified (magnitude 1 after normalization)

**New code: `make_cut_simplex()` (lines 820–840)**
- Standard 4-simplex with centroid at origin, vertices at (±2, ...)
- Cutting plane `x₁ + c·x₂ ≤ 2` passes through v₀=(2,0,0,0)
- Normalization correct: `norm = sqrt(1 + c²)`
- Heights match expected distances from origin

**Dataset generation (main function)**
- F=9,10 added to random generic loop (line 866)
- Non-simple polytopes added as Part 4 (lines 972–1047)
- All polytopes pass `Polytope4D::new()` validation

**Potential concern — RNG state drift:**
Adding F=9,10 polytopes BEFORE Lagrangian products shifts the RNG state.
Result: all Lagrangian products (3x3, 3x4, 4x4) have different random data
compared to `main`. Generic F=5..8 are unchanged (generated first with same seed).
This is not a code bug — the Lagrangian products are still valid random polytopes,
and all variants agree on them. But it means LaTeX table values from `main` are stale.

### Python (ablation.py)

**`fit_scaling_exponent()` (lines 285–343)**
- Correct: fits log(A2/A0 ratio) = b·F + log(a) via `np.polyfit`
- R² computed correctly: `1 - ss_res/ss_tot`
- Guard against insufficient data (≥3 F values required)

**`print_nonsimple_table()` (lines 250–278)**
- Correctly includes both `non_simple` group and `regression_cut_simplex`
- Pruning percentage computation correct: `100 * (a2 - a3) / a2`

**Timing figure updates**
- Integer x-ticks added for clarity (lines 410–413)
- No issues

### LaTeX (ablation.tex)

**Well-written additions:**
- Scaling exponent paragraph is clear and well-motivated
- Non-simple table (tab:ablation-nonsimple) is informative
- Figure caption updated for F=5..10 range

## 5. Data Pipeline Verification

Python script run on branch data: **all 54 polytopes agree across all 4 variants** (max diff < 10⁻⁸).

### LaTeX numbers vs JSONL data

| Claim | LaTeX value | Actual (from JSONL) | Status |
|---|---|---|---|
| Dataset size | 54 polytopes × 4 variants = 216 | 216 entries | ✓ |
| F=8 generic A0/A1/A2/A3 (mean) | 16064 / 5347 / 136 / 136 | 16064.0 / 5347.0 / 136.0 / 136.0 | ✓ |
| F=9 generic A0/A1/A2/A3 (mean) | 125664 / 31633 / 391 / 391 | 125664.0 / 31632.6 / 390.8 / 390.8 | ✓ (rounded) |
| F=10 generic A0/A1/A2/A3 (mean) | 1112073 / 68985 / 531 / 531 | 1112073.0 / 68984.8 / 531.2 / 531.2 | ✓ (rounded) |
| **Lagrangian 4×4 A2/A3** | **497 / 497** | **569.2 / 569.2** | **✗ STALE** |
| Hypercube A0/A1/A2/A3 | 16064 / 5556 / 1970 / 1970 | 16064 / 5556 / 1970 / 1970 | ✓ |
| Non-simple table (all 6 rows) | Various | All match | ✓ |
| Scaling fit: a=28.3, b=−1.05, R²=0.96 | As stated | 28.35, −1.046, 0.962 | ✓ |
| "531 out of 1,112,073" | As stated | 531.2 / 1112073.0 | ✓ |
| "~3.5% for Lagrangian at F=8" | ~3.5% | 569.2/16064 = 3.54% | ✓ (prose is correct, table wrong) |
| "Hypercube least prunable at 12%" | 12% | 1970/16064 = 12.3% | ✓ |
| **"A1 prunes 59–94%"** | 59–94% | **Per-polytope range: 55.5%–96.1%** | **✗ Inaccurate** |

## 6. Issues

### Issue 1 (data error): Lagrangian 4×4 table entry is stale

**Table `tab:ablation-iterations`, row "Lagrangian 4×4 (F=8)":** claims A2=497, A3=497.
Actual data gives A2=A3=569 (mean of [608, 482, 560, 674, 522]).

**Root cause:** Adding F=9,10 generic polytopes before Lagrangian products shifted the RNG state.
The old value 497 was from `main`'s data; the branch regenerated data with different
Lagrangian products but didn't update this table row.

**Impact:** The actual Lagrangian A2/A0 ratio is 569/16064 = 3.54%, so the prose claim
"~3.5% for Lagrangian products" is coincidentally still correct. But the table is wrong.

**Fix:** Update table row to A2=569, A3=569 (or regenerate the table from the Python output).
Also note: Lagrangian 3x3 and 3x4 data also changed — if any other claims reference those,
they'd need updating too. (Currently no table rows reference 3x3 or 3x4 iterations.)

### Issue 2 (minor): "A1 prunes 59–94%" range is inaccurate

The actual per-polytope range across F=8–10 generic polytopes is 55.5%–96.1%.
This appears to be carried over from the old text ("59–65%") and widened for F=9,10
without checking the new per-polytope data.

**Fix:** Change to "55–96%" or qualify as "on average, 67–94% (F=8–10)".

### Issue 3 (cosmetic): "six non-simple polytopes (4 cut simplices and 2 bipyramids)"

The `non_simple` group has 5 entries (3 cut simplices + 2 bipyramids). The 4th cut simplex
(`regression_cut_simplex`, c=2.0) is in the `regression` group. The claim of "six non-simple"
is correct mathematically (all 6 are non-simple), but "4 cut simplices" requires counting
one from the regression group. This is fine as written — the Python nonsimple table already
includes both groups — just noting for awareness.

## 7. Strengths

- **Scaling analysis adds real value.** The exponential decay of the A2/A0 ratio (b ≈ −1.05)
  is a clean quantitative result that strengthens the ablation's thesis contribution.
- **Non-simple polytopes are well-chosen.** Bipyramids (F=10, apex on 5 facets) create a
  dramatic A3-vs-A2 gap (98% pruned), demonstrating that A3 matters for non-simple inputs.
  Cut simplices at varying depths (c=1.5, 2.5, 4.0) show consistent A3 pruning.
- **All 54 polytopes agree across all 4 variants** — strong correctness evidence.
- **Construction code is mathematically sound.** Both `make_bipyramid` and `make_cut_simplex`
  produce valid polytopes with the advertised non-simplicity properties.
- **Commit messages are descriptive** with bullet-point summaries.

## 8. Pre-existing Issues

- `ruff check` reports 2 F541 errors in `experiments/benchmark/benchmark.py` (f-strings
  without placeholders). Not introduced by this branch.

---

## Executive Summary

**Summary of findings:**
1. **Table bug (Lagrangian 4×4 row):** A2=497 and A3=497 are stale values from `main`'s
   data. Branch regenerated JSONL with different Lagrangian products (RNG state drift from
   added F=9,10 generics), but didn't update this table row. Actual values: A2=A3=569.
2. **Prose inaccuracy ("59–94%"):** A1 pruning range doesn't match per-polytope data
   (actual: 55.5%–96.1%). Minor.
3. **Everything else checks out:** All 54 polytopes agree, scaling fit is correct, non-simple
   table is correct, construction math is sound, builds and tests pass.

**Recommendation:** Fix Issue 1 (update table row) and optionally Issue 2 (update range),
then merge. The data and analysis are solid; only the LaTeX table has a stale entry.

**Time investment:** ~45min review
