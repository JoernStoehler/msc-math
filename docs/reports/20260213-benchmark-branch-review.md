**STALE (2026-03-22):** References `review-rust skill` (old skill name) and `experiments/CLAUDE.md` (deleted). Conventions now live in `.claude/skills/`.

# Code Review: benchmark Branch

**Reviewer:** Claude Sonnet 4.5 (review-rust skill)
**Date:** 2026-02-13
**Branch:** `/workspaces/worktrees/benchmark`
**Base:** `main` at `3232bef` (local, not origin/main)
**Commits:** 10 (696ddfa...ef5e986)

---

## Decision: MERGE

**Recommendation:** Merge to `main` without modifications.

| Criterion | Status | Evidence |
|-----------|--------|----------|
| Tests pass | ✓ | `cargo test`: 25 passed, 0 failed, 40.48s |
| Clippy clean | ✓ | Zero warnings |
| Deletions verified | ✓ | All replacements appropriate (see table below) |
| Conventions | ✓ | Rust/Python/LaTeX CLAUDE.md compliance |
| Data pipeline | ✓ | CSV → Python → JSON → LaTeX coherent |
| Commits | ✓ | Atomic, well-messaged, logical progression |

---

## Summary

This branch replaces the old timing model (based on 5 symmetric polytopes) with a comprehensive benchmark using 76 random polytopes across F=5-12. The key finding: random polytopes exhibit a lower growth rate (4.82x vs 5.74x per facet) because their sparser adjacency graphs allow better pruning.

**Scope:**
- Added: benchmark.rs (Rust binary), benchmark.py (Python orchestration), benchmark.tex (LaTeX writeup), data files
- Deleted: old timing binaries/scripts, superseded .md writeups, ad-hoc profiling code
- Modified: Cargo.toml (binary entries), main.rs (constant refactor), minor Python fixes

**Impact:**
- Better timing model: 76 data points vs 5, fitted to random polytopes (more representative)
- Updated practical limits: F≤8 for large datasets (interpolation within fitted range)
- Crosspolytope infeasibility documented: ~400-day ETA with hk2017 algorithm

---

## Strengths

1. **Thorough deletion verification:** Every deleted file has a clear replacement or documented reason for removal. No functionality lost unintentionally.

2. **Multi-language quality:** Code follows conventions across all three languages (Rust, Python, LaTeX). Each file has proper headers, clear structure, appropriate documentation.

3. **Data pipeline integrity:** CSV columns match Python parser exactly. JSON model parameters match LaTeX writeup. Stats table in .tex matches computed values from CSV to 0.1ms precision.

4. **Statistical correctness:** Log-linear regression for exponential model is appropriate. R² = 0.997 indicates excellent fit. Median computation handles even/odd sample counts correctly.

5. **Reproducibility:** Deterministic seed (42), timeout strategy (180s per sample), and release mode requirement all documented. Pipeline can be rerun from zero.

6. **Commit quality:** Atomic commits with clear messages. Logical progression: add code → run → capture data → write analysis → cleanup. Co-authored-by present throughout.

7. **Scientific rigor:** Crosspolytope partial run documented with progress log (18 min, m=2-8/16, ETA ~422 days ≈ 400 days stated in .tex). Growth rate comparison (4.82x vs 5.74x) explained by adjacency graph density.

---

## Deletion Verification Table

| Deleted File | LOC | Purpose | Replacement | Verdict |
|--------------|-----|---------|-------------|---------|
| time_capacity.rs | 34 | Time 6 known polytopes | benchmark.rs (76 random polytopes, broader scope) | ✓ Appropriate |
| timing_model.py | 170 | Fit model, project datasets | benchmark.py (fit + orchestrate + figure) | ✓ Core replaced* |
| profile_capacity.rs | 16 | Ad-hoc profiling stub | None (debugging code) | ✓ No replacement needed |
| timing_model.md | 32 | Model writeup | benchmark.tex (thesis section) | ✓ Migrated to LaTeX |
| timing_data.csv | 5 | Old timing data | benchmark.csv (76 rows) | ✓ Better data |
| Cargo.toml binaries | — | profile-capacity, time-capacity | benchmark binary | ✓ Matches deletions |

\* `timing_model.py` had a `project_dataset_size()` function (estimates polytopes/hour for dataset planning). This was intentionally removed — projections field deleted from `timing_model.json`. Not a loss, as the model is now fitted to random polytopes and provides better practical limits directly.

---

## Minor Findings (No Action Required)

1. **Crosspolytope ETA rounding:** Log shows 422 days, .tex states ~400 days. Acceptable rounding to nearest hundred for readability.

2. **Model formula notation:** Python uses `a * b^F`, LaTeX uses `a \cdot b^F`. Both correct, minor style difference between code and prose.

3. **Dataset projection removal:** Old `timing_model.py` had dataset size projection. Intentionally removed (not needed with better model + practical limits table). Future dataset planning can reference practical limits in benchmark.tex instead.

4. **Figure not committed:** `benchmark_timing.png` exists in worktree (69KB) but is gitignored. Expected per `experiments/CLAUDE.md` (figures/ is gitignored, regenerated from scripts).

---

## Verification Checklist (Complete)

**Thesis integration:**
- ✓ `experiments.tex` includes `benchmark.tex` at line 17
- ✓ Cross-references valid: `\ref{tab:benchmark-stats}`, `\ref{fig:benchmark-timing}`
- ✓ Figure path correct: `../experiments/figures/benchmark_timing.png`

**Data integrity:**
- ✓ benchmark.csv: 77 lines (76 data + 1 header)
- ✓ Stats table matches CSV: median, mean, min, max all ±0.1ms
- ✓ Model parameters: a=3.2e-7, b=4.82 (JSON matches .tex)
- ✓ Crosspolytope log: ETA ~422 days ≈ 400 days stated in .tex

**Build:**
- ✓ `cargo test` passes (25 passed, 0 failed, 4 ignored)
- ✓ `cargo clippy` clean (zero warnings)
- ✓ Working tree clean (no uncommitted changes)

---

## Meta-Learnings for /review Skill Expansion

**What worked:**
1. **Read deleted code first** — Prevented false alarms about "missing functionality" by comparing old vs new directly
2. **Three-language conventions** — Rust/Python/LaTeX each have different CLAUDE.md files, all checked systematically
3. **Data pipeline tracing** — CSV → Python → JSON → LaTeX verified end-to-end (columns, params, stats)
4. **Build verification early** — cargo test/clippy caught integration issues before deep review

**What could improve:**
1. **Deletion verification** — Manual comparison tedious. Could use subagent to produce "deletion verification table" (file, LOC, purpose, replacement).
2. **Statistical correctness** — Manual verification of log-linear regression, R² formula. Could use "math QC" subagent for model fitting checks.
3. **Cross-file coherence** — Checking CSV columns match Python parser is manual. Could automate with schema validation or inline assertions.

**Skill design insight:**
- Multi-language review (Rust+Python+LaTeX) requires convention files for each language + cross-file coherence checks (data pipeline, integration points)
- Deletion verification is a distinct phase — can't trust commit messages alone, must compare functionality
- Scientific correctness (statistical analysis, model fitting) overlaps with code quality but needs domain-specific checks

---

## Recommendation

**Merge to `main`** without modifications. The branch is high quality across all dimensions: code, tests, documentation, commits, and scientific rigor. No improvements needed before merge.

**Post-merge:** Consider extracting deletion verification and multi-language review patterns into the planned `/review` skill to cover Rust+Python+LaTeX systematically.
