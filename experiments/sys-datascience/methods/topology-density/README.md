# Calibrated topology and density diagnostics

## Decision and scope

This packet asks whether clouds of named fixed-side planar factors show a
stable, interpretable signal of multiple dense components, holes, or a
low-dimensional shape structure. It is a feasibility check after the accepted
multi-seed generator atlas, not a search for a `sys` candidate and not a
population-support claim.

Each polygon is represented separately within its fixed side count. The
primary vector is formed by subtracting the centroid, dividing by RMS radius,
rotating each cyclic relabelling so its first vertex has zero phase, and
keeping the lexicographically least cyclic representative. Translation,
positive scale, rotation, and cyclic relabelling are therefore quotiented;
reflection is retained. A sorted normalized-radius vector is a second
diagnostic view. Law, population/knob label, side count, and seed are never
pooled silently.

## Methods and calibration

* Persistent Vietoris--Rips homology uses pinned `ripser==0.6.12` with H0 and
  H1 summaries. A bar is called significant only above `0.2 * median(nonzero
  pair distance)` for H0 or `0.35 *` that scale for H1. The H1 threshold was
  chosen before reading atlas results because the filled-disk negative control
  otherwise produced short finite-sample loops.
* Density components use DBSCAN with `min_samples=k`, where `k=min(8,n-1)`.
  Three explicit neighbourhood scales (`0.8`, `1.0`, and `1.2` times the
  median kth-neighbour distance) are evaluated, followed by 32 deterministic
  bootstrap resamples of exactly `n` rows with replacement. Repeated indices
  are retained in DBSCAN, so multiplicity is part of the declared resampling
  contract. Component counts, modal bootstrap fractions, and the observed
  duplicate-row fraction are reported; they are density diagnostics, not
  topology.
* HDBSCAN and a density-level-set tree were not retained. The seven-control
  calibration already selected the explicit DBSCAN contract (7/7 signatures)
  and the extra dependency/knob surface would not answer a distinct question
  at the current sample sizes. This is a measured scope decision, not an
  assertion that HDBSCAN is invalid.

The synthetic suite contains a noisy circle, filled disk, separated Gaussian
mixture, narrow-bridge dumbbell, exact duplicates, anisotropic line, and a
boundary-heavy square. The persistent-H1 method retains the circle and
boundary loop while rejecting the filled disk; density scales distinguish the
separated mixture and bridge; duplicate multiplicity and line controls pass
their explicit guards. The calibration result is retained in
`artifacts/calibration/`.

## Inputs and reproduction

The copied factor rows under `artifacts/input/` are the six core/zonogon files
from the accepted atlas confirmation (`3f09eeeb`, seeds `20260716`--`20260718`).
They are source-bound JSONL, not regenerated or target-derived data. Their
SHA-256 values and corresponding clean
`generator-zoo-factor-only-report-v1` hashes are recorded in
`artifacts/analysis/report.json`; when the owner worktree is available, the
producer checks the copied bytes against that exact owner path as well. The
analyzer fails closed if any declared source file is modified, if the six
input hashes or reports disagree, or if upstream report provenance is dirty.
The report also records the clean source commit/tree and hashes of analyzer,
README, and tests.

From this directory:

```bash
uv run --script analyze.py \
  --out-dir artifacts/calibration --calibration-only \
  --write-synthetic-fixture artifacts/synthetic.jsonl

uv run --script analyze.py \
  --input artifacts/input/seed-20260716/core/factor-shapes.jsonl \
  --input artifacts/input/seed-20260716/zonogon/factor-shapes.jsonl \
  --input artifacts/input/seed-20260717/core/factor-shapes.jsonl \
  --input artifacts/input/seed-20260717/zonogon/factor-shapes.jsonl \
  --input artifacts/input/seed-20260718/core/factor-shapes.jsonl \
  --input artifacts/input/seed-20260718/zonogon/factor-shapes.jsonl \
  --out-dir artifacts/analysis

uv run --with pytest --with numpy==2.2.6 --with scipy==1.15.3 \
  --with scikit-learn==1.6.1 --with ripser==0.6.12 \
  pytest -q test_topology_density.py
```

`calibration.json` and `real.json` are generated reports; `report.json` is
the provenance and interpretation contract. Re-running with the same inputs
is byte-identical, including all real-data artifacts (the command records
output paths as placeholders).

The canonical quotient has a lexicographic section: nearly tied cyclic
representatives can switch under floating-point perturbations, and exact
symmetries produce ties resolved by tuple ordering. This is a numerical
coordinate choice, not a smooth global chart. The radial view is intentionally
many-to-one because it discards cyclic adjacency; it is only a stability view,
never an alternative topology claim. Reflections remain distinct and are
tested explicitly.

## Observations and interpretation

All 23 named population/side strata have 51--72 observations from all three
seeds and are therefore descriptive rather than underpowered by this packet's
`n>=12`, two-seed gate. Canonical shape clouds show zero significant H1 bars in
every stratum. Density component counts are usually one at the middle and
large scales, with several side-3 or concentrated-law strata retaining
multiple small-scale DBSCAN components. The radial view agrees on the absence
of persistent H1; occasional component differences between views and scales
are stability warnings, not discovered topology.

Allowed use is limited to comparing these finite-sample component/loop
summaries inside an explicitly named law, population knob, side count, and
seed panel, and to deciding whether a larger shape-cloud study is worthwhile.
The packet does not support pooling side counts or law knobs, natural-law
probabilities, population topology/support, a `sys` or capacity association,
target transfer, or a mechanism claim. No robust H1 bar is detected at these
thresholds and sample sizes in either shape view. This is an inconclusive
finite-sample observation, not evidence that the underlying law has no holes
or has trivial topology.

Reopen before consequential use if the atlas changes, a stratum has fewer than
two independent seeds, the view/scale disagreement becomes material, or a
future claim needs an inferential population statement. Any such reopening
requires a new calibration or an explicitly justified threshold and cannot be
repaired by renaming DBSCAN connectivity as topology.
