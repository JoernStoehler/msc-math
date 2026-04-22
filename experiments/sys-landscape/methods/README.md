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
- legacy consumer scripts such as `feature-pattern-search/analyze.py` still live
  in their historical folders;
- future refactors should move consumer-only analysis here after the dataset
  producer layout is stable.
