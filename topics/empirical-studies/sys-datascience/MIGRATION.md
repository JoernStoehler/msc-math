# Path migration and frozen records

On 12 October 2026 the complete `experiments/sys-datascience/` tree moved to
`topics/empirical-studies/sys-datascience/`. Its dataset, prepared-table and
landscape siblings moved alongside it. The scientific packets and retained
outputs keep their internal structure; the move does not regenerate evidence.

| Original repository prefix | Current repository prefix |
| --- | --- |
| `experiments/sys-datascience/` | `topics/empirical-studies/sys-datascience/` |
| `experiments/polytope-datasets/` | `topics/empirical-studies/polytope-datasets/` |
| `experiments/polytope-invariant-table/` | `topics/empirical-studies/polytope-invariant-table/` |
| `experiments/sys-landscape/` | `topics/empirical-studies/sys-landscape/` |

Active source imports, manifests, repository-root calculations, path defaults
and Markdown links follow the current location. Sibling-relative feature
imports remain relative to the same sibling packages. Frozen artifact/source
snapshots, numerical outputs, designs, configs, receipts and historical path
labels remain byte-identical. An original path inside a retained manifest is
part of that record, not a claim that the old live file still exists.

[The Python resolver](methods/_shared/relocation.py) translates these original
labels only when reading current files. The orientation reader keeps original
labels for schema/identity comparisons and `git show` at retained commits;
the endpoint checker also distinguishes retained-commit bytes from current
relocated bytes. Their source-drift warnings remain meaningful. The orientation
Rust evaluator uses the same distinction for current file reads. The scalar
proposer resolves a frozen config's old repository-relative output label to
its new packet location without rewriting the config. Use an explicit fresh
`--out-dir` for deliberate new runs.

`source-v1/`, provenance source snapshots, artifact JSONL/TSV/JSON and retained
adapter controls are historical evidence. Snapshot manifests are meant to be
reconstructed in their original checkout context, not compiled in place after
the move. Existing source/build receipts do not authenticate a freshly built
relocated executable: rebuild and obtain a new receipt if fresh evaluation is
separately selected. [Current-body adapter](methods/current-body-evaluator/README.md)
records actual build/source bytes for that purpose; the retained controls are
still readable without target calls.

[The diagonal CEM packet](methods/diagonal-cem-pilot/README.md) binds its analyzer
to historical source and `.git/codex/...` executable hashes. Its default
analysis is therefore a historical reconstruction route, not a portable
current rerun. Reconstructing the original commit named in its frozen config
restores source paths but does not by itself recover executable bytes. Inspect
its retained summary and raw rows for ordinary use; do not weaken hash checks,
rewrite the frozen config or overwrite outputs merely to make a rerun pass.

Migration checks use synthetic consumer boundaries, retained-control readers,
manifest metadata and standalone compilation. No producer population, capacity
request or scientific analyzer is rerun by this migration.
