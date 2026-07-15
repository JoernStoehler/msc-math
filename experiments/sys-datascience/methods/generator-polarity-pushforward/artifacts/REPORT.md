# Polarity pushforward and center audit

This is a target-free finite-panel audit of planar generator transfer. It does not evaluate `sys`, any capacity, or a target-derived feature.

## Retained panel

- Source law: IID sorted normal angles and IID support heights in `[0.8,1.2)`, with max cyclic angular gap `< pi`, every intersection satisfying every original halfspace, and every input line active before explicit `Fraction.limit_denominator(1e9)` reconstruction and an exact hull.
- Exact panel: 72 source/image pairs, strata `n=3,4,6`, 24 per stratum across seeds `20260715,20260716,20260717`. Aggregate seed counts are `{'20260715': 24, '20260716': 24, '20260717': 24}` and seed-by-stratum counts are `{'20260715': {'3': 8, '4': 8, '6': 8}, '20260716': {'3': 8, '4': 8, '6': 8}, '20260717': {'3': 8, '4': 8, '6': 8}}`; bounded-generation failures are `0`. Every row retains source, preserved-mark polar, centroid polar, and both double-polar controls.
- Product arms: 144 exact rows (36 paired cells, four arms per cell: `QxP`, `Q^circ x P`, `Q x P^circ`, `Q^circ x P^circ`). Cartesian H reconstruction, incidence counts, and volume `area(Q) area(P)` are exact.
- No relative-rotation knob is used. Shape views are centered, area-normalized support samples on a 64-direction grid; distances minimize cyclic shifts and reflection. Factor area normalization is recorded by exact `scale_squared=1/area`; normalized product volume is the exact string `1`, while raw rational areas/volumes remain available.

## Mathematical controls

The marked law translates by an explicitly preserved interior mark `c`, applies `B^circ={y:<x,y><=1 for x in B}`, and retains the translated origin mark. The centroid law chooses the exact area centroid and records that choice. The exact fixture results are in `fixtures.json`: marked double polarity residual `0.0`, centroid translation covariance residual `0.0`, raw-origin translation status `defined: raw origin remains interior` with raw-vs-centroid residual `0.47580645161290325`, and recenter-every-step residual `0.04592980271273779` (non-involution). The symmetric double-polar negative control has residual `0.0`. Synthetic support-metric controls are in `fixtures.json`; scale, translation, 90-degree rotation, and reflection should be zero while the distinct-triangle control should be positive.

## Interpretation boundary

Polar images are deterministic pushforwards paired to their sources. `P_#mu` is not an independent law draw, double polarity is not new coverage, and finite-panel nearest-cross summaries are not proof of law equivalence. These rows support only geometric calibration and descriptive finite-panel comparisons; they do not support density/support exhaustion, population rank, independence, effective sample-size, target, `sys`, or capacity claims. Raw-origin polarity language is prohibited unless an origin mark is explicitly supplied. Exact fields end at the rationalization boundary; support/shape views and diversity summaries are f64 diagnostics.

## Provenance and replay

Source revision `9608b0954940839542a51d7cc46a5f41438b8527`, tree `8ca6cb936b4cd217c201c873a9d3c7ce0155c2cd`, tracked-dirty before artifact creation `false`. Producer SHA-256 `856f607f6a62de8d4acdca94b4e25f4395cda4fc42998ad3a9e02b0195bc445f`. Python dependencies are standard library only. Reproduce from that source revision (the later artifact commit changes repository `HEAD`) with:

```text
git worktree add --detach /tmp/generator-polarity-replay 9608b0954940839542a51d7cc46a5f41438b8527
python3 /tmp/generator-polarity-replay/experiments/sys-datascience/methods/generator-polarity-pushforward/run.py --out-dir /tmp/polarity-artifacts --seed 20260715 --per-stratum 24
sha256sum experiments/sys-datascience/methods/generator-polarity-pushforward/artifacts/{panel.jsonl,diversity.tsv,product-arms.jsonl,fixtures.json}
```

Volatile timings are excluded. No external or LFS input is used.
