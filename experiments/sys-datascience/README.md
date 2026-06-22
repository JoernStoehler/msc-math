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

Current closure status: pending. The authoritative method-state surfaces are
`methods/README.md` for conventions and the active
`methods/<method>/README.md` packets for row state. Use
`methods/method-coverage-checklist.md` only as a recall aid. A closed method
table exists only when current method packets record each standard-method
disposition or deferral with enough evidence for the thesis wording above.

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
- `prepare/`: owns shared canonization, reusable feature computation, and the
  retained prepared table outputs.
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
- `prepare/README.md`: prepare-stage ownership, retained prepared outputs, and
  fingerprints.
- `feature-space-coverage-ledger.md`: current feature-object coverage,
  audit status, invariance caveats, and next feature/method round.
- `methods/README.md`: method packet conventions and current packet list.
- `methods/random-only-closure-summary.md`: current cross-method dashboard for
  the trusted random/product method slice and its remaining evidence gate.
- `methods/random-only-method-dispositions.md`: run/defer/reject/out-of-scope
  dispositions for checklist families in the trusted random/product scope.
- `methods/method-coverage-checklist.md`: recall checklist for standard-method
  coverage while choosing, reviewing, or closing method-table rows.
- relevant `methods/<method>/README.md`: durable state for that method packet.

The task and research notes are not ordinary entry points for this slice. Use
them only when auditing cross-thesis claim wording or older context.

## Data Flow

```text
produce/  ->  prepare  ->  methods/
```

- `produce/` owns accepted producer outputs, run traces, and caches for
  expensive computations. It should preserve source-truth facts such as
  computed polytope payloads, branch/action windows, and enough run metadata to
  reproduce how rows were sampled. It should not own method-facing rectangular
  feature shapes when those features can be derived cheaply downstream.
- `prepare/` is the shared downstream stage. It owns reusable row entities,
  canonization choices, reusable feature computation, deliberate deduplication,
  and retained prepared table outputs. Promote preparation logic here when
  several methods would otherwise reimplement the same joins, normalization,
  feature extraction, or consistency checks. This is justified by repeated
  development/debugging cost, not only by CPU time.
  Operationally, this split gives two commands to run: produce expensive
  polytopes/capacity payloads once, then recompute canonization and features as
  needed. Prepare-stage canonization and feature computation should be local by
  default because they are cheap compared with capacity search; use LICCA for
  prepare only when table size or feature cost actually makes local runs
  impractical.
- `methods/` owns current method scripts, retained method artifacts, and
  method-packet README files. Methods answer one research question from
  producer/table data. They may do cheap method-specific projections,
  regressions, plots, and reports, but should not recompute capacity or own
  general-purpose joins that are already reused.

Consumers do not control producer outputs. If a method needs a rectangular
input shape, build it inside the method folder unless concrete reuse, repeated
implementation/debugging cost, or compute cost justifies promoting it to
the shared prepare stage or another explicitly shared prepare surface.

Use these examples to place changes:

- Adding a scalar column that several methods should reuse: edit `prepare/`.
- Building PCA input columns from existing retained columns: build the input
  inside the PCA method folder.
- Saving a PCA-specific transformed matrix: keep it under that method folder.
- Changing which polytopes are retained: edit `produce/`, then rebuild
  `prepare/`.
- Deleting old current-looking reports: delete from HEAD; git history is the
  archive.
- Renaming a retained table: treat it as an API change and scan stale
  references across README files, map files, research notes, task notes,
  scripts, and generated manifests.

If an example does not fit, decide from row ownership: producer outputs belong
to `produce/`, reusable retained data to the shared prepare stage, and
method-specific inputs or artifacts to `methods/`. If a row entity changes,
choose a table name for the new row entity rather than preserving the old name.

## Retained Tables

Retained table output path:

```text
experiments/sys-datascience/prepare/
```

Expected contents after rebuilding with the current table builder:

- `polytope-table.jsonl`: one row per retained exact polytope geometry keyed by `poly_id`;
  contains defining dual vertices, computed polytope-level quantities such as
  `volume`, capacity, and `sys`, derived scalar features, and capacity/orbit
  audit fields.
- `computed-polytope-observation-table.jsonl`: one row per fixed-F ascent
  producer computed-polytope observation; records ascent context. Intermediate
  ascent observations may reference producer-retained polytopes that are not
  materialized as feature rows in `polytope-table.jsonl`.
- `polytope-provenance-table.jsonl`: one row per retained provenance record
  keyed by `provenance_id`; records how a retained polytope entered the
  datascience prepared tables, including source, role, optimizer, seed, path, and
  lineage.
- `polytope-ascent-run-table.jsonl`: one row per ascent or continuation
  provenance record keyed by `provenance_id`; records run-level and
  trajectory-summary fields. Random-sample provenance rows do not appear here.

Checked-in retained-table fingerprint on this branch:

- polytope rows: `32610`
- computed-polytope observations: `879235`
- provenance rows: `22611`
- ascent run rows: `8275`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`
- computed observations without retained polytope rows: `829497`
- source counts:
  - `gradient_ascent_general`: `4096`
  - `gradient_ascent_products`: `4089`
  - `random_product_sample`: `10240`
  - `random_sample`: `4096`
  - `variable_f_ascent`: `90`
- sha256:
  - `polytope-table.jsonl`:
    `18c94481590e3f5739748f195817f78de24335f5bdd66fa344bcecb12744b6b7`
  - `computed-polytope-observation-table.jsonl`:
    `61e7d8f25810c022b4e7d6f0aa53f0a99a8917e047e22f71ff415b042e69c121`
  - `polytope-provenance-table.jsonl`:
    `23bf91c55aecd7b9e139ea5b3a324942cdf716ce26d1febc4ee8c112f09f5e5c`
  - `polytope-ascent-run-table.jsonl`:
    `b75b29c66f30ca27e3d6dd289f1f9a8169bca532e6be1b0e0da816fe1963c420`

From repo root, check the retained tables with:

```bash
uv run --script experiments/sys-datascience/fingerprint-dataset.py \
  experiments/sys-datascience/prepare
```

From repo root, build or refresh the retained tables from committed producer
caches with:

```bash
experiments/sys-datascience/build-dataset.sh
```

Current feature-closure branch operational note: the post-feature full retained
rebuild should be run on LICCA, not retried locally by default. On 2026-06-22 a
local canonical rebuild loaded `32610` polytopes, `22611` provenance rows, and
`879235` computed-polytope observation rows, then was aborted during table
construction after the local compute/memory guard fired. Use
`licca-build-dataset.slurm.sh` for the next full rebuild gate, then rerun the
method packets against the rebuilt retained tables.

Other operational entry points:

- `pipeline.local.sh`: local command map for focused smoke, shard, merge,
  cache-benchmark, scan, and retained-table build steps.
- `smoke-pipeline.sh`: temp-output integration smoke; useful when checking the
  older full produce-to-table surface, not as a cheap command check.
- `licca-build-dataset.slurm.sh`: LICCA retained-table rebuild from canonical
  producer files; distinct from the new run-local `produce`/`prepare` smoke.
- `licca-post-feature-rebuild.md`: bounded handoff for the current
  feature-closure rebuild gate, including branch/LFS preconditions, LICCA
  submission, retrieval, and local method reruns.

Refresh `prepare/` only after an intentional producer or prepare-stage
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

Candidate-proposer threshold: a method is a candidate-proposer only when it
gives a concrete, reproducible rule or ranking that selects new polytopes or
producer settings for targeted follow-up with positive expected value. A
diagnostic saying that such a rule might exist is not yet a candidate-proposer.

## Architecture Rules

1. Operational truth lives in these README files, not in chat history.
2. Producer outputs live with the producer that owns their meaning.
3. Accepted reusable columns and retained prepared output live in `prepare/`.
4. Ordinary methods read retained prepared tables from `prepare/` and build
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
- Prepare stage and retained outputs: `prepare/README.md`
- Method stage: `methods/README.md`
