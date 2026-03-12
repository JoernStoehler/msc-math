# TASKS

Deferred tasks, ideas, and identified work items. Grows stale; that's fine.

## Identified refactors

### Unify `find_positive_beta_1d` / `find_positive_beta_nd` in kkt.rs

**What:** `crates/src/kkt.rs` has two separate functions for finding β > 0 in the KKT null space: a 1d interval-arithmetic path and an nd coordinate-ascent heuristic. These solve the same problem — find a feasible point in `{β₀ + V·α | β > 0}` — which is a standard LP regardless of null-space dimension.

**Why refactor:**
- The 1d/nd split has no profiling justification
- The nd "coordinate ascent" is an ad-hoc heuristic, not a standard algorithm
- An LP formulation (maximize `min_j βⱼ`) handles all dimensions uniformly
- The nd path is the only untested code path in kkt.rs (no known input triggers a 2D+ null space)

**Thesis/code tension:** The main thesis (`lem:rank-deficiency-dismissal` in `general-case-algorithm-proof.tex`) proves that pairs with δβ ≠ 0 in the null space are *redundant* — a smaller pair dominates, so the algorithm may discard them. The code does the opposite: when the system is near-singular, it searches the null space for β > 0. These aren't contradictory (the lemma says "may discard", and the code handles *near*-singular systems where rank deficiency is approximate), but the relationship needs to be made explicit:
- Main thesis (exact): rank-deficient → discard (dominated by smaller pair)
- Appendix-numerical (approximate): near-singular → the pseudoinverse β₀ may have some β_i < 0 due to noise; shifting along approximate null-space directions can recover feasibility without changing Q (which is constant along null directions)
- The "how to find β > 0" is a numerical implementation detail that belongs in appendix-numerical, not the main proof. It's dimension-agnostic (LP feasibility in the affine subspace).

**Scope:** Replace both functions with a single LP-based approach, add appendix-numerical writeup explaining the numerical null-space search, verify on existing regression tests.
