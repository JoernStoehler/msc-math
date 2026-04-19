# Numerics Next Steps

Active objective: finish the last production-facing reliability gaps before final
migration of numerics ideas into durable crates or formal claims.

Primary thread:

1. Close the remaining error-bound and solver-edge proof gaps in `error-bounds`.
   - Objective: extend the current η-bound discussion to cover near-null eigendirections in LP correction and remove the `cor:taylor-structure` gap.
   - Blockers: formal statement refinement is pending; existing empirical violations need to be encoded as guarded assumptions.
   - Commands:
     - `cargo test -p dev-numerical-analysis --test verify_numerics_tests`
     - edit `formal/numerics/error-bounds.tex`
     - if needed, update `experiments/numerics/error-bounds/tests.rs`.
   - Stop condition: the above test target and the formal file both reflect a
     complete bound argument for the LP-shift branch.

2. Decide when to extract `src/algebraic` into a reusable crate.
   - Objective: finalize the scalar boundary as `crates/algebraic-numbers` or
   another deliberately chosen permanent crate name with the minimal API decided above.
   - Blockers: naming decision and compatibility checks with any callsites that still
     depend on experiment-local details.
   - Commands:
     - `cargo test -p dev-numerical-analysis`
     - `cargo test -p dev-numerical-analysis --test verify_numerics_tests` (for algebraic/KKT assertions).
   - Stop condition: stable crate boundary exists and `experiments/numerics`
     imports the same API without local duplication.

3. Complete the Sage feasibility judgement with data in hand.
   - Objective: confirm completion/timing outcomes for `F=5..10` smoke+canonical cases and
     decide whether Sage stays a baseline-only lane.
   - Blockers: exact/approx path differences in Sage can hide integration bugs.
   - Commands:
     - `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --smoke`
     - `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --canonical`
     - `cd experiments/numerics/sage-feasibility && sage -python analyze.py --smoke`
     - `cd experiments/numerics/sage-feasibility && sage -python analyze.py --canonical`
   - Stop condition: `sage-feasibility.jsonl` is updated and classifies each planned
     F-case as completed, timed out, or infeasible.
