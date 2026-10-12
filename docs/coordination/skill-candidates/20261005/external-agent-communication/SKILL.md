---
name: external-agent-communication
description: "Use for communication and work handoffs across independent agent contexts. Examples include exchanges between Codex, Claude Code and ChatGPT/Pro, user-relayed messages and returned work packages. Choose routes, preserve sufficient context and integrate replies. Standard native subagent-tree transport needs no additional protocol."
---

# Communication across independent contexts

Make findings, questions, assignments and results usable across contexts without losing intent, scope, authority or return paths. A routine update needs its message and delivery requirements, not a fresh research plan.

## Choose the recipient and route

Determine whether the exchange is information, a judgment request, an assignment or an ownership transfer. Supply missing sender, recipient, requested action and reply route; reuse transport metadata. A reply to the user does not send a message to an agent.

Compare local completion with sending work out when that choice is still open. Consider recipient capabilities/tools, necessary context, independence, interaction, user relay effort, latency, uncertain costs and integration together. Confirm consequential capabilities rather than infer them from product names.

| Situation | Route | What must be established |
| --- | --- | --- |
| Independent workstation/session exchange | Existing agent transport, session API/CLI or agreed mailbox | Addressable recipient, access, return route and whether sending can initiate work |
| Bounded independent Claude Code task | Installed CLI/session | Current help, authentication/model, scoped tools, permitted effects and substantive return |
| Human-mediated ChatGPT Pro research | Focused ZIP through Jörn and Taildrop | Sufficient dossier, actual Chromebook target and human submission/return instructions |
| Human relay needed | Separate copyable message through Jörn | Destination, requested action and reply path |

Native child messages/finals already use their tool contract. Do not add another transport protocol or administrative receipt layer for standard tree-internal communication. Use an actual external route when the native contract cannot reach an independent context.

## Supply sufficient context

An assignment carries outcome and reason, evidence pointers, confirmed choices, uncertainty, scope/authority, resources/stopping point, expected return, assessment and receiving/integrating owner. Carry exploratory questions as questions. A handoff adds or transfers work explicitly; it does not replace existing work or establish new authorization silently.

For independent comparison, give reviewers the same actual question and necessary material, preserve real alternatives and withhold earlier recommendations unless evaluating them is the assignment. Compare substance and limits; similar vocabulary does not establish the same solution.

## Independent sessions and Claude Code

Use available transport or an agreed mailbox. Read `docs/codex.md` and current tool help when addressing, prompting or wakeup behavior matters. Historical release-specific behavior is not a current guarantee. A saved file proves availability, not receipt or acceptance.

For Claude Code, inspect installed help and project access guidance, launch a bounded task with tools appropriate to its effects, and retain the substantive return. Read-only review needs no editing capability. Preserve user-selected model/effort; report the actual returned model where relevant rather than assume one. Startup success is not task completion.

If a route cannot perform the exchange, name the missing capability and use an available alternative. Ask Jörn to relay only when his action is required.

## ChatGPT Pro round trip

Jörn operates ChatGPT Pro. Prepare a focused ZIP with `GOAL.md` at its root and useful supporting material. A small mathematical dossier previously worked; do not include the whole repository or datasets by default.

An invoked Pro handoff authorizes the transfer without another confirmation. Discover the current Chromebook target rather than hardcoding an IP:

```sh
tailscale file cp --targets
tailscale file cp /absolute/path/task.zip <chromebook-target>:
```

Give Jörn the filename and established prompt:

> Prompt: <GOAL.md inside the zip>. Please work through this and report back with a single zip file that i will send back to the project agent. Thanks!

Previously reported 40–90-minute Astra Pro waits are observations, not a timer or guarantee. Continue independent authorized work where useful.

When Jörn says he sent the return, receive it into a fresh scratch directory:

```sh
mkdir -p /tmp/pro-return-unique
tailscale file get --verbose --conflict=rename /tmp/pro-return-unique
```

This drains the inbox; preserve unrelated arrivals. Retain the original return, independently assess consequential findings and integrate useful material into its ordinary project home. Scratch is not its only lasting owner.

## Relay, failures and integration

For human relay, explicitly ask Jörn to forward a separate copyable message with destination, action, sufficient context and reply route. Do not make him rewrite the brief. Omit administrative receipts to him unless he requested status or must act.

Claim submission, delivery, receipt, acceptance and completion only with evidence for each. Automatic acknowledgment does not establish user receipt. Preserve the material and pending request through channel changes. Failed understanding calls for context repair; failed delivery calls for transport repair. Repeating unverified sends does not establish progress.

Match returns to assignments, preserve consequential raw evidence, assess findings at the required strength and integrate into the receiving owner's work. Recover the parent objective and next action. Neither successful transmission nor a reviewed component closes the whole outcome. Preserve unresolved approval/resource boundaries through ownership changes.
