# Task graph

`tasks.dot` is the current overview; `00-now.dot` records the prepared handoff.
Their SVGs are rendered views. The source recovery and preparation are complete;
literature and pentagon replacements are ready but not integrated. These graphs
do not start a new sprint or establish a deadline. The kickoff packet and
current task ownership remain in `docs/sprint-two/KICKOFF.md` and
`memories/todos.md`.

All earlier components, SVGs, and the data-science action notes were copied
unchanged from `.git/codex/task-graph/` into `history/2026-09-14/`. Their old
assignments and September 14 target are historical, not active instructions.
The originals under `.git/` remain untouched. The previous skill's explicit
Git-common-directory rule applied to graph storage; it did not direct thesis
sources into `.git/`.

Render and view from the repository root:

```sh
for source in docs/task-graph/*.dot; do
  dot -Tsvg "$source" -o "${source%.dot}.svg"
done
scripts/view-graph.sh --list
scripts/view-graph.sh 00-now.dot
```

Graphviz provides `dot`; `graph-easy` supplies the optional terminal view.
The viewer's old `codex/task-graph/FILE` argument maps to the new location.
Graph state now follows ordinary checkout/version-control semantics rather
than implicitly sharing hidden state between worktrees. This migration has
not been committed.
