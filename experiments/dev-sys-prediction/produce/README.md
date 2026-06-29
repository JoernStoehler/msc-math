# Dev Sys Prediction Panel

This surface owns deterministic local panels for `sys(a0 + t u)` prediction
audits. The public data flow is:

```text
basepoints -> states -> perturbation events -> observations/reports
```

The realized sampling law is documented in `src/panel.rs`. In short: basepoints
are selected separately inside each configured facet-count bucket, and
directions are deterministic functions of the basepoint branch geometry plus
fixed pseudo-random controls. Do not interpret current panels as iid samples
from an independent law on `(F, a0, u)`.

The smoke config uses the compact retained panel from
`facet-scale-baseline-error/`, so hydrate that LFS file before running it in a
fresh worktree:

```bash
git lfs pull --include='experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl'
```

Run a smoke panel with:

```bash
cargo run -p exp-dev-sys-prediction --release \
  --bin dev-sys-prediction-panel -- \
  --config experiments/dev-sys-prediction/produce/configs/smoke.json \
  --out-dir /tmp/dev-sys-prediction-panel-smoke
```

`local-small.json` uses the default prepared sys-datascience table instead;
hydrate `experiments/sys-datascience/prepare/polytope-table.jsonl` before
running that larger local config in a fresh worktree.

The config carries the compute-relevant run scale. Current fields are:

- `polytope_table`: input prepared-table or compact panel JSONL path; defaults
  to the LFS-backed sys-datascience prepared table;
- `source`: capacity-source filter used when selecting basepoints from the
  input table; defaults to `random_sample`;
- `buckets`: per-facet basepoint counts and beta-boundary row counts;
- `steps`: perturbation radii for `a = a0 + t u`;
- `sys_cache_inputs`: optional extra JSONL cache files for expensive full
  orbit searches;
- `sys_cache_output`: optional cache output path; defaults to
  `<out-dir>/sys-computation-cache.jsonl`;
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

Code ownership is deliberately shallow:

- `src/panel.rs`: config parsing, stage orchestration, and summary assembly;
- `src/basepoints.rs`: prepared-table basepoint selection and provenance rows;
- `src/prediction_cloud.rs`: perturbations, target `sys` recomputation, and
  prediction-error rows;
- `src/panel_analysis.rs`: summary slices over produced perturbation and
  beta-boundary rows;
- `src/panel_cache.rs`: richer full-orbit cache path resolution;
- `src/panel_io.rs`: packet-local JSON/JSONL helpers;
- `src/schema.rs`: basepoint/state/event identity rows;
- `src/sysext_beta_boundary_scan.rs`: beta-boundary report rows.

The active `sys` cache stores the richer full-orbit computation payload used by
prediction diagnostics, not just scalar `sys`. It is an acceleration artifact,
not the dataset identity. The default output cache is also loaded as an input,
so rerunning the same output directory reuses prior expensive searches. Change
counts and radii in `produce/configs/*.json` for smoke versus larger local
runs. `local-small.json` is a larger local development config, not a retained
production panel recipe.
