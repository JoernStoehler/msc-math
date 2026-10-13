# Codex goal-loop failures and maintained corrections

Consult after an unexplained `/goal` activation, a loop that cannot get required input, or a missing end-turn notification. This is rare feature knowledge, not a rule to load goal details into every task.

## Mechanism found in the inspected source

At `rust-v0.162.1`, commit `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`:

- The TUI parser requires a leading slash. An ordinary “Goal:” heading is not the `/goal` command. The separate `create_goal` tool description says only to create a goal on explicit request, not infer it from ordinary tasks. Diagnose the actual activation call/configuration before blaming the parser.
- An active goal can create another turn from the idle hook. The normal `AgentTurnComplete` notification gate excludes active goals. Ending an individual turn therefore need not produce the normal waiting-for-user signal.
- Goal continuation guidance requires the same hard blocker to recur for at least three consecutive goal turns before `blocked`. It does not immediately stop at the first missing prerequisite.
- `paused` is reserved for an explicit user request; the model is told not to pause on its own initiative. The user-facing `/goal pause` control and interrupt pause path exist. Do not recommend an unauthorized `update_goal(paused)` call as though current tool policy permits it.

See `../evidence/goal-source-review.md` and hashed excerpts. The user's workstation activation/notification failure was reported on 2026-10-13; this package did not inspect or replay that live session. The source explains continuation and notification suppression, but not why that particular ordinary heading resulted in `create_goal`.

## Asking can have its own attention path

Ordinary completion suppression is not suppression of every input request. The inspected live async-question handler independently emits `notify(AsyncQuestion)`; a `request_user_input` modal emits `notify(PlanModePrompt)`. User notification preferences/custom allow-lists can suppress those signals. The request tool has no goal-specific exclusion: availability depends on its experimental feature/mode flags. In Plan mode it sets a blocking flag and waits for a response; Default-mode availability is conditional and its UI can auto-resolve, so an empty response is not consent.

For the reported failure, first inspect the actual activation, collaboration mode, question tool exposure and notification configuration. A suitable question route may obtain input while the goal remains active; do not assert the agent must end the goal just to ask. If that route is absent or fails to obtain attention, identify that narrower blocker and the exposed user pause/interrupt controls. See `../evidence/human-input-source-review.md`. This distinction was discovered during source review; the live workstation configuration is still unknown.

## Intervention choices

| Choice | Useful when | Effect and boundary |
| --- | --- | --- |
| Short repository AGENTS.md pointer | Misuse comes from not knowing rare feature semantics | Makes feature guidance discoverable before use; does not change runtime/tool policy |
| Conditional goal-recovery skill | More detail, diagnostic steps and regression cases are needed only for goal incidents | Keeps the ordinary instruction load small; must be reachable in the actual client's discovery scope |
| Supported catalog description override | An exposed overridable tool description is misleading | Inspect catalog schema, effective model and startup loading; not every tool is overridable |
| Harness/runtime change | Required human input cannot stop continuation or get attention under permitted controls | Fix continuation/status/notification interaction at its owner; a longer prompt cannot enforce a UI notification |

Example proposed AGENTS.md line:

> Before explicitly enabling Codex `/goal`, consult the goal-loop reference or recovery skill for activation, blocker and notification behavior. A plain “Goal:” heading is not a request to enable that feature.

Adapt the actual path to a repo-contained resource or installed skill; don't insert a broken link. This is a candidate maintenance change, not an activated user configuration.

## Catalog findings

The inspected catalog is configured via top-level `model_catalog_json` in `config.toml` and loaded at process startup. A per-thread override is a no-op in the inspected tests. The JSON replaces the bundled catalog rather than merging a tiny partial patch.

`models[].model_messages.tools.send_user_message_async.description` can change the async attention tool's description. The inspected schema/merge path exposes no equivalent override for `create_goal` or `update_goal`; their descriptions are bundled in the goal extension. A description change can explain the supported goal-independent question attention route, but cannot make an async tool await a reply, bypass pause/status restrictions or change the completion-notification gate. `instructions_template` is a much broader model-base-instruction intervention, with precedence and history implications, not a surgical goal fix. See `../evidence/model-catalog-review.md`.

Do not ship a fabricated catalog JSON or claim it is active. Inspect the actual user's client version, catalog and effective descriptions before preparing an authorized catalog change.

## Regression demonstration

A useful test reproduces an activation trace plus missing required input, confirms that no answer was invented, and checks whether the selected fix reaches the next agent and permits a visible input request. A source inspection plus a fixture diagnosis establishes corrective reasoning and artifacts; only live client replay establishes notification behavior. Optional questions should still allow unrelated work, and an explicitly requested valid goal should remain available.
