# Sys-Landscape Datascience Methods

This directory owns current method packets for the sys-landscape datascience
method table.

Read first:

1. `../README.md`
2. `../tables/README.md`
3. the relevant method folder `README.md`

## Current State

Current method packets in HEAD:

- `scan-sys-gt-1/`: baseline target-predicate scan over the retained tables.

Old PCA, clustering, regression, classification, supervised-alternative,
exact/f64, prompt example, and review artifacts were deleted from HEAD because
they were stale, architecture-inconsistent, or cheaper to rerun cleanly than to
maintain. Use git history only if a specific extraction has positive expected
value after contamination risk.

## Method Question

The global question is fixed:

> Can a standard datascience method help find a `sys > 1` polytope outside the
> already explained HKO2024-derived class, including symplectic images and
> controlled perturbations?

Each method packet states only the method-specific reduction of that question:
what this method would count as positive evidence, negative evidence,
abandonment, deferral, or escalation.

## Inputs

Ordinary rectangular datascience methods should start from retained tables
under `../tables/` and build method-specific input matrices inside the method
folder. Copy and adapt input-building code between methods when that is cheaper
than maintaining shared helper code.

Current retained table facts:

- polytope rows: `8445`
- provenance rows: `8445`
- ascent run rows: `8275`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`

## Method Packets

Create a method folder only when running or recording a current method packet.
One active method folder should support one method-table row or one explicitly
named row group.

A method folder `README.md` is the durable control surface for that packet. It
should record the current method-specific state that future agents need:

- research question;
- datascience method being applied to the black-box search problem;
- retained input tables and method-local input or feature construction;
- commands to run or rerun;
- retained generated artifacts, if any;
- observations and proposed interpretation with epistemic status;
- validity guards, leakage concerns, and scope limits;
- Jörn feedback that is specific to the method;
- cross-references to related method folders;
- current disposition;
- remaining worthwhile questions;
- predicted stability under rerun;
- thesis use;
- reopen triggers.

A method folder may contain:

- a small script or scripts;
- small generated artifacts needed for audit;
- figures or assets if they are directly used;
- disposable GPT-5.5 interpretation notes if they have short-term value.

Do not use `report.md` as the durable method state. If a worker produces a
report, extract any current value into the method `README.md` and then delete
the report once the packet is integrated. Delete stale scripts, reports,
generated artifacts, and review traces once their value in HEAD is lower than
their maintenance and confusion cost. Git history is the archive for obsolete
runs.

## Disposition Vocabulary

Use current-disposition language instead of hard signoff/finality language.
Useful fields are:

- `current disposition`: what the packet currently supports, defers, abandons,
  or escalates;
- `remaining worthwhile questions`: follow-up checks with positive expected
  thesis value;
- `predicted stability under rerun`: whether the packet is likely to change if
  rerun on unchanged retained tables;
- `reopen triggers`: concrete source-truth changes that make the packet stale;
- `thesis use`: what thesis-facing statement the packet can support and what it
  must not be used to claim.

Escalate before unrelated method work continues when a method records a
validated new `sys > 1` row outside the known HKO2024-derived source, records a
candidate-proposer, or gives evidence that should change thesis wording.

## Jörn Feedback

Record method-specific Jörn feedback in the relevant method `README.md`, near
the observation or validity guard it affects.

Example to preserve when PCA is recreated: PCA analysis still did not check
whether PC0 from random generic polytopes and random Lagrangian-product
polytopes are similar.

Cross-method thesis interpretation belongs in
`thesis/black-box-datascience-content.md` or future thesis content files, not
in a central method-status ledger. If someone needs a dashboard, ask an agent
to read the method READMEs and synthesize the current view.

## Coverage Checklist

Use `method-coverage-checklist.md` to avoid forgetting datascience methods,
tactics, patterns, and concepts that may deserve a method-table disposition.
It is intentionally redundant and not a taxonomy, repo-state summary, thesis
evidence, or task queue.

Read it when choosing or reviewing method-table coverage:

- `method-coverage-checklist.md`
