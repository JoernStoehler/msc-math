# Crosspolytope Experiment Reasoning

This topic is the one-off capacity computation for the 4D crosspolytope (hyperoctahedron, 16 facets). The experiment supplies a numeric capacity certificate for `crates/symplectic` in `crates/symplectic/src/geom/known_polytopes.rs` and a fast regression in `crates/symplectic/src/algorithms/hk2017/tests_literature.rs`.

The binary in `experiments/crosspolytope/main/main.rs` does not use the public `ehz_capacity` API because that path lacked the required hooks for symmetry reduction and checkpointed backtracking. It performs:
- Directed-graph search over admissible facet cycles rather than `(m-1)!` permutation enumeration.
- Symmetry reduction using `Aut(crosspolytope) ∩ Sp(4, R)` to keep one canonical subset per orbit.
- JSON checkpointing by subset size so interrupted runs can restart from the last completed `m`.

The current local output is `experiments/crosspolytope/main/crosspolytope.jsonl`:
- `facet_count = 16`, `iterations = 31,779,448`.
- `search_complete_through_m = 13`.
- `best_subset = [0, 3, 12, 15]`, `best_permutation = [0, 12, 15, 3]`, `best_beta = [0.25, 0.25, 0.25, 0.25]`.
- `capacity ≈ 4.0`, `sys ≈ 0.75`, `time_capacity_ms ≈ 1,146,164`.
- `symmetry_group_order = 32`, `hyperoctahedral_group_order = 384`.

Interpretation from these artifacts:
- `crates/symplectic/src/geom/known_polytopes.rs` records the crosspolytope capacity as `4.0` with source `computed (no literature value)`.
- `crates/symplectic/src/algorithms/hk2017/tests_literature.rs` contains a fast correctness certificate (`crosspolytope_upper_bound`) based on the same minimizing orbit.
- `formal/crosspolytope/main.tex` references this experiment as the empirical source for the result and symmetry discussion.

Scope/caveat:
- Search is high-confidence complete only through subset size 13. `m = 14..16` are not enumerated in current artifacts, so the absolute minimum remains a computation+assumption statement rather than a fully exhaustive proof inside this package.
