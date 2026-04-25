<!--
Purpose: navigation cache for durable Rust crates.
Context: this map helps agents find crate subsystems, public API tiers, and
code/math boundaries. It is descriptive, not a refactor backlog.
-->

# Crates Map

## Status

- State: split from the old root `ARCHITECTURE.md`.
- Last updated: 2026-04-25.
- Source surfaces: `crates/**/src/`, `crates/**/README.md`, `formal/library/`,
  `contracts/`, and local crate tests.
- Refresh when: crate module boundaries, public reexports, persistence
  contracts, or exact/floating representation boundaries change.

## Crate Roles

| Crate | Current role | Start here |
| --- | --- | --- |
| `crates/symplectic/` | main symplectic geometry, capacity algorithms, KKT solvers, persistence helpers, derivatives, and known polytope constructors | `crates/symplectic/src/lib.rs`, `crates/symplectic/README.md` |
| `crates/algebraic-numbers/` | exact ordered algebraic scalar arithmetic used by exact-validation paths | `crates/algebraic-numbers/src/lib.rs`, `crates/algebraic-numbers/README.md` |

## Symplectic Subsystems

| Subsystem | Current role | Notes |
| --- | --- | --- |
| `geom` | single-polytope geometry layer | owns `Polytope4D`, exact/rational geometry utilities, symplectic form helpers, volume/facet helpers, constructors, and related geometry routines |
| `kkt` | context-free constrained-QP solver layer | operates on abstract matrices `(C, d, H)`; `qp_assembly` crosses from polytope geometry into solver inputs |
| `algorithms` | symplectic/capacity algorithm layer | owns HK2017, billiard, shared capacity-accumulator logic, and related pruning/combinatorics |
| `database` / `dataset` | persistence/schema support layer | owns JSONL storage helpers and row schemas; callers choose paths |
| `derivatives` | differential support layer | analytical derivatives with respect to dual vertices `a_i` |
| `random` | sampling/generation support layer | seeded random polytope generation for experiments |
| `exact` | exact-validation support layer | exact polytope/orbit/derivative helpers used by theorem-facing validation paths |

## Navigation Shortcuts

| If you need... | Start here |
| --- | --- |
| geometry of one polytope | `crates/symplectic/src/geom/` |
| one orbit candidate / KKT solve | `crates/symplectic/src/kkt/` |
| capacity computation | `crates/symplectic/src/algorithms/` |
| recovered primal orbit / trajectory | `crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs` |
| derivatives with respect to dual vertices | `crates/symplectic/src/derivatives.rs` |
| JSONL polytope records and stored rows | `crates/symplectic/src/database.rs`, `crates/symplectic/src/dataset.rs` |
| exact algebraic scalar behavior | `crates/algebraic-numbers/src/` |

## Core Entities

| Entity | Current role | Main surface |
| --- | --- | --- |
| `Polytope4D` | central polytope object for geometry and algorithms | `crates/symplectic/src/lib.rs`, `crates/symplectic/src/geom/` |
| `OrbitSearchResult` | shared capacity/orbit search result returned by the root capacity family; contains orbit list plus `min_action` bounds and iterations | `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/lib.rs` |
| `OrbitKktData` | one solved orbit payload: `sigma`, `beta`, action interval, `q`, optional multipliers, admissibility | `crates/symplectic/src/algorithms/orbit_search.rs` |
| `GeometricOrbit` | recovered geometric trajectory/orbit data derived from an `OrbitKktData` payload | `crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs` |
| `PolytopeRecord` | persisted JSONL row with rational geometry and optional computed summaries | `crates/symplectic/src/database.rs` |

## API Tiers

| Tier | Current meaning | Examples |
| --- | --- | --- |
| simple public | short root reexports and preset routers in `crates/symplectic/src/lib.rs` | `symplectic::ehz_capacity`, `symplectic::ehz_capacity_pruned`, `symplectic::ehz_capacity_unpruned`, `symplectic::ehz_capacity_billiard`, `symplectic::OrbitSearchResult`, `symplectic::volume`, `symplectic::omega0`, `symplectic::lagrangian_product` |
| expert public | deeper modules and building blocks used by experiments that need non-default control | `symplectic::database`, `symplectic::random`, `symplectic::derivatives`, `symplectic::algorithms::solve_orbit_sigma`, `symplectic::algorithms::aggregate_orbits`, `symplectic::algorithms::hk2017`, `symplectic::algorithms::billiard`, `symplectic::kkt::saddle_point_solver`, `symplectic::algorithms::facet_adjacency` |
| unclear | public paths whose long-term experiment-facing status is not explicit | `symplectic::algorithms::hk2017::orbit_recovery`, `symplectic::algorithms::billiard::facet_classification`, `symplectic::kkt::qp_assembly::build_augmented_system` |
| accidental internal | public-in-practice helpers that experiments currently reach through | `symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation` |

The practical experiment-facing library surface is larger than the short root
reexport surface. Treat API promotion/cleanup as future work unless a retained
thesis claim, verification task, or writing blocker needs it.

## Representation Boundary

- Defining polytope geometry is stored and keyed in rational form in
  `PolytopeRecord`.
- Many algorithmic computations run in `f64`, including normal-path capacity
  algorithms, orbit recovery, derivatives, and qhull-backed volume.
- `kkt::rational_solver` and `exact` modules are exact-validation tracks, not
  the main capacity hot path.
- Persisted records cross this boundary: rational geometry plus optional
  floating computed fields such as `volume`, `capacity`, and `sigmas`.

## Open Edges

- Which deep public paths should remain supported experiment-facing imports?
- Which repeated helpers belong in topic helper crates versus
  `crates/symplectic/`?
- Which reusable stored fields should downstream consumers be allowed to trust
  as a stable cache contract?
- Should experiment code standardize more strongly on root reexports, or is the
  deep expert-public form acceptable during the thesis push?
