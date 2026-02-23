# Review: sys-optimization branch

**Reviewer:** Claude Opus 4.6 (automated)
**Date:** 2026-02-22
**Branch:** `sys-optimization` (commits `071cc52..637f03a`, 11 commits)
**Base:** local `main` at `e279a2c`

## Summary

The branch adds a complete experiment pipeline: Rust binary, Python analysis, JSONL data, PNG figures, LaTeX writeup, and reproduce.sh integration. The experiment computes the gradient of the systolic ratio w.r.t. facet heights using the envelope theorem, then takes finite gradient steps to try to push sys past 1.

**Recommendation: Merge with caveats.** The code produces correct results (validated numerically). The main issues are (1) misleading documentation of the sign convention in the envelope theorem formula, (2) a factual error in the .tex prose, and (3) clippy warnings. None block merge, but issues 1 and 2 should be fixed before thesis inclusion.

## Phase 1: Fast Checks

| Check | Result |
|-------|--------|
| `cargo test --lib` | PASS (180 passed, 28 ignored) |
| `cargo clippy --lib` | PASS (0 warnings) |
| `cargo build --release` (experiments) | PASS (2 warnings: dead fields) |
| `cargo clippy` (experiments) | FAIL: 3 errors in sys_optimization.rs, 1 pre-existing in ablation.rs |
| `ruff check` | Pre-existing error in benchmark.py (not this branch) |
| Working tree | Clean |
| Deleted files | None |

**Clippy errors in sys_optimization.rs (new on this branch):**
1. `subset` field in `ValidOrbit` is never read
2. `capacity_uncertain` and `iterations` fields in `InstrumentedResult` never read
3. `k % 2 == 0` should be `k.is_multiple_of(2)` (in copied Heap's algorithm)

These are minor and fixable with `#[allow(dead_code)]` or field removal.

## Phase 2: Deletion Verification

No files deleted. All changes are additions.

## Phase 3: Code and Data Review

### Finding 1 (HIGH): Envelope theorem sign convention — misleading documentation

**Severity: High (documentation), None (correctness)**

The code formula `-nu * beta[i0] / (2.0 * q_sq)` is **numerically correct** — I validated this with finite-difference checks across multiple polytopes. The d_cap values match FD to ~1e-8 relative error, and the negated formula matches 0/140 polytopes.

However, the documentation is contradictory and misleading:

- The **doc comment** (line 637) states: `dA/dh_k = ν · β_{i₀} / (2Q²)` (positive sign)
- The **inline comment** (line 656) states: `dA/dh_k = −ν · β_{i₀} / (2Q²)` (negative sign)
- The **code** (line 664) computes: `-nu * beta[i0] / (2.0 * q_sq)` (negative sign)
- The **.tex** (line 54) states: `dA/dh_k = ν β_{i₀} / (2Q²)` (positive sign)

The resolution is a **sign convention mismatch**:

- In the standard ("forward traversal") convention used in the .tex and doc comment, ν is positive for typical orbits, and the formula is `+ν β / (2Q²)`.
- In the code's "reverse traversal" convention (where the code maximizes Q > 0 by choosing the reverse-traversal representative), ν is negative for the winning orbit, and the formula `-ν β / (2Q²)` gives the same positive result.

The inline comment's explanation (lines 658-662) is confused: it claims a sign flip in ν between conventions, then derives the positive formula, but the code uses the negative formula. The claim "Empirical cross-check confirms the negated sign" (line 663) has no corresponding cross-check code.

**Recommendation:** Fix the documentation to be internally consistent. Either:
(a) State the formula in the code's convention explicitly: "In the code's reverse-traversal convention where Q > 0, the Lagrange multiplier ν is negative, giving dA/dh_k = -ν β / (2Q²) > 0."
(b) Or remove the sign discussion and just say: "The formula is validated against finite differences."

The .tex should also clarify the convention or use the code's convention.

**CRITICAL FLAG FOR JORN: The envelope theorem derivation (both .tex and code comments) has NOT been verified by Jorn. The numerical results are correct, but the written mathematical justification is muddled and needs human verification of the sign convention before thesis inclusion.**

### Finding 2 (MEDIUM): Factual error in .tex prose

**Severity: Medium**

The .tex writeup (line 135-136) states:
> "The best sys achieved is 0.837, starting from a polytope with sys = 0.758 (a random 4×5 Lagrangian product)."

The data shows:
- Best sys after step: 0.8369, achieved by `random_4x5_1` which started at **sys = 0.7349**
- The polytope with max starting sys (0.7579) is `random_4x5_6`, which actually **worsened** to sys = 0.7294

The .tex conflates two different polytopes: the one with the highest starting sys and the one that achieves the highest final sys. The correct statement should identify `random_4x5_1` as the starting polytope (sys = 0.735, not 0.758).

### Finding 3 (LOW): Instrumented HK2017 doc comment says "with A2 pruning" but has no pruning

**Severity: Low**

Line 520: `/// Instrumented version of ehz_capacity with A2 pruning.`

The function does NOT implement A2 pruning — it enumerates ALL subsets and cyclic permutations without an upper-bound cutoff. Since it needs ALL valid orbits (not just the best), pruning would be incorrect here. The comment should say "without pruning" or "collects ALL valid orbits."

### Finding 4 (LOW): Many Lagrangian products have zero runner-up gap

43 of 80 Lagrangian products have `runner_up_gap < 1e-14` (effectively zero). At these orbit-switching boundaries, the capacity is non-smooth and the envelope theorem derivative is one-sided. The code mentions this caveat (line 643-644) but does not flag these polytopes in the output data.

This means the gradient direction is unreliable for ~31% of the dataset. Some of the 18/137 smallest-step failures (13%) may be attributable to this.

**Recommendation:** Add a flag `is_degenerate` (runner_up_gap < threshold) to the sensitivity JSONL and discuss in the .tex how many polytopes are affected.

### Finding 5 (INFO): KKT solver is SVD-only — confirmed correct

The experiment binary copies the KKT solver from the library and extends it to return ν. The commit `2e16225` removes the LU path, leaving SVD-only as the code comment states. This matches the library's production path. Verified that constants (EPS_BETA_POSITIVE, SVD_CONDITION_TAU, etc.) match the current library values.

### Finding 6 (INFO): Data pipeline is complete and consistent

Pipeline trace:
1. **Rust binary** reads `random-sweep.jsonl` + `random-product-sweep.jsonl`, filters F ≤ 10 → 140 polytopes
2. Writes `sys-optimization-sensitivity.jsonl` (140 rows) and `sys-optimization-steps.jsonl` (700 rows = 140 × 5 step fractions)
3. **Python script** reads both JSONLs, produces 3 PNGs + 1 stats .tex table
4. **LaTeX writeup** includes figures and stats table

Numbers consistency check:
| Claim (.tex) | Stats table | Computed from data | Match? |
|---|---|---|---|
| 140 polytopes | 140 | 140 | Yes |
| 127 (91%) improved | 127 | 127 | Yes |
| mean Δsys = 0.033 | 0.0327 | 0.0327 | Yes |
| median Δsys = 0.014 | 0.0135 | 0.0135 | Yes |
| best sys = 0.837 | 0.8369 | 0.8369 | Yes |
| "starting from sys=0.758" | n/a | **0.7349** | **NO** |
| 94% favorable | n/a | 94.6% | Yes |

### Finding 7 (INFO): Cross-check with library capacity

The code includes a runtime cross-check (line 1036-1045): the instrumented capacity must match `ehz_capacity()` from the library to within 1e-8. This provides confidence that the instrumented solver produces the same results as the production code.

### Finding 8 (INFO): Commit quality

11 commits with clear, descriptive messages following the project conventions. Data regeneration commits are separate from code changes. The branch history shows a clean progression: initial experiment → analytical derivative → LU removal → data regeneration → review cleanup.

### Finding 9 (LOW): Step bound computation for non-simple vertices is very conservative

The `compute_step_bound` function (lines 790-818) handles non-simple vertices (>4 incident facets) with a very crude bound: `t_max ≤ slack / max_g`. This significantly underestimates the actual step bound for Lagrangian products (which have many non-simple vertices). The conservative bound means gradient steps are smaller than necessary, potentially missing larger improvements.

## Recommendation

**Merge with the following fixes:**

1. **Must fix before thesis:** Finding 2 (factual error in .tex prose about starting polytope)
2. **Should fix before thesis:** Finding 1 (clarify sign convention documentation in code and .tex)
3. **Nice to fix:** Findings 3, 4 (mislabeled comment, degenerate flag)
4. **Can ignore:** Clippy warnings (experiment code, not library), step bound conservatism

The experiment is well-structured, produces correct results, and follows repo conventions. The envelope theorem approach is a genuine advance over finite differences (exact, cheaper, no numerical differentiation noise). The mathematical derivation needs Jorn's verification before thesis inclusion.
