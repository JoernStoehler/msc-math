# Sys-Landscape Datascience

Read this file before touching sys-landscape datascience code, data, reports,
or worker prompts.

## Thesis Role

This folder supports the thesis data-science/search result. The target result
is a closed method table, not a folder or a single model.

Working thesis sentence:

> The closed method table records no new source of `sys > 1` examples and no
> candidate-proposer for finding one, beyond examples that are already explained
> by the HKO2024 construction and its symplectic images or controlled
> perturbations.

Do not weaken this to "representative methods". Standard-method coverage must
be run, ruled inapplicable, abandoned for cost, deferred with reason, or
escalated if positive.

The "standard repertoire" means the known data-science method/tool repertoire
that is relevant to this search interface. It does not mean proving exhaustion
over every possible data-science method. If a known applicable method is not
run, record the reason in `methods/README.md` or the row's `report.md`.

Do not prewrite this slice as purely negative before retained evidence and
documented deferrals or abandonments support the thesis claim. If a positive
or conjectured-positive pattern appears, record it and escalate before
continuing unrelated method cleanup.

## Required Navigation

Read these files for ordinary datascience work:

- `dataset/README.md`: current retained dataset identity and fingerprint.
- `produce/README.md`: accepted producer rows, caches, and LICCA rules.
- `tables/README.md`: accepted reusable table columns.
- `methods/README.md`: method rows and method folder conventions.
- `methods/STATUS.md`: orchestrator-approved method-row status.

The task and research notes are not ordinary entry points for this slice. Use
them only when auditing cross-thesis claim wording or older context.

## Prompt Examples

These files are examples from the PCA method-row workflow, not templates:

- `prompt-example-executor-pca-projection.md`
- `prompt-example-technical-reviewer-pca-projection.md`
- `prompt-example-thesis-reviewer-pca-projection.md`
- `prompt-example-post-run-calibration-pca-projection.md`

They live here because they describe method-row orchestration. Method
evidence and reports still live under `methods/`.

The post-run calibration prompt is sent after an executor or reviewer has
finished. Do not include it in the initial prompt; it is meant to reveal what
was unclear without priming the agent's work.

## Thesis-Success Loop

The data-science slice is successful when the retained evidence and method
table support the thesis claim with calibrated positive and negative results,
and no known open question appears worth answering after a quick
back-of-the-envelope value-of-information vs wall-time estimate.

Agents should prioritize work by thesis value, value of information, and
wall-time to useful evidence. Negative, ambiguous, or abandoned results are
valuable only when they are reproducible or explicitly non-runnable with reason,
calibrated, interpretable, and not overclaimed.

The integration branch is not scratch space. It should stay maintainable,
navigable, and documented enough that future agents can continue without
repairing stale artifacts or reconstructing intent from chat. Merge non-final
or bounded experimental artifacts only when the committed artifact has a
narrower current purpose and states its evidence status, thesis use, reason for
stopping, reopen trigger if any, and what can be deleted later. Examples are
current evidence, an explicit non-evidence status marker, or a bounded deferral
or abandonment record. Do not merge work that leaves future agents to infer
what it means.

Salvage value is not enough. Work is not mergeable if a future agent must rerun,
repair, further interpret, inspect stale files, or reconstruct intent from chat
before relying on it. Request repair, split follow-up, deferral, abandonment,
or escalation instead when a proposed merge would create maintenance burden or
false closure.

Methods should stay separated unless shared code has clear current value.
Because this is exploratory research, do not preserve legacy or superseded code
by default. Replace, delete, or prune to the takeaway once HEAD maintenance cost
exceeds the value of keeping the material available outside the git log.

## Status Authority

Method reports own evidence. Reviewers own findings. Orchestrator-approved
status lives in `methods/STATUS.md`.

Worker-written report summaries, YAML `result` fields, README summaries, and
reviewer verdicts are not authoritative method status. They are evidence with
scope limited by the artifact, prompt, and review trace. A green review means
only that the reviewer did not report a blocker under the checks it actually
performed.

If `methods/STATUS.md` has no approved status for a row, agents must read the
report and reason from evidence instead of inferring status from a report
header, README row, or reviewer verdict.

## Integration Decision Vocabulary

- Repair when the intended merge artifact is valuable but technically or
  interpretively unreliable.
- Split follow-up when the current artifact is mergeable and the remaining
  question is separable.
- Defer when the question may matter later but has lower current thesis value
  than other work.
- Abandon when expected thesis value is below maintenance and execution cost.
- Escalate when there is candidate-proposer evidence, a validated new `sys > 1`
  row, or evidence that should change thesis wording before unrelated method
  work continues.

## Method Folder Local Status

A method folder owns local evidence and local considerations for one
method-table row or explicitly named row group. It does not decide thesis
success by itself.

A method report should expose what future agents need for the thesis-success
loop: current evidence, validity limits, interpretation, important caveats,
cheap follow-up ideas, reasons to defer or abandon, and interactions with
nearby methods when they matter.

Local follow-up notes are inputs to prioritization, not automatic blockers and
not automatic todo items. Use quick value-of-information vs wall-time estimates
to decide whether they should be run now, split out, deferred, or left as
future work.

## Source Truth Order

For ordinary datascience work:

1. `dataset/README.md` and `fingerprint-dataset.py` identify the retained
   dataset.
2. `produce/README.md` and `tables/README.md` identify accepted producers and
   reusable table columns.
3. `methods/STATUS.md` records orchestrator-approved method-row status.
4. `methods/README.md` is the method-row navigation index.
5. Each method folder's `report.md` and source code are source truth for that
   method row's command, evidence, observations, proposed interpretation, and
   caveats.

For thesis wording, start from this README, `methods/STATUS.md`, and
`methods/README.md`. Older research/task notes may explain why a decision was
made, but method reports and source code are evidence artifacts, not
orchestrator-approved status.

## Data Flow

```text
produce/  ->  tables/  ->  dataset/  ->  methods/
```

- `produce/` owns accepted polytope/observation producer outputs and caches.
- `tables/` owns accepted reusable table columns and the table builder.
- `dataset/` owns the retained shared input tables.
- `methods/` owns method evidence artifacts and reports.

Method agents read `dataset/`. They do not rebuild `produce/`, edit `tables/`,
or overwrite `dataset/` unless explicitly assigned that stage.

## Current Dataset

Current retained dataset:

```text
experiments/sys-landscape/datascience/dataset/
```

Contents:

- `polytope-table.jsonl`
- `observation-table.jsonl`

Fingerprint:

- polytope rows: `8445`
- observation rows: `8445`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`
- source counts:
  - `gradient_ascent_general`: `4096`
  - `gradient_ascent_products`: `4089`
  - `random_product_sample`: `100`
  - `random_sample`: `70`
  - `variable_f_ascent`: `90`

Use `fingerprint-dataset.py` when a report or review needs explicit row
counts, hashes, max `sys`, or `sys > 1` count:

```bash
uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
  experiments/sys-landscape/datascience/dataset
```

Build or refresh the retained dataset from committed producer caches with:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Refresh `dataset/` only after an intentional producer/table-stage change.

## Architecture Rules

1. Operational truth lives in these README files, not in chat history.
2. Accepted reusable columns live in `tables/` and appear in `dataset/`.
3. Methods split retained table columns into model inputs in memory.
4. Do not track duplicate method-local `feature_*.jsonl` views unless a current
   report names a concrete consumer.
5. One active method folder should support one method-table row.
6. Current-looking reports must either contain current retained-dataset
   evidence or explicitly say they are stale/status markers.
7. Obsolete experiment artifacts are deleted by default. Extract old work only
   if it has positive expected value after contamination risk.
8. If a method records a validated new `sys > 1` row outside the known
   HKO2024-derived source, or records a candidate-proposer, stop unrelated
   method work and write an escalation note stating the evidence, affected
   thesis claim or wording, and recommended next action before continuing.

## Deletion-First Rule

Old work is not valuable because it exists. Before extracting from old
experiments, check:

- Which method-table row does it support now?
- Does it run on the current retained dataset?
- Does it avoid stale paths, old row counts, duplicate local data, and vague
  vocabulary?
- Is adapting it safer than rewriting a small clean script?

If not, delete or leave it in git history.

## Stage Documentation

- Producer stage: `produce/README.md`
- Table stage: `tables/README.md`
- Dataset stage: `dataset/README.md`
- Method stage: `methods/README.md`
