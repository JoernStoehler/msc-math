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
