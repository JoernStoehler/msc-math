<!--
Purpose: Packet 2 planning report for the shared capacity/orbit search frontend.
Context: maps the current common enumerate -> solve -> classify -> track seam
across hk2017 pruned, hk2017 unpruned, and billiard before the main thread
rewires them onto the new shared orbit-search surface.
-->

# Packet 2 Shared Search Frontend Seam Report

## Recommended seam

Extract the shared collector **after frontend-specific sigma generation/pruning,
but before frontend-specific certified-winner metadata bookkeeping**.

In current code, that seam is the point where each frontend already has a
concrete `sigma: &[usize]` and then does the same work:

1. solve `(polytope, sigma)` through the saddle-point bridge,
2. convert `KktResult` into `Solution`,
3. submit `Solution` into [`CapacityAccumulator`](../../../library/src/algorithms/capacity_accumulator.rs:98),
4. finalize to a capacity summary.

This is the smallest extraction that lets Packet 2 add shared collector
entrypoints in [`orbit_search.rs`](../../../library/src/algorithms/orbit_search.rs:1)
without changing solver math or forcing the three candidate generators into one
abstraction.

## Files and functions in the current loop

- HK2017 public wrappers:
  [ehz_capacity_unpruned](../../../library/src/algorithms/hk2017/api.rs:18),
  [ehz_capacity](../../../library/src/algorithms/hk2017/api.rs:26)
  wrap the enumeration outcome into `EhzResult`.
- HK2017 enumeration:
  [enumerate_unpruned](../../../library/src/algorithms/hk2017/enumeration.rs:18),
  [enumerate_pruned](../../../library/src/algorithms/hk2017/enumeration.rs:22),
  [enumerate_impl](../../../library/src/algorithms/hk2017/enumeration.rs:26).
- HK2017 candidate generators used inside the loop:
  `combinations` from `hk2017/combinatorics.rs`,
  [for_each_cyclic_permutation](../../../library/src/algorithms/hk2017/permutations.rs:35),
  and optionally
  [build_transition_matrix](../../../library/src/algorithms/facet_adjacency.rs:30) +
  [is_feasible_cycle](../../../library/src/algorithms/facet_adjacency.rs:44).
- HK2017 solver bridge:
  [solve_and_convert](../../../library/src/algorithms/hk2017/solver_bridge.rs:8)
  with private
  [kkt_result_to_solution](../../../library/src/algorithms/hk2017/solver_bridge.rs:14).
- Billiard public frontend:
  [billiard_capacity](../../../library/src/algorithms/billiard/mod.rs:109).
- Billiard candidate preparation/generation:
  `classify_facets` from `billiard/facet_classification.rs`,
  [enumerate_blocks](../../../library/src/algorithms/billiard/block_enumeration.rs:68),
  [enumerate_k_bounce_sigmas](../../../library/src/algorithms/billiard/block_enumeration.rs:102),
  and the same
  [build_transition_matrix](../../../library/src/algorithms/facet_adjacency.rs:30) +
  [is_feasible_cycle](../../../library/src/algorithms/facet_adjacency.rs:44).
- Billiard solver bridge:
  [solve_and_convert](../../../library/src/algorithms/billiard/mod.rs:172)
  with private
  [kkt_result_to_solution](../../../library/src/algorithms/billiard/mod.rs:180).
- Shared tracking/finalization:
  [CapacityAccumulator::new](../../../library/src/algorithms/capacity_accumulator.rs:109),
  [submit](../../../library/src/algorithms/capacity_accumulator.rs:121),
  [finalize](../../../library/src/algorithms/capacity_accumulator.rs:177).
- Shared lower solver contract:
  [solve_kkt_for](../../../library/src/kkt/saddle_point_solver.rs:292),
  [KktOutcome::feasible](../../../library/src/kkt/saddle_point_solver.rs:167),
  [KktResult](../../../library/src/kkt/saddle_point_solver.rs:198).
- Shared Packet 1 target surface for Packet 2 wiring:
  [OrbitKktData / OrbitSearchResult / enums](../../../library/src/algorithms/orbit_search.rs:15).

## Stage map and data passed

| Stage | HK2017 pruned/unpruned | Billiard | Data passed onward |
| --- | --- | --- | --- |
| Candidate generation | `m`, `subset`, cyclic `perm` from [enumerate_impl](../../../library/src/algorithms/hk2017/enumeration.rs:26) | `k`, block selections, `sigma` from [enumerate_k_bounce_sigmas](../../../library/src/algorithms/billiard/block_enumeration.rs:102) | Concrete `sigma: &[usize]` plus frontend-local metadata (`subset` or `k`) |
| Cheap pruning | optional directed adjacency via [is_feasible_cycle](../../../library/src/algorithms/facet_adjacency.rs:44) | same directed adjacency check in [billiard_capacity](../../../library/src/algorithms/billiard/mod.rs:131) | surviving `sigma: &[usize]` |
| Solve | [hk2017::solve_and_convert](../../../library/src/algorithms/hk2017/solver_bridge.rs:8) | [billiard::solve_and_convert](../../../library/src/algorithms/billiard/mod.rs:172) | `Option<Solution>` |
| Classify/convert | private `kkt_result_to_solution` computes `margin = min(beta)`, `verdict = classify_margin(margin)`, and maps `q_corrected -> q` in [hk2017/solver_bridge.rs](../../../library/src/algorithms/hk2017/solver_bridge.rs:14) | identical conversion in [billiard/mod.rs](../../../library/src/algorithms/billiard/mod.rs:180) | `Solution { verdict, q, beta, margin }` |
| Track shared capacity state | [acc.submit(perm, &solution)](../../../library/src/algorithms/hk2017/enumeration.rs:51) | [acc.submit(sigma, &solution)](../../../library/src/algorithms/billiard/mod.rs:150) | two-tier best candidate state inside `CapacityAccumulator` |
| Track frontend-local metadata | HK2017 tracks `best_subset_certified: Option<(action, subset)>` in [enumerate_impl](../../../library/src/algorithms/hk2017/enumeration.rs:30) | Billiard tracks `best_bounce_certified: Option<(action, k)>` in [billiard_capacity](../../../library/src/algorithms/billiard/mod.rs:128) | local wrapper metadata only |
| Finalize | [acc.finalize()?](../../../library/src/algorithms/hk2017/enumeration.rs:57) then derive `best_subset` fallback from `best_permutation` | [acc.finalize()](../../../library/src/algorithms/billiard/mod.rs:155) then default `bounce_count` | `CapacityResult` + local wrapper fields |

## Why this seam is the smallest useful one

- The duplicate code is not the high-level candidate generator.
  HK2017 and billiard produce sigmas in structurally different ways:
  subset/permutation traversal versus block-structured bounce enumeration.
- The duplicate code starts exactly at `sigma -> solve_and_convert -> acc.submit`.
- The frontend-local data that still matters after solving is small and
  different:
  HK2017 needs the certified winner's unordered subset,
  billiard needs the certified winner's `k`.
- Packet 1 already created the shared target result surface in
  [`orbit_search.rs`](../../../library/src/algorithms/orbit_search.rs:1), so
  Packet 2 can wire a collector there without first solving the candidate
  generation differences.

## Risks if the seam is wrong

- **Too early: extract at candidate generation.**
  This would try to unify `subset + cyclic permutation` and `q/p block bounce`
  enumeration in one abstraction before the new collector exists. Risk: Packet 2
  turns into generic search-framework work instead of plumbing the existing
  stages together.
- **Too late: extract only at wrapped result level.**
  This would leave both private `solve_and_convert` copies in place and still
  duplicate backend/guarantee plumbing when Packet 2 adds the new shared
  entrypoints. Risk: shared API names land, but the actual search frontends stay
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
