---
name: input-and-goal-recovery
description: Preserve required user-input dependencies through steering and diagnose Codex goal activation, continuation, and notification behavior.
---

# Required input and goal recovery

Use this guide when a task has a required missing fact or when a Codex `/goal` loop is proposed or already active. It gives agent procedure; it cannot change client UI delivery, tool policy, or runtime continuation.

## Required answers

- If the user asks to be questioned before work, ask consequential required questions first and pause actions that depend on their answers. Continue only independent inspection or preparation.
- A successful async question call means delivery was accepted, not that the user answered or that execution paused. Record the missing answer and the decision it gates.
- After every steer, reconcile it against each outstanding question. A style instruction such as “keep the report concise” does not answer a deployment-target question. If still required, repeat the self-contained question in a visible supported route; do not rely on vanished pending UI, silence, elapsed time, or an unrelated message.
- Optional preferences may remain async while independent work proceeds. State a bounded fallback and do not let optional silence block unrelated work.
- Never guess a required target, treat silence as permission, or mark work that depends on it complete.

## Codex goal loops

- Activate the Codex goal feature only when the user explicitly requests it or clearly invokes `/goal`. A plain heading such as `Goal: migrate the pipeline` is not an invocation. If the agent called `create_goal` after such a heading, investigate the caller/configuration; do not attribute it to slash parsing without evidence.
- A goal loop is not a workaround for missing required input. Preserve the dependency and use a user-facing question route available in the actual session. In pinned source `rust-v0.162.1`, `request_user_input` has no active-goal exclusion: when exposed and the current mode is Plan, it raises a `PlanModePrompt`, blocks the composer, and awaits a response. The tool is feature-gated. In Default mode it is available only with an additional feature flag and may auto-resolve after a grace period; an empty response is not consent. Inspect actual tool exposure, mode, and notification settings before choosing. Do not assume either route is available.
- In the inspected Codex source at `rust-v0.162.1`, an active goal can continue from the idle hook, and the normal end-of-turn notification gate excludes active goals. Goal guidance requires the same hard blocker for three consecutive turns before `blocked`; `paused` is reserved for explicit user request. These are version-scoped source findings, not a guarantee about another client/version.
- Available user controls include `/goal pause` and interrupt. Do not tell the model to self-call a pause/status transition that its active tool policy prohibits. Do not invent tool-description/catalog overrides: inspect effective schema and startup config, and recognize that descriptions do not alter enforcement or notification gates.
- If no available question route obtains the required answer, record the exact client/version/configuration and escalate that specific defect to the harness/runtime owner. Agent instructions cannot guarantee runtime delivery. Do not claim a repository edit changed runtime behavior.

## Recovery record

For each required input, retain the question, gated action, whether an answer arrived, and the visible route used to ask. For goal incidents, separately record activation evidence, loop/notification evidence, source/version, user controls, and what was not reproduced. Stop dependent work while the answer is missing; make no completion claim for the blocked objective.
