# Sys-Landscape Data-Science Idea Ledger

## Purpose

This file owns the data-science idea-exhaustion loop for the hostile
sys-landscape strand.

Finishing this strand means that every standard or plausible idea has one of
these outcomes:

- it produced a candidate-proposer and was escalated;
- it produced a bounded no-search-output result;
- it was tried and falsified as a positive route;
- it was rejected because the value of information is too low for its cost;
- it was deferred as future work with a concrete reopen trigger.

The method ledger records methods already visible in the repo. This file records
the broader queue of proposed ideas, spike attempts, process lessons, and
reasons not to continue.

Process authority: `tasks/planning-notes.md` owns current data-science
route choices, readiness gates, and scale/no-scale decisions.
`research/sys-landscape-datascience/worker-procedure.md` owns the reusable
worker-packet procedure. This ledger owns the idea rows and their evidence
links.

## Current Dataset

Observed on 2026-06-03 from the committed producer caches:

- Build command:
  `experiments/sys-landscape/datascience/build-dataset.sh`
- Output tables:
  `experiments/sys-landscape/datascience/dataset/polytope-table.jsonl`
  and
  `experiments/sys-landscape/datascience/dataset/observation-table.jsonl`
- Dataset checks, when needed:
  `uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py experiments/sys-landscape/datascience/dataset`
- Row counts:
  `282` polytope rows and `282` observation rows.
- Observation counts:
  `70` random generic, `100` random product, `10` general ascent,
  `12` product ascent, `90` variable-F continuation.
- Table width:
  `135` polytope-table fields and `53` observation-table fields.
- Current maximum in the rebuilt table:
  `sys ~= 0.906316153431123`, with `0` rows satisfying `sys > 1`.

This is a maintained dataset artifact. A method wave should cite its
fingerprint and pass that dataset path to all workers in the wave.

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

Use exactly one terminal verdict per semantic idea slug.

- `positive-escalate`: the spike found an actual `sys > 1` row outside the
  known HKO2024-derived source, or another result that changes the research
  direction. Stop the wave and escalate to Jörn.
- `candidate-proposer`: the spike records a reproducible rule that proposes
  candidate polytopes or rows before their `sys` values are evaluated. For
  data-science rows, the rule must not use endpoint labels, producer identity,
  optimizer provenance, or post-hoc inspection of `sys`.
- `falsified-candidate-proposer`: a candidate-proposer was tried within its
  declared budget and did not find the target object.
- `no-search-output`: under the stated data, metric, and complexity bounds, the
  method records neither a candidate-proposer nor a validated new `sys > 1` row.
- `rejected-low-voi`: the idea is standard or plausible, but current cost,
  ambiguity, or implementation risk is too high for expected information.
- `future`: useful after thesis closeout or after a named prerequisite changes.
- `current-review`: existing artifact or idea must be reviewed or resolved by
  the current task route before its thesis role is classified.
- `bug-redo`: the spike exposed a data, code, prompt, or methodology bug; fix the
  bug before interpreting the method result.

## Spike Contract

See `tasks/planning-notes.md` for current route choices and
`research/sys-landscape-datascience/worker-procedure.md` for the reusable
worker procedure. The notes below are retained as local delegation context.

Use one-turn worker delegation by default:

1. the lead chooses and defines one bounded experiment;
2. the lead delegates it once to a worktree worker;
3. the lead waits when blocked on the result;
4. if waiting times out, the lead inspects the worktree, `/tmp` artifacts, and
   running processes before messaging or closing the worker;
5. the lead reviews the artifacts and decides whether to merge, trash, leave a
   follow-up branch, or record a no-search-output/rejected verdict;
6. the lead updates this ledger, the toolbox audit, and task priorities.

Do not use interactive chat checkpoints as the default control path for these
spikes. If progress observability matters, require durable report and output
paths in the original packet.

Every spike packet should state:

- required cwd/worktree and a first command that prints `pwd`;
- shared dataset path and any row-count expectation relevant to the packet;
- idea, hypothesis, and what would count as positive, no-search-output, or
  inconclusive;
- allowed write scope;
- maximum local runtime and whether LICCA is out of scope;
- leakage guards, including grouped splits where prediction is involved;
- sanity checks, including null or permutation baselines when cheap;
- output artifact path or report path, written early enough that the lead can
  inspect partial work if the agent times out or must be shut down;
- stop conditions, especially `sys > 1`, stale dataset mismatch, or an
  implementation bug that invalidates the result.

Workers may write temporary artifacts in their worktree or under `/tmp`, but
their dataset source should be `experiments/sys-landscape/datascience/dataset/`
unless the packet
explicitly asks for a canonical refresh.

## Initial Idea Queue

| Slug | Idea | Type | Scope | Desired evidence | Current verdict | Evidence / next action |
| --- | --- | --- | --- | --- | --- | --- |
| `feature-block-regression` | Feature-block regression with ridge and random forest | method | Existing feature-block packet over random and endpoint rows | Grouped CV, random-to-endpoint prediction, null baseline | `no-search-output` | Cached in `research/sys-landscape-toolbox-audit.md`; reopen if refreshed feature tables predict endpoint rows well enough to define a candidate-proposer. |
| `regime-classification` | Endpoint-vs-random classification | method | Existing endpoint-vs-random classification packet | Whether non-provenance blocks separate endpoint rows from random rows better than null and metadata caveats | `no-search-output` | Current-contract pilot in `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`; use as supporting/caveat only, not as a candidate-proposer. |
| `endpoint-residualized-regression` | Endpoint residualized regression beyond metadata | method | Existing residualized-regression packet | Whether endpoint geometry/orbit/trajectory blocks add endpoint-only grouped-CV association beyond metadata | `no-search-output` | Repaired on 2026-06-03. Durable report: `experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_residual_summary.md`. Endpoint-side associations exist, but the row gives no candidate-proposer and no validated new `sys > 1` row. Use as supporting/caveat only. |
| `pca-cluster-anomaly` | PCA / clustering / anomaly scan over current feature blocks | method spike | Existing 282-row table and committed feature JSONL | A cluster or component rule specified before inspecting `sys`, or a bounded no-search-output result | `no-search-output` | Main commit `39039550` creates `experiments/sys-landscape/datascience/methods/pca-cluster-spike/` with script and report. |
| `supervised-alternatives` | Cheap supervised alternatives: lasso, elastic net, boosting, kNN | method spike | Current feature tables only | Whether standard extra models change `feature-block-regression` / endpoint-vs-random classification conclusions under the same grouped split policy | `no-search-output` | Main commit `5e8db378` creates `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/`; lead had to run the worker's script to produce the report. |
| `stat-sanity` | Null, permutation, and bootstrap uncertainty checks | sanity | Existing feature-block / endpoint-vs-random classification / residualized-regression outputs | Chance baseline and fold uncertainty for claimed associations or no-search-output results | `future` | Downgraded on 2026-06-03 because the only available source truth is scratch output in `/tmp/ds-stat-sanity-spike/summary.json`, not a committed script/report. The scratch check may be used only as non-load-bearing caveat context unless a repo-owned method packet is added later. |
| `exact-f64-spot-check` | Exact-vs-f64 spot checks for mathematical columns | sanity | Sampled rows from table-stage features | Detect whether a column implementation turns a true association into noise | `no-search-output` | Main commit `e8528963` creates `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/`; sampled checked columns showed only f64-scale drift. |
| `deep-latent-models` | Neural networks or deep latent models | method | Current 282-row dataset | Would need overfit controls and enough rows for flexible models | `rejected-low-voi` | Too small and too easy to overfit before thesis closeout; reopen only with much larger data. |
| `svm-supervised-baseline` | SVM regression/classification baseline | method spike | Current feature tables only | Whether a standard margin-based model changes the supervised random-to-endpoint prediction or endpoint-vs-random classification story under the same grouped split policy | `future` | Candidate for one optional small parallel wave. Skip if setup cost is not clearly lower than its thesis value; otherwise write source truth under `experiments/sys-landscape/datascience/methods/svm-supervised-baseline/`. |
| `interpretable-tail-rules` | Simple threshold/tree/interaction rule mining for high-`sys` tails | method spike | Current feature tables only | A candidate-proposer that suggests where to search next before inspecting forbidden inputs, or a bounded no-search-output result for simple interpretable rule classes | `future` | Candidate for one optional small parallel wave. Must not use endpoint labels, producer identity, or target leakage to define the final candidate rule. |
| `surrogate-guided-search` | Bayesian optimization / surrogate-guided search loop | search | Candidate generator plus exact evaluation budget | New high-sys candidates or clear comparison against random/local baselines | `future` | Reopen only with a bounded candidate space and compute budget approved by Jörn. |
| `geometric-feature-columns` | New symplectic/geometric feature columns from informal intuition | column | Table-stage additive columns | Computable definition, sanity check, and expected information gain | `future` | Split each proposed column into its own row before implementation. |
| `hko-positive-region-random-walk` | Random walk away from HKO2024 while staying in the `sys > 1` region | search / geometry probe | HKO2024 neighborhood, not the principal generic hostile-landscape table unless later promoted | How far positive examples persist after quotienting or renormalizing symmetry-group motion | `future` | Jörn idea recorded on 2026-06-03: run a random walk away from HKO2024, always staying in the `sys > 1` regime, possibly with renormalization to quotient out the `sys`-symmetry group so perturbation size does not mainly measure symmetry movement. Reopen only as HKO-neighborhood/future-work exploration unless a later packet promotes it to thesis-facing evidence. |

## Next Wave Queue

Prepared on 2026-05-02 from the current task state and current committed data.
This queue is a launch plan, not a claim that the future rows are valuable after
they run.

| Order | Slug | Shape | Why now | Dependency / stop condition | Expected terminal outcome |
| --- | --- | --- | --- | --- | --- |
| 1 | `endpoint-residualized-regression` | Narrow repair for existing packet | Completed on 2026-06-03. | Reopen only if endpoint data or feature packets change, or if someone derives a candidate-proposer without forbidden inputs. | `no-search-output`; supporting/caveat only. |
| 2 | `stat-sanity` | Source-truth downgrade | Completed on 2026-06-03. | Reopen only if a repo-owned script/report is added because thesis text needs null/permutation numbers. | `future`; not thesis-bearing. |
| 3a | `svm-supervised-baseline` | Optional small parallel probe | SVMs are a standard omitted-family caveat and may be cheap because the supervised table machinery already exists. | Run only after the serial row does not produce a positive follow-up. Stop if dependency/setup churn exceeds the value of an omitted-family row. | Negative/supporting report or explicit skipped-low-VOI row. |
| 3b | `interpretable-tail-rules` | Optional small parallel probe | This is the simplest interpretable row that could falsify the no-candidate-proposer story by producing a candidate-proposer. | Run only after the serial row does not produce a positive follow-up. Stop for any candidate-proposer and hand to a falsification/search packet before adding more unrelated methods. | Candidate-proposer plus follow-up packet, or a bounded no-search-output result for simple rules. |

Wave-level rule: do not seek new polytopes locally. If a row needs a larger
candidate pool, cluster-scale generation, or a new geometric feature definition,
record the LICCA/Jörn gate and leave the row `future` or `Jorn decision needed`.

## Process Lessons

Record only lessons that change future delegation or spike design.

| Date | Source | Lesson | Consequence |
| --- | --- | --- | --- |
| 2026-04-30 | Initial planning | Table generation from current producer caches is cheap enough for a lead wave setup but not free for every worker. | Historical rule: build one frozen temp dataset per wave and pass the path to workers. Superseded by the retained dataset rule below. |
| 2026-06-03 | Pipeline maintenance | The old temp-dataset rule made method inputs hard to share across worktrees and easy to lose after a session. | Supersede it with a retained dataset under `experiments/sys-landscape/datascience/dataset/`; compute row counts and hashes on demand with `fingerprint-dataset.py`. |
| 2026-04-30 | Initial planning | A useful spike must end in a verdict with evidence, not only a plot or script. | Worker packets require a report path with a reviewer-readable result header. |
| 2026-04-30 | `pca-cluster-anomaly` worker spike | One worker could implement and run a complete PCA/clustering spike from the packet, including row guards, non-provenance features, a permutation sanity check, and a readable verdict. | Full-method spikes are feasible when the dataset path, allowed write scope, and verdict vocabulary are explicit. |
| 2026-04-30 | `pca-cluster-anomaly` worker spike | The useful lifecycle is one-turn delegation: choose the experiment, define it, delegate, wait, inspect, then merge, trash, or leave a follow-up. Interactive checkpointing is not the default control path. | Future packets should require durable report/output paths for inspection, not chat-style progress management. |
| 2026-04-30 | `stat-sanity` worker spike | The corrected one-turn lifecycle worked: the worker returned a completed status and left a report for review; any JSON sidecar is auxiliary. | Keep default spike packets one-turn, require durable artifact paths, then inspect artifacts before the explicit terminal decision: merge, trash, or follow-up. |
| 2026-04-30 | Earlier pilot 1 `pca-cluster-anomaly` | Source-truth repair worked after timeout inspection and one corrective nudge. The worker produced script/report evidence, and the lead reran the script and committed `dc4f11a5` on `ds-pilot1-pca-cluster`. | Capability evidence only; this does not validate the reset contract. |
| 2026-04-30 | Earlier pilot 2 `supervised-alternatives` | The harder method pilot exposed a lifecycle failure: after nudging, the worker created a substantial script but did not run it or produce the required report. The lead ran the script and committed `c9b7cb77` on `ds-pilot2-supervised-alts`. | Worker packets need an early report/blocker note and a lead-repair disposition so partial code is not mistaken for completed source truth. This does not validate the reset contract. |
| 2026-04-30 | Process revision after pilots | The workflow requires an early report/blocker note, worker self-run proof, explicit `lead-repair` disposition, a normal long wait before inspection, and worker cleanup after disposition. | The early report is a worker-output requirement, not a lead-side polling ritual. Default local-pilot wait is `10` minutes unless the packet says otherwise. |
| 2026-04-30 | Earlier pilot 3 `exact-f64-spot-check` | Waiting long enough let the worker complete early report, script, final report, and self-run proof without lead repair. Manual early-report inspection was unnecessary and likely contributed to overdiagnosing earlier workers as stuck. | Process can work serially, but the required `summary.json` schema was an unjustified abstraction. The current contract is `report.md` plus ledger row; this pilot did not validate that final report-ledger workflow. |
| 2026-05-01 | `regime-classification` reset pilot | The legacy/full-history agent path failed: one invalid agent amended the integration branch instead of running the packet. The v1 no-context path then passed an exact-reply smoke, a required-cwd smoke, and the full worker packet using `fork_context=false`. | For this workflow, launch workers with v1 `spawn_agent` and `fork_context=false`; first run an exact-reply/cwd smoke after any agent-system change before trusting subagent output. |

### `regime-classification` Regime Classification

Disposition: current report-ledger serial pilot accepted and merged to the integration
branch.

Historical worker commands; current reruns should use
`experiments/sys-landscape/datascience/dataset/`:

```bash
uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P
uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P
```

Evidence:

- Branch: `ds-pilot-reset-regime-classification`.
- Integration commit: `3785cf9a` merges worker commit `be5e5fbb`.
- Report path:
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`.
- Summary path:
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_summary.md`.

Observation:

- Input guards passed for `282` polytope rows and `282` observation rows, with
  max `sys = 0.906316153431123` and zero `sys > 1` rows.
- Grouped CV used `212` groups over `170` random rows and `112` endpoint rows.
- Provenance metadata separated regimes perfectly for logistic regression and
  random forest.
- Non-provenance geometry/orbit blocks also separated current regimes above the
  null baseline: best logistic geometry/orbit block was `skeleton`
  (`balanced_accuracy ~= 0.8728`, `ROC AUC ~= 0.9223`), and best random-forest
  geometry/orbit block was `face_geometry` (`balanced_accuracy ~= 0.9275`,
  `ROC AUC ~= 0.9831`).
- The aggregate `orbit` block was stronger but mixes geometry, combinatorics,
  and search/KKT availability scalars.

Inference:

Endpoint-produced and random-produced rows are distinguishable in the current
table, including by non-provenance feature families, but this does not give a
candidate-proposer. The result does not find a `sys > 1` row and does not give
a rule for proposing candidates before inspecting `sys`, endpoint labels,
producer identity, or optimizer provenance.

Verdict: `no-search-output`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = medium`;
`thesis_use = supporting/caveat only`.

## Completed Spike Notes

### `pca-cluster-anomaly` PCA / Clustering / Anomaly Scan

Disposition: source-truth repair merged to `main`.

Historical worker command; current reruns should use
`experiments/sys-landscape/datascience/dataset/`:

```bash
uv run --script experiments/sys-landscape/datascience/methods/pca-cluster-spike/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr --out-dir experiments/sys-landscape/datascience/methods/pca-cluster-spike
```

Evidence:

- Branch: `ds-pilot1-pca-cluster`.
- Main commit: `39039550`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/pca-cluster-spike/report.md`.
- Historical auxiliary metadata sidecar was removed on 2026-06-04 because the
  report is source truth and no current consumer needs the JSON.

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

PCA and clustering see within-table structure, but the structure does not define
a candidate-proposer. The high-sys side mostly recovers existing endpoint
producers. A useful follow-up would need a sampling rule for a feature-space
region specified before inspecting `sys`, endpoint labels, dataset identity, or
optimizer provenance.

Verdict: `no-search-output`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = high`;
`thesis_use = supporting/caveat only`.

### `supervised-alternatives` Cheap Supervised Alternatives

Disposition: source-truth repair merged to `main`; process result was a partial
worker failure repaired by the lead.

Historical lead command; current reruns should use
`experiments/sys-landscape/datascience/dataset/`:

```bash
uv run --script experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr --permutations 20
```

Evidence:

- Branch: `ds-pilot2-supervised-alts`.
- Main commit: `5e8db378`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/REPORT.md`.
- Historical auxiliary metadata sidecar was removed on 2026-06-04 because the
  report is source truth and no current consumer needs the JSON.

Observation:

- Input guards passed for `282` polytope rows and `282` observation rows, with
  max `sys = 0.906316153431123` and zero `sys > 1` rows.
- Regression panel: lasso, elastic net, histogram gradient boosting, extra
  trees, and kNN.
- Claim-bearing feature matrices excluded target/capacity columns, raw arrays,
  ids, and observation provenance; the cleaner block also excluded orbit-search
  scalar columns.
- Best random-to-endpoint prediction among claim-bearing alternatives was
  histogram gradient boosting on `intrinsic_no_orbit_search`, with
  `R^2 ~= -2.8894`.
- Within-random and within-endpoint fits were positive (`R^2 ~= 0.8995` and
  `R^2 ~= 0.3268` for the best intrinsic blocks), but those do not give positive
  random-to-endpoint prediction.
- Endpoint-vs-random classification from intrinsic numeric features remained
  strong: best balanced accuracy `~= 0.9451`, ROC AUC `~= 0.9931`.

Inference:

Cheap supervised alternatives do not change the `feature-block-regression`
no-candidate-proposer story:
the load-bearing random-to-endpoint prediction surface remains strongly
negative even with flexible tree and kNN alternatives. Endpoint-vs-random
classification remains a table observation and not a rule for proposing new
high-`sys` candidates.

Verdict: `no-search-output`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = medium`;
`thesis_use = supporting/caveat only`.

### `stat-sanity` Null / Permutation / Bootstrap Sanity Checks

Disposition: downgraded on 2026-06-03 to non-load-bearing caveat evidence. The
script, report, and run outputs are not committed under the current
report-ledger contract, so do not quote the numerical null/permutation results
as thesis source truth unless a repo-owned method packet is added later.

Historical worker command; current reruns should use
`experiments/sys-landscape/datascience/dataset/`:

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

Scratch inference:

The null checks make the within-row-source associations harder to dismiss as
pure chance, but they do not produce a candidate-proposer. The
random-to-endpoint prediction result can beat a weak permuted-label baseline
while still being unusable for endpoint prediction, because its actual endpoint
`R^2` is strongly negative. Classification confirms that the table separates
producer sources, including with non-metadata blocks, but this does not identify
where to sample for new `sys > 1` rows.

Verdict: `future`.

Thesis use: do not use as a claim-bearing method row. It may explain why
within-row-source associations were not promoted into a candidate-proposer, but
the main no-candidate-proposer result must rest on committed method reports and
the toolbox audit rather than these scratch numbers.

### `exact-f64-spot-check` Exact-vs-f64 Spot Check

Disposition: source-truth branch merged to `main`.

Historical worker command; current reruns should use
`experiments/sys-landscape/datascience/dataset/`:

```bash
uv run --script experiments/sys-landscape/datascience/methods/exact-f64-spot-check/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr
```

Evidence:

- Branch: `ds-pilot3-exact-f64`.
- Main commit: `e8528963`.
- Report path after merge:
  `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/report.md`.
- Historical auxiliary metadata sidecar was removed on 2026-06-04 because the
  report is source truth and no current consumer needs the JSON.

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

Verdict: `no-search-output`.

Qualifiers: `evidence_strength = medium`; `implementation_trust = high`;
`thesis_use = supporting/caveat only`.
