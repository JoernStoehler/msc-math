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

- `idea_id` and copied ledger row.
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

```text
Required cwd/worktree: <ABSOLUTE_WORKTREE_PATH>

Use this worktree for all commands and edits. Do not edit `main` or the root
checkout.

If you make tracked edits outside <ABSOLUTE_WORKTREE_PATH>, revert only your own
edits when `git diff -- <path>` shows only those edits. If you detect
pre-existing or ambiguous tracked edits outside <ABSOLUTE_WORKTREE_PATH>, stop
and report the exact files before continuing.

You are not alone in the codebase; do not revert or overwrite changes made by
others.

Approved surface:
<one sentence naming the data-science subexperiment group and the selected
idea_id>

Objective:
<lead-written objective that turns the ledger row into a concrete experiment>

Why this row belongs in the shared blocker:
<short explanation of why this standard/plausible method, sanity check, column,
or search idea must be tried, rejected, or deferred before the data-science
strand stops blocking thesis submission>

Base dataset snapshot:
- Path: <DATASET_DIR>
- Producer command: <COMMAND>
- Expected polytope rows: <N>
- Expected observation rows: <N>
- Expected max sys: <VALUE>
- Expected sys > 1 rows: <N>

Your method-local choices:
You may choose row filters, feature subsets, train/test or grouped split policy,
model parameters, plots, and sanity checks that fit the objective. Record each
choice in the report, including why it answers the objective. If a choice would
change the objective, stop and report the needed change instead of silently
doing a different experiment.

Binding constraints:
- First command must print `pwd`.
- Stay within <ALLOWED_WRITE_SCOPE>.
- Create <EVIDENCE_DIR>/report.md or a blocker note before implementing the
  full method.
- Final source truth must be repo-owned: code/script if needed, command run,
  dataset snapshot, filters/subsets, generated outputs if any, and report.md.
- Do not make `summary.json` required. Create machine-readable metadata only if
  a repo-owned checker or follow-up script consumes it.
- Do not rely on `/tmp`, chat, or a deleted worktree for a terminal verdict.
- Do not edit tracked `.jsonl` outputs unless this packet explicitly asks for a
  canonical refresh.
- Runtime budget: <LOCAL_RUNTIME_BUDGET>. LICCA: <IN_SCOPE_OR_OUT_OF_SCOPE>.
- Stop immediately and report if you find `sys > 1`, a stale dataset mismatch,
  a leakage bug that invalidates the experiment, or an ambiguity that makes the
  objective impossible to test.

Report contract:
Write <EVIDENCE_DIR>/report.md. Start with this header:

Status: draft | blocked | complete
Idea ID:
Objective:
Base dataset snapshot:
Dataset filtering/subsetting:
Command run:
Verdict:
Evidence strength:
Implementation trust:
Thesis use:
Caveat:
Reopen trigger:
Evidence paths:

After the header, include sections for:
- observations;
- inference;
- checks run;
- failure modes and caveats;
- whether this does or does not help find `sys > 1`.

Verdict vocabulary:
Use exactly one of `positive-escalate`, `conjectured-positive`,
`falsified-positive`, `negative`, `rejected-low-voi`, `future`, or `bug-redo`.

Success check:
The result is complete only if the report exists, names the command you ran,
records dataset filters/subsets, separates observation from inference, and gives
a verdict with evidence strength, implementation trust, thesis use, caveat, and
reopen trigger.

Final response:
State the files changed, the command run, the verdict, and any missing success
signals. Do not claim the row is closed; the lead owns review and integration.
```

## Candidate Serial-Pilot Packet: `DS-I002`

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
Reset-contract serial pilot for data-science idea `DS-I002`, regime
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
