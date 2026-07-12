# product-bounce-distribution review

Review date: 2026-07-12.

Verdict: Luna accepted this as a descriptive retained-random-product packet.

- A rerun on the reviewed P2 reconstruction was byte-identical for
  `summary.json`.
- The two required input hashes, 14,336-row counts, unique/identical `poly_id`
  join, 10,240-row retained slice, and 3,979/6,261 bounce-class counts were
  checked.
- Bucketed tail counts and direction consistency were checked: three-bounce
  rows are lower-tail depleted and upper-tail enriched in every exact `(k,m)`
  bucket.
- The one ridge adjustment is a transparent descriptive check, not a model
  search. Its partial attenuation cannot establish ridge mediation or a
  mechanism.
- The all-row class-minimum audit reconstructed every retained producer row.
  Where the stored producer capacity was classified as two or three bounces,
  its value agreed with the corresponding exactly certified class winner from
  the existing f64 solved candidate stream; the detailed identity check is in
  `artifacts/class-minima-summary.json`.
- `A_2` is present on all 10,240 rows. `A_3` is null on rows with no admissible
  three-bounce candidate returned by that f64 solved stream; null is an
  availability state, not zero, infinity, or a conclusion about an exhaustive
  global candidate space.

The packet may support only the descriptive association stated in `README.md`.
It may not be used as a proposer, causal, capacity-branch, or geometric
mechanism result.
