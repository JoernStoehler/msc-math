<!--
Purpose: navigation cache for durable Rust crates.
Context: this map helps agents find crate subsystems, public API tiers, and
code/math boundaries. It is descriptive, not a refactor backlog.

Map maintenance:
- Source truth is crate source, crate READMEs, manifests, formal labels cited
  from code, and tests; this file is a cache over those sources.
- To check staleness, compare affected sections against those source files with
  targeted `rg`, `rg --files`, manifests, and local module headers.
- To refresh, update only navigation facts and route API/refactor decisions to
  the owning implementation files or owner-local notes.
- Keep entries short; point to source files instead of duplicating details.
-->

# Crates Map

## Status

- State: split from the old root `ARCHITECTURE.md`.
- Last updated: 2026-06-13.
- Source surfaces: `crates/**/src/`, `crates/**/README.md`,
  `crates/**/DEVELOPMENT.md`, `formal/`, crate manifests, and local crate
  tests.
- Refresh when: crate module boundaries, public reexports, persistence
  contracts, or exact/floating representation boundaries change.

## Map Type And Authority

- Type: subtree navigation cache.
- Agent question: which durable crate, subsystem, entity, or API tier should I
  inspect first?
- Authority: crate source files, crate README files, formal labels referenced
  from code, and tests overrule this map.
- Non-authority: this file does not create API stability promises or refactor
  obligations. Route those through the relevant implementation task.

## Crate Roles

| Crate | Current role | Start here |
| --- | --- | --- |
| `crates/symplectic/` | main symplectic geometry crate for 4D convex polytopes, capacity algorithms, KKT solvers, persistence helpers, derivatives, sampling, and known polytope constructors | `crates/symplectic/src/lib.rs`, `crates/symplectic/README.md` |
| `crates/algebraic-numbers/` | exact scalar arithmetic over `Q` and statically chosen real algebraic extensions, plus dense generic exact linear algebra | `crates/algebraic-numbers/src/lib.rs`, `crates/algebraic-numbers/README.md` |
| `crates/euclidean-polytopes/` | ordinary convex-polytope geometry in ambient `R^4`; currently owns exact origin-interior, exact extreme-point, exact polar vertex enumeration, incidence-derived face helpers, known-incidence f64/exact volume, known-incidence facet 3-volume, random dual-vertex candidate sampling, and incidence-only 2-face ordering inside volume decomposition | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |

For cross-experiment algorithm and evidence routing, read
`experiments/MAP.md` section `Algorithm Units`. This file stays focused on
durable crate API, module ownership, and representation boundaries; the
experiment map tracks development packets, numerics/performance/verification
homes, and thesis-support evidence routing.

## Symplectic Subsystems

| Subsystem | Current role | Notes |
| --- | --- | --- |
| `geom` | geometry helper layer | owns flat dual-vertex validation, exact/rational geometry utilities, symplectic form helpers, known flat fixtures, and related geometry routines |
| `kkt` | context-free constrained-QP solver layer | operates on abstract matrices `(C, d, H)`; `qp_assembly` crosses from polytope geometry into solver inputs |
| `algorithms` | symplectic/capacity algorithm layer | owns HK2017, billiard, flow-graph, shared capacity-accumulator logic, and related pruning/combinatorics |
| `database` / `dataset` | persistence/schema support layer | owns JSONL storage helpers and row schemas; callers choose paths |
| `derivatives` | differential support layer | analytical derivatives with respect to dual vertices `a_i` |
| `random` | sampling/generation support layer | seeded random polytope generation for experiments |
| `exact` | exact-validation support layer | exact polytope/orbit/derivative helpers used by theorem-facing validation paths |
| `constants` | shared numeric tolerance layer | cross-module tolerances; check users before changing values |

`algorithms::flow_graph` is a development work surface for the flow-graph
capacity algorithm. Start at
`crates/symplectic/src/algorithms/flow_graph/README.md`; current f64 routines
are development evidence, not exact `c_EHZ` certificates.

## Algebraic-Numbers Subsystems

| Subsystem | Current role | Notes |
| --- | --- | --- |
| `algebraic_element` / `field_specification` / `exact_scalar` | core scalar API | `Algebraic<F>`, `RealAlgebraicField`, `ExactScalar`, and canonical coefficient access |
| `row_reduction` / `linear_solve` / `definiteness` | dense exact linear algebra | `row_reduction`, `rank`, `kernel_basis`, `solve_linear_system`, `LinearSystemSolution`, and `is_negative_definite` |
| `polynomial_arithmetic` / `sign_ordering` | internal algebraic arithmetic and exact ordering | polynomial reduction, inversion modulo the field polynomial, and rational interval refinement |

## Navigation Shortcuts

| If you need... | Start here |
| --- | --- |
| geometry of one polytope | `crates/symplectic/src/geom/` |
| exact rational vertex enumeration | `crates/symplectic/src/geom/vertex_enumeration/` |
| one orbit candidate / KKT solve | `crates/symplectic/src/kkt/` |
| capacity computation | `crates/symplectic/src/algorithms/` |
| recovered primal orbit / trajectory | `crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs` |
| skeletons / face adjacency | `crates/euclidean-polytopes/src/faces.rs`, `crates/symplectic/src/algorithms/facet_adjacency.rs` |
| derivatives with respect to dual vertices | `crates/symplectic/src/derivatives.rs` |
| JSONL polytope records and stored rows | `crates/symplectic/src/database.rs`, `crates/symplectic/src/dataset.rs` |
| exact one-sigma validation kernels | `crates/symplectic/src/exact/` |
| exact algebraic scalar behavior | `crates/algebraic-numbers/src/` |

## Core Entities

| Entity | Current role | Main surface |
| --- | --- | --- |
| flat known-polytope fields | reusable fixture geometry as `dual_vertices`, `vertices`, `vertex_facet_incidence`, `facet_intersection_is_nonempty`, `omega_signs`, and f64 copies | `crates/symplectic/src/geom/known_polytopes.rs` |
| face helpers | Euclidean face data derived from vertex-facet incidence | `crates/euclidean-polytopes/src/faces.rs` |
| `OrbitSearchResult` | shared capacity/orbit search result returned by explicit capacity aggregation paths; contains orbit list plus `min_action` bounds and iterations | `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/lib.rs` |
| `OrbitKktData` | one solved orbit payload: `sigma`, `beta`, action interval, `q`, optional multipliers, admissibility | `crates/symplectic/src/algorithms/orbit_search.rs` |
| `GeometricOrbit` | recovered geometric trajectory/orbit data derived from an `OrbitKktData` payload | `crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs` |
| `PolytopeRecord` | persisted JSONL row with rational geometry and optional computed summaries | `crates/symplectic/src/database.rs` |
| `ExactVerticesWithIncidence` / `ExactOrbitKktData` | exact ordered-field payloads for polar vertices, incidence, and one-sigma validation | `crates/symplectic/src/exact/` |
| `Algebraic<S>` | scalar element over a compile-time real algebraic field specification | `crates/algebraic-numbers/src/lib.rs` |

## API Tiers

| Tier | Current meaning | Examples |
| --- | --- | --- |
| simple public | short root reexports and ordinary geometry helpers in `crates/symplectic/src/lib.rs` | `symplectic::OrbitSearchResult`, `symplectic::known_polytopes`, `symplectic::omega0`, `symplectic::lagrangian_product`, `symplectic::regular_polygon_2d`, `symplectic::rotate_polygon_2d`, `symplectic::classify_facets_from_dual_vertices`, `symplectic::solve_pruned_hk2017_candidates`, `symplectic::solve_unpruned_hk2017_candidates`, `symplectic::solve_billiard_candidates` |
| expert public | deeper modules and building blocks used by experiments that need non-default control | `symplectic::database`, `symplectic::dataset`, `symplectic::random`, `symplectic::derivatives`, `symplectic::exact`, `symplectic::algorithms::aggregate_orbits_with_dual_vertices_exact`, `symplectic::algorithms::hk2017`, `symplectic::algorithms::billiard`, `symplectic::kkt::saddle_point_solver`, `symplectic::algorithms::facet_adjacency` |
| exact scalar public | root reexports in `crates/algebraic-numbers/src/lib.rs` | `algebraic_numbers::Algebraic`, `algebraic_numbers::RealAlgebraicField`, `algebraic_numbers::ExactScalar`, `algebraic_numbers::row_reduction`, `algebraic_numbers::rank`, `algebraic_numbers::kernel_basis`, `algebraic_numbers::solve_linear_system`, `algebraic_numbers::LinearSystemSolution`, `algebraic_numbers::is_negative_definite` |
| Euclidean polytopes target | flat functions over `Vector4<T>`; implemented slices include `origin_in_interior_of_conv_exact`, `all_points_are_extreme_exact`, `polar_vertices_exact`, incidence-derived face helpers, `volume_from_incidence_exact`, `volume_from_incidence_f64`, known-incidence facet-volume helpers, random dual-vertex candidate sampling, and incidence-only 2-face ordering for volume decomposition | `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| unclear | public paths whose long-term experiment-facing status is not explicit | `symplectic::algorithms::hk2017::orbit_recovery`, `symplectic::algorithms::billiard::facet_classification`, `symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices` |
| accidental internal | public-in-practice helpers that experiments currently reach through | `symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation` |

The practical experiment-facing library surface is larger than the short root
reexport surface. Treat API promotion/cleanup as future work unless a retained
thesis claim, verification task, or writing blocker needs it.

## Representation Boundary

- Defining polytope geometry is stored and keyed in rational form in
  `PolytopeRecord`.
- Many algorithmic computations run in `f64`, including normal-path capacity
  algorithms, orbit recovery, derivatives, and persisted volume fields.
  Ordinary Euclidean volume computation is owned by `euclidean-polytopes`.
- `kkt::rational_solver` and `exact` modules are exact-validation tracks, not
  the main capacity hot path.
- Persisted records cross this boundary: rational geometry plus optional
  floating computed fields such as `volume`, `capacity`, and `sigmas`.

## Open Edges

- Which deep public paths should remain supported experiment-facing imports?
- Which experiment-local caches should remain local now that shared polytope
  containers are gone, and which repeated helper should move into a durable
  crate function?
- Which repeated helpers belong in topic helper crates versus
  `crates/symplectic/`?
- Which reusable stored fields should downstream consumers be allowed to trust
  as a stable cache contract?
- Should experiment code standardize more strongly on root reexports, or is the
  deep expert-public form acceptable during the thesis push?
