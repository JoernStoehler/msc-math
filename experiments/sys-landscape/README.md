# Sys-Landscape Package

Scope: `experiments/sys-landscape`.

This package owns non-local global and hostile-landscape experimentation for sys on 4D polytopes and product families.

Packet map:
- `random-sample/` owns generic random polytopes (`F=5..12`, rejection-sampled), with canonical output `random-sweep.jsonl`.
- `random-product-sample/` owns random Lagrangian product families (`3<=k<=m<=6`), with canonical output `random-product-sweep.jsonl`.
- `rejection-calibration/` owns sampling acceptance calibration, with `acceptance.jsonl`.
- `rotated-regular-products/` owns regular polygon rotation sweeps and base regular-family maxima.
- `gradient-ascent-general/` owns fixed-`F` general-polytopes continuation (`n=4F` gradient ascent with escape).
- `gradient-ascent-products/` owns fixed-`F` Lagrangian-product projected ascent.
- `variable-f-ascent/` owns `F=10→F=11` continuation trials (`RQ1`, `RQ2`) and comparison metrics.
- `feature-pattern-search/` owns hostile-landscape closure regression pipeline and normalized feature blocks (`feature_*.jsonl`).
- `normalized-dataset/` owns the shared `polytopes.jsonl`, `states.jsonl`, `capacity_results.jsonl`, `step_events.jsonl` contract.
- `feature-*` binaries (`sys-feature-skeleton`, `sys-feature-face-geometry`, `sys-feature-face-symplectic`, `sys-feature-omega`, `sys-feature-orbit`, `sys-feature-trajectory`) own feature-enrichment extractors.
- `pentagon-rotation-formula/` owns branchwise formula exploration for `P5 x_L R(θ)P5`.
- `gradient-ascent-dev/` is scaffold for step-strategy calibration (`step-calibration`, `strategy-comparison`) and remains non-production.

Shared code and interface:
- `src/lib.rs` centralizes ascent helpers, step bounds, and shared event types used by ascent and feature binaries.
- `Cargo.toml` bins currently include:
  - `sys-rotated-regular-products`, `sys-gradient-ascent-general`, `sys-gradient-ascent-products`,
    `sys-random-sample`, `sys-random-product-sample`, `sys-rejection-calibration`,
    `sys-variable-f-ascent`, `sys-normalized-dataset`, `sys-pentagon-rotation-formula`,
    and all `sys-feature-*` binaries.
