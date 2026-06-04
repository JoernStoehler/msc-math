# Sys-Landscape Datascience Tables

This directory owns the single dataset stage for `experiments/sys-landscape/datascience/`.

Pipeline:
- `produce/` writes cache and summary JSONL files
- `tables/main.rs` loads those files, enriches them, and writes the final tables
- `methods/` reads the written tables as black-box inputs

Normal table builds do not repair missing capacity payloads. Fixed-F ascent
summary rows must have matching producer-cache rows (`ascent-cache.jsonl` and
`ascent-product-cache.jsonl`) with capacity, volume, sigmas, and orbit scalars
before this stage runs.

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

The current rules for method-only runs, reusable table features, producer
changes, reviewers, and speculative datasets live in `../README.md`.

Code ownership:
- `main.rs` orchestrates `load -> enrich -> write`
- `load_caches.rs` reads producer files, merges them into unified rows, and
  validates required producer payload
- `features.rs` computes the polytope-level columns from already-loaded
  capacity and geometry payload; it does not run capacity search
- `features_trace.rs` computes the observation / trace columns
- `write_database.rs` writes the final JSONL tables

Smoke path:
- [smoke-pipeline.sh](../smoke-pipeline.sh)

This path writes only to temporary directories, but it can still take minutes;
use it as integration smoke, not as a cheap command check.
