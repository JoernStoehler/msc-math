# f64 Capacity Numerics Lens

This packet converts the f64 verification manifest scans into numerics events.
It evaluates capacity agreement where an exact audit exists, records stored
label differences separately, and emits scan-row diagnostics that expose
indeterminacy, fallback, and preprocessing behavior.

The exact-audit rows are direct f64-vs-exact comparisons. Retained rows with
stored artifact labels are route diagnostics unless they run a fresh exact audit
on the same row being measured. Verification owns the expectation and claim-scope
check. Numerics owns quantitative f64-vs-exact capacity loss, stored-label
differences, and numeric scan diagnostics for the same f64 path.

## Current Variables

The producer emits these observations for every verification manifest row:

- capacity, with exact-audit error fields when the row has fresh exact audit;
- stored-label capacity differences when the row only has a retained label;
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
- ascent endpoint rows;
- HKO/HKO-like fallback-visible rows;
- edge fixtures for invalid input, product rounding drift, and
  near-redundant-facet preprocessing visibility.

Add only route-relevant cases:

- near-redundant products when preprocessing bounds matter for migration;
- high-sys and near-`sys=1` rows when threshold claims are live;
- targeted perturbations around product ties or f64 fallback boundaries.

## Interpretation

The numerics lens supports calibrated empirical claims about capacity loss,
stored-label differences, and where the f64 scan reports indeterminacy. It does
not certify f64 capacity globally and should not reimplement the exact
correctness suite.

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

The current version emits capacity-level exact-vs-f64 observations for
exact-audit manifest cases, stored-label comparison diagnostics for retained
manifest cases, and scan-row diagnostics for indeterminacy, candidate counts,
origin LP residuals, product rounding, and near-redundant preprocessing.

Next useful additions are traced candidate-level variables: action intervals,
beta margins, q/error bounds, KKT residuals, exact/f64 predicate pairs, and
exact action ordering inside f64 near-minimum bands.
