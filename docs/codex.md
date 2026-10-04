# Codex TUI, configuration and implementation reference

Checked 3 October 2026 against installed CLI **0.160.0** and upstream tag
`rust-v0.160.0`, commit `a956835d020762cb2b570053af06f643a11c0ecc`.
This page owns the project feature/configuration reference; host installation
and operations remain in [dotfiles INSTALL.md](/home/joern/.dotfiles/INSTALL.md).
The earlier [workflow proposal](coordination/workflow-proposal.md) contains a
dated 0.159.2 inventory, not the current inventory.

## Where to look for Codex behavior

Start with current official documentation for the specific question:

| Question | Official entry |
| --- | --- |
| Thread resume, runtime status and loaded instruction paths | [App-server lifecycle and thread resume](https://learn.chatgpt.com/docs/app-server#start-or-resume-a-thread) |
| CLI resume and server commands | [Command reference](https://learn.chatgpt.com/docs/developer-commands?surface=cli) |
| Subagent controls and custom agents | [Subagent guide](https://learn.chatgpt.com/docs/agent-configuration/subagents) |
| Instruction-file discovery | [AGENTS.md guide](https://learn.chatgpt.com/docs/agent-configuration/agents-md) |
| Configuration layers and settings | [Config basics](https://learn.chatgpt.com/docs/config-file/config-basic), [reference](https://learn.chatgpt.com/docs/config-file/config-reference) |

Use official documentation search/fetch when available. If those pages do not
answer a lifecycle detail, [refresh the installed-release source](#refresh-source-before-tracing-a-feature)
and follow the implementation and regression tests. The daemon's own
[upstream README](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server-daemon/README.md)
documents its lifecycle commands and shutdown grace. Distinguish documented
API behavior, release-specific source findings and locally exercised behavior;
do not substitute general restart advice for the relevant mechanism.

## Daemon restart and subagent recovery

Source inspection on 3 October 2026 at the release pinned above establishes:

- A managed daemon shutdown snapshots successfully persisted, loaded root
  threads. The replacement restores them through normal cold resume in the
  background, without requiring an attached client. Ephemeral threads and
  failed persistence are excluded.
- An eligible interrupted root turn can automatically start a new continuation
  turn. This requires recorded, uncanceled regular work, the matching local
  environment and permission profile, and no intervening completion or abort.
  It is continuation from saved history, not preservation of an executing process.
- Cold resumption of a V2 root restores the identities of its open descendants
  without eagerly loading their runtimes. Messaging or a follow-up can lazily
  reload the same child with saved history and identity. `send_message` queues
  context; `followup_task` can start a turn. Parent ownership must be available.
  This does not promise automatic continuation of every interrupted child turn.

Thus native recovery includes the subagent tree; a manual reconstruction packet
is not the default prerequisite for a managed restart. Keep new decisions and
constraint-change requests in the top-level thread so its persisted conversation
contains them. Add external notes only for information the native history does
not carry. After recovery, inspect the restored roster and current files/results
before replacing workers or repeating actions.

Reopening a TUI can reattach to an existing daemon/runtime; it does not itself
establish a daemon restart or fresh configuration adoption. Conversely, native
cold resume preserves some session settings and history. The generic AGENTS.md
startup guide alone does not establish which changed instructions/settings a
resumed root or child uses. Check the affected setting's consumer and effective
runtime state. Daemon lifecycle commands belong to the host operations owner;
this documentation update did not restart any process.

Implementation and existing regression tests, inspected but **not run here**:

| Behavior | Pinned upstream source |
| --- | --- |
| Root snapshot and background restoration | [snapshot selection](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/request_processors/daemon_snapshot.rs), [recovery startup](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/daemon_thread_recovery.rs), [restore dispatch](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/message_processor.rs#L774) |
| Interrupted root continuation and its exclusions | [turn capture](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/daemon_recovery.rs), [continuation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/request_processors/daemon_continuation.rs), [daemon recovery tests](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/tests/suite/v2/daemon_update_recovery.rs) |
| V2 descendant identities and lazy child reload | [restore/load implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/spawn.rs#L188), [message/follow-up dispatch](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/api.rs#L100), [cold-root resume test](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/tests/suite/multi_agent_resume.rs#L173) |

## Settings ownership and maintenance

[Project config](../.codex/config.toml) explicitly controls project agent limits
and feature choices, even when their values equal host defaults. This prevents
later host-default changes from silently selecting a different project setup.
Trusted project config participates in layering; `/debug-config` shows the
actual layers and requirements for the active TUI. A file edit does not prove
an existing thread has adopted the change; inspect effective settings and use
the affected setting's reload mechanism. See [restart and recovery](#daemon-restart-and-subagent-recovery)
before assuming a fresh TUI or daemon restart resets an existing thread.
See [official configuration guidance](https://learn.chatgpt.com/docs/config-file/config-basic).

**Model and reasoning effort stay in [host config](/home/joern/.codex/config.toml).**
Jörn explicitly requested this on 3 October 2026 because of reported frontend
confusion involving the VS Code extension and ChatGPT desktop app. The config
format permits project model/effort keys; the frontend issue has not been
reproduced here. Do not add project model/effort overrides or agent model
defaults as a workaround without revisiting that decision. UI preferences,
authentication/provider routing, browser paths, host telemetry and host trust
state also remain with their host owners. Permission defaults currently inherit
the host's unrestricted filesystem/network and `approval_policy = "never"`;
this package does not change those permissions or enable automatic review.

**Approval for edits to project config:** Jörn requested on 3 October 2026 that
edits require his explicit approval before application or retention. Show the
proposed diff and expected consequences first; prepare unapproved candidates
outside the loaded config. An existing explicit instruction authorizes changes
within its stated scope, without another confirmation. Silence and automatic
tool approval do not authorize a change. This is an agent instruction in the
config comments, not enforcement by the TOML parser, and is a user-selected
boundary rather than an empirically validated general policy.

When changing an intentional project override, **maintain comments giving its
purpose, scope and reason**, either beside it or through a resolvable file or
session reference. Keep important boundaries beside the setting. State unknown
rationale rather than inventing one; record dates and reconsideration conditions
for trials or version-dependent behavior. References need not copy transcripts.
Machine-maintained host values should be identified as such rather than given
invented policy rationales. This requirement is now in the project config;
the inspected host config/dotfiles guidance previously had no equivalent rule.

Current project choices preserve the observed setup rather than select new
trials. The original reason for disabling `shell_snapshot` is unknown. Host
`analytics_plan_history = true` is not copied into project config: its original
selection rationale is unknown and its consumer is the TUI account-usage view,
not the project's orchestration policy.

## Config controls that change injected instructions

The selected scope of this walkthrough is problem-driven configuration work.
Broader communication/process design can be handled by its other session owners;
this thread traces relevant settings, instruction effects and adoption mechanisms.
No ownership transfer or independent-session receipt is established by this note.

| Setting | Effect in release 0.160.0 | Implementation |
| --- | --- | --- |
| `developer_instructions` | Supplies a developer-role instruction fragment in addition to the ordinary prompt. A runtime override can take precedence over the file value. | [config resolution](/home/joern/.cache/codex-source/codex-rs/core/src/config/mod.rs:4005), [injection](/home/joern/.cache/codex-source/codex-rs/core/src/session/mod.rs:4323) |
| `model_instructions_file` | Replaces the selected model's built-in base instructions, rather than editing one sentence. Requires accounting for everything the replacement omits. | [field contract](/home/joern/.cache/codex-source/codex-rs/config/src/config_toml.rs:266), [loader](/home/joern/.cache/codex-source/codex-rs/core/src/config/mod.rs:3992) |
| `features.multi_agent_v2.root_agent_usage_hint_text`, `.subagent_usage_hint_text` | Replace the corresponding V2 role guidance; a configured empty string suppresses its fallback. These are role hints, not the whole base prompt. | [role-hint resolver](/home/joern/.cache/codex-source/codex-rs/core/src/session/multi_agents.rs:39) |
| `features.multi_agent_v2.subagent_developer_instructions` | Overrides inherited developer instructions for subagents without role-specific instructions. | [field contract](/home/joern/.cache/codex-source/codex-rs/features/src/feature_configs.rs:283), [child config](/home/joern/.cache/codex-source/codex-rs/core/src/agent/child_config.rs:145) |
| `include_apps_instructions`, `include_collaboration_mode_instructions`, `include_permissions_instructions` | Control inclusion of named instruction blocks. Omitting a block is broader than repairing its wording; actual tools and permissions have separate controls. | [fields](/home/joern/.cache/codex-source/codex-rs/config/src/config_toml.rs:254) |

First identify the concrete unwanted behavior and relevant effective instruction;
then prepare the smallest candidate diff outside loaded config, explain its
consequences and obtain the required approval. `codex debug prompt-input` can
render a fresh diagnostic prompt input list without a model generation; it is
not by itself the exact prompt of an already-running thread. Distinguish prompt
delivery from a behavioral improvement. The courtesy-only ending in this
walkthrough occurred despite existing continuation guidance; no conflicting
active prompt sentence has been identified as its cause. Adding another
instruction or replacing a whole prompt is therefore a candidate, not an
established repair. No prompt-control settings were changed by this assessment.

## Refresh source before tracing a feature

The former `/workspaces/codex` source checkout/build was removed on 27 September;
see [host history](/home/joern/.dotfiles/memories/host-estate.md).
A shallow **source-only** checkout now lives at
[/home/joern/.cache/codex-source](/home/joern/.cache/codex-source).
It is independent of the installed CLI and creates no Rust build cache.

**Run the refresh helper before relying on local source links:**

```bash
bash scripts/update-codex-source.sh
```

[The helper](../scripts/update-codex-source.sh) selects `rust-v<VERSION>` from
`codex --version`, fetches that release and prints its exact commit. It refuses
a dirty checkout or an unexpected origin. It does not build/install Codex,
change runtime configuration, update the CLI or restart the daemon. An absent
release tag is an error, not permission to silently substitute development code.
`CODEX_SOURCE_DIR` overrides the cache location; local links below assume the
default workstation path.

For an explicitly chosen upstream-development investigation:

```bash
bash scripts/update-codex-source.sh --main
```

`main` can differ from the installed CLI. Return to the default command before
diagnosing installed behavior. The online links below pin the checked commit;
after a release change, recheck paths, semantics and line anchors before updating
this page. Config/feature registration identifies accepted settings and defaults;
follow the consumer and relevant tests to establish what a setting actually does.

## Orchestration: flags are not the whole runtime

`codex features list` reports resolved flag values/stages. It does **not** prove
which multi-agent backend an active thread uses. In this release,
`multi_agent_version_override` forces V2 only when its flag is true; otherwise
`multi_agent_version_for_model` can select the version advertised by the model.
An existing thread can retain its selected version. The present session exposes
V2 tools (`followup_task`, `send_message`, `list_agents`, `interrupt_agent`)
despite `multi_agent_v2 = false` in the fresh-CLI flag inventory.
Source: [local resolver](/home/joern/.cache/codex-source/codex-rs/core/src/config/mod.rs:1586),
[pinned resolver](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/config/mod.rs#L1586).

V1 uses depth limits and the legacy spawn/send/resume/wait/close tool family.
V2 adds named tree paths, configurable context inheritance (`fork_turns`),
separate messaging and follow-up turns, mailbox waiting, interruption and agent
listing. Its handlers live beside V1, not behind the message-board feature.
The separate board adapter requires both its board flag and the V2 flag, and
does not start or restore recipients to notify idle agents.
Source: [tool registration](/home/joern/.cache/codex-source/codex-rs/core/src/tools/spec_plan.rs:1302),
[board adapter](/home/joern/.cache/codex-source/codex-rs/core/src/agent_message_board.rs:34).

`agents.max_depth = 99` limits **V1 nesting only**; V2 ignores it.
The shared `agents.max_concurrent_threads_per_session = 99` permits 99 spawned
threads in both backends. Its V2 conversion adds the root to obtain a total of
100, then subtracts it when deriving the child allowance. The separate
`features.multi_agent_v2.max_concurrent_threads_per_session` field, if selected
later, instead counts the root in its configured value.
`agents.job_max_runtime_seconds` is retained only as a **legacy no-op**.
The host's comment promising an hour timeout is therefore inaccurate for this
release; the project config intentionally omits the key. These ceilings do not
authorize workloads or budgets. [AGENTS.md](../AGENTS.md) retains assignment,
supervision and spending requirements; async questions remain locally prohibited.
Source: [agent fields](/home/joern/.cache/codex-source/codex-rs/config/src/config_toml.rs:730),
[effective concurrency](/home/joern/.cache/codex-source/codex-rs/core/src/config/mod.rs:1617),
[V2 config conversion](/home/joern/.cache/codex-source/codex-rs/core/src/config/mod.rs:2760),
[no-op regression test](/home/joern/.cache/codex-source/codex-rs/core/src/config/config_tests.rs:9553).

## Regenerable feature inventory and version provenance

[All flags](codex-features.md) and [machine-readable details](codex-features.json)
are generated by [the inventory script](../scripts/codex-feature-inventory.py):

```bash
python3 scripts/codex-feature-inventory.py
python3 scripts/codex-feature-inventory.py --check
```

The first command refreshes the source to the installed CLI release, then reads
the registry, enum descriptions, candidate consumer files, fresh CLI flag values,
project overrides and Git line provenance. It makes no model-generation call and
does not edit configuration. The check rejects changed CLI/daemon versions,
source/config/generator fingerprints, changed line provenance or an edited
generated Markdown file. A dirty source checkout is refused. A saved snapshot
can become stale; this provides cheap regeneration and explicit detection,
not a promise of perpetual freshness. It is not the active thread's effective
configuration or prompt. Runtime-model defaults and retained thread settings
can differ from the fresh CLI inventory.

On 3 October, all **154** registry flags matched the installed CLI: 47 stable,
60 under development, 3 experimental, 4 deprecated and 40 removed. Every flag
has a first-pass project review group; newly added, unmapped flags are visibly
unreviewed. This screen used source descriptions, with deeper consumer tracing
for orchestration, time/context/budget controls and message encryption. It is
not a behavioral evaluation of all 154 flags. A maturity label alone neither
selects nor rejects a feature. Requirements-only desktop gates are not project
preferences; removed flags can retain protocol/composition behavior, so do not
assume every removed key is inert.

**Which Codex version wrote a config line?** Git identifies a line's last commit
and date; it does not normally record the running Codex version. The generated
provenance table marks that runtime version unknown and uncommitted lines as
working-tree changes. The explicit project overrides were assessed against
0.160.0 here. Future intentional edits should retain their dated release
assessment, reason and review trigger in their comments or linked evidence;
do not reconstruct historical runtime versions from commit dates alone.

## Time, context capacity, expenditure and caching

These controls are distinct. The repository enables time/context awareness as
provisional trials and fixes `rollout_budget`, `runtime_metrics` and
`context_management` off. Those off choices are not findings that the features
are useless.
[Configuration decisions and proposals](codex-config-proposals.md) record the
approved provisional time-reminder and remaining-context trials.
Jörn approved automatic time reminders on 3 October: system clock, 60-second
interval, delivery after user/tool output, with interruptible sleep available.
He considers that cadence fast; it is explicitly provisional. Observe actual
reminder frequency/usefulness and lengthen the interval or disable if unhelpful.
At 22:14 UTC on 3 October 2026, the running root had repeatedly received
`current_time_reminder` developer messages. This establishes delivery in that
root, without establishing exact config provenance or cadence. Cache/cost and
behavioral benefit remain unmeasured.
Jörn subsequently approved `token_budget`: context tools and a warning below
20,000 tokens before compaction, marked provisional. That threshold is speculative;
review warning usefulness, model compatibility and compaction continuity. The
optional history/notes extension stays off. At the same observation, current
context-window metadata was present and `get_context_remaining` returned
`tokens_left: 120014`. Threshold-warning firing, delivery in other agents and
compaction continuity remain unverified.

| Control | What reaches the agent / behavior | Project assessment |
| --- | --- | --- |
| `current_time_reminder` | Appends a UTC time message before eligible inferences; can expose clock tools. Default interval is one second, with every inference eligible. | Approved provisional trial: 60 seconds, after user/tool output; initial/new-window reminder still forced. It does not enforce a wall-clock deadline. |
| `token_budget` | Context-window metadata, `get_context_remaining` and `new_context_window` tools; optional remaining-context threshold warning. | Approved provisional trial: warning below 20,000 tokens before compaction; no history/notes extension. It is context capacity, not money or the tree's spending allowance. |
| `rollout_budget` | Shared weighted-token accounting for the root and children; reminders at selected thresholds; session stops when exhausted. | Requires an explicitly chosen positive limit and reminder thresholds. No expenditure limit was selected here. Accounting is not a dollar estimate or a preflight guarantee against overshoot. |
| `goals` | Automatic continuation includes the objective and, when a goal budget exists, used/remaining budget guidance. | Already enabled. A goal budget and a rollout hard stop are different mechanisms; create a goal only when requested. |
| `runtime_metrics` | Runtime metrics reader, telemetry and TUI summaries. | No prompt injection found in the inspected consumer paths. Host telemetry configuration and exporter availability determine collection. |
| `context_management` | Experimental context rollover/history integration, with model/account eligibility and optional history/notes. | Separate continuity/storage evaluation; not needed merely to obtain a clock reminder. |

Time reminders record new history items rather than rewriting a timestamp in
the initial instructions. Token/rollout threshold reminders likewise append
items. This preserves the preceding prefix. Enabling a feature can change
initial tools/instructions, and compaction changes history. The client's
WebSocket continuation check also requires earlier request properties and the
input/output prefix to match; that transport optimization is not evidence of
billed cache hits.

Official [prompt-caching guidance](https://developers.openai.com/api/docs/guides/prompt-caching)
requires matching rendered prefixes, including tools and instructions; cache
availability and eligible boundaries also matter. Therefore **append-only
reminders can preserve reusable earlier context, but cache neutrality is not
established**. Additional messages cost tokens, and no paid cache comparison
was run. Do not sell a timestamp setting as a verified latency/cost improvement.

Time reminders can already be enabled by persistent-model defaults when the
setting is absent; an explicit config choice overrides that default. New
context windows force a reminder even with the user/tool-output delivery mode.
The stock clock-error path can fail a turn; `nonfatal_clock_read_errors` is a
separate choice, not implicitly enabled in the proposal.

**Cumulative expenditure preference (Jörn, 3 October):** `rollout_budget` has an
internal used-token accumulator, but its injected developer message reports
remaining weighted tokens. It sends an initial/new-window reminder and updates
when selected remaining-budget thresholds are crossed. No dedicated query tool
is registered by this flag in the inspected release. Its supported configuration
requires a positive cap and stops the session tree on exhaustion; there is no
count-up-only, uncapped mode or configurable reminder-text template.

The existing `get_goal` tool returns accumulated `tokens_used` and
`time_used_seconds` for a goal, and a token cap is optional. Its accounting includes
descendant token usage; the current token formula counts non-cached input plus
output, so it is not dollars or all raw input tokens. Goal continuation messages
also include used tokens. This is an available cumulative-usage mechanism for an
explicitly created goal, but goals carry automatic-continuation behavior and are
not a passive session meter. No goal or spending limit was selected by asking
about this feature. Ordinary sessions already have passive count-up reports:
`scripts/subtree-usage.py` attributes local observed counters by root/workers/model;
`scripts/coordination-load.py` provides bounded telemetry and per-thread attribution,
with independent session trees kept separate from the root's workers. They require
neither goals nor budget-limit flags. See the
[usage accounting contract](../scripts/subtree-usage.md) and
[coordination command](coordination/README.md#workload-supervision).
Earlier chat wording that an ordinary-session count-up meter was wholly unresolved
overstated the gap. Agent-visible prompt delivery and reliable translation into
subscription quota remain unresolved; dated shadow prices are not quota/billing.
The 3 October follow-up [workflow/trust review](../scripts/subtree-usage.md#workflow-and-trust-review-3-october-2026)
reproduced copied-fork double counting and silent zero defaults for missing
required fields; Jörn instructed fixing these. Both local readers now share
owned-response deduplication, legacy ownership boundaries and strict required
fields, with ordinary forks separate from spawned workers. Thirty-five focused
tests and a live owned-response sum check passed. The on-demand
`scripts/usage-awareness.py` report remains usable. Jörn rejected the provisional
after-tool candidate's complexity on 3 October; it is withdrawn, with no
activation approval pending. [Proposal C](codex-config-proposals.md#c-prepared-ongoing-work-count-up-reminder-activation-awaits-approval)
retains its exact declaration, trust append and checks as historical provenance.
Actual automatic delivery was not verified. Task-start/handoff delivery was also
rejected; ongoing resource awareness remains an unfinished parent outcome.

Pinned source evidence:

| Mechanism | Implementation |
| --- | --- |
| Time defaults, boundary gating and appended reminder | [defaults/state](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/time_reminder.rs), [message text](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/context/current_time_reminder.rs), [config fields](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/feature_configs.rs#L413) |
| Context capacity metadata/tools/reminder | [world state](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/world_state.rs), [reminder](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/token_budget.rs), [remaining-context tool](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/get_context_remaining.rs) |
| Shared rollout accounting and stop | [accounting](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/rollout_budget.rs), [reminders](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/rollout_budget.rs), [stop control](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/budget.rs) |
| Goal continuation budget text | [goal runtime](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/runtime.rs), [continuation template](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/templates/goals/continuation.md) |
| Goal cumulative token/time query and accounting | [tool response](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/tool.rs#L449), [goal accounting](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/accounting.rs#L278), [token formula](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/accounting.rs#L527) |
| Runtime telemetry | [OTEL setup](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/otel_init.rs), [TUI reader](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/chatwidget/turn_runtime.rs) |
| Prefix matching for WebSocket continuation | [client](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/client.rs#L1380) |

Other problem-driven candidates from the full screen: `defer_mailbox_preemption`
changes when worker mail interrupts sampling; `deferred_tool_world_state` changes
the agent's deferred-namespace awareness; `retain_client_developer_messages`
changes retention across compaction. Each needs its own consumer/context review.
Jörn did not select these three changes on 3 October: no concrete solved problem
was identified, and their fresh CLI values remain off. They are not pending trials.
`executed_tool_call_metadata` concerns internal request metadata, not an assured
user-facing tool log. Model/catalog support can already select some tooling
despite a false flag. Do not enable these merely to make an inventory look full.

## Subagent message encryption: supported and unknown controls

**No supported project/host config switch was identified to disable V2 message
encryption in 0.160.0.** Do not put a fictitious `encrypt_subagent_messages = false`
key in config. The relevant fields in spawn, send-message and follow-up tool
schemas carry `encrypted: true`; model-catalog parameter overrides preserve that
annotation. The normal delivery path forwards the opaque string as encrypted
content. No local key/decrypt route was found in the inspected client paths.

However, **"client patches cannot change it" is not established**. The client
explicitly supports a backend-signaled plaintext path: an empty
`encrypted_function_args` list selects `DirectPlaintextMessage`, and existing
mock integration tests exercise plaintext delivery. A patch can change schema
annotations or local dispatch. Whether production honors a request to emit
plaintext is unverified; relabeling already encrypted bytes does not decrypt
them. No patch/build/backend experiment was performed.

This does not mean the child's entire prompt or every communication is encrypted.
Inherited base/developer/user history has ordinary inspectable entries; opaque
reasoning/compaction items are distinct. Child final results also have a plain
delivery path. A prompt/communication view must distinguish those item types and
show encrypted entries as opaque, with provenance and size rather than fabricated
plaintext. The recurring-view session accepted that view-design handoff on
3 October; config implications remain in this reference.

| Evidence | Pinned source |
| --- | --- |
| Encrypted schema fields and annotation-preserving overrides | [schemas](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/multi_agents_spec.rs), [override policy](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/multi_agent_tool.rs), [annotation type](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tools/src/json_schema/types.rs) |
| Plaintext signal and delivery | [router](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/router.rs#L45), [V2 handler](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/multi_agents_v2.rs), [delivery](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/delivery.rs), [mock regression](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/tests/suite/subagent_notifications.rs#L2214) |
| Child inheritance and final-result path | [spawn](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/spawn.rs), [completion](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/control/completion.rs) |

## Implementation map

Paths below were checked at the pinned release. Directory links are search
entry points; the named symbols identify the behavior to trace.

| Feature or setting | Local implementation | Pinned upstream source / what to inspect |
| --- | --- | --- |
| Feature stages/defaults | [features/src/lib.rs](/home/joern/.cache/codex-source/codex-rs/features/src/lib.rs) | [registry](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs): `FEATURES`, `FeatureSpec` |
| Structured feature settings | [feature_configs.rs](/home/joern/.cache/codex-source/codex-rs/features/src/feature_configs.rs) | [feature config types](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/feature_configs.rs): V2, code mode, budget/context controls |
| Config loading/precedence | [config loader](/home/joern/.cache/codex-source/codex-rs/config/src/loader/mod.rs) | [loader](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/config/src/loader/mod.rs); [schema](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/config.schema.json) |
| V1 collaboration handlers | [multi_agents](/home/joern/.cache/codex-source/codex-rs/core/src/tools/handlers/multi_agents) | [handlers](https://github.com/openai/codex/tree/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/multi_agents) |
| V2 collaboration handlers | [multi_agents_v2](/home/joern/.cache/codex-source/codex-rs/core/src/tools/handlers/multi_agents_v2) | [handlers](https://github.com/openai/codex/tree/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/multi_agents_v2); tool schemas/registration in `tools/spec_plan.rs` |
| Spawn, model/effort inheritance and child config | [agent/child_config.rs](/home/joern/.cache/codex-source/codex-rs/core/src/agent/child_config.rs) | [inheritance](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent/child_config.rs); lifecycle in `agent/control.rs` |
| Agent picker | [app/agent_picker.rs](/home/joern/.cache/codex-source/codex-rs/tui/src/app/agent_picker.rs) | [picker](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/app/agent_picker.rs) |
| Goals | [thread_goal_processor.rs](/home/joern/.cache/codex-source/codex-rs/app-server/src/request_processors/thread_goal_processor.rs) | [goal requests](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/request_processors/thread_goal_processor.rs); TUI `app/thread_goal_actions.rs` |
| Hooks | [hooks/src/lib.rs](/home/joern/.cache/codex-source/codex-rs/hooks/src/lib.rs) | [hooks](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/hooks/src/lib.rs); core integration `hook_runtime.rs` |
| Message board | [agent_message_board.rs](/home/joern/.cache/codex-source/codex-rs/core/src/agent_message_board.rs) | [adapter](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/agent_message_board.rs); extension owns storage/tools |
| Plan-history flag | [tui/src/analytics.rs](/home/joern/.cache/codex-source/codex-rs/tui/src/analytics.rs:156) | [consumer](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/analytics.rs#L156) |
| TUI commands | [slash_command.rs](/home/joern/.cache/codex-source/codex-rs/tui/src/slash_command.rs) | [commands](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/slash_command.rs); dispatch in `tui/src/app` and `chatwidget` |

## Quick tour and bounded trial

| Task | TUI / CLI |
| --- | --- |
| Inspect live configuration and usage | `/status`, `/debug-config`, `/usage`; fresh CLI `codex features list`, `codex doctor --summary` |
| Select model/effort; ask for a plan | `/model`, `/plan` (model/effort persisted on host) |
| Track a goal and automatic continuation | `/goal`, `/goal pause`, `/goal resume` |
| Inspect workers or other local sessions | `/agent`; `codex agents` |
| Branch/recover context | `/fork`, `/resume`, `/side`, `/compact` |
| Review edits and background commands | `/diff`, `/review`, `/ps`, `/stop` |
| Inspect capabilities and lifecycle hooks | `/skills`, `/plugins`, `/apps`, `/mcp`, `/hooks` |
| Change UI or experimental choices | `/statusline`, `/keymap`, `/experimental` |

[Official command guide](https://learn.chatgpt.com/docs/developer-commands?surface=cli)
and [subagent guide](https://learn.chatgpt.com/docs/agent-configuration/subagents)
describe supported usage; model/client gates can hide commands.

Suggested first trial (not performed by this documentation change): assign two
independent read-only questions to two explicitly requested agents, name their
outputs and receiving owner, wait for both, then inspect `/agent` and integrate
the returned findings. No V2/message-board toggle is needed to exercise the
current tools. Record actual delivery, coordination effort and usage where
available rather than treating a successful spawn as successful collaboration.

## Verification and remaining gaps

Fresh CLI inventory: `multi_agent`, `goals`, `hooks`, `apps`, `plugins`, `worktrees`
enabled/stable; V2 flag disabled/stable; `memories` disabled/stable;
`current_time_reminder`, `token_budget` enabled/under development;
`agent_message_board`, `runtime_metrics`, `rollout_budget`, `context_management`
disabled/under development. These are observations of the
fresh config loader, not a full account/runtime capability check.

The earlier doctor check loaded config and found the daemon running, but reported
three session-history consistency issues while its database check was healthy.
Project TOML passed the release JSON schema, the fresh feature inventory loaded
the overrides, local documentation links resolved, and the refresh helper
passed shell syntax, a real release refresh and a dirty-checkout refusal check.
Two bounded source-inspection subagents returned the time/budget/cache and
encryption findings; this coordinator integrated them. No Codex source
tests/build, paid cache/workload trial or history repair was performed.
The full inventory regenerated against the installed release and passed its
freshness check; negative checks rejected modified Markdown and a stale version
fingerprint. Both initial unactivated proposals parsed and passed the release
JSON schema. Following Jörn's explicit approval, the provisional time-reminder
setting was applied; Jörn later explicitly approved the provisional context trial,
which was also applied. These checks
establish artifact/config structure, not live adoption. The applied time-reminder
config passed the release schema and fresh CLI resolution reported the flag
enabled with the intended structured settings. Runtime observations at 22:14 UTC
on 3 October 2026 establish timestamp-message delivery, current context-window
metadata and a successful `get_context_remaining` response (`tokens_left: 120014`)
in the running root. They do not establish exact config provenance/cadence,
threshold-warning firing, other agents, compaction continuity, usefulness or
cache neutrality. No restart was performed.
The source/config assessment does not establish the
reported frontend bug, behavioral improvement, complete telemetry/billing or
reliable cross-client adoption. Recheck after CLI/frontend upgrades, model-catalog
changes or deliberate settings changes.
