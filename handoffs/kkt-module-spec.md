# KKT Module Spec — Implementation Reference

**Purpose:** Single source of truth for subagents implementing the new KKT module.
**Read alongside:** The algorithm design doc (plan file) for mathematical context.

---

## 1. Design priorities (from Jörn)

1. **Math-code correspondence:** Reading the code, you can verify it implements the math. Variable names match thesis notation. Steps match the algorithm description.
2. **Test-math correspondence:** Tests verify specific mathematical propositions. Test names reference what they verify, not which function they call.
3. **Correctness architecture:** Smoke tests run always (`cargo test --lib`), slow tests are `#[ignore]`.
4. **Downstream suitability:** The API is what experiments actually need.
5. **Maintainability:** Clean design for the future, not backward compat with legacy.

---

## 2. File tree

```
crates/src/kkt/
  mod.rs                    — Types (QP, Solution, Verdict), entry point solve()
  projection_solver.rs      — The projection-based solver (Steps 1-5)
  constraint_solver.rs      — Step 1: solve Cβ = d → affine solution set
  margin_search.rs          — Step 3: max-margin feasibility in affine subspace
  augmented.rs              — Legacy augmented (m+5) solver, kept for ablation
  tests.rs                  — Integration tests: solver properties, cross-variant agreement
```

**Dependency graph (within kkt/):**
```
mod.rs
  └─ projection_solver.rs
       ├─ constraint_solver.rs
       └─ margin_search.rs
  └─ augmented.rs (independent, for ablation)
```

**Callers (outside kkt/):**
- `algorithms/hk2017/mod.rs` — assembles QP from dual vertices, calls `kkt::solve()`
- `algorithms/billiard/mod.rs` — same pattern
- Experiment binaries — same pattern (after migration)
- `kkt_rational.rs` — stays at current location, separate implementation

---

## 3. Types (in mod.rs)

```rust
use nalgebra::{DMatrix, DVector};

/// Constrained quadratic program: max (1/2) βᵀHβ  s.t. Cβ = d, β > 0.
///
/// For EHZ capacity: C encodes closure + normalization, H encodes symplectic action,
/// β are dwell-time coefficients. But this struct is context-free.
///
/// # Dimensions
/// - C: p × m (p constraints, m variables)
/// - d: p × 1
/// - H: m × m, symmetric
pub struct QP {
    pub c: DMatrix<f64>,
    pub d: DVector<f64>,
    pub h: DMatrix<f64>,
}

/// Trinary verdict for feasibility of β > 0.
///
/// **Critical invariant:** FALSE is never returned unless certified.
/// When in doubt, return INDETERMINATE. The accumulator handles resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Certified feasible: all β_k > ε. Safe to use Q and β.
    True,
    /// Certified infeasible: no β > 0 exists in the solution set.
    False,
    /// Ambiguous: β has near-zero components, or near-null eigenvalues
    /// prevent definitive classification. Q is still valid.
    Indeterminate,
}

/// Result of solving a QP.
///
/// Q is always valid when verdict ≠ False. β is the best point found.
/// margin = min_k β_k quantifies clearance from the positivity boundary.
#[derive(Clone, Debug)]
pub struct Solution {
    pub verdict: Verdict,
    /// Optimal objective value: Q = (1/2) βᵀHβ.
    /// Constant over the solution set (null space of projected Hessian).
    /// Valid for True and Indeterminate. Zero for False.
    pub q: f64,
    /// Solution vector. For True: all components > 0. For Indeterminate:
    /// best-effort max-margin point. For False: empty or not meaningful.
    pub beta: Vec<f64>,
    /// min_k β_k. Positive → True, negative → False, near-zero → Indeterminate.
    pub margin: f64,
}
```

**Verdict thresholds** (constants in mod.rs):
```rust
/// β_k > EPS_MARGIN_TRUE → component is certified positive.
const EPS_MARGIN_TRUE: f64 = 1e-9;

/// β_k < -EPS_MARGIN_FALSE → component is certified negative (infeasible).
const EPS_MARGIN_FALSE: f64 = 1e-9;

/// Minimum Q to consider meaningful (avoids 1/Q blow-up in capacity).
pub const EPS_Q_POSITIVE: f64 = 1e-15;
```

**Verdict logic:**
```
margin > EPS_MARGIN_TRUE   → True
margin < -EPS_MARGIN_FALSE → False
otherwise                  → Indeterminate
```

---

## 4. Entry point (in mod.rs)

```rust
/// Solve the constrained QP: max (1/2) βᵀHβ s.t. Cβ = d, β > 0.
///
/// Uses the projection method: project to the constraint null space,
/// optimize the reduced objective, search for β > 0 via max-margin LP.
///
/// # Panics
/// - If dimensions are inconsistent (C.ncols ≠ H.nrows, etc.)
pub fn solve(qp: &QP) -> Solution {
    // Validate dimensions
    // Delegate to projection_solver::solve_projected(qp)
}
```

---

## 5. constraint_solver.rs

### Mathematical statement

Given C ∈ R^{p×m} and d ∈ R^p, find the affine solution set { x : Cx = d }.

Decompose as: x = x₀ + Vα, where x₀ is a particular solution and V ∈ R^{m×k}
has orthonormal columns spanning ker(C), k = m - rank(C).

### Public interface

```rust
/// Solution of the linear constraint system Cx = d.
pub struct ConstraintSolution {
    /// Particular solution x₀ (minimum-norm).
    pub x0: DVector<f64>,
    /// Orthonormal null-space basis V ∈ R^{m×k}. Columns span ker(C).
    /// k = m - rank. Empty (m×0) if full-rank.
    pub null_basis: DMatrix<f64>,
    /// Numerical rank of C.
    pub rank: usize,
}

/// Solve Cx = d via SVD with threshold rank detection.
///
/// Returns None if the system is inconsistent (d is not in the column space of C).
///
/// # Rank detection
/// Singular values σ_i with σ_i < σ_max · EPS_RANK_THRESHOLD are treated as zero.
pub fn solve_constraints(
    c: &DMatrix<f64>,
    d: &DVector<f64>,
) -> Option<ConstraintSolution>
```

### Algorithm

1. Compute thin SVD: C = U Σ Vᵀ (nalgebra `c.svd(true, true)`)
2. Rank detection: r = count of σ_i > σ_max · threshold
3. Consistency check: ‖(I - U_r U_rᵀ) d‖ < tolerance → inconsistent if large
4. Particular solution: x₀ = V_r Σ_r⁻¹ U_rᵀ d (minimum-norm)
5. Null basis: V_null = V[:, r:m] (columns of V for zero singular values)
6. Return (x₀, V_null, r)

### Constants

```rust
/// Relative threshold for SVD rank detection: σ_i < σ_max · τ → null.
/// 1e-10: well above machine epsilon, catches near-rank-deficiency.
const EPS_RANK_THRESHOLD: f64 = 1e-10;

/// Maximum residual ‖Cx₀ - d‖ for consistency.
const EPS_CONSISTENCY: f64 = 1e-8;
```

### Edge cases

| Case | rank | null_basis cols | Behavior |
|------|------|-----------------|----------|
| m < p, consistent | m | 0 | Unique solution (overdetermined but consistent) |
| m < p, inconsistent | < p | — | Return None |
| m = p, full rank | p | 0 | Unique solution |
| m > p, full rank(C) = p | p | m - p | Underdetermined: (m-p)-dim null space |
| rank(C) < min(m,p) | < min(m,p) | m - rank | Rank-deficient |
| C = 0 | 0 | m | d must be 0; x₀ = 0, full null space |

### Tests (in #[cfg(test)] module)

Smoke tests (always run):
- **known_rank_systems**: 3×6 with rank 3, verify null_basis has 3 columns
- **overdetermined_consistent**: 5×3 with exact solution, verify x₀
- **overdetermined_inconsistent**: 5×3 with no solution, verify None
- **identity_constraints**: C = I, d = e_1, verify x₀ = e_1, empty null basis
- **zero_rhs**: Cx = 0, verify x₀ ≈ 0, null basis spans ker(C)
- **round_trip**: for random (C, d) with known solution, verify Cx₀ ≈ d and CV ≈ 0
- **singular_values_near_threshold**: σ = [1.0, 1e-11, 1e-15], verify rank = 1

---

## 6. margin_search.rs

### Mathematical statement

Given base point β₀ ∈ R^m and direction matrix V ∈ R^{m×k}, find:

  max  min_j (β₀ + Vα)_j   over α ∈ R^k

This is the Chebyshev center of the polytope {α : β₀ + Vα ≥ 0} — the point
with maximum clearance from all positivity constraints.

### Public interface

```rust
/// Result of the max-margin feasibility search.
pub struct MarginResult {
    /// The maximum margin: max_α min_j (β₀ + Vα)_j.
    /// Positive → feasible, negative → infeasible, near-zero → ambiguous.
    pub margin: f64,
    /// The optimal α achieving the margin (in the null-space coordinates).
    pub alpha: DVector<f64>,
    /// The solution point β = β₀ + V·α.
    pub beta: DVector<f64>,
}

/// Find the point in the affine subspace {β₀ + Vα} with maximum minimum component.
///
/// For k = 0: margin = min(β₀), no search needed.
/// For k = 1: analytic solution (midpoint of feasible interval).
/// For k ≥ 2: iterative coordinate ascent on the most-violated constraint.
///
/// The iterative method is not exact but satisfies the critical property:
/// if it returns margin < -ε, the subspace is genuinely infeasible.
/// If it fails to find margin > +ε, the verdict should be INDETERMINATE (not FALSE).
pub fn find_max_margin(
    beta0: &DVector<f64>,
    null_basis: &DMatrix<f64>,
) -> MarginResult
```

### Algorithm

**k = 0:** No degrees of freedom.
```
margin = min_j β₀[j]
alpha = empty
beta = β₀
```

**k = 1:** Analytic solution (current find_positive_beta_1d logic).
```
For each j: bound on α from β₀[j] + v[j]·α ≥ 0
  v[j] > 0 → α ≥ -β₀[j]/v[j] (lower bound)
  v[j] < 0 → α ≤ -β₀[j]/v[j] (upper bound)
  v[j] ≈ 0 → skip (β₀[j] is fixed)
If lo ≥ hi: infeasible
α = (lo + hi) / 2 (midpoint for max margin)
margin = min_j (β₀[j] + v[j]·α)
```

**k ≥ 2:** Iterative coordinate ascent (current find_positive_beta_nd logic, refined).
```
α = 0
for iter in 0..MAX_ITER:
    β = β₀ + V·α
    j* = argmin_j β[j]
    if converged (margin change < ε): break
    gradient of β[j*] w.r.t. α: g = V[j*, :]ᵀ
    if ‖g‖² ≈ 0: can't improve, break
    step α along g to push β[j*] toward target
margin = min_j (β₀ + V·α)[j]
```

### Constants

```rust
/// Maximum iterations for the coordinate ascent (k ≥ 2 case).
/// 100 suffices for m ≤ 16, k ≤ 11.
const MAX_ITER: usize = 100;

/// Convergence tolerance: stop when margin improvement < this.
const EPS_CONVERGENCE: f64 = 1e-14;

/// Component of null-space vector below this is treated as zero (k=1 case).
const EPS_DIRECTION_ZERO: f64 = 1e-15;
```

### Tests (in #[cfg(test)] module)

Smoke tests:
- **trivial_feasible**: β₀ = [1,1,1], V empty → margin = 1.0
- **trivial_infeasible**: β₀ = [-1,1,1], V empty → margin = -1.0
- **one_dim_feasible**: β₀ = [-1, 2], V = [1, 0]ᵀ → α = 1, margin = 0 or better
- **one_dim_infeasible**: β₀ = [-1, -2], V = [1, -1]ᵀ → margin < 0
- **one_dim_midpoint**: β₀ = [0, 0], V = [1, -1]ᵀ → α = 0 (midpoint), margin = 0
- **two_dim_feasible**: β₀ = [-1,-1,3], V = [[1,0,0],[0,1,0]]ᵀ → margin > 0
- **margin_is_tight**: verify that returned margin = min(β) exactly
- **null_basis_empty_equals_k0**: V is m×0, same as k=0 case

---

## 7. projection_solver.rs

### Mathematical statement (Part C.2 of algorithm design)

Solve: max (1/2) βᵀHβ  subject to  Cβ = d, β > 0.

**Step 1 — Solve constraints.** Cβ = d → particular solution β₀, null-space basis V.
If inconsistent: return False.

**Step 2 — Project objective.** H' = VᵀHV (reduced Hessian), b' = VᵀHβ₀ (reduced gradient).
Solve H'α = b' for α₀ via eigendecomposition of H'.

**Step 3 — Determine search space.** Partition eigenvalues of H' into retained (|λ| > threshold)
and null (|λ| ≤ threshold). Null eigenvectors of H' become additional search directions W.
The full search space for β > 0 is: β = β₀ + V(α₀ + Wγ).

**Step 4 — Max-margin search.** Find γ maximizing min_k β_k in the affine subspace.
Classify verdict from margin.

**Step 5 — Compute Q.** Q = (1/2) βᵀHβ. Constant over the solution set (because H'
has zero eigenvalues along the search directions).

### Public interface

```rust
/// Solve the QP via constraint projection.
///
/// Returns Solution with verdict, Q, β, and margin.
///
/// # Algorithm steps (matching Part C.2 of algorithm design)
/// 1. Solve constraints Cβ = d → (β₀, V, rank)
/// 2. Project: H' = VᵀHV, solve H'α = b' → (α₀, W)
/// 3. Max-margin search in β₀ + V(α₀ + Wγ)
/// 4. Compute Q = (1/2) βᵀHβ
pub(crate) fn solve_projected(qp: &QP) -> Solution
```

### Algorithm (detailed)

```
Step 1: Constraint projection
  result = solve_constraints(&qp.c, &qp.d)
  if None → return Solution { verdict: False, q: 0, beta: [], margin: -inf }
  (β₀, V, rank) = result
  k = V.ncols()  // null-space dimension = m - rank

  if k == 0:
    // Unique β from constraints. No optimization needed.
    margin = min(β₀)
    q = 0.5 * β₀ᵀ H β₀
    verdict = classify(margin)
    return Solution { verdict, q, beta: β₀, margin }

Step 2: Project and optimize
  H' = VᵀHV                    // k × k symmetric matrix
  b' = Vᵀ(H β₀)               // k × 1 vector (note: b' = VᵀHβ₀, not negative)

  // Eigendecompose H' = P Λ Pᵀ
  eig = H'.symmetric_eigen()

  // Partition eigenvalues
  λ_max = max(|λ_i|)
  threshold = λ_max * EPS_EIGEN_THRESHOLD
  retained = { i : |λ_i| > threshold }
  null_indices = { i : |λ_i| ≤ threshold }

  // Particular solution for H'α = b' (using retained eigenvalues only)
  // α₀ = Σ_{i ∈ retained} (pᵢᵀ b' / λᵢ) pᵢ
  α₀ = pseudoinverse_solve(eig, b', retained)

  // Null-space directions of H' (in α-space)
  W_alpha = [p_i for i in null_indices]    // k × |null_indices| matrix

Step 3: Compose search space and find max-margin point
  // The search point is: β = β₀ + V·(α₀ + W_alpha·γ)
  // Rewrite as: β = β_base + V_search · γ
  //   where β_base = β₀ + V·α₀
  //         V_search = V · W_alpha
  β_base = β₀ + V * α₀
  V_search = V * W_alpha          // m × |null_indices|

  margin_result = find_max_margin(&β_base, &V_search)

Step 4: Compute Q and classify
  β = margin_result.beta
  q = 0.5 * βᵀ H β
  margin = margin_result.margin
  verdict = classify(margin)

  return Solution { verdict, q, beta: β.as_slice().to_vec(), margin }
```

### Eigenvalue partitioning constants

```rust
/// Eigenvalue threshold for H' (the reduced Hessian).
/// Near-null eigenvalues mean Q varies little along those directions
/// but β varies a lot — so include them in the search space.
///
/// Same role as EIGEN_CONDITION_TAU in augmented.rs.
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

/// Absolute floor: if max|λ| < this, treat H' as zero (Q = 0 along all directions).
const EPS_EIGEN_FLOOR: f64 = 1e-12;
```

### Edge cases

| m | rank(C) | k = m-rank | H' dims | Situation |
|---|---------|------------|---------|-----------|
| ≤ p | ≤ m | 0 | — | Overdetermined constraints. k=0: unique β or inconsistent |
| p+1 | p | 1 | 1×1 | One free variable. H' is scalar. Trivial optimization. |
| > p | p | m-p | (m-p)×(m-p) | Generic case. Project and optimize. |
| any | < p | m-rank | larger | Rank-deficient constraints. Larger search space. |

### Tests (in #[cfg(test)] module)

**Smoke tests (always run):**
- **inconsistent_constraints**: C = I_5, d = [0,0,0,0,2], H irrelevant → False
- **unique_beta_positive**: m=5, rank=5, β₀ > 0 → True, Q correct
- **unique_beta_negative**: m=5, rank=5, some β₀ < 0 → False
- **one_free_variable**: m=6, rank=5, verify Q matches hand computation
- **q_constant_along_null_space**: for rank-deficient H', verify Q(β₀ + Vα) = Q(β₀) for random α in null space

**Mathematical proposition tests (always run):**
- **prop_constraint_satisfaction**: for every returned β with verdict ≠ False, verify ‖Cβ - d‖ < ε
- **prop_kkt_stationarity**: for True solutions, verify Hβ + Cᵀμ ≈ 0 for some μ (least-squares)
- **prop_q_is_half_beta_h_beta**: verify returned Q = (1/2)βᵀHβ to machine precision
- **prop_margin_equals_min_beta**: verify margin = min(β) exactly

**Cross-variant tests (always run, using fixture polytopes):**
- **augmented_agrees_on_simplex**: both solvers return same Q for 4-simplex
- **augmented_agrees_on_hypercube**: both solvers return same Q for hypercube
- **augmented_agrees_on_hko**: both solvers return same Q for HKO pentagon

---

## 8. augmented.rs

The existing code from the old `kkt.rs`, moved as-is. Contains:
- `build_kkt_system(normals, heights, perm)` — builds the (m+5)×(m+5) augmented matrix
- `solve_kkt(normals, heights, perm)` → `Option<KktResult>` — the legacy entry point
- `KktResult` — the legacy result type
- `q_from_beta(normals, perm, beta)` — Q computation from normals (symplectic-aware)
- `EPS_BETA_POSITIVE`, `EPS_Q_POSITIVE` — legacy thresholds
- Internal: `solve_kkt_eigen_path`, `try_pseudoinverse_with_threshold`, `find_positive_beta_1d/nd`

**Changes from current kkt.rs:**
- Add `pub(crate)` or `pub` visibility to items that mod.rs re-exports
- No algorithmic changes

**Purpose:** Ablation comparison. Experiments can run both solvers on the same input
and compare Q values. Eventually the augmented solver may be retired, but not yet.

---

## 9. mod.rs — full structure

```rust
pub mod constraint_solver;
pub mod margin_search;
mod projection_solver;
mod augmented;

// New general interface
pub use constraint_solver::ConstraintSolution;
pub use margin_search::MarginResult;

// Types defined here: QP, Solution, Verdict (see §3)

/// Solve the constrained QP.
pub fn solve(qp: &QP) -> Solution {
    projection_solver::solve_projected(qp)
}

/// Compute Q = (1/2) βᵀHβ from pre-assembled H and β.
pub fn q_value(h: &DMatrix<f64>, beta: &[f64]) -> f64 {
    let b = DVector::from_column_slice(beta);
    0.5 * b.dot(&(h * &b))
}

// ── Legacy interface (for augmented solver / backward compat during migration) ──

pub use augmented::KktResult;
pub use augmented::EPS_BETA_POSITIVE;
pub use augmented::EPS_Q_POSITIVE;
pub use augmented::solve_kkt;
pub use augmented::build_kkt_system;
pub use augmented::q_from_beta;
```

---

## 10. Assembly — how callers build QP

This lives **outside** the KKT module (in hk2017/billiard or a shared helper).
The KKT module never imports symplectic geometry.

```rust
use crate::geom::symplectic::omega0;
use crate::kkt::QP;

/// Assemble the KKT quadratic program from dual vertices and a facet permutation.
///
/// Given dual vertices aᵢ (from Polytope4D) and permutation σ:
/// - C = [a_{σ(1)}ᵀ; ...; a_{σ(m)}ᵀ; 1ᵀ]  (5 × m, closure + normalization)
/// - d = [0; 0; 0; 0; 1]
/// - H[i][j] = ω₀(a_{σ(i)}, a_{σ(j)})      (m × m, symmetric action matrix)
///
/// In the dual vertex representation aᵢ = nᵢ/hᵢ, all heights are 1,
/// so the normalization row of C is all ones.
fn assemble_qp(dual_verts: &[Vector4<f64>], perm: &[usize]) -> QP {
    let m = perm.len();
    let p = 5;

    let mut c = DMatrix::zeros(p, m);
    let mut h = DMatrix::zeros(m, m);

    for i in 0..m {
        let a = &dual_verts[perm[i]];
        // Closure constraints: rows 0-3 of C
        for d in 0..4 {
            c[(d, i)] = a[d];
        }
        // Normalization constraint: row 4 of C
        c[(4, i)] = 1.0;

        // Action matrix: H[i][j] = ω₀(a_{perm[i]}, a_{perm[j]})
        for j in (i + 1)..m {
            let val = omega0(a, &dual_verts[perm[j]]);
            h[(i, j)] = val;
            h[(j, i)] = val;
        }
    }

    let d = DVector::from_fn(p, |i, _| if i == 4 { 1.0 } else { 0.0 });
    QP { c, d, h }
}
```

**Where this function lives:** Either as a free function in each algorithm module
(hk2017, billiard), or as a shared helper. Start with duplication (one copy per
algorithm module) — extract only if they diverge or a third consumer appears.

---

## 11. Caller migration pattern

### Current (hk2017/mod.rs):
```rust
let normals = polytope.normals_f64();
let heights = polytope.heights_f64();
// ...
if let Some(result) = solve_kkt(&normals, &heights, perm) {
    let q_val = result.q_corrected;
    // ...
}
```

### New:
```rust
let dual_verts = polytope.dual_vertices_f64();
// ...
let qp = assemble_qp(dual_verts, perm);
let result = kkt::solve(&qp);
match result.verdict {
    Verdict::True => {
        let q_val = result.q;
        let action = 0.5 / q_val;
        // update best_certified
    }
    Verdict::Indeterminate => {
        let q_val = result.q;
        let action = 0.5 / q_val;
        // update best_uncertain
    }
    Verdict::False => {
        // skip — no valid orbit
    }
}
```

The certified/uncertain tracking in ehz_capacity() maps naturally to the trinary
verdict: True → certified, Indeterminate → uncertain, False → skip.

---

## 12. Test plan

### Tier 1: Always run (`cargo test --lib`)

**KKT sub-components:**
- constraint_solver: known-rank, round-trip, edge cases (§5 tests)
- margin_search: feasible/infeasible/boundary (§6 tests)

**KKT solver properties:**
- Mathematical propositions: constraint satisfaction, KKT stationarity, Q = ½βᵀHβ (§7 tests)
- Small hand-computable QPs (m=5,6,7)

**Cross-variant agreement:**
- Both solvers return same Q (within tolerance) on fixture polytopes (§7 cross-variant tests)

**Capacity pipeline:**
- Known polytope capacities match published values (existing tests, migrated to new API)

### Tier 2: On demand (`cargo test -- --ignored`)

**Large-scale:**
- Random polytope sweeps: both solvers agree on Q across F=5..12
- f64 vs rational agreement on INDETERMINATE nodes
- Performance regression baselines

---

## 13. Subagent assignments

Each subagent reads this spec file + their assignment section. They implement exactly
what's described: types, algorithm, tests. They do NOT modify files outside their scope.

**Assignment A: constraint_solver.rs**
- Read: §5 (constraint_solver.rs spec)
- Write: `crates/src/kkt/constraint_solver.rs`
- Produces: ConstraintSolution type, solve_constraints function, tests

**Assignment B: margin_search.rs**
- Read: §6 (margin_search.rs spec)
- Write: `crates/src/kkt/margin_search.rs`
- Produces: MarginResult type, find_max_margin function, tests

**Assignment C: projection_solver.rs**
- Read: §7 (projection_solver.rs spec), and must reference §5 and §6 interfaces
- Write: `crates/src/kkt/projection_solver.rs`
- Produces: solve_projected function, tests
- **Depends on A and B** — uses their public interfaces

**Assignment D: mod.rs + augmented.rs + tests.rs**
- Read: §8, §9, §12
- Write: `crates/src/kkt/mod.rs`, adjust `crates/src/kkt/augmented.rs` visibility
- Write: `crates/src/kkt/tests.rs` (cross-variant integration tests)
- Produces: QP/Solution/Verdict types, solve() entry point, legacy re-exports

**Assignment E: caller migration**
- Read: §10, §11
- Modify: `crates/src/algorithms/hk2017/mod.rs`, `crates/src/algorithms/billiard/mod.rs`
- Produces: migrated capacity pipeline using new kkt::solve()
