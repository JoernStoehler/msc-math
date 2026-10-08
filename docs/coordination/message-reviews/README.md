# Prepared-message approval evidence

[Root instructions](../../../AGENTS.md#review-prepared-messages-before-sending) own the trigger; [message-review](../../../.agents/skills/message-review/SKILL.md) owns the reviewer workflow and receipt contract. Keep candidate text, relevant user context and reviewer-authored receipts together under dated packet directories. Native private reviews do not recursively require another review.

The sender cross-checks reviewer identity against the runtime and runs the repository verifier before delivering the exact approved text. Neither a locally writable receipt nor a passing checker authenticates its author or prevents the sender bypassing the protocol. A changed candidate or relevant user steering requires renewed review.

## Initial enforcement boundary, 8 October 2026

Installed Codex 0.161.0 was checked against release source commit `979011409de0a60b52f179721948e65531d26144`. Its [hook enum](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/protocol/src/protocol.rs#L1579) has no arbitrary assistant-message pre-delivery event. Text deltas are [emitted during sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L2284); [Stop hooks run after sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L653). A Stop hook can request continuation but cannot withhold text already streamed. This is release-specific source inspection, not an exercised runtime intercept test.

A hard block would need a delivery-owning client/proxy that buffers outgoing text or a runtime change. None is installed by this package. The current selected mechanism is the active instruction, independent exact-text review and pre-send consistency check. Mechanical unit tests cover altered text, invalid receipts, self-review, missing/failing checklist entries and changed context. Actual reviewed progress/final packets demonstrate use, not reliable future compliance or reviewer quality.
