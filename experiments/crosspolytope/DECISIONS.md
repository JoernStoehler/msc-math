# Crosspolytope Experiment Decisions

Operational decisions that affect future work:
- Keep computation in `experiments/crosspolytope` instead of moving to `crates/symplectic`.
  - Reason: it is a one-off numeric derivation with specialized search machinery and local output artifacts.
- Keep custom KKT-complete flow in-package rather than reusing only library public APIs.
  - Reason: the library path at this stage lacks symmetry hooks and checkpointing required for the feasible runtime at `F = 16`.
- Use symmetry reduction with canonical representatives per subset orbit.
  - Decision point: group intersection is fixed to `Aut(crosspolytope) ∩ Sp(4, R)` (order 32).
- Run in release mode only.
  - `MAX_SUBSET_SIZE = 13` in `main.rs` is a pragmatic cutoff despite non-proven tail-optimality.
  - This avoids several-hour runs at larger `m` while preserving the best known action at `m = 4`.
- Treat `crosspolytope_upper_bound` as a fast *upper-bound* certificate.
  - It demonstrates feasible orbit action `4.0` (`beta = 1/4`) in tests; it does not itself prove global optimality without the symmetry-reduced enumeration from this experiment.
- Preserve output only at `experiments/crosspolytope/main/crosspolytope.jsonl`.
  - This is the single durable local artifact used by downstream files.

Rejected or delayed routes:
- No full exhaustive `m = 14..16` continuation in current run.
- No attempt to refactor experiment notes back into the removed `research/` namespace.
