---
name: data-science-subexperiment
description: "Use when leading, prompting, running, reviewing, or integrating delegated data-science subexperiments with worker agents: turn a semantic ledger slug into a bounded objective, delegate execution in a worktree, require report-ledger source truth, classify verdicts, and update process/readiness state."
---

# Data-Science Subexperiment

Use this skill for a lead data-science agent workflow. Combine with
`$subagent-delegation` before spawning workers, `$harness-engineering` when
changing prompt material, `$experiment-conventions` / `$python-conventions` for
experiment code, and `$dataset-conventions` when dataset freshness or generated
tables matter.

## Core Split

The lead owns objective construction, review, and integration. The worker owns
experiment execution inside the objective.

Do not ask the worker to decide the thesis or project blocker from scratch. The
lead gives the selected idea, why it matters, what outcome would be useful, and
what source truth must remain. The worker may choose method-local details such
as filters, feature subsets, split policy, model parameters, and sanity checks
when those choices stay inside the objective and are recorded in the report.

## Lead Loop

1. Select one semantic ledger slug or method row. Do not bundle unrelated methods.
2. Freeze or identify the base dataset snapshot for the wave:
   path, producer command, row counts, max target value, and target-threshold
   count.
3. Convert the row into a bounded objective:
   question, standard/plausible-method rationale, positive/negative/bug
   meanings, allowed write scope, runtime budget, and stop conditions.
4. Write one worker packet. Use an isolated worktree and name the required cwd.
5. Delegate once using the v1 subagent path with `fork_context=false`. Do not
   use full-history forks for worker packets: inherited context can cause the
   worker to pursue the lead's process task instead of the assigned
   subexperiment.
6. Do not use interactive checkpointing as the default control
   path.
7. Wait the expected runtime before inspecting. Poll early artifacts only after a
   timeout or during review.
8. Review repo-owned artifacts before accepting any claim.
9. Choose a disposition: accept/merge, reject/trash, follow-up branch,
   bug-redo, lead-repair, future, low-value rejection, or positive escalation.
10. Update the ledger/task surfaces before starting the next row. Close the
   worker agent after disposition.

After any agent-system change, run two smoke tests before a real worker packet:
an exact-reply no-context message test, then a required-cwd read-only test. Do
not launch a research worker until both pass.

## Source Truth

Required source truth is human-readable and repo-owned:

- code or script when execution required one;
- exact command run;
- base dataset snapshot and experiment-local filters/subsets;
- generated outputs when relevant;
- `report.md`;
- ledger or task row update after lead review.

Do not require JSON or another machine-readable sidecar unless a repo-owned
checker or follow-up script consumes it. A sidecar may be useful, but it is not
the review surface.

## Worker Packet Fields

Every worker packet should include:

- required cwd/worktree and first command that prints `pwd`;
- selected semantic idea slug and lead-written objective;
- why the row matters to the shared experiment group;
- base dataset snapshot and expected counts;
- method-local choices the worker may make and must record;
- binding constraints, allowed write scope, runtime budget, and whether cluster
  compute is in scope;
- stop conditions, especially target threshold found, stale data mismatch,
  leakage bug, or objective ambiguity;
- report path and report contract;
- verdict meanings for this row;
- final-response requirements.

## Report Contract

The report starts with this Markdown header:

```markdown
Status: draft | blocked | complete
Idea slug:
Objective:
Base dataset snapshot:
Dataset filtering/subsetting:
Command run:
Verdict:
Evidence strength:
Implementation trust:
Thesis/project use:
Caveat:
Reopen trigger:
Evidence paths:
```

Then separate:

- observations;
- inference;
- checks run;
- failure modes and caveats;
- whether the result gives an actionable way to search for the target cases.

## Review Rules

Accept a result only after checking:

- the report exists in the agreed repo-owned path;
- the command/provenance are enough to rerun or audit;
- the worker, not only the lead, ran the declared command unless disposition is
  `lead-repair`;
- row counts and target-threshold counts match the packet or explain the
  mismatch;
- filters, feature exclusions, split policy, and leakage/provenance guards are
  recorded;
- statistical or numerical checks match the claim type;
- the verdict follows from observations and does not overclaim;
- caveats include density, sample-size, method-class, runtime/search-budget,
  stale-data, and implementation limits when relevant.

## Generic Worker Prompt Skeleton

```text
Required cwd/worktree: <ABSOLUTE_WORKTREE_PATH>

Use this worktree for all commands and edits. Do not edit `main` or the root
checkout. You are not alone in the codebase; do not revert or overwrite changes
made by others.

Approved surface:
<experiment group and selected semantic idea slug>

Objective:
<lead-written objective>

Why this row matters:
<why this method/check/search idea must be tried, rejected, or deferred>

Base dataset snapshot:
- Path: <DATASET_DIR>
- Producer command: <COMMAND>
- Expected row counts: <COUNTS>
- Expected max target value: <VALUE>
- Expected target-threshold count: <COUNT>

Method-local choices:
You may choose filters, feature subsets, split policy, model parameters, plots,
and sanity checks that fit the objective. Record each choice. If a choice would
change the objective, stop and report the needed change.

Binding constraints:
- First command prints `pwd`.
- Stay within <ALLOWED_WRITE_SCOPE>.
- Create <REPORT_PATH> or a blocker note before full method work.
- Final source truth is repo-owned: code/script if needed, command run, dataset
  snapshot, filters/subsets, outputs if any, and report.md.
- Do not require machine-readable sidecars unless a repo-owned consumer exists.
- Runtime budget: <BUDGET>. Cluster compute: <IN_OR_OUT_OF_SCOPE>.
- Stop for target-threshold hits, stale data mismatch, leakage bugs, or
  objective ambiguity.

Report:
Write <REPORT_PATH> using the report header and sections from
`$data-science-subexperiment`.

Verdict meanings:
<row-specific positive/conjectured-positive/falsified-positive/negative/bug-redo
meanings>

Final response:
State files changed, command run, verdict, and missing success signals. Do not
claim the row is closed; the lead owns review and integration.
```
