# Independent-seed atlas confirmation

This packet checks whether the one-seed, 544-row geometry atlas is stable
enough to guide later generator work. It uses three new deterministic master
seeds (`20260716`, `20260717`, `20260718`) and the same eight populations,
side-count strata, and nominal 24-row allocation as the accepted atlas. The
factor-only producer is rebuilt from the declared generator-zoo source closure
at `fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e`; the producer source blobs are
identical in the base checkout at `a50e1e930e21541506df6228aaed16355d830372`.
No exact 4D reconstruction, `sys`, or target field is used.

## Reproduce

From this directory, first build a clean producer checkout:

```bash
cargo build --release --locked --package exp-sys-landscape \
  --bin sys-datascience-generator-zoo-smoke
```

For each seed, run the two factor-only commands in the transferred prompt
(the commands are recorded in each `artifacts/raw/seed-*/{core,zonogon}/`
report). Then run:

```bash
python3 analyze.py \
  --input artifacts/raw/seed-20260716/core/factor-shapes.jsonl \
  --input artifacts/raw/seed-20260716/zonogon/factor-shapes.jsonl \
  --input artifacts/raw/seed-20260717/core/factor-shapes.jsonl \
  --input artifacts/raw/seed-20260717/zonogon/factor-shapes.jsonl \
  --input artifacts/raw/seed-20260718/core/factor-shapes.jsonl \
  --input artifacts/raw/seed-20260718/zonogon/factor-shapes.jsonl \
  --producer-report artifacts/raw/seed-20260716/core/factor-only-report.json \
  --producer-report artifacts/raw/seed-20260716/zonogon/factor-only-report.json \
  --producer-report artifacts/raw/seed-20260717/core/factor-only-report.json \
  --producer-report artifacts/raw/seed-20260717/zonogon/factor-only-report.json \
  --producer-report artifacts/raw/seed-20260718/core/factor-only-report.json \
  --producer-report artifacts/raw/seed-20260718/zonogon/factor-only-report.json \
  --producer-executable ../../../../target/release/sys-datascience-generator-zoo-smoke \
  --out-dir artifacts/analysis
```

The analyzer is copy-local. It retains exact input hashes, producer report
hashes, producer executable hash, source blobs, analyzer hashes, repository
revision/tree, and a tracked-clean predicate in `artifacts/analysis/report.json`.
The grid-distance name and boundaries are explicit: L2 is a declared-grid
circular-correlation rotation quotient; arbitrary rotations are approximate.
Positive-Gram participation is not an intrinsic or metric dimension, and
negative eigenmass only diagnoses a non-Euclidean embedding. Raw unstandardized
feature aggregates remain scale-sensitive diagnostics, never a score.

## Retained artifacts

There are 1,635 validated factor rows (545 per seed: 497 core and 48 zonogon).
Each core seed exhausts the same seven primal-hull triangle requests under the
128-attempt cap; all other requested strata are accepted. The compact analysis
contains:

- `per-seed-effects.tsv`: predeclared contrast effects and pass flags;
- `joint-effects.tsv`: mean, median, between-seed standard deviation, and pass
  rate for each named effect;
- `rank-stability.tsv`: the regular/alpha16/alpha4/alpha1 order per seed and
  explicit reversals;
- `within-population.tsv`, `saturation.tsv`, and `acceptance-cost.tsv`;
- `report.json`: provenance, thresholds, interpretation boundaries, and
  deferrals.

With the predeclared thresholds, baseline/alpha=1 overlap is bidirectionally
substantial in every side stratum for all three seeds. The regular-to-alpha16-
to-alpha4-to-alpha1 distance order is stable at sides 4 and 6 but reverses at
side 3 for seed 20260716. The zonogon diversity ratio does not reach the
predeclared strong threshold (2.0) at side 4 in any confirmation seed, so the
one-seed side-4 excess is unstable; side 6 remains below baseline. Baseline
and alpha=1 retain q95 anisotropy tails above the predeclared 10 threshold,
with triangles exceeding both non-triangle strata in the named side-stratified
comparison. Saturation and cost remain descriptive finite-panel summaries.

These observations do not select a best generator, establish population
support or natural-law probabilities, imply a mechanism or target transfer,
or authorize post-hoc criterion invention. Four-dimensional exact/product
classification, target evaluation, and inferential uncertainty are deferred.
