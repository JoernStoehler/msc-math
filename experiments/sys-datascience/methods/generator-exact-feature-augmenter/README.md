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
  --out-dir /tmp/generator-transfer-smoke/tangential-source --seed 20260714 \
  --attempts 128 --runtime-cap-ms 2000 --rows-per-law 1 \
  --only-family factorial --identity-scope generator-transfer-smoke-v1
target/release/sys-datascience-alternative-generator-smoke \
  --out-dir /tmp/generator-transfer-smoke/tangential-replay --seed 20260714 \
  --attempts 128 --runtime-cap-ms 2000 --rows-per-law 1 \
  --only-family factorial --identity-scope generator-transfer-smoke-v1 \
  --geometry-sidecar
uv run --script augment.py --orientation /tmp/generator-transfer-smoke/orientation/rows.jsonl \
  --tangential-source /tmp/generator-transfer-smoke/tangential-source/smoke-rows.jsonl \
  --tangential-replay /tmp/generator-transfer-smoke/tangential-replay/smoke-rows.jsonl \
  --out-dir /tmp/generator-transfer-smoke/features
uv run --script analyze.py --input /tmp/generator-transfer-smoke/features/features.jsonl \
  --augment-report /tmp/generator-transfer-smoke/features/augment-report.json \
  --out-dir /tmp/generator-transfer-smoke/report --design disposable
```

The disposable design is exactly one orientation base in `3x3` and one
tangential pairing in each of `3x3`, `4x6`, and `6x6` (17 rows). The retained
design is selected with `--design retained`: two orientation bases in each of
`3x3`, `4x4`, `4x6`, and `6x6`, and 64 tangential pairings in each of
`3x3`, `4x6`, and `6x6` (808 rows). Use `--require-clean`,
`--expected-revision`, and the three expected SHA256 flags on augmentation;
pass the resulting `augment-report.json` to the analyzer, which verifies the
feature hash/count, source hashes, revision, and clean-state evidence before
enforcing the declared design.

## Features and numerical boundary

The dual/primal reconstruction, incidence, facet/vertex/two-face counts, and
strict signs use `Fraction` arithmetic. For an ordered two-face polygon,
Euclidean area is the norm of its 4D bivector integral divided by two; unsigned
symplectic area is `abs(sum omega(v_i,v_{i+1}))/2`. Both are normalized by
`sqrt(volume)`. The decomposition is `symplectic_area = euclidean_area*kappa`;
the packet reports kappa summaries, Euclidean-weighted kappa, covariance,
symplectic max/top-three concentration, entropy/effective-face count, and
absolute and relative identity error. Structural-zero symplectic faces remain
in the face population and summaries.

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

The authoritative tangential JSONL and separately regenerated replay are
joined one-to-one by `sample_id`; every non-timing row contract field and null
target is compared before replay geometry is consumed. Augment/report outputs
record input paths, SHA256 hashes, row counts, schema/command/tolerances, and
the local Git revision/dirty state.

The analyzer keeps orientation buckets and tangential buckets/arms explicit,
requires complete five-variant orientation grids and four-arm tangential grids,
rejects duplicate/truncated/wrongly joined rows, and rejects any target field
in the input path. It enforces the orthogonal and U(2) controls (with
scale-aware `1e-9` absolute/relative tolerance), reports scaled maxima by
controlled field, and emits bucket-stratified tangential distributions for
every Euclidean, symplectic, kappa, rho, condition, and covariance-status
field. Tangential paired deltas include Euclidean/symplectic sums and means,
symplectic max-share, weighted kappa, rho, and condition; Euclidean arm ranges
include an explicit overlap interval, tolerance-aware boolean, and
union-normalized overlap.
No bucket is pooled into a ranking.

## Retained target-free audit

The retained audit was produced from clean revision
`93056377359ce16fa61e34201d7efdf58bd14405`. The augmenter was run with
`--require-clean`, that expected revision, and expected SHA256 hashes for all
three inputs:

- orientation panel: `b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367`;
- authoritative tangential panel: `0752d44113fe2b1e6bbf0c5af56e1e74e594d7986d48b50a065ed1966bced5ab`;
- exact tangential replay: `0dc72ef98fb860ad838026e5354c36f254985aca9b90a060aee2f58f231832d2`.

The retained artifacts are under `artifacts/full-panels/`: the replay, the
808-row `features.jsonl`, `augment-report.json`, and the analyzer `report.json`.
Their feature, augmentation-report, and analyzer-report SHA256 hashes are,
respectively,
`e7cc585b2e774bc6ee5dcd658e49b02cefd7cdd914fb1ffaba759ccb64d6b624`,
`67d58ec4a9fedd3a62cd79167e3970b20176e60bbc962dd74a76de9bedc6a63a`, and
`3f094ffd96caa19be7c59f8e6fb701326cef0e4d806cbd0ed470fdeb180ce496`.
The exact design audit passed: 40 orientation rows and 768 tangential rows.
Both U(2) and orthogonal controls passed, with maximum scaled controlled error
`1.93e-13`. The 66 strict-cycle-bearing rows are exactly the two orientation
`3x3` identity bases and 64 tangential `3x3` factorial-baseline rows; the
behavioral audit confirms that this metadata affected no grouping or selection.

The orientation panel gives the clearest intervention. Haar-random SO(4)
rotation preserves volume and Euclidean/covariance controls while changing the
symplectic ridge profile on every retained base: mean absolute changes across
the eight bases are about `15.32` in normalized symplectic-area sum, `0.816` in
normalized symplectic-area mean, `0.206` in Euclidean-weighted kappa, and
`23.83` in Williamson rho. Symplectic max-share decreased for all eight bases
(mean delta `-0.0627`, range `[-0.1449,-0.00605]`). U(2) controls are unchanged
to numerical tolerance. The deterministic SO(4) transform also happens to
preserve the unsigned symplectic features on these bases, so the evidence for
an orientation intervention comes specifically from the Haar SO(4) arm.

Tangential perturbations are much more closely matched in these aggregate
geometry features. The `3x3` bucket is an exact/numerical negative control.
For the both-versus-baseline arm, normalized Euclidean ridge-area sum changes
by about `-0.63%` in `4x6` and `-1.01%` in `6x6`; normalized symplectic sum by
about `-0.29%` and `-1.24%`; and symplectic max-share by about `-2.24%` and
`-3.19%`. Arm-range overlap is high for Euclidean mean/sum, including about
`0.99` union-normalized overlap in `6x6`, but some distribution-shape features
are less closely matched: the minimum reported `6x6` Euclidean feature overlap
is about `0.506`. Williamson-rho and covariance-condition summaries are
heavy-tailed in the smaller buckets, so their paired means are descriptive,
not stable effect estimates.

These are target-free geometry results, not evidence about `sys` or population
frequency. There are only eight orientation bases. The retained audit therefore
selects a small Haar-SO(4) orientation target pilot as the first downstream
experiment; the tangential panel is a secondary matched-control pilot, with
bucket- and feature-specific matching diagnostics retained rather than a claim
of exact distributional matching.
