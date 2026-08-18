# Dev Sys Prediction Panel

This is the generation surface for deterministic local panels used in
`sys(a0 + t u)` prediction audits. The public data flow is:

```text
basepoints -> states -> perturbation events -> observations/reports
```

The realized sampling law is documented in `src/panel.rs`. In short: basepoints
are selected separately inside each configured facet-count bucket, and
directions are deterministic functions of the basepoint branch geometry plus
fixed pseudo-random controls. Do not interpret current panels as iid samples
from an independent law on `(F, a0, u)`.

The smoke config uses the compact retained panel from
`facet-scale-baseline-error/`. It remains small and tracked in Git.

Run a smoke panel with:

```bash
cargo run -p exp-dev-sys-prediction --release \
  --bin dev-sys-prediction-panel -- \
  --config experiments/dev-sys-prediction/produce/configs/smoke.json \
  --out-dir /tmp/dev-sys-prediction-panel-smoke
```

Run the retained production panel with:

```bash
scripts/artifacts.py materialize polytope-datasets

cargo run -p exp-dev-sys-prediction --release \
  --bin dev-sys-prediction-panel -- \
  --config experiments/dev-sys-prediction/produce/configs/production.json \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/runs/production
```

The config carries the compute-relevant run scale. Current fields are:

- `polytope_table`: input geometry JSONL path, either a sys-datascience
  producer file such as `produce/random.jsonl` or a compact retained panel;
- `source`: capacity-source filter used when selecting basepoints from the
  input table; defaults to `random_sample`;
- `buckets`: per-facet basepoint counts and beta-boundary row counts;
- `steps`: perturbation radii for `a = a0 + t u`;
- `sysext_cache_inputs`: optional extra JSONL cache files for expensive
  target `sysext` rows;
- `sysext_cache_output`: optional cache output path; defaults to
  `<out-dir>/sysext-cache.jsonl`;
- `trace_iterations`: optional trace depth, usually omitted for smoke panels.

Core identity outputs are:

- `basepoints.jsonl`: selected basepoint identities;
- `basepoint-event-panel/basepoint-provenance-panel.jsonl`: basepoint
  selection provenance;
- `states.jsonl`: base and target polytope states;
- `events.jsonl`: perturbation-target relations.

Observation/detail outputs such as `local-geometry-probe.jsonl`,
`prediction-cloud.jsonl`, branch annotations, beta-boundary rows, and
`dataset-summary.json` support current analysis and run auditing. The
perturbation-detail files live under
`basepoint-event-panel/perturbation-cloud/`; the root-level identity rows are
copies for quick joins and provenance checks.

## Code map

- `src/panel.rs`: config parsing, stage orchestration, and summary assembly;
- `src/basepoints.rs`: geometry-table basepoint selection and provenance rows;
- `src/prediction_cloud.rs`: perturbations, target `sys` recomputation, and
  prediction-error rows;
- `src/panel_analysis.rs`: summary slices over produced perturbation and
  beta-boundary rows;
- `src/panel_cache.rs`: `sysext` cache path resolution;
- `src/panel_io.rs`: packet-local JSON/JSONL helpers;
- `src/schema.rs`: basepoint/state/event identity rows;
- `src/sysext_beta_boundary_scan.rs`: beta-boundary report rows.

The active cache stores one `sysext` row per target polytope: geometry key,
facet count, volume, minimum action, scalar `sys`, iteration count, and
`sigma_results` entries with `sigma`, `action`, and `beta_positive`. The
`beta_positive` flag is retained to document that the branch scan is not
silently restricted to valid `beta>0` sys branches. Gradients, direction
choices, prediction windows, and summary tables are derived outputs, not cache
state.

The cache is an acceleration artifact, not dataset identity. The default
output cache is also loaded as an input, and cache misses append immediately,
so rerunning the same output directory reuses prior expensive target searches.
Keep the durable generation surface to `produce/configs/smoke.json` and
`produce/configs/production.json`; change counts and radii there instead of
adding special-case run folders.
