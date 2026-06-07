# Datascience Method Status

This file is the orchestrator-owned current status ledger for
`experiments/sys-landscape/datascience/methods/`.

## Current State

No method row currently has approved HEAD evidence after the table-output
ownership reset.

The previous PCA packet, old status-marker reports, old review traces, and old
prompt examples were removed from HEAD during the reset. They remain in git
history as historical committed states. Do not infer current method status from
those deleted files.

## Status Rules

1. A method folder may contain scripts, generated artifacts, and `report.md`
   only when that material has current positive value in HEAD.
2. A `report.md` is a research ledger: it records retained observations,
   proposed interpretation, caveats, and follow-up ideas. It is not raw
   evidence and it is not approval.
3. Raw evidence is current code, retained input data, generated artifacts,
   commands, and reproducible checks.
4. Git history is the archive for obsolete reports, stale scripts, deleted
   generated artifacts, and old prompt/review material.
5. Reviewers own findings. A green review means only that the reviewer did not
   report a blocker under the checks it actually performed.
6. This file owns approved current method-row status. Update it only after the
   orchestrator inspects the retained evidence and review trace.
7. Jörn approval is required for status that changes thesis wording, records a
   candidate-proposer, records a validated new row, or is otherwise
   ambiguous/high-impact.

## Approved Status Ledger

| Method | Approved current status | Evidence in HEAD | Scope / reopen trigger |
| --- | --- | --- | --- |
| none | No approved current method-row evidence after the reset. | None. | Start new method work from `methods/README.md` and current retained tables. |
