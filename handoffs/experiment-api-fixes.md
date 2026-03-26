# Handoff: Experiment API Fixes

Three experiment binaries broken by library API drift. All mechanical — no design decisions needed.

## Tasks

### 1. gradient-search + generate-seeds (non-trivial)

**Broken imports:** `capacity_derivatives_h`, `volume_derivatives_h`, `normals_f64()`, `heights_f64()`.

**What happened:** Library migrated to dual-vertex `_a` API. This experiment was on a branch and never caught up.

**Fix approach (draft):**
- Replace `capacity_derivatives_h` / `volume_derivatives_h` with `capacity_derivatives_a` / `volume_derivatives_a` from `symplectic::derivatives`
- The `_a` functions return `Vec<Vector4<f64>>` (one gradient per facet in R^4) instead of separate scalar ∂/∂h and tangent-projected ∂/∂n
- Step-bound computation currently uses h_k and n_k separately — needs rethinking in a_i terms. Look at how gradient-descent/run.rs and hko-neighborhood/run.rs handle this (they're already migrated).
- `normals_f64()` / `heights_f64()` → use `polytope.dual_vertices_f64()` and compute h = 1/|a|, n = a/|a| locally if still needed for step bounds

**Risk:** The step-bound logic is the tricky part. The other 4 migrated experiments may not all use step bounds the same way gradient-search does (it overshoots intentionally). Don't assume copy-paste from gradient-descent will work — read gradient-search's logic first.

**Verify:** `cargo build --release --bin gradient_search --bin generate_seeds`, then smoke-test on 5 seeds.

### 2. visualization (mechanical)

**Broken imports:**
- `recover_base_point` → merged into `recover_and_verify`
- `verify_orbit` → merged into `recover_and_verify`
- `build_directed_adjacency_matrix` → renamed to `build_transition_matrix`
- Type mismatch on the result

**Fix approach:** Grep the library for `recover_and_verify` and `build_transition_matrix` signatures. Adapt call sites. The function merging means fewer calls, not different logic.

**Verify:** `cargo build --release --bin visualization`, then spot-check one polytope export matches existing data.

### 3. orbit-recovery (mechanical)

Same `recover_base_point` / `verify_orbit` → `recover_and_verify` rename as visualization.

**Verify:** `cargo build --release --bin orbit_recovery`, then `cargo run --release --bin orbit_recovery` should reproduce `orbit-recovery.jsonl`.

### 4. Q error threshold (investigate, don't just bump)

hko-neighborhood triggers panic at E=1.68e-6 (threshold 1e-6) on a perturbed HKO pentagon. Caught by experiment, but worth understanding.

**Investigate:** Is this a one-off near-degenerate case, or does the threshold need revisiting? Check `saddle_point_solver.rs:504` for context. The 1e-6 threshold was validated by q-error experiment on 1.1M nodes — but those were all non-perturbed polytopes.

**Don't:** Silently raise the threshold without understanding why it's exceeded.

## Session notes

- Use a worktree for this batch — all changes are to experiments, no library modifications needed for tasks 2-3.
- Task 1 (gradient-search) touches the most code and has the most risk. Consider doing it separately if the others are quick.
- After all fixes: `cd experiments && cargo build --release` should compile all 22 binaries.
