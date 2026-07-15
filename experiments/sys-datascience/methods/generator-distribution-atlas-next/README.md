# Scaled multi-view generator-distribution atlas

This copy-local packet extends the reviewed shape-quality methods to a bounded
factor-only panel over all eight reviewed generator-zoo populations at line
commit `fd9c3e7d`:

* current baseline `delta=0.2`;
* primal-hull uniform disk;
* regular mutation;
* Dirichlet-gap controls `alpha=1`, `4`, `16`, and `regular`;
* zonogon.

The producer requested 24 rows per population/side-count stratum (side counts
`3,4,6`, with zonogons restricted to `4,6`). It retained 544 validated factor
rows: 24 rows in each non-zonogon stratum where accepted, 16 primal-hull
rows at three sides, and 24 rows in each zonogon stratum. The producer reports
retain attempts, exhaustion, generation time, source revision, and dirty scope.
The panel is geometry-only; it does not call the exact four-dimensional
boundary or `sys`.

## Reproduce

Build the reviewed producer from a clean checkout at `fd9c3e7d`; do not rely on
an existing target cache. From that checkout, run
`cargo build --release --locked --package exp-sys-landscape --bin
sys-datascience-generator-zoo-smoke`, verify the executable SHA-256 against
`artifacts/panel/provenance.json`, and use the resulting binary below:

```bash
$PRODUCER_BIN \
  --factor-only --factor-out-dir /tmp/generator-atlas-core \
  --seed 20260715 --attempts 128 --factor-rows-per-population 24 \
  --factor-side-counts 3,4,6 \
  --factor-population 'current-baseline|delta=0.2' \
  --factor-population 'primal-hull-uniform-disk|points=n+4,origin=interior' \
  --factor-population 'repulsive-gap|alpha=1' \
  --factor-population 'repulsive-gap|alpha=4' \
  --factor-population 'repulsive-gap|alpha=16' \
  --factor-population 'repulsive-gap|regular' \
  --factor-population 'regular-mutation|steps=4,scale=0.03'

$PRODUCER_BIN \
  --factor-only --factor-out-dir /tmp/generator-atlas-zonogon \
  --seed 20260715 --attempts 128 --factor-rows-per-population 24 \
  --factor-side-counts 4,6 \
  --factor-population 'zonogon|lengths=uniform(0.5,1.5)'
```

Concatenate the two `factor-shapes.jsonl` files into `artifacts/panel/` and
run:

```bash
python3 atlas.py \
  --input artifacts/panel/factor-shapes.jsonl \
  --producer-report artifacts/panel/core-report.json \
  --producer-executable "$PRODUCER_BIN" \
  --producer-report artifacts/panel/zonogon-report.json \
  --exact-input /workspaces/msc-math/.worktrees/generator-transfer/experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --out-dir artifacts/atlas
python3 -m py_compile atlas.py shape_quality.py
python3 test_shape_quality.py
```

`shape_quality.py` is a self-contained copy of the reviewed quality analyzer,
including strict convexity/CCW validation, exact polygon Steiner centering,
area normalization, rotation quotient, and deterministic hash-ranked grouping.
`atlas.py` keeps the copy-local adapter explicit and uses a declared-grid
circular-correlation distance for the many pair views so the bounded panel is
cheap to rerun. The focused continuous-refinement implementation remains
available in `shape_quality.py` for a later narrow check.

## Views and interpretation

The generated `artifacts/atlas/` directory contains compact tables for:

* within-population L2/L-infinity diversity, nearest neighbors, duplicates,
  effective dimension, and negative eigenmass;
* between-population cross-distance and invariant-feature centroid separation;
* directed nearest cross-population overlap;
* fixed-panel side-count/source-bucket occupancy;
* acceptance, exhaustion, attempts, and generation cost;
* deterministic sample-size saturation at 4, 8, 12, and 24 rows;
* feature covariance spectrum, quantile-range overlap, and population-label
  eta-squared confounding. The anisotropy feature is the eigenvalue ratio of
  centered vertex covariance, so it is invariant under global rotation,
  translation, and positive scale.

The `source-exact-validation-witness` is a source-backed staged witness
selected from the reviewed product factor artifact (`2` rows per
population/side-count group). Its `linkage.json` permits only
population/side-count linkage: witness IDs do not match the larger factor-only
panel, so no row-level or causal transfer is claimed.

The retained `artifacts/panel/provenance.json` binds the panel to the exact
producer executable SHA-256, source revision and Git blob IDs for the producer
Rust source, package manifest, and lockfile. It also records the SHA-256 hashes
of `atlas.py` and `shape_quality.py`. The executable path is capture metadata,
not a reproduction dependency: rebuild it from the pinned source revision with
`cargo build --release --locked --package exp-sys-landscape --bin
sys-datascience-generator-zoo-smoke`, then compare the resulting executable
hash before regeneration.

The packet intentionally does not produce a quality score or global ranking.
Pilot/confirmation separation and repeated-seed rank stability are deferred;
the retained feature and saturation diagnostics are cheap calibration, not
uncertainty estimates. Four-dimensional coordinate/affine/Lagrangian product
classification is also deferred because this factor-only panel has no 4D
normals. Classifier failure must not be read as loss of productness. These
tables support finite-panel redundancy, coverage, under-sampling, and cost
decisions only; they do not establish natural-law probabilities, population
generalization, mechanism, `sys`, or target prediction.
