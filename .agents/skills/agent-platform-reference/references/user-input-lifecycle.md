# User questions during active work

Consult this when a user asks for questions before execution, or when a long-running task depends on a user answer. The relevant fact is whether the user can answer the active question and whether dependent work remains paused—not whether a question-shaped message was emitted.

## Required input

If the user explicitly asks for questions before starting, ask the consequential questions and stop before dependent execution until answers arrive. Inspect only enough context to form useful questions. Use a supported blocking question mechanism, or end the turn with a clear self-contained question. Commentary or a fire-and-forget async question is not evidence of a pause, user visibility, or an answer.

Track each unresolved question together with the decision/action it gates. A user steering message can modify the task without answering the question. Reconcile that message against the outstanding question; if still needed, keep the dependent action paused and present the current question again through a visible supported route. Never treat a disappeared UI item, silence, elapsed time or an unrelated steer as an answer or permission.

## Optional input

Optional preferences can be asked asynchronously while independent work continues. State a bounded fallback where appropriate. After a steer, check whether the question is answered, superseded, canceled or still needed; don't repeatedly ask a superseded question. Optional silence may justify an explicitly permitted fallback, not invented approval. Keep required input and optional preference questions distinct.

## Codex TUI evidence

At source tag `rust-v0.162.1`, `request_user_input_async` returns `accepted:true` after emitting an async-delivery item; it does not await an answer. Accepted ordinary prompts clear pending question UI state, including queued input; the pinned source includes a focused test for this. This does not prove the transcript item was deleted. See `evidence/question-source-review.md` and hashed source excerpts.

Jörn reported on 2026-10-13 that a workstation TUI steer clears the async queue and moves commentary outside the visible screen. The queue behavior is corroborated by source inspection; the rendering/notification experience was not independently run here. Do not generalize it to all clients or versions. In that surface, choose a question route that actually reaches the user and persists the unresolved dependency after steering.

## Goal loops and notification boundaries

An ordinary heading such as “Goal: …” is not evidence the user invoked the `/goal` product feature. Decide whether the feature is requested and suitable before enabling it. A loop is unsuitable as a way to avoid asking for missing required information.

If a running goal reaches a required-input blocker, preserve the unanswered dependency and diagnose the current status/notification controls. Do not invent an answer or mark unfinished work complete to escape the loop. In the inspected Codex source, model-initiated `paused` is prohibited without an explicit user request and `blocked` has a three-consecutive-turn audit threshold. Separate question notifications and a feature-enabled Plan-mode blocking question route can operate while a goal is active; inspect these before deciding the goal must stop. User `/goal pause` or an interrupt is an exposed immediate escape; a harness change may be necessary for reliable first-blocker notification. See [goal-loops.md](goal-loops.md) for the mechanism and alternative maintained fixes. Don't tell the model to use a pause operation its current tool policy forbids.

The user reported a Codex TUI loop suppressing end-turn pings while active and continuing despite required missing information. This is a reported failure, not an independently replayed UI observation. The pinned source/available controls are reviewed separately in `evidence/goal-source-review.md`; do not invent a stop command or assume Claude Code `/goal` has identical behavior.
