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

Do not prewrite this slice as purely negative before row closure. If a
positive or conjectured-positive pattern appears, record it and escalate before
continuing unrelated method cleanup.

## Required Navigation

Read these files for ordinary datascience work:

- `dataset/README.md`: current retained dataset identity and fingerprint.
- `produce/README.md`: accepted producer rows, caches, and LICCA rules.
- `tables/README.md`: accepted reusable table columns.
- `methods/README.md`: method rows, current/stale report status, and method
  folder conventions.

The task and research notes are not ordinary entry points for this slice. Use
them only when auditing cross-thesis claim wording or older context.

## Prompt Examples

These files are examples from the PCA method-packet flow, not templates:

- `prompt-example-executor-pca-projection.md`
- `prompt-example-technical-reviewer-pca-projection.md`
- `prompt-example-thesis-reviewer-pca-projection.md`
- `prompt-example-post-run-calibration-pca-projection.md`

They live here because they describe method-packet orchestration. Method
evidence and reports still live under `methods/`.

The post-run calibration prompt is sent after an executor or reviewer has
finished. Do not include it in the initial prompt; it is meant to reveal what
was unclear without priming the agent's work.

Method packet merge standard:

- The executor hands back a committed method-packet diff on its method branch.
  Reviewers review the commit, not a loose dirty worktree, unless dirty state
  affects reproducibility or makes the committed packet ambiguous.
- Reviewers report findings, consequences, severity, reproducibility evidence,
  and whether issues block direct use as current evidence. They do not own the
  final acceptance decision.
- The orchestrator accepts a packet into the integration branch only when the
  packet is directly usable as current method-table evidence with ordinary
  thesis-writing work remaining.
- Useful raw work, salvage value, or "better than nothing" is not enough. If a
  packet needs substantial repair, reinterpretation, rerun, or archaeology
  before the row can be trusted, keep it out of the integration branch or revise
  it before merging.
- A packet with a candidate-proposer or validated new row should be escalated
  before ordinary method-table closure continues.

Method packet success standard:

- A committed method packet is successful when it can be used as current
  method-table evidence without rerunning the experiment, repairing the
  analysis, further interpreting the result, or inspecting stale files.
- The report should include enough context for a future agent to understand and
  audit the method-table evidence. This usually includes the method question,
  dataset, command, reproducibility status, what the method found, what follows
  from that evidence and how strongly, what does not follow, whether the packet
  contains candidate-proposer evidence or a validated new `sys > 1` row,
  whether it contains other positive or ambiguous descriptive evidence, and
  what should happen next for the row.
- A negative method verdict must not hide positive or ambiguous evidence. If
  the method finds a pattern that changes the row's interpretation, thesis
  wording, or follow-up, the report should say so before presenting the
  terminal state.
- If the packet is not usable as current method-table evidence, state what
  blocks use and what next action is recommended.

## Source Truth Order

For ordinary datascience work:

1. `dataset/README.md` and `fingerprint-dataset.py` identify the retained
   dataset.
2. `produce/README.md` and `tables/README.md` identify accepted producers and
   reusable table columns.
3. `methods/README.md` is the method-row navigation index.
4. Each method folder's `report.md` and source code are the source truth for
   that method row's command, evidence status, result, and caveats.

For thesis wording, start from this README and `methods/README.md`. Older
research/task notes may explain why a decision was made, but method reports and
source code are the source truth for current row-level evidence status.

## Data Flow

```text
produce/  ->  tables/  ->  dataset/  ->  methods/
```

- `produce/` owns accepted polytope/observation producer outputs and caches.
- `tables/` owns accepted reusable table columns and the table builder.
- `dataset/` owns the retained shared input tables.
- `methods/` owns method evidence packets and reports.

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
   method work and write an escalation packet before continuing.

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
