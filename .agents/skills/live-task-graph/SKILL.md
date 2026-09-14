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

Keep the skill in the repository, but keep live state outside tracked
worktrees. Resolve the shared location with:

```bash
graph_dir="$(git rev-parse --path-format=absolute --git-common-dir)/codex/task-graph"
mkdir -p "$graph_dir"
```

Use `$graph_dir/tasks.dot` as the source and `$graph_dir/tasks.svg` as the
rendered view. The Git common directory is shared by all worktrees and does not
create perpetual working-tree changes. Recover an existing graph; never
replace it merely because a new agent session started.

## Model

Use Graphviz DOT with stable node identifiers. Point edges from prerequisite
to dependent. Use labeled outgoing edges from a diamond for outcomes that
change downstream work. Use dashed edges for non-blocking influence.

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
dot -Tsvg "$graph_dir/tasks.dot" -o "$graph_dir/tasks.svg"
```

Treat a failed render as a broken task map and repair it immediately.

## Jörn's view

Publish a narrow review URL reachable from Jörn's current device; do not assume
that the agent and Jörn share a desktop or Chrome instance.

When Jörn is working in Herdr and `graph-easy` is installed in the Herdr
session's environment, prefer a dedicated terminal pane with live box-art:

```bash
watch -n 1 "graph-easy --from=dot --as=boxart \"$graph_dir/tasks.dot\""
```

This needs no listener, works through remote Herdr/SSH, and leaves Ctrl+C as the
ordinary way to stop the viewer. Herdr itself does not supply a graph renderer;
`graph-easy` is an optional environment dependency. Do not install it or
change pane layout without the authority normally required for those actions.

When the `review-files` skill is available in the current environment, use its
`--print-only` route for `tasks.svg` and give Jörn the returned Tailscale URL.
Re-render into the same path after updates; the URL remains the viewing surface
for the server lifetime and Jörn can reload it from a Chromebook, phone, or the
host. Open a desktop-local tab only when Jörn explicitly says that is useful.

A sandbox agent must not infer that it can bind a host listener, configure
Tailscale, or use an offered port-forwarding feature. If the checkout is
host-mounted and a host coordinator is reachable, render the graph in the
shared Git directory and hand that coordinator the exact SVG path for
publication. If no such route exists, report the rendered path and the missing
publishing capability instead of starting a broad HTTP server or inventing
network setup.

Do not introduce a custom server, watcher, or port forward until manual reload
is shown to be a real burden and the environment owning the listener is known.
DOT plus Graphviz and the existing narrow Tailscale review server are the
smallest predictable stack. The DOT source remains readable in Micro when a
terminal-only view is preferable.
