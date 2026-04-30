<!--
Purpose: hostile sys-landscape and novel-sys roadmap.
Context: main thesis story around negative search evidence and landscape
hostility.
-->

# Hostile Landscape Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-30.
- Source surfaces: `research/sys-landscape.md`,
  `research/sys-landscape-datascience/`, `research/sys-landscape-toolbox-audit.md`,
  `experiments/sys-landscape/`, `tasks/verify-thesis-done.md`.
- Refresh when: retained hostile-landscape wording obligations or endpoint datasets
  change.

## Steering Cache

- [accepted 2026-04-15] Hostile sys-search landscape is part of the thesis
  spine.
  Source: Kai/Jorn state in legacy tracker, now routed through
  `research/INDEX.md` and this bundle.
  Why it matters: retained negative-search evidence must be compressed for
  thesis, not treated as optional infrastructure.
- [accepted 2026-04-24] Standard data-science methods may be used to support
  the hostile-landscape wording, but do not invent new method programs during
  closeout.
  Source: legacy tracker finish-mode wording.
  Why it matters: prevents open-ended ML/statistics exploration from expanding
  scope.
- [accepted 2026-04-30] Finish for the data-science strand means idea
  exhaustion: every standard or plausible data, column, method, or search idea
  should have a tried, rejected, deferred, falsified, or escalated verdict.
  Source: Jorn steering in data-science closeout discussion.
  Why it matters: the active artifact is now an idea ledger and spike process,
  not only a static method audit.
- [accepted 2026-04-24] Visualization as negative mathematical exploration is a
  Kai-discussed standalone result if it can be included from current evidence
  with low incremental work.
  Source: finish-mode result scope.
  Why it matters: write as current-state result, not improvement program.

## Data-Science Submission Blockers

Use this section before running more data-science agents. The data-science
component no longer blocks hostile-landscape writing exactly when all seven
blockers below are closed. If any blocker is open, cite the blocker name and work
only on that blocker.

1. **Coverage blocker**
   - Open while there is no finite before-submission list of standard or
     high-value data-science method, feature/column, transformation, sanity-check,
     and search-follow-up ideas.
   - Closed when `research/sys-landscape-datascience/idea-ledger.md` lists the
     before-submission idea set, or explicitly marks an idea family as future,
     rejected, or out of scope for thesis closeout.
2. **Verdict blocker**
   - Open while any before-submission idea lacks a status that tells a future
     agent whether to run it, redo it, cite it, defer it, or stop.
   - Closed when each before-submission idea is marked as one of: found positive
     and escalated, conjectured-positive awaiting follow-up, falsified-positive,
     negative, rejected-low-VOI, future, or bug-redo with a named repair step.
3. **Positive-follow-up blocker**
   - Open while any conjectured-positive pattern has not been tested as a search
     route, escalated to Jorn, or explicitly moved to future work.
   - Closed when no untested conjectured-positive remains in the
     before-submission set.
4. **Evidence blocker**
   - Open while any tried, negative, falsified, or positive result that affects
     the hostile-landscape story points only to `/tmp`, chat, a deleted worktree,
     or unsupported ledger prose.
   - Closed when every such result points to committed source truth: experiment
     code, generated artifact, report, or another repo-owned evidence path plus
     the command/provenance needed to audit it.
5. **Experiment-validity blocker**
   - Open while a tried result has source truth but lacks the checks needed for
     its verdict type.
   - Closed when each tried result used for closure records the relevant checks:
     reproducibility command, input/provenance and seed/config; row-count and
     schema sanity; freshness check or stale-data caveat; success/failure metric;
     baseline or null comparison where the method is statistical; grouped or
     lineage split where prediction is involved; fold/bootstrap/permutation
     uncertainty when the claim relies on finite-sample signal; exact-vs-f64 or
     tolerance checks when a mathematical column is claim-bearing; and lead or
     reviewer confirmation that the evidence supports the stated verdict and not
     a stronger one.
   - Reproducibility belongs here, not in the Evidence blocker: Evidence asks
     whether repo-owned source material exists; Experiment validity asks whether
     a future agent can rerun or audit that material and trust the stated result.
6. **Caveat blocker**
   - Open while the data, feature/column, method-class, compute-budget, density,
     or complexity limits of the negative result are implicit.
   - Closed when the audit or linked research note states the dataset/table
     snapshot, feature/column scope, included method classes, excluded/deferred
     method classes, runtime/search budget, and the caveat that the result is not
     a density theorem or impossibility theorem.
7. **Thesis-use blocker**
   - Open while the thesis-facing status of closed results is unknown.
   - Closed when `research/sys-landscape-toolbox-audit.md` or this bundle marks
     every closed data-science result as cite in thesis, supporting/caveat only,
     omit before submission, or future work. This includes explicit decisions for
     `M012` regime classification and `M013` residualized endpoint regression.

Current known open blockers:

- Evidence blocker: `DS-I004` and `DS-I006` were recorded from scratch worker
  outputs; their code/report source truth was not preserved in a committed
  experiment or evidence path.
- Thesis-use blocker: `M012` and `M013` still need Jorn's cite/omit/future
  decision.

## Data-Science Process Maturity

Use this section before scaling to many subagents. The data-science process is
not ready to run to completion until it reaches stage 5.

1. **Proposed**: candidate process exists.
2. **Documented enough to test**: a pilot agent can follow the process without
   reconstructing chat.
3. **Tested**: one or more pilots run through the documented process.
4. **Revised and settled**: pilot lessons are folded in; this becomes the default
   process. Reaching this stage requires at least one source-truth/lifecycle
   pilot and one harder method pilot unless Jorn explicitly closes the second
   requirement.
5. **Ready to scale / run to completion**: docs are in the repo, the current
   dashboard shows open/closed blockers, packet and review templates exist,
   future lead agents can resume without this chat, and scaling will not create
   ambiguous scratch.
6. **Maintained during execution**: each closed, failed, deferred, or bug-redo
   subexperiment updates the dashboard and evidence surfaces as part of the
   process.

Current maturity: stage 2, **Documented enough to test**.

Do not run a batch of data-science subagents until stage 4 is reached. A single
pilot is allowed at stage 2 only after Jorn accepts the workflow surface, and
only if the pilot names which submission blocker it tests and what repo-owned
source truth it must leave behind.

### Data-Science Subexperiment Workflow

This workflow is the task-facing convention for the data-science idea
exhaustion loop. It exists so future lead agents can close rows without
reconstructing chat history.

Authority split:

- This bundle owns the blocker list, process maturity stage, worker/reviewer
  checklist, and scale/no-scale decision.
- `research/sys-landscape-datascience/idea-ledger.md` owns the idea queue,
  per-idea verdicts, evidence links, and process lessons.
- `research/sys-landscape-toolbox-audit.md` owns thesis-facing method rows after
  the underlying evidence has been checked.

Lead loop for one subexperiment:

1. Pick one idea row and name which submission blocker the run is intended to
   close or advance.
2. Freeze the input dataset for the wave and record command, paths, row counts,
   max `sys`, and `sys > 1` count.
3. Write a worker packet with the fields below.
4. Delegate once to a worker in an isolated worktree; do not use interactive
   checkpoints as the default control path.
5. Wait when blocked on the result. If `wait_agent` times out, inspect the
   worktree, durable report path, scratch output paths, and running processes
   before deciding whether to message, wait again, or close the worker.
6. Review the worker's repo-owned artifacts before accepting any claim.
7. Choose one disposition: merge/promote, reject/trash, leave follow-up branch,
   `bug-redo`, `future`, `rejected-low-voi`, or `positive-escalate`.
8. Update the idea ledger, toolbox audit if thesis-facing, and this task bundle
   before starting the next subexperiment.

Required worker-packet fields:

- `idea_id`, blocker target, and why the row is before-submission or not.
- Required cwd/worktree and a first command that prints `pwd`.
- Frozen dataset path, producer command, expected row counts, expected max
  `sys`, and expected `sys > 1` count.
- Question, hypothesis, and what counts as positive, conjectured-positive,
  falsified-positive, negative, inconclusive, or bug-redo for this row.
- Allowed write scope and required repo-owned evidence path. `/tmp` may be used
  for scratch, but no terminal verdict may rely only on `/tmp`, chat, or a
  deleted worktree.
- Maximum local runtime, whether LICCA is out of scope, and stop conditions.
- Leakage/provenance guards, including grouped or lineage splits where
  prediction is involved.
- Statistical checks when relevant: baseline/null, fold/bootstrap/permutation
  uncertainty, and finite-sample caveat.
- Numerical checks when relevant: exact-vs-f64 comparison, tolerance, row/schema
  guards, and stale-data check.
- Required report sections: command/provenance, observation, inference, verdict,
  evidence strength, implementation trust, caveats, thesis-use proposal, and
  reopen trigger.

Required result qualifiers:

- `verdict`: one of `positive-escalate`, `conjectured-positive`,
  `falsified-positive`, `negative`, `rejected-low-voi`, `future`, or `bug-redo`.
- `evidence_strength`: `high`, `medium`, or `low`, measuring statistical or
  experimental information content under the stated data, metric, and budget.
- `implementation_trust`: `high`, `medium`, or `low`, measuring whether the
  code/data/run likely tested the intended question.
- `thesis_use`: `cite in thesis`, `supporting/caveat only`, `omit before
  submission`, `future work`, or `Jorn decision needed`.
- `caveat`: dataset/table snapshot, feature scope, method class, runtime/search
  budget, density limit, complexity limit, and any stale-data or overfit limit.

Closure rules:

- A `positive-escalate` result stops the wave and goes to Jorn before more
  scaling.
- A `conjectured-positive` row does not close the Positive-follow-up blocker
  until a search/falsification follow-up is run, escalated to Jorn, or moved to
  future work.
- A `negative` row with `implementation_trust = low` does not close a
  before-submission blocker; mark `bug-redo`, `future`, or `omit before
  submission`.
- A `negative` row with `evidence_strength = low` may close only a narrowly
  caveated row, and the caveat must appear in the idea ledger or toolbox audit.
- A tried result cannot close the Evidence blocker unless its code, report, or
  generated artifact is repo-owned and the command/provenance is recorded.
- A tried result cannot close the Experiment-validity blocker unless a lead or
  reviewer records that the checks match the verdict and that the wording does
  not overclaim.

Reviewer checklist:

- Evidence path exists in the repo or in a branch intended for merge.
- Commands/provenance are sufficient for a future agent to rerun or audit.
- Dataset row counts, schema, freshness, max `sys`, and `sys > 1` count match the
  packet or explain the mismatch.
- Leakage, provenance, grouped splits, and metadata restrictions match the
  question.
- Statistical or numerical checks match the claim type.
- Observation, inference, verdict, evidence strength, implementation trust,
  caveat, thesis use, and reopen trigger are all recorded.

Stage-advancement checks:

- Stage 2 is reached when this workflow, the worker-packet fields, result
  qualifiers, closure rules, and reviewer checklist are documented in this
  bundle and linked from the idea ledger.
- Stage 3 is reached after one pilot follows the documented workflow and leaves
  repo-owned evidence, even if the pilot ends as `bug-redo` or rejected.
- Stage 4 is reached only after pilot lessons are folded back into this
  workflow and the next packet can be launched without chat reconstruction.
- Stage 5 is reached only when the before-submission dashboard shows every open
  row's blocker target, required evidence path, current status, review state,
  and next action.

Proposed stage-3 pilot 1 packet:

- `idea_id`: `DS-I004`.
- Goal: rerun the PCA / clustering / anomaly scan as a source-truth repair, not
  as a new interpretation program.
- Blocker target: Evidence blocker first; Experiment-validity blocker only if
  the report records the required checks and a lead/reviewer accepts the wording.
- Why this pilot: it tests the exact failure mode that caused the current
  process repair, namely a useful spike whose code/report source truth was not
  preserved in a repo-owned path.
- Scope limit: this pilot intentionally uses a method family where a previous
  agent already got useful work done. It tests whether the workflow preserves
  source truth and supports review, not whether agents can handle a harder or
  less pre-shaped method experiment.
- Required evidence path:
  `experiments/sys-landscape/datascience/methods/pca-cluster-spike/`.
- Required artifacts: analysis script, markdown report, machine-readable summary
  such as `summary.json`, and commands sufficient to rerun against a frozen
  dataset path.
- Dataset packet: use one frozen temp table rebuilt from committed producer
  caches, and record command, row counts, max `sys`, and `sys > 1` count in the
  report.
- Acceptance check: the report must separate observation from inference; exclude
  metadata/provenance features for the claimed scan; state whether any cluster,
  component, or anomaly rule gives a non-post-hoc candidate generator; record
  `verdict`, `evidence_strength`, `implementation_trust`, `thesis_use`, caveat,
  and reopen trigger.
- Reject/trash condition: if the worker leaves only `/tmp` outputs, omits the
  report, cannot reproduce row counts, uses provenance columns for the claimed
  signal, or states a stronger result than the artifacts support.
- Scale decision after pilot: do not move to stage 4 until the lead records what
  the pilot taught about packet wording, artifact paths, review cost, and whether
  one-turn delegation produced an auditable result.

Required later pilot before scaling:

- Run a second pilot after pilot 1 succeeds or fails cleanly.
- The second pilot must test a harder method experiment or a less pre-shaped row
  than `DS-I004`.
- Its goal is to measure whether the workflow still works when the worker must
  make more local method-design choices, not only reconstruct a known useful
  spike.
- Do not mark the process stage 4 or run a batch unless this second-pilot
  requirement is closed by the pilot result or by an explicit Jorn decision.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Hostile-landscape retained-claim compression | `[map-input]` | mainline thesis | agent prep then Jorn | Draft the bounded claim surface: current evidence found no new transferable `sys > 1` regime beyond pentagon-pentagon, while seed counts are too small for a density or brute-force-impossibility claim. | `research/sys-landscape.md`, `research/sys-landscape-toolbox-audit.md` |
| Method-ledger/audit population | `[map-input]` | mainline thesis | agents | Populate current packet evidence plus explicit skipped/deferred standard-toolbox rows into the hostile-landscape audit without opening new methods. | `research/sys-landscape-datascience/`, `research/sys-landscape-toolbox-audit.md` |
| Data-science idea exhaustion loop | `[active]` | mainline thesis | lead agent with subagents, Jorn gates | Close the blockers in "Data-Science Submission Blockers"; before running a worker, name which blocker the worker closes and what committed source truth will remain. | `research/sys-landscape-datascience/idea-ledger.md`, `experiments/sys-landscape/datascience/` |
| Remaining method packet status | `[Jorn]` | map input | Jorn | Decide thesis-facing status of `M012` regime classification and `M013` residualized endpoint regression after the artifact-backed audit rows exist. | `research/sys-landscape-datascience/method-ledger.md` |
| LICCA endpoint refresh | `[future]` | future/follow-up | external compute | Leave pending unless results are already available with low integration cost. | legacy LICCA rows |
| Visualization negative exploration | `[Jorn]` | contingent during writing | Jorn | Decide during TOC work whether visualization is standalone thesis material or only supporting/future material if figures become useful. | `research/visualization.md`, `research/INDEX.md` |
| Pentagon rotation formula | `[future]` | contingent during writing | Jorn/math | Include only status-level current finding unless proof/CAS write-up becomes free. | `research/sys-landscape.md`, `formal/sys-landscape/pentagon-rotation-formula.tex` |

## Agent Cache

- [fresh 2026-04-24] Strong completed signal: feature regression/local-maxima
  pattern search landed and found endpoint-side features useful within random
  data but weak for transfer.
  Refresh by: reading `research/sys-landscape-datascience/method-ledger.md`.
- [fresh 2026-04-24] Rotated regular products found only the 5x5 at 18 degrees
  achieving sys>1 among tested grids; larger mixed pairs are not a current
  obligation.
  Refresh by: checking `experiments/sys-landscape/rotated-regular-products/`.
- [fresh 2026-04-24] Witness-oracle and reduced-model ideas are future unless
  needed to explain retained claims.
  Refresh by: checking `research/sys-landscape.md` and thesis wording.
- [fresh 2026-04-25] `research/sys-landscape-datascience/method-ledger.md`
  already caches attempted methods `M001` through `M013`; `M012` regime
  classification and `M013` residualized endpoint regression are present but
  `thesis_use = undecided`.
  Refresh by: reading the method ledger and checking the cited analyzers under
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/`.
- [fresh 2026-04-30] `research/sys-landscape-toolbox-audit.md` has Phase-2
  rows for artifact-backed methods and named skipped/deferred families through
  Bayesian optimization. Remaining work is not row population from scratch; it
  is reconciliation with the active idea ledger, summary artifacts, and Jorn's
  status decisions for M012/M013.
  Refresh by: reading `research/sys-landscape-toolbox-audit.md`.
- [fresh 2026-04-30] `research/sys-landscape-datascience/idea-ledger.md`
  owns the active spike queue and finish-by-idea-exhaustion process. Current
  planning snapshot rebuilds `282` polytope rows and `282` observation rows from
  committed producer caches, with max `sys ~= 0.906316153431123` and zero
  `sys > 1` rows.
  Refresh by: rebuilding a temp table with
  `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir <tmp>`.
- [fresh 2026-04-25] Formal sys-landscape notes cite older random-sample JSONL
  paths. Check those paths only if the formal notes become a thesis or
  provenance source; do not run a broad data-refresh pass by default.
  Refresh by: checking `formal/sys-landscape/random-sample.tex`,
  `formal/sys-landscape/random-product-sample.tex`, and current producer
  README files.

## Pruned / Stale

- [stale 2026-04-24] Broad systematic landscape analysis remains future work.
  Thesis closeout needs retained-claim compression, not a new search program.
