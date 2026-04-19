# HKO Local Maximum Package

Scope: `experiments/hko-local-maximum`.

This package owns neighborhood and proof-route experiments around the HKO2024 counterexample.

Packet map:
- `exact-clarke/` owns the exact first-order proof surface for the `M_10` claim in `R^40`.
- `gradient-analysis/` owns baseline Clarke-subdifferential and exact-certification-bank scaffolding.
- `facet-splitting/` owns fixed-`F` perturbations `F=10→11` without continuation.
- `second-order/` owns finite-difference curvature checks along flat directions.
- `cut-and-ascent/` owns `F=11` post-cut continuation (gradient ascent after one facet split).
- `lagrangian-boundary/` owns local region calibration inside Lagrangian product coordinates.
- `perturbation-neighborhood/` owns direct random perturbation falsification around HKO.
- `sage-validation/` owns Sage cross-checking of selected exact rows.
- `subdifferential-lp/` currently contains the broken old `Phase C` LP script.

Shared code and interface:
- `src/lib.rs` is the shared helper surface (`EXACT_BANK_ENTRIES`, `exact_hko_polytope()`, strict-admissibility HK2017 collector, derivative helpers).
- `Cargo.toml` defines all package bins:
  - `hko-gradient-analysis`, `hko-facet-splitting`, `hko-lagrangian-boundary`,
    `hko-lagrangian-probe`, `hko-perturbation`, `hko-second-order`,
    `hko-cut-and-ascent`, `hko-sage-validation`.

Primary design anchors:
- `research/hko-local-maximum/design/exact-clarke-subgradient.md`
- `research/hko-local-maximum/design/exact-clarke-closure-plan.md`
- `research/hko-local-maximum/design/cut-and-ascent.md`
- `research/hko-local-maximum/design/facet-splitting.md`
- `research/hko-local-maximum/design/second-order.md`
- `research/hko-local-maximum/design/lagrangian-boundary.md`
- `research/hko-local-maximum/design/perturbation-neighborhood.md`
- `research/hko-local-maximum/design/sage-validation.md`
