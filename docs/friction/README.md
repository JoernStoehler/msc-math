# Collaboration friction

This directory owns detailed friction tracking for the msc-math collaboration workflow.
The [Pages overview](https://chatgpt.com/space/page_9d9e95b76fbc81919da91c9c8370e5e3) is Jörn's concise view for understanding current friction and discussing what to address.
Agents maintain the records here and refresh that view during relevant work; Jörn does not need to inspect these files or request routine reconciliation.
This is coordination work, not permission to restart the paused thesis completion run.

## Records

Each ID has one canonical file. Read only relevant records; preserve IDs and history when status changes.

- [F01 Prior conversation access](F01.md)
- [F02 Broken source reference](F02.md)
- [F03 Conflicting repository handoff](F03.md)
- [F04 Visibility of active sessions](F04.md)
- [F05 Claims and expiry in Page tables](F05.md)
- [F06 Launching work from a Page](F06.md)
- [F07 Rendered Page preview](F07.md)
- [F08 Permission inspection redirect](F08.md)
- [F09 Rejected hub edit](F09.md)
- [F10 Truncated tool output](F10.md)
- [F11 Delay before the interaction test](F11.md)
- [F12 Attachment unexpectedly available on retry](F12.md)
- [F13 Prompt launch ergonomics](F13.md)
- [F14 Concurrent updates to the friction table](F14.md)
- [F15 Duplicated session reports and unassigned follow-up](F15.md)

[Trial evidence and open capability questions](trial.md) retains the trial results, procedures and decision criteria imported from the Page.

## Status and evidence

- `unresolved`: friction remains, or closure is unconfirmed. A suggested or unverified workaround does not close it.
- `no_longer_relevant`: the affected need or situation no longer applies; record why and what would make it relevant again.
- `resolved`: the observed friction is gone; record the action or observation that establishes this.
- `mitigated`: friction is reduced with residual limits or dependence on a workaround.

For resolved or mitigated records, `outcome_kind` distinguishes `deliberate_change`, `workaround` and `unknown_cause`. Leave the cause unknown when disappearance is observed without an explanation. Append dated evidence when reopening or reclassifying a record; do not erase the earlier incident or transfer an old result to a new artifact.
Capture the reporting surface, attempted action, symptom, effect, measured retries/time when available, handling, cause/confidence and next discriminating test. Untested capabilities remain questions in trial.md, not observed failures.

## Maintenance and the human view

When authorized work notices or changes consequential friction, update the relevant record on current published main. Use the next unused F number for a genuinely new issue; check current filenames before allocating it. Repeated events reuse the existing ID. GitHub SHA guards or normal Git conflict handling protect updates; these records do not provide atomic task leases.

The agent making the change owns reconciliation of the Pages overview, without a separate request from Jörn. Keep the overview short: material unresolved problems, their effect and a useful possible response; a compact account of mitigated/resolved and irrelevant items; decisions or help actually needed from Jörn. Keep detailed events, diagnostics, trial procedures and agent handoffs here. Link to evidence rather than copying the log.

Treat Jörn's comments and conversations as inputs. Record resulting decisions or corrections here and update the overview during that authorized work. Do not invent an acknowledgement task or require Jörn to launch a session merely to confirm an update.

If a session cannot write the Page, keep the canonical record accurate and report the overview as stale in its handoff, naming the affected IDs. If repository writes are unavailable, report the unsaved facts and affected records; do not silently turn the Page into a competing detailed log. This contract operates during agent turns; it does not establish a continuously running monitor or schedule.

## Migration provenance

Imported 30 September 2026 from the existing Page's fifteen records, including concurrent session observations. Private session/comment identifiers were omitted from public files; the existing Page and hub retain their access controls. Existing trial evidence is a dated snapshot, with its uncertainties preserved. Public repository content is limited to operational observations and project coordination.

Jörn chose Pages for persistent human-facing views and GitHub for durable agent workspace content. No custom plugin, thesis work or automation is launched by this migration.
