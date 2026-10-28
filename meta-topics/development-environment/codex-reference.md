# Codex project policy and retained trial evidence

Reusable product mechanisms are maintained centrally in the
[Codex 0.160 reference](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md).
That page retains the 3–4 October 2026 release pin, citations and evidence scope.
This adapter owns msc configuration authority, selected trials, generated project
inventory and their verification limits. Trial observations and inventory numbers
below are dated 3 October evidence; they do not assert present runtime state. Source consolidation changes none of
those selections. For current technical questions, start with the shared
[Codex specialist](/workspaces/agent-fleet-tools/skills/codex-reference/SKILL.md)
or the selected [agent-platform reference](/workspaces/agent-fleet-tools/skills/agent-platform-reference/SKILL.md);
consult only the relevant material and recheck version-sensitive claims.

## Where to look for Codex behavior

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#where-to-look-for-codex-behavior).

## Cheap current quota check

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#cheap-current-quota-check).

## Daemon restart and subagent recovery

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#daemon-restart-and-subagent-recovery).

## Settings ownership and maintenance

[Project config](../../.codex/config.toml) explicitly controls project agent limits
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

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#config-controls-that-change-injected-instructions).

The courtesy-only ending in the original config walkthrough occurred despite
existing continuation guidance; no conflicting active prompt sentence was
identified as its cause. Prompt replacement remained a candidate, with no
prompt-control setting selected by that assessment. Project approval and
configuration-comment requirements above still govern any actual change.

## Refresh source before tracing a feature

Use the shared [version-matched source route](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/source-and-evidence.md#fast-local-source)
for technical investigations. The legacy `scripts/update-codex-source.sh` remains
a project inventory dependency: it updates the mutable cache consumed by the
project generator below, refuses dirty/unexpected checkouts and never builds,
installs, changes configuration or restarts Codex. That project-specific caller
is separate from the shared specialist's immutable per-commit source cache.

## Orchestration: flags are not the whole runtime

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#orchestration-flags-are-not-the-whole-runtime).

## Regenerable feature inventory and version provenance

[Maintained flag analysis](codex-feature-analysis.md) interprets the selected
release. The [generated Markdown inventory](../../docs/codex-features.md) and
[machine-readable snapshot](../../docs/codex-features.json) are produced by
[the inventory script](../../scripts/codex-feature-inventory.py):

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
[Configuration decisions and proposals](../../docs/codex-config-proposals.md) record the
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

| Control | Project selection at the dated assessment |
| --- | --- |
| `current_time_reminder` | Approved provisional trial: 60 seconds, after user/tool output; initial/new-window reminder still forced. It does not enforce a wall-clock deadline. |
| `token_budget` | Approved provisional trial: warning below 20,000 tokens before compaction; no history/notes extension. It is context capacity, not money or the tree's spending allowance. |
| `rollout_budget` | Requires an explicitly chosen positive limit and reminder thresholds. No expenditure limit was selected here. Accounting is not a dollar estimate or a preflight guarantee against overshoot. |
| `goals` | Already enabled. A goal budget and a rollout hard stop are different mechanisms; create a goal only when requested. |
| `runtime_metrics` | No prompt injection found in the inspected consumer paths. Host telemetry configuration and exporter availability determine collection. |
| `context_management` | Separate continuity/storage evaluation; not needed merely to obtain a clock reminder. |
Technical injection, accounting and cache findings are maintained in the
[shared mechanism reference](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#time-context-capacity-expenditure-and-caching).

Earlier chat wording that an ordinary-session count-up meter was wholly unresolved
overstated the gap. Agent-visible prompt delivery and reliable translation into
subscription quota remain unresolved; dated shadow prices are not quota/billing.
The 3 October follow-up [workflow/trust review](../../scripts/subtree-usage.md#workflow-and-trust-review-3-october-2026)
reproduced copied-fork double counting and silent zero defaults for missing
required fields; Jörn instructed fixing these. Both local readers now share
owned-response deduplication, legacy ownership boundaries and strict required
fields, with ordinary forks separate from spawned workers. Thirty-five focused
tests and a live owned-response sum check passed. The on-demand
`scripts/usage-awareness.py` report remains usable. Jörn rejected the provisional
after-tool candidate's complexity on 3 October; it is withdrawn, with no
activation approval pending. [Proposal C](../../docs/codex-config-proposals.md#c-withdrawn-ongoing-work-count-up-reminder-historical-candidate)
retains its exact declaration, trust append and checks as historical provenance.
Actual automatic delivery was not verified. Task-start/handoff delivery was also
rejected; ongoing resource awareness remains an unfinished parent outcome.

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

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#subagent-message-encryption-supported-and-unknown-controls).

## Implementation map

See the [centrally maintained mechanisms](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#implementation-map).

## Quick tour and bounded trial

The [shared command tour](/workspaces/agent-fleet-tools/skills/codex-reference/references/writeups/codex-0.160-mechanisms.md#tui-and-cli-command-tour)
owns reusable TUI/CLI navigation.

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
