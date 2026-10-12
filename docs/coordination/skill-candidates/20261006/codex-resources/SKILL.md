---
name: codex-resources
description: Check Codex account quota, reset inventory and expiry; inspect scoped session usage and workload; reason about resource allocation; and carry out explicitly authorized redeem-now or scheduled reset work with recovery ownership. Use when those resource questions or actions are part of the task, not for ordinary Codex setup or every mention of tokens. Checking resources does not authorize consuming a reset or starting monitoring.
---

# Codex resources

Make resource decisions from the right observation and retain ownership of any selected action through its actual outcome. Account quota, session token accounting, billed spending and coordinator workload are different quantities. Missing observations stay unknown; a snapshot's age matters.

## Check the quantity the task needs

Run project commands from the assigned checkout root. Reuse a sufficiently current observation instead of repeating account reads without a decision need.

| Question | Existing route | What it establishes |
| --- | --- | --- |
| Current account quota, available reset count and returned expiry details | `python3 scripts/codex-quota-status.py` (`--json` for allowlisted metadata) | Timestamped server-reported account state; no model turn or redemption. Returned credit details may be capped. Reset eligibility remains unknown. |
| Usage recorded for this local thread tree | `python3 scripts/usage-awareness.py THREAD_UUID --json` | Local accounting with its recorded coverage/errors; ordinary forks are separate. Not account quota or billing. |
| Cumulative local attribution by root/workers/model | `python3 scripts/subtree-usage.py ROOT_THREAD_UUID` | Owned local response accounting under the script's contract. Any dated price estimate is not a quota conversion or a bill. |
| Workload observations for a selected execution window | Existing credential setup plus `python3 scripts/coordination-load.py ROOT_THREAD_UUID --minutes 15 --brief` | Scoped telemetry and advisory signals. Exit 2 means incomplete observation; quiet telemetry does not establish idle capacity. |

[Codex reference](../../../../../docs/codex.md) and [coordination contract](../../../../../docs/coordination/README.md) own prerequisites, accounting limits and scoped supervision. Do not print credentials or raw account/credit/operation identifiers. Use existing sign-in; an authentication error does not authorize another login grant.

Select the smallest useful observation. An account-quota question usually needs no full transcript accounting; a question about this assignment's cost cannot be answered by the account percentage alone. Record observation time, quantity, scope and material unknowns with the current coordination owner when they affect assignments or decisions.

## Use observations for allocation

Assess scientific/product value, the needed quality, parallel capacity, coordinator attention, elapsed time and actual authorization together. Token counts alone do not identify overload or optimal model selection. Preserve selected models, user spending boundaries and execution cutoffs. Unknown dollar/token thresholds do not create a new gate for every small task.

Use the existing selected supervision route where one is needed. Do not revive withdrawn automatic usage hooks, start permanent monitoring, consume a reset or launch more work merely because a report suggests capacity. When a task becomes materially more expensive, prepare the concrete options and expected outcomes needed for the existing spending decision.

A returned expiring credit can make an action decision urgent. Check whether redeem-now or scheduling is already authorized and identify the selected credit/account and deadline. Route that existing action without asking the same approval again. An inspection request alone does not select an action; present a specific consequential choice if the next step truly needs the user.

## Redeem an explicitly selected reset

Use the real installed CLI's `/usage` route, or an equivalent whose request and recovery behavior have been demonstrated for the intended environment. Discover the current UI/adapter; do not invent a noninteractive CLI command or treat a dated private script as the selected implementation.

Preserve the selected account and credit. The public quota helper intentionally omits opaque credit IDs; selection-dependent operations need a private access route with the actual identifier. Do not omit credit selection and let the backend choose another credit when a particular one was selected. If the available route cannot preserve the selection or perform the action, retain that capability gap visibly.

Before an attempt, identify its receiving/recovery owner and the authorized recovery scope. Keep the logical attempt identity in its private execution record. A lost response is an ambiguous outcome; reconcile it before declaring failure or using another attempt identity. Reuse the same operation key for retries of that ambiguous logical attempt. The investigation's local CLI mock established that reopening confirmation after a clear refusal can start a new attempt for the same credit; it did not establish that production will accept it. A new attempt must remain within the selected recovery authority.

Refresh account state after a confirmed consume outcome. The [official protocol](https://learn.chatgpt.com/docs/app-server#8-earned-rate-limit-resets-chatgpt) documents success, idempotent prior success, refusal and no-credit outcomes, and requires post-operation state refresh. These response categories do not by themselves explain a disputed production failure.

| Observation | Next disposition |
| --- | --- |
| Confirmed successful redemption, including an idempotent prior success | Refresh state, retain evidence and report the actual result. Do not infer new research authority from refreshed quota. |
| Clear refusal such as `nothingToReset` | Preserve the response and reconcile current state/context. Keep recovery ownership; investigate or use the selected fallback within authority. Do not treat the label as a complete causal explanation. |
| Ambiguous transport error or interrupted response | Reconcile whether the logical attempt completed; preserve/reuse its identity during retry. Do not double-consume while guessing. |
| No credit, expired selected credit, or unresolved failure at the actual deadline | Report the supported final disposition and release action ownership explicitly. Do not substitute another credit, buy capacity or generate dummy work. |

Backend defects, client/context differences and undocumented policy have not been distinguished for the retained October refusal. Minimum-usage advice was withdrawn. Do not manipulate workload to create eligibility or prefer automation claims over contradictory manual evidence.

## Schedule with a complete recovery route

Scheduling adds persistence and a later receiving owner to the same redemption workflow. Preserve the intended timezone, execution time, selected account/credit, expiry and authorized recovery window. Do not turn an expiry margin into a claimed recovery capability merely by choosing an early timestamp.

Select a durable execution mechanism that actually exists in the owning environment. Verify the saved job and its target before reporting it scheduled. State separately what is scheduled and what has not yet run. The receiver must have the CLI/adapter, diagnostics and authority needed for the fallback; a prompt limited to rerunning the failed script does not provide recovery. Confirm the notification/return route when it matters to the task.

A timer firing or a subprocess starting does not complete redemption. Keep ownership of unconfirmed outcomes through recovery, confirmed success or the credit's actual expiry. A chat-turn end or acknowledgment from another agent must not discard that ownership. Transfer it explicitly with the action state, pending obligations and return route when continuation needs another owner. Honor user stops and hard execution cutoffs: arrange an authorized receiver where needed, or report the missing continuation capability instead of promising coverage or overrunning the boundary.

**Implementation limit:** This candidate adds no scheduler, persistent controller or fallback executor. The dated October timers did not establish a working recovery loop. Inspect and verify an actual selected implementation before making a future-execution promise; missing persistent capability cannot be repaired by wording alone.

## Completion and maintenance

For a check, return the requested quantity, timestamp, scope and limits. For allocation, return the justified choice and actual pending decision, if any. For scheduling, report the verified job plus its receiving/recovery ownership without claiming redemption. For a due action, report confirmed redemption or the supported final failure/expiry disposition, retaining unresolved evidence and releasing ownership explicitly.

Read [design notes](references/design-notes.md) when maintaining this skill. They retain context and candidates, not additional runtime authority.
