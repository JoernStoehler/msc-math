# Sys-Landscape Methods

This directory is reserved for consumer-side method scripts that read one or more
produced dataset surfaces under `experiments/sys-landscape/`.

Target rule:
- dataset producers own raw/derived JSONLs;
- method scripts do not create competing canonical datasets;
- a method may consume several producer families at once through the normalized
  join layer or explicit dataset paths.

Current status:
- legacy consumer scripts such as `feature-pattern-search/analyze.py` still live
  in their historical folders;
- future refactors should move consumer-only analysis here after the dataset
  producer layout is stable.
