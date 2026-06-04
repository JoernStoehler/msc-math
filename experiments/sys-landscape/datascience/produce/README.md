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
- `ascent-product.jsonl`
- `ascent-product-trace.jsonl`
- `continuation.jsonl`
- `shared-cache.jsonl`
- `continuation-cache.jsonl`

LICCA shard outputs for this branch should live under:

```text
licca-shards/general/
licca-shards/product/
```

These shard files are producer-stage artifacts. Review and merge them into the
canonical producer files before rebuilding `../dataset/`.

## Producer binaries

- `sys-dataset-random` writes `random.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-random-product` writes `random-product.jsonl` and updates `shared-cache.jsonl`.
- `sys-dataset-ascent` writes `ascent.jsonl` and `ascent-trace.jsonl`.
- `sys-dataset-ascent-product` writes `ascent-product.jsonl` and `ascent-product-trace.jsonl`.
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
2026-05-31, the temp-output path was safe, but the script was stopped after
about two minutes while `sys-dataset-continuation --smoke` was still running.
Use `--help`, compile checks, or narrower producer smoke commands for fast
validation.

## LICCA Fixed-F Ascent Shards

Use the `licca-ascent-*.slurm.sh` Slurm scripts on the
`licca-datascience-datasets` branch to scale fixed-F ascent without shared
output races. Submit these scripts directly; do not pass production settings as
`sbatch` flags. Each Slurm script is self-contained and includes its resources,
seed range, output path, resume rule, and exact Rust command.

The script writes one summary JSONL and one derived `*-trace.jsonl` per Slurm
array task. Defaults skip existing committed seed ranges: general starts at
`n-start=10`, and product starts at `n-start=12`.

- [licca-ascent-smoke-general.slurm.sh](licca-ascent-smoke-general.slurm.sh): one
  `test`-partition general shard with `2` seeds.
- [licca-ascent-smoke-product.slurm.sh](licca-ascent-smoke-product.slurm.sh): one
  `test`-partition product shard with `2` seeds.
- [licca-ascent-production-general.slurm.sh](licca-ascent-production-general.slurm.sh):
  production general wave with array `10-13`, `1024` seeds per shard, and
  `128` CPUs per shard.
- [licca-ascent-production-product.slurm.sh](licca-ascent-production-product.slurm.sh):
  production product wave with array `10-13`, `1024` seeds per shard, and
  `128` CPUs per shard.

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

This requests `4096` new general seeds and `4096` new product seeds, starting
after the already submitted shard ids `0-9`.
