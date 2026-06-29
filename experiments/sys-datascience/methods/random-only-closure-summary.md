# Random-Only Method Closure Summary

Purpose: current cross-method dashboard for the trusted random/product
`sys` datascience slice. This is a navigation and disposition summary, not an
additional method result. Source truth remains each method packet README,
method artifacts, retained table fingerprints, and the feature-space ledger.

Epistemic status: current-schema scratch evidence exists, but final closure is
not yet reviewed into durable method packets. On 2026-06-25, the full
random/product prepare rerun and active method reruns completed in `/tmp`:
`/tmp/sys-ds-random-only-full-current` and `/tmp/sys-ds-full-current/`. The
remaining closure work is method/statistics review, updating packet READMEs,
and deciding whether the full prepared table should be retained or regenerated
on demand.

## Active Data Slice

Trusted rows are defined by `trusted-random-dataset/` and shared helper code in
`_shared/random_only.py`.

Current full scoped random/product counts from the 2026-06-25 scratch rerun:

| dataset | rows | `sys > 1` rows |
| --- | ---: | ---: |
| `random_sample` | `4096` | `0` |
| `random_product_sample` | `10240` | `0` |
| trusted random/product total | `14336` | `0` |

These counts must be refreshed against the current prepare schema before final
thesis use if producer data or feature schema changes again.

## Active Method Summary

| Method group | Packet | Current role | Pending gate |
| --- | --- | --- | --- |
| trusted input filter | `trusted-random-dataset/` | row/provenance filter and audit | review and README update |
| target predicate scan | `scan-sys-gt-1/` | direct `sys > 1` scan | review and README update |
| EDA and tail summaries | `random-tail-eda/` | finite-sample marginal and source/parameter EDA | tail-language review |
| scalar association screening | `statistical-associations/` | univariate association and source/factor checks | statistics review |
| projections and structure | `projection-structure/` | PCA, clustering, anomaly diagnostics | interpretation review |
| supervised ranking | `prediction-ranking/` | in-table signal and held-out ranking diagnostics | proposer-language review |
| interpretable tail rules | `tail-rule-mining/` | shallow invariant-feature versus strata/provenance-control high-tail rules | method/statistics review |

Removed from the active slice: ascent endpoint diagnostics, local-behavior
prediction, continuation, and perturbation panels.

## Current Blocking Gate

1. Review method/statistics interpretations and update packet READMEs from the
   2026-06-25 scratch artifacts.
2. Decide whether to retain the full prepared table or treat it as regenerated
   evidence.

## Thesis Claim Status

Currently supported by 2026-06-25 current-schema scratch artifacts:

- the random/product method table used an explicit trusted filter;
- the scoped random/product table had no recorded `sys > 1` row;
- EDA/model artifacts found in-table structure but no validated
  generated-candidate proposer.
- the old trial tail-rule method found geometry-only high-tail enrichment beyond
  strata/provenance controls on grouped holdouts, but those artifacts predate
  the invariant-only active schema and need rerun before thesis use.

Not yet supported:

- final random-only method-table closure;
- claims about arbitrary random distributions beyond the retained producer
  contract.

## Reopen Triggers

- retained tables are rebuilt or producer data changes;
- a new random/product distribution is added;
- a method reports `sys > 1`, near-threshold behavior requiring escalation, or
  a credible candidate-proposer;
- thesis wording asks for broader random distributions or stronger statistical
  interpretation.
