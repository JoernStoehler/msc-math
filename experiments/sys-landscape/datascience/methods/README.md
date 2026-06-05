# Sys-Landscape Datascience Methods

This directory owns evidence packets for datascience method-table rows. It does
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
- terminal state when available.

Reports are current evidence only if they were generated from the current
retained dataset. Otherwise they must say they are status markers.

## Method Packet Workflow

Use separated roles for current method packets:

- Orchestrator: prepares the integration branch, chooses the next method row or
  method surface, and records the short `report.md` header when useful.
- Executor: runs the fresh method experiment in a method worktree branched from
  the integration branch. The executor owns the method folder, designs the
  analysis, writes the final report, and deletes or ignores stale source
  material unless extraction is explicitly justified.
- Reviewer: independently checks the packet after execution. The reviewer has
  full repo access, but should not inherit the executor's context window. The
  review focuses on leakage, interpretation, reproducibility, terminal state,
  and whether the packet is useful for the method table.

Prep and queue edits may be made directly on the integration branch. Merge a
method packet into the integration branch only after review and orchestrator
acceptance.

## Result Types

Use one primary terminal state per method row:

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

If a row gets either positive state, stop unrelated method work and write the
follow-up/escalation packet before continuing method-table closure.

## Active Method Rows

This table is a navigation cache. For details, the row's `report.md` and source
code overrule this table.

| Slug | Method row and question | Current result or status | Evidence status | Command or script | Source truth | Validity guard and thesis use |
| --- | --- | --- | --- | --- | --- | --- |
| `table-column-regression` | Table-column regression. Do retained table-column groups trained on random rows predict endpoint rows? | No current retained-dataset result. Old broad-folder artifacts were deleted. | Not evidence until a clean current run exists or the row is abandoned. | No current command; write a row-specific script only if this row remains required. | `table-column-regression/report.md` | Guard: grouped random-to-endpoint prediction. Thesis use: none before rerun or abandonment. |
| `regime-classification` | Endpoint-vs-random classification. Can standard classifiers separate endpoint rows from random rows, and is the separator not just provenance? | No current retained-dataset result. Old broad-folder artifacts were deleted. | Not evidence until a clean current run exists or the row is abandoned. | No current command; write a row-specific script only if this row remains required. | `regime-classification/report.md` | Guard: separate metadata/provenance from geometric table columns. Thesis use: caveat/diagnostic only before any candidate-proposer. |
| `endpoint-residualized-regression` | Endpoint residualized regression. Do endpoint table-column groups add endpoint-only association beyond metadata? | No current retained-dataset result. Old pre-LICCA disposition was deleted. | Not evidence until a clean current run exists or the row is abandoned. | No current command; write a row-specific script only if this row remains required. | `endpoint-residualized-regression/report.md` | Guard: grouped endpoint CV after metadata-first residualization. Thesis use: none before rerun or abandonment. |
| `pca-projection` | PCA projection. Does a low-dimensional linear projection expose a reproducible rule for proposing high-`sys` candidates? | Planned fresh experiment. The old bundled PCA/clustering/anomaly packet is stale source material only. | Not evidence until a fresh current retained-dataset packet replaces `pca-projection/report.md`. | No current command; the worker should design a fresh PCA experiment instead of rerunning the stale bundled script by default. | `pca-projection/report.md` | Guard: a candidate-proposer must be specified before evaluating `sys` and must not use endpoint labels, dataset identity, optimizer provenance, or post-hoc `sys` inspection. Thesis use: pending fresh packet or abandonment. |
| `pca-cluster-anomaly` / clustering row | Clustering or manifold grouping. Does unsupervised grouping define a candidate-proposer? | Same bundled evidence packet as PCA; no current retained-dataset result. | Not evidence until rerun on the `8445`-row retained dataset. | `pca-cluster-anomaly/analyze.py` | `pca-cluster-anomaly/report.md` | Guard: cluster-to-search rule, not rediscovery of producer families. Thesis use: pending rerun or abandonment. |
| `pca-cluster-anomaly` / anomaly row | Anomaly scan. Do outliers in retained table columns point to new high-`sys` candidates? | Same bundled evidence packet as PCA; no current retained-dataset result. | Not evidence until rerun on the `8445`-row retained dataset. | `pca-cluster-anomaly/analyze.py` | `pca-cluster-anomaly/report.md` | Guard: anomaly rule must propose candidates before claiming search value. Thesis use: pending rerun or abandonment. |
| `supervised-alternatives` | Boosting and nearest-neighbor supervised alternatives. Do cheap standard alternatives change the regression/classification verdicts? | Script retained and LICCA path guards updated; report is a status marker only. | Not evidence until rerun on the `8445`-row retained dataset. | `supervised-alternatives/analyze.py` | `supervised-alternatives/report.md` | Guard: same grouped-CV and random-to-endpoint framing as the baseline supervised rows. Thesis use: pending rerun or abandonment. |
| `exact-f64-spot-check` | Exact-vs-f64 spot check. Do stored f64 geometry columns agree with exact rational source coordinates on sampled rows? | Script retained and dataset guards updated; report is a status marker only. | Not evidence until rerun on the `8445`-row retained dataset. | `exact-f64-spot-check/analyze.py` | `exact-f64-spot-check/report.md` | Guard: exact rational recomputation on sampled rows. Thesis use: supporting/caveat only. |

`pca-cluster-anomaly/` is stale source material. It is not current evidence and
should not be rerun by default. It mixed PCA, clustering, anomaly scanning,
feature policy, and report prose in one old packet. Extract from it only if a
fresh worker records a concrete reason. Delete it once current replacement
packets cover the useful rows or explicitly abandon them.

Supporting script:

- `eda.py`: exploratory dataset summary helper, not a method-table verdict by
  itself.

## Coverage Policy

The method table is the deliverable, not any single row. Rows needed by the
thesis should be driven to a terminal state with repo-owned evidence or an
explicit abandoned/deferred reason.

Do not call the method set merely representative. Standard-method coverage must
be run, ruled inapplicable, abandoned for cost, deferred with reason, or
escalated if positive.

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
- terminal state;
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
