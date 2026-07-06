# Session Resume Packet Design Notes

Read this only when updating, reviewing, or diagnosing this skill or nearby
AGENTS/chat-convention guidance. Do not read it just to write an ordinary
session resume packet.

This file records enough context from the session that created the skill for a
future agent to continue the design without rederiving the problem from chat.
It is not source truth for thesis mathematics or project status.

## Contents

- [Problem](#problem)
- [Design Split](#design-split)
- [Design Choices](#design-choices)
- [Evidence And Validation](#evidence-and-validation)
- [Future Update Guidance](#future-update-guidance)

## Problem

Jörn manages several Codex sessions in parallel. When he switches back, a
message called a "report" often fails to help him resume. Recurring failures:

- reports become logs, evidence dumps, status reports, generic summaries, or
  proof-of-effort;
- requests are hidden, indirect, too compressed, falsely finite, or ask Jörn to
  redo reasoning the agent could do;
- reports assume commentary, earlier final answers, code symbols, or transcript
  text are still in Jörn's working memory;
- agents send correction-as-patch messages and expect Jörn to mentally merge
  old and new text;
- recovery guidance can switch the topic into meta-repair and destroy the
  usefulness of Jörn's current working memory.

The chosen term is **session resume packet**. It names Jörn's action, not the
agent's output, and avoids the overloaded word "report." Short form:
**resume packet**.

## Design Split

The current skill covers two slices:

1. **Genre/context induction**: teach the agent what object it is writing and
   what the object is for.
2. **Production support**: give process knowledge that helps an agent write a
   useful packet once it understands the genre.

The live `SKILL.md` now holds both slices in condensed form. The first draft put
production support in `references/how-to-write.md`, but Jörn judged that this
created one extra active indirection after the trigger had already fired. Since
the skill itself says when not to use it, the added live guidance is only mildly
harmful in false-positive triggers and more useful in normal triggers.

Known adjacent work not solved here:

- The branch adds one compact AGENTS.md bullet for session-switching context.
  Tune that bullet if it over-triggers packet writing or fails to make agents
  model Jörn's multi-session workflow.
- A separate chat-quality rescue skill, if AGENTS.md remains too weak.
- Replacement or narrowing of `stalled-session-recovery`, which may be harmful
  because it can trigger meta-recovery during live discussion.
- Forward-testing this skill on more real transcripts and adding examples if
  agents still fail after reading it.

## Design Choices

- Trigger on "session resume packet" and "resume packet" directly.
- Treat "report" as this skill only when session switching or resumption
  context is present.
- Exclude live back-and-forth, live observation/calibration, generic handoffs,
  durable experiment reports, and tiny completed tasks.
- Make clear whether the session is waiting on Jörn. If it is not, state the
  autonomous default directly so Jörn does not have to infer whether the agent
  is continuing, stopping, or waiting.
- When Jörn explicitly asks for a resume packet, write the packet first instead
  of adding a "continue" turn. The packet should assume near-zero working
  memory and report the current state: requesting Jörn, continuing
  autonomously, complete, blocked, or abandon/restart.
- Require requests to include the agent's current model or recommendation when
  that saves Jörn from reconstructing reasoning.
- Add a review-surface rule because listing files without current/optional
  status can create a broad review burden.
- Include "replace, do not patch" because Jörn reported mental transcript
  patching as slow and costly.

## Evidence And Validation

Evidence came from Jörn's repeated observations in the creating session and
from the agent's own failures during that session:

- premature input-tool question with compressed options;
- premature `<proposed_plan>` while Jörn was still adding observations;
- conversion of observations into rules before Jörn requested synthesis;
- correction messages that assumed Jörn could mentally patch the transcript.

Non-durable draft artifacts were prepared in `/tmp` during the creating
session. They were useful for iteration but are not source truth for future
review.

A cold-reader subagent evaluated the sample and guide. It found:

- the original trigger was too broad around awaiting Jörn input;
- the sample did not make the review surface precise enough;
- the sample request was too optional and broad.

Those issues were revised before the repo skill was created.

## Future Update Guidance

If agents overproduce packets, tighten the trigger and add stronger non-trigger
language before adding more process.

If packets still look like logs or evidence dumps, add compact contrast examples
to `SKILL.md` or, if examples become bulky, create a reference file for examples
only.

If packets are good but Jörn still has to remember to ask for them, revisit the
AGENTS.md session-switching bullet and the skill metadata trigger.

If agents keep switching into meta-recovery during live discussion, revise or
remove `stalled-session-recovery` before creating another broad recovery skill.
