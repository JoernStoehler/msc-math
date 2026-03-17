# Migration Done Checklist

Derived from migration-target.md. Every item is verifiable. Check each one.

## 1. Module tree (target lines 12-99)

- [ ] Every file in the tree exists at the target path
- [ ] Every file is non-empty (no stubs)
- [ ] No extra .rs files that aren't in the tree (orphaned old files deleted)
- [ ] mod.rs files have correct `pub mod` + `#[cfg(test)] #[path]` declarations matching the tree

## 2. lib.rs re-exports (target lines 101-120)

- [ ] Crate doc comment (5 lines)
- [ ] `pub use geom::polytope::{ConstructionError, Polytope4D}`
- [ ] `pub use geom::skeleton::Skeleton`
- [ ] `pub use geom::QhullError`
- [ ] `pub use algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned, EhzResult}`
- [ ] `pub use algorithms::billiard::{billiard_capacity, BilliardError, BilliardResult}`
- [ ] `pub use geom::volume::volume`
- [ ] `pub use geom::symplectic_form::omega0`
- [ ] `pub use geom::lagrangian_product::lagrangian_product`
- [ ] `pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d}`
- [ ] `pub use geom::known_polytopes`
- [ ] `pub use geom::test_utils`

## 3. New modules (target lines 139-223)

### qp_assembly.rs
- [ ] `build_qp(polytope, perm) -> QP` exists
- [ ] `build_augmented_system(polytope, perm) -> (DMatrix, DVector)` exists

### capacity_accumulator.rs
- [ ] `CapacityAccumulator` with `new()`, `submit()`, `finalize()` exists
- [ ] `CapacityResult` with fields: capacity, capacity_uncertain, best_permutation, best_beta, iterations
- [ ] `EhzResult { result: CapacityResult, best_subset: Vec<usize> }` (in hk2017/mod.rs)
- [ ] `BilliardResult { result: CapacityResult, bounce_count: usize }` (in billiard/mod.rs)

### facet_adjacency.rs
- [ ] `build_adjacency_matrix(polytope) -> Vec<Vec<bool>>`
- [ ] `build_directed_adjacency_matrix(polytope) -> Vec<Vec<bool>>`
- [ ] `is_adjacent_cycle(perm, adj) -> bool`

## 4. Interface changes (target lines 225-266)

### saddle_point_solver
- [ ] `EigenInfo` struct with eigenvalues, eigenvectors, n_positive, n_negative, n_zero
- [ ] `solve_kkt_for(polytope, perm) -> Option<KktResult>` convenience wrapper
- [ ] `EPS_BETA_POSITIVE` and `EPS_Q_POSITIVE` are `pub`

### reeb_trajectory
- [ ] `DEFAULT_MAX_SEGMENTS = 50`
- [ ] `DEFAULT_CLOSURE_TOL = 1e-6`
- [ ] `simulate(polytope, start_point, start_facet)` (uses defaults)
- [ ] `simulate_with(polytope, start_point, start_facet, max_segments, closure_tol)`

### skeleton
- [ ] `facet_centroid` is a method on Skeleton (not a free function in reeb_trajectory)

### orbit_recovery
- [ ] `recover_and_verify()` exists (combines old recover_base_point + verify_orbit)
- [ ] `OrbitRecovery` struct with: breakpoints, dwell_times, max_violation, action, closure_error, facet_sequence

## 5. Test organization (target lines 268-305)

### hk2017 test split (7 files from 3)
- [ ] literature_test.rs — capacity values for known polytopes
- [ ] kkt_edge_cases_test.rs — rank-deficient, degenerate
- [ ] pruning_test.rs — pruned == unpruned
- [ ] regression_test.rs — past bug pins
- [ ] conformality_test.rs — c(αK) = α²c(K)
- [ ] symplectic_invariance_test.rs — symplectomorphism invariance
- [ ] capacity_derivative_test.rs — FD derivatives

### vertex_enumeration test split (3 files from 1)
- [ ] vertex_enumeration_test.rs — pipeline tests
- [ ] vertex_enumeration_linalg_test.rs — rational linear algebra
- [ ] construction_validation_test.rs — error paths

### Test file conventions
- [ ] Every test file has //! header with proposition + strategy
- [ ] Every #[test] function has /// doc comment
- [ ] Every #[ignore] has a comment explaining why and how to run

## 6. Documentation (target lines 307-327)

- [ ] Every .rs file has //! header (purpose + mathematical correspondence)
- [ ] Every pub item has /// doc comment
- [ ] Thesis cross-refs use [lem:label] format, never "Lemma 3.2"
- [ ] Magic numbers have rationale comments

## 7. Automated gates (process lines 130-143)

- [ ] `cargo test --lib` passes (0 failures)
- [ ] `cargo clippy --lib -- -D warnings` passes (0 warnings)
- [ ] `cargo clippy --tests -- -D warnings` passes (0 warnings)
- [ ] `cargo build` in experiments/ (all binaries compile or fail only on expected missing wave items)
- [ ] `ruff check experiments/` passes
- [ ] `cd thesis/ && latexmk` passes

## 8. Experiments (target lines 331-381)

- [ ] #13: Experiment READMEs standardized per template (random-sweep, pentagon-perturb, unknown-predicates, gradient-descent)
- [ ] #14: All experiment binaries use new module paths
- [ ] Code extraction: 4 experiments replaced duplicated code with library imports
- [ ] EhzResult/BilliardResult field access uses nested pattern (.result.capacity)

## 9. Meta-layer (target lines 385-401)

- [ ] Fix contradiction 1: verification authority (tex-content vs review)
- [ ] Fix contradiction 2: test exhaustiveness (rust-tests vs review)
- [ ] Fix contradiction 3: review parallelism (figure-review vs sequential)
- [ ] Fix contradiction 4: unreviewed-default vs mandatory-review
- [ ] Drop archaeology skill → move known-broken items to archaeology/README.md
- [ ] Drop "plain %" comment category from tex-format
- [ ] Add thesis/lookup.sh

## 10. Cleanup

- [ ] No orphaned old files on disk (kkt_rational.rs, augmented.rs, etc. — 21 files listed in process #17)
- [ ] No stale TODO comments referencing agent numbers or wave dependencies
- [ ] No empty test bodies with TODO placeholders
- [ ] No local function copies that should be library imports
