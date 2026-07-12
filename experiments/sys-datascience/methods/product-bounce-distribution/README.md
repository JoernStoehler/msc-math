# product-bounce-distribution

## Research Question

Among retained `random_product_sample` rows, how is producer-owned
`product_bounces` (2 versus 3) associated with the conditional `sys`
distribution after exact `(product_k, product_m)` bucketing?

This is an existing-data descriptive packet. It performs no capacity or
geometry evaluations. In particular, it does **not** interpret the four cyclic
orderings inspected in low product rows as four capacity-value branches: they
are forced orderings inside two same-type pair blocks.

## Input Contract And Command

Rebuild the reviewed current-schema P2 prepared tables from the tracked
producer objects, rather than treating a surviving `/tmp` directory as input
authority:

```bash
TABLES_DIR="$(mktemp -d /tmp/sys-ds-product-bounce.XXXXXX)"
experiments/sys-datascience/prepare/build-random-only-slice.sh full "$TABLES_DIR"
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
