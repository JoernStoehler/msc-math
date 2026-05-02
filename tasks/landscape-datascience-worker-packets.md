<!--
Purpose: reusable worker-packet template and pilot example for sys-landscape
data-science subexperiments.
Context: tasks/landscape.md owns the workflow and readiness gates; this file
keeps the concrete prompt surface short enough to copy into a worker.
-->

# Data-Science Worker Packets

Use this file when a lead research data-science agent turns one row from
`research/sys-landscape-datascience/idea-ledger.md` into a bounded worker
assignment.

The lead owns objective construction. The worker owns experiment execution
inside the objective. The lead reviews and integrates the result.

## Lead Construction Checklist

Before spawning a worker, fill these fields:

- semantic idea slug and copied ledger row.
- Worktree path and branch name.
- Fresh base dataset snapshot:
  - path;
  - producer command;
  - polytope and observation row counts;
  - max `sys`;
  - number of rows with `sys > 1`.
- Objective paragraph: what question this row answers inside the shared
  data-science blocker.
- Method-local freedom: which filtering, feature selection, split policy, model
  choice, or sanity-check choices the worker may make and must record.
- Binding constraints: allowed write paths, runtime budget, no `/tmp`-only
  source truth, no required JSON sidecar, and stop conditions.
- Verdict interpretation: what counts as `positive-escalate`,
  `conjectured-positive`, `falsified-positive`, `negative`, `bug-redo`,
  `rejected-low-voi`, or `future` for this row.
- Review command or check the lead expects to run after the worker returns.

Do not ask the worker to decide the thesis blocker. All rows serve the same
shared blocker: finish the standard/plausible data-science strand so thesis
submission is not blocked by untried or untrusted methods.

## Generic Worker Prompt

Load `$data-science-subexperiment` for the generic workflow and prompt skeleton.
This file only instantiates that workflow for the current sys-landscape queue.

## Next-Wave Trial Architecture

Use this architecture for the 2026-05-02 Objective A/B wave unless Jorn changes
the research priority.

Lead setup:

- Build one fresh temp dataset snapshot with
  `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir <tmp>`.
- Record the dataset path, command, polytope rows, observation rows, max `sys`,
  and `sys > 1` count in every packet.
- Use v1 subagents with `fork_context=false` after the exact-reply and
  required-cwd smokes from `$data-science-subexperiment` pass.

Workspace and folder rule:

- Each worker gets an isolated git worktree and, for new methods, a separate
  method folder under
  `experiments/sys-landscape/datascience/methods/<slug>/`.
- Do not refactor shared helpers during this wave. For new method folders,
  prefer copying the small loader/check code needed for the local method over
  creating a shared import surface. If a worker must reuse an existing helper,
  the report must name that coupling.
- Use `/tmp` only for scratch inputs or outputs. Terminal verdicts need
  repo-owned code/report/source truth.

Execution shape:

1. Run `endpoint-residualized-regression` first because it repairs an existing
   undecided packet.
2. If that row yields `conjectured-positive`, stop unrelated method work and
   write a falsification/search packet for the candidate rule.
3. If it is negative or future-only, run `stat-sanity` source-truth repair or
   downgrade next.
4. Optionally run one small parallel probe with at most two independent folders:
   `svm-supervised-baseline` and `interpretable-tail-rules`.
5. Stop local scaling when the row needs new polytopes, cluster-scale compute,
   or a new Jorn-owned feature definition.

Review outcome:

- Content success is either a `positive-escalate`/`conjectured-positive` search
  rule or a negative result with explicit method, data, runtime, and leakage
  caveats.
- Process success is a report-ledger result whose review does not require a new
  schema, shared helper refactor, or chat reconstruction.

## Next-Wave Packet Deltas

These deltas instantiate the generic worker prompt. The lead still fills the
worktree path and fresh dataset snapshot.

### `endpoint-residualized-regression`

- Approved surface: source-truth repair/review for the existing
  `feature-pattern-search/analyze_residual.py` packet.
- Objective: decide whether endpoint feature blocks add grouped-CV signal beyond
  metadata strongly enough for a thesis-facing claim, only a caveat, or omission.
- Allowed write scope:
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/`,
  especially `endpoint-residualized-regression-report.md` and any narrow repair
  to `analyze_residual.py`.
- Method-local freedom: inspect, run, and narrowly repair the existing analyzer;
  add baseline/null or fold-uncertainty summaries only if needed for the verdict.
- Positive meaning: a residual feature pattern gives a concrete label-free
  sampling/search rule or a falsifiable conjectured-positive follow-up.
- Negative meaning: endpoint residual signal is absent, too weak, or not
  transferable/actionable after metadata and grouped split guards.
- Review check: report records endpoint row counts, group policy, metadata
  baseline, additive feature metrics, missing uncertainty caveats, and thesis-use
  proposal.

### `stat-sanity`

- Approved surface: source-truth repair or downgrade for the promoted
  null/permutation/bootstrap sanity result.
- Objective: either recreate enough repo-owned script/report evidence to support
  the current caveated negative sanity claim, or explicitly mark the result
  non-load-bearing before thesis use.
- Allowed write scope:
  `experiments/sys-landscape/datascience/methods/stat-sanity/` and ledger/audit
  rows after lead review.
- Method-local freedom: choose the cheapest sanity panel that addresses the
  load-bearing transfer and regime-classification caveats; do not rebuild a
  broad statistics program.
- Positive meaning: only a sanity check exposing a real bug or actionable
  overlooked signal; otherwise this is a source-truth repair row.
- Negative meaning: null/permutation/fold evidence still supports only the
  existing bounded negative/caveat story.
- Review check: report names the exact previous claim it repairs or downgrades,
  the commands run, and whether the toolbox audit can keep citing it.

### `svm-supervised-baseline`

- Approved surface: optional omitted-family supervised baseline.
- Objective: test whether SVM regression/classification changes the current
  supervised feature-table verdict under the same grouped split and transfer
  guards as the existing feature-block and supervised-alternatives packets.
- Allowed write scope:
  `experiments/sys-landscape/datascience/methods/svm-supervised-baseline/`.
- Method-local freedom: choose a small linear/RBF SVM panel, scaling, and cheap
  parameter grid. Avoid broad tuning.
- Positive meaning: SVMs produce an actionable feature-space rule or materially
  improve random-to-endpoint transfer beyond existing models.
- Negative meaning: SVMs do not change the transfer/search-usefulness story.
- Review check: compare against the existing supervised-alternatives report and
  state whether the audit row remains an omitted-family caveat or becomes an
  attempted negative.

### `interpretable-tail-rules`

- Approved surface: optional simple pattern-finding spike.
- Objective: look for simple, label-free rules over non-provenance features that
  isolate high-`sys` tails and could guide fresh candidate generation.
- Allowed write scope:
  `experiments/sys-landscape/datascience/methods/interpretable-tail-rules/`.
- Method-local freedom: use shallow trees, one/two-feature thresholds, sparse
  interactions, or monotone/rule-list scans. Keep the complexity small enough
  that a human can state the rule in one paragraph.
- Positive meaning: a rule can be tested on new candidates without using `sys`,
  endpoint labels, producer identity, or optimizer provenance.
- Negative meaning: simple interpretable rules either recover known producer
  structure, overfit the 282-row table, or fail against grouped/null checks.
- Review check: report separates the discovered rule, leakage guard, null or
  grouped validation, and whether a follow-up falsification/search packet is
  warranted.

## Candidate Serial-Pilot Packet: `regime-classification`

This is a filled example for the reset-contract pilot. Refresh the dataset path
and worktree path before use.

```text
Required cwd/worktree: /workspaces/msc-math/.codex/worktrees/ds-pilot-reset-regime-classification

Use this worktree for all commands and edits. Do not edit `main` or the root
checkout.

If you make tracked edits outside /workspaces/msc-math/.codex/worktrees/ds-pilot-reset-regime-classification,
revert only your own edits when `git diff -- <path>` shows only those edits. If
you detect pre-existing or ambiguous tracked edits outside that worktree, stop
and report the exact files before continuing.

You are not alone in the codebase; do not revert or overwrite changes made by
others.

Approved surface:
Reset-contract serial pilot for data-science idea `regime-classification`, regime
classification.

Objective:
Turn the existing regime-classification attempt into a reviewed source-truth
report under the reset contract. The question is whether endpoint-vs-random
regime classification from the current feature tables gives a thesis-usable
observation, a caveated supporting-only observation, or a result to omit before
submission. The report must distinguish provenance/metadata separation from
non-provenance geometric or orbit-feature separation.

Why this row belongs in the shared blocker:
Regime classification is a standard supervised-learning diagnostic and already
exists in the repo as `M012`, but its thesis role is undecided. The data-science
strand is not handoffable until this row has a reviewed report/ledger
disposition or is explicitly deferred.

Base dataset snapshot:
- Path: <FRESH_DATASET_DIR>
- Producer command: cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir <FRESH_DATASET_DIR>
- Expected polytope rows: <FILL>
- Expected observation rows: <FILL>
- Expected max sys: <FILL>
- Expected sys > 1 rows: <FILL>

Your method-local choices:
You may run, repair, or wrap
`experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py`.
You may choose feature-block groupings or exclusions needed to separate
provenance-heavy metadata from non-provenance feature blocks. Record all row
filters, feature exclusions, grouped split choices, and metric choices in the
report. If the existing script already answers the objective after inspection,
prefer a narrow report repair over rewriting the experiment.

Binding constraints:
- First command must print `pwd`.
- Allowed write scope:
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/`
  and this report path:
  `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`.
- Create the report or a blocker note before implementing method changes.
- Final source truth must be repo-owned: command run, dataset snapshot,
  filters/subsets, any figure or table outputs, and the report.
- Do not make `summary.json` required. Create machine-readable metadata only if
  a repo-owned checker or follow-up script consumes it.
- Do not rely on `/tmp`, chat, or a deleted worktree for a terminal verdict.
- Runtime budget: 10 minutes local wall time after dependencies are available.
  LICCA: out of scope.
- Stop immediately and report if row counts or max `sys` disagree with the
  packet, if a leakage bug invalidates the classification, or if the run finds a
  `sys > 1` row.

Report contract:
Write
`experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`.
Start with the required header from the generic template.

Verdict interpretation for this row:
- `positive-escalate`: only if the work finds an actual `sys > 1` row or a
  direct actionable search rule.
- `conjectured-positive`: a non-provenance classification pattern yields a
  concrete sampling/search rule that can be tested without using labels or
  producer identity.
- `negative`: classification may separate regimes, but does not yield an
  actionable search rule and is at most supporting/caveat evidence.
- `bug-redo`: stale data, leakage, script failure, or unclear feature provenance
  prevents interpreting the result.
- `future` or `rejected-low-voi`: use only if the method cannot be made
  thesis-useful within the local budget, with a concrete reopen trigger.

Success check:
The report must state whether the claim-bearing observation uses metadata,
non-provenance features, or both; give grouped-CV metrics against a null or
baseline where available; and propose one of `cite in thesis`,
`supporting/caveat only`, `omit before submission`, `future work`, or
`Jorn decision needed`.

Final response:
State the files changed, the command run, the verdict, and any missing success
signals. Do not claim the row is closed; the lead owns review and integration.
```
