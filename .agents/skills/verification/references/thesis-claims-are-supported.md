# Thesis Claims Are Supported

## Use When

Use this packet when the task is to check whether current or proposed
thesis-facing claims are actually supported, overstated, incomplete, or missing
required caveats.

This packet is useful before the thesis text is final. When the thesis prose is
still partial, verify retained thesis stories against `research/README.md`,
topic research notes, and `tasks/*.md` obligations instead of waiting for the
last writing pass.

## Authority And Scope

Use these surfaces in this order:

1. `research/README.md` for the thesis story index, plus topic `research/*.md`
   notes for interpretation and proof-route state.
2. `FINAL-VERIFICATION.md` for final node IDs and thesis-done semantics,
   especially `T1`, `T2`, `T4`, and `T6`.
3. The current thesis artifact when the wording already exists in `thesis/`.
4. The cited formal notes, experiment packages, preserved artifacts, and
   research notes that the claim relies on.
5. `ROADMAP.md` and `tasks/*.md` for known open blockers, proof/writeup
   obligations, pending evidence, or explicit defer/future status.

Do not let a current experiment note silently strengthen a thesis claim beyond
what the research interpretation, task obligations, or thesis text say.

## Procedure

1. Name the exact claim block or subsection under review.
2. List the support the claim would need:
   - theorem/proof;
   - experiment/data artifact;
   - figure/table/provenance;
   - caveat/future-work wording;
   - repo/reproducibility promise.
3. Inspect the actual supporting sources and classify each claim as one of:
   - `supported`
   - `supported only with caveat`
   - `missing support`
   - `future/cut unless support arrives`
   - `Jörn decision needed on claim strength or framing`
4. Report findings first with file paths, node IDs, and concrete missing pieces.
5. If the support is absent but the honest weaker wording is obvious, propose
   the weaker wording or the needed caveat instead of only saying "not done".

## Ask Jörn Only For

- choosing between plausible claim strengths when the evidence does not decide;
- thesis-framing decisions about whether to keep, weaken, or cut a claim;
- mathematical judgment that the available artifacts do not settle.

Do not ask Jörn to do agent work such as locating the cited source, comparing
the claim text against the artifact, or inventorying which support type is
missing.

## Output Shape

Prefer findings first in severity order. For each finding, say:

- the claim or claim block;
- status;
- evidence checked;
- what is missing or what caveat is required;
- whether the issue is agent-fixable or Jörn-only.
