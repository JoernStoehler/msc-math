# Review: algorithm-improvements branch (ablation study + A2 promotion + polish)

**Branch:** `algorithm-improvements` (29 commits, merged into main 2026-02-22)
**Base:** local `main` at `18ab808`
**Date:** 2026-02-22
**Status:** All findings resolved. See "Resolution" section at end of file.

---

## Build Verification

| Check | Status |
|-------|--------|
| `cargo test --lib` (from crates/) | 180 passed, 0 failed, 28 ignored |
| `cargo clippy --lib -- -D warnings` | Clean |
| `cargo build --release` (from experiments/) | All binaries build |
| `ruff check experiments/` | 1 warning (benchmark.py:267, pre-existing on main) |
| `latexmk` (from thesis/) | Compiles, no undefined `\ref` |
| Data agreement | All 38 polytopes × 4 variants agree (0 disagreements) |

## Commit Quality

20 commits total. Most have clear "why" messages. Co-authored-by present on all. No fixup/squash commits. Atomic logical changes.

Minor: several commits add documentation/docs first, then code in later commits — this is fine for the authoring workflow but means some early commits reference code that doesn't exist yet.

## Findings

### CRITICAL — A1 Mislabeling After A2 Promotion

**What happened:**
1. The A2 promotion (commit `9a9ea1b`) added `build_directed_adjacency_matrix()` to the library's `ehz_capacity_pruned`
2. The ablation binary's A1 variant calls `ehz_capacity_pruned` (line 750 of ablation.rs)
3. After promotion, A1 now runs A2-level pruning (vertex adjacency + directed ω₀ condition)
4. The ablation binary's A2 variant (`ehz_capacity_a2`, line 535) uses its own local implementation with `EPS_DIRECTED = 1e-8` tolerance

**Evidence:**
- 36/38 polytopes: A1 iterations == A2 iterations (identical pruning graphs)
- 2/38 polytopes: A1 iterations < A2 iterations (library uses strict `>= 0.0`, ablation uses `<= 1e-8`)
- A1 timing ≈ A2 timing ≈ A3 timing in current data (all ~130× over A0 at F=8)

**Impact on ablation narrative:**
- The .tex table (lines 228–249) shows OLD data (pre-A2-promotion) where A1 was 3–42× slower than A2
- The current data shows A1 ≈ A2 ≈ A3 — the table is wrong
- The caption "The directed ω₀ condition (A2) provides the dominant speedup" is no longer supported
- The figure (ablation_timing.png) correctly shows current data where A1/A2/A3 cluster together

**Actual data vs stale table:**

| Group | F | A0 (actual/table) | A1 (actual/table) | A2 (actual/table) |
|-------|---|-------------------|-------------------|-------------------|
| Generic | 5 | 0.7/0.5 | **0.09/0.5** | 0.11/0.1 |
| Generic | 6 | 5.0/3.2 | **0.31/2.5** | 0.34/0.2 |
| Generic | 7 | 25.8/26.9 | **0.59/13.7** | 0.61/0.6 |
| Generic | 8 | 227/234 | **1.75/74** | 1.78/1.8 |

The A1 column is wrong by 5–42×. A0 and A2 are approximately correct.

**Root cause:** The A2 promotion was correct for the library (production code should use the best pruning available). But the ablation study needs its own isolated A1 implementation that uses vertex-adjacency-only, not the library function.

**Additional confound:** The library's `ehz_capacity_pruned` uses `solve_kkt` (new condition-number SVD with `SVD_CONDITION_TAU`), while the ablation's A2 uses `solve_kkt_full` (old gap-ratio SVD). This means even if A1 and A2 had the same adjacency graph, timing differences could be partly attributable to the SVD solver, not just pruning. For a clean ablation study, all variants should use the same KKT solver.

**Suggested fix (for Jörn to decide):**
Add a true A1 function in the ablation binary that uses `build_adjacency_matrix` (undirected) instead of `build_directed_adjacency_matrix`. All variants should use the same local `solve_kkt_full` solver for an apples-to-apples comparison. Regenerate data and update the table and narrative.

### MEDIUM — Appendix Intro Promises 3 Subsections, Delivers 4

`appendix-numerical.tex` lines 18–30 describe "three parts": near-singular, three-valued, error-tracking. The ablation subsection (§A.4) was added at line 41 via `\input{../experiments/ablation/ablation}` without updating the intro.

**Fix:** Add "Section~\ref{sec:ablation-pruning} analyses adjacency graph pruning" to the intro paragraph.

### MEDIUM — R_k Notation Clash

Ablation.tex line 69: "Write R_k = J₀ n_k for the characteristic direction on F_k."
Thesis (general-case-algorithm-proof.tex line 80, simple-minimizer-existence.tex line 356): R_i = (2/h_i) J₀ n_i.

The ablation uses J₀n_k (unit characteristic direction), the thesis uses (2/h_k)J₀n_k (Reeb vector with contact normalization). These differ by a factor 2/h_k.

**Fix:** Either match the thesis convention (R_k = (2/h_k)J₀n_k) or use a different letter (e.g., V_k = J₀n_k) to avoid confusion. Since the ablation only uses the direction (not the speed), the characteristic direction is more natural for the ablation context — a short note clarifying the distinction would suffice.

### MEDIUM — Shaved Hypercube Construction (Example A.11)

The shaved hypercube construction places 0 on the boundary of the polytope (one halfspace has h=0). This violates the standing assumption that K contains the origin in its interior (required by `[def:polytope]`). The polytope is degenerate under the standard definition.

**Fix:** Shift all heights slightly (h_shaved = ε instead of 0), or note that the example works in the limit and the A2≠A3 phenomenon is a limiting behavior.

### LOW — Tolerance Asymmetry Between Library and Ablation

Library `build_directed_adjacency_matrix`: `omega0(n_j, n_i) >= 0.0` (strict)
Ablation `build_directed_adjacency`: `omega0(n_i, n_j) <= EPS_DIRECTED` (with 1e-8 tolerance)

These are mathematically equivalent modulo tolerance (omega0 is antisymmetric). The library is stricter. This is fine for production (prune more aggressively), but the ablation study should document which tolerance it uses and why.

### LOW — Pre-existing: benchmark.py Ruff Error

`experiments/benchmark/benchmark.py` line 267: f-string without placeholders. This exists on main, not introduced by this branch.

## Strengths

1. **Clean architecture:** Ablation binary is self-contained (~1000 lines) with all 4 variants, shared infrastructure, and clean data output. The experiment folder convention (all artifacts colocated) is well followed.

2. **Mathematical content:** Lemma A.9 (Transition Feasibility IFF), Corollary A.10 (Ridge Sufficiency), and Example A.11 (Shaved Hypercube) are well-structured with clear proof steps. (Unreviewed by Jörn — see Items for Jörn below.)

3. **σ refactor:** Clean separation of algebraic vs physical orbit direction with reversal at the output boundary. Consistent across hk2017 and billiard modules.

4. **A2 promotion to library:** Correct engineering decision — production code should use the best pruning. The issue is only that the ablation binary wasn't updated to compensate.

5. **Experiment writeup polish:** Random-sweep and random-product-sweep observations are well-calibrated (specific numbers, no overclaiming).

6. **Agreement verification:** All 38 polytopes × 4 variants agree on capacity to within 1e-5. Strong correctness signal.

## Items for Jörn

### Must verify (unreviewed mathematical content)

1. **Lemma A.9 (Transition Feasibility IFF)** — States F_i → F_j feasible iff ω₀(n_i, n_j) ≥ 0 AND LP on blocking set B is feasible
2. **Corollary A.10 (Ridge Sufficiency)** — For simple polytopes, directed vertex-adjacency (A2) = Reeb-flow feasibility (A3)
3. **Example A.11 (Shaved Hypercube)** — Non-simple polytope where A2 ≠ A3

### Must decide

4. **A1 mislabeling fix** — How to handle: add true A1 to ablation binary? Rewrite narrative? (See CRITICAL finding above)
5. **Appendix intro update** — "three parts" → mention ablation
6. **R_k notation** — Match thesis or use different letter

## Executive Summary

**Summary of findings:**
1. **A1 mislabeling (CRITICAL):** After A2 promotion to library, ablation's A1 variant runs A2-level pruning. The timing table is stale (A1 column wrong by 5–42×), the figure is correct but now shows A1≈A2≈A3 (no differentiation). Needs Jörn's decision on fix approach.
2. **Appendix intro (MEDIUM):** Says "three parts", has four. Quick fix.
3. **R_k notation (MEDIUM):** Ablation uses J₀n_k, thesis uses (2/h_k)J₀n_k. Needs clarification.
4. **Shaved hypercube (MEDIUM):** 0 on boundary violates polytope definition. Needs adjustment.
5. **Unreviewed math:** Lemma A.9, Corollary A.10, Example A.11 need Jörn's verification.

**Recommendation:** Fix the MEDIUM issues (appendix intro, R_k notation, shaved hypercube — 10 min each). The A1 mislabeling requires Jörn's decision on approach before fixing. Do NOT merge until the A1 issue is resolved and timing table/narrative match the data.

**Time investment:** ~90min review

---

## Resolution (post-review fixes, 2026-02-22)

All five findings have been addressed:

1. **A1 mislabeling (CRITICAL):** Fixed. Added true `ehz_capacity_a1` using undirected vertex adjacency + same `solve_kkt_full` solver. Replaced timing table with iteration count table (Jörn's direction). Data regenerated.
2. **Appendix intro:** Fixed. "three parts" → "four parts", added §A.4 mention.
3. **R_k notation:** Fixed. Renamed to V_k = J₀n_k throughout ablation.tex, with clarifying note distinguishing from thesis R_k = (2/h_k)J₀n_k.
4. **Shaved hypercube:** Replaced entirely. The original was broken (improper facets made it simple). New "cut simplex" construction: 4-simplex ∩ {x₁+2x₂≤2}, producing 6-facet non-simple polytope with empirically confirmed A2≠A3 (39 vs 33 orderings).
5. **Unreviewed math:** Lemma A.9, Corollary A.10 unchanged (still need Jörn's verification). Example A.11 rewritten with cut simplex — computationally verified but proof argument needs Jörn's check.
