# Prepared-message approval evidence

[Root instructions](../../../AGENTS.md#collaboration-and-planning) own the trigger; [message-review](../../../.agents/skills/message-review/SKILL.md) owns the reviewer workflow. Keep frozen candidate versions, relevant user context and native review returns together under dated packet directories. Native private reviews do not recursively require another review.

The sender cross-checks reviewer identity and verdict against the actual native return and frozen candidate version, then delivers that exact text. A changed candidate or relevant user steering requires renewed review. The reviewer returns approval or rejection with substantive findings; JSON, hashes and repeated candidate text are no longer required. Earlier packets retain the original receipt protocol as historical evidence.

## Initial enforcement boundary, 8 October 2026

Installed Codex 0.161.0 was checked against release source commit `979011409de0a60b52f179721948e65531d26144`. Its [hook enum](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/protocol/src/protocol.rs#L1579) has no arbitrary assistant-message pre-delivery event. Text deltas are [emitted during sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L2284); [Stop hooks run after sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L653). A Stop hook can request continuation but cannot withhold text already streamed. This is release-specific source inspection, not an exercised runtime intercept test.

A hard block would need a delivery-owning client/proxy that buffers outgoing text or a runtime change. None is installed by this package. The selected mechanism is the active instruction and separate approval of a frozen candidate version. The initial schema/hash verifier's tests and packets are retained as historical evidence, not a requirement of current review or proof of reviewer quality. It did not intercept delivery, and its cost/quality benefit was not established.
