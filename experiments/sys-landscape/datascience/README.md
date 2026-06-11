# Sys-Landscape Datascience

Read this file before touching sys-landscape datascience code, data, method
packets, or worker prompts.

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
over every possible data-science method.

Do not prewrite this slice as purely negative before retained evidence and
documented deferrals or abandonments support the thesis claim. If a positive
or conjectured-positive pattern appears, record it and escalate before
continuing unrelated method cleanup.

## Evidence Model

Current HEAD is the working surface. Git history is the archive for obsolete
runs, stale scripts, deleted reports, old generated artifacts, old review
traces, and old prompt examples.

Files in HEAD must have current value. Do not keep historical material only
because it once existed.

- `produce/`: owns row production code, producer caches, and producer outputs.
- `tables/`: owns reusable table-column computation and the retained table
  outputs under `tables/`.
- `methods/`: owns current method packets, one durable `README.md` per active
  method folder.

A method folder `README.md` is the durable method-packet control surface:
research question, datascience method, input tables and features, commands,
retained artifacts, method-specific Jörn feedback, related method folders,
validity guards and leakage concerns, current disposition, remaining
worthwhile questions, predicted stability under rerun, thesis use, and reopen
triggers.

`report.md` files are optional disposable GPT-5.5 interpretation notes, not
durable state. If a worker produces one, extract current value into the method
`README.md` before integration and then delete the report. Future agents should
re-check claims they rely on against current code, retained data, generated
method artifacts, table fingerprints, and the method `README.md`.

## Required Navigation

Read these files for ordinary datascience work:

- `produce/README.md`: accepted producer rows, caches, and LICCA rules.
- `tables/README.md`: table builder ownership, retained table outputs, and
  fingerprints.
- `methods/README.md`: method packet conventions and current packet list.
- relevant `methods/<method>/README.md`: durable state for that method packet.

The task and research notes are not ordinary entry points for this slice. Use
them only when auditing cross-thesis claim wording or older context.

## Data Flow

```text
produce/  ->  tables/  ->  methods/
```

- `produce/` owns accepted polytope producer outputs and caches.
- `tables/` owns accepted reusable table columns, the table builder, and the
  retained table outputs under `tables/`.
- `methods/` owns current method scripts, retained method artifacts, and
  method-packet README files.

Consumers do not control producer outputs. If a method needs a rectangular
input shape, build it inside the method folder unless a later concrete reuse or
compute-cost case justifies promoting it to a shared helper.

Use these examples to place changes:

- Adding a scalar column that several methods should reuse: edit `tables/`.
- Building PCA input columns from existing retained columns: build the input
  inside the PCA method folder.
- Saving a PCA-specific transformed matrix: keep it under that method folder.
- Changing which polytopes are retained: edit `produce/`, then rebuild
  `tables/`.
- Deleting old current-looking reports: delete from HEAD; git history is the
  archive.
- Renaming a retained table: treat it as an API change and scan stale
  references across README files, map files, research notes, task notes,
  scripts, and generated manifests.

If an example does not fit, decide from row ownership: producer outputs belong
to `produce/`, reusable retained data to `tables/`, and method-specific
inputs or artifacts to `methods/`. If a row entity changes, choose a table name
for the new row entity rather than preserving the old name.

## Current Retained Tables

Current retained table output:

```text
experiments/sys-landscape/datascience/tables/
```

Contents:

- `polytope-table.jsonl`: one row per retained polytope keyed by `poly_id`;
  contains defining dual vertices, computed polytope-level quantities such as
  `volume`, capacity, and `sys`, derived scalar features, and capacity/orbit
  audit fields.
- `polytope-provenance-table.jsonl`: one row per retained provenance record
  keyed by `provenance_id`; records how a retained polytope entered the
  datascience tables, including source, role, optimizer, seed, path, and
  lineage.
- `polytope-ascent-run-table.jsonl`: one row per ascent or continuation
  provenance record keyed by `provenance_id`; records run-level and
  trajectory-summary fields. Random-sample provenance rows do not appear here.

Fingerprint:

- polytope rows: `8445`
- provenance rows: `8445`
- ascent run rows: `8275`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`
- source counts:
  - `gradient_ascent_general`: `4096`
  - `gradient_ascent_products`: `4089`
  - `random_product_sample`: `100`
  - `random_sample`: `70`
  - `variable_f_ascent`: `90`
- sha256:
  - `polytope-table.jsonl`:
    `bc96000d2c7a70c4aa777891a020bf3c8f7d11d8ee17a084519e2706ce2b4554`
  - `polytope-provenance-table.jsonl`:
    `abe2976decf84531b935132259bb526707b8dcf77844b23a4b64780f53673e8f`
  - `polytope-ascent-run-table.jsonl`:
    `b75b29c66f30ca27e3d6dd289f1f9a8169bca532e6be1b0e0da816fe1963c420`

Check the retained tables with:

```bash
uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
  experiments/sys-landscape/datascience/tables
```

Build or refresh the retained tables from committed producer caches with:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Refresh `tables/` only after an intentional producer or table-stage
change.

## Thesis-Success Loop

The data-science slice is successful when retained evidence and method-table
coverage support the thesis claim with calibrated positive and negative
results, and no known open question appears worth answering after a quick
value-of-information versus wall-time estimate.

Agents should prioritize work by thesis value, value of information, and
wall-time to useful evidence. Negative, ambiguous, or abandoned results are
valuable only when they are reproducible or explicitly non-runnable with
reason, calibrated, interpretable, and not overclaimed.

The integration branch is not scratch space. It should stay maintainable,
navigable, and documented enough that future agents can continue without
repairing stale artifacts or reconstructing intent from chat.

Methods should stay separated unless shared code has clear current value.
Because this is exploratory research, do not preserve legacy or superseded code
by default. Replace, delete, or prune to the takeaway once HEAD maintenance cost
exceeds the value of keeping the material available outside the git log.

## Method-Packet Authority

Method-specific durable state lives in `methods/<method>/README.md`.
Reviewers own findings, and worker-written report summaries, YAML `result`
fields, and reviewer verdicts are not durable method state by themselves. A
green review means only that the reviewer did not report a blocker under the
checks it actually performed.

Use current-disposition, remaining-worthwhile-question, predicted-stability,
reopen-trigger, and thesis-use language. Avoid hard signoff or finality
language unless quoting old text.

Cross-method thesis interpretation belongs in
`thesis/black-box-datascience-content.md` or future thesis content files when
those surfaces exist. If someone needs a cross-method dashboard, ask an agent
to read the method READMEs and synthesize the current view.

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

## Architecture Rules

1. Operational truth lives in these README files, not in chat history.
2. Producer outputs live with the producer that owns their meaning.
3. Accepted reusable columns and retained table output live in `tables/`.
4. Ordinary methods read retained tables from `tables/` and build
   method-specific rectangular inputs inside the method folder.
5. Do not promote method-local input builders into shared code until a concrete
   reuse or compute-cost case exists.
6. Do not track duplicate method-local `feature_*.jsonl` inputs unless a
   method README names a concrete consumer.
7. One active method folder should support one method-table row or explicitly
   named row group.
8. Method READMEs are durable method-packet state. Delete integrated, stale, or
   status-marker `report.md` files instead of keeping them in HEAD.
9. Obsolete experiment artifacts are deleted by default. Extract old work only
   if it has positive expected value after contamination risk.
10. If a method records a validated new `sys > 1` row outside the known
    HKO2024-derived source, or records a candidate-proposer, stop unrelated
    method work and write an escalation note stating the evidence, affected
    thesis claim or wording, and recommended next action before continuing.

## Deletion-First Rule

Old work is not valuable because it exists. Before extracting from old
experiments, check:

- Which method-table row does it support now?
- Does it run on the current retained tables?
- Does it avoid stale paths, old row counts, duplicate local data, and vague
  vocabulary?
- Is adapting it safer than rewriting a small clean script?

If not, delete or leave it in git history.

## Stage Documentation

- Producer stage: `produce/README.md`
- Table stage and retained outputs: `tables/README.md`
- Method stage: `methods/README.md`
