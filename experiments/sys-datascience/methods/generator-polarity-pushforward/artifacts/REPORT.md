# Polarity pushforward and center audit

This is a target-free finite-panel audit of planar generator transfer. It does not evaluate `sys`, any capacity, or a target-derived feature.

## Retained panel

- Source law: IID sorted normal angles and IID support heights in `[0.8,1.2)`, conditioned on a bounded irredundant polygon, followed by explicit `Fraction.limit_denominator(1e9)` reconstruction and an exact hull.
- Exact panel: 72 source/image pairs, strata `n=3,4,6`, 24 per stratum. Every row retains source, preserved-mark polar, centroid polar, and both double-polar controls.
- Product arms: 144 exact rows (36 paired cells, four arms per cell: `QxP`, `Q^circ x P`, `Q x P^circ`, `Q^circ x P^circ`). Cartesian H reconstruction, incidence counts, and volume `area(Q) area(P)` are exact.
- No relative-rotation knob is used. Factor area normalization is recorded by exact `scale_squared=1/area`; normalized product volume is therefore one by construction, while raw rational areas/volumes remain available.

## Mathematical controls

The marked law translates by an explicitly preserved interior mark `c`, applies `B^circ={y:<x,y><=1 for x in B}`, and retains the translated origin mark. The centroid law chooses the exact area centroid and records that choice. The exact fixture results are in `fixtures.json`: marked double polarity residual `0.0`, centroid translation covariance residual `0.0`, raw-origin translation status `undefined: translated origin is outside`, and recenter-every-step residual `0.04592980271273779` (non-involution). The symmetric double-polar negative control has residual `0.0`; a metric cannot pass only by reporting nonzero distances.

## Interpretation boundary

Polar images are deterministic pushforwards paired to their sources. `P_#mu` is not an independent law draw, double polarity is not new coverage, and finite-panel nearest-cross summaries are not proof of law equivalence. These rows support only geometric calibration and descriptive finite-panel comparisons; they do not support density/support exhaustion, population rank, independence, effective sample-size, target, `sys`, or capacity claims. Raw-origin polarity language is prohibited unless an origin mark is explicitly supplied. Exact fields end at the rationalization boundary; support/shape views and diversity summaries are f64 diagnostics.

## Provenance and replay

Source revision `ded422fd076b8d741a5a8dc651d02caddb25c7b4`, tree `b0ef60590feb23963fbf33b97c8255a872296993`, tracked-dirty before artifact creation `false`. Producer SHA-256 `e7f70483216a967c23983da61e0750639b9bb5476b0c3fafd77252177a93eb1c`. Python dependencies are standard library only. Reproduce from that source revision (the later artifact commit changes repository `HEAD`) with:

```text
git worktree add --detach /tmp/generator-polarity-replay ded422fd076b8d741a5a8dc651d02caddb25c7b4
python3 /tmp/generator-polarity-replay/experiments/sys-datascience/methods/generator-polarity-pushforward/run.py --out-dir /tmp/polarity-artifacts --seed 20260715 --per-stratum 24
sha256sum experiments/sys-datascience/methods/generator-polarity-pushforward/artifacts/{panel.jsonl,diversity.tsv,product-arms.jsonl,fixtures.json}
```

Volatile timings are excluded. No external or LFS input is used.
