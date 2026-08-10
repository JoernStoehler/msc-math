---
name: ask-joern
description: Send a one-way project inbox message to Jörn while continuing agent work. Use when Jörn needs to review an artifact or plan, answer a question, make a private or stakeholder decision, or perform another human-only action and useful agent-doable work can continue before his response. Jörn reads these messages in codex-gui and replies as a normal user message in the owning Codex thread. Do not use when his response blocks all remaining work or merely to avoid ending the current turn.
---

# Ask Jörn

Send one self-contained Markdown message with the bundled CLI. The inbox is a
one-way delivery channel: Jörn's response arrives as a normal user message in
the owning Codex root thread, not in the inbox message. Jörn independently uses
read/unread plus `open`, `in_progress`, and `archived` to organize his inbox;
`archived` is not itself a response or approval.

## Send

Write the message body to a Markdown file, then run this exact command from the
repository root:

```bash
node .agents/skills/ask-joern/scripts/mail.mjs send \
  --body <body.md> \
  --title "<concise inbox title>"
```

Both options are required. The command reads `$CODEX_THREAD_ID` itself,
submits the message once, and prints its generated ID. A failed submission
returns nonzero: report the failure and do not retry it automatically.

Write a concise, self-contained request that says what Jörn should do and
provides the exact artifact, question, or decision. Use project-root-relative
Markdown links for project artifacts. Do not repeat the title as a Markdown
heading.

After sending, continue useful independent work. Do not poll continuously.

## Inspect

At a natural checkpoint, return only the current handling status:

```bash
node .agents/skills/ask-joern/scripts/mail.mjs status <message-id>
```

Use `list` to recover messages sent by the current thread and `show <id>` to
inspect one complete message. Normally consume Jörn's thread reply instead.
Never infer an answer from inbox status.

## Jörn's workflow

Jörn sees the title, preview, age, read state, and handling state in the
project's codex-gui inbox. Opening a new message marks it read. He can then mark
it in progress or archive it, or choose **Reply in thread** to open the owning
thread's normal composer. Sending a reply never changes the inbox state, and
changing inbox state never sends a thread message. The usual lifecycle is
`open` unread → `open` read → `in_progress` read → `archived`.
