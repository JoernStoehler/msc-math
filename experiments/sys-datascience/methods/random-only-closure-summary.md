# Random-Only Method Closure Summary

Purpose: current cross-method dashboard for the trusted random/product
`sys` datascience slice. This is a navigation and disposition summary, not an
additional method result. Source truth remains each method packet README,
method artifacts, retained table fingerprints, and the feature-space ledger.

Epistemic status: the active packets have been rerun under the invariant
feature contract, and durable method READMEs record the current numbers from
the retained invariant tables. This summary is a navigation dashboard; thesis
wording still needs separate judgment about which results are worth claiming.

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

No schema-repair gate remains for the active invariant-feature table. The next
gate is thesis-facing interpretation: whether to cite these packets as
finite-sample explanatory evidence, or to run a separate generated-candidate
experiment before making a stronger data-science claim.

## Thesis Claim Status

Currently supported by the integrated invariant-schema rerun:

- the random/product method table used an explicit trusted filter;
- the scoped random/product table had no recorded `sys > 1` row;
- EDA/model artifacts found in-table structure but no validated
  generated-candidate proposer.
- the integrated invariant-schema rerun found high-tail enrichment from ridge
  symplectic-area invariants, while separately reported coarse
  strata/provenance feature sets were much weaker on grouped holdouts. This
  remains in-table evidence rather than a validated generated-candidate
  proposer.

Not yet supported:

- claims about arbitrary random distributions beyond the retained producer
  contract.
- a validated generated-candidate proposer.

## Reopen Triggers

- retained tables are rebuilt or producer data changes;
- a new random/product distribution is added;
- a method reports `sys > 1`, near-threshold behavior requiring escalation, or
  a credible candidate-proposer;
- thesis wording asks for broader random distributions or stronger statistical
  interpretation.
