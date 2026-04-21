---
name: verification
description: Verification workflow for final thesis-done checks and ongoing readiness passes. Use when asked whether claims, repo promises, code/data outputs, figures, or experiment surfaces are supported, complete, reproducible, truthful, or ready, or when asked what verification gates exist.
---

# Verification

This skill routes verification work between the repo's authority surfaces.

## Authority Split

- `FINAL-VERIFICATION.md` is the authoritative final finished-state spec.
- `TASKS.md` owns milestones, sequencing, and ownership.
- `RESULTS.md` owns the intended thesis claim surface.
- `references/*.md` in this skill own reusable operational verification passes.
- `references/index.md` is the coverage map for which final-tree areas already
  have runnable packets and which ones still borrow a nearest packet.

Do not promote a pre-final readiness check into `FINAL-VERIFICATION.md` unless
Jörn explicitly wants it to become part of thesis-done.

## Default Workflow

1. Decide whether the user is asking about:
   - final thesis/project done conditions;
   - a reusable pre-final readiness pass; or
   - current status of an open verification surface.
2. Load only the minimum authority surface first:
   - final done question -> `FINAL-VERIFICATION.md`
   - claim-support or overstatement question ->
     `references/thesis-claims-are-supported.md`
   - repo-promise or reproducibility question ->
     `references/repo-promises-are-truthful.md`
   - milestone/ownership question -> `TASKS.md`
3. If no exact packet is obvious, read `references/index.md` first.
4. If packet discovery is still unclear, list the available packet files:
   `ls .agents/skills/verification/references`
5. Packet files in `references/` intentionally use statement-form names such as
   `x-are-y.md`. Treat that as a signal that the file owns a real reusable
   verification condition, not scratch notes or generic background prose. The
   only intentional exception is `references/index.md`.
6. If the dedicated packet is missing, use the nearest authority surface and
   state the gap explicitly instead of inventing a new packet in the reply.
7. Report findings first. Separate:
   - supported/pass;
   - caveat needed;
   - missing support or stale evidence;
   - Jörn-only judgment.

## Packet Selection

- **Thesis claim support**: load
  `references/thesis-claims-are-supported.md`.
- **Repo promises and reproducibility**: load
  `references/repo-promises-are-truthful.md`.
- **Literal thesis/project done**: load `FINAL-VERIFICATION.md` directly.

If a task sounds close to an existing condition but does not match exactly,
prefer the nearest packet with the smallest starter read set and state the
missing packet boundary explicitly in the reply.

This skill is intended for ongoing verification work during the project, not
only a last-hours pre-submission sweep.
