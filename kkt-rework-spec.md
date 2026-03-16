# KKT Solver Rework — Status & Plan

**Last updated:** 2026-03-16 (after Phase 2 implementation session)

## Current status

Phase 2 (projection-based solver) is implemented and tested. The KKT module has been restructured from a single file into a directory module with four sub-components. 262 library tests pass (46 new + 216 existing).

**Key commit:** `5a3e174` — restructure KKT module and implement projection solver.
**Previous commit:** `d0e57ad` — Phase 1, switch Polytope4D to dual vertex representation.

## What exists now

```
crates/src/kkt/
  mod.rs                 — QP, Solution, Verdict types; solve() entry point
  projection_solver.rs   — NEW: projection-based solver (Steps 1-5), 11 tests
  constraint_solver.rs   — NEW: SVD-based Cβ=d solver, 17 tests
  margin_search.rs       — NEW: max-margin Chebyshev center search, 18 tests
  augmented.rs           — MOVED: legacy (m+5) augmented solver, unchanged
```

**The projection solver** (`kkt::solve(qp)`) takes a context-free `QP { c, d, h }` and returns `Solution { verdict, q, beta, margin }`. It does NOT know about symplectic geometry — callers assemble C, d, H from dual vertices.

**Cross-variant agreement** verified on simplex, hypercube, and HKO pentagon: the projection solver and augmented solver produce matching capacities.

**The augmented solver** (`kkt::augmented::solve_kkt(normals, heights, perm)`) is unchanged and still used by hk2017 and billiard. Callers import from `crate::kkt::augmented::` directly.

## Design decisions made during implementation

1. **No re-export grab-bags.** `mod.rs` defines its own types (QP, Solution, Verdict) and declares `pub mod` for sub-modules. Callers import from the source module directly (e.g. `crate::kkt::augmented::solve_kkt`). Temporary backward-compat re-exports exist in mod.rs but are marked for removal.

2. **SVD for constraint solving** (not QR). SVD gives rank detection, particular solution, and null basis in one decomposition. Implementation detail: nalgebra's thin SVD doesn't produce full V^T for underdetermined systems (p < m), so the constraint solver pads C with zero rows before SVD.

3. **Capacity comparison, not Q comparison, for cross-variant tests.** The augmented solver uses (normals, heights) with β satisfying Σβ_k h_k = 1. The projection solver uses dual vertices aᵢ with β satisfying Σβ_k = 1. Different β, same Q, same capacity = 0.5/Q. Tests compare capacity.

4. **Iterative coordinate ascent for margin search** (k ≥ 2 case), not a proper LP solver. This is a heuristic that satisfies "no false Infeasible" — if it fails to find β > 0, the verdict is Indeterminate, not False. A proper LP can be added later if needed.

## What's next

### Phase 3: Wire projection solver into capacity pipeline

Migrate hk2017 and billiard to call `kkt::solve()` instead of `kkt::augmented::solve_kkt()`. This means:
- Add `assemble_qp(dual_verts, perm)` helper in each algorithm module
- Replace the solve_kkt call with kkt::solve + verdict matching
- The certified/uncertain tracking maps to True/Indeterminate/False
- Verify all existing capacity tests still pass

**Depends on:** nothing (code is ready). Can be done by a subagent.

### Phase 4: Fix experiment binaries

15 experiment binaries are broken (144 call sites using old Polytope4D API from Phase 1). Mechanical migration. Can be parallelized across experiment files.

### Phase 5: Thesis .tex update (deferred)

Update thesis notation from (n, h) to aᵢ. Separate session after code is stable.

### Open items from the algorithm design doc

- **Accumulator abstraction** (Part C.6): not yet needed. The capacity-only accumulator is just "track max Q, prune below" — inline in hk2017/billiard. Add the abstraction when action-gap or all-orbits consumers appear.
- **Rational fallback for Indeterminate nodes** (Decision 4): the rational solver (`kkt_rational.rs`) exists but isn't wired into the trinary pipeline yet. This is Phase 3 work.
- **Error bounds for the projection solver** (§5 of old spec): not derived. The augmented solver has the q-error-bound lemma; the projection solver just computes Q = (1/2)βᵀHβ directly. Empirical comparison on large datasets is the path forward.
- **Multiplier recovery** (Step 5): not implemented yet. The projection solver returns Q, β, margin, verdict — but not Lagrange multipliers. Add when needed (gradient computation).

## Reference documents

- `handoffs/kkt-module-spec.md` — detailed implementation spec for subagents (§5-§13)
- Algorithm design doc — the comprehensive plan (was passed inline in the implementation task; covers Parts A-D, decisions, implementation phases)
