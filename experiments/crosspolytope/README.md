# Crosspolytope Computation

This package owns the crosspolytope-facing state for the one-off 4D crosspolytope
(hyperoctahedron, 16 facets) capacity computation. Code, runs, and artifacts
stay under `experiments/crosspolytope/`.

## Scope

The experiment provides a numeric capacity certificate for
`crates/symplectic/src/geom/known_polytopes.rs` and a fast regression in
`crates/symplectic/src/algorithms/hk2017/tests_literature.rs`.

## Current State

The binary in `main/main.rs` does not use the public `ehz_capacity` API because
that path lacked the hooks needed for symmetry reduction and checkpointed
backtracking. The current binary performs:
- directed-graph search over admissible facet cycles rather than `(m-1)!`
  permutation enumeration;
- symmetry reduction using `Aut(crosspolytope) ∩ Sp(4, R)` to keep one canonical
  subset per orbit;
- JSON checkpointing by subset size so interrupted runs can restart from the
  last completed `m`.

The current local output is `main/crosspolytope.jsonl`:
- `facet_count = 16`, `iterations = 31,779,448`;
- `search_complete_through_m = 13`;
- `best_subset = [0, 3, 12, 15]`,
  `best_permutation = [0, 12, 15, 3]`,
  `best_beta = [0.25, 0.25, 0.25, 0.25]`;
- `capacity ≈ 4.0`, `sys ≈ 0.75`, `time_capacity_ms ≈ 1,146,164`;
- `symmetry_group_order = 32`, `hyperoctahedral_group_order = 384`.

## Evidence And Interpretation

- `crates/symplectic/src/geom/known_polytopes.rs` records the crosspolytope
  capacity as `4.0` with source `computed (no literature value)`.
- `crates/symplectic/src/algorithms/hk2017/tests_literature.rs` contains a fast
  correctness certificate (`crosspolytope_upper_bound`) based on the same
  minimizing orbit.
- This package remains the local computational source for the result and symmetry
  discussion in downstream notes.

Scope caveat:
- Search is high-confidence complete only through subset size 13. `m = 14..16`
  are not enumerated in current artifacts, so the absolute minimum remains a
  computation-plus-assumption statement rather than a fully exhaustive proof
  inside this package.

## Decisions

- Keep computation in `experiments/crosspolytope` instead of moving it to
  `crates/symplectic`.
  - Reason: this is a one-off numeric derivation with specialized search
    machinery and local output artifacts.
- Keep the custom KKT-complete flow in-package rather than reusing only library
  public APIs.
  - Reason: the library path at this stage lacks symmetry hooks and checkpointing
    required for feasible runtime at `F = 16`.
- Use symmetry reduction with canonical representatives per subset orbit.
  - Decision point: the group intersection is fixed to
    `Aut(crosspolytope) ∩ Sp(4, R)` (order 32).
- Run in release mode only.
  - `MAX_SUBSET_SIZE = 13` in `main.rs` is a pragmatic cutoff despite non-proven
    tail optimality.
  - This avoids several-hour runs at larger `m` while preserving the best known
    action at `m = 4`.
- Treat `crosspolytope_upper_bound` as a fast upper-bound certificate.
  - It demonstrates feasible orbit action `4.0` (`beta = 1/4`) in tests; it does
    not itself prove global optimality without the symmetry-reduced enumeration
    from this experiment.
- Preserve output only at `main/crosspolytope.jsonl`.
  - This is the single durable local artifact used by downstream files.

## History

- Rejected or delayed route: no full exhaustive `m = 14..16` continuation in the
  current run.
- Rejected or delayed route: no attempt to move this topic into an earlier
  `research/` namespace design.

## Next Steps

Immediate objective: decide whether the package should move from high-confidence
to fully proven complete search for `m = 14..16`.

Current blocker:
- No rigorous proof in this topic that the minimum cannot drop at `m = 14..16`,
  so `search_complete_through_m = 13` remains an explicit confidence tradeoff.

Next work packet:
1. If a full proof is required, either:
   - derive a mathematical exclusion argument for `m = 14..16`, or
   - extend this binary and run with a larger cap.
2. To continue compute:
   - edit `main/main.rs` (`MAX_SUBSET_SIZE`);
   - run `cargo run -p crosspolytope --release --bin crosspolytope`;
   - verify `main/crosspolytope.jsonl` for
     `search_complete_through_m`, best orbit, and `time_capacity_ms`.
3. If the capacity candidate changes:
   - update `crates/symplectic/src/geom/known_polytopes.rs` and
     `crates/symplectic/src/algorithms/hk2017/tests_literature.rs`;
   - rerun targeted tests for `crosspolytope_upper_bound`.

Stop condition:
- Stop when either a proof excludes the remaining subset sizes or a complete run
  records a finalized search through `m = 16` with a new artifact and matching
  crate-level checks.

Watch list:
- Open question: is `c_EHZ(crosspolytope) = c_EHZ(hypercube)` a general duality
  phenomenon or an isolated coincidence worth formalizing separately?
