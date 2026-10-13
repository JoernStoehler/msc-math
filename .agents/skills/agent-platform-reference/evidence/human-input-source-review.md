# Human-input routes while a goal is active

**Pinned source:** `benchmark-run/shallow`, tag `rust-v0.162.1`, commit `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`. Relevant exact source snippets and source/excerpt SHA-256 hashes are in `evidence/human-input-source/provenance.json`.

## The completion alert is only one notification route

The ordinary `AgentTurnComplete` notification is suppressed while the TUI still sees an active goal (`goal-source-review.md`, `turn-complete-notification-gate.rs`). That is not a blanket suppression of human-attention events:

- When a live async-question agent item completes, the streaming path calls `add_async_questions`; adding unanswered questions independently calls `notify(Notification::AsyncQuestion { .. })` (`async-question-item-route.rs`, `async-question-notification.rs`).
- When a `request_user_input` tool request reaches the TUI, its handler independently calls `notify(Notification::PlanModePrompt { .. })` and opens the input overlay (`blocking-question-notification.rs`).
- Notification delivery is subject to the user's notification settings (globally enabled or custom allow-list); the accepted type names include `async-question` and `plan-mode-prompt`. When allowed, the TUI posts whichever pending notification it holds (`notification-delivery-settings.rs`, `notification-types-and-settings.rs`). Thus an alert can occur before end-of-turn, subject to those settings. The alert itself does not prove that the model is waiting for a human reply.

## Blocking `request_user_input` availability and semantics

`request_user_input` is registered only when `experimental_request_user_input_enabled` is true. Its available modes are Plan, plus Default only when `DefaultModeRequestUserInput` is enabled (`request-user-input-feature-gate.rs`, `request-user-input-mode-availability.rs`). There is no Goal `ModeKind`; the handler checks the current collaboration mode and root-thread status, and the inspected registration/handler paths contain no separate active-goal exclusion (`mode-blocking-semantics.rs`, `request-user-input-blocking-and-await.rs`). Therefore an active goal does not, by itself, rule out this tool in the pinned source, provided the goal's current turn is in an allowed collaboration mode and the feature is enabled.

The handler sets `is_blocking` exactly when current mode is Plan, sends the request, and awaits the session response (`request-user-input-blocking-and-await.rs`, `request-user-input-response-wait.rs`). The TUI's request overlay disables the main composer with “Answer the questions to continue.” When `is_blocking` is false, the overlay has a fixed auto-resolution grace/countdown policy; blocking requests disable that timer (`blocking-overlay-pauses-main-input.rs`, `default-request-auto-resolution.rs`). So Plan mode provides a genuinely blocking user-input route. Default mode can expose the tool only behind its feature flag and is explicitly nonblocking at the request UI level because it may auto-resolve. For a required answer, an agent must not interpret that automatic empty/no-answer resolution as user consent.

`request_user_input_async` is distinct. It emits a completed async-delivery item and returns accepted, so it does not hold the goal turn open. Its TUI question notification is a useful attention signal, not a blocking gate. The earlier goal report should therefore be read narrowly: active goals suppress the normal completion notification, but source includes separate question alerts and a blocking question path in Plan mode.

## What this establishes and what it does not

The pinned source contains ways to alert the user during an active goal and, in Plan mode when enabled, wait inside the turn for an answer. That weakens the claim that asking necessarily requires ending the goal/turn as a general source fact. Whether the workstation session exposed those routes depends on the live build/config, current collaboration mode, selected model tool availability, notification preferences, and how the model chose to ask; none were inspected here. This remains user-reported behavior plus pinned source analysis, not a reproduction. A correct agent behavior for a hard prerequisite is to use the genuinely blocking route when it is available, keep dependency-specific work paused until an actual answer, and never treat an auto-resolved empty answer or async notification as consent. Optional questions can still remain async so unrelated goal work continues.
