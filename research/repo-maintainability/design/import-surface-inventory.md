<!--
Purpose: durable import-surface inventory for the repo maintainability /
architecture program.
Context: discovery packet D1. This note records the current experiment-facing
library import paths, the evidence used to classify them, and the remaining
boundary questions that still need a Jörn decision.
-->

# Import Surface Inventory

## Status

- Status: discovery complete, boundary decisions pending.
- Date: 2026-04-16.

## Method / Evidence

- Commands checked:
  - `cd /workspaces/msc-math && pwd`
  - `rg -n "use symplectic::" experiments -g '*.rs'`
  - `rg -n "symplectic::" experiments -g '*.rs'`
  - `rg -n "use symplectic::constants::|symplectic::constants::" experiments -g '*.rs'`
  - `sed -n '1,220p' library/src/lib.rs`
  - `sed -n '1,220p' library/src/geom/mod.rs`
  - `sed -n '1,220p' library/src/kkt/mod.rs`
  - `sed -n '1,220p' library/src/algorithms/mod.rs`
  - `sed -n '1,220p' library/src/algorithms/hk2017/mod.rs`
  - `sed -n '1,220p' library/src/algorithms/billiard/mod.rs`
  - `sed -n '1,240p' library/src/database.rs`
  - `sed -n '1,240p' library/src/dataset.rs`
  - `sed -n '1,240p' library/src/derivatives.rs`
  - `sed -n '1,220p' library/src/random.rs`
  - `sed -n '1,200p' library/src/constants.rs`
  - `sed -n '1,220p' library/src/algorithms/hk2017/orbit_recovery.rs`
  - `sed -n '1,220p' library/src/algorithms/facet_adjacency.rs`
  - `sed -n '1,220p' library/src/geom/test_utils.rs`
  - `sed -n '1,220p' library/src/geom/known_polytopes.rs`
  - `sed -n '1,260p' library/src/kkt/saddle_point_solver.rs`
  - `sed -n '1,220p' library/src/kkt/projection_solver.rs`
  - `sed -n '1,220p' library/src/kkt/beta_feasibility.rs`
  - `sed -n '1,220p' library/src/kkt/constraint_solver.rs`
  - `sed -n '1,260p' research/repo-maintainability/design/main.md`
- Files checked for callers:
  - `experiments/**/**/*.rs`
  - `library/src/lib.rs`
  - `library/src/**/mod.rs`

## Inventory

### Simple public

- `symplectic::ehz_capacity`, `symplectic::ehz_capacity_pruned`, `symplectic::ehz_capacity_unpruned`, `symplectic::ehz_capacity_billiard`, `symplectic::OrbitSearchResult`, `symplectic::volume`, `symplectic::omega0`, `symplectic::lagrangian_product`, `symplectic::regular_polygon_2d`, `symplectic::rotate_polygon_2d`, `symplectic::known_polytopes`, `symplectic::test_utils`, `symplectic::Polytope4D`, `symplectic::ConstructionError`, `symplectic::Skeleton`, `symplectic::QhullError`.
- Why: these are the root reexports in `library/src/lib.rs` and read like the intended short path for routine experiment code.
- Examples of current callers:
  - `experiments/combinatorial-cells/omega-hypothesis/main.rs` uses `symplectic::ehz_capacity`.
  - `experiments/hko-local-maximum/cut-and-ascent/main.rs` and `experiments/sys-landscape/random-sample/main.rs` use the root auto wrapper.
  - `experiments/verification/correctness/main.rs` and `experiments/sys-landscape/random-product-sample/main.rs` use the root-style geometry helpers via the deep module path instead of the root reexport.

### Expert public

- `symplectic::geom::polytope::Polytope4D`, `symplectic::geom::known_polytopes`, `symplectic::geom::polygon::{random_polygon_2d, regular_polygon_2d, rotate_polygon_2d}`, `symplectic::geom::lagrangian_product::lagrangian_product`, `symplectic::geom::volume::volume`, `symplectic::geom::symplectic_form::omega0`.
- `symplectic::geom::facet_volume::facet_volume_3d`, `symplectic::geom::reeb_trajectory`.
- `symplectic::algorithms::hk2017::{combinations, for_each_sigma_pruned, for_each_sigma_unpruned}` and `symplectic::algorithms::billiard::{for_each_sigma, bounce_count_from_sigma}`.
- `symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle}`.
- `symplectic::database::{load, load_many, save, DualVerticesKey, PolytopeRecord, SigmaAction, Source}`, `symplectic::dataset::AcceptanceRow`, `symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a}`, `symplectic::random::{sample_random_polytope, generate_polytope, generate_random_polytopes}`.
- `symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, KktResult, EPS_BETA_POSITIVE, EPS_Q_POSITIVE}`.
- `symplectic::kkt::rational_solver`.
- Why: these paths are public, documented, and used by the current experiments as normal working APIs, but they are deeper than the short root surface.
- Examples of current callers:
  - `experiments/combinatorial-cells/boundary-characterization/main.rs`, `experiments/combinatorial-cells/cell-widths/main.rs`, and `experiments/hko-local-maximum/gradient-analysis/main.rs` import `build_transition_matrix`, `is_feasible_cycle`, `combinations`, and `for_each_cyclic_permutation`.
  - `experiments/verification/orbit-recovery/main.rs` imports `recover_and_verify`, `GeometricOrbit`, and `solve_kkt_for`.
  - `experiments/sys-landscape/gradient-ascent-general/main.rs`, `experiments/sys-landscape/random-sample/main.rs`, and `experiments/sys-landscape/rejection-calibration/main.rs` import `database`, `random`, and `dataset` helpers.
  - `experiments/numerics/gradient/numerics/main.rs` and `experiments/hko-local-maximum/second-order/main.rs` import `solve_kkt_for`, `KktResult`, and `EPS_Q_POSITIVE`.
  - `experiments/numerics/gradient/numerics-edge-cases/main.rs` imports `facet_volume_3d`.
  - `experiments/visualization/main/main.rs` imports `geom::reeb_trajectory`.
  - `experiments/numerics/q-error/main.rs` imports `kkt::rational_solver as kkt_rational`.

### Accidental internal

- `symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation`.
- Why: `hk2017/mod.rs` does not advertise `permutations` as part of the module's public architecture; it is a helper module that current experiment code reaches through because it happens to be `pub`.
- Examples of current callers:
  - `experiments/hko-local-maximum/gradient-analysis/main.rs`
  - `experiments/combinatorial-cells/boundary-characterization/main.rs`
  - `experiments/numerics/error-bounds/collect_poly.rs`

### Unclear

- `symplectic::algorithms::hk2017::orbit_recovery::{recover_and_verify, GeometricOrbit}`.
- `symplectic::algorithms::hk2017::permutations` as a module boundary, not just the function above.
- `symplectic::algorithms::billiard::facet_classification::{classify_facets, FacetClassification}`.
- `symplectic::kkt::qp_assembly::build_augmented_system`.
- Why: the code is public and used, but the router module does not list these as part of the canonical API surface. The current tree does not settle whether they should remain supported experiment-facing dependencies or move behind a narrower helper layer.
- Example callers:
  - `experiments/verification/orbit-recovery/main.rs`
  - `experiments/visualization/main/main.rs`
  - `experiments/sys-landscape/gradient-ascent-products/main.rs`
  - `experiments/numerics/gradient/numerics-subdifferential/main.rs`

## Unresolved Questions

- Should `hk2017::orbit_recovery` stay a supported public dependency, or become experiment-local code that only the root `ehz_capacity*` family and `OrbitSearchResult` consume?
- Should `hk2017::permutations::for_each_cyclic_permutation` be treated as a stable expert surface, or as an accidental helper that later code should not import directly?
- Should `billiard::facet_classification` remain reachable from experiments, or move behind a topic-local helper once the gradient-ascent-products dependency is revisited?
- Should `kkt::qp_assembly::build_augmented_system` remain an experiment-facing low-level tool for numerics, or be treated as internal assembly code?
- Should the experiment code standardize on the root reexports in `library/src/lib.rs`, or is the deep-module form acceptable for the thesis phase?
- Should `database`, `dataset`, `random`, and `derivatives` remain expert public library APIs, or be moved to topic-local helper crates when the program turns from discovery to refactor?

## Next Safe Resume Point

- Re-run the caller scan, then ask Jörn to decide only the boundary-sensitive rows:
  - `hk2017::orbit_recovery`
  - `hk2017::permutations`
  - `billiard::facet_classification`
  - `kkt::qp_assembly`
  - whether the deep experiment imports should be normalized to root reexports or left as-is for the thesis push.
