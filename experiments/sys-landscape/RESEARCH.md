# Sys-Landscape Research Mapping

Research anchors and current local home:
- `research/sys-landscape/design/random-sample.md` → `random-sample/`
- `research/sys-landscape/design/random-product-sample.md` → `random-product-sample/`
- `research/sys-landscape/design/rejection-calibration.md` → `rejection-calibration/`
- `research/sys-landscape/design/rotated-regular-products.md` → `rotated-regular-products/`
- `research/sys-landscape/design/gradient-ascent-general.md` → `gradient-ascent-general/`
- `research/sys-landscape/design/gradient-ascent-products.md` → `gradient-ascent-products/`
- `research/sys-landscape/design/variable-f-ascent.md` → `variable-f-ascent/`
- `research/sys-landscape/design/feature-pattern-search.md` → `feature-pattern-search/` + `normalized-dataset/`
- `research/sys-landscape/design/pentagon-rotation-formula.md` → `pentagon-rotation-formula/`
- `research/sys-landscape/design/witness-search-program.md` → next planned program.

Current status snapshot:
- `random-sample/random-sweep.jsonl`: max `sys = 0.739` on 70 rows; no `sys>1`.
- `random-product-sample/random-product-sweep.jsonl`: max `sys = 0.794` on 100 rows; no `sys>1`.
- `gradient-ascent-general/gradient-ascent-general.jsonl`: best local `sys=0.9030` (10 seeds; no `>1`).
- `gradient-ascent-products/gradient-ascent-products.jsonl`: best local `sys=0.8727` (12 seeds; no `>1`).
- `variable-f-ascent/variable-f-ascent.jsonl`: 90 trials; `F=10→11` improves often (`RQ1` 45/50 improved) but no `>1`.
- `lagrangian` evidence and neighborhood evidence are split into `hko-local-maximum/lagrangian-boundary` and `hko-local-maximum/perturbation-neighborhood` and are intentionally referenced from this package via design notes.
- `feature-pattern-search/analyze.py` supports the closure for "hostile landscape" transfer checks and currently reports negative transfer from random baselines to endpoint regimes.
- `normalized-dataset` contract has current bounded counts `282` states / `282` capacity rows / `287` step events.
- `pentagon-rotation-formula/theta-sweep.jsonl` matches the active 2-bounce branch formula near current sweep coverage and isolates short 3-bounce competitor shortlist.

Near-term open line:
- witness-search is the next active thesis-scope extension.
- Primary successor baselines are `variable-f-ascent/` and `experiments/hko-local-maximum/cut-and-ascent/` per
  `research/sys-landscape/design/witness-search-program.md`.
