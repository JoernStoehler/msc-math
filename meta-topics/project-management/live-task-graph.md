# Live task graph binding

The reusable [live-task-graph](/workspaces/agent-fleet-tools/skills/live-task-graph/SKILL.md)
workflow is centrally maintained. This page supplies msc-specific storage and
execution paths; [coordination.md](coordination.md) owns selection, assignment,
resource and authorization semantics.

`current.json` owns selected assignments and resource observations. The semantic
dashboard is Jörn's primary view; DOT/SVG is an optional export. From the assigned
checkout root:

```bash
python3 scripts/render-workflow-graph.py
python3 scripts/render-workflow-graph.py --check
```

`graphs/tasks.dot` and `graphs/tasks.svg` are generated from `current.json`.
Edit the registry, then regenerate; do not edit those generated files. The check
compares generated DOT and the SVG's embedded state hash. It proves freshness,
not mathematics or owner liveness; generic Graphviz rendering loses the marker.
Explicitly selected manual components such as `00-now.dot` may be rendered with
`dot -Tsvg`, and their outcome/branch detail must agree with selected work.

Exploratory graph inputs, outputs and metadata start under
`/tmp/msc-math/<task>/<run>/`. Promote retained coordination artifacts
deliberately. Keep completed execution overlays under dated `graphs/history/`
when they no longer describe current assignments or deadlines.
