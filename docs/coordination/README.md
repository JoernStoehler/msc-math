# Project coordination

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
is not yet defined; `graph_label` supplies a compact readable label. A crux is
an unresolved choice or evidence question, not a special status. A ready task
without an owner is available planning work, not a running session. Unselected
and parked candidates are visible without implying execution authorization.

The `workflow-done` milestone means the selected project-local skills and
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
the dependent, with PROJECT DONE at the top. Solid arrows describe selected
dependencies; dashed arrows connect proposals to unselected candidate routes.
Completed assignments and parked routes use compact summary nodes; individual
outcomes and gaps remain in the registry and dashboard task rows.
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
