# Polarity pushforward and center audit

This is a target-free finite-panel audit of planar generator transfer. It does not evaluate `sys`, any capacity, or a target-derived feature.

## Retained panel

- Source law: IID sorted normal angles and IID support heights in `[0.8,1.2)`, with max cyclic angular gap `< pi`, every intersection satisfying every original halfspace, and every input line active before explicit `Fraction.limit_denominator(1e9)` reconstruction and an exact hull.
- Exact panel: 72 source/image pairs, strata `n=3,4,6`, 24 per stratum across seeds `20260715,20260716,20260717`. Aggregate seed counts are `{'20260715': 24, '20260716': 24, '20260717': 24}` and seed-by-stratum counts are `{'20260715': {'3': 8, '4': 8, '6': 8}, '20260716': {'3': 8, '4': 8, '6': 8}, '20260717': {'3': 8, '4': 8, '6': 8}}`; bounded-generation failures are `0`. Every row retains source, preserved-mark polar, centroid polar, and both double-polar controls.
- Product arms: 144 exact rows (36 paired cells, four arms per cell: `QxP`, `Q^circ x P`, `Q x P^circ`, `Q^circ x P^circ`). Cartesian H reconstruction, incidence counts, and volume `area(Q) area(P)` are exact; every row carries source IDs and polar image IDs where used.
- No relative-rotation knob is used. Shape views are centered, area-normalized support samples on a 64-direction grid; distances minimize cyclic shifts and reflection. Diversity reports paired-source-included and leave-pair-out directed polar-to-source nearest views separately; the included view is not leave-pair-out and neither is an independence estimate. Factor area normalization is recorded by exact `scale_squared=1/area`; normalized product volume is the exact string `1`, while raw rational areas/volumes remain available.

## Mathematical controls

The marked law translates by an explicitly preserved interior mark `c`, applies `B^circ={y:<x,y><=1 for x in B}`, and retains the translated origin mark. The centroid law chooses the exact area centroid and records that choice. The exact fixture results are in `fixtures.json`: marked double polarity residual `0.0`, centroid translation covariance residual `0.0`, raw-origin translation status `defined: raw origin remains interior` with raw-vs-centroid residual `0.47580645161290325`, and recenter-every-step residual `0.04592980271273779` (non-involution). The symmetric double-polar negative control has residual `0.0`. Synthetic support-metric controls are in `fixtures.json`; scale, translation, 90-degree rotation, and reflection should be zero while the distinct-triangle control should be positive.

## Interpretation boundary

Polar images are deterministic pushforwards paired to their sources. `P_#mu` is not an independent law draw, double polarity is not new coverage, and finite-panel nearest-cross summaries are not proof of law equivalence. These rows support only geometric calibration and descriptive finite-panel comparisons; they do not support density/support exhaustion, population rank, independence, effective sample-size, target, `sys`, or capacity claims. Raw-origin polarity language is prohibited unless an origin mark is explicitly supplied. Rational vertices/facets/incidence/areas/centroids/polars/Mahler fields are exact after the rationalization boundary. `scale_f64`, support samples, distances, and stored residual numbers are f64 diagnostics, even when derived from exact equalities.

## Provenance and replay

Source revision `381acb31537998192a260cb941cc930f18c2d017`, tree `3549e22cb5d962fde27c364ab1be4c43eadf94c7`, tracked-dirty before artifact creation `false`. Producer SHA-256 `0754518e2e9ca501d2ed338b02dda78434ebe0e120b587eb05dfe3f585171c5d`. Python dependencies are standard library only. Reproduce from that source revision (the later artifact commit changes repository `HEAD`) with:

```text
git worktree add --detach /tmp/generator-polarity-replay 381acb31537998192a260cb941cc930f18c2d017
python3 /tmp/generator-polarity-replay/experiments/sys-datascience/methods/generator-polarity-pushforward/run.py --out-dir /tmp/polarity-artifacts --seed 20260715 --per-stratum 24
sha256sum /tmp/polarity-artifacts/{panel.jsonl,diversity.tsv,product-arms.jsonl,fixtures.json}
```

Volatile timings are excluded. No external or LFS input is used.
