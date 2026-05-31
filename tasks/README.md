# Tasks

Live task files:

- `definition-of-success.md`: success conditions and acceptance authority.
- `current-state.md`: current state, source paths, refresh triggers.
- `planning-notes.md`: route choices, rejected routes, invalidation triggers.

<!--
Migration-review note: keeping stale topic files in the live tree is cheaper
than deletion short-term, but it risks recreating a second task layer. Benefit
of deletion: future agents see one task model. Cost: old prose is less visible;
the old files were deleted after the carry-over gates passed.
-->
Old topic task files are not live authorities. Do not use them to choose or
prioritize work. They were deleted during this migration; use git history for
historical prose.

<!--
Migration-review note: read order costs a few minutes per substantial session.
Benefit: it forces agents to start from success and current state before
following route notes or old packet shapes.
-->
Before selecting work that can affect thesis success:

1. Read this file.
2. Read `definition-of-success.md`.
3. Read `current-state.md`.
4. Read only the relevant sections of `planning-notes.md`.

<!--
Migration-review note: old task rows and executable packets are not enough
evidence of value. Cost: this rule can slow obvious work and reject exploration
too early. Benefit: it blocks high-cost busywork before agents spend thesis
calendar or Jörn review time on it. Remove this comment after migration if the
visible rule carries that tradeoff during the scheduled comment cleanup.
-->
Accept work only when it can change thesis-success state. Otherwise classify it
as optional/future or reject it.

Exploration is acceptable when it names the thesis claim, gate, or source
surface it can change, has bounded cost and a stop condition, and records any
thesis-relevant result in a durable source file.

<!--
Migration-review note: durable prompt queues become stale and get executed
after their assumptions expire. Cost: `/tmp` prompts are not durable. Benefit:
only changed success/state/reasoning survives in repo.
-->
Prompt handoffs and executable worker packets go in `/tmp`.

<!--
Migration-review note: agents otherwise append progress logs and recreate a
tracker. Cost: some work leaves no task-file trace. Benefit: durable files stay
about changed claims, not activity history.
-->
After a session, update only files whose current claim changed:

- `definition-of-success.md` when success criteria, authority, or required gates
  changed.
- `current-state.md` when source-backed current state or a refresh trigger
  changed.
- `planning-notes.md` when route reasoning, rejected routes, or invalidation
  conditions changed.

Do not edit these files merely to record that work happened.

<!--
Migration-review note: this is the old task-retention rule in shorter form.
Cost: some context is deleted. Benefit: future agents read decision-relevant
state instead of a large historical tracker.
-->
Keep a fact only if it changes a future decision, prevents a likely agent
mistake, records Jörn/Kai/external assessment, or gives a concrete resume/check
condition.

<!--
Migration-review note: summaries are useful but dangerous when they become
authority. This rule preserves fast navigation without letting caches overrule
proof/code/data/forms/Jörn/Kai.
-->
Source truth stays in thesis files, code, tests, data, research notes,
experiment artifacts, official forms, and accepted Jörn/Kai decisions. Task
files may summarize those surfaces; summaries never overrule source truth.

Use source/cache/refresh wording when a stale or overtrusted summary could
change proof strength, experiment status, code promises, thesis readiness, or
Jörn/Kai decisions. Otherwise use plain prose and paths that let a future agent
refresh the claim cheaply.
