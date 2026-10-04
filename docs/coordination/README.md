# Project coordination

**Closure verdict:** Jörn graded this Codex config/resource-workflow attempt FAIL.
The component repairs and scoped checks described here remain evidence; they
are not human acceptance of the workflow. See [current handoff](handoff.md).

This is the agent-facing home for project management. Jörn's browser view is
[../dashboard/index.html](../dashboard/index.html); it renders the shared state
rather than maintaining another assignment list.

| Surface | Owns | Update when |
| --- | --- | --- |
| [current.json](current.json) | Selected work, owners, dependencies, decisions, resource observations and recent changes | An assignment, result, blocker, decision or resource constraint changes |
| [thesis-work.md](thesis-work.md) | Detailed thesis obligations, dispositions and completion checks | Source/evidence checking or Jörn changes an obligation |
| [handoff.md](handoff.md) | Current continuation constraints and recovery pointers | The next session would otherwise receive wrong directions |
| [../project-facts.md](../project-facts.md) | Attributed Jörn-confirmed scope and decisions | A confirmed fact changes; preserve attribution and date |

The registry covers work selected in reported sessions, its material planning
gaps and explicitly unselected or parked candidate routes. It is not every
possible task or every running process on the host. The thesis backlog is a separate detailed
owner, not a list of automatically authorized assignments. Research packets may
contain local obligations; their old next steps become project assignments only
when selected here. Use stable IDs and link the exact packet instead of copying
its scientific conclusions into coordination files.

## Updating shared state

The coordinating agent owns `current.json` for its execution window. Topic agents
report their changed outcome, source, status, owner and next dependency to that
coordinator. Independent sessions merge their changes before handoff; separate
worktrees do not share updates automatically. Read the current file before
editing and preserve other sessions' entries. Split into per-topic files only
when real editing conflicts or size warrant it; the present single registry
keeps discovery and rendering simple.

For each selected task record its outcome, status (`ready`, `running`, `blocked`,
`done`, `parked`, or `dropped`), owner (or null), source paths, and dependencies
by ID. A blocked task names the concrete reason. An owner names a current thread
or agreed person, not an old terminal pane. Record uncertainty as uncertainty.
State does not authorize thesis completion, external publication, or spending.

Optional task fields distinguish the outcome map from an assignment list:
`kind` is `task` (default), `milestone`, or `crux`; `selection` is `selected`
(default), `unselected`, or `parked`; `plan_gap` names material work whose route
is not yet defined; `graph_label` supplies a compact readable label. `project`
groups related branches; `authorization` records their actual execution scope;
`next_step` gives the next useful step or selection; `parallel_ready` means an
independent bounded candidate could run **if selected**, never permission to
start it. A READY unselected packet is technically available but unassigned.
A crux is
an unresolved choice or evidence question, not a special status. A ready task
without an owner is available planning work, not a running session. Unselected
and parked candidates are visible without implying execution authorization.

`msc-math-done` is the whole-project completion outcome. Its source-backed
model includes supported/readable selected thesis claims, code/evidence and
reproduction promises, one exact candidate reaching the recorded human PASS
threshold, actual administrative/archive dispositions, working selected
workflows and released assignments. Current administrative status is unknown:
historical unsubmitted/deadline notes do not establish a remaining hand-in.
This model is a planning projection; it does not establish readiness or
authorize execution. The detailed thesis backlog and exact domain sources
retain their authority. Optional research and writing-method trials are
visible alternatives, not automatic prerequisites of project completion.

The `workflow-done` subgoal means the selected project-local skills and
workflow deliverables are integrated, relevant acceptance is complete, and
their assignments and sessions are released. Returned plans alone cannot
close it. Explicitly deferring a capability preserves that gap without making
every proposed pilot mandatory. Thesis execution remains separately stopped.
Unavailable historical transport-cause evidence is separate from acceptance
of the selected communication route; successful reads alone do not verify send
delivery. Unknown usage and numeric spending thresholds do not create a gate
for every small authorized task.

Before planning or delegating substantial work, use the collaboration boundary
in [AGENTS.md](../../AGENTS.md) and its
[commentary](../../AGENTS.md.commentary.md). Record an unanswered spending/plan
decision here even when its async UI entry disappears. Ownership transfer must
name the receiving owner and preserve the approval boundary.

Timestamp meaningful observations in UTC. Resource observations name their
scope and evidence; unknown quota/spending stays null. Historical resource
ceilings are not current permission or balance. The page warns after 30 minutes
without an update; this is an age signal, not a liveness monitor. Mark work done
and release its owner when handing off. Keep recent consequential changes short;
use Git or `docs/history/` for the older record.

The compact dependency graph is generated solely from `current.json`:

```bash
python3 scripts/render-workflow-graph.py
python3 scripts/render-workflow-graph.py --check
```

[graphs/tasks.dot](graphs/tasks.dot) and [graphs/tasks.svg](graphs/tasks.svg)
are projections, not another canonical store. Prerequisite arrows point toward
the dependent, with msc-math PROJECT DONE at the top. Solid arrows describe selected
dependencies; dashed arrows connect proposals to unselected candidate routes.
Completed assignments, parked routes and independent candidates use compact
summary nodes. Their full outcomes, exact dependencies and execution boundaries
remain in the registry and the semantic dashboard, which is the primary view;
DOT/SVG is an optional export rather than the browser interaction model.
The SVG embeds the SHA256 of the exact state bytes; `--check` compares that
marker and the generated DOT without rerendering. Regenerate after updating
state, even if only an observation changed. This checks projection freshness,
not workflow reliability, session liveness or mathematical correctness. It
requires installed Graphviz to render and creates no listener.

Run `python3 scripts/serve-project-dashboard.py --check` before handoff. This
checks state structure, paths, dependencies and the browser's source contract;
it does not validate the mathematical claims or prove that an owner is alive.
Run the server with `python3 scripts/serve-project-dashboard.py`, then open
<http://127.0.0.1:8765/>. It rereads files and the browser refreshes reported
state every 15 seconds. The listener remains local unless explicitly configured.

## Messaging protocol

The [cross-session-messaging skill](../../.agents/skills/cross-session-messaging/SKILL.md)
owns guidance for sending or receiving cross-session agent messages and handoffs,
including messages relayed by Jörn. Its catalog description supplies the trigger;
the protocol is not duplicated in AGENTS.md or here.

## Workload supervision

The coordinator owns supervision, including its own pending messages, reviews,
integration work and delegatable context. The read-only workstation checker uses
existing OpenObserve export and local usage metadata:

```bash
. "$HOME/.config/openobserve/codex-env.sh"
python3 scripts/coordination-load.py ROOT_THREAD_UUID --minutes 15 --brief
```

Replace `ROOT_THREAD_UUID` with the actual conversation UUID. It includes local
descendants and UUID owners/related threads recorded in the registry; repeat
`--thread UUID` for an additional independent project session. Cloud and Pro
are outside coverage. It projects counters and event metadata, excluding
prompts, reasoning, tool content and credentials. Native observations and local
fallback remain separate; unknown counters and billed spend remain unknown.
Exit 2 means incomplete observation, not an overload diagnosis.

The compact report separates `root_native`, `workers_native` (locally discovered
descendants of this root) and `other_scoped_threads_native` (other recorded or
explicit sessions and their descendants). Fallback counters have the same three
separate groups and are never added to native totals. Root/worker context-size
comparisons use descendants only; unrelated roots are not workers. The full
report retains per-thread counters and `scope_role` for attribution. These are
passive measurements, with no goal or configured budget limit required.

The 3 October native-reader correction counts counterless `duration_ms`-marked
raw SSE completions separately as `raw_transport_completion_events`, without
inflating usage or hiding real counter errors. Twenty focused tests passed.
The selected-root-only 22:10:46–22:25:46 UTC safe-projection query returned 134
metadata rows and 29 valid parsed completions, with no explicit nulls; 105 rows
omitted counter keys. No raw-SSE pair was live observed in this websocket root.
Native export deduplication remains unverified; no live-tree duplicate was
established. [The retained contract](../../scripts/subtree-usage.md#native-completion-contract-and-bounded-check)
records source anchors, regression command and unknown-data boundaries.

For a cumulative local-log report by root/workers/model, use
`python3 scripts/subtree-usage.py ROOT_THREAD_UUID`; see its
[accounting contract](../../scripts/subtree-usage.md). Its dated shadow-price
table is not subscription quota. Account-wide native quota snapshots, where
available, do not establish which thread consumed which quota percentage.
The [3 October workflow/trust review](../../scripts/subtree-usage.md#workflow-and-trust-review-3-october-2026)
reproduced copied-fork double counting and silent zero defaults; Jörn instructed
fixing them. Both local readers now share response ownership/deduplication and
strict required counters, and ordinary forks stay separate from workers.
Thirty-five focused tests and a live owned-response sum check passed. The
proposed watch is not active automation; task-start/handoff delivery was rejected.
The on-demand `scripts/usage-awareness.py` command remains usable. Jörn rejected
the throttled after-tool candidate's complexity on 3 October; it is withdrawn,
with no activation approval pending. [Proposal C](../codex-config-proposals.md#c-prepared-ongoing-work-count-up-reminder-activation-awaits-approval)
retains the exact rejected additions and checks as historical provenance.
The withdrawn hook's automatic model-visible delivery was not verified. During
useful audit/repair on 3 October, the supervisor independently inspected root
actions and delivered three count-up notes at 22:19:38, 22:22:11 and 22:25:20 UTC;
root and the active worker confirmed receipt. Direct worker-to-newer-supervisor
sibling acknowledgment was rejected; worker → root → supervisor successfully
relayed receipt. This establishes bounded supervised delivery through the existing
reader and agent transport. The selected local attribution/discovery assessment,
accounting repairs and bounded ongoing delivery are complete; future monitoring
remains an operational responsibility. Permanent or all-agent adoption, quota
conversion and native-export deduplication remain unestablished and outside this
completion claim. Broader workflow and project milestones remain open.

For selected ongoing work, a bounded observer can run it every four minutes and
send new, materially changed or resolved warnings to its receiving coordinator.
Name the watch's stop/review point; no permanent service or paid model loop is
required. Use `collaboration.send_message` when available; tool availability
depends on the agent role. On 30 September the lightweight observer lacked
native send tools and successfully returned reports through its TUI thread
transport. Confirm the receiving route rather than assuming every child has
the coordinator's tools. Suppress repeated warnings
by signal type, subject and stable evidence, excluding observation timestamps
and silence durations. Relative context growth, cache misses and repeated error
events trigger review, not automatic spending gates or proof of overload.
Nested tool events can describe the same failure twice. Quiet telemetry can
mean waiting, in-flight work or missing export. Review actual pending work and
act on bottlenecks; never turn absent queue measurements into a zero backlog.

## Knowledge and human views

Reusable interpretation belongs in [../knowledge/](../knowledge/README.md),
with sources, limitations and triggers for rechecking. Proofs, implementations,
data and review evidence retain their domain owners. Do not create a parallel
`memories/` or `knowledge/` tree at the root.

[../dashboard/thesis.html](../dashboard/thesis.html) is a human-facing scientific
synthesis, separate from live work accounting. Before calling it current, check
Jörn's scope and PASS threshold, selected manuscript claims, caveats, evidence
owners and rendered PDF. Update its source baseline only after reconciliation;
until then show the affected claim or view as stale. Changed source hashes are
only a warning, not proof of correctness or completeness.

Historical stopped runs and replaced planning views are indexed at
[../history/coordination-surface-migration/](../history/coordination-surface-migration/README.md).
Preservation manifests and frozen PDF checks stay in `docs/resume/`; they are
recovery evidence, not current coordination state.
