# Sys-Landscape Datascience Methods

This directory owns current method packets for the sys-landscape datascience
method table.

Read first:

1. `../README.md`
2. `../tables/README.md`
3. `STATUS.md`

## Current State

There are no current method packets in HEAD after the table-output ownership
reset.

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

A method folder may contain:

- a small script or scripts;
- small generated artifacts needed for audit;
- figures or assets if they are directly used;
- `report.md` as the current research ledger.

Delete stale scripts, reports, generated artifacts, and review traces once
their value in HEAD is lower than their maintenance and confusion cost. Git
history is the archive for obsolete runs.

## Reports

A `report.md` is a research ledger. It is not raw evidence and it is not
approved status.

Use a report to record:

- the method-specific question;
- retained table input and method-local input construction used;
- command and runtime;
- retained generated artifacts;
- observation;
- proposed interpretation;
- validity limits;
- thesis use;
- follow-up, deferral, abandonment, or escalation recommendation.

Future agents should use reports to orient, then re-check any claim they rely
on against current code, retained data, generated artifacts, and `STATUS.md`.

## Status

Approved current method-row status lives in `STATUS.md`.

Executors and reviewers may recommend status, but they do not approve it. A
green review is evidence only for the checks it actually performed.

## Coverage References

Taxonomy snapshots are under `taxonomies/`. They are coverage references, not
repo-state summaries and not thesis evidence.

Read them only when deciding whether a standard method family exists or should
be named:

- `taxonomies/README.md`

Optional or deferred method ideas live in `future-method-ideas.md`. Do not use
that file as an automatic task queue.
