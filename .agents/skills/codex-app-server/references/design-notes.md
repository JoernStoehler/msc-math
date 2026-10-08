# Design and validation, 8 October 2026

## Selected scope

Jörn requested a repository-local skill for claude.ai cloud microVMs to use Codex through `codex app-server daemon`, including installer/device-code login and a shell wrapper that covers the ordinary `codex_tui.*` cross-session workflows for agents lacking those tools. He explicitly distinguished this from OpenAI's Claude Code plugin and a UI-targeted integration. This package implements a local shell client, not a plugin, VM-to-workstation bridge or bidirectional transport into an existing Claude conversation.

The separate workspace `codex-cli-claude-cloud` package covers bounded one-shot `codex exec` runs; it is not a prerequisite or bundled dependency of this package. Historical per-grant authority derives from Jörn’s explicit choice recorded in the [interop source](https://github.com/JoernStoehler/agent-skills/blob/main/codex-claude-interop.md); September classifier/UI observations are dated evidence, not a claim about every current cloud session. This new package does not change host configuration or carry credentials between environments. Claude discovery links to the same repository owner.

## Implementation choices and alternatives

Use native daemon lifecycle management and its reported socket, instead of maintaining a second daemon/broker or killing a process to end a polling window. Native daemon state lets a task continue after the short-lived shell client disconnects. The control socket uses WebSocket framing; a plain JSON-lines connection was rejected during the first read-only probe. `codex app-server proxy` only relays bytes, so it does not eliminate the WebSocket client requirement. Use `websocket-client` (setup pins 1.8.0, host smoke used installed 1.7.0), avoiding a bespoke WebSocket implementation. Official protocol documentation: <https://learn.chatgpt.com/docs/app-server>. Installed 0.161.0 CLI help and generated JSON schemas established actual field/method availability. Daemon/source inspection used upstream commit `979011409de0a60b52f179721948e65531d26144`, notably `app-server-daemon/README.md`, `app-server-client/src/remote.rs`, `app-server/src/request_processors/thread_processor.rs` and `login/src/device_code_auth.rs`.

Create defaults to workspace-write/never so a short-lived client cannot leave ordinary command approvals orphaned or approve autonomously. The explicit sandbox switch does not grant task authority. Follow-ups preserve existing policy/model unless an intended model override is supplied. Pending interactive requests are reported, not fabricated. A persistent subscription/approval broker would improve richer interactive workflows but introduces state/ownership/recovery work; it is deliberately outside this initial shell-client contract. Native TUI clients can provide that interaction against the same daemon.

Read uses metadata plus paginated turns instead of deprecated full-history hydration. Wait exposes task/turn status rather than pretending idle, notLoaded, failed or input-blocked tasks succeeded. Timeout means observation ended, not cancellation. Unknown mutation outcomes are not retried. The create output preserves a thread ID even if a later title/start RPC fails. Native app-server archive semantics are used; exact TUI descendant behavior is not promised.

## Observed validation

Host CLI and running app-server both reported 0.161.0. The existing workstation daemon was used without restart, update, remote-control changes, new login grant or installer execution.

- Wrapper create returned thread `01a11b87-b090-79a2-a6a9-f1d429de4987` and turn `01a11b87-b251-79b1-89f9-b36ace58d117`; the creating process disconnected. A subsequent wait/read retrieved `completed` and the exact final text `ready` (duration 4351 ms).
- A fresh client resumed that thread and started turn `01a11b87-f3e6-7611-9884-9dbc76f30cb1`; subsequent inspection retrieved `completed` and exact final text `resumed` (2339 ms). These were two bounded transport-smoke inference turns, not a research workload or model entitlement survey.
- Fork through the first completed turn returned `01a11b87-f482-7141-8376-3d1760764455`. Rename, archive, restore and re-archive succeeded. Both smoke threads were archived, and an archived list filtered to their scratch cwd found both. No smoke task remains running.
- Thirteen offline regression tests cover interleaved notifications/request IDs, unsupported server requests without approval, RPC error/disconnection/timeout without retry, global polling deadlines, created-ID preservation, empty-prompt non-mutation, inherited model/policy, steer's active-turn precondition, timeout without interruption, incomplete snapshot reporting, failed/input-blocked status visibility, and invalid numeric windows.
- Shell syntax and skill-frontmatter validation passed. The launcher uses the dedicated venv when present and otherwise an already compatible Python installation. Real installation/venv creation are not part of the host smoke.

Reproduce the offline checks from the repository root:

```sh
python3 .agents/skills/codex-app-server/scripts/test_client.py
sh -n .agents/skills/codex-app-server/scripts/setup.sh .agents/skills/codex-app-server/scripts/codex-session
python3 "${CODEX_HOME:-$HOME/.codex}/skills/.system/skill-creator/scripts/quick_validate.py" .agents/skills/codex-app-server
```

The separate bounded native review `/root/wrapper_review` identified a real polling-window defect (sequential per-RPC timeouts exceeded the window) and the then-missing maintenance reference. Polling now clamps each RPC to the remaining shared deadline, reports incomplete snapshots and restores the client's prior deadline. Discovery and initialization are explicitly outside the polling window; zero seconds requests one separately bounded snapshot. The maintenance reference is this file. Message review remains separately owned by `/root/message_reviewer`.

## Remaining uncertainty

Fresh claude.ai installation, dependency download, current network allowlists, device grant/approval, detached-daemon survival under cloud process cleanup and VM deletion have not been tested in this session. No automatic cloud-success claim follows from workstation RPC checks. A cloud forward test should use the exact skill/scripts, a user-authorized device grant and two tiny create/follow-up turns, retaining versions, lifecycle results and substantive outputs without credentials. Live steer/interrupt and pending human-interaction behavior have offline coverage only. CLI/app-server upgrades may change experimental daemon/pagination fields. Numeric spend is unknown; test durations are not billing evidence.
