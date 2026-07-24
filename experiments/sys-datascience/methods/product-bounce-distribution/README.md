# product-bounce-distribution

## Research Question

Among retained `random_product_sample` rows, how is producer-owned
`product_bounces` (2 versus 3) associated with the conditional `sys`
distribution after exact `(product_k, product_m)` bucketing?

This packet has two retained-data components. `analyze.py` is the original
prepared-table description and performs no capacity or geometry evaluations.
`class-minima.rs` reconstructs the already-retained product rows, runs the
existing f64 solved billiard candidate stream, and exactly certifies its
admissible two- and three-bounce class minima. It is not a generated-candidate
experiment or an exhaustive global enumeration. In particular, it does
**not** interpret forced cyclic orderings inside two same-type pair blocks as
independent capacity-value branches.

## Input Contract And Command

Rebuild the reviewed current-schema P2 prepared tables from the tracked
producer objects, rather than treating a surviving `/tmp` directory as input
authority:

```bash
TABLES_DIR="$(mktemp -d /tmp/sys-ds-product-bounce.XXXXXX)"
experiments/polytope-invariant-table/build-random-only-slice.sh full "$TABLES_DIR"
python3 experiments/sys-datascience/methods/product-bounce-distribution/analyze.py \
  --table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --out experiments/sys-datascience/methods/product-bounce-distribution
```

The rebuild contract and producer-object hashes are owned by
`../standard-baseline-p2/README.md`. The required derived-table identities are:

- `polytope-table.jsonl`: 14,336 rows, SHA-256
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`: 14,336 rows, SHA-256
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.

`analyze.py` checks the one-to-one `poly_id` join and writes the detailed
machine-readable `summary.json`. Quantiles are linearly interpolated from all
retained rows within each exact `(k,m)` bucket. Lower-tail membership is
`sys <= q10`; upper-tail membership is `sys >= q90` or `sys >= q95`.

The exact class-minimum artifact uses the raw retained producer input (not the
prepared table, which intentionally omits dual geometry):

```bash
mkdir -p experiments/sys-datascience/methods/product-bounce-distribution/artifacts
cargo run -p exp-sys-datascience --release \
  --bin sys-datascience-product-bounce-class-minima -- \
  --input experiments/polytope-datasets/random-product.jsonl \
  --output experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl
python3 experiments/sys-datascience/methods/product-bounce-distribution/summarize_class_minima.py \
  --input experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --out experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-summary.json
python3 experiments/sys-datascience/methods/product-bounce-distribution/write_class_minima_provenance.py \
  --input experiments/polytope-datasets/random-product.jsonl \
  --class-minima experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --summary experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-summary.json \
  --out experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-provenance.json
```

The bounded availability audit is retained in
`artifacts/class-minima-null-availability.jsonl`, with aggregate counts in
`artifacts/class-minima-availability-audit.json`. It selects the 785 rows
whose main artifact has a null A3, re-runs the existing transition-filtered
stream, and exactly certifies every f64-rejected three-bounce sigma. The audit
found 470 rows with no transition-feasible three-bounce sigma and 315 rows
whose generated three-bounce sigmas were all f64-inadmissible; no numerical
failures and no exact-admissible f64 rejections occurred. The audit is a
stream-contract check, not a proof of global mathematical infeasibility. The
bounded solver command and input/artifact hashes are recorded in the
provenance note; the retained checker above validates the result without a
routine rerun.

To reproduce the bounded audit itself (785 rows only):

```bash
cargo run -p exp-sys-datascience --release \
  --bin sys-datascience-product-bounce-null-audit -- \
  --input experiments/polytope-datasets/random-product.jsonl \
  --class-minima experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --output experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-null-availability.jsonl
```

Validate the retained audit without rerunning the expensive solver:

```bash
python3 experiments/sys-datascience/methods/product-bounce-distribution/check_class_minima_availability.py \
  --audit experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-availability-audit.json \
  --rows experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-null-availability.jsonl \
  --class-minima experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl
```

The runner parses the stored exact dual/vertex coordinates, reconstructs the
facet relations, runs the existing f64 solved billiard candidate stream, then
uses `CertifiedOrbitSetMode::MinimizersOnly` to exactly certify its admissible
candidates separately in each bounce class.
The detailed JSONL records every returned exact minimizer sigma, active-vertex
count, and the formula `(A3-A2)/A2`. A null `A_b` means no admissible solved
candidate was present in that f64 solved stream for that retained row; it is
not a zero or a numerical estimate. The generated summary retains the per-bucket
producer-bounce log-`sys` capacity/volume decomposition and the angular-gap
association only as descriptive observations. The portable provenance artifact
records repository-relative paths, full SHA-256 values, command, tool versions,
and source revision.

## Observation

The retained slice has 10,240 random-product rows: 3,979 two-bounce and 6,261
three-bounce rows. The pooled `sys` means are 0.23671 and 0.39763; medians are
0.19897 and 0.41372. The three-minus-two mean is positive in all 10 `(k,m)`
buckets (0.15811--0.20439), with a pooled within-bucket-standardized difference
of 0.94890 SD.

Within-bucket tail pooling gives 19.00% versus 4.38% bottom-decile membership,
1.33% versus 15.60% top-decile membership, and 0.60% versus 7.92% top-5%
membership (two versus three bounces). The same tail direction holds in all 10
buckets. Exact bucket counts, distribution quantiles, thresholds, and tail
memberships are recorded in `summary.json`.

## Ridge-Summary Check

One predeclared descriptive adjustment was used, not a model search: OLS of
within-`(k,m)`-standardized `sys` on the three-bounce indicator, then the same
regression additionally including within-bucket-standardized ridge normalized
entropy, ridge maximum share, and fraction of ridge areas at most `1e-2`.

The three-bounce coefficient falls from 0.94890 to 0.69283 SD (73.0% retained)
while `R^2` rises from 0.21393 to 0.59798. The selected ridge summaries explain
some covariance but do not account for the bounce association under this
minimal adjustment.

## Interpretation And Disposition

Allowed conclusion: among retained random-product rows, three-bounce producer
metadata has a strong descriptive association with low-end exclusion and
upper-tail enrichment of `sys` after exact `(k,m)` conditioning.

Not allowed: a candidate-proposer claim, causal or geometric mechanism claim,
independent capacity-branch multiplicity claim, or claim that the ridge
summaries mediate/explain the association. `product_bounces` is producer
metadata, and the ridge regression is observational.

This packet is usable as a constraint on future product-sampling or mechanism
questions. Reopen only for a separately pre-specified comparison design that
can support one of the prohibited stronger claims.

For the exact class-minimum audit, the allowed conclusion is narrower: it
describes the existing candidate API on these already evaluated rows, including
where a class has no admissible solved candidate. It does not establish a
generic two-versus-three-bounce theorem, a branch-takeover mechanism, or a
causal explanation for the producer-metadata association.
