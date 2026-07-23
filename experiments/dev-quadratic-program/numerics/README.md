# f64 Capacity Numerics Lens

This packet converts the f64 verification manifest scans into numerics events.
It records capacity differences from fresh reference-route or stored artifact
labels and emits scan-row diagnostics that expose indeterminacy, fallback, and
preprocessing behavior.

Fresh reference-route labels are not exact-capacity oracles. Their producer
validates the binary64 input as exact rational geometry, then calls the mixed
`capacity_auto` route, which includes binary64 route selection and candidate
generation before exact action aggregation. The event field
`exact_geometry_validation_status` preserves the exact geometry-validation fact
separately; it does not strengthen the capacity label. Retained rows have only
stored artifact labels. The verification packet defines expectations and
checks their claim scope; this numerics packet records label differences and
scan diagnostics for the f64 path.

## Current Variables

The producer emits these observations for every verification manifest row:

- capacity, with comparison-label difference fields when a fresh
  reference-route or stored artifact label exists;
- `near_minimizing_sigma_count`, `min_action_gap`, and `sigma_count`;
- KKT admissible, indeterminate, inadmissible, and numerical-failure counts;
- vertex, near-singular, bounded-near-singular, ambiguous-incidence,
  facet-intersection-indeterminate, and omega-indeterminate counters;
- origin LP margin and residual;
- product-rounding drift and near-redundant-facet removal bounds.

It does not yet emit candidate-level or predicate-pair observations. Add those
only through `tracing` events or numerics event rows, not by adding stable
scan-row fields, for variables such as:

- candidate-level action and action interval;
- beta margin, q/error-bound fields, and KKT residuals;
- exact/f64 predicate pairs for vertex incidence, facet intersection, and omega
  signs;
- why a candidate entered the near-minimum band;
- perturbation-run deltas around product ties, HKO-like rows, and near-threshold
  `sys` cases.

## Input Classes

Start with the same cases as the verification packet:

- clean generated and retained generic rows;
- product rows with `near_minimizing_sigma_count > 1`;
- HKO/HKO-like fallback-visible rows;
- edge fixtures for invalid input, product rounding drift, and
  near-redundant-facet preprocessing visibility.

The former ascent endpoint cases are deliberately absent: their retained input
artifacts were retired, so the current verification manifest cannot identify
or reproduce them. Add ascent cases again only with current, owned inputs and a
route-specific comparison contract.

Add only route-relevant cases:

- near-redundant products when preprocessing bounds matter for migration;
- high-sys and near-`sys=1` rows when threshold claims are live;
- targeted perturbations around product ties or f64 fallback boundaries.

## Interpretation

The numerics lens supports calibrated empirical claims about agreement with the
named comparison routes and where the f64 scan reports indeterminacy. It does
not measure f64 error against an exact-capacity oracle, certify f64 capacity
globally, or reimplement the exact correctness suite.

Ranking preservation and gradient/ascent step-decision comparisons belong here
only if f64-native search remains a live route question.

## Current Producer

This packet has a first producer:

```bash
experiments/dev-quadratic-program/numerics/run.sh /tmp/f64-capacity-numerics
```

It reuses the f64 verification scans when present, or runs the verification
packet first. Then it converts the selected scan rows into numerics
`events.jsonl` and runs `scripts/summarize_observations.py`.

The stable output is `events.jsonl` plus the processed CSV summaries written by
the shared numerics summarizer. Any generated Markdown report is only a
same-run reading aid; do not carry it forward as source truth.

The current version emits fresh reference-route comparison diagnostics for
requested generated and preprocessed cases, stored-label comparison diagnostics
for retained manifest cases, and scan-row diagnostics for indeterminacy,
candidate counts, origin LP residuals, product rounding, and near-redundant
preprocessing. These capacity events intentionally have no `oracle_kind`,
`exact`, `abs_error`, or `rel_error` fields.

The separate `f64-capacity-near-singular` binary in `near_singular/` scans
retained artifact cases for nearly singular four-facet intersections and
reports least-squares/recovered-vertex diagnostics:

```bash
cargo run -p exp-dev-quadratic-program --bin f64-capacity-near-singular -- --help
```

Next useful additions are traced candidate-level variables: action intervals,
beta margins, q/error bounds, KKT residuals, exact/f64 predicate pairs, and
exact action ordering inside f64 near-minimum bands.
