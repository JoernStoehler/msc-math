<!--
Purpose: discovery packet D2 for the repo maintainability / architecture program.
Context: records repeated helper logic across experiments so later sessions can
resume from concrete implementations and copies instead of chat history. This
note separates observed duplication from the suggested helper home; final moves
into `library/` or any new public API promise remain reserved for the main
thread or Jörn.
-->

# Shared-Helper Inventory

## Status

- Status: discovery packet D2, seeded.
- Last updated: 2026-04-16.
- Scope: repeated orbit-enumeration wrappers, `sys` helpers, step-bound logic,
  and solver instrumentation helpers in `experiments/`.

## Method / Evidence

- `rg -n "fn compute_step_bound_detailed|fn ehz_capacity_instrumented|fn enumerate_all_orbits|fn enumerate_all_orbits_inclusive|fn safe_sys|fn compute_sys\\(|fn try_step_a\\(|fn compute_capacity_result|fn compute_capacity\\(" experiments -g '!**/target/**'`
- `nl -ba experiments/sys-landscape/src/lib.rs | sed -n '1,220p'`
- `nl -ba experiments/combinatorial-cells/cell-widths/main.rs | sed -n '150,260p'`
- `nl -ba experiments/combinatorial-cells/convexity/main.rs | sed -n '150,250p'`
- `nl -ba experiments/combinatorial-cells/multiple-crossings/main.rs | sed -n '150,260p'`
- `nl -ba experiments/combinatorial-cells/boundary-characterization/main.rs | sed -n '260,360p'`
- `nl -ba experiments/hko-local-maximum/gradient-analysis/main.rs | sed -n '150,260p'`
- `nl -ba experiments/hko-local-maximum/second-order/main.rs | sed -n '120,220p'`
- `nl -ba experiments/hko-local-maximum/cut-and-ascent/main.rs | sed -n '120,280p'`
- `nl -ba experiments/sys-landscape/gradient-ascent-general/main.rs | sed -n '100,170p'`
- `nl -ba experiments/sys-landscape/gradient-ascent-products/main.rs | sed -n '110,180p'`
- `nl -ba experiments/sys-landscape/variable-f-ascent/main.rs | sed -n '170,230p'`
- `nl -ba experiments/numerics/gradient/numerics/main.rs | sed -n '150,230p'`
- `nl -ba experiments/numerics/gradient/numerics-subdifferential/main.rs | sed -n '345,390p'`
- `nl -ba experiments/numerics/gradient/numerics-edge-cases/main.rs | sed -n '485,535p'`
- `sed -n '1,220p' experiments/numerics/gradient/Cargo.toml`
- `sed -n '1,220p' experiments/numerics/gradient/src/lib.rs`

## Helper Families

### Step-bound event logic

- Current locations:
  - `experiments/sys-landscape/src/lib.rs:83`
  - `experiments/combinatorial-cells/cell-widths/main.rs:130`
  - `experiments/combinatorial-cells/convexity/main.rs:224`
  - `experiments/combinatorial-cells/multiple-crossings/main.rs:238`
  - `experiments/combinatorial-cells/boundary-characterization/main.rs:331`
  - `experiments/hko-local-maximum/cut-and-ascent/main.rs:130`
- Duplication evidence:
  - The `compute_step_bound_detailed` body in `sys-landscape/src/lib.rs` is the same boundary-event classifier used by the combinatorial-cells copies: incidence-flip scan, `omega0` ridge scan, and dual-vertex degeneration scan.
  - `experiments/combinatorial-cells/cell-widths/main.rs:127` says the function was copied from the shared sys-landscape helper crate.
  - The combinatorial-cells files keep the same helper shape and only vary local constants or surrounding reporting.
- Suggested home: `topic-local helper crate`.
- Reason:
  - The shared core is topic-level geometry, not a thesis-facing stable API.
  - `experiments/sys-landscape/src/lib.rs` already exists as the shared helper crate for this family, so it is the natural extraction point for the common classifier.
  - Promotion into `library/` would commit to a public API around an exploratory boundary classifier before that boundary is settled.
- Jörn decision point:
  - Whether the combinatorial-cells copies should keep the local wrapper until the topic boundary is frozen, or import the shared helper crate directly.

### Sys quotient and ascent scaffold

- Current locations:
  - `experiments/sys-landscape/gradient-ascent-general/main.rs:110`
  - `experiments/sys-landscape/gradient-ascent-products/main.rs:116`
  - `experiments/sys-landscape/variable-f-ascent/main.rs:175`
  - `experiments/hko-local-maximum/cut-and-ascent/main.rs:252`
  - `experiments/hko-local-maximum/gradient-analysis/main.rs:491`
  - `experiments/hko-local-maximum/facet-splitting/main.rs:79`
- Duplication evidence:
  - `compute_sys` and `try_step_a` are repeated almost verbatim in the sys-landscape binaries; the only differences are backend policy and, in `variable-f-ascent`, the extra DB argument.
  - `compute_capacity_result` is the same two-step pattern everywhere: compute capacity, then recover the best permutation, but the backend changes between HK2017, billiard, and DB cache.
  - `safe_sys` in `hko-local-maximum` is a local policy wrapper around the same systolic-ratio arithmetic.
- Suggested home: `topic-local helper crate` for the pure quotient + step wrapper; backend lookup stays `per-binary local`.
- Reason:
  - The arithmetic part is stable and reusable.
  - The capacity source is not stable across the binaries, so it should not be forced into one shared signature yet.
  - The DB-cached variant in `variable-f-ascent` is already policy-heavy enough that a single generic wrapper would leak local assumptions.
- Jörn decision point:
  - Whether to extract only the shared quotient/step helpers into `experiments/sys-landscape/src/lib.rs`, or leave the current copies in place until the backend policy also converges.

### Orbit-enumeration wrappers

- Current locations:
  - `experiments/numerics/gradient/numerics/main.rs:353`
  - `experiments/numerics/gradient/numerics-subdifferential/main.rs:353`
  - `experiments/numerics/gradient/numerics-subdifferential/main.rs:361`
  - `experiments/numerics/gradient/numerics-edge-cases/main.rs:495`
- Duplication evidence:
  - `enumerate_all_orbits` is duplicated between the numerics/basic and numerics-subdifferential binaries.
  - `numerics-subdifferential` adds `enumerate_all_orbits_inclusive`, which is the same loop with a different beta threshold.
  - The shared `experiments/numerics/gradient/src/lib.rs` helper crate exists, but it is still empty except for the module header.
- Suggested home: `topic-local helper crate`.
- Reason:
  - The common core is a pure subset/permutation enumeration plus KKT filtering loop, which fits the numerics topic crate better than `library/`.
  - The strict vs inclusive threshold policy is caller-specific and should stay in the binaries.
  - The result shape (`(action, perm, kkt_result)`) is still experiment-facing, so it does not yet justify a public library surface.
- Jörn decision point:
  - Whether the shared core should become a small helper in `experiments/numerics/gradient/src/lib.rs`, or stay copied until more numerics binaries need it.

### Solver instrumentation helpers

- Current locations:
  - `experiments/combinatorial-cells/cell-widths/main.rs:168`
  - `experiments/combinatorial-cells/convexity/main.rs:162`
  - `experiments/combinatorial-cells/multiple-crossings/main.rs:171`
  - `experiments/combinatorial-cells/boundary-characterization/main.rs:264`
  - `experiments/hko-local-maximum/gradient-analysis/main.rs:162`
  - `experiments/hko-local-maximum/second-order/main.rs:126`
- Duplication evidence:
  - Each file defines an `ehz_capacity_instrumented` helper plus local `ValidOrbit` / `InstrumentedResult` types.
  - The shared loop is the same: enumerate cycles, test feasibility, solve KKT, filter by positive `q` and positive beta, sort by action, then report the best orbit and orbit gap.
  - The row payload differs by experiment: some record `orbit_gap`, some record `iterations`, some keep `capacity_uncertain`, and some attach derivative summaries.
- Suggested home: `unclear`.
- Reason:
  - The math core is shared, but the output schema is not.
  - These helpers are instrumentation, not production algorithms, so folding them into `library/` would create a public promise around debug-only metadata.
  - A shared core would still need a decision on which orbit metadata is mandatory versus optional.
- Jörn decision point:
  - Whether the shared core should expose just the orbit enumeration result, or also the extra instrumentation fields used by some binaries.
- Classification stop note:
  - This family is marked `unclear` because the output contract is not settled yet.

## Blockers / Jörn Decision Points

- Do not promote any of these families into `library/` without an explicit API decision.
- Do not collapse the solver instrumentation variants into one shared type until the required metadata is fixed.
- Do not merge the step-bound helper into a new public surface before the combinatorial-cells topic boundary is agreed.

## Next Safe Resume Point

- Re-run the evidence scan above, then inspect `experiments/sys-landscape/src/lib.rs`,
  `experiments/numerics/gradient/src/lib.rs`, `experiments/hko-local-maximum/src/lib.rs`,
  and `experiments/combinatorial-cells/src/lib.rs` to decide which families are ready
  for extraction and which must remain per-binary.
