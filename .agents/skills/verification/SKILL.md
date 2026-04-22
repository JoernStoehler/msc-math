---
name: verification
description: Verification workflow for final thesis-done checks and ongoing readiness passes. Use when asked whether claims, repo promises, code/data outputs, figures, or experiment surfaces are supported, complete, reproducible, truthful, or ready, or when asked what verification gates exist.
---

# Verification

This skill routes verification work between the repo's authority surfaces.
The workflows and packets below are jumpstarters for verification work, not
guarantees that the listed checks suffice, are always the most useful next
checks, or rule out an obvious ad-hoc improvement. Use them as the default
starting point, then tighten, skip, or extend checks when the actual surface
demands it.

## Authority Split

- `FINAL-VERIFICATION.md` is the authoritative final finished-state spec.
- `TASKS.md` owns milestones, sequencing, and ownership.
- `RESULTS.md` owns the intended thesis claim surface.
- `references/*.md` in this skill own reusable operational verification passes.

Do not promote a pre-final readiness check into `FINAL-VERIFICATION.md` unless
Jörn explicitly wants it to become part of thesis-done.

## Workflow

1. Decide whether the user is asking about:
   - final thesis/project done conditions;
   - a reusable pre-final readiness pass; or
   - current status of an open verification surface.
2. If the task is about literal thesis/project done conditions, start with
   `FINAL-VERIFICATION.md`.
3. Otherwise list the available packet files:
   `ls .agents/skills/verification/references`
4. Packet files in `references/` intentionally use statement-form names such as
   `x-are-y.md`. Treat that as a signal that the file owns a real reusable
   verification condition, not scratch notes or generic background prose.
5. If the dedicated packet is missing, use the nearest authority surface and
   state the gap explicitly instead of inventing a new packet in the reply.
6. Report findings first. Separate:
   - supported/pass;
   - caveat needed;
   - missing support or stale evidence;
   - Jörn-only judgment.

## Packet Notes

- Packet files should mostly contain:
  - the property being checked;
  - concrete checks;
  - minimal packet-specific caveats.
- Keep general workflow, fallback routing, and output-shape reminders here in
  `SKILL.md`, not repeated in every packet.
- Treat packet checklists as suggested starter workflows. They are not proofs of
  sufficiency and they do not forbid adding a sharper local check when one is
  obvious.

If a task sounds close to an existing condition but does not match exactly,
prefer the nearest packet with the smallest starter read set and state the
missing packet boundary explicitly in the reply.

## Coverage

Status labels:

- `packet exists`
- `use nearest packet`
- `missing packet`
- `mostly Jörn-only`

Current coverage:

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

Near-term packet candidates:

- `data-and-figures-are-reproducible.md`
  Repeated figure/provenance tasks currently borrow
  `repo-promises-are-truthful.md`, but the read set is already distinct enough
  (`DATAFLOW.md`, producer/analyze scripts, figure provenance).
- `references-resolve.md`
  Likely useful once the thesis text is less placeholder-heavy.
- `submission-artifacts-are-complete.md`
  Likely useful closer to final assembly.

Non-final adjacent conditions:

- `code-is-high-quality.md`
  `packet exists`
- `test-coverage-is-high.md`
  `packet exists`
- `verification-experiments-try-to-falsify-the-story.md`
  `packet exists`

## Large Passes

For long checklists, do not force one top-to-bottom pass in one burst.

- Split by independent read sets when that is obviously cheaper.
- Pause at reasoning steps instead of free-associating through the whole list.
- Write intermediate notes to `/tmp/` or another scratch path when the pass is
  large enough that you would otherwise lose state.

This skill is intended for ongoing verification work during the project, not
only a last-hours pre-submission sweep.
