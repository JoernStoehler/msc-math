# Generator polarity pushforward and center audit

This packet audits centroid-centered polarity as a deterministic pushforward of
the current planar factor law. It is deliberately target-free: it never calls
`sys`, a capacity evaluator, or any target-derived selector.

## What is separated

For a rational polygon `Q` and explicit interior mark `c`, the producer forms
`B=Q-c` and `B^circ={y : <x,y> <= 1 for x in B}`. The panel retains both the
preserved-mark image and the centroid-canonicalized image, with exact double
polarity controls. The centroid law records the exact area centroid and is
translation-covariant. Re-centering after every polar is tested separately and
is not assumed involutive. Raw-origin polarity under translation is an explicit
failure control. Independently area-normalized factors retain the exact scale
correction `scale^2=1/area`; they are not described as literal polars after
normalization.

## Producer and artifacts

`run.py` samples IID sorted normal angles and IID supports in `[0.8,1.2)` as in
the current factor law, conditions on bounded irredundant polygons, then makes
an explicit `Fraction.limit_denominator(1e9)` reconstruction. All incidence,
area, centroid, polarity, double-polar, and Mahler fields after that boundary
are exact. Shape views and nearest-cross summaries are f64 diagnostics.

The retained default has 24 sources in each side-count stratum `3,4,6` (72
source/image pairs), plus 144 product-arm rows over paired `QxP`,
`Q^circxP`, `QxP^circ`, and `Q^circxP^circ` cells. `fixtures.json` contains
non-self-polar, marked double-polar, centroid covariance, raw-origin failure,
normalization, recenter-every-step, and symmetric double-polar negative
controls. `REPORT.md` and `manifest.json` are generated, not hand-edited.

```text
python3 experiments/sys-datascience/methods/generator-polarity-pushforward/run.py \
  --out-dir experiments/sys-datascience/methods/generator-polarity-pushforward/artifacts \
  --seed 20260715 --per-stratum 24
```

The manifest binds source revision/tree, tracked-clean status before artifact
creation, producer bytes, inputs (none), output hashes, seed, and exact/f64
boundaries. The packet uses Python standard library only and no LFS objects.

## Interpretation guardrails

`P_#mu` is a paired deterministic pushforward, not an independent draw.
Double polarity is a representation/pushforward control, not new coverage.
Finite-panel nearest-cross summaries cannot prove law equivalence, density,
support exhaustion, population rank, independence, or effective sample size.
No target, `sys`, or capacity claim is permitted. A later target pilot would
need a fresh review of the exact evaluator and preserved source/image pairing.
