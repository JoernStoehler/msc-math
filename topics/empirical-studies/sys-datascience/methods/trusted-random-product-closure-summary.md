# Trusted Random/Product Method Closure Summary

Purpose: current cross-method dashboard for the trusted random/product
`sys` datascience slice. This is a navigation and disposition summary, not an
additional method result. Source truth remains each method packet README,
method artifacts, retained table fingerprints, and the feature-space ledger.

Epistemic status: packet READMEs retain results under two invariant schemas:
39-feature registered-table analyses and a later 45-feature rebuilt table used
by P2 and tail-rule diagnostics. Today's shared selector requires 45 features,
so the earlier summaries are frozen historical results rather than outputs of
a no-argument rerun today. See [the method account](../../methods.md).
This navigation summary does not settle which results deserve thesis claims.

## Active Data Slice

Trusted rows are defined by `trusted-random-dataset/` and shared helper code in
`_shared/random_only.py`.

Current retained random/product counts from the invariant table:

| dataset | rows | `sys > 1` rows |
| --- | ---: | ---: |
| `random_sample` | `4096` | `0` |
| `random_product_sample` | `10240` | `0` |
| trusted random/product total | `14336` | `0` |

These counts must be refreshed if producer data, retained filters, or the active
feature schema changes again.

## Active Method Summary

| Method group | Packet | Current role | Current gate |
| --- | --- | --- | --- |
| trusted input filter | `trusted-random-dataset/` | row/provenance filter and audit | thesis-use judgment |
| target predicate scan | `scan-sys-gt-1/` | direct `sys > 1` scan | none for table-scoped absence |
| EDA and tail summaries | `random-tail-eda/` | finite-sample marginal and source/parameter EDA | tail-language judgment |
| scalar association screening | `statistical-associations/` | univariate association and source/factor checks | thesis-use judgment |
| projections and structure | `projection-structure/` | PCA, clustering, anomaly diagnostics | thesis-use judgment |
| supervised ranking | `prediction-ranking/` | in-table signal and held-out ranking diagnostics | proposer-language judgment |
| interpretable tail rules | `tail-rule-mining/` | shallow invariant-feature versus strata/provenance-control high-tail rules | thesis-use judgment |

Removed from the active slice: ascent endpoint diagnostics, local-behavior
prediction, continuation, and perturbation panels.

## Current Gate

A source-reproducible 45-feature rebuild is documented by P2, but the registered
prepared snapshot still lacks six required ridge columns. Default strict
feature-consuming reruns therefore need a rebuilt run-local table or a newly
registered snapshot. Detailed tail-rule outputs additionally remain outside
this packet tree. These reproduction limitations are separate from thesis-use
judgment and from the independent generated-candidate validation boundary.

## Thesis Claim Status

Currently supported by the integrated invariant-schema rerun:

- the random/product method table used an explicit trusted filter;
- the scoped random/product table had no recorded `sys > 1` row;
- EDA/model artifacts found in-table structure. A later fresh generated packet
  validated one exact sub-threshold ridge-concentration pool-screening rule;
  it did not validate threshold crossing.
- the integrated invariant-schema rerun found high-tail enrichment from ridge
  symplectic-area invariants, while separately reported coarse
  strata/provenance feature sets were much weaker on grouped holdouts. This
  remains in-table evidence rather than a validated generated-candidate
  proposer.

Not yet supported:

- claims about arbitrary random distributions beyond the retained producer
  contract.
- a proposer validated for finding `sys > 1` or a calibrated threshold hit rate.

## Reopen Triggers

- retained tables are rebuilt or producer data changes;
- a new random/product distribution is added;
- an active method produces a new `sys > 1` candidate/source beyond declared
  known-positive reference/control inputs, near-threshold behavior requiring
  escalation, or a credible new candidate-proposer;
- thesis wording asks for broader random distributions or stronger statistical
  interpretation.
