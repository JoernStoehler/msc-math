---
name: session-resume-packet
description: >-
  Write a session resume packet for Jörn. Use when Jörn asks for a "session
  resume packet" or "resume packet", asks to resume/catch up/get status after
  being away, says he is switching away from or returning to a Codex session, or
  asks for a report in a session-switching or resumption context. Also use at
  the end of a nontrivial turn only when the session is explicitly entering an
  async/inactive state and Jörn will likely need to resume cold later. Do not
  use for ordinary experiment reports, durable research writeups, generic
  fresh-agent handoffs, live back-and-forth where Jörn is still engaged, live
  discussion where Jörn is still adding observations or calibration, ordinary
  single-question waits during synchronous work, or tiny completed tasks where
  Jörn needs no resumption help.
---

# Session Resume Packet

A session resume packet is what Jörn reads after switching back to a Codex
session. Its job is to make him able to resume with low effort.

Do not treat it as a normal report. It is not a chronological log, proof that
work happened, a dump of every true fact, a generic summary, or a handoff to an
unknown future agent.

## Apply Carefully

Write a session resume packet when Jörn asks for one, when he is leaving or
returning to the session, or when he asks to resume, catch up, get status, or
find where the session stands after being away.

If Jörn says "report" after a context switch, before switching away, or while
catching up on a session, read that as "session resume packet" unless another
report genre is clearly intended.

Proactively write a packet only when ending a nontrivial turn in an explicitly
async or inactive state where Jörn will likely need to resume cold later. Do not
turn every ordinary wait for one answer into a packet.

Do not switch into this skill just because the chat is awkward. If Jörn is
currently adding observations or thinking aloud, usually listen, acknowledge
compactly, or ask one clarification. Do not replace the live topic with
meta-recovery.

If the right next move is ordinary synchronous back-and-forth, or Jörn is
actively engaged and has not asked for a packet, continue the task instead of
stopping to write one.

## Context To Remember

Jörn often manages several sessions in parallel. When he returns, real time and
other work may have displaced this session from working memory. The transcript
exists, but prior chat, commentary updates, and your last message are not
automatically shared context.

The packet is for Jörn, not for a general reader. Optimize for his next useful
action, not for completeness, polish, or showing effort.

The scarce resource is Jörn attention. Include a fact only when it helps him
resume, answer a visible request, review a narrow surface, avoid a likely wrong
conclusion, or understand what will happen next.

## Before Writing

Decide what use case this packet serves:

- Jörn needs to answer one question or make one decision.
- Jörn needs to review a narrow artifact, claim, or diff.
- Jörn is returning cold and needs the current state.
- The session is complete and Jörn needs validation, residual risk, and whether
  any action is needed.
- The session should be abandoned or restarted elsewhere.

If none of these fits and no Jörn action or resumption risk exists, a session
resume packet is probably not needed.

Use scratch when useful. First identify possible Jörn actions, delete every ask
you can resolve yourself by local inspection or ordinary reasoning, then write
the smallest packet that makes the remaining action cheap.

Read `references/design-notes.md` only when updating, reviewing, or diagnosing
this skill or nearby AGENTS/chat-convention material. Do not load design notes
just to write an ordinary packet.

## Packet Contract

Make the session state and any request hard to miss. In the first few lines,
state the current status and either the Jörn action or that no Jörn action is
needed.

State:

- whether this is complete, continuing asynchronously, awaiting Jörn, blocked,
  or only a checkpoint;
- the current task in ordinary project terms;
- what changed since Jörn last needed to hold the context, if that matters;
- what Jörn needs to do now, or that no Jörn action is needed.

When asking Jörn for anything:

- put the request visibly, not hidden in prose;
- state the current agent-side model, default, or recommendation first;
- ask only for the smallest judgment, fact, decision, or review that only Jörn
  can supply;
- name the exact uncertainty and what changes after likely answers;
- give answer shapes when that makes answering cheaper, without pretending the
  answer space is closed when it is not.

When correcting or replacing an earlier bad message, do not send a mental
patch. Say what to ignore and restate the smallest complete replacement.

## Writing Guidance

Build the packet around Jörn's next action. If there is no Jörn action, say so
early; do not make him infer that silence means no action.

Assume Jörn is returning cold. Reload the smallest state needed for the next
action:

- objective in ordinary project terms;
- current status;
- the relevant distinction, formula, claim, or artifact;
- what changed since the last context Jörn likely held;
- why the requested answer, review, or action matters.

Do not make Jörn dereference code symbols when the question is mathematical,
architectural, or thesis-scope. Translate code names back into the controlling
idea.

Ask at the level where Jörn's answer changes the work:

- mathematical issue: ask for the proposition, intended formula, or error model;
- architecture issue: ask for the design constraint or ownership boundary;
- thesis-scope issue: ask for claim strength or review gate;
- verification issue: ask for the acceptance condition.

Implementation details belong only when they let Jörn inspect or answer the
higher-level question cheaply.

Give answer shapes when useful, but do not pretend the answer space is closed
when it is not. Prefer "My default is X because Y; the uncertainty is Z" over a
bare "Should I do X, Y, or Z?" when the real issue is the criterion.

When asking Jörn to review something, define the review surface:

- current review target;
- optional background;
- files, diffs, or transcript sections that are intentionally unnecessary.

Avoid listing paths without saying whether they are review targets, background,
provenance, or disposable scratch.

Filter facts by use. Usually omit git state, full command logs, broad file
lists, stale dead ends, facts that were merely recently salient, and validation
that does not change status or residual risk.

Before sending, check:

- Can Jörn tell in the first few lines whether he must act?
- Is every included fact used by the action, status, or resumption safety?
- Did you translate symbols/files into the controlling math, design, or thesis
  issue?
- Would this still work if Jörn remembered nothing from the last turn?

## Output Shape

For nontrivial packets, short headings usually help. Common parts:

- **State:** current status, task, and whether Jörn action is needed.
- **Jörn action:** the visible question, review request, action, or "No Jörn
  action needed."
- **Reload:** only the context needed to answer, review, or resume safely.
- **Evidence / validation:** only checks, files, or facts used by the request
  or status.
- **Next:** what you will do after Jörn answers, what happens if no answer is
  needed, or why you are stopping.

Do not include git status, file lists, command logs, or unrelated discoveries
unless they change Jörn's action or the safety of resuming.

Compact example, not a template:

```text
State: Awaiting Jörn review. The worktree contains a new session-resume-packet
skill plus one AGENTS.md bullet; validation passed.

Jörn action: Review only the skill trigger/contract and the AGENTS.md bullet.
design-notes.md is background unless the contract feels wrong.

Reload: The skill is for return-after-async cases, not ordinary experiment
reports, generic handoffs, or live discussion while Jörn is still engaged.
```
