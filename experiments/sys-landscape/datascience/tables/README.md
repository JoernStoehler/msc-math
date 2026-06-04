# Sys-Landscape Datascience Tables

This directory owns the single dataset stage for `experiments/sys-landscape/datascience/`.

Pipeline:
- `produce/` writes cache and summary JSONL files
- `tables/main.rs` loads those files, enriches them, and writes the final tables
- `methods/` reads the written tables as black-box inputs

Current outputs:
- `polytope-table.jsonl`
- `observation-table.jsonl`

For method waves, the table output should live under:

```text
experiments/sys-landscape/datascience/dataset/
```

Build the current dataset with:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Code ownership:
- `main.rs` orchestrates `load -> enrich -> write`
- `load_caches.rs` reads producer files and merges them into unified rows
- `features.rs` computes the polytope-level columns
- `features_trace.rs` computes the observation / trace columns
- `write_database.rs` writes the final JSONL tables

Smoke path:
- [smoke-pipeline.sh](../smoke-pipeline.sh)

This path writes only to temporary directories, but it can still take minutes;
use it as integration smoke, not as a cheap command check.
