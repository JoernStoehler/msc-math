# Facet-Count Scale And Baseline Prediction Error

Status: compact empirical calibration packet for future
`dev-sys-prediction` radius-grid and normalization choices.

This packet answers a practical question, not an optimizer-design question:
how absolute flattened-coordinate radii compare across facet counts, and where
the current lower-envelope linear prediction starts to fail on a small
high-`sys` panel.

## Scope

Inputs:

- source table:
  `experiments/sys-datascience/prepare/polytope-table.jsonl`;
- compact selected panel:
  `polytope-panel.jsonl`;
- facet counts: `F=6,10,12`;
- selected rows per facet count: `2`, highest `sys` rows available in the
  prepared table;
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
- `total error`: predicted target `sys` from that lower-envelope model minus
  recomputed target `sys`.
- `linearization error`: error from linearly approximating the fixed base
  branch window at the target point.
- `sigma-window error`: difference between the exact best `sys` inside the
  base branch window and the recomputed target `sys`; nonzero values mean the
  base window missed the target behavior even if fixed-branch linearization was
  good.
- `target_best_not_in_base_window`: count of valid rows where the target
  minimizer was not among base candidate-window sigmas.
- `target_best_base_sys_gap`: for visible target winners, how much larger the
  target-winning branch value was than `sys(a0)` at the base point.

Do not read this packet as evidence that one facet count is intrinsically
easier or harder than another. In this selected high-`sys` panel, facet count
is confounded with degeneracy regime: `F=6` rows are large-gap, `F=10` rows are
high-degeneracy, and `F=12` rows are narrow-gap.

## Regeneration

The prepared table is LFS-backed:

```bash
git lfs pull --include='experiments/sys-datascience/prepare/polytope-table.jsonl,experiments/sys-datascience/prepare/polytope-provenance-table.jsonl'
```

Select the compact panel:

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

Run the finite-radius local prediction cloud with candidate-window
decomposition columns written into `local-geometry-probe.jsonl`:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/local-decomp-cloud \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels large_gap,narrow_gap,high_degeneracy \
  --max-fixtures-per-label 99 \
  --steps 1e-4,1e-3,1e-2,3e-2 \
  --trace-iterations 0 \
  --skip-endpoint-diagnostics
```

Summarize:

```bash
python3 experiments/dev-sys-prediction/facet-scale-baseline-error/summarize_panel.py \
  --branch-dir experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic \
  --prediction-dir experiments/dev-sys-prediction/facet-scale-baseline-error/local-decomp-cloud \
  --out-dir experiments/dev-sys-prediction/facet-scale-baseline-error/summaries
```

## Results

Generated tables:

- `summaries/global-scale-by-facet.csv`;
- `summaries/branch-window-by-facet.csv`;
- `summaries/prediction-error-by-facet-step.csv`;
- `summaries/SUMMARY.md`.

When `--trace-iterations 0 --skip-endpoint-diagnostics` is used,
`run-trace.jsonl`, `prediction-cloud.jsonl`, and endpoint JSONL files are
expected to be empty. The local finite-radius rows are in
`local-decomp-cloud/local-geometry-probe.jsonl`.
The empty files are retained because the producer writes its standard artifact
set even for local-only runs; deleting them would make the checked-in packet
less faithful to regeneration.

Global coordinate scale:

- Median per-coordinate RMS is essentially stable across the retained counts:
  about `0.510` for `F=6,10,12`.
- Median flattened norm grows with `sqrt(F)`: `2.50` at `F=6`, `3.24` at
  `F=10`, `3.53` at `F=12`.
- Median sampled inter-polytope flattened distance is much larger than tested
  radii: `3.24` at `F=6`, `4.41` at `F=10`, `3.87` at `F=12`.
- A fixed `t=1e-2` is therefore tiny globally, but not identical as a
  per-coordinate perturbation: unit-direction coordinate RMS is `0.204` at
  `F=6`, `0.158` at `F=10`, and `0.144` at `F=12`.

Branch windows at relative threshold `0.01`:

- `F=6`: both selected rows are large-gap; median near-active count `1`.
- `F=10`: both selected rows are high-degeneracy; median near-active count `8`,
  max `10`.
- `F=12`: both selected rows are narrow-gap; median near-active count `3.5`.

Prediction/error behavior:

- No base recomputation failures occurred in `branch-diagnostic`.
- The local finite-radius cloud selected `6` fixtures and wrote `112` rows.
  `111` rows were valid; one `F=10`, `t=3e-2` row failed target-polytope
  construction.
- At `t=1e-2`, median absolute candidate-window total error was:
  `3.5e-3` for `F=6`, `3.1e-5` for `F=10`, and `5.3e-5` for `F=12`.
- At `t=3e-2`, median absolute total error was:
  `6.7e-2` for `F=6`, `2.7e-4` for `F=10` over valid rows, and `6.9e-4` for
  `F=12`; max errors were much larger, up to `0.414` for `F=6` and `0.033`
  for `F=12`.
- The dominant failure mode is not one monotone function of `F`. It depends on
  branch-window coverage. In the selected `F=6` large-gap rows, sigma-window
  error appears at `1e-2` and dominates by `3e-2`. In the selected `F=10`
  high-degeneracy rows, `1e-2` remains accurate and `3e-2` starts showing one
  construction failure and one large max error. In the selected `F=12`
  narrow-gap rows, median behavior remains good at `3e-2`, but max error shows
  window breakdown.

Target-winner base gaps:

- The summary columns `target_best_not_in_base_window`,
  `median_target_best_base_sys_gap`, `p90_target_best_base_sys_gap`, and
  `max_target_best_base_sys_gap` answer: for the sigma that is minimal at
  `a0+t*u`, how far above the base minimum was that sigma at `a0`?
- At `t<=1e-3`, every valid `F=6` and `F=10` target winner was already in the
  base candidate window; the recorded median base sys gap is `0`. For `F=12`,
  five rows already have target winners outside the base candidate window even
  at small radii, although the resulting prediction error remains tiny at
  `1e-4` and `1e-3`.
- At `t=1e-2`, target winners outside the base candidate window appear in
  `5/8` valid `F=6` rows, `1/10` `F=10` rows, and `5/10` `F=12` rows. Among
  rows where the target winner is visible at base, p90 base sys gap is `0` for
  `F=6`, `0.01098` for `F=10`, and `0.00397` for `F=12`.
- At `t=3e-2`, the outside-window counts are `5/8`, `2/9` valid `F=10` rows,
  and `5/10`. This is the cleanest reason to treat `3e-2` as a stress radius:
  the target minimizer often no longer belongs to the base branch window used
  for prediction.

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
detecting target construction failures and sigma-window breakdown, but the
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
