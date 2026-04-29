<!--
Purpose: historical Packet 2 planning report for the shared capacity/orbit
search frontend.
Context: maps an April 2026 seam across hk2017 pruned, hk2017 unpruned, and
billiard. Verify against current code before reuse.
-->

# Packet 2 Shared Search Frontend Seam Report

> Historical snapshot. Do not treat this note as current instruction or live
> architecture state; verify facts against current maps, tasks, and code before
> reuse.

## Recommended seam

Extract the shared collector **after frontend-specific sigma generation/pruning,
but before frontend-specific certified-winner metadata bookkeeping**.

In current code, that seam is the point where each frontend already has a
concrete `sigma: &[usize]` and then does the same work:

1. solve `(polytope, sigma)` through the saddle-point bridge,
2. convert `KktResult` into `OrbitKktData`,
3. aggregate the solved orbit(s) with [`aggregate_orbits`](../../../crates/symplectic/src/algorithms/orbit_search.rs:545) or [`aggregate_orbits_f64_only`](../../../crates/symplectic/src/algorithms/orbit_search.rs:596),
4. finalize to an [`OrbitSearchResult`](../../../crates/symplectic/src/algorithms/orbit_search.rs:95).

This is the smallest extraction that lets Packet 2 add shared collector
entrypoints in [`orbit_search.rs`](../../../crates/symplectic/src/algorithms/orbit_search.rs:1)
without changing solver math or forcing the three candidate generators into one
abstraction.

## Files and functions in the current loop

- HK2017 public wrappers on the current surface:
  [ehz_capacity_pruned](../../../crates/symplectic/src/lib.rs:80),
  [ehz_capacity_unpruned](../../../crates/symplectic/src/lib.rs:90),
  [ehz_capacity](../../../crates/symplectic/src/lib.rs:121)
  route through the shared orbit-search result layer.
- HK2017 enumeration:
  [enumerate_unpruned](../../../crates/symplectic/src/algorithms/hk2017/enumeration.rs:18),
  [enumerate_pruned](../../../crates/symplectic/src/algorithms/hk2017/enumeration.rs:22),
  [enumerate_impl](../../../crates/symplectic/src/algorithms/hk2017/enumeration.rs:26).
- HK2017 candidate generators used inside the loop:
  `combinations` from `hk2017/combinatorics.rs`,
  [for_each_cyclic_permutation](../../../crates/symplectic/src/algorithms/hk2017/permutations.rs:35),
  and optionally
  [build_transition_matrix](../../../crates/symplectic/src/algorithms/facet_adjacency.rs:30) +
  [is_feasible_cycle](../../../crates/symplectic/src/algorithms/facet_adjacency.rs:44).
- Shared one-sigma solve seam now used by HK2017:
  [solve_orbit_sigma](../../../crates/symplectic/src/algorithms/orbit_search.rs:192)
  called from the HK2017 traversal helpers and wired into the shared result
  surface.
- Billiard public frontend:
  [ehz_capacity_billiard](../../../crates/symplectic/src/lib.rs:102).
- Billiard candidate preparation/generation:
  `classify_facets` from `billiard/facet_classification.rs`,
  [enumerate_blocks](../../../crates/symplectic/src/algorithms/billiard/block_enumeration.rs:68),
  [enumerate_k_bounce_sigmas](../../../crates/symplectic/src/algorithms/billiard/block_enumeration.rs:102),
  and the same
  [build_transition_matrix](../../../crates/symplectic/src/algorithms/facet_adjacency.rs:30) +
  [is_feasible_cycle](../../../crates/symplectic/src/algorithms/facet_adjacency.rs:44).
- Shared one-sigma solve seam now used by billiard:
  [solve_orbit_sigma](../../../crates/symplectic/src/algorithms/orbit_search.rs:192)
  called from the billiard traversal helpers and wired into the shared result
  surface.
- Shared tracking/finalization:
  [aggregate_orbits](../../../crates/symplectic/src/algorithms/orbit_search.rs:545),
  [aggregate_orbits_f64_only](../../../crates/symplectic/src/algorithms/orbit_search.rs:596),
  [OrbitSearchResult](../../../crates/symplectic/src/algorithms/orbit_search.rs:95).
- Shared lower solver contract:
  [solve_kkt_for](../../../crates/symplectic/src/kkt/saddle_point_solver.rs:292),
  [KktOutcome::feasible](../../../crates/symplectic/src/kkt/saddle_point_solver.rs:167),
  [KktResult](../../../crates/symplectic/src/kkt/saddle_point_solver.rs:198).
- Shared Packet 1 target surface for Packet 2 wiring:
  [OrbitKktData / OrbitSearchResult / enums](../../../crates/symplectic/src/algorithms/orbit_search.rs:15).

## Stage map and data passed

| Stage | HK2017 pruned/unpruned | Billiard | Data passed onward |
| --- | --- | --- | --- |
| Candidate generation | `m`, `subset`, cyclic `perm` from [enumerate_impl](../../../crates/symplectic/src/algorithms/hk2017/enumeration.rs:26) | `k`, block selections, `sigma` from [enumerate_k_bounce_sigmas](../../../crates/symplectic/src/algorithms/billiard/block_enumeration.rs:102) | Concrete `sigma: &[usize]` plus frontend-local metadata (`subset` or `k`) |
| Cheap pruning | optional directed adjacency via [is_feasible_cycle](../../../crates/symplectic/src/algorithms/facet_adjacency.rs:44) | same directed adjacency check in [ehz_capacity_billiard](../../../crates/symplectic/src/lib.rs:102) | surviving `sigma: &[usize]` |
| Solve | [solve_orbit_sigma](../../../crates/symplectic/src/algorithms/orbit_search.rs:192) | same helper path | `OrbitKktData` or `OrbitSolveError` |
| Classify/convert | `OrbitAdmissibility` is attached to each solved orbit; no separate result wrapper is involved | same helper path | `OrbitKktData { admissibility, q, beta, ... }` |
| Track shared capacity state | `aggregate_orbits(...)` and `aggregate_orbits_f64_only(...)` own the shared trimming/sorting/finalization logic | same helper path | `OrbitSearchResult { orbits, min_action, min_action_lower, min_action_upper, iterations }` |
| Track frontend-local metadata | At planning time, HK2017 still carried winner-subset metadata and billiard still carried winner-bounce metadata in their local wrappers | local wrapper metadata only |
| Finalize | `OrbitSearchResult` plus scalar convenience accessors like `capacity()` and `best_subset()` | same | `OrbitSearchResult` + local wrapper fields |

## Why this seam is the smallest useful one

- The duplicate code is not the high-level candidate generator.
  HK2017 and billiard produce sigmas in structurally different ways:
  subset/permutation traversal versus block-structured bounce enumeration.
- The duplicate code starts at `sigma -> solve -> classify -> aggregate`;
  Packet 2 now replaces that with the shared
  `sigma -> solve_orbit_sigma -> aggregate_orbits` seam.
- The frontend-local data that still matters after solving is small and
  different:
  HK2017 needs the certified winner's unordered subset,
  billiard needs the certified winner's `k`.
- Packet 1 already created the shared target result surface in
  [`orbit_search.rs`](../../../crates/symplectic/src/algorithms/orbit_search.rs:1), so
  Packet 2 can wire a collector there without first solving the candidate
  generation differences.

## Risks if the seam is wrong

- **Too early: extract at candidate generation.**
  This would try to unify `subset + cyclic permutation` and `q/p block bounce`
  enumeration in one abstraction before the new collector exists. Risk: Packet 2
  turns into generic search-framework work instead of plumbing the existing
  stages together.
- **Too late: extract only at wrapped result level.**
  This would still duplicate backend/guarantee plumbing above the new
  `solve_orbit_sigma` primitive and leave search-level accumulation logic forked.
  Risk: shared API names land, but the actual search frontends stay partially
  forked.
- **Swallow frontend metadata into the collector too early.**
  `best_subset_certified` and `best_bounce_certified` are both “certified
  winner side channels”, but they are not the same thing. Risk: Packet 2
  hard-codes one metadata notion into the shared collector and makes the other
  frontend awkward.

## Packet-2-friendly extraction sketch

Use a shared internal collector that owns:

- backend dispatch to the current solver bridge,
- conversion from feasible `KktResult` into search-level orbit payloads,
- shared capacity/orbit accumulation,
- final `OrbitSearchResult` assembly.

Keep frontend-specific code responsible for:

- generating `sigma` candidates,
- applying any candidate-family-specific pruning before solve,
- maintaining local certified-winner metadata (`subset`, `k`, later maybe
  nothing once old wrappers disappear).

That split is narrow enough for Packet 2 and does not require changing solver
math or deciding the final generic abstraction shape in this planning pass.
