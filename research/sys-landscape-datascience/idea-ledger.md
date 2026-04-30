# Sys-Landscape Data-Science Idea Ledger

## Purpose

This file owns the data-science idea-exhaustion loop for the hostile
sys-landscape strand.

Finishing this strand means that every standard or plausible idea has one of
these outcomes:

- it produced an actionable search rule and was escalated;
- it produced a bounded negative result;
- it was tried and falsified as a positive route;
- it was rejected because the value of information is too low for its cost;
- it was deferred as future work with a concrete reopen trigger.

The method ledger records methods already visible in the repo. This file records
the broader queue of proposed ideas, spike attempts, process lessons, and
reasons not to continue.

Process authority: `tasks/landscape.md` owns the current data-science
subexperiment workflow, including worker-packet fields, result qualifiers,
closure rules, reviewer checklist, and process maturity stage. This ledger owns
the idea rows and their evidence links.

## Current Dataset Snapshot

Observed on 2026-04-30 from the committed producer caches:

- Command:
  `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir /tmp/sys-ds-plan-tables`
- Output tables:
  `/tmp/sys-ds-plan-tables/polytope-table.jsonl` and
  `/tmp/sys-ds-plan-tables/observation-table.jsonl`
- Row counts:
  `282` polytope rows and `282` observation rows.
- Observation counts:
  `70` random generic, `100` random product, `10` general ascent,
  `12` product ascent, `90` variable-F continuation.
- Table width:
  `133` polytope-table fields and `47` observation-table fields.
- Current maximum in the rebuilt table:
  `sys ~= 0.906316153431123`, with `0` rows satisfying `sys > 1`.

This is a planning snapshot, not a maintained artifact. A spike wave should
build one fresh temp snapshot, record its command and counts, and pass that
snapshot path to all workers in the wave.

## Roles

- Lead research data scientist: owns triage, worker packets, result
  interpretation, trash-or-merge decisions, and Jörn escalation.
- Spike worker: owns one bounded method, column, sanity check, or search idea in
  an isolated worktree.
- Reviewer: checks leakage, stale data, metric validity, overclaiming, and
  whether the worker output supports the requested verdict.
- Jörn: decides thesis-facing interpretation, expensive compute, mathematical
  acceptability, and any discovered `sys > 1` escalation.

## Verdict Vocabulary

Use exactly one terminal verdict per idea row.

- `positive-escalate`: the spike found an actual `sys > 1` row or another result
  that changes the research direction. Stop the wave and escalate to Jörn.
- `conjectured-positive`: the spike found an actionable search rule but has not
  produced the target object. Follow with a falsification/search spike.
- `falsified-positive`: an actionable rule was tried within its declared budget
  and did not find the target object.
- `negative`: the method class produced no useful pattern under stated data,
  metric, and complexity bounds.
- `rejected-low-voi`: the idea is standard or plausible, but current cost,
  ambiguity, or implementation risk is too high for expected information.
- `future`: useful after thesis closeout or after a named prerequisite changes.
- `bug-redo`: the spike exposed a data, code, prompt, or methodology bug; fix the
  bug before interpreting the method result.

## Spike Contract

See `tasks/landscape.md` for the current binding workflow. The notes below are
retained as the local shape of one-turn worker delegation and should be read
through that task-bundle workflow.

Use one-turn worker delegation by default:

1. the lead chooses and defines one bounded experiment;
2. the lead delegates it once to a worktree worker;
3. the lead waits when blocked on the result;
4. if waiting times out, the lead inspects the worktree, `/tmp` artifacts, and
   running processes before messaging or closing the worker;
5. the lead reviews the artifacts and decides whether to merge, trash, leave a
   follow-up branch, or record a negative/rejected verdict;
6. the lead updates this ledger, the toolbox audit, and task priorities.

Do not use interactive chat checkpoints as the default control path for these
spikes. If progress observability matters, require durable report and output
paths in the original packet.

Every spike packet should state:

- required cwd/worktree and a first command that prints `pwd`;
- frozen dataset path and row-count expectation;
- idea, hypothesis, and what would count as positive, negative, or
  inconclusive;
- allowed write scope;
- maximum local runtime and whether LICCA is out of scope;
- leakage guards, including grouped splits where prediction is involved;
- sanity checks, including null or permutation baselines when cheap;
- output artifact path or report path, written early enough that the lead can
  inspect partial work if the agent times out or must be shut down;
- stop conditions, especially `sys > 1`, stale dataset mismatch, or an
  implementation bug that invalidates the result.

Workers may write scratch artifacts in their worktree or under `/tmp`, but they
must not edit tracked `.jsonl` outputs unless the packet explicitly asks for a
canonical refresh.

## Initial Idea Queue

| ID | Idea | Type | Scope | Desired evidence | Current verdict | Evidence / next action |
| --- | --- | --- | --- | --- | --- | --- |
| `DS-I001` | Feature-block regression with ridge and random forest | method | Existing M011 packet over random and endpoint regimes | Grouped CV, random-to-endpoint transfer, null baseline | `negative` | Cached in `research/sys-landscape-toolbox-audit.md`; reopen if refreshed feature tables transfer to endpoints. |
| `DS-I002` | Regime classification | method | Existing M012 packet | Whether non-provenance blocks separate endpoint from random better than null and metadata caveats | `future` | Existing script lacks durable summary; decide thesis role after summary and review. |
| `DS-I003` | Endpoint residualized regression beyond metadata | method | Existing M013 packet | Whether endpoint geometry/orbit/trajectory blocks add grouped-CV signal beyond metadata | `future` | Existing script lacks durable summary; decide thesis role after summary and review. |
| `DS-I004` | PCA / clustering / anomaly scan over current feature blocks | method spike | Existing 282-row table and committed feature JSONL | A non-post-hoc cluster or component rule that suggests where to search, or a bounded negative result | `negative` | Source-truth repair commit `dc4f11a5` on branch `ds-pilot1-pca-cluster` creates `experiments/sys-landscape/datascience/methods/pca-cluster-spike/` with script, report, and summary. Merge pending. |
| `DS-I005` | Cheap supervised alternatives: lasso, elastic net, boosting, kNN | method spike | Current feature tables only | Whether standard extra models change M011/M012 conclusions under the same grouped split policy | `negative` | Harder pilot commit `c9b7cb77` on branch `ds-pilot2-supervised-alts` creates `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/`; lead had to run the worker's script to produce report/summary. Merge pending. |
| `DS-I006` | Null, permutation, and bootstrap uncertainty checks | sanity | Existing M011-M013 outputs | Chance baseline and fold uncertainty for claimed pattern or non-pattern | `negative` | Worker spike found above-null within-regime pockets, but the load-bearing random-to-endpoint transfer still has strongly negative R^2. Evidence was promoted into this ledger and the toolbox audit; scratch worktree discarded rather than merged. |
| `DS-I007` | Exact-vs-f64 spot checks for mathematical columns | sanity | Sampled rows from table-stage features | Detect whether a column implementation turns a true signal into noise | `negative` | Revised-process pilot commit `b7b59ac5` on branch `ds-pilot3-exact-f64` creates `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/`; sampled checked columns showed only f64-scale drift. |
| `DS-I008` | Neural networks or deep latent models | method | Current 282-row dataset | Would need overfit controls and enough rows for flexible models | `rejected-low-voi` | Too small and too easy to overfit before thesis closeout; reopen only with much larger data. |
| `DS-I009` | Bayesian optimization / surrogate-guided search loop | search | Candidate generator plus exact evaluation budget | New high-sys candidates or clear comparison against random/local baselines | `future` | Reopen only with a bounded candidate space and compute budget approved by Jörn. |
| `DS-I010` | New symplectic/geometric feature columns from informal intuition | column | Table-stage additive columns | Computable definition, sanity check, and expected information gain | `future` | Split each proposed column into its own row before implementation. |

## Process Lessons

Record only lessons that change future delegation or spike design.

| Date | Source | Lesson | Consequence |
| --- | --- | --- | --- |
| 2026-04-30 | Initial planning | Table generation from current producer caches is cheap enough for a lead wave setup but not free for every worker. | Build one frozen temp dataset per wave and pass the path to workers. |
| 2026-04-30 | Initial planning | A useful spike must end in a verdict with evidence, not only a plot or script. | Worker packets require a report path and reviewer-readable summary. |
| 2026-04-30 | DS-I004 worker spike | One worker could implement and run a complete PCA/clustering spike from the packet, including row guards, non-provenance features, a permutation sanity check, and a readable verdict. | Full-method spikes are feasible when the dataset path, allowed write scope, and verdict vocabulary are explicit. |
| 2026-04-30 | DS-I004 worker spike | The useful lifecycle is one-turn delegation: choose the experiment, define it, delegate, wait, inspect, then merge, trash, or leave a follow-up. Interactive checkpointing is not the default control path. | Future packets should require durable report/output paths for inspection, not chat-style progress management. |
| 2026-04-30 | DS-I006 worker spike | The corrected one-turn lifecycle worked: the worker returned a completed status and left a report plus JSON summary for review. | Keep default spike packets one-turn, require durable artifact paths, then inspect artifacts before the explicit terminal decision: merge, trash, or follow-up. |
| 2026-04-30 | Stage-3 pilot 1 `DS-I004` | Source-truth repair worked after timeout inspection and one corrective nudge. The worker produced script/report/summary, and the lead reran the script and committed `dc4f11a5` on `ds-pilot1-pca-cluster`. | Keep timeout inspection in the lead loop. A timeout with no files and no running process should trigger one corrective message, not indefinite waiting. |
| 2026-04-30 | Stage-3 pilot 2 `DS-I005` | The harder method pilot exposed a lifecycle failure: after nudging, the worker created a substantial script but did not run it or produce the required report/summary. The lead ran the script and committed `c9b7cb77` on `ds-pilot2-supervised-alts`. | Before stage 4, worker packets need an early artifact heartbeat and a lead-repair disposition so partial code is not mistaken for completed source truth. |
| 2026-04-30 | Process revision after pilots | The workflow now requires an early artifact heartbeat, worker self-run proof, explicit `lead-repair` disposition, a normal long wait before inspection, and worker cleanup after disposition. | Heartbeat is a worker-output requirement, not a lead-side polling ritual. Default local-pilot wait is `10` minutes unless the packet says otherwise. |
| 2026-04-30 | Stage-4 pilot `DS-I007` | Waiting long enough let the worker complete heartbeat, script, report, summary, and self-run proof without lead repair. Manual early heartbeat inspection was unnecessary and likely contributed to overdiagnosing earlier workers as stuck. | Process is revised and settled enough for serial execution. Before scaling, standardize `summary.json` top-level keys and build the row dashboard. |

## Completed Spike Notes

### `DS-I004` PCA / Clustering / Anomaly Scan

Disposition: source-truth repair branch pending merge.

Worker command:

```bash
uv run --script experiments/sys-landscape/datascience/methods/pca-cluster-spike/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr --out-dir experiments/sys-landscape/datascience/methods/pca-cluster-spike
```

Evidence:

- Branch: `ds-pilot1-pca-cluster`.
- Commit: `dc4f11a5`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/pca-cluster-spike/report.md`.
- Summary path after merge:
  `experiments/sys-landscape/datascience/methods/pca-cluster-spike/summary.json`.

Observation:

- Input row guards passed for `282` polytope rows and `282` observation rows.
- The source-truth repair used `99` nonconstant intrinsic numeric polytope
  features and excluded target/capacity columns, raw vertex arrays, ids,
  sigma/orbit-search witness columns, and all observation metadata/provenance.
- PC1 explained about `38.6%` of standardized feature variance and had
  `|corr(sys)| ~= 0.758`; its top absolute-score rows had mean `sys ~= 0.213`
  and max `sys ~= 0.833`.
- Silhouette selected KMeans `k = 2`; the best mean-`sys` cluster had `234`
  rows, mean `sys ~= 0.564`, max `sys ~= 0.906`, and mixed endpoint/random
  membership.
- Across `k = 2..8`, the highest mean-`sys` cluster was endpoint/dataset-heavy
  with dominant dataset `variable_f_ascent`.
- IsolationForest anomalies were not high-sys enriched; anomaly mean
  `sys ~= 0.067` versus normal mean `sys ~= 0.532`.

Inference:

PCA and clustering see real structure in the current feature table, but the
structure does not define a non-post-hoc candidate generator. The high-sys side
mostly recovers existing endpoint producers. A useful follow-up would need a
generator-side rule for sampling toward a feature-space region without using
`sys`, endpoint labels, dataset identity, or optimizer provenance.

Verdict: `negative`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = high`;
`thesis_use = supporting/caveat only`.

### `DS-I005` Cheap Supervised Alternatives

Disposition: source-truth repair branch pending merge; process result was a
partial worker failure repaired by the lead.

Lead command:

```bash
uv run --script experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr --permutations 20
```

Evidence:

- Branch: `ds-pilot2-supervised-alts`.
- Commit: `c9b7cb77`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/REPORT.md`.
- Summary path after merge:
  `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/summary.json`.

Observation:

- Input guards passed for `282` polytope rows and `282` observation rows, with
  max `sys = 0.906316153431123` and zero `sys > 1` rows.
- Regression panel: lasso, elastic net, histogram gradient boosting, extra
  trees, and kNN.
- Claim-bearing feature matrices excluded target/capacity columns, raw arrays,
  ids, and observation provenance; the cleaner block also excluded orbit-search
  scalar columns.
- Best random-to-endpoint transfer among claim-bearing alternatives was
  histogram gradient boosting on `intrinsic_no_orbit_search`, with
  `R^2 ~= -2.8894`.
- Within-random and within-endpoint fits were positive (`R^2 ~= 0.8995` and
  `R^2 ~= 0.3268` for the best intrinsic blocks), but those do not transfer to
  the load-bearing endpoint prediction surface.
- Endpoint-vs-random classification from intrinsic numeric features remained
  strong: best balanced accuracy `~= 0.9451`, ROC AUC `~= 0.9931`.

Inference:

Cheap supervised alternatives do not change the M011 search-usefulness story:
the load-bearing random-to-endpoint surface remains negative even with flexible
tree and kNN alternatives. Regime classification remains a table/regime
observation and not a generator rule for new high-`sys` candidates.

Verdict: `negative`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = medium`;
`thesis_use = supporting/caveat only`.

### `DS-I006` Null / Permutation / Bootstrap Sanity Checks

Disposition: evidence promoted into this ledger and
`research/sys-landscape-toolbox-audit.md`; scratch worktree discarded.

Worker command:

```bash
uv run experiments/sys-landscape/datascience/methods/stat-sanity-spike/analyze.py --dataset-dir /tmp/sys-ds-plan-tables --out-dir /tmp/ds-stat-sanity-spike
```

Observation:

- Input row guards passed for `282` polytope rows and `282` observation rows,
  with `170` random rows, `112` endpoint rows, `212` grouped split ids, and
  `0` rows satisfying `sys > 1`.
- The spike used ridge regression with grouped splits for within-random,
  within-endpoint, and random-to-endpoint tasks, plus balanced logistic
  regression for endpoint-vs-random classification.
- Regression nulls permuted `sys` labels inside each evaluation surface; the
  random-to-endpoint null trained on permuted random-regime labels and scored on
  true endpoint labels.
- The best within-random result was the `all` block with real `R^2 = 0.5985`
  against null p95 `0.0741`.
- The best within-endpoint result was the `face_symplectic` block with real
  `R^2 = 0.4132` against null p95 `0.1960`.
- The best random-to-endpoint transfer result was also `face_symplectic`, with
  real `R^2 = -9.1481` against null p95 `-14.6865`.
- Endpoint-vs-random classification was strongest for metadata, with balanced
  accuracy `1.0000` and ROC AUC `1.0000`; the `all_non_metadata` block also
  reached balanced accuracy `1.0000`.

Inference:

The null checks make the within-regime signal harder to dismiss as pure chance,
but they do not produce a search rule. The transfer result can beat a weak
permuted-label baseline while still being unusable for endpoint prediction,
because its actual endpoint `R^2` is strongly negative. Classification confirms
that the table separates producer regimes, including with non-metadata blocks,
but this does not identify where to sample for new `sys > 1` rows.

Verdict: `negative`.

### `DS-I007` Exact-vs-f64 Spot Check

Disposition: source-truth branch merged into the integration branch; not merged
to `main`.

Worker command:

```bash
uv run --script experiments/sys-landscape/datascience/methods/exact-f64-spot-check/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr
```

Evidence:

- Branch: `ds-pilot3-exact-f64`.
- Commit: `b7b59ac5`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/report.md`.
- Summary path after merge:
  `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/summary.json`.

Observation:

- Input guards passed for `282` polytope rows and `282` observation rows, with
  max `sys = 0.906316153431123` and zero `sys > 1` rows.
- The deterministic sample had `14` rows, including top-`sys` rows.
- Exact rational `dual_vertices_rational` matched stored
  `dual_vertices_f64` / `dual_vertices_flat_f64` with max coordinate error
  `0.0` in the sample.
- Selected f64 geometry scalar recomputation differed by at most `1.776e-15`.

Inference:

The sampled vertex encodings and selected geometry scalar columns are internally
consistent at f64 scale for the checked rows. This does not check exact
semantics for volume, capacity, skeleton/ridge, transition, or orbit-search
quantities.

Verdict: `negative`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = high`;
`thesis_use = supporting/caveat only`.
