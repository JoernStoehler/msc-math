# Sys-Landscape Methods

This directory is reserved for consumer-side method scripts that read one or more
dataset tables under `experiments/sys-landscape/`.

Target rule:
- `raw/` owns geometry/witness corpus and generator traces;
- `datasets/` owns the shared dataset tables written from the raw corpus;
- method scripts do not create competing canonical geometry datasets;
- a method may consume several dataset tables at once and perform its own
  filtering, centering, or design-matrix construction locally.

Current status:
- `eda.py` is the first flat consumer-side script; it reads one dataset
  directory and writes smoke plots / summary JSON to an ad hoc output
  directory;
- legacy consumer scripts such as `feature-pattern-search/analyze.py` still live
  in their historical folders;
- future refactors should move consumer-only analysis here after the dataset
  producer layout is stable.

## Smoke Path

[smoke-pipeline.sh](../smoke-pipeline.sh) finishes by running `uv run
methods/eda.py` on the temp dataset output directory. This is the current check
that the flat Rust producer surface is already pleasant for low-friction Python
consumer work. The dataset stage takes `--raw-dir <tmp/raw>` as a smoke
convenience alias, so the shell does not have to restate every raw corpus file
path.
