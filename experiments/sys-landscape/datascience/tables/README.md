# Sys-Landscape Datascience Tables

This directory owns the retained table stage for
`experiments/sys-landscape/datascience/`.

Pipeline:
- `produce/` writes producer JSONL files that preserve expensive computed
  polytope facts and producer context
- `tables/main.rs` loads the producer files needed for the current table row
  entities, enriches them, and writes the retained tables next to the table code
- `methods/` reads the retained tables as black-box inputs and may build
  method-local rectangular inputs

`sys-datascience-prepare` is the prepare-stage command for the new run-local
producer path. It consumes a producer output directory containing
`computed-polytopes.jsonl` plus producer metadata files and writes the same
retained table filenames as the current table builder:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-prepare -- \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --out-dir /tmp/ds-prepare-smoke
```

On LICCA, [licca-datascience-prepare.slurm.sh](licca-datascience-prepare.slurm.sh)
runs only the prebuilt `sys-datascience-prepare` binary. Build on the login node
before submitting so Slurm time measures prepare, not Rust compilation:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-datascience-prepare
```

Validate the produce directory before submitting prepare, then fingerprint the
prepare output after the job finishes. Those are explicit login-node gates, not
hidden Slurm job steps.
Submit the Slurm script from this `tables/` directory; it uses
`SLURM_SUBMIT_DIR` for run-local output paths because Slurm may execute a spool
copy of the script.
Use `--partition=test` for prepare smoke submissions; the script default is the
production-shaped `epyc` partition. Smoke submissions should also override the
CPU count, memory, and timeout, for example `--cpus-per-task=4 --mem=8G
--time=00:20:00`.

Normal prepare runs require the expensive payload for every producer metadata
row. They do not run capacity search. Local smoke evidence on this branch:
`18` produced polytopes became `18` polytope-table rows and `18` provenance
rows; `fingerprint-dataset.py` reported max `sys=0.8015672385893916` and
`0` rows with `sys > 1`. Prepare also writes `prepare-stats.json` next to the
tables so a smoke or LICCA run records row counts, max `sys`, and wall time
without relying only on Slurm stdout.

Normal table builds do not repair missing capacity payloads. Fixed-F ascent
summary rows must have matching producer-cache rows (`ascent-general-cache.jsonl` and
`ascent-product-cache.jsonl`) with capacity, volume, sigmas, and orbit scalars
before this stage runs.

Producer `*-computed-polytopes.jsonl` files preserve additional computed-polytope
facts from ascent, including intermediate ascent-run polytopes. The table stage
does not eagerly materialize intermediate ascent-run steps into
`polytope-table.jsonl`; current method-facing geometry rows include ascent
starts/finals plus the other retained non-intermediate sources. The table stage
records computed ascent occurrence context in
`computed-polytope-observation-table.jsonl`.

Ascent producers also emit `*-ascent-events.jsonl` and
`*-expensive-computations-cache.jsonl`. Those are the durable producer-side
split between run metadata and reusable expensive capacity/orbit-search
payloads. Current tables still read endpoint summary/cache compatibility files;
switching table construction to derive endpoint rows from ascent events should
be done as a table-loader change, not by changing producer shard semantics.

Current outputs:
- `polytope-table.jsonl`: one row per retained exact polytope geometry keyed by `poly_id`;
  contains defining dual vertices, computed polytope-level quantities such as
  `volume`, capacity, and `sys`, derived scalar features, and capacity/orbit
  audit fields.
- `computed-polytope-observation-table.jsonl`: one row per successful producer
  computed-polytope observation from fixed-F ascent; records ascent context.
  Intermediate observations may reference a `poly_id` that is producer-retained
  but not materialized in `polytope-table.jsonl`.
- `polytope-provenance-table.jsonl`: one row per retained provenance record
  keyed by `provenance_id`; records how a retained polytope entered the
  datascience tables, including source, role, optimizer, seed, path, and
  lineage.
- `polytope-ascent-run-table.jsonl`: one row per ascent or continuation
  provenance record keyed by `provenance_id`; records run-level and
  trajectory-summary fields. Random-sample provenance rows do not appear here.

For method waves, the table output should live under:

```text
experiments/sys-landscape/datascience/tables/
```

Build the current retained table output with:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Visualize the retained dataset composition for method planning with:

```bash
uv run --script experiments/sys-landscape/datascience/tables/plot_dataset_composition.py
```

This script reads `polytope-table.jsonl` and
`polytope-provenance-table.jsonl`, prints counts by dataset source, and writes
`dataset-composition.png` by default. The plot is table-scoped: it describes
the polytopes currently available to methods.

The current rules for method-only runs, reusable table columns, producer
changes, reviewers, and speculative datasets live in `../README.md`.

Code ownership:
- `main.rs` orchestrates `load -> enrich -> write`
- `load_caches.rs` reads producer files, merges them into unified rows, and
  validates required producer payload
- `features.rs` computes the polytope-level table columns from already-loaded
  capacity and geometry payload; it does not run capacity search
- `features_trace.rs` computes provenance and ascent-run table columns
- `write_database.rs` writes the retained JSONL tables

Accepted reusable retained columns belong here.

Columns in this stage are reusable retained data with stable meaning outside
one method packet. Model-specific transformed features, train/test split
labels, temporary report audits, and rectangular convenience columns belong in
`../methods/` until a concrete reuse or compute-cost case justifies promotion.

The old generic observation table name caused confusion. If future data no
longer fits polytope, provenance, or ascent-run rows, add a new table for the
concrete row entity and name that entity directly.

Smoke path:
- [smoke-pipeline.sh](../smoke-pipeline.sh)

This path writes only to temporary directories, but it can still take minutes;
use it as integration smoke, not as a cheap command check.
