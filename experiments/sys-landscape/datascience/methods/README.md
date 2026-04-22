# Sys-Landscape Datascience Methods

This directory owns consumer-side datascience scripts for `experiments/sys-landscape/datascience/`.

Current rule:
- methods read tables written by `tables/main.rs`
- methods do not rebuild producer caches
- method-local filtering, scaling, and model matrices stay local to the method

Current consumers:
- `eda.py`
- `feature-pattern-search/`

The current dataset inputs are:
- `polytope-table.jsonl`
- `observation-table.jsonl`
