# Neighborhood Evidence Plan

Scope:
- consolidate local max-like evidence around HKO2024 and keep false-positive risk explicit.

Current evidence baseline:
- `perturbation-neighborhood/pentagon-perturb.jsonl` is committed historical local evidence (`n=101`) and stays separate from current production runs.
- `lagrangian-boundary/lagrangian-search.jsonl` shows a rapid decay of `sys>1` mass in 20D Lagrangian perturbations (`sys>1` nearly zero by `eps≈0.15`).
- `facet-splitting/hko-neighborhood-splitting.jsonl`: 536 splits, all below baseline HKO sys.
- `cut-and-ascent/cut-and-ascent.jsonl` is scaffolded continuation and currently short.

Active plan:
1. Refresh perturbation falsification packet
   - run `job-smoke.sh` for local smoke checks in `perturbation-neighborhood/`.
   - when available, stage LICCA outputs in `perturbation-neighborhood/data/licca-eps-*.jsonl`.
   - keep `pentagon-perturb.jsonl` as historical context only and clearly split from `analyze.py` reads.

2. Keep fixed-combinatorics falsification split clean
   - `facet-splitting/` remains the fixed-combinatorics probe.
   - `cut-and-ascent/` remains the ascent-after-cut continuation probe.

3. Consolidate neighborhood orientation
   - `lagrangian-boundary/` provides the structured coordinate view for LP-type robustness.
   - `second-order/` provides curvature evidence in the fixed `F=10`, `a_i`-space neighborhood.
   - both feeds into exact-clarke planning through `research/hko-local-maximum/design/exact-clarke-subgradient.md`.

4. Gate future continuation work
   - new continuation trials should only be added after `cut-and-ascent` and `second-order` are updated, or when exact-clarke enters `Packet 3` completion.

Stop conditions:
- no new permanent files except committed packets in `perturbation-neighborhood/`, `facet-splitting/`, and `cut-and-ascent/`.
- if a run materially changes findings, update this plan and the matching task item before touching thesis claims.
