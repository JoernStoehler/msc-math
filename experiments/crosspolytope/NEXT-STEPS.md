# Crosspolytope Experiment Next Steps

Immediate objective: decide whether the package should move from “high-confidence” to fully proven complete search for `m = 14..16`.

Current blocker:
- No rigorous proof in this topic that the minimum cannot drop at `m = 14..16`, so `search_complete_through_m = 13` remains an explicit confidence tradeoff.

Next work packet:
1. If a full proof is required, either:
   - derive a mathematical exclusion argument for `m = 14..16`, or
   - extend this binary and run with a larger cap.

2. To continue compute:
   - Edit `experiments/crosspolytope/main/main.rs` (`MAX_SUBSET_SIZE`).
   - Run: `cargo run -p crosspolytope --release --bin crosspolytope`.
   - Verify `experiments/crosspolytope/main/crosspolytope.jsonl` for `search_complete_through_m`, best orbit, and `time_capacity_ms`.

3. If the capacity candidate changes:
   - Update `crates/symplectic/src/geom/known_polytopes.rs` and `crates/symplectic/src/algorithms/hk2017/tests_literature.rs`.
   - Re-run targeted tests for `crosspolytope_upper_bound`.

Stop condition:
- Stop when either a proof excludes the remaining subset sizes or a complete run records a finalized search through `m = 16` with a new artifact and matching crate-level checks.

Watch list:
- Open question remains: is `c_EHZ(crosspolytope) = c_EHZ(hypercube)` a general duality phenomenon or an isolated coincidence worth formalizing separately?
