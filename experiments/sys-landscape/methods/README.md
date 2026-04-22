# Sys-Landscape Methods

This directory is reserved for consumer-side method scripts that read one or more
dataset tables under `experiments/sys-landscape/`.

Target rule:
- `raw/` owns geometry/witness corpus and generator traces;
- `datasets/` owns join tables and durable feature blocks;
- method scripts do not create competing canonical geometry datasets;
- a method may consume several dataset tables at once and perform its own
  filtering, centering, or design-matrix construction locally.

Current status:
- `eda.py` is the first flat consumer-side script; it reads normalized tables
  and optional feature blocks from ad hoc directories and writes smoke plots /
  summary JSON to an ad hoc output directory;
- legacy consumer scripts such as `feature-pattern-search/analyze.py` still live
  in their historical folders;
- future refactors should move consumer-only analysis here after the dataset
  producer layout is stable.

## Smoke Path

[smoke-pipeline.sh](../smoke-pipeline.sh) finishes by running `uv run
methods/eda.py` on the temp normalized / feature outputs. This is the current
check that the flat Rust producer surface is already pleasant for low-friction
Python consumer work.
