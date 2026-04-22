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
