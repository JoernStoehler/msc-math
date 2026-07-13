# Self-review

## Checks performed

- `cargo check` and release build for the standalone manifest.
- Identity checks on both declared inputs: counts, one-to-one names, and SHA-256.
- 128-row evenly spaced smoke spanning all ten `(k,m)` buckets.
- Full run: 10,240 rows, 10,471 same-support pair records; fixed-sigma exact
  solves completed below the ten-minute stop rule.
- Exact stored-action reproduction for every primary pair.
- Exact cyclic-Q and action-ratio identities (the producer aborts on the first
  failure). During final interpretation, the lead separately checked all
  10,471 retained pair records and found that the exact term sum equals the
  exact stored `signed_gap_q` in every case. The producer does not currently
  hard-assert that term-sum equality, despite the earlier broader wording.
- Explicit within-word cyclic-rotation beta/Q invariance for every rotation of
  both words.
- Physical recovery thresholds (`max_violation`, closure, action agreement)
  checked for every primary pair; measured closure errors are retained and all
  retained primary pairs passed.
- Deterministic 100-permutation support shuffle and artifact hash generation.
- JSONL parsing/count/hash checks after generation.
- Row-level min/max/range summaries for tied pair gaps.
- Deterministic alignment fixture records six A2 and six A3 rotations (36
  joint candidates per pair), distinguishing symmetric alignment from a fixed
  one-sided cut.

## Findings and repairs

The first smoke exposed a sign/order alignment bug: the selected A2/A3
rotations were returned in reverse tuple positions, making the decomposition
the negative of `Q2-Q3`. The alignment tuple was corrected and the smoke then
passed. Summary generation needed explicit handling of null gaps and exact
rational beta-product strings. Review then found that support shuffles included
A3-null rows, the prior physical branch did not implement the specified geometry,
and closure errors were replaced by zero. The repair restricts shuffles to
complete rows, removes invalid physical fields entirely, and serializes the measured
closure error. A later exact-rank helper initially failed a Rust borrow check;
its pivot row is now cloned before elimination.

## Interpretation boundary

All primary retained pairs are one-term swaps in this generator; there is no
empirical two-term cancellation stratum to interpret. The optional physical
branch is omitted. Eight-facet rows are reported separately with exact KKT kernel
dimensions, all zero here; this falsifies the anticipated rank-deficiency/
nonunique-beta rival for the stored words. Their solver-selected beta is not
used as a primary margin.

The durable alignment convention is symmetric: every A2 and every A3 cyclic
rotation is considered, then candidates are ordered by flip count, gross term
sum, and lexical sigma order. Pair records are not independent rows; tied
representatives are retained pairwise. A factor-swap fixture was omitted since
constructing one cheaply would duplicate the geometry path; this omission is
explicit in `summary.json` and is safe for the exact algebraic claims because
the producer's identities are asserted directly.

No thesis text, shared source, capacity algorithm, or reviewed input artifact
was edited.

One non-claim-bearing schema limitation remains: row records retain the unused
fields `min_pairing_factor` and `min_cancellation_ratio` as null. The reviewed
component summaries read `normalized_pairing_factor` and
`cancellation_ratio` from pair records. A future consumer must not interpret
the row-level nulls as measured zeros or as absence of the one-term factor.

The term-sum equality is currently artifact-verified rather than protected by
a producer assertion. This is sufficient for the identified reviewed artifact,
but a future producer refresh should add the exact assertion before its output
inherits the same claim automatically.
