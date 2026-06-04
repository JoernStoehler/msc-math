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
method workers. See `../README.md`.

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
