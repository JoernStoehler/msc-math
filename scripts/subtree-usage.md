# Subtree usage

**Closure verdict:** Jörn graded this Codex config/resource-workflow attempt FAIL.
The component repairs and scoped checks described here remain evidence; they
are not human acceptance of the workflow. See [current handoff](../docs/coordination/handoff.md).

**Accounting fixes applied on 3 October.** Owned response records now prevent
copied-fork double counting, ordinary forks stay separate from workers, and
missing required fields are rejected. Legacy forks without an ownership
boundary produce an explicit diagnostic. The [ongoing-work report](#ongoing-work-awareness)
is usable on demand; its automatic-hook candidate was withdrawn on 3 October
after Jörn rejected its complexity. No activation approval is pending.

```sh
python3 scripts/subtree-usage.py
python3 scripts/subtree-usage.py ROOT_THREAD_ID
```

Without an ID, uses `CODEX_THREAD_ID` and follows available parent metadata to
the top-level root. An explicit ID selects precisely that subtree. Run from
any directory using the script's absolute path. Requires only Python's standard
library; reads local logs, makes no network requests and changes no files.

Reports root start/current UTC time, elapsed time, latest recorded usage,
total/last-30-minute/last-5-minute consumption, model breakdown and root versus
all descendants. It does not stop agents or measure Jörn-time. Start a fresh root for a
fresh sprint total; an old root includes its previous work and idle wall time.

## Accounting convention

The checked-in rates are **Standard API-equivalent shadow prices as of
2026-09-16**, not actual subscription quota or billed spend, and not the older
project price table. Sources:

- https://developers.openai.com/api/docs/models/gpt-6-astra
- https://developers.openai.com/api/docs/models/gpt-5.6-sol
- https://developers.openai.com/api/docs/models/gpt-5.6-luna

The Sol/Luna pages also list the comparison rates used for Terra, GPT-5.5 and
GPT-5.4. All usage, including historical usage, is valued at this fixed rate
card. No service-tier/fast-mode surcharge is inferred. API long-context
multipliers apply per recorded request above 272,000 input tokens. Cache-write
tokens, when present, use 1.25 times the uncached-input rate. Tool fees are
excluded. Reasoning output is already included in output, not charged twice.

Current `token_usage_record` envelopes supply thread ownership and response IDs;
only records owned by the selected thread count, and repeated response IDs count
once. Cumulative `token_count` mirrors do not add another charge. Root histories
that transition from legacy counters to response records retain earlier usage.

Legacy counters require the input, cached-input and output fields. Missing
optional cache-write fields remain unknown, including their split-input and
price consequences. Counter-delta discrepancies warn rather than inventing a
missing request or its model. For copied legacy fork history, a thread-owned
settings snapshot or subagent history ordinal separates inherited counters from
new work. The initial fork counter alone is an ambiguous baseline; it is never
charged as a new request. If no ownership boundary is available, legacy fork
usage is excluded with the thread ID and reason. Owned response records remain
usable independently of those boundaries.

Descendants follow spawn-parent metadata, not `forked_from_id`. An ordinary fork
has its own scope and does not become a worker of its source. Native/archive
copies of the same thread count once (largest file). Only locally available logs
can be read.
`--codex-home PATH` selects an imported Codex home containing `sessions/` and/or
`archived_sessions/`. `--now ISO_TIMESTAMP` freezes the reporting cutoff.

Windows are based on usage-event timestamps, not token generation times.
In-flight/unflushed calls are not yet counted. Detected missing prices or malformed,
inconsistent, or absent usage telemetry print warnings and return exit code 2;
unpriced rows show `+?`, never a fabricated complete dollar total. Unknown or
unavailable descendants cannot be discovered from absent metadata.
When accounting errors occur, numeric rows are labelled observed subtotals.
An unpriced model is a separate pricing diagnostic; it does not invalidate its
recorded token counts. Other excluded or omitted records name their source/reason.

Tests: `python3 -m unittest discover -s scripts -p test_subtree_usage.py`.

## Ordinary sessions and other measurement routes

This report works without a goal, `token_budget` or `rollout_budget`. It counts
up observed usage without imposing an allowance or stopping work. Cached input,
cache writes and output remain separate rather than being collapsed into one
unqualified resource number.

`scripts/coordination-load.py ROOT_THREAD_ID --minutes 15 --brief` additionally
queries bounded OpenObserve telemetry and can include other explicitly named or
registry-owned project sessions. Source the documented OTLP authentication
environment first; [the coordination guide](../docs/coordination/README.md#workload-supervision)
owns the command. Since the 3 October attribution correction, its brief report
separates this root's workers from other scoped session trees; full JSON retains
per-thread counters and scope roles. Local fallback and native observations stay
separate.

### Native completion contract and bounded check

On 3 October, `coordination-load.py` was corrected to separate counterless raw
SSE completion polls marked by `duration_ms` as `raw_transport_completion_events`.
They do not count as completed usage, cause missing-counter diagnostics or erase
a known cache-write zero. Partial/malformed counters and unmarked counterless
completions remain diagnostics; raw-only usage totals remain unknown. CLI 0.160.0
logs [raw polls](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/otel/src/events/session_telemetry.rs#L977)
separately from [parsed usage](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/otel/src/events/session_telemetry.rs#L1102).
Twenty focused reader tests passed; rerun with
`python3 -m unittest discover -s scripts -p test_coordination_load.py`.

The selected-root-only safe-projection query for 3 October 2026,
22:10:46–22:25:46 UTC returned 134 metadata rows: 29 parsed completions, all
valid, zero explicit nulls, and 105 rows omitting counter keys; `duration_ms` was
available. This websocket root supplied no observed raw-SSE completion pair, so
the raw branch was not live exercised. Native export deduplication remains
unverified; no duplicate was established in the live tree.

A live 3 October 2026 check succeeded for CLI 0.160.0, discovering this config
walkthrough's root and two descendants. `gpt-6.1-sol` is absent from the fixed
16 September rate table, so its dollar estimate is unknown. Do not read `0.00+?`
as zero cost. No current-price or actual-billing validation was performed.

Native `token_count` records can also contain account-wide rate-limit snapshots:
used percentage, window duration and reset timestamp. The latest inspected
snapshot at 2026-10-03T20:47:22Z reported 2% used in a 10,080-minute window, with
no secondary window. This is a dated account observation, not this thread's quota
consumption or a live balance. The helper does not currently report those fields;
the mapping from attributed model/token counters to subscription quota remains
unestablished. Separate roots, cloud and Pro activity may share account quota
while being outside the local attribution scope.

## Workflow and trust review, 3 October 2026

The initial review found the defects below despite twenty passing tests. Jörn
then instructed fixing them and rejected the proposed task-start/handoff
delivery. The current disposition is:

| Requirement | Observed implementation and gap |
| --- | --- |
| Agent discovery | The coordination guide and Codex reference link the helpers. The scripts index previously omitted them. There is no dedicated callable accounting tool or accounting skill in the inspected project catalog. Documentation routing does not verify that agents retrieve or use it. |
| Automatic delivery | Helpers remain available on demand; the hook candidate is withdrawn. A bounded supervisor delivered three count-up notes during ongoing audit/repair, with root and active-worker receipt confirmed; see [the scoped check](#ongoing-work-awareness). This establishes supervised delivery using existing agents, not a permanent service or all-agent adoption. Native time/context reminders deliver different information. |
| Session attribution | Local metadata supplies lineage; token/context envelopes supply counters and models. OpenObserve supplies bounded per-conversation events. The scripts do not read transcript prose. Registry owners select other sessions, but there is no event-time task/packet assignment join, work-versus-wait attribution, or cross-cloud/Pro reconciliation. |
| Fork attribution | Repaired: spawn ancestry and fork origin are separate. The cumulative report and telemetry fallback share the owned-response/legacy-boundary reader. Regression cases cover copied and referenced forks, ordinal boundaries, resumes, and replay. Ambiguous old counters produce an explicit diagnostic. |
| Unknown data | Repaired: missing required fields and malformed numbers are rejected without zero defaults or an aborted report; missing optional fields remain unknown. Both local readers now check legacy counter discrepancies. Native export deduplication remains unverified; mixed valid/invalid native records leave observed subtotals with error diagnostics. |
| Resource interpretation | Token categories are available as diagnostics. The price table is dated and incomplete. Subscription quota and billed spend cannot be derived from these totals using an established conversion. The bounded supervised check establishes receipt in its selected sessions, not general automatic injection. |

The pre-fix copied-history defect is source-supported at CLI 0.160.0, commit
`a956835d020762cb2b570053af06f643a11c0ecc`:
[fork reconstruction/persistence](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/mod.rs#L1620)
can persist the inherited prefix, while the
[JSONL writer](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/rollout/src/recorder.rs#L2071)
assigns the write time to each item. A synthetic probe matching that persistence
shape supplied one copied counter plus one newly incurred counter: the helper
counted two completions, with no warning, when only one belonged to the fork.
Another probe omitted the last-request input field and observed a zero input
count with no warning. These are reproduced helper defects, not measurements
of the numerical error in any particular live session. No new fork was launched.

The approved repair is implemented in the shared reader. Thirty-five focused
tests pass, including the regression cases above. A live fixed-cutoff check of
this root and its two workers matched all 189 unique owned response records
and their token counts; the repaired OpenObserve checker also queried successfully.
These checks verify the selected fixes, not actual billing or behavioral use.

The earlier task-start/handoff delivery recommendation was withdrawn. Jörn then
instructed continuing the incomplete workflow. The resulting after-tool candidate
below was also withdrawn after Jörn rejected its complexity. Its rejection alone
did not close the parent outcome of ongoing resource awareness; the subsequent
bounded delivery check completed the selected local package.

## Ongoing-work awareness

```sh
python3 scripts/usage-awareness.py
python3 scripts/usage-awareness.py --json
```

This shared-reader report gives the current thread's count, the whole local
spawned tree's count, per-thread rows and explicit errors. Units are noncached
input plus output, without model weighting. This is the count-up proxy previously
discussed, independent of goals, caps and the incomplete price table. Unfinished
calls enter the report after usage is recorded. Ordinary forks have their own
root. The report uses local accounting envelopes, not transcript interpretation.

On 3 October 2026, the supervisor independently inspected actual root actions
and delivered count-up notes during useful audit/repair at 22:19:38, 22:22:11 and
22:25:20 UTC. Root and the active worker confirmed receipt. A direct worker-to-newer-
supervisor sibling acknowledgment was rejected; worker → root → supervisor
relayed receipt successfully. This demonstrates bounded ongoing supervised
delivery using the existing reader and agent transport. The selected local
attribution/discovery assessment, accounting repairs and bounded ongoing delivery
are complete; future monitoring remains an operational responsibility. Permanent
or all-agent adoption, quota conversion and native-export deduplication remain
unestablished and outside this completion claim. Broader workflow and project
milestones remain open.

The same script's `--hook` command produces Codex `PostToolUse` additional context,
with provisional five-minute per-agent throttling, comparable checkpoint deltas,
no-change/error suppression, and an on-demand command in the message. It stores
counter-only state in `~/.cache/msc-math/usage-reminders/`. This describes the
withdrawn candidate, which is not activated; no configuration decision is
pending. Its [exact declaration, trust entry and checks](../docs/codex-config-proposals.md#c-prepared-ongoing-work-count-up-reminder-activation-awaits-approval)
are retained as historical provenance. Eleven focused awareness tests and fresh
CLI declaration/trust loading passed; actual automatic delivery was not verified.
Run the awareness tests with
`python3 -m unittest discover -s scripts -p test_usage_awareness.py`.
