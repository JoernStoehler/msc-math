# Sys-Landscape Datascience Methods

This directory owns evidence artifacts for datascience method-table rows. It does
not own the retained dataset and it does not own thesis wording.

Read `../README.md` and `../dataset/README.md` before running a method.

## Current Inputs

All methods default to:

```text
experiments/sys-landscape/datascience/dataset/
```

Required files:

- `polytope-table.jsonl`
- `observation-table.jsonl`

Current retained dataset: `8445` polytope rows, `8445` observation rows, max
`sys=0.9750768559799221`, and `0` rows with `sys > 1`.

## Method Folder Rule

One active method folder supports one method-table row. A method folder must
make these visible:

- method-table row role;
- dataset path;
- command, or an explicit statement that no current command exists;
- validity guard;
- current/stale status;
- report path;
- primary result/status label when available.

Reports are current evidence only if they were generated from the current
retained dataset. Otherwise they must say they are status markers.

## Method Row Workflow

Use separated roles for current method-row work:

- Orchestrator: keeps the integration branch maintainable, chooses the next
  method-table row or explicitly named row group by thesis value, value of
  information, and wall-time, and decides whether reviewed work should merge,
  be repaired, split, deferred, abandoned, or escalated.
- Executor: works in a method worktree branched from integration. The executor
  owns the assigned method folder, designs the
  analysis, writes the final report, and deletes or ignores stale source
  material unless extraction is explicitly justified.
- Reviewer: independently checks reviewed work after execution. The reviewer has
  full repo access, but should not inherit the executor's context window. The
  review focuses on leakage, interpretation, reproducibility,
  maintainability, thesis usefulness, and whether the work would create false
  closure or useful follow-up.

Prep and queue edits may be made directly on the integration branch. Merge
method-row work into the integration branch only after review and orchestrator
acceptance.

## Result Types

Use one primary result/status label per current method report:

- `ran with no candidate-proposer and no new validated row`
- `ran as local optimization only`
- `not applicable to this data/search interface`
- `not run within the stated implementation bound`
- `not run within the stated compute/data bound`
- `implementation bug; no method verdict`
- `candidate-proposer`
- `validated new row`

Optional secondary labels:

- `supporting evidence only`
- `validity caveat`
- `future reopen trigger`

A `candidate-proposer` is a reproducible rule that proposes candidate
polytopes or rows before their `sys` values are evaluated. A validated new row
is a committed row with `sys > 1` outside examples already explained by the
HKO2024 construction and its symplectic images or controlled perturbations.

If a row gets either positive state, stop unrelated method work and write an
escalation note stating the evidence, affected thesis claim or wording, and
recommended next action before continuing.

## Active Method Rows

This table is a navigation cache. For details, the row's `report.md` and source
code overrule this table.

| Slug | Method row and question | Current result or status | Evidence status | Command or script | Source truth | Validity guard and thesis use |
| --- | --- | --- | --- | --- | --- | --- |
| `table-column-regression` | Table-column regression. Do retained table-column groups trained on random rows predict endpoint rows? | No current retained-dataset result. Old broad-folder artifacts were deleted. | Not evidence until a clean current run exists or an explicit abandonment reason is recorded. | No current command; write a row-specific script only if this row remains required. | `table-column-regression/report.md` | Guard: grouped random-to-endpoint prediction. Thesis use: none before rerun or abandonment. |
| `regime-classification` | Endpoint-vs-random classification. Can standard classifiers separate endpoint rows from random rows, and is the separator not just provenance? | No current retained-dataset result. Old broad-folder artifacts were deleted. | Not evidence until a clean current run exists or an explicit abandonment reason is recorded. | No current command; write a row-specific script only if this row remains required. | `regime-classification/report.md` | Guard: separate metadata/provenance from geometric table columns. Thesis use: caveat/diagnostic only before any candidate-proposer. |
| `endpoint-residualized-regression` | Endpoint residualized regression. Do endpoint table-column groups add endpoint-only association beyond metadata? | No current retained-dataset result. Old pre-LICCA disposition was deleted. | Not evidence until a clean current run exists or an explicit abandonment reason is recorded. | No current command; write a row-specific script only if this row remains required. | `endpoint-residualized-regression/report.md` | Guard: grouped endpoint CV after metadata-first residualization. Thesis use: none before rerun or abandonment. |
| `pca-projection` | PCA projection. Does a low-dimensional linear projection expose a reproducible rule for proposing high-`sys` candidates? | Planned fresh experiment. The old bundled PCA/clustering/anomaly work is stale source material only. | Not evidence until fresh current retained-dataset work replaces `pca-projection/report.md`. | No current command; the worker should design a fresh PCA experiment instead of rerunning the stale bundled script by default. | `pca-projection/report.md` | Guard: a candidate-proposer must be specified before evaluating `sys` and must not use endpoint labels, dataset identity, optimizer provenance, or post-hoc `sys` inspection. Thesis use: pending fresh work or abandonment. |
| `pca-cluster-anomaly` / clustering row | Clustering or manifold grouping. Does unsupervised grouping define a candidate-proposer? | Same bundled stale source material as PCA; no current retained-dataset result. | Not evidence until rerun on the `8445`-row retained dataset. | `pca-cluster-anomaly/analyze.py` | `pca-cluster-anomaly/report.md` | Guard: cluster-to-search rule, not rediscovery of producer families. Thesis use: pending rerun or abandonment. |
| `pca-cluster-anomaly` / anomaly row | Anomaly scan. Do outliers in retained table columns point to new high-`sys` candidates? | Same bundled stale source material as PCA; no current retained-dataset result. | Not evidence until rerun on the `8445`-row retained dataset. | `pca-cluster-anomaly/analyze.py` | `pca-cluster-anomaly/report.md` | Guard: anomaly rule must propose candidates before claiming search value. Thesis use: pending rerun or abandonment. |
| `supervised-alternatives` | Boosting and nearest-neighbor supervised alternatives. Do cheap standard alternatives change the regression/classification verdicts? | Script retained and LICCA path guards updated; report is a status marker only. | Not evidence until rerun on the `8445`-row retained dataset. | `supervised-alternatives/analyze.py` | `supervised-alternatives/report.md` | Guard: same grouped-CV and random-to-endpoint framing as the baseline supervised rows. Thesis use: pending rerun or abandonment. |
| `exact-f64-spot-check` | Exact-vs-f64 spot check. Do stored f64 geometry columns agree with exact rational source coordinates on sampled rows? | Script retained and dataset guards updated; report is a status marker only. | Not evidence until rerun on the `8445`-row retained dataset. | `exact-f64-spot-check/analyze.py` | `exact-f64-spot-check/report.md` | Guard: exact rational recomputation on sampled rows. Thesis use: supporting/caveat only. |

`pca-cluster-anomaly/` is stale source material. It is not current evidence and
should not be rerun by default. It mixed PCA, clustering, anomaly scanning,
feature policy, and report prose in one old bundle. Extract from it only if a
fresh worker records a concrete reason. Delete it once current replacement
work covers the useful rows or explicitly abandons them.

Supporting script:

- `eda.py`: exploratory dataset summary helper, not a method-table verdict by
  itself.

## Coverage Policy

The method table is the deliverable, not any single row. Rows needed by the
thesis should accumulate current evidence or explicit reasons for
inapplicability, deferral, abandonment, split-out follow-up, or escalation.

Do not call the method set merely representative. Standard-method coverage must
be run, ruled inapplicable, abandoned for cost, deferred with reason, or
escalated if positive. Prioritize remaining work by thesis value, value of
information, and wall-time rather than local row-completion language.

Known omitted/deferred families should be named when they matter for reader
expectations. Current deferred examples include SVMs, neural networks, Bayesian
optimization, and other high-capacity or compute-heavy variants listed in
`future-method-ideas.md` and `taxonomies/`.

## Deleted Broad Folder

`feature-pattern-search/` was deleted from the active tree.

Reason:

- it was not one standard method;
- it mixed regression, classification, residual checks, shared helpers,
  generated local feature views, stale reports, and figures;
- the tracked `feature_*.jsonl` files duplicated retained dataset columns;
- preserving it had high contamination risk.

Do not resurrect files from that folder unless the extraction has positive
expected value after contamination risk. Prefer a small clean script in the
row-specific folder.

## Shared Code

No shared Python package is currently tracked. Create a single shared module
only after two current method scripts need the same helper.

Preferred future path:

```text
common/retained_table_columns.py
```

Shared code must have no report and no thesis claim.

## Report Requirements

A current method report must include:

- dataset path and fingerprint facts used;
- command run, or that no current command exists;
- runtime or confirmation that it fits the accepted local bound;
- validity guard;
- observation;
- inference;
- primary result/status label;
- thesis use;
- reopen condition.

Each report should separate:

- observation: what the committed run or artifact literally shows;
- inference: what the observation supports;
- thesis use: whether the row supports a main claim, only a caveat, only
  future work, or should stay out of the thesis.

Experiments over 30 minutes are not accepted as reproducible local method
evidence. Use smoke runs for development. If a method needs longer compute,
record the reason and escalate before treating it as current evidence.

## Coverage References

Taxonomy snapshots are under `taxonomies/`. They are coverage references, not
repo-state summaries and not thesis evidence.

Read them only when deciding whether a standard method family exists or should
be named:

- `taxonomies/README.md`

Older research/task notes may contain history, but they are not the ordinary
entry point for this method slice. Method reports and source code overrule them
for current row-level evidence status.
