---
name: stalled-session-recovery
description: >-
  Recover derailed chat sessions with Jörn. Use when chat is causing repair
  overhead instead of helping the work progress, especially after repeated
  corrections, confusion, missed requests, or Jörn saying the interaction is
  stuck, looping, or wasting attention. Do not use for slow commands or hard
  code/proof/search attempts while useful progress is still happening.
---

# Stalled Session Recovery

Use this skill to recover chat interaction, not to solve the underlying task in
a special mode. Recovery succeeds when the session can return to productive
work that conforms to `AGENTS.md` guidelines.

## First Move

- Reread `AGENTS.md`.
- Before sending another chat message, first reorganize the session state in
  scratch so the next message is based on the recovered state, not on the last
  confusing turn.
- Use separate `/tmp` scratch files for separate concerns such as recap,
  usefulness/context, cruxes/beliefs, and question drafts.
- Write the scratch files for yourself. Do not paste long recovery files to
  Jörn; use normal polished chat for information transfer with Jörn.

## Rebuild Context

- Recap what changed about the state, not old reasoning in detail. Include
  relevant repo state, `/tmp` state, decisions, confirmations, questions asked
  or missed, and claims/constraints learned during chat.
- Treat past agent thoughts as weak evidence, not source truth. Reason anew
  from observations, Jörn's messages, repo state, scratch artifacts, and source
  truth.
- Reorient on thesis success. Compare the current work, next milestones, and
  candidate goals by expected contribution to thesis success, including
  opportunity cost. Identify which high-level or low-level plan cruxes matter
  now.
- Extract beliefs, assumptions, inferred constraints, uncertainties, and cruxes.
  Treat them as weaker than usual when the session has already shown failure.

## Recover With Jörn

- If many beliefs may be stale or misinterpreted, first reduce them locally to
  the smallest set whose truth would change the work and cannot be checked by
  repo inspection or ordinary reasoning. Only then ask Jörn to mark each item
  as either "settled enough" or "needs further discussion". Silence is not an
  answer. Discuss marked items afterward.
- For open questions where the answer space is not a claim list, use normal
  focused discussion with enough context. Give options when they help, and leave
  room for "other" when the space is not closed.
- Use dependency structure to reduce Jörn's work. Ask the easier or more
  informative questions first.
- Asking for permission is usually low-value and high-cost compared to asking
  about the underlying uncertainties.
- Preserve the narrow facts already known.
- State the uncertainty that controls the next step.
- Ask about that uncertainty when it matters.
- During recovery, shift toward re-asking visible unanswered requests instead of
  treating silence or omission as evidence.
- Make requests and assumptions visible.

## Recover Unmerged Work

When the stalled surface includes substantial unmerged or unapproved work, do
not treat the whole messy branch, diff, scratch set, or discussion as the next
review unit by default. First identify whether progress can be recovered by
extracting a smaller natural checkpoint:

- Identify a coherent packet that is already nearly finished and likely
  reviewable after bounded completion work.
- Selectively pick from the existing labor and add only the extra work needed to
  make that packet complete, source-backed, and easy to review.
- Keep the packet large enough to be worth review, but small enough that its
  purpose, because-clause, validation, and remaining risks are legible.
- Leave the rest of the messy work unmerged. Iterate with further focused
  checkpoints only while each next packet is similarly natural and close to
  mergeable.
- Once the remainder is incomplete, low-confidence, or no longer close to a
  reviewable packet, continue autonomously only where the next step is clear.
  Ask Jörn only for the missing judgment that blocks a useful recovery path; do
  not ask him to steer merely because several agent-doable branches exist.

Do not use this as permission to perform another broad branch transformation.
It applies only when there is a nearly finished checkpoint to extract; if the
work is too contaminated or only tiny artificial slices are available, stop and
plan locally, or ask for the missing judgment only when local planning cannot
resolve the next useful recovery path.

## Observed High-Cost Failure Modes

Jörn reported these as high-cost failures. Use them as examples of what this
skill is meant to interrupt, not as an exhaustive list. Future skill updates
should distinguish observed failures from hypothesized failures.

- **Agent stops driving the session.** All progress is steered by Jörn; there is
  no pull from the agent side. Questions drop to zero or collapse into
  permission/ownership questions such as "should I edit this file now?" The
  agent is not communicating what it knows, what it knows it does not know,
  uncertainties, assumptions, or cruxes. Jörn has to give feedback on the whole
  session surface, much of which GPT-5.5 could have generated itself.
- **Agent misinterprets Jörn and does not correct.** Local feedback becomes a
  broad hard constraint, such as treating "do not do the experiment this way" as
  "do not do the experiment at all." Preserve the narrower statement; mark the
  broader interpretation as an assumption unless confirmed.
- **Agent takes silence as an answer.** A question was hidden in a long message,
  phrased as a statement, or otherwise not made visible; Jörn answered other
  points; the agent treats non-response as a constraint or confirmation. Re-ask
  visibly if the answer matters.
- **Agent spends a whole turn on what went wrong.** An apology or diagnosis
  without a changed action, feedback request, or downstream use burns attention.
  If failure analysis is worth saying, attach it to a recovery action or
  targeted request.
- **Agent lets a broad unapproved work surface stay broad.** A messy branch or
  accumulated scratch work becomes too large to review, and the agent alternates
  between asking Jörn to judge the whole surface and proposing another broad
  transformation. Prefer extracting and completing a natural, high-confidence
  checkpoint when one exists, so progress is made and the remaining stalled
  surface shrinks.
