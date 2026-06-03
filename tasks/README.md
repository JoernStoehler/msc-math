# Tasks

Live task files:

- `definition-of-success.md`: success conditions and acceptance authority.
- `current-state.md`: current state, source paths, refresh triggers.
- `planning-notes.md`: route choices, rejected routes, invalidation triggers.

Dated references:

- `references/*.md` stores only dated material whose expected future value is
  high enough to justify future reading and staleness costs.
- Dated references are not the live planning layer and are not source truth.
- Prefer deleting or not committing transient planning reports after extracting
  durable consequences. Git history is the normal quarantine.
- Use retained dated references only to avoid expensive rederivation or
  repeated Jörn feedback, then refresh affected claims from source files and the
  live task files before executing work or making stronger claims.

Old topic task files are not live authorities. Do not use them to choose or
prioritize work. They were deleted during this migration; use git history for
historical prose.

Before selecting work that can affect thesis success:

1. Read this file.
2. Read `definition-of-success.md`.
3. Read `current-state.md`.
4. Read only the relevant sections of `planning-notes.md`.

Accept work only when it can change thesis-success state. Otherwise classify it
as optional/future or reject it.

Exploration is acceptable when it names the thesis claim, gate, or source
surface it can change, has bounded cost and a stop condition, and records any
thesis-relevant result in a durable source file.

Prompt handoffs and executable worker packets go in `/tmp`.

Roadmap reasoning does not go in `/tmp`. If a future packet, packet ordering,
cost/value argument, stop condition, or cut/repair decision should be visible
to many later agents, record the durable part in `planning-notes.md`. Use
`/tmp` only when a packet is ready to be handed to one deliberately spawned
fresh agent, or for that agent's scratch report before durable consequences are
copied back to source files.

After a session, update only files whose current claim changed:

- `definition-of-success.md` when success criteria, authority, or required gates
  changed.
- `current-state.md` when source-backed current state or a refresh trigger
  changed.
- `planning-notes.md` when route reasoning, rejected routes, or invalidation
  conditions changed.

Do not edit these files merely to record that work happened.

Keep a fact only if it changes a future decision, prevents a likely agent
mistake, records Jörn/Kai/external assessment, or gives a concrete resume/check
condition.

Source truth stays in thesis files, code, tests, data, research notes,
experiment artifacts, official forms, and accepted Jörn/Kai decisions. Task
files may summarize those surfaces; summaries never overrule source truth.

Use source/cache/refresh wording when a stale or overtrusted summary could
change proof strength, experiment status, code promises, thesis readiness, or
Jörn/Kai decisions. Otherwise use plain prose and paths that let a future agent
refresh the claim cheaply.
