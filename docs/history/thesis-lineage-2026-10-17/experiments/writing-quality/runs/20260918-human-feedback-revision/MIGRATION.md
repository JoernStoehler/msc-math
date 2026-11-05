# Historical figure-source snapshot and current reproduction route

12 October 2026. This September run is retained historical writing/review
material. Its `hashes.json` freezes both READMEs, the plot source and generated
figures/receipt; those bytes remain unchanged during repository migration.
Its plot script's default provenance path therefore records the original
`experiments/polytope-invariant-table/` location and is not the current default
entry point.

The maintained plot producer is
[thesis/figures/plot_association.py](../../../../thesis/figures/plot_association.py),
whose current default provenance path is
`topics/empirical-studies/polytope-invariant-table/polytope-provenance-table.jsonl`.
For new outputs, the maintained thesis script accepts a scratch `--out`
directory. To reconstruct with the original frozen script, first copy it into a
scratch directory: it writes beside `__file__` and has no `--out` option. Pass
the relocated provenance-table path with `--provenance`; its recorded input SHA-256
requirements remain unchanged. Do not overwrite the retained figure snapshot
or rewrite its hashes merely to follow current folder names. No plot generation
or scientific computation was rerun for this migration.
