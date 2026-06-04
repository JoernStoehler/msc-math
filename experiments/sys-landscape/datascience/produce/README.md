# Sys-Landscape Produce

This directory owns the producer stage for `experiments/sys-landscape/datascience/`.

It owns the cache-worthy geometry and witness payloads:
- exact or reconstructible polytope/state identity;
- provenance and lineage metadata;
- near-minimal `sigma[]` witness sets and cutoffs;
- raw traces or generator logs when those are part of the expensive search surface.

Canonical file naming follows:
- `name.jsonl`
- `name-trace.jsonl`
- `name-cache.jsonl`
- transient smoke outputs `smoke-name.jsonl`

Current committed producer artifacts:
- `random.jsonl`
- `random-product.jsonl`
- `ascent.jsonl`
- `ascent-trace.jsonl`
- `ascent-cache.jsonl`
- `ascent-product.jsonl`
- `ascent-product-trace.jsonl`
- `ascent-product-cache.jsonl`
- `continuation.jsonl`
- `shared-cache.jsonl`
- `continuation-cache.jsonl`

Current fixed-F ascent producer counts from the 2026-06-04 LICCA
cache-complete wave:

- `ascent.jsonl`: `4096` rows; `ascent-cache.jsonl`: `4096` rows.
- `ascent-product.jsonl`: `4089` rows; `ascent-product-cache.jsonl`: `4089`
  rows. The product wave attempted seeds `0..4095`; seven seeds did not emit
  summary/cache rows.
- Both fixed-F cache files have complete `capacity`, `volume`, `sigmas`, and
  `orbit_scalars` payloads for every committed summary endpoint.

LICCA shard outputs for this branch should live under:

```text
licca-shards/general/
licca-shards/product/
```

These shard files are producer-stage artifacts. Review and merge them into the
canonical producer files before rebuilding `../dataset/`.

Use [merge-licca-ascent-shards.py](merge-licca-ascent-shards.py) to consolidate
canonical ascent files and LICCA shard directories into branch-local merged
producer files for review. It reports row counts, cache coverage, missing
expected seed indices, max `final_sys`, and any `final_sys > 1` rows. Pass
`--require-cache` for future LICCA campaigns where every summary endpoint must
have a matching shard-local producer-cache row.

## Producer binaries

- `sys-dataset-random` writes `random.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-random-product` writes `random-product.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-ascent` writes `ascent.jsonl`, `ascent-trace.jsonl`, and
  `ascent-cache.jsonl`.
- `sys-dataset-ascent-product` writes `ascent-product.jsonl`,
  `ascent-product-trace.jsonl`, and `ascent-product-cache.jsonl`.
- `sys-dataset-continuation` writes `continuation.jsonl` and `continuation-cache.jsonl`.

Older `sys-landscape` experiment directories still exist outside `datascience/`, but
this directory is the maintained producer surface for the datascience pipeline.

## Dataset And Smoke Paths

For method waves, do not ask every method worker to regenerate or privately
copy producer data. Build the shared table dataset under
`experiments/sys-landscape/datascience/dataset/` and pass that dataset path to
method workers. The current role rules live in `../README.md`.

## Smoke Path

Use [smoke-pipeline.sh](../smoke-pipeline.sh) to exercise the full low-friction
surface on temporary outputs:

- producer binaries write under a temp `produce/` directory;
- `sys-dataset --produce-dir <tmp/produce>` consumes the current canonical smoke stems
  from that directory and writes the shared dataset outputs;
- `sys-dataset-ascent` and `sys-dataset-ascent-product` run with a small
  `--seed-time-budget-secs` override;
- no tracked `.jsonl` files are touched.

Runtime caveat: this is integration smoke, not a cheap command check. On
2026-06-04, the default temp-output path took about 49 seconds locally with
`ASCENT_BUDGET_SECS=1`. The default runs both ascent producers twice and checks
that the second pass resumes without duplicate summary/cache rows. Set
`RUN_CONTINUATION_SMOKE=1` only when the older slow continuation integration
path is specifically needed.

## LICCA Fixed-F Ascent Shards

Use the `licca-ascent-*.slurm.sh` Slurm scripts on the
`licca-datascience-datasets` branch to scale fixed-F ascent without shared
output races. Submit these scripts directly; do not pass production settings as
`sbatch` flags. Each Slurm script is self-contained and includes its resources,
seed range, output path, resume rule, and exact Rust command.

The script writes one summary JSONL, one derived `*-trace.jsonl`, and one
derived `*-cache.jsonl` per Slurm array task. The cache file is shard-local and
is still written when the binary runs with `--no-db-update`; that flag only
prevents shared cache writes. Defaults skip existing committed seed ranges:
general starts at `n-start=10`, and product starts at `n-start=12`.

- [licca-ascent-smoke-general.slurm.sh](licca-ascent-smoke-general.slurm.sh): one
  `test`-partition general shard with `2` seeds.
- [licca-ascent-smoke-product.slurm.sh](licca-ascent-smoke-product.slurm.sh): one
  `test`-partition product shard with `2` seeds.
- [licca-ascent-production-general.slurm.sh](licca-ascent-production-general.slurm.sh):
  production general wave with array `0-3`, `1024` seeds per shard, and
  `128` CPUs per shard. It starts at seed `0` and writes a fresh
  cache-complete output directory.
- [licca-ascent-production-product.slurm.sh](licca-ascent-production-product.slurm.sh):
  production product wave with array `0-3`, `1024` seeds per shard, and
  `128` CPUs per shard. It starts at seed `0` and writes a fresh
  cache-complete output directory.

Resume rule: do not delete partial shard files after timeout. Rerun the same
wrapper with the same array index and constants. The Rust binary reads
completed summary rows and skips them, then canonicalizes output on a normal
exit.

Build the binaries on LICCA first:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape \
  --bin sys-dataset-ascent \
  --bin sys-dataset-ascent-product
```

Smoke-submit one small shard per kind:

```bash
cd "$HOME/msc-math/experiments/sys-landscape/datascience/produce"
sbatch licca-ascent-smoke-general.slurm.sh
sbatch licca-ascent-smoke-product.slurm.sh
```

After reviewing logs, submit the production wrappers directly:

```bash
cd "$HOME/msc-math/experiments/sys-landscape/datascience/produce"
sbatch licca-ascent-production-general.slurm.sh
sbatch licca-ascent-production-product.slurm.sh
```

This requests `4096` cache-complete general seeds and `4096` cache-complete
product seeds. It writes to `licca-shards/general-cache-production-1024/` and
`licca-shards/product-cache-production-1024/` so it does not collide with older
no-cache waves under `licca-shards/general/`,
`licca-shards/product/`, `licca-shards/general-production-1024/`, or
`licca-shards/product-production-1024/`.

After shard review, consolidate on LICCA or locally with:

```bash
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py --write
```

For future cache-complete LICCA campaigns, use `--require-cache` during review
and write. Use `--fresh-fixed-f` when replacing old fixed-F ascent data instead
of merging old no-cache waves:

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
ascent-licca-merged.jsonl
ascent-licca-merged-trace.jsonl
ascent-licca-merged-cache.jsonl
ascent-product-licca-merged.jsonl
ascent-product-licca-merged-trace.jsonl
ascent-product-licca-merged-cache.jsonl
```

These files are review targets. Promote them to the canonical producer filenames
only after row guards and logs are accepted.
