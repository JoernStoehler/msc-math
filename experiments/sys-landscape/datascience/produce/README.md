# Sys-Landscape Raw Corpus

This directory is the flat raw-corpus layer for `experiments/sys-landscape/`.

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

## Flat Raw Producers

| Flat binary | Raw file stem | Notes |
| --- | --- | --- |
| `sys-dataset-random` | `random` | generic random baseline corpus |
| `sys-dataset-random-product` | `random-product` | product baseline corpus |
| `sys-dataset-ascent` | `ascent` | fixed-`F` ascent corpus |
| `sys-dataset-ascent-product` | `ascent-product` | fixed-`F` product-ascent corpus |
| `sys-dataset-continuation` | `continuation` | variable-`F` continuation corpus |

The legacy packet directories remain in place until later cleanup removes or archives them.

## Smoke Path

Use [smoke-pipeline.sh](../smoke-pipeline.sh) to exercise the full low-friction
surface on temporary outputs:
- raw producers write under a temp `raw/` directory;
- `sys-dataset --raw-dir <tmp/raw>` consumes the current canonical smoke stems
  from that directory and writes the shared dataset outputs;
- `sys-dataset-ascent` and `sys-dataset-ascent-product` run with a small
  `--seed-time-budget-secs` override;
- no tracked `.jsonl` files are touched.
