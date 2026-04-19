# HKO Local Maximum Research Mapping

Research anchors and current local home:
- `research/hko-local-maximum/design/exact-clarke-subgradient.md` → `exact-clarke/`
- `research/hko-local-maximum/design/exact-clarke-closure-plan.md` → `exact-clarke/`
- `research/hko-local-maximum/design/exact-clarke-orbit-catalog.md` → `exact-clarke/`
- `research/hko-local-maximum/design/gradient-analysis.md` → `gradient-analysis/`
- `research/hko-local-maximum/design/facet-splitting.md` → `facet-splitting/`
- `research/hko-local-maximum/design/second-order.md` → `second-order/`
- `research/hko-local-maximum/design/cut-and-ascent.md` → `cut-and-ascent/`
- `research/hko-local-maximum/design/lagrangian-boundary.md` → `lagrangian-boundary/`
- `research/hko-local-maximum/design/perturbation-neighborhood.md` → `perturbation-neighborhood/`
- `research/hko-local-maximum/design/subdifferential-lp.md` → `subdifferential-lp/`
- `research/hko-local-maximum/design/sage-validation.md` → `sage-validation/`

Current orientation (high-signal):
- `exact-clarke/` is `open`, not closed: exact `M_10` certificate still blocked by unresolved representative-row multiplicity.
- `exact-clarke/hko-geometry.json` and `hko-symmetry-tangent.json` are committed and define the quartic setup (`Q(tan(pi/5))`).
- `exact-clarke/billiard-sigma-counts.json` records the current orbit-surface split (`50,400` raw / `6,240` directed-feasible / `717` valid KKT orbits / `150` exact minima).
- `exact-clarke/widened-seed-witness.json` is validated by `verify_widened_seed_witness.sage` but still does not prove the final active-cone certificate.
- `gradient-analysis/hko-neighborhood-sensitivity.jsonl` plus `hko-neighborhood-ascent.jsonl` carry the local derivative baseline.
- `facet-splitting/hko-neighborhood-splitting.jsonl` is complete and shows all 536 tested splits had lower sys than HKO.
- `second-order/second_order_base.jsonl` reports `rank(G)=25`, `dim ker(G)=15`, negative curvature on all 15 flat directions and random check directions.
- `lagrangian-boundary/lagrangian-search.jsonl` supports a shrinking sys>1 region in regular Lagrangian perturbations (`eps*≈0.035`) and no random-product-scale hits.
- `perturbation-neighborhood/pentagon-perturb.jsonl` remains the historical 101-row committed neighborhood packet; LICCA run artifacts would land under `data/licca-eps-*.jsonl`.
- `subdifferential-lp/phase_c_lp_test.py` is currently broken against the migrated `dual_vertices` schema and should be refreshed before reuse.
- `sage-validation/analyze.py` currently validates the selected exact bank against Sage on exact-number outputs.

Open local actions:
- keep exact-clarke witness data in lockstep: `numerical-permutation-orbits.json`, `endpoint-seed-rows.json`, `midpoint-seed-rows.json`,
  and refresh any next `*_witness*.json` artifacts only from script outputs in `exact-clarke/`.
- use `perturbation-neighborhood/analyze.py` and `exact-clarke/*.jsonl` outputs as the single source for orientation summaries; avoid reinterpreting legacy notes not reflected by current artifacts.
