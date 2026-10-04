# Supervisor assignment: bounded final source check

**The exact supervisor remit remains unresolved.** Historical raw call arguments and child assignment messages contain encrypted text. The inspected native projections, including a final direct app-server `thread/read` request, expose the relevant spawn and renewal as `subAgentActivity` records without prompt text. Encryption in the raw record and omission from a projection are different limits.

This read-only check ran on **4 October 2026, 00:53:48–00:54:52 UTC**, after the historical freeze. It queried the existing configuration root with `includeTurns:true` and selected only the two known call IDs. No full returned history, encrypted text, credentials or unrelated messages are retained. It made no model call, task dispatch or configuration change.

The two selected receipts were:

```json
{
  "turn_id":"01a103ca-575f-7cb1-8e22-f2e4de38396d",
  "item":{
    "type":"subAgentActivity",
    "id":"call_kPLn2g8B2jlnjrPYRop6SHCW",
    "kind":"started",
    "agentThreadId":"01a103cb-1d52-7a20-8131-6b70889cc9de",
    "agentPath":"/root/root_scope_supervisor"
  }
}
```

```json
{
  "turn_id":"01a103d4-17d1-7dd2-b881-b2a4aa3602dd",
  "item":{
    "type":"subAgentActivity",
    "id":"call_NcqfMco5emrJsHHZrSY8BO3Y",
    "kind":"interacted",
    "agentThreadId":"01a103cb-1d52-7a20-8131-6b70889cc9de",
    "agentPath":"/root/root_scope_supervisor"
  }
}
```

Their frozen correspondences are configuration root **C L2154, 3 October 22:03:25.365 UTC** (spawn) and **C L2400, 22:14:16.895 UTC** (renewal). Exact line hashes, the bounded read request, observation times and source-file hashes are retained in [SUPERVISOR-REMIT.json](SUPERVISOR-REMIT.json). The complete frozen C source is identified in the [main evidence manifest](evidence.json).

At release `rust-v0.160.0`, commit `a956835d020762cb2b570053af06f643a11c0ecc`, the observed variant contains:

```rust
SubAgentActivity {
    id: String,
    kind: SubAgentActivityKind,
    agent_thread_id: String,
    agent_path: String,
},
```

The protocol defines an optional `prompt` on a different variant, `CollabAgentToolCall`. Its presence does not make that field available on the two actual records. The relevant source is [request hydration](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server-protocol/src/protocol/v2/thread.rs#L1671), [the variants](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server-protocol/src/protocol/v2/item.rs#L374), and [their conversion](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server-protocol/src/protocol/v2/item.rs#L961). Source/version matching has the [existing provenance limitation](report.html#runtime-source); it is not a historical reproducible-build attestation.

This eliminates one specific untested retrieval possibility. It does not establish that plaintext is unavailable everywhere. The supervisor's observation, narrow completion judgment and non-waking delivery remain supported. Whether a narrow parent brief or the child's interpretation produced the coverage gap cannot be isolated; the report does not claim that the supervisor knowingly violated an explicit broader assignment.
