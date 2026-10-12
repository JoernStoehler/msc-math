# Context loss: release-matched source audit, 10 October 2026

Retained on 12 October from the 337f preparation work. All runtime/config observations below are dated 10 October; no current-state or activation check was performed during integration. Historical paths identify the inspected sources.

Investigation: 14:37–14:41 UTC. Receiving/integrating owner: root. No runtime restart, config activation, source patch/build or model-generation probe performed.

## Finding

The observed loss matches **Codex's enabled `token_budget` path deliberately replacing model-visible history with a fresh context window without summarization**. This feature changes compaction behavior; it does not merely expose a counter and warning. The project trial has `enabled = true`, `reminder_threshold_tokens = 20000`, `use_history_notes_extension = false` at `.codex/config.toml:70`. Current project comments/documentation describe the trial as capacity awareness and must disclose the reset semantics.

This is a source-qualified causal diagnosis supported by the event's exact output shape, not just a label of “compaction.” The narrow runtime branch/threshold telemetry was not retained, so the precise threshold reconstruction below remains conditional.

## Version and observed event

Ran `bash scripts/update-codex-source.sh` before inspection. It selected installed CLI tag **rust-v0.162.1**, commit **092d3acd6bec3e3a14bdc7e7a2810ab628ab759d**, source `/home/joern/.cache/codex-source`. `codex app-server daemon version` reports current CLI, managed daemon package and running app-server all **0.162.1**. Source thread `session_meta` also records `cli_version: 0.162.1`; this does not independently prove the daemon version at every historical instant.

Rollout: `/home/joern/.codex/sessions/2026/10/10/rollout-2026-10-10T13-50-53-01a125a6-d5b7-72a0-8064-6cee3eeb95fc.jsonl`.

At **2026-10-10T14:13:52.393Z** (16:13:52 Berlin), immediately after next turn start and before incoming user text is appended:

- `type: compacted`, `window_number: 2`, empty `message`, null `compaction_response_id`.
- `replacement_history`: four developer instruction messages plus one project/environment user instruction message; **no discussion, summary or opaque compaction response**.
- `retained_context`: eight user messages and eight assistant messages, **both `*_messages_incomplete: true`**. This is a bounded review record, not a full conversation backup or ordinary model input.
- The compaction lifecycle completed in roughly nine milliseconds. No summarization request/response is recorded. Same model `gpt-6.1-sol` and `comp_hash: 3000` before/after. No `new_context` call found between preceding checkpoint and event.

The ordinary rollout still contains the earlier discussion; it was removed from the model's active history, not erased from the file. Existing `context-loss/evidence.json` omits the incomplete flags and should be supplemented by this finding.

## Exact producer/consumer chain

All source paths below are relative to `codex-rs/` at the commit above. Official permalink base: `https://github.com/openai/codex/blob/092d3acd6bec3e3a14bdc7e7a2810ab628ab759d/codex-rs/`.

1. `core/src/session/turn.rs:1303` runs pre-sampling compaction when `token_status.token_limit_reached` (line 1315), before current user turn sampling.
2. `core/src/session/turn.rs:1460` tests `Feature::TokenBudget`; when enabled it invokes `compact_token_budget::run_inline_auto_compact_task` and **returns before normal remote/local summarization** (lines 1463–1469).
3. `core/src/compact_token_budget.rs:29` explicitly documents that this path “skips model/server summarization and installs a fresh context window”; line 58 calls `Session::start_new_context_window`.
4. `core/src/session/mod.rs:4550` builds replacement items from `build_initial_context_with_world_state` (line 4580), optionally adding only retained client-authored developer messages. It supplies empty summary and no compaction response ID (4595–4599). Ordinary conversation is not copied. This produces the observed five-item replacement.
5. `core/src/session/mod.rs:4073` installs these replacement items as active history, while separately persisting `retained_context` (4087).
6. `core/src/context_manager/history.rs:600` / `:608` builds model input from normalized `self.items`, **not the separate `retained_context`**. Cold replay likewise installs `replacement_history` while restoring retained context for review (`core/src/session/rollout_reconstruction.rs:476`). Restart/resume alone therefore does not restore the discussion to model input.
7. `history/src/retained_context.rs:24` caps each retained family at eight records; line 44 documents conversational text retained outside summarization **for review**. Eight/eight is expected, not evidence of a model-facing recovery mechanism.

## Why the threshold could fire despite a 146k usage report

Preceding reported `last_token_usage.total_tokens` was **146,253**, model usable context **258,400**. The client has a distinct active-context estimator:

- `core/src/context_manager/history.rs:934`: if `server_reasoning_included` is false, it adds estimates for earlier opaque reasoning items to the last usage. Default state is false (`core/src/state/session.rs:131`); stream events can change it (`core/src/session/turn.rs:2930`).
- Replay of replacement checkpoint plus response-item suffix before the event yields 98 earlier opaque reasoning items. Applying source's `max(0, encoded_len*3/4 - 650)` bytes then ceil(bytes/4) estimate (`history.rs:1053`, `:1069`) gives **100,682 tokens**.
- Thus the conditional local estimate is **146,253 + 100,682 = 246,935**, exceeding **244,800**, the default auto-compaction threshold (90% of 272,000; `protocol/src/openai_models.rs:527`). Config's explicit threshold settings disable model-owned token-budget defaults; no fallback prompt/buffer is selected in the project trial.

This explains the otherwise surprising timing. Current model catalog confirms 272,000 context, 95% usable, no explicit auto limit; it was refreshed at 14:38:29 and is not a historical catalog snapshot. Exact then-effective config, the server-reasoning flag, and estimator value at that turn are absent from bounded logs. Treat threshold accounting as a strong conditional reconstruction, **not a proven double-counting defect**. Do not claim reported 146k itself exhausted the 258k usable context.

## Concrete proposal; activation belongs to root/Jörn

The narrow supported mitigation is prepare the project config diff:

```diff
 [features.token_budget]
-enabled = true
+enabled = false
```

Explain in adjacent comments that disabling the trial restores the summarizing auto-compaction branch in inspected 0.162.1, at the cost of token-budget metadata/tools/warnings. Preserve the user's approval boundary: file diff may be prepared outside loaded config, but do not activate or retain config changes without applicable authorization. Explicit false blocks model-default activation (`core/src/session/token_budget.rs:132`–151); `context_management` is already false. Source inspection establishes branch selection, **not quality or reliability of later summaries**. A fresh config check is not evidence that an existing runtime adopted it; effective loaded settings need verification. Recovery requires importing/re-reading saved discussion after this already-completed reset.

An upstream code fix would decouple capacity tools/reminders from no-summary reset semantics, use summarizing compaction for automatic threshold events, and retain explicit `new_context` as an intentional reset. It needs regression tests/build and an approved activation route; none was attempted. Simply enabling history/notes is a larger alternative with checkpoint/retrieval duties and does not automatically restore conversation. Raising context limits would only delay the same destructive path.

Bounded daemon stderr searches returned no matching event/turn IDs. `codex doctor --summary` reported healthy DBs and three historical rollout/DB consistency issues, but did not connect them to this event; do not use those warnings as its cause.
