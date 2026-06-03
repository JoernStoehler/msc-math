# DS-I007 Exact-vs-f64 Spot Check

idea_id = DS-I007

## Command And Provenance

- Actual command run: `uv run --script experiments/sys-landscape/datascience/methods/exact-f64-spot-check/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr`
- Dataset path: `/tmp/sys-ds-pilot1-tables-tH33Hr`
- Producer command from packet: `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir /tmp/sys-ds-pilot1-tables-tH33Hr`
- Run timestamp UTC: `2026-04-30T16:23:40.138816+00:00`
- Python: `3.12.3`

## Heartbeat

- Heartbeat artifact path: `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/report.md`.
- Heartbeat status: this report path was created with `idea_id`, dataset path, planned command, and current status before full method implementation.

## Dataset Snapshot

- Polytope rows: `282` expected `282`.
- Observation rows: `282` expected `282`.
- Max `sys`: `0.906316153431123` expected `0.906316153431123`.
- `sys > 1` count: `0` expected `0`.
- Dataset guard passed: `True`.

## Sample Policy

Observation: deterministic bounded sample: min/quartile/median/quartile/max sys rows, top five sys rows, and every 53rd row after sorting by sys; duplicates by poly_id removed. Sample size: `14`.

Sampled `sys` values:

```json
[
  0.004489394168139556,
  0.12919967268781993,
  0.2094453173254387,
  0.3228091869465833,
  0.47113141230797684,
  0.598925434564416,
  0.798937037463905,
  0.7996731483730771,
  0.875353292983978,
  0.9029645269387064,
  0.9031764782043712,
  0.9037701674852815,
  0.9046800866780647,
  0.906316153431123
]
```

## Checked Quantities

Observation: each sampled row checks `dual_vertices_rational` parsed as exact `Fraction` values against `dual_vertices_f64` and `dual_vertices_flat_f64`.
Observation: maximum exact rational-to-stored-f64 absolute coordinate error was `0.000e+00`; maximum relative coordinate error was `0.000e+00`.
Observation: maximum nested-vs-flat f64 disagreement was `0.000e+00`.

Observation: the script also recomputes the following f64 geometry columns from exact rational coordinates converted to f64 and the table's f64 `volume`:

```json
[
  "geom_vol1_norm_mean",
  "geom_vol1_norm_std",
  "geom_vol1_norm_min",
  "geom_vol1_norm_max",
  "geom_vol1_centroid_norm",
  "geom_vol1_coord_std_x",
  "geom_vol1_coord_std_y",
  "geom_vol1_coord_std_z",
  "geom_vol1_coord_std_w",
  "geom_cosine_mean",
  "geom_cosine_std",
  "geom_cosine_min",
  "geom_cosine_max",
  "geom_vol1_pairwise_dist_mean",
  "geom_vol1_pairwise_dist_std",
  "geom_vol1_pairwise_dist_min",
  "geom_vol1_pairwise_dist_max"
]
```

Observation: maximum geometry-column recomputation error was `1.776e-15` in column `geom_vol1_pairwise_dist_max` for poly_id `f7f0e8b2a9fff060d2b2eab2efc6c4d93b72c735322c85d82c4eea772f911f7a`.

## Numerical Tolerances

- Dataset max-`sys` guard tolerance: absolute `1e-15`.
- Coordinate verdict threshold: absolute `1e-15`.
- Geometry recomputation verdict threshold: absolute `5e-13`.

## Inference

Inference: the sampled rational vertex coordinates survive table conversion to f64 at ordinary binary64 rounding scale, and the flat f64 vertex array is exactly consistent with the nested f64 vertex array in the sample.
Inference: the selected geometry scalar columns are internally consistent with the table writer formulas when recomputed from rational coordinates through f64 arithmetic.
Inference caveat: this does not prove exactness of derived scalar semantics. The spot check does not reconstruct exact volume, capacity, skeleton, ridge, transition, or orbit-search quantities.

## Verdict

- verdict: `supporting sanity check`
- evidence_strength: `medium`
- implementation_trust: `high`
- thesis_use: `supporting/caveat only`
- caveat: This is a bounded sanity check. It checks exact rational-to-f64 vertex encoding and f64 recomputation of selected geometry scalars, not exact capacity, volume, skeleton, ridge, or orbit-search scalar semantics.
- reopen_trigger: Reopen if DS-I004/DS-I005 start relying on a surprising scalar column, if the dataset producer changes, or if a future table adds exact source columns for volume, skeleton, ridge, capacity, or orbit quantities.
