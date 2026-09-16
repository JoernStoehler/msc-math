---
name: live-task-graph
description: Maintain a shared, rendered dependency graph for long-running or multi-agent project work, including blockers, decision cruxes, owners, branches, and completed outcomes. Do not use for a short linear todo list.
---

# Live task graph

Use the graph as a compact projection of the work, not as a substitute for
investigation or as evidence that a result is correct. The coordinator owns
updates. Subagents report evidence and outcomes to the coordinator rather than
editing the graph concurrently.

## Storage

Keep graph sources and their rendered views in ordinary project storage.
Resolve the location from the current checkout with:

```bash
graph_dir="$(git rev-parse --show-toplevel)/docs/task-graph"
mkdir -p "$graph_dir"
```

Use `$graph_dir/tasks.dot` as the portfolio overview. When one graph would hide
important outcomes or become awkward in a terminal, add ordered component
sources such as `10-thesis.dot` and `20-software.dot`; do not make the overview
a duplicate of every detail. Render each source beside it as `.svg`. These
files belong to the project and follow its normal version-control workflow;
different worktrees do not implicitly share updates. Recover existing graphs;
never replace them merely because a new agent session started. When a completed
execution window leaves stale assignments or deadlines, preserve its components
under a dated `history/` directory and update the live overview and overlay.

For a multi-hour execution window, use `00-now.dot` as a small disposable
overlay containing only the selected live portfolio, owners, timeboxes, hedges,
kill/fan-in conditions, and reserved integration capacity. It is not another
backlog. Keep durable domain outcomes in the later component files.

## Model

Use Graphviz DOT with stable node identifiers. Point edges from prerequisite
to dependent. Use labeled outgoing edges from a diamond for outcomes that
change downstream work. Use dashed edges for non-blocking influence.

Do not encode a merely hoped-for result as an ordinary prerequisite edge. When
an operation may plausibly produce materially different results, make it a
crux and show the consequential branches. Cover at least success, useful
partial information, and failure or evidence that invalidates the planned
route when those outcomes imply different next work. By default, downstream
work belongs only on the branch that justifies it. An explicit `HEDGE` may
start before the crux resolves when its cost is bounded, parallel capacity is
genuinely spare, and buying the option now reduces deadline risk more than
waiting. Give each hedge a kill or fan-in condition; do not let speculative
work silently become permanent scope.

For a non-routine operation in the live overlay, account for four dispositions
when material: intended result, useful partial result, evidence that invalidates
the route, and timeout. A failed attempt can unlock a different route; it is not
automatically a dead end or permission to repeat the same strategy.

Distinguish robustly instrumental work from plan-contingent work. A robust task
is worth doing early because its output remains useful across most plausible
outcomes; show its influence on those branches. When a chain contains several
uncertain steps, expose the uncertainty instead of presenting their joint
success as the plan. Add probabilities only when they are decision-relevant
and evidence supports the estimate; otherwise use plain confidence language
such as `routine`, `uncertain`, or `speculative`. Treat coordinator attention
as a limited resource too: prompts, message triage, branch review, and
integration compete with direct work. When fan-out would saturate that
attention, prefer fewer bounded assignments with compact return contracts and
preserve capacity for fan-in rather than launching work that cannot be
evaluated in time.

Do not multiply marginal success or failure probabilities unless causal
independence is actually justified. Project outcomes usually share causes:
source quality, mathematical premises, manuscript state, environment, reviewer
criteria, or integration capacity. Represent a material shared cause as its own
node or scenario branch and connect everything it can move. Correlation can be
helpful too: one sound theorem, dataset, or interface may unlock several
downstream outcomes together. Calling parallel work a hedge requires a
meaningfully different failure mechanism; several agents repeating the same
approach are capacity replication, not diversification.

Prefer outcome nodes over activity narration. A live node should communicate:

- status: `ready`, `running`, `blocked`, `crux`, `done`, or `dropped`;
- the result required, or the result obtained when done;
- owner and branch/worktree when running;
- the concrete dependency when blocked; and
- for a crux, the bounded probe and material outcome branches.

Do not record a cheap unresolved investigation as a future blocker: perform it
first. A known unknown belongs in the graph only when resolving it is material
and not already practical within the current task. Graph state does not grant
merge permission or authorize external or destructive actions.

Keep the critical path visually distinct from background work. Retain compact
completed outcomes while they explain later decisions; drop process history
that no longer affects the graph.

## Update points

Patch `tasks.dot` after a material state change: work starts or finishes, a
dependency is discovered or removed, evidence changes a decision branch, a
branch becomes reviewable, or work is explicitly dropped. Do not churn the
file for routine progress messages.

After each patch, validate and render:

```bash
for source in "$graph_dir"/*.dot; do
  dot -Tsvg "$source" -o "${source%.dot}.svg"
done
```

Treat a failed render as a broken task map and repair it immediately.

## Jörn's view

Publish a narrow review URL reachable from Jörn's current device; do not assume
that the agent and Jörn share a desktop or Chrome instance.

When the repository provides `scripts/view-graph.sh`, use it for manual review
of one component instead of making Jörn locate its source path:

```bash
scripts/view-graph.sh 00-now.dot
```

When Jörn is working in Herdr and `graph-easy` and `inotifywait` are installed
in the Herdr session's environment, prefer a dedicated terminal pane with
scrollable, event-driven box-art:

Soft wrapping corrupts box-art topology. Before handoff, inspect the target
pane width and measure the longest rendered line of every component. Keep a
small margin (normally four or more columns). If a component is too wide,
shorten labels or split it by crux; do not rely on horizontal scrolling or call
wrapped output valid merely because Graphviz accepted the DOT.

```bash
show_graphs() {
  for source in "$graph_dir/tasks.dot" "$graph_dir"/[0-9][0-9]-*.dot; do
    [ -f "$source" ] || continue
    printf '\n===== %s =====\n\n' "${source##*/}"
    graph-easy --from=dot --as=boxart "$source"
  done
}
show_graphs
while inotifywait -qq -e close_write,moved_to --include='\.dot$' "$graph_dir"; do
  show_graphs
done
```

This needs no listener, works through remote Herdr/SSH, appends a new complete
snapshot only after an update, and leaves Ctrl+C as the ordinary way to stop
the viewer. Do not use `watch`: its alternate screen prevents Herdr from
retaining a graph taller than the viewport in scrollback. Herdr itself does not
supply a graph renderer; `graph-easy` and `inotifywait` are optional environment
dependencies. Do not install them or change pane layout without the authority
normally required for those actions.

When the `review-files` skill is available in the current environment, use its
`--print-only` route for `tasks.svg` and give Jörn the returned Tailscale URL.
Re-render into the same path after updates; the URL remains the viewing surface
for the server lifetime and Jörn can reload it from a Chromebook, phone, or the
host. Open a desktop-local tab only when Jörn explicitly says that is useful.

A sandbox agent must not infer that it can bind a host listener, configure
Tailscale, or use an offered port-forwarding feature. If the checkout is
host-mounted and a host coordinator is reachable, render the graph in the
ordinary project directory and hand that coordinator the exact SVG path for
publication. If no such route exists, report the rendered path and the missing
publishing capability instead of starting a broad HTTP server or inventing
network setup.

Do not introduce a custom server, watcher, or port forward until manual reload
is shown to be a real burden and the environment owning the listener is known.
DOT plus Graphviz and the existing narrow Tailscale review server are the
smallest predictable stack. The DOT source remains readable in Micro when a
terminal-only view is preferable.
