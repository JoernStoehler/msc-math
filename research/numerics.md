# Numerics Research Note

## Scope

`experiments/numerics` is the exploratory and validation surface for KKT-capacity numerics used in the thesis.
Durable math claims should move to formal files and `crates/` only after stabilization.
This note owns the topic state while keeping runnable code and canonical outputs in
`experiments/numerics`.

## Current State

`experiments/numerics` remains the active evidence layer for the following packets:

- `algebraic-exactness` owns exact-algebraic checks and selected exact KKT experiments.
  Canonical artifacts include `exact-polytopes.jsonl` and `exact-kkt-comparison.jsonl`.
  `smoke-*` files are still non-canonical local-run products.
- `error-bounds` is the abstract-numerics validation packet.
  Its three-stage workflow (`collect` → `run` → `analyze`) and `tests.rs`
  make it a reproducible f64-vs-exact and bound-behavior harness.
- `q-error` and `kkt-inertia` are confirmation packets for known-polytopes coverage and
  tested winners.
- `unknown-predicates` checks whether UNKNOWN admissibility currently changes selected
  outputs; current data suggest it is a numeric-noise concern rather than a correctness gap.
- `sage-feasibility` remains exploratory and compares Rust orchestration with a Sage baseline
  on controlled benchmark families.

Recent consolidation rule is to avoid expanding this tree as a chronological log.
Each packet is now expected to stay narrow with explicit artifact contracts, while
`crates/` owns durable API only when stability is reached.

## Evidence And Interpretation

- Core empirical support in this scope is the bound
  `|Q−Q*| ≤ ||H||·||β̃||·||r||/σ_min(C)` on relevant datasets.
- A structure-based failure mode has been documented for rank-deficient cases.
- `q-error` reports bound correctness and exact comparison on all non-singular `F≤10` winners.
- `kkt-inertia` confirms inertia decomposition on tested known polytopes and classifies a few
  mismatches as eigenvalue-threshold artifacts.
- `unknown-predicates` does not currently show output changes from UNKNOWN admissibility
  cases on current datasets.
- `sage-feasibility` contributes timing and feasibility quantification without imposing new
  API complexity on Rust crate architecture.

## Decisions

- The experiment-first boundary is preserved for geometry and exact-KKT prototyping; `crates/`
  behavior is not being retrofitted first.
- Algebraic arithmetic is currently an ordered-field boundary, not a symbolic CAS.
- `BigRational` is used directly (or via alias) in v1, with no generic runtime-field API.
- `Sign`/`cmp_real` remain the admissibility branch criterion; `to_f64()` is reporting only.
- Serialization uses canonical coefficients only to keep persisted artifacts deterministic.
- KKT error propagation remains trinary (`TRUE` / `FALSE` / `INDETERMINATE`) with lazy exact fallback.
- In `num-projection` and interior `β>0` settings, the practical bound is
  `E = ||H||·||β̃||·||r||/σ_min(C)`, with exact arithmetic as diagnostic ground truth.
- `algebraic-exactness` stays experiment-owned until `algebraic-numbers` reaches a stable minimal
  surface and formalized tests.
- `error-bounds`, `q-error`, and `kkt-inertia` stay anchored to known-polytopes evidence while
  formal gaps are addressed before data expansion.
- `sage-feasibility` stays temporary and method-focused unless timing and completion evidence
  justifies broader architectural impact.

## History

- `README.md`, `RESEARCH.md`, and `PLAN-error-bounds.md` are superseded by this topic note layer.
- `numerics` planning and migration work previously split across three legacy note files under `experiments/numerics`
  has been merged here.
- The `experiments/numerics` note files have been used as the archival record for what is now the
  research-facing state and not a running execution runbook.

## Next Steps

1. Close remaining error-bound and solver-edge proof gaps in `error-bounds`:
   - extend the current η-bound discussion to near-null eigendirections in LP correction,
     remove the `cor:taylor-structure` gap, and encode current empirical violations as guarded assumptions;
   - edit `formal/numerics/error-bounds.tex` and update `experiments/numerics/error-bounds/tests.rs` as needed;
   - finish when the bound argument for the LP-shift branch is reflected in both test and formal text.
   - relevant commands include
     `cargo test -p dev-numerical-analysis --test verify_numerics_tests`.
2. Decide when to extract `src/algebraic` into a reusable crate:
   - finalize scalar boundary as `crates/algebraic-numbers` or another dedicated crate;
   - resolve compatibility calls for any remaining experiment-local details;
   - relevant commands include
     `cargo test -p dev-numerical-analysis` and
     `cargo test -p dev-numerical-analysis --test verify_numerics_tests`.
3. Complete Sage feasibility judgement with planned data:
   - confirm completion/timing for `F=5..10` smoke and canonical runs;
   - verify baseline-only positioning if required;
   - relevant commands include
     `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --smoke`,
     `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --canonical`,
     `cd experiments/numerics/sage-feasibility && sage -python analyze.py --smoke`,
     `cd experiments/numerics/sage-feasibility && sage -python analyze.py --canonical`.
