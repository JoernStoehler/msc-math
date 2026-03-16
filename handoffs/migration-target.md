# Migration Target State

What the codebase looks like when the migration is done.

---

## crates/src/ — Rust library

### Module tree

```
crates/src/
├── lib.rs                                  # Re-exports + 5-line crate doc
│
├── geom/
│   ├── mod.rs                              # "Geometry primitives for convex polytopes in R^4"
│   │                                       #   Lists every file below with one-line description
│   ├── polytope.rs                         # Polytope4D: the central type (dual vertices, rational + f64)
│   ├── polytope_test.rs                    # Polytope4D construction, accessors, invariants
│   ├── construction_validation_test.rs     # Construction error paths: too few facets, zero normal, unbounded, ...
│   ├── skeleton.rs                         # Face lattice + facet_centroid method
│   ├── skeleton_test.rs                    # Skeleton construction, facet_centroid correctness
│   ├── symplectic_form.rs                  # J₀ matrix, ω₀ bilinear form, coordinate convention (q₁,q₂,p₁,p₂)
│   ├── symplectic_form_test.rs             # ω₀ antisymmetry, J₀²=−I, Lagrangian subspace checks
│   ├── volume.rs                           # 4D volume via qhull triangulation
│   ├── volume_test.rs                      # Volume computation vs known values
│   ├── volume_properties_test.rs           # Volume scaling, positivity, simplex formula
│   ├── polygon.rs                          # 2D convex polygon constructors (regular, rotated)
│   ├── polygon_test.rs                     # Polygon area, vertex count, regularity
│   ├── lagrangian_product.rs               # K_q ×_L K_p product constructor from two 2D polygons
│   ├── lagrangian_product_test.rs          # Product facet count, Q/P classification, volume
│   ├── cross_product_4d.rs                 # 4D cross product of three vectors
│   ├── cross_product_4d_test.rs            # Orthogonality, magnitude, orientation
│   ├── validation.rs                       # Polytope boundedness and redundancy checks
│   ├── validation_test.rs                  # Validation accept/reject on known polytopes
│   ├── rational_arithmetic.rs              # Exact rational number type and operations
│   ├── rational_arithmetic_test.rs         # f64↔rational roundtrip, sign agreement, pipeline consistency
│   ├── vertex_enumeration.rs               # Exact vertex enumeration from halfspaces over Q
│   ├── vertex_enumeration_test.rs          # Vertex coords, affine rank, boundedness for simplex/hypercube
│   ├── vertex_enumeration_linalg_test.rs   # Low-level: rational det4, solve4, rank_over_q, cross_product
│   ├── qhull.rs                            # qhull FFI wrapper (pub(crate))
│   ├── reeb_trajectory.rs                  # Piecewise-linear Reeb flow simulation on polytope boundary
│   ├── reeb_trajectory_test.rs             # Trajectory closure, facet sequence, segment count
│   ├── known_polytopes.rs                  # Named constructors: HKO2024, simplex, hypercube, pentagon, products
│   └── test_utils.rs                       # Test-only polytope constructors and helpers
│
├── kkt/
│   ├── mod.rs                              # QP struct, Solution, Verdict enum, classify_margin
│   │                                       #   Lists every file below with one-line description
│   ├── qp_assembly.rs                      # Polytope4D + permutation → QP matrices (C,d,H) or augmented system
│   ├── qp_assembly_test.rs                 # Assembly output matches hand-computed matrices
│   ├── saddle_point_solver.rs              # (m+5)×(m+5) eigendecomposition solver (was: augmented.rs)
│   ├── saddle_point_solver_test.rs         # Solver correctness, rank-deficient cases, error bounds
│   ├── constraint_solver.rs                # Solve Cx=d for particular solution + null space basis via SVD
│   ├── constraint_solver_test.rs           # Solution satisfies Cx=d, null space orthogonality
│   ├── beta_feasibility.rs                 # Max-margin LP search for β>0 in affine solution set (was: margin_search.rs)
│   ├── beta_feasibility_test.rs            # Feasible/infeasible/indeterminate classification
│   ├── projection_solver.rs                # Project to constraint null space, optimize reduced objective
│   ├── projection_solver_test.rs           # Projection solver agrees with saddle-point solver
│   ├── rational_solver.rs                  # Exact rational KKT solver (was: kkt_rational.rs)
│   └── rational_solver_test.rs             # Exact vs f64 agreement, null-space handling
│
├── algorithms/
│   ├── mod.rs                              # "Three capacity algorithms + shared infrastructure"
│   │                                       #   Lists every file below with one-line description
│   ├── capacity_accumulator.rs             # Certified/uncertain candidate tracking across enumeration
│   ├── capacity_accumulator_test.rs        # Submit/finalize, gap invariant, empty accumulator
│   ├── facet_adjacency.rs                  # Undirected + directed (ω₀-aware) facet adjacency matrices
│   ├── facet_adjacency_test.rs             # Adjacency correctness on simplex, hypercube, products
│   ├── hk2017/
│   │   ├── mod.rs                          # ehz_capacity + ehz_capacity_unpruned (separate fns, both use accumulator)
│   │   ├── literature_test.rs              # Capacity values for simplex, hypercube, pentagon, products
│   │   ├── kkt_edge_cases_test.rs          # Rank-deficient, degenerate, near-singular KKT systems
│   │   ├── pruning_test.rs                 # Pruned == unpruned on all test polytopes
│   │   ├── regression_test.rs              # Pins for past bugs: nullspace sign, eigen gap ratio
│   │   ├── conformality_test.rs            # c(αK) = α²c(K) scaling property
│   │   ├── symplectic_invariance_test.rs     # Fixture-based: symplectomorphism invariance, monotonicity
│   │   ├── capacity_derivative_test.rs             # Finite-difference derivative validation (∂c/∂h, Euler)
│   │   ├── permutations.rs                 # Cyclic permutation generation (allocating + callback)
│   │   ├── permutations_test.rs            # Permutation count, uniqueness, cyclic equivalence
│   │   ├── orbit_recovery.rs               # Base point recovery + orbit verification (was: recover.rs)
│   │   ├── orbit_recovery_test.rs          # Recovery + verification on known orbits
│   │   └── generate_capacity_fixtures.rs                 # Fixture generation: 33 polytopes with precomputed capacities
│   ├── billiard/
│   │   ├── mod.rs                          # billiard_capacity for Lagrangian products (uses accumulator)
│   │   ├── capacity_test.rs                # Billiard capacity correctness, cross-validation with hk2017
│   │   ├── kkt_benchmark.rs                # KKT solver performance measurement (was: bench_kkt.rs)
│   │   ├── block_enumeration.rs            # Enumerate Q/P block permutations for k=2,3 bounces
│   │   └── facet_classification.rs         # Classify facets as Q-type or P-type (was: lagrangian.rs)
│   └── tube/
│       ├── mod.rs                          # Tube algorithm placeholder
│       └── capacity_test.rs                    # Placeholder
│
├── constants.rs                            # Shared tolerance constants with rationale comments
├── dataset.rs                              # JSONL dataset row types and serialization
├── dataset_test.rs                         # Serialization roundtrip
├── random.rs                               # Random polytope sampling (generic + Lagrangian)
└── random_test.rs                          # Sample validity, facet count distribution
```

### `lib.rs` re-exports

```rust
// Types
pub use geom::polytope::{ConstructionError, Polytope4D};
pub use geom::skeleton::Skeleton;
pub use geom::QhullError;

// Algorithms
pub use algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned, EhzResult};
pub use algorithms::billiard::{billiard_capacity, BilliardError, BilliardResult};

// Geometry utilities
pub use geom::volume::volume;
pub use geom::symplectic_form::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
pub use geom::known_polytopes;
pub use geom::test_utils;
```

### Orphaned/moved files

| Current file | Disposition |
|-------------|-------------|
| `kkt_rational.rs` | Rewritten as `kkt/rational_solver.rs` |
| `kkt_rational_test.rs` | Rewritten as `kkt/rational_solver_test.rs` |
| `dataset.rs`, `dataset_test.rs` | Rewritten in place (documentation pass only) |
| `random.rs`, `random_test.rs` | Rewritten in place (documentation pass only) |
| `constants.rs` | Rewritten in place (documentation pass only) |
| `algorithms/hk2017/square_product_diagnostic.rs` | Delete (diagnostic code, 927 lines — if needed, recreate as an experiment) |
| `algorithms/billiard/bench_kkt.rs` | Rewritten as `billiard/kkt_benchmark.rs`; update imports after adjacency move |
| `geom/lib_test.rs` | Delete (cross-module smoke tests folded into individual module tests) |

### `lib.rs` compilation across waves

`lib.rs` and all `mod.rs` files are pre-written in the scaffold (before wave 1). They declare all modules with empty stubs, so the crate compiles at every stage. Subagent #16 rewrites `lib.rs` with the final re-exports in wave 4.

### New modules

#### `kkt/qp_assembly.rs`

Bridges geometry → solver. Currently every caller manually extracts normals/heights from the polytope then passes them separately to the solver.

```rust
/// Assemble the QP {C, d, H} from a polytope and cyclic permutation.
///
/// C encodes closure (Σ aᵢ βᵢ = 0) + normalization (Σ βᵢ = 1).
/// H encodes symplectic action (H_ij = ω₀(aᵢ, aⱼ)).
/// Uses dual vertices directly.
///
/// [lem:kkt]
pub fn build_qp(polytope: &Polytope4D, perm: &[usize]) -> QP

/// Assemble the augmented (m+5)×(m+5) KKT system.
/// Uses normals/heights parameterization (the augmented solver's native format).
///
/// [lem:kkt]
pub fn build_augmented_system(polytope: &Polytope4D, perm: &[usize]) -> (DMatrix<f64>, DVector<f64>)
```

#### `algorithms/capacity_accumulator.rs`

Extracts the certified/uncertain candidate tracking that's currently copy-pasted identically in ehz_capacity_unpruned (~lines 88-170), ehz_capacity (~lines 270-347), and billiard_capacity (~lines 98-177). The pattern: track best candidates in two tiers, classify by beta_min vs EPS thresholds, assert gap invariant, assert positivity/finiteness.

```rust
/// Accumulates capacity candidates across an enumeration.
///
/// Two tiers:
/// - Certified: all β_k > +EPS (trustworthy)
/// - Uncertain: all β_k > -EPS (might be valid, floating-point ambiguity)
///
/// finalize() asserts the gap invariant (certified - uncertain ≤ 1e-10)
/// and returns the best certified result.
pub struct CapacityAccumulator { ... }

impl CapacityAccumulator {
    pub fn new() -> Self;
    pub fn submit(&mut self, perm: &[usize], result: &KktResult);
    pub fn finalize(self) -> Option<CapacityResult>;
}

pub struct CapacityResult {
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub best_permutation: Vec<usize>,
    pub best_beta: Vec<f64>,
    pub iterations: u64,
}
```

`EhzResult` and `BilliardResult` contain a `CapacityResult` plus algorithm-specific fields:

```rust
pub struct EhzResult {
    pub result: CapacityResult,
    pub best_subset: Vec<usize>,  // facet indices in the optimal subset S
}

pub struct BilliardResult {
    pub result: CapacityResult,
    pub bounce_count: usize,      // k value (2 or 3) of the optimal orbit
}
```

No Deref — just access `.result.capacity` etc. The accumulator returns `CapacityResult`; the algorithm function wraps it with the extra field before returning.

#### `algorithms/facet_adjacency.rs`

Moved from `hk2017/mod.rs` (lines 200-254). These functions are used by both hk2017 and billiard (billiard currently imports them cross-module).

```rust
/// Undirected facet adjacency: adj[i][j] = true iff F_i ∩ F_j ≠ ∅.
pub fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>>

/// Directed adjacency: vertex adj + ω₀(n_i, n_j) ≥ 0.
/// Uses exact omega_signs from the rational pipeline.
/// [lem:numerical-transition-feasibility]
pub fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>>

/// Check if a cyclic permutation forms an adjacent cycle.
pub fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool
```

### Interface changes

**`kkt/saddle_point_solver.rs`:**
- `try_pseudoinverse_with_threshold`: group eigendecomposition params into `EigenInfo` struct (11 params → 7)
  ```rust
  pub(crate) struct EigenInfo {
      pub eigenvalues: DVector<f64>,
      pub eigenvectors: DMatrix<f64>,
      pub n_positive: usize,
      pub n_negative: usize,
      pub n_zero: usize,
  }
  ```
- Add convenience wrapper: `pub fn solve_kkt_for(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult>` (uses qp_assembly.rs internally)
- Make `EPS_BETA_POSITIVE`, `EPS_Q_POSITIVE` `pub` (experiments currently duplicate these)

**`reeb_trajectory.rs`:**
- `simulate()` gets default params. Currently `max_segments` and `closure_tol` are always the same at all call sites.
  ```rust
  pub const DEFAULT_MAX_SEGMENTS: usize = 50;  // all current call sites use 20-50
  pub const DEFAULT_CLOSURE_TOL: f64 = 1e-6;   // all current call sites use 1e-6

  pub fn simulate(polytope: &Polytope4D, start_point: Vector4<f64>, start_facet: usize) -> ReebTrajectory
  pub fn simulate_with(polytope: &Polytope4D, start_point: Vector4<f64>, start_facet: usize, max_segments: usize, closure_tol: f64) -> ReebTrajectory
  ```

**`skeleton.rs`:**
- `facet_centroid` moves here as a method: `Skeleton::facet_centroid(&self, polytope: &Polytope4D, facet: usize) -> Vector4<f64>`. Currently a free function in reeb_trajectory.rs requiring a separate Skeleton argument.

**`hk2017/recover.rs`:**
- `recover_base_point()` + `verify_orbit()` combined into `recover_and_verify()`. These are always called together (recover_test.rs lines 23-26, 210-214).
- Return type: `Option<OrbitRecovery>` where:
  ```rust
  pub struct OrbitRecovery {
      pub breakpoints: Vec<Vector4<f64>>,   // points where trajectory hits facet boundaries
      pub dwell_times: Vec<f64>,            // time spent on each facet (should match β)
      pub max_violation: f64,               // max constraint violation across breakpoints
      pub action: f64,                      // computed action A = 1/(2Q)
      pub closure_error: f64,               // ||last breakpoint - first breakpoint||
      pub facet_sequence: Vec<usize>,       // facet indices visited in order
  }
  ```

### Test file organization

**Split 3 hk2017 test files into 7 (subagent #11):**

Source: `hk2017_test.rs` (707 lines), `capacity_properties_test.rs` (411 lines), `sensitivity_test.rs` (360 lines)

| New file | Source(s) | Concern |
|----------|-----------|---------|
| `literature_test.rs` | `hk2017_test.rs` (simplex/hypercube/product capacity tests) + `capacity_properties_test.rs` (literature_capacity_values) | Capacity values for known polytopes |
| `kkt_edge_cases_test.rs` | `hk2017_test.rs` (solve_kkt_two_facets, solve_kkt_four_facets_symplectic, solve_kkt_rank_deficient, solve_kkt_degenerate) | KKT solver edge cases |
| `pruning_test.rs` | `hk2017_test.rs` (pruned_matches_unpruned, combinations_basic) + `capacity_properties_test.rs` (pruned_matches_unpruned_from_fixture) | Pruning correctness |
| `regression_test.rs` | `hk2017_test.rs` (kkt_nullspace_* (5), eigen_gap_ratio_* (2), pentagon_capacity) | Regression pins for past bugs |
| `conformality_test.rs` | `hk2017_test.rs` (capacity_scales_quadratically) + `capacity_properties_test.rs` (capacity_conformality) | Conformality c(αK)=α²c(K) |
| `symplectic_invariance_test.rs` | `capacity_properties_test.rs` (symplectomorphism invariance, monotonicity tests) | Symplectic invariance + monotonicity |
| `capacity_derivative_test.rs` | `sensitivity_test.rs` (all 7 tests — renamed file, single-concern already) | FD derivative validation (∂c/∂h, Euler) |

**Split vertex_enumeration_test.rs (429 lines, 3 concerns) into 3 (subagent #12):**

| New file | Tests | Concern |
|----------|-------|---------|
| `vertex_enumeration_test.rs` | `exact_simplex_vertices`, `exact_hypercube_vertices`, coordinate tests, affine rank, boundedness | Exact vertex computation |
| `vertex_enumeration_linalg_test.rs` | `rank_over_q_basic`, `det4_known_values`, `solve4_exact`, `cross_product_4d_rational_perpendicular` | Rational linear algebra primitives |
| `construction_validation_test.rs` | `reject_too_few_facets`, `reject_zero_normal`, `reject_nonpositive_height`, `reject_redundant_facet`, `reject_unbounded_*`, `non_simple_polytope_accepted` | Construction validation & error paths |

**Single-concern test files (no split needed, just rename + rewrite):**
- `rational_arithmetic_test.rs` (was `rational_test.rs`) — f64↔rational agreement, pipeline consistency

**Test file location:** All test files are colocated with source (same directory), using `#[path = "foo_test.rs"]` pattern. New split test files follow the same pattern — they live next to the source file they test.

**Test file header (every test file):**
```rust
//! Tests for {module}: {proposition or concern}.
//!
//! Proposition: {mathematical statement being tested}
//! Reference: [lem:label] or [thm:label]
//!
//! Strategy: {fixture-based | proptest N cases | exhaustive for F≤6}
```

### Documentation strategy

**Progressive disclosure (reading order for agents):**
1. `lib.rs` → public API surface via re-exports
2. `{module}/mod.rs` → 5-10 line doc comment: what this module does, key types, what to read next
3. Individual file headers → mathematical correspondence, key invariants
4. Function doc comments → input/output contract, which lemma justifies correctness

**File header (every .rs file):**
```rust
//! {One-line purpose.}
//!
//! {2-5 lines: role in the crate, key interactions, invariants.}
//!
//! Mathematical correspondence: [lem:X], [def:Y]
```

**Doc comment requirements for pub items:**
- MUST: what it does (1 sentence), mathematical correspondence (thesis label), error conditions
- SHOULD: why this approach (when non-obvious), known edge cases
- MAY SKIP: obvious accessors, internal helpers with self-documenting names

---

## experiments/

### README template

All 16 experiments use this structure (sections marked * are optional by experiment type):

```markdown
# {Title}

{1-2 sentence: what question does this answer?}

## Status
{Complete | In Progress | Blocked}

## Design*
{Strategy, dataset description, parameter choices}

## Key findings*
{2-5 bullets: what was discovered}

## Files
| File | Purpose |
|------|---------|

## Run
```bash
{reproduction commands}
```

## Known limitations*
{Caveats, untested scenarios}
```

**Experiments needing the most README work** (currently sparse):
- `random-sweep`: no findings, no dataset description
- `pentagon-perturb`: no results, only speculative ideas
- `unknown-predicates`: no file listing, ambiguous status
- `gradient-descent`: no dataset section

### Code extraction to library

Stable code duplicated across 4 experiments (gradient-descent, sys-optimization, hko-neighborhood, omega-obstacle — ~1500 lines byte-identical) gets replaced by library imports:

| Duplicated code | Moves to |
|----------------|----------|
| KKT system assembly | `kkt/qp_assembly.rs` |
| Adjacency matrix builders | `algorithms/facet_adjacency.rs` |
| combinations, cyclic permutations | Move `combinations()` from `hk2017/mod.rs` to `algorithms/hk2017/permutations.rs` (subagent #6). `cyclic_permutations` and `for_each_cyclic_permutation` are already in `permutations.rs`. Reexport all three from `hk2017/mod.rs`. |
| `EPS_BETA_POSITIVE`, `EPS_Q_POSITIVE` | Made `pub` in `kkt/saddle_point_solver.rs` |

Experiment-specific code stays: custom solver variants, diagnostic instrumentation, custom accumulation logic.

---

## Meta-layer

### Fix 4 contradictions across skills

1. **Verification authority** (tex-content says "Jörn verifies after every edit" vs review says "spawn agents for math correctness"): Add to review skill: "Agents catch surface issues (undefined terms, missing steps, obvious errors). Jörn verifies mathematical correctness. Agents cannot provide Jörn-level verification."

2. **Test exhaustiveness** (rust-tests says "Jörn's domain" vs review says "check coverage"): Add to review skill: "Agents check test implementation quality (fixtures, naming, coverage of stated propositions). Agents cannot decide which propositions need testing — that's Jörn's domain."

3. **Review parallelism** (figure-review vs sequential phases): Add to review skill: "Figure-review runs within Phase 1, parallel with other Phase 1 subagents."

4. **Unreviewed-default vs mandatory-review** (tex-build vs CLAUDE.md): Add to tex-build: "Unreviewed is the WIP default. Review is mandatory before presenting to Jörn. These are compatible."

### Simplifications

- **Drop archaeology skill.** CLAUDE.md already says "Do not trust, adopt, edit, copy from, or load into context." Move the "known-broken items" list to `archaeology/README.md`.
- **Drop "plain %" comment category** from tex-format. Comments are either purposeful (prefixed: `% Jörn:`, `% QC:`, `% TODO:`) or don't need a formal category.
- **Add `thesis/lookup.sh`**: takes a label name, returns rendered theorem/section number from `main.aux`. Currently agents manually grep for this.

---

## thesis/

No structural changes. Experiment `.tex` writeups may need minor updates if experiment code/data changes during migration. Cross-references from code (`[lem:...]`) validated against thesis labels.

---

## Deviations from original spec (discovered during waves 1-2)

### Visibility convention established

`pub(super)` is too restrictive for utility functions used across modules (e.g. `omega0_rational` needed by both `geom/` and `kkt/`). Convention: **utility functions used cross-module → `pub(crate)`**. Only use `pub(super)` for helpers truly private to one module subtree.

### Additional rename: `reeb_vector` → `reeb_direction`

Not in the original spec. The function returns the direction J₀n_i, not the full Reeb vector field R_i = (2/h_i)J₀n_i. The rename is more accurate. Experiments updated.

### `rational_solver` behavioral change

Old `kkt_rational.rs` returned `Some(result)` with non-positive beta (callers filtered). New `kkt/rational_solver.rs` returns `None` if any beta ≤ 0. This makes the API contract cleaner but wave 3+ callers must handle `None` instead of post-filtering.

### `constants.rs` has only one constant

The original spec said "shared tolerance constants" (plural). Only `EPS_FACET_INCIDENCE` ended up here. Other tolerances (`EPS_BETA_POSITIVE`, `EPS_Q_POSITIVE`, `EPS_MARGIN_TRUE/FALSE`) live in their respective solver modules. This is fine — constants live near their users.

### `CapacityAccumulator::submit` takes `&Solution` not `&KktResult`

The spec showed `submit(&mut self, perm: &[usize], result: &KktResult)`. The implementation uses `&Solution` (the new type from `kkt/mod.rs`). `KktResult` doesn't exist in the new codebase — it was the old type name. Wave 3 subagents (#6, #7) should use `Solution`.
