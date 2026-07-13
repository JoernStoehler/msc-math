# Product-bounce class degeneration

## Question and scope

This packet asks whether the small exact two-/three-bounce class gap in the
retained random planar products is associated with a common six-facet support
whose cyclic order changes. It consumes the reviewed raw product stream and
`product-bounce-distribution` class-minima artifact; it does not generate
polygons, search new capacity candidates, reopen A3 nulls, or edit thesis
text. Results are diagnostic evidence for the named retained generator, not a
generic billiard theorem or a `sys` mechanism.

The Rust producer re-solves every retained A2/A3 minimizer sigma with the
exact rational KKT solver, checks stored action and cyclic-invariance
identities, aligns all rotations of both words deterministically, and writes
pair- and row-level JSONL. The pair artifact records every same-support
six-facet pair, exact order-swap terms, beta factors, normalized symplectic
factors, and recovery diagnostics. Pair multiplicity is intentional: tied
minimizer representatives are retained pairwise, and the symmetric convention
rotates A2 and A3 independently before choosing the lexicographically simplest
decomposition. Eight-facet A3 minima are retained as a separate combinatorial
control and are excluded from the six-facet order population. `summarize.py`
writes deterministic bucket summaries and 100 within-bucket support shuffles
(seed `20260713`) over complete rows only.

## Reproduction

From the repository root:

```bash
cargo build --release --manifest-path experiments/sys-datascience/methods/product-bounce-class-degeneration/Cargo.toml
cargo run --release --manifest-path experiments/sys-datascience/methods/product-bounce-class-degeneration/Cargo.toml -- \
  --raw experiments/sys-datascience/produce/random-product.jsonl \
  --class experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --out experiments/sys-datascience/methods/product-bounce-class-degeneration/artifacts
python3 experiments/sys-datascience/methods/product-bounce-class-degeneration/summarize.py \
  --rows experiments/sys-datascience/methods/product-bounce-class-degeneration/artifacts/degeneration-rows.jsonl \
  --pairs experiments/sys-datascience/methods/product-bounce-class-degeneration/artifacts/degeneration-pairs.jsonl \
  --out experiments/sys-datascience/methods/product-bounce-class-degeneration/artifacts/summary.json
```

The producer verifies 10,240 raw rows and class rows, one-to-one names, 9,455
complete rows, 785 A3-null rows, and the declared input SHA-256 values before
computing. A 128-row smoke uses evenly spaced source indices, hence spans all
ten `(k,m)` buckets; its output is disposable.

## Current packet result

The research lead's final scientific interpretation and disposition are in
`interpretation.md`; generated metrics remain sourced from
`artifacts/summary.json`.

The generated `artifacts/summary.json` is the metric source. It contains 10,240
row records, 9,455 complete rows, 3,559 rows with at least one equal-support
pair, and 10,471 pair records. Equal-support frequency is 37.6% over complete
rows, 49.3% for `|g|<=0.01` (55.1% for `|g|<=0.001`); the 100 deterministic
within-bucket shuffles over the same complete-row population have mean 12.6%
and 95th percentile about 13.0%. Every retained same-support pair selected one
nonzero order-swap term under the symmetric alignment convention. Exact
recovery passed for all 10,471 pairs, with the measured closure error retained.
The beta-product and normalized pairing factor are retained as exact factors
and descriptive associations; the packet does not attribute the gap to either
factor. No two-term rows exist in this retained primary set, so
cancellation is not measured there.

Disposition: shared-support cyclic-order competition is enriched near ties in a
substantial generator-local subset, and the exact one-swap term is the measured
factorization object. Branch E remains for the majority different-support
population. Branch C is not supported in this packet. The optional physical
branch is omitted because no valid factor-vertex/edge-endpoint measurement is
retained. These are descriptive associations, not causal or universal claims.

## Artifacts and reopen rule

- `artifacts/degeneration-pairs.jsonl`: exact pair/order decomposition and
  measured recovery errors.
- `artifacts/degeneration-rows.jsonl`: row identities, tie ranges, support and
  control strata, recovery and eight-facet fields.
- `artifacts/summary.json`: deterministic summaries and shuffle controls.
- `artifacts/provenance.json`: input/output hashes, command, and source revision.

The eight-facet KKT kernel dimensions are all zero in the stored stratum; this
falsifies the anticipated rank-deficiency/nonunique-beta rival for these words.
The stratum remains separate only because its eight-facet combinatorics differ.

Reopen only for a separately specified comparison or physical-geometry study
that preserves the exact action identities and the six-facet/eight-facet
separation. A factor-swap fixture was omitted because no cheap standalone
factor relabeling path was available without duplicating geometry construction.
