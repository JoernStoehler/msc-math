# Exact-geometry feature augmenter

This packet augments the reviewed orientation panel and factorial tangential
panel with target-free geometry features. It never reads or computes capacity,
`sys`, iterations, bounce labels, or target artifacts.

## Geometry recovery and joins

Orientation rows already carry rational transformed dual vertices, rational
primal vertices, and a labeled incidence signature. The augmenter checks every
incident equality `a_i dot v_j = 1`, every nonincident inequality, and source
facet/vertex counts.

Tangential source rows do not carry geometry. The alternative-generator smoke
therefore has an opt-in `--geometry-sidecar` mode. It executes the same
deterministic law/seed/bucket/row/attempt path and writes rational dual/primal
vertices, incidence, volume, and a sidecar copy of `sample_id`/`pairing_id`.
The augmenter requires those IDs and volume to match the retained source row;
it does not substitute an unrelated fresh sample. Existing retained JSONL
inputs are not changed.

Example disposable smoke (one complete orientation base and one complete
tangential four-arm panel):

```text
cargo build -p exp-sys-landscape --release \
  --bin sys-datascience-generator-orientation-smoke \
  --bin sys-datascience-alternative-generator-smoke
target/release/sys-datascience-generator-orientation-smoke \
  --out-dir /tmp/generator-transfer-smoke/orientation \
  --rows-per-bucket 1 --buckets 3x3
target/release/sys-datascience-alternative-generator-smoke \
  --out-dir /tmp/generator-transfer-smoke/tangential --seed 20260714 \
  --attempts 128 --runtime-cap-ms 2000 --rows-per-law 1 \
  --only-family factorial --identity-scope generator-transfer-smoke-v1 \
  --geometry-sidecar
python3 augment.py --orientation /tmp/generator-transfer-smoke/orientation/rows.jsonl \
  --tangential /tmp/generator-transfer-smoke/tangential/smoke-rows.jsonl \
  --out-dir /tmp/generator-transfer-smoke/features
PYTHONPATH=. python3 analyze.py --input /tmp/generator-transfer-smoke/features/features.jsonl \
  --out-dir /tmp/generator-transfer-smoke/report
```

## Features and numerical boundary

The dual/primal reconstruction, incidence, facet/vertex/two-face counts, and
strict signs use `Fraction` arithmetic. For an ordered two-face polygon,
Euclidean area is the norm of its 4D bivector integral divided by two; unsigned
symplectic area is `abs(sum omega(v_i,v_{i+1}))/2`. Both are normalized by
`sqrt(volume)`. The decomposition is `symplectic_area = euclidean_area*kappa`;
the packet reports kappa summaries, Euclidean-weighted kappa, covariance, and
maximum identity error.

Centered distinct-primal-vertex covariance uses population normalization.
Ordinary eigenvalues use a symmetric numerical eigensolver. Williamson values
use the exact owner formula `s=-tr((J C)^2)/2`, `p=det(C)`, and the roots of
`t^2-s t+p`; `rho=nu2/nu1`. These eigendecompositions and aggregate summaries
are numerical features computed from exact-reconstructed coordinates, not exact
claims.

Strict Hamiltonian six-cycle metadata is emitted only for orientation
`3x3` identity and tangential `3x3` factorial-baseline rows. Every q-to-p
transition requires exact `omega > 0`, and every p-to-q transition requires
exact `omega < 0`; zero edges are never labeled strict. The strict-sign-cell
flag is retained separately. Cycle metadata is never used for acceptance,
grouping, ranking, or selection; the analyzer report contains an explicit
`strict_cycle_used_for_grouping_or_selection: false` audit.

The analyzer keeps orientation buckets and tangential buckets/arms explicit,
requires complete five-variant orientation grids and four-arm tangential grids,
rejects duplicate/truncated/wrongly joined rows, and rejects any target field
in the input path.
