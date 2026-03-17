# TASKS

Deferred tasks, ideas, and identified work items. Grows stale; that's fine.

## Handoff files (ready for separate sessions)

- `handoffs/experiment-deduplication.md` — extract 4× duplicated KKT solver + derivatives to library
- `handoffs/tube-algorithm.md` — implement tube algorithm (9 steps from tube-algorithm-plan.md)
- `handoffs/hko-neighborhood.md` — complete HKO neighborhood writeup and assess if more experiments needed

## Active design work (this session / next session with Jörn)

KKT solver rework: dual-vertex parameterization, projection-based solver, β > 0 as LP, near-null eigenvalue handling. Needs spec file written collaboratively with Jörn before implementation. Code first, thesis follows.

## Test data pipeline restructuring

**Problem:** Default test suite takes 7 min. Top 10 tests = 1000s of 1050s total. Root cause: fixture-consuming tests regenerate all 33 polytope capacity values on every run instead of loading cached data.

**Profiling data (cargo nextest, 2026-03-17):**

| Test | Time | Category |
|------|------|----------|
| `catalog_determinism` | 162s | Fixture regeneration |
| `fixture_staleness_check` | 158s | Fixture regeneration |
| `literature_capacity_values` | 98s | Full EHZ on ~8 polytopes |
| `volume_scales_with_fourth_power` | 92s | Proptest with qhull |
| 5 fixture-consuming tests | 85-89s ea | Block on fixture generation |
| `random_polytopes_pass_validation` | 46s | Proptest |

**Work items:**
1. `generate_capacity_fixtures` writes to `fixtures/capacity_fixtures.json` (on-disk, checked in)
2. 7 fixture-consuming tests load JSON instead of regenerating (85-98s → <1s each)
3. Staleness detection per `data-pipeline` skill (semantic + generator hash)
4. `known_polytopes` constructors get `LazyLock` caching (~50ms × 30 tests saved)
5. Proptest: fewer cases in default suite, full cases in `#[ignore]`

**Target:** Default suite < 2 min. Full suite (with `--ignored`) < 10 min.

**Depends on:** Migration merge (tests exist on migration-scaffold branch).

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

### Audit Reeb vector factor: R = 2/h J₀ n, not 1/h J₀ n — AUDITED, CLEAN

**Audit result (2026-03-14):** No wrong instances found.
- Thesis .tex: consistently uses R_i = (2/h_i) J₀ n_i (basic-definitions.tex:478, general-case-algorithm-proof.tex:80, simple-minimizer-existence.tex:488, lagrangian-product-algorithm-proof.tex:85/92, appendix-numerical.tex:205, tube-algorithm.tex:339)
- Library code: `reeb_vector()` returns J₀ n (direction only, documented as such). Only caller needing magnitude is `recover.rs:167` which correctly uses `* (2.0 / h)`.
- The `reeb_vector()` function name could be confusing since it returns direction, not the actual Reeb vector. Consider renaming to `reeb_direction()` if/when refactoring.

### KKT solver: implement projection-based variant

**What:** Implement a second KKT solver variant that solves constraints first, then projects H onto the constraint space:
1. Solve `(N^T | h^T) β = (0 | 1)` → (m-5)-dim affine solution space
2. Project H onto constraint space → (m-5)×(m-5) symmetric system H'
3. Eigendecompose H' → near-null eigenvalues = constant-action directions
4. β > 0 check as LP feasibility on the projected null space
5. Recover Lagrange multipliers μ, ν from Hβ + Nμ + hν = 0

**Why:** Cleaner separation of concerns. Constraints satisfied by construction (no residual). Near-null eigenvalues have clear geometric meaning. Conservative inclusion of near-null eigenvalues is safe (constraints can't be violated). Better for the β > 0 feasibility check.

**Interaction:** Keep existing augmented-system solver for ablation comparison. Both variants should agree on Q values.

### Consider switching thesis from (n, h) to dual vertices a_i

**What:** Replace the (n_i, h_i) ∈ S³ × R>0 parameterization with dual polytope vertices a_i = n_i/h_i ∈ R⁴\{0} throughout the thesis.

**Why:**
- Unit-length constraint on n is never used mathematically
- a_i ∈ R⁴\{0} is unconstrained — simpler parameter space
- Reeb vector becomes R_i = 2 J₀ a_i (direct, no division)
- Gradients ∂/∂a_i need no tangent-space projection
- Code already stores dual vertices as rationals

**Scope:** Mostly a thesis notation change. Needs impact assessment: which formulas change, how many .tex files are affected, whether code interfaces change.

**Ordering:** Should happen BEFORE the KKT projection refactor if we're doing both, to avoid refactoring twice.

### Experiment cleanup: extract shared code to library

**What:** 4 experiments (gradient-descent, sys-optimization, hko-neighborhood, omega-obstacle) each contain 930-2347 LOC copies of an instrumented KKT solver + derivative computation. Extract to shared module.

**Identified duplications:**
- Instrumented KKT solver (build_kkt_system, solve_kkt_svd_path, find_positive_beta_nd, ehz_capacity_instrumented)
- Derivative computation (compute_capacity_derivatives_analytical, _normal, _fd; compute_volume_derivatives_analytical, _normal)
- Adjacency/permutation logic (combinations, heap_permutations, build_adjacency_matrix) — 6+ copies

**Scope:** Extract instrumented KKT + derivatives to `crates/` as public modules. Update 4 experiment binaries to use shared code. Verify outputs unchanged.
