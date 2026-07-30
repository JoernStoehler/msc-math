---
name: session-resume-packet
description: Write Jörn a cold-resumption packet when he asks for one or returns after being away; also use when a nontrivial session explicitly becomes inactive and will likely resume later. Do not use for live discussion, ordinary reports, new-agent handoffs, or routine waits.
---

# Session Resume Packet

Jörn works across concurrent sessions. After switching away, assume other work
has displaced this chat from working memory. A packet should let him resume
safely without rereading the transcript; it is not a chronology, effort report,
or generic project summary.

State first whether the session is awaiting a Jörn-only answer/review/action,
continuing autonomously, complete, blocked, or being abandoned/restarted.

Include only what supports resumption:

- the full session objective and current focus;
- status within that scope and what materially changed;
- the controlling mathematical, architectural, or thesis distinction;
- the default continuation, or the exact blocker/request;
- evidence, validation, residual risk, and artifact links when they affect
  status or the requested judgment.

When waiting on Jörn, give the current recommendation and ask the smallest
question only he can answer. Explain what changes with the answer and identify
the exact review target. Otherwise say explicitly that the session is not
waiting on him.

Assume Jörn remembers nothing from the last turn, but omit command logs, broad
file lists, dead ends, and recently salient facts that do not change his next
action. Translate code symbols into the controlling project issue. An immediate
next command can be an execution cursor, but cannot replace objective, status,
and reason.

Resolve temporal references for the later reader. Identify mutable state and
what must be rechecked only when drift could change the continuation; otherwise
avoid unnecessary dates, commits, and status metadata. Do not use references
such as “this session” or “current draft” when they could point to a different
session or artifact at resumption time.

When replacing an inadequate earlier message, make the packet self-contained;
do not ask Jörn to mentally patch the old message with scattered corrections.

Read `references/design-notes.md` when updating, reviewing, or diagnosing this
skill or session-resume-related AGENTS/chat-convention guidance.
