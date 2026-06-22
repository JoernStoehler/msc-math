# Sys-Landscape Datascience Prepare

This directory owns the shared prepare stage for
`experiments/sys-datascience/`: load producer outputs, canonize
representatives, compute reusable features, and write retained prepared tables.

Pipeline:
- `produce/` writes producer JSONL files that preserve expensive computed
  polytope facts and producer context
- `prepare/main.rs` loads the producer files needed for the current row
  entities, enriches them, and writes retained prepared tables next to this code
- `methods/` reads the retained prepared tables as black-box inputs and may build
  method-local rectangular inputs

The stage split is operational as well as conceptual. Run `produce/` when the
expensive polytope/capacity payloads need to change. Rerun `prepare/` when
canonization, feature computation, deduplication, or retained table shape
changes. Prepare should be local by default because canonization and feature
computation are cheap relative to capacity search; use LICCA only when the
prepared table size or feature cost actually makes local runs impractical.
For random/product development and evidence runs, scope the dataset before
feature construction. Use:

```bash
experiments/sys-datascience/prepare/build-random-only-slice.sh smoke
experiments/sys-datascience/prepare/build-random-only-slice.sh method
```

`smoke` builds `8` generic random rows plus `20` product rows for fast
prepare-stage feedback. `method` builds `512` generic random rows plus `1024`
product rows for method-code feedback. The full random/product evidence run is
the same scoped builder without row limits; prefer LICCA for that final gate.
The earlier all-source retained-table rebuild path was too broad for this goal:
it included ascent/continuation rows before feature construction.

The size presets call `sys-dataset --random-only-size <smoke|method|full>`.
Limited presets take deterministic stratified prefixes: generic rows are spread
over `facet_count`, and product rows are spread over `(k, m, bounces)`. Prefixes
within those homogeneous producer blocks are still ordinary random samples from
that block; the stratification avoids accidentally testing only the first file
blocks such as `facet_count = 5`.

Current scoped local evidence after the random/product scoping fix:

- `smoke` with hydrated producer files from the main checkout built `28`
  random/product rows, `0` computed-observation rows, and `0` ascent-run rows;
  generic random rows covered facet counts `5..12` once each, product rows
  covered each of the `20` `(k, m, bounces)` blocks once, table construction
  took `1.7s`, and the fingerprint found max `sys = 0.7542347878889757`.
- `method` with the same producer files built `1536` random/product rows,
  `0` computed-observation rows, and `0` ascent-run rows; generic random rows
  covered facet counts `5..12` with `64` rows each, product rows covered each
  of the `20` `(k, m, bounces)` blocks with `51` or `52` rows each, table
  construction took `40.9s`, and the fingerprint found max
  `sys = 0.8247662746241669`.
- The method-feedback slice was sufficient to run `random-tail-eda`,
  `statistical-associations`, `projection-structure`, and
  `prediction-ranking` in seconds with reduced permutation/tree counts.
- A full local prepare rerun was not part of the wrapup cleanup after local CPU
  warnings. Full current-schema evidence should be regenerated deliberately,
  then fingerprinted and reviewed before thesis-facing method closure wording.

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
Submit the Slurm script from this `prepare/` directory; it uses
`SLURM_SUBMIT_DIR` for run-local output paths because Slurm may execute a spool
copy of the script.
Use `--partition=test` for prepare smoke submissions; the script default is the
production-shaped `epyc` partition. Smoke submissions should also override the
CPU count, memory, and timeout, for example `--cpus-per-task=4 --mem=8G
--time=00:20:00`.

Normal prepare runs require the expensive payload for every producer metadata
row. They do not run capacity search. Local smoke evidence for the current
prepare path:
`18` produced polytopes became `18` polytope-table rows and `18` provenance
rows; `fingerprint-dataset.py` reported max `sys=0.7163711008250128` and
`0` rows with `sys > 1`. Prepare also writes `prepare-stats.json` next to the
prepared tables so a smoke or LICCA run records row counts, max `sys`, and wall time
without relying only on Slurm stdout.

LICCA prepare evidence for commit `8b685cf0`: job `9826142` consumed the
production run-local producer output from job `9826141`, completed in
`00:01:07`, and wrote `14336` polytope rows plus `14336` provenance rows.
`fingerprint-dataset.py` reported max `sys=0.9015863923873765`, `0` rows with
`sys > 1`, random counts `4096`/`10240`, and sha256:

- `polytope-table.jsonl`:
  `9e828f85cc6c40632c47749782bfb4aef3569e0f6453f5d580a332a4bf92cb23`
- `polytope-provenance-table.jsonl`:
  `e6b6f0e046de0c5bb745cc51dfc3ab069c4055de52ff0144ec9fd71c029f1390`
- empty `polytope-ascent-run-table.jsonl` and
  `computed-polytope-observation-table.jsonl`:
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

Normal prepare builds do not repair missing capacity payloads. Fixed-F ascent
summary rows must have matching producer-cache rows (`ascent-general-cache.jsonl` and
`ascent-product-cache.jsonl`) with capacity, volume, sigmas, and orbit scalars
before this stage runs.

Producer `*-computed-polytopes.jsonl` files preserve additional computed-polytope
facts from ascent, including intermediate ascent-run polytopes. The prepare stage
does not eagerly materialize intermediate ascent-run steps into
`polytope-table.jsonl`; current method-facing geometry rows include ascent
starts/finals plus the other retained non-intermediate sources. The prepare stage
records computed ascent occurrence context in
`computed-polytope-observation-table.jsonl`.

BUG: ascent run endpoints are not local maximizers of `sys(a)` right now. See
`../methods/endpoint-local-max-diagnostic/README.md`: sampled retained ascent
endpoints still have tiny improving quotient directions.

Ascent producers also emit `*-ascent-events.jsonl` and
`*-expensive-computations-cache.jsonl`. Those are the durable producer-side
split between run metadata and reusable expensive capacity/orbit-search
payloads. Current prepare code still reads endpoint summary/cache compatibility
files; switching prepared-table construction to derive endpoint rows from ascent
events should be done as a prepare-loader change, not by changing producer
shard semantics.

Current prepared outputs:
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
  datascience prepared tables, including source, role, optimizer, seed, path, and
  lineage.
- `polytope-ascent-run-table.jsonl`: one row per ascent or continuation
  provenance record keyed by `provenance_id`; records run-level and
  trajectory-summary fields. Random-sample provenance rows do not appear here.

For method waves, the prepared output should live under:

```text
experiments/sys-datascience/prepare/
```

Build the current retained prepared output with:

```bash
experiments/sys-datascience/build-dataset.sh
```

Visualize the retained dataset composition for method planning with:

```bash
uv run --script experiments/sys-datascience/prepare/plot_dataset_composition.py
```

This script reads `polytope-table.jsonl` and
`polytope-provenance-table.jsonl`, prints counts by dataset source, and writes
`dataset-composition.png` by default. The plot is table-scoped: it describes
the polytopes currently available to methods.

The current rules for method-only runs, reusable table columns, producer
changes, reviewers, and speculative datasets live in `../README.md`.

Code ownership:
- `main.rs` orchestrates `load -> canonize/enrich -> write`
- `load_caches.rs` reads producer files, merges them into unified rows, and
  validates required producer payload
- `features.rs` computes the polytope-level prepared columns from already-loaded
  capacity and geometry payload; it does not run capacity search
- `features_trace.rs` computes provenance and ascent-run table columns
- `write_database.rs` writes the retained JSONL prepared tables

Accepted reusable retained columns belong here.

Columns in this stage are reusable prepared data with stable meaning outside
one method packet. Canonization choices and feature columns that multiple
black-box methods should reuse belong here. Model-specific transformed
features, train/test split labels, temporary report audits, and rectangular
convenience columns belong in `../methods/` until a concrete reuse or
compute-cost case justifies promotion.

The old generic observation table name caused confusion. If future data no
longer fits polytope, provenance, or ascent-run rows, add a new table for the
concrete row entity and name that entity directly.

Smoke path:
- [smoke-pipeline.sh](../smoke-pipeline.sh)

This path writes only to temporary directories, but it can still take minutes;
use it as integration smoke, not as a cheap command check.
