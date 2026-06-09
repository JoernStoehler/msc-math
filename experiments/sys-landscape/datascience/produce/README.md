# Sys-Landscape Produce

This directory owns producer programs, producer caches, and producer outputs for
`experiments/sys-landscape/datascience/`.

Producer outputs split expensive polytope computations from run metadata:

- `*-expensive-computations-cache.jsonl`: exact polytope key plus expensive
  capacity/orbit-search payload, and volume/sys when already computed. This is
  cache-shaped data. Shard workers may read previous cache files as immutable
  inputs and write self-contained shard-local cache outputs.
- `*-ascent-events.jsonl`: run metadata saying where an ascent run used a
  polytope: seed, phase, iteration, role, accepted/final flags, and
  `polytope_key`. This is not cache data.

Do not cache ascent orchestration such as rejected/accepted decisions,
gradients, or seed state. Rerun cheap control flow; reuse expensive
capacity/orbit results.

Producer outputs should not decide the final method-facing table shape.
Reusable datascience feature columns, deliberate deduplication, retained table
row entities, and method-specific rectangular matrices belong downstream.

Canonical file naming follows:
- `name.jsonl`
- `name-trace.jsonl`
- `name-cache.jsonl`
- `name-computed-polytopes.jsonl`
- `name-ascent-events.jsonl`
- `name-expensive-computations-cache.jsonl`
- transient smoke outputs `smoke-name.jsonl`

Current committed producer artifacts:
- `random.jsonl`
- `random-product.jsonl`
- `ascent-general-endpoints.jsonl`
- `ascent-general-trace.jsonl`
- `ascent-general-cache.jsonl`
- `ascent-general-ascent-events.jsonl`
- `ascent-general-expensive-computations-cache.jsonl`
- `ascent-product-endpoints.jsonl`
- `ascent-product-trace.jsonl`
- `ascent-product-cache.jsonl`
- `ascent-product-ascent-events.jsonl`
- `ascent-product-expensive-computations-cache.jsonl`
- `expensive-computations-cache.jsonl` after review/promotion of merged
  cache rows
- `ascent-events.jsonl` after review/promotion of merged ascent events
- `continuation.jsonl`
- `shared-cache.jsonl`
- `continuation-cache.jsonl`

Current fixed-F ascent endpoint/cache compatibility counts from the 2026-06-04
LICCA wave:

- `ascent-general-endpoints.jsonl`: `4096` rows; `ascent-general-cache.jsonl`: `4096` rows.
- `ascent-product-endpoints.jsonl`: `4089` rows; `ascent-product-cache.jsonl`: `4089`
  rows. The product wave attempted seeds `0..4095`; seven seeds did not emit
  summary/cache rows.
- Both fixed-F cache files have complete `capacity`, `volume`, `sigmas`, and
  `orbit_scalars` payloads for every committed summary endpoint.

LICCA fixed-F ascent shard outputs for this branch live under:

```text
licca-shards/general-computed-production-1024/
licca-shards/product-computed-production-1024/
```

These shard files are producer-stage artifacts. Review and merge them into the
canonical producer files before rebuilding `../tables/`.

Use [merge-licca-ascent-shards.py](merge-licca-ascent-shards.py) to consolidate
LICCA shard directories into branch-local merged producer files for review. It
reports row counts, producer-cache coverage, event/cache coverage, missing
expected seed indices, max `final_sys`, and any `final_sys > 1` rows. Pass
`--require-cache --fresh-fixed-f` for the fixed-F replacement wave.

## Producer binaries

- `sys-dataset-random` writes `random.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-random-product` writes `random-product.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-ascent` writes `ascent-general-endpoints.jsonl`, `ascent-general-trace.jsonl`,
  `ascent-general-cache.jsonl`, `ascent-general-computed-polytopes.jsonl`,
  `ascent-general-ascent-events.jsonl`, and
  `ascent-general-expensive-computations-cache.jsonl`.
- `sys-dataset-ascent-product` writes `ascent-product-endpoints.jsonl`,
  `ascent-product-trace.jsonl`, `ascent-product-cache.jsonl`, and
  `ascent-product-computed-polytopes.jsonl`,
  `ascent-product-ascent-events.jsonl`, and
  `ascent-product-expensive-computations-cache.jsonl`.
- `sys-dataset-continuation` writes `continuation.jsonl` and `continuation-cache.jsonl`.

Older `sys-landscape` experiment directories still exist outside `datascience/`, but
this directory is the maintained producer surface for the datascience pipeline.

## Dataset And Smoke Paths

For method waves, do not ask every method worker to regenerate or privately
copy producer data. Build the shared table dataset under
`experiments/sys-landscape/datascience/tables/` and let method workers
build method-local rectangular inputs when needed. The current role rules live
in `../README.md`.

## Smoke Path

Use [smoke-pipeline.sh](../smoke-pipeline.sh) to exercise the full low-friction
surface on temporary outputs:

- producer binaries write under a temp `produce/` directory;
- `sys-dataset --produce-dir <tmp/produce>` consumes the current canonical smoke stems
  from that directory and writes the shared dataset outputs;
- `sys-dataset-ascent` and `sys-dataset-ascent-product` run with a small
  `--seed-time-budget-secs` override;
- no tracked `.jsonl` files are touched.

Runtime caveat: this is integration smoke, not a cheap command check. The
default runs both ascent producers twice: cold first, then hot against the
same shard-local expensive-computation cache. Set `RUN_CONTINUATION_SMOKE=1`
only when the older slow continuation integration path is specifically needed.

For a focused cold/hot cache benchmark, run:

```bash
SEED_TIME_BUDGET_SECS=1 \
  experiments/sys-landscape/datascience/pipeline.local.sh cache-benchmark
```

The benchmark runs cold local general/product shards, merges their cache/events,
reruns hot against the merged cache, then reports row counts. Check the producer
logs for `Expensive-computation cache: hits=..., misses=...`.

## LICCA Fixed-F Ascent Shards

Use the `licca-ascent-*.slurm.sh` Slurm scripts on the
`licca-datascience-datasets` branch to scale fixed-F ascent without shared
output races. Submit these scripts directly; do not pass production settings as
`sbatch` flags. Each Slurm script is self-contained and includes its resources,
seed range, output path, resume rule, and exact Rust command.

Rule: keep smoke scripts shaped like production scripts. Only make them smaller
and cheaper. Run smoke before production.

The script writes one endpoint summary JSONL, and derived `*-trace.jsonl`,
`*-cache.jsonl`, `*-computed-polytopes.jsonl`, `*-ascent-events.jsonl`, and
`*-expensive-computations-cache.jsonl` per Slurm array task. Endpoint summaries
are transitional compatibility output for current tables. The cache/event split
is the durable producer surface for the ascent polytopes.

Shard scripts pass `--fresh`: retry rewrites shard outputs instead of appending
timing-dependent duplicate summaries. Before rewriting, the producer loads any
existing shard-local expensive-computation cache plus the canonical
`expensive-computations-cache.jsonl` if present. The rerun recomputes cheap
control flow and reuses cached capacity/orbit-search payloads.

- [licca-ascent-general-smoke.slurm.sh](licca-ascent-general-smoke.slurm.sh): one
  `test`-partition general shard with `2` seeds in `licca-shards/general-smoke/`.
- [licca-ascent-product-smoke.slurm.sh](licca-ascent-product-smoke.slurm.sh): one
  `test`-partition product shard with `2` seeds in `licca-shards/product-smoke/`.
- [licca-ascent-general-smoke.local.sh](licca-ascent-general-smoke.local.sh):
  local general smoke companion with one seed by default, one-second local
  budget, and a LICCA-shaped temp output directory.
- [licca-ascent-product-smoke.local.sh](licca-ascent-product-smoke.local.sh):
  local product smoke companion with one seed by default, one-second local
  budget, and a LICCA-shaped temp output directory.
- [licca-ascent-general-production.slurm.sh](licca-ascent-general-production.slurm.sh):
  production general wave with array `0-3`, `1024` seeds per shard, and
  `128` CPUs per shard. It starts at seed `0` and writes the fresh fixed-F
  output directory.
- [licca-ascent-product-production.slurm.sh](licca-ascent-product-production.slurm.sh):
  production product wave with array `0-3`, `1024` seeds per shard, and
  `128` CPUs per shard. It starts at seed `0` and writes the fresh fixed-F
  output directory.

Retry rule: rerun the same wrapper with the same array index and constants.
Do not edit canonical producer files from worker jobs. Worker jobs only read
canonical expensive-computation cache files and write shard-local outputs.

Build the binaries on LICCA first:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape \
  --bin sys-dataset-ascent \
  --bin sys-dataset-ascent-product
```

Before production, run the production-shaped ascent smoke dependency chain:

```bash
cd "$HOME/msc-math/experiments/sys-landscape/datascience/produce"
./submit-licca-ascent-smoke-pipeline.sh
```

This submits the existing smoke general/product ascent array jobs, then submits
the same merge Slurm job with an `afterok` dependency on both smoke arrays. It
checks Slurm array submission, dependency wiring, shard output paths, and merge
behavior before production submission.

After the smoke dependency chain passes, submit the production ascent pipeline
with one wrapper:

```bash
cd "$HOME/msc-math/experiments/sys-landscape/datascience/produce"
./submit-licca-ascent-pipeline.sh
```

This submits the general and product production arrays, then submits
[licca-merge-ascent-shards.slurm.sh](licca-merge-ascent-shards.slurm.sh) with an
`afterok` dependency on both arrays. The merge job writes review targets. It
does not promote those files to canonical producer filenames, so table build
submission remains a separate step after review.

This requests `4096` computed-polytope general seeds and `4096`
computed-polytope product seeds and the corresponding cache/event streams. It writes to
`licca-shards/general-computed-production-1024/` and
`licca-shards/product-computed-production-1024/` so it does not collide with
older waves under `licca-shards/general/`, `licca-shards/product/`,
`licca-shards/general-production-1024/`,
`licca-shards/product-production-1024/`,
`licca-shards/general-cache-production-1024/`, or
`licca-shards/product-cache-production-1024/`.

After shard review, consolidate on LICCA or locally with:

```bash
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py --write
```

For the fixed-F replacement wave, use `--require-cache --fresh-fixed-f` during
review and write. This omits older fixed-F shard waves from the merge:

```bash
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
  --require-cache \
  --fresh-fixed-f
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
  --require-cache \
  --fresh-fixed-f \
  --write
```

The `--write` form creates:

```text
ascent-general-licca-merged-endpoints.jsonl
ascent-general-licca-merged-trace.jsonl
ascent-general-licca-merged-cache.jsonl
ascent-general-licca-merged-computed-polytopes.jsonl
ascent-product-licca-merged-endpoints.jsonl
ascent-product-licca-merged-trace.jsonl
ascent-product-licca-merged-cache.jsonl
ascent-product-licca-merged-computed-polytopes.jsonl
expensive-computations-cache-licca-merged.jsonl
ascent-events-licca-merged.jsonl
```

These files are review targets. Promote them to the canonical producer filenames
only after row guards and logs are accepted.
