# Handoff: Remaining API Fixes + Migration

Updated 2026-03-26. Tasks 2-3 (visualization, orbit-recovery) completed this session.

## Remaining tasks

### 1. gradient-search + generate-seeds — probably skip

**Status:** Broken (imports nonexistent `capacity_derivatives_h` etc.). This experiment is being superseded by the new `boundary-crossing` experiment (see TASKS.md `experiment-quality` section). Fixing it may not be worth the effort unless it's needed as reference for boundary-crossing.

**If fixing anyway:** see gradient-descent/run.rs and hko-neighborhood/run.rs for how the `_a` API is used. The step-bound logic needs rethinking in a_i terms — that's the non-trivial part.

### 2. Q error threshold (investigate, don't just bump)

hko-neighborhood triggers panic at E=1.68e-6 (threshold 1e-6) on perturbed HKO. sys-optimization hits E=7.15e-6 on a gradient-stepped polytope (|λ_min|=2.29e-9). Both are near-degenerate polytopes where the smallest KKT eigenvalue is near machine epsilon.

**Investigate:** Is the error bound formula `E = |r| / |λ_min|` the right metric when λ_min is tiny? The actual residual |r| is small (6e-8) — the bound is pessimistic because of the small eigenvalue. Check `saddle_point_solver.rs:504`.

**Don't:** Silently raise the threshold without understanding why it's exceeded.

### 3. math.tex migration to a_i notation

`kkt/math.tex` and `algorithms/math.tex` still present formulas in (n_i, h_i) notation. Rewrite to use a_i = n_i/h_i throughout. The library code and derivative API already use a_i.

**Scope:** Notation changes only — the mathematical content doesn't change, just the parameterization used to express it. Check that all formulas remain correct after substitution.

**Verify:** Build math.pdf and review that formulas are consistent.

## Completed (this session)

- ✅ visualization/run.rs — API renames (commit a9edb7d)
- ✅ orbit-recovery/run.rs — API renames (commit a9edb7d)
- ✅ sys-optimization/run.rs — InputRow deserialization fix (commit c6ba95f)
