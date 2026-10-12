# Design notes

## Generated portfolio ownership, 6 October 2026

The coordination registry became the sole portfolio owner, while this skill retained manual `tasks.dot` edits and generic rendering. Those edits would be overwritten and generic rendering removes the state-hash marker. Portfolio changes now go through current.json and render-workflow-graph.py. Optional manually owned component graphs remain useful for richer decision branches; their detail must agree with selected work.

Crux/hedge and shared-cause guidance is retained. Its benefit in actual planning remains a question for Jörn's sweep, distinct from source freshness. Keep runtime state in coordination and historical execution overlays under history; do not make design memory another assignment store. See the [pre-sweep assessment](../../../../docs/history/skill-presweep-20261006/README.md).
