# Verification Coverage Index

This file is the coverage map for the verification skill. It is the one
intentional non-condition file in `references/`.

Use it when you need to answer:

- which final-tree areas already have a runnable operational packet;
- which areas currently borrow a nearest packet;
- which areas are still missing or remain mostly Jörn-only.

Authority:

- `FINAL-VERIFICATION.md` stays authoritative for literal thesis-done
  conditions.
- The files below only map current operational coverage under this skill.

Status labels:

- `packet exists`: there is a runnable condition file for this area.
- `use nearest packet`: no dedicated packet yet; borrow the named packet and say
  so explicitly.
- `missing packet`: no good operational packet yet.
- `mostly Jörn-only`: agents can gather evidence, but the dominant remaining
  decision is audience/taste/framing/claim-strength judgment.

## Current Coverage

- `T1` thesis-facing result surface is included:
  `use nearest packet` -> `thesis-claims-are-supported.md`
- `T2` thesis claims have support of the right type and strength:
  `packet exists` -> `thesis-claims-are-supported.md`
- `T3` thesis is understandable enough for its audience:
  `mostly Jörn-only`
- `T4.1` bibliography resolves:
  `missing packet`
- `T4.2` internal thesis cross-references resolve:
  `missing packet`
- `T4.3` theorem / definition / proof-source references resolve:
  `use nearest packet` -> `thesis-claims-are-supported.md`
- `T4.4` figure and table provenance resolves:
  `use nearest packet` -> `repo-promises-are-truthful.md`
- `T4.5` experiment / dataset / code / result-artifact references resolve:
  `use nearest packet` -> `repo-promises-are-truthful.md`
- `T4.6` algorithm and method references resolve:
  `use nearest packet` -> `thesis-claims-are-supported.md`
- `T5` repo artifact matches thesis promises:
  `packet exists` -> `repo-promises-are-truthful.md`
- `T6` unfinished work is cut, caveated, or labeled future:
  `use nearest packet` -> `thesis-claims-are-supported.md`
- `T7` submission and mechanical handin are complete:
  `missing packet`
- `T8` thesis is useful to its audience:
  `mostly Jörn-only`

## Near-Term Packet Candidates

- `data-and-figures-are-reproducible.md`
  Why next: repeated figure/provenance tasks currently borrow
  `repo-promises-are-truthful.md`, but the read set is already distinct enough
  (`DATAFLOW.md`, producer/analyze scripts, figure provenance).
- `references-resolve.md`
  Why later: would cover `T4.1`-`T4.3` once the thesis text is less placeholder-
  heavy.
- `submission-artifacts-are-complete.md`
  Why later: useful closer to final assembly, not yet a frequent operational
  pass.
