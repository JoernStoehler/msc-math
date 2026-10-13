# Async request_user_input source review

**Pinned source:** `benchmark-run/shallow`, tag `rust-v0.162.1`, commit `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`. Excerpts and hashes are listed in `evidence/question-source/provenance.json`.

## Source observations

- `request_user_input_async` is a fire-and-forget tool at the model-turn level: its handler emits a completed async-delivery agent item carrying questions and returns `{"accepted":true}`. It does not suspend the turn awaiting the user's answer (`async-question-handler.rs`). The TUI then attaches these questions to a retained editor keyed by the message/item ID.
- A normal prompt send clears all pending questions after `submit_op` accepts the operation (`input_submission-prompt-clear.rs`). The queued-input path also clears pending questions when a prompt is queued behind ongoing work (`input_submission-direct-clear.rs`'s neighboring source path is corroborated by `input_flow.rs` behavior; full excerpt available in the pinned source). The focused test `ordinary_follow_up_clears_unanswered_questions_after_accepted_input` explicitly expects unanswered questions to disappear for both direct and queued follow-ups (`question-steer-test.rs`). This is the relevant source-level behavior behind the reported “steer clears queue” symptom. It describes local TUI pending UI/editor state; the excerpt does not show deletion of the already-emitted transcript item.
- Answers are removed on explicit resolution: accepted local answer/skip removes the current question; a committed user message containing an async-question reply ID resolves matching question IDs (including legacy whole-message IDs) (`async-question-clear-resolve.rs`, `async-question-replay-resolution.rs`). The async tool itself reports acceptance before any such answer.
- New collapsed questions receive a 30-second `expires_at` used by the remaining-time/countdown UI. Expanding or interacting snoozes that timer by setting expiry to `None` (`async-question-state.rs`). The inspected code does not show timeout-based answer submission or timeout-based removal. Thus the timer is not evidence that the user answered or consented.
- Thread reset explicitly clears pending questions. The examined source shows no generalized cancellation/expiry event that turns an unanswered question into an answer. A user steering message is a separate accepted prompt and currently clears pending question state.

## Consumer implications

For a behavior that asks the user questions before starting dependent work, treat this async tool as a notification/editor mechanism, not a blocking question/answer primitive. A separate `request_user_input` tool can wait inside the active turn in Plan mode when its feature is enabled; see `human-input-source-review.md` for mode, auto-resolution, and notification conditions. Keep dependent work paused until an actual answer arrives. A steer can clear the TUI's pending async-question queue, so keep the unresolved question and dependency in durable turn context and surface it again when appropriate; do not infer an answer from the steer, a timer, or an empty auto-resolution. Conversely, optional questions should not block unrelated work. If a decision is a hard prerequisite, make that dependency explicit and pause only the dependent action.

## User report and verification boundary

The user reports that latest TUI steering clears the async queue and commentary scrolls offscreen. Queue clearing is consistent with the pinned source and the cited test. The commentary-scroll observation is recorded as user-reported only; this review did not trace or run the terminal rendering path. No inference, runtime, or live TUI test was run, so neither the complete runtime behavior nor commentary scroll behavior is verified here.
