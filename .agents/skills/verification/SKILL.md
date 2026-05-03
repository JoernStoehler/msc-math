---
name: verification
description: Repeated verification and quality-measurement workflow for thesis stories, repo promises, code/data outputs, figures, experiment surfaces, and cached operational definitions. Use when asked whether a surface is supported, complete, reproducible, truthful, ready, or how to run reusable quality checks.
---

# Verification

This skill routes verification work between the repo's authority surfaces.
The workflows and packets below are repeated quality-measurement tools and
cached operational definitions. They are jumpstarters for verification work,
not guarantees that the listed checks suffice, are always the most useful next
checks, or rule out an obvious ad-hoc improvement. Use them as the default
starting point, then tighten, skip, or extend checks when the actual surface
demands it.

## Authority Split

- `tasks/verify-thesis-done.md` owns the once-run final thesis-done gate.
- `ROADMAP.md` and `tasks/*.md` own milestones, sequencing, ownership, and
  cached task knowledge.
- `research/INDEX.md` and `research/*.md` own thesis story interpretation and
  proof-route state; `tasks/*.md` owns the remaining obligations caused by
  desired thesis stories.
- `references/*.md` in this skill own reusable operational verification passes,
  quality measurements, and cached definitions of what a check means.

Do not expand `tasks/verify-thesis-done.md` with repeated workflows. Put
read-many/run-many check procedures here, and let the final gate point to the
packet that produces the finding.

## Workflow

1. Decide whether the user is asking about:
   - final thesis/project done conditions;
   - a reusable pre-final readiness pass; or
   - current status of an open verification surface.
2. If the task is about literal thesis/project done conditions, start with
   `tasks/verify-thesis-done.md`.
3. Otherwise list the available packet files:
   `ls .agents/skills/verification/references`
4. Packet files in `references/` intentionally use statement-form names such as
   `x-are-y.md`. Treat that as a signal that the file owns a real reusable
   verification condition, not temporary notes or generic background prose.
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

Current coverage is a cached routing aid. Before reporting `packet exists`,
verify that the named file is present under `.agents/skills/verification/references/`.

Current coverage:

- thesis-facing research story surface is included:
  `packet exists` -> `thesis-stories-are-supported.md`
- thesis claims have support of the right type and strength:
  `packet exists` -> `thesis-stories-are-supported.md`
- thesis is understandable enough for its audience:
  `mostly Jörn-only`
- bibliography and internal thesis cross-references resolve:
  `packet exists` -> `references-resolve.md`
- theorem / definition / proof-source / algorithm references resolve:
  `packet exists` -> `references-resolve.md`
- figure, table, dataset, code, and experiment-artifact provenance resolves:
  `packet exists` -> `data-and-figures-are-traceable.md`
- repo artifact matches thesis promises:
  `packet exists` -> `repo-promises-are-truthful.md`
- unfinished work is cut, caveated, or labeled future:
  `packet exists` -> `thesis-stories-are-supported.md`
- submission artifacts and archive prerequisites are complete:
  `packet exists` -> `submission-artifacts-are-complete.md`
- thesis is useful to its audience:
  `mostly Jörn-only`

Near-term packet candidates:

- Add a dedicated readability/usefulness packet only if agents can define
  concrete checks that save Jörn time before a human final read.
- Split `data-and-figures-are-traceable.md` only when thesis figures and
  tables grow enough that one packet becomes a bad read set.

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
- Write intermediate notes to `/tmp/` when the pass is
  large enough that you would otherwise lose state.

This skill is intended for ongoing verification work during the project, not
only a last-hours pre-submission sweep.
