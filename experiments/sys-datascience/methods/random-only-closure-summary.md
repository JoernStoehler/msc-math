# Random-Only Method Closure Summary

Purpose: current cross-method dashboard for the trusted random/product
`sys` datascience slice. This file is a navigation and disposition summary, not
an additional method result. Source truth remains each method packet README,
the method artifacts, the retained table fingerprint, and the feature-space
coverage ledger.

For checklist-family dispositions, read
`random-only-method-dispositions.md`. The checklist itself remains a recall aid,
not a result ledger.

Epistemic status on this branch: implementation and smoke coverage improved,
and the main random-only method packets have been rerun with this branch's
method code against the hydrated checked-in retained tables from
`/workspaces/msc-math/experiments/sys-datascience/prepare`. Those tables still
predate the prepare-stage rebuild with new omega/two-face/provenance columns.
Do not use this file to claim thesis closure until the pending prepare rebuild,
post-rebuild reruns, and reviews are recorded in the method packets and the
feature-space ledger.

## Current Data Slice

Trusted random/product rows are defined by
`trusted-random-dataset/` and shared helper code in `_shared/random_only.py`.

Current retained-table random-only fingerprint recorded by method packets:

| dataset | rows | `sys > 1` rows |
| --- | ---: | ---: |
| `random_sample` | `4096` | `0` |
| `random_product_sample` | `10240` | `0` |
| trusted random/product total | `14336` | `0` |

Current branch caveat: the checked-in retained tables predate the new omega,
two-face, and explicit provenance metadata schema changes in this branch.
Current method artifacts are full-table reruns for the old retained schema plus
branch method-side diagnostics; they are not thesis evidence for the new
prepare columns.

## Method Summary

| Method group | Packet | Data slice | Test performed | Current result | Caveat | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| trusted input filter | `trusted-random-dataset/` | retained random/product rows | shared provenance filter, row counts, duplicate/exclusion audit | `14336` trusted rows, `0` duplicates, no excluded-label hits | retained artifact predates new explicit provenance fields | input contract; rerun after retained rebuild |
| target predicate scan | `scan-sys-gt-1/` | retained random/product rows and broader retained table when not using `--random-only` | direct `sys > 1` scan | random-only scoped run: `0` positives in `14336` rows | table-scoped absence only; does not validate capacity/sys computations | baseline negative scan |
| EDA and tail summaries | `random-tail-eda/` | retained random/product rows and overlapping source/facet/product filters | marginal distribution, quantiles, top tail, generator contract, source-parameter availability | current old-schema artifact has max `sys = 0.86258589584944`, no positive row; explicit source-parameter fields absent in old provenance | tail extrapolations are unstable; prepare rebuild pending for new provenance fields | descriptive EDA, not a proposer |
| scalar association screening | `statistical-associations/` | retained random/product rows | Pearson/Spearman screens, family-max permutation, bootstrap source mean difference, source factor tests | current old-schema artifact has strongest Spearman `0.9384368671850424`, family-max p `0.004975124378109453`, and source/facet/product factor tests | artifact predates new omega/two-face/provenance prepare columns | explanatory screening; clean proposer evidence only after separate generated-row test |
| projections and structure | `projection-structure/` | retained random/product geometry features | PCA, k-means summaries, isolation-forest anomaly overlap, source/facet/product metadata overlays | current old-schema artifact has `88` geometry features, no anomaly/top-tail overlap, and metadata overlays | artifact predates new omega/two-face/provenance prepare columns | exploratory structure only |
| supervised ranking | `prediction-ranking/` | retained random/product geometry features, plus metadata-only diagnostics | grouped holdout ridge/random forest, enrichment permutation, metadata-only source/facet/product baselines | current old-schema artifact: geometry RF `R^2 = 0.921983825923774`, metadata-only RF `R^2 = -0.04953269595337506` | held-out rows already have `sys`; artifact predates new prepare columns | in-table signal, no validated candidate-proposer |
| non-gradient perturbation | `non-gradient-perturbation/` | tiny hash-selected trusted random/product basepoint panel | fixed random directions and fixed radii, no gradient/ascent semantics | smoke panel ran, no `sys > 1`, max increase about `7.9e-4` | deliberately tiny; not broad perturbation coverage | route smoke only |

## Checklist Mapping

This mapping is deliberately coarse. Use
`method-coverage-checklist.md` for recall,
`random-only-method-dispositions.md` for family-level run/defer/reject/out-of-scope
decisions, and the packet READMEs for evidence.

| Checklist family | Current handling in random-only slice | Remaining gap |
| --- | --- | --- |
| baseline scans and target predicate | `scan-sys-gt-1/`, `trusted-random-dataset/` | rerun after retained rebuild |
| distribution, quantiles, source/facet/product EDA | `random-tail-eda/` | prepare rebuild rerun with explicit provenance fields |
| missingness, duplicates, provenance/filter audit | `trusted-random-dataset/`, `random-tail-eda/` availability diagnostics | prepare rebuild rerun with new provenance schema |
| scalar statistical associations | `statistical-associations/` | prepare rebuild rerun with new omega/two-face/source metadata columns |
| grouped validation, leakage, null checks | prediction grouped split, association permutation/bootstrap, metadata-only baselines | method/statistics review after prepare rebuild reruns |
| projections, clustering, density/anomaly checks | `projection-structure/` | prepare rebuild rerun with new geometry columns |
| supervised prediction/ranking | `prediction-ranking/` | prepare rebuild rerun; generated-candidate follow-up only if thesis value justifies it |
| bounded non-gradient perturbation search | `non-gradient-perturbation/` | larger panel only if promoted by value-of-information |
| gradient/ascent/continuation/attractor methods | out of clean random-only scope for this feature-space goal | separate ascent/local-max data needed before claims |
| broad derivative-free optimization families | not closed by the current tiny perturbation smoke | defer or create explicit non-gradient panels only if they remain high value after retained reruns |
| post-capacity orbit/KKT interpretation | kept out of clean proposer features by shared selector; available only for interpretation | audit only if thesis wording needs post-capacity explanation |

## Current Blocking Gate

Prepare-stage random/product rebuild and post-rebuild method reruns are the
next evidence gate. Do not use the all-source retained-table rebuild for this
goal by default. Use the scoped prepare tiers first (`smoke`, then `method`),
then full `--random-only` on LICCA for evidence. Until the scoped rebuild and
post-rebuild packet reruns are recorded, current artifacts support only the old
retained schema plus method-side diagnostics, not the updated thesis-facing
random/product datascience claim for new prepare columns.
Use `../licca-post-feature-rebuild.md` for the bounded handoff.

## Thesis Claim Status

Currently supported by old retained artifacts:

- the retained random/product table slice used an explicit trusted filter;
- the old retained random/product table slice had no recorded `sys > 1` rows;
- old EDA/model artifacts found in-table structure but no validated generated
  candidate-proposer.

Not yet supported for the updated branch:

- association, projection, and prediction claims using the new omega matrix,
  omega-sign, normalized-omega, two-face-tail, or explicit provenance metadata
  fields;
- a final random-only method-table closure claim;
- any ascent endpoint, local-maximum, attractor, basin, or continuation claim.

## Reopen Triggers

- retained tables are rebuilt or producer data changes;
- new random/product or non-gradient perturbation rows become trusted inputs;
- a method reports `sys > 1`, near-threshold behavior requiring escalation, or
  a candidate-proposer that ranks unevaluated rows before `sys` computation;
- thesis wording asks for broader random distributions, broader optimization
  families, or ascent/attractor structure.
