---
name: cross-session-messaging
description: Use when sending or receiving cross-session agent messages or handoffs, including messages relayed by Jörn.
---

# Cross-session messaging

The recipient and delivery route must be clear. Use the actual agent transport
or an agreed mailbox. An ordinary reply to Jörn is shown to him; it does not
send a reply to another agent. Native subagent replies use the harness's route.

When sending, supply any missing sender, recipient, requested action and reply
route. Reuse metadata already supplied by transport; no mandatory header or
message ID is needed. If Jörn must relay a message, explicitly ask him to forward
a separate copyable message.

When receiving, establish who the message addresses and what it asks you to do.
Send any requested receipt through the agent reply route. In replies to Jörn,
omit administrative receipts and their paths/status unless he asked for that
status or needs to act. A handoff adds or transfers work explicitly; it does not
silently replace existing work or establish new user authorization.

Claim delivery, receipt, acceptance or completion only with evidence for that
state. Saving a file is not proof that the recipient read it. If a needed route
is unavailable, say so; ask Jörn for relay only when his action is required.
