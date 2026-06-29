# Facet-Count Scale And Baseline Prediction Error

Status: compact empirical calibration packet for future
`dev-sys-prediction` radius-grid and normalization choices.

This packet answers a practical question, not an optimizer-design question:
how absolute flattened-coordinate radii compare across facet counts, and where
the current lower-envelope linear prediction starts to fail on a small
high-`sys` panel.

The main checked panel remains the six-basepoint high-`sys` calibration panel.
The `larger-random-panel/` directory is a follow-up comparison panel with
`8` deterministic `random_sample` basepoints per facet count. It tests whether
the headline error-decomposition pattern was only a tiny high-`sys` panel
artifact.

## Scope

Inputs:

- compact selected panel:
  `polytope-panel.jsonl`;
- facet counts: `F=6,10,12`;
- selected rows per facet count: `2`, highest `sys` rows available in the
  prepared table when this packet was produced;
- prediction branch window: relative action window `0.01`;
- radii: `1e-4,1e-3,1e-2,3e-2`.

The producer direction vectors are normalized in flattened `R^(4F)` dual-vertex
coordinate space. Thus `t` is an absolute Euclidean step length. Per coordinate,
a unit direction has RMS `1/sqrt(4F)`, so the same absolute `t` is a smaller
per-coordinate perturbation at larger `F`.

This is a calibration panel, not a broad statistical study. It deliberately
uses high-`sys` rows because later prediction/error-model sessions care about
search-relevant basepoints.

## Reading The Columns

Common terms:

- `t`: absolute Euclidean step length in flattened dual-vertex coordinates.
- `candidate-window lower envelope`: the base-window model
  `min_sigma(gap_sigma(a0) + t D sys_sigma(a0)[u])`.
- `candidate_window_predicted_delta_sys`: the predicted delta for this
  lower-envelope model.
- `direction_model_predicted_delta_sys`: the model used to propose the tested
  direction; older rows call this `predicted_delta_sys`. Do not use it as the
  candidate-window prediction unless the direction model was explicitly the
  candidate-window model.
- `total error`: predicted target `sys` from that lower-envelope model minus
  recomputed target `sys`.
- `fixed-sigma linearization error`: error from linearly approximating the
  predicted winning branch itself.
- `inside-window selection error`: error from choosing the predicted winning
  branch instead of the exact best branch inside the base candidate window.
- `window-miss error`: difference between the exact best `sys` inside the
  base branch window and the recomputed target `sys`; nonzero values mean the
  base window missed the target behavior even if fixed-branch linearization was
  good.
- `capacity/volume/interaction errors`: split the fixed-sigma linearization
  error using `sys_sigma(a)=c_sigma(a)^2/(2 vol(a))`.
- `target_best_not_in_base_window`: count of valid rows where the target
  minimizer was not among base candidate-window sigmas.
- `target_best_base_sys_gap`: for visible target winners, how much larger the
  target-winning branch value was than `sys(a0)` at the base point.

Do not read this packet as evidence that one facet count is intrinsically
easier or harder than another. In this selected high-`sys` panel, facet count
is confounded with degeneracy regime: `F=6` rows are large-gap, `F=10` rows are
high-degeneracy, and `F=12` rows are narrow-gap.

## Regeneration

The checked packet is reproduced from the tracked compact panel and tracked
diagnostic outputs in this directory. The prepared table is LFS-backed and can
be used to refresh the panel, but doing so changes the experiment input if the
prepared schema/table has changed.

```bash
git lfs pull --include='experiments/sys-datascience/prepare/polytope-table.jsonl,experiments/sys-datascience/prepare/polytope-provenance-table.jsonl'
```

Refresh the compact panel only when intentionally rerunning this packet against
the current prepared table:

```bash
python3 experiments/dev-sys-prediction/facet-scale-baseline-error/select_panel.py
```

Run the branch-window diagnostic:

```bash
cargo run -p exp-dev-gradient-ascent --release --bin dev-gradient-ascent-branch-diagnostic -- \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --provenance-table experiments/sys-datascience/prepare/polytope-provenance-table.jsonl \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --max-rows 99 \
  --thresholds-relative 1e-6,1e-3,1e-2
```

Run the finite-radius local prediction cloud:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-produce -- \
  --diagnostic-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/local-decomp-cloud \
  --selection-threshold-relative 0.01 \
  --action-window-relative 0.01 \
  --degeneracy-labels high_degeneracy,large_gap,narrow_gap \
  --max-fixtures-per-label 2 \
  --steps 1e-4,1e-3,1e-2,3e-2 \
  --trace-iterations 0 \
  --skip-endpoint-diagnostics \
  --direction-model near-active
```

New panels should use `dev-sys-prediction-panel`; see `../produce/README.md`.

Summarize:

```bash
uv run --script experiments/dev-sys-prediction/facet-scale-baseline-error/summarize_panel.py \
  --branch-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --prediction-dir experiments/dev-sys-prediction/facet-scale-baseline-error/local-decomp-cloud \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/summaries
```

## Outputs

Generated tables:

- `summaries/SUMMARY.md`: compact human/GPT-readable table of the current
  scale, branch-window, and prediction-error patterns;
- `summaries/MANIFEST.json`: row counts, byte sizes, SHA-256 hashes, and
  expected-empty status for source and summary artifacts;
- `summaries/branch-window-by-facet.csv`: branch-window size by facet count and
  threshold;
- `summaries/prediction-error-by-facet-step.csv`: prediction error and
  target-winner visibility by facet count and radius;
- `summaries/panel-scale.csv`: selected basepoint scale fields.
- `summaries/prediction-error-by-radius.svg`: dependency-free audit plot of
  median and max absolute prediction error against radius.

When `--trace-iterations 0 --skip-endpoint-diagnostics` is used,
`run-trace.jsonl`, `prediction-cloud.jsonl`, and endpoint JSONL files are
expected to be empty. The local finite-radius rows are in
`local-decomp-cloud/local-geometry-probe.jsonl`.
The same directory also retains `summary.json` plus `basepoints.jsonl`,
`states.jsonl`, and `events.jsonl` from the current producer so the local
prediction rows are inspectable without reconstructing identity/provenance
from code.
The empty files are retained because the producer writes its standard artifact
set even for local-only runs; deleting them would make the checked-in packet
less faithful to regeneration.

The larger comparison panel is reproduced by:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-panel -- \
  --config experiments/dev-sys-prediction/facet-scale-baseline-error/larger-random-panel/config.json \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/larger-random-panel
```

Its retained `dataset-summary.json` reports `24` basepoints, `408` local probe
rows, and about `725s` walltime on the local devcontainer run.

## Interpretation

The useful calibration is qualitative. Median per-coordinate RMS is stable
across the checked compact panel, while flattened norms grow with `sqrt(F)`.
The same absolute radius is therefore a smaller per-coordinate perturbation at
larger `F` on this panel, but current evidence does not justify replacing the
shared absolute grid by a purely `F`-scaled grid.

The checked panel supports treating radii through `1e-2` as the shared local
grid for the next prediction-error sessions. The `3e-2` radius is a stress
radius: it exposes construction failure or branch-window breakdown in this
small panel. The detailed counts and error magnitudes live in
`summaries/prediction-error-by-facet-step.csv`.

The dominant failure mode is not one monotone function of `F`. It depends on
branch-window coverage and is confounded with degeneracy regime in this panel:
selected `F=6` rows are large-gap, selected `F=10` rows high-degeneracy, and
selected `F=12` rows narrow-gap. The retained local prediction rows expose the
full decomposition into fixed-sigma linearization, inside-window branch
selection, and window-miss terms.

The larger random-sample comparison panel supports the mechanism-level part of
this interpretation: the inside-window selection term stays zero in every
`(F,t)` bucket, and nonzero window-miss effects appear only at the stress
radius in a few rows. It does not reproduce the high-`sys` panel's extreme
`F=6` stress-radius errors, so those should be treated as panel-specific tail
events, not as a stable `F=6` claim.

Statistical uncertainty:

- The table reports `mean_abs_total_error`, sample `sd_abs_total_error`, and
  `normal_se_mean_abs_total_error`, but this is only a rough Gaussian-style
  uncertainty calculation for the mean absolute error.
- The sample size per `(F,t)` cell is only `8` or `10` rows before failures.
  If the distribution is heavy-tailed, this panel has weak tail coverage. With
  `10` valid rows, the probability of seeing at least one member of a true
  `5%` tail is only about `0.401`; for a `1%` tail it is about `0.096`. With
  `8` rows those probabilities are about `0.337` and `0.077`.
- Therefore the medians and normal SE columns are useful for local calibration,
  but max-error and p90 columns should be treated as lower bounds on tail risk,
  not reliable tail estimates.

## Radius Recommendation

For the next prediction-error-model sessions, use a shared absolute grid first:

```text
1e-4, 3e-4, 1e-3, 3e-3, 1e-2
```

Use `3e-2` as a stress radius, not as the default local radius. It is useful for
detecting target construction failures and window-miss breakdown, but the
selected panel already shows failures or large errors there.

Record normalization columns with every later panel:

```text
t
t / median_flat_norm(F)
t / median_inter_polytope_dist(F)
t / sqrt(4F)              # per-coordinate RMS displacement for unit directions
```

Current evidence does not justify replacing the shared absolute grid by a
purely `F`-scaled grid. It does justify reporting normalized radii, because
the same absolute `t` has smaller per-coordinate RMS at larger `F`.

Conservative defaults by facet count:

| `F` | default local grid | stress grid |
| --- | --- | --- |
| 6 | `1e-4,3e-4,1e-3,3e-3,1e-2` | add `3e-2` only for breakdown checks |
| 10 | `1e-4,3e-4,1e-3,3e-3,1e-2` | add `3e-2` for stress/error tails |
| 12 | `1e-4,3e-4,1e-3,3e-3,1e-2` | add `3e-2` for stress/error tails |

## Thesis-Authoring Use

This packet can support a thesis-facing sentence of the form: on a compact
high-`sys` calibration panel, absolute radii through `1e-2` stayed in the
useful local regime for the current lower-envelope prediction model, while
`3e-2` exposed construction failures or large model-error tails.

Do not cite it as:

- a statistical estimate of prediction error over the full search
  distribution;
- evidence that `F=6`, `F=10`, and `F=12` differ for facet-count reasons alone;
- optimizer evidence;
- a tail-risk bound.

## Residual Risks

- Degeneracy regime is confounded with facet count in this high-`sys` panel:
  selected `F=6` rows are large-gap, selected `F=10` rows high-degeneracy, and
  selected `F=12` rows narrow-gap.
- `F=12` is the only larger count checked. There is no `F>12` evidence here.
- The `F=6` selected rows are much lower `sys` than the selected `F=10` rows,
  because that is what the prepared table contains.
- Endpoint diagnostics were deliberately skipped; optimizer endpoint behavior
  is out of scope for this packet.
- The full all-six trace-stage cloud was too slow or session-fragile before the
  local-decomposition instrumentation. The retained evidence is the local
  cloud with decomposition fields, which is the right surface for this radius
  calibration question.
- Tail risk is under-sampled. A future tail-focused packet should deliberately
  increase directions/basepoints per `(F,t)` cell instead of only adding more
  facet counts.
