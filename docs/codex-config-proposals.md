# Project Codex configuration decisions and proposals

**A and B approved and applied as provisional trials on 3 October 2026.** Jörn authorized the automatic time reminders and
requested an explicit provisional/test-phase label. He considers 60 seconds
fast; it is a starting cadence, not an established recommendation.

Model and effort remain host-owned. The feature is enabled in project config;
fresh CLI resolution can check that choice. At 22:14 UTC on 3 October 2026, the
running root had repeatedly received `current_time_reminder` developer messages,
had current context-window metadata, and `get_context_remaining` returned
`tokens_left: 120014`. This establishes delivery in that root; exact config
provenance/cadence, threshold-warning firing, other agents, compaction continuity,
usefulness and cache neutrality remain unverified.

## A: approved provisional time-reminder trial

Codex automatically appends a UTC timestamp at eligible user/tool-output
boundaries when at least 60 seconds have elapsed; the initial/new-window
reminder is delivered regardless of interval. It does not wake an agent every
minute or enforce a deadline. The proposed interruptible sleep-tool availability
and accurate encryption-limit comment are retained.

The cadence, behavioral benefit and cache/cost effects are unmeasured. Review
actual reminders and lengthen the interval or disable if they are too frequent
or unhelpful. The earlier prefix is preserved by appended reminders, but that
does not establish cache neutrality. No paid experiment or daemon restart was
selected. The stock clock-error behavior remains.

Applied config block:

```toml
# Capability limit checked at CLI 0.160.0: no supported config switch to disable
# V2 message encryption. Production support for a plaintext-forcing client patch
# is unverified; do not claim client patches are proved ineffective (docs/codex.md).

# PROVISIONAL TRIAL: approved by Jörn on 2026-10-03, assessed at CLI 0.160.0.
# Codex injects UTC timestamps automatically for elapsed-time awareness. The 60s
# cadence is speculative: Jörn considers it fast; usefulness and cache/cost
# effects are unmeasured. Recheck after observing actual reminders; lengthen the
# interval or disable if too frequent or unhelpful. This is not a deadline.
[features.current_time_reminder]
enabled = true
reminder_interval_seconds = 60
clock_source = "system"
delivery_mode = "after_user_or_tool_output"
# Preserve availability of the input-interruptible clock.sleep described in A;
# this explicit setting can also expose it for other selected models.
sleep_tool = true
```

## B: approved provisional remaining-context trial

Jörn explicitly approved enabling and marking this trial on 3 October. The
project config now exposes context-window metadata, `get_context_remaining`
and `new_context_window`, plus a warning below 20,000 tokens remaining before
compaction. This threshold is speculative; usefulness, model compatibility and
continuity across compaction must be observed before calling it established.

This tracks context capacity rather than expenditure. The optional history/notes
extension is explicitly off. The rollout hard-stop feature remains off.

Applied config block:

```toml
# PROVISIONAL TRIAL: approved by Jörn on 2026-10-03, assessed at CLI 0.160.0.
# Expose remaining context capacity and context-window tools. Warn once per
# window below 20,000 tokens before auto-compaction; this starting threshold is
# speculative, not a measured optimum or a spending allowance. Recheck useful
# warnings, model compatibility and continuity across compaction; revise/disable
# if unhelpful. See docs/codex.md and docs/codex-config-proposals.md.
[features.token_budget]
enabled = true
reminder_threshold_tokens = 20000
# The optional history/notes extension is outside this trial.
use_history_notes_extension = false
```

## Choices not bundled into these proposals

No rollout hard limit, model/effort override, permission change, experimental
context/history extension, new encryption key, source patch, build, restart or
paid cache experiment is selected. `rollout_budget` requires a chosen allowance
and reminder thresholds; it cannot be enabled as merely a passive meter.

See [the maintained assessment](codex.md#time-context-capacity-expenditure-and-caching)
and [full generated flag inventory](codex-features.md). TOML/schema validation can
check these candidates without activating them; it does not establish successful
live adoption or behavioral benefit.

## Later candidates and cumulative-usage preference

Jörn did not select developer-message retention, deferred mailbox preemption or
deferred-tool world state: no concrete solved problem was identified for these
changes. Their fresh CLI values remain off; do not present them as pending trials.

He prefers expenditure information that counts upward. Stock `rollout_budget`
adds remaining-budget developer messages initially/per new window and at crossed
thresholds; no dedicated query tool is registered by this flag. It requires a
positive cap, with stopping on exhaustion, and has no count-up-only setting.
`get_goal` can report accumulated goal token/time usage without a token cap, but
requires an explicitly created goal and comes with automatic goal continuation.
This is a candidate distinction, not selection of a goal, a cap or a new meter.
See the source-backed [assessment](codex.md#time-context-capacity-expenditure-and-caching).

<a id="c-prepared-ongoing-work-count-up-reminder-activation-awaits-approval"></a>

## C: withdrawn ongoing-work count-up reminder; historical candidate

**Withdrawn on 3 October after Jörn rejected its complexity. No approval is
pending and no activation is selected.** The following retains the rejected
candidate for provenance; the repaired counters and on-demand command remain.

Jörn instructed continued resource-workflow completion after accounting repairs,
and rejected task-start/handoff delivery. The prepared mechanism supplies a
compact observation after successful tool results during ongoing work. Each
receiving agent gets its own count and its shared root/worker-tree count, plus
change since the previous comparable reminder. Units are noncached input plus
output without model weighting; they are not subscription quota or billed spend.
The message includes its scope, cutoff, latest recorded usage, errors and the
on-demand command. Ordinary forks are separate; no task assignment is guessed.

`python3 scripts/usage-awareness.py --json` is usable now. Its `--hook` mode is
not active. The candidate runs once at the first eligible tool result, then
at most once per five minutes per agent, suppressing unchanged observations and
repeated errors. Five minutes is provisional, motivated by Jörn's earlier view
that a 60-second time cadence is fast; usefulness remains to be observed.
It cannot interrupt uninterrupted reasoning or a running tool; the next eligible
result delivers it. This differs from a timer that wakes an agent.

The candidate stores only counter/checkpoint fields under
`~/.cache/msc-math/usage-reminders/`. It runs without model calls, watches or
listeners. A lock suppresses overlapping callbacks. Errors inform rather than
stop the agent. The command discards hook tool-input/output fields and never
retains them. Permission/stop controls and model settings are unchanged.

Eleven delivery/accounting tests pass, including worker identity, throttling
without scanning, delta changes, ordinary-fork independence, error suppression,
partial checkpoints and body non-retention. The repaired underlying accounting
has 35 focused tests. A real local command invocation took 0.544 seconds to read
this three-thread tree and 0.039 seconds on the throttled path; these are single
observations, not a performance guarantee. A fresh temporary CLI 0.160.0
app-server loaded the exact declaration and listed its normalized settings/hash,
first untrusted and then trusted with the matching temporary user-layer entry.
No inference request or live configuration change was made. Actual automatic
model-visible delivery remains an activation check, not a completed claim.

The project declaration proposed for `.codex/config.toml` is:

```toml
# PROVISIONAL CANDIDATE: periodic count-up resource awareness during work.
# First successful tool result, then at most once per five minutes per agent.
# Cadence is a trial value, not a budget or measured optimum. Reads owned local
# usage and stores counter-only checkpoints; no quota/billing conversion.
# Candidate needs explicit project-config approval and user-layer hook trust.
[[hooks.PostToolUse]]
matcher = "*"
[[hooks.PostToolUse.hooks]]
type = "command"
command = "python3 /home/joern/.codex/worktrees/60b9/msc-math/scripts/usage-awareness.py --hook --interval-seconds 300"
timeout = 5
additionalContextLimit = 0
```

Codex reads hook trust from the user/session-flags layer, not project trust
settings. The accompanying append-only user-layer fragment for
`~/.codex/config.toml` is:

```toml
# PROVISIONAL msc-math ongoing-usage reminder; project declaration approved separately.
[hooks.state."/home/joern/.codex/worktrees/60b9/msc-math/.codex/config.toml:post_tool_use:0:0"]
trusted_hash = "sha256:ef98e1ed7b6750db1b11114a383260718d0d1f5f8eb45813779e6d113457dbf8"
```

This trusts the one declared command at the one project path. It does not bypass
hook trust or modify existing entries. Both loaded configuration edits require
the recorded explicit approval before application. After approval, the owner
must check runtime adoption and receive an automatic report before claiming
delivery. If adoption requires a restart, present that concrete requirement
instead of treating a successful fresh loader check as live adoption.

Release-matched sources:
[post-tool command contract](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/hooks/src/events/post_tool_use.rs#L27),
[additional-context delivery](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/registry.rs#L734),
[trust layers](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/hooks/src/config_rules.rs#L9),
[declaration hashing](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/hooks/src/engine/discovery.rs#L775).
