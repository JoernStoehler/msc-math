# Independent review: stratified ridge/sys tail empirics

## Verdict: repair before retaining as an owner-local research packet

The packet is computationally sound for its descriptive and bounded smoke-use
purpose, but two statements overread resampling from one run as evidence about
selection pressure, and the retained provenance is incomplete for a durable
handoff.  After the repairs below, it is suitable to retain as an
owner-local packet and to motivate (but not justify a positive result from) an
8-target endpoint-path smoke.

## Checks passed

- `python3 analyze.py --out-dir /tmp/ridge-empirics-review-rerun` completed
  successfully and produced outputs byte-identical to the packet TSVs and
  `metadata.json`.  It streamed the stated 1,000,000-row feature table and
  checked all 466 evaluated IDs and their `(k,m,ridge-sum)` fields.
- The retained table has 14,336 rows; its 10,240
  `random_product_sample` rows join one-to-one to the 10,240 tracked product
  rows on unique serialized `sys`, giving ten 1,024-row `(k,m)` strata.  This
  is an exact recovery within these inputs.  It is appropriately not promoted
  to a general identity-key contract.
- There are no ridge-sum or `sys` ties at any of the four retained cutoff
  ranks, so stable sort order does not affect the reported same-q sets.
- The selection labels agree with the frozen plans: the 1M source is 100,000
  candidates per bucket with pre-target low-sum top-10 selection (`q=10^-4`),
  and the 100k source is 10,000 per bucket with its stage-1 low-sum 100-row
  selection (`q=.01`).  Each comparison arm has its stated ten rows (1M) or
  100 rows (100k) per bucket, and selected and matched-baseline IDs are
  disjoint.
- Pooled same-q arithmetic is correct.  With ten equal 1,024-row strata and
  `t=ceil(q*1024)`, the null expected overlap is
  `sum_b t^2/1024`; for `q=.01`, this is `10*11^2/1024 = 1.181640625`, so
  `15/1.181640625 = 12.6942x`.  This is the correct stratified null for the
  literal within-stratum low-ridge and high-`sys` rank sets, not the unstratified
  `110^2/10240` null.
- The q=.01 bucket overlaps are exactly `0,4,6,4,0,1,0,0,0,0` in the table's
  bucket order.  Calling the pooled result heterogeneous is a fair descriptive
  warning: 14 of 15 overlaps come from three `k=3` buckets.  It is not a test
  of a common mechanism.
- The packet correctly separates post-target retained association from
  pre-target selection, does not derive a population tail rate from 466
  evaluated 1M rows, and makes no `sys > 1` claim.

## Required repairs

1. Correct the bootstrap interpretation in `REPORT.md`.

   The 1M selected-minus-baseline interval is made by resampling the ten
   already-computed bucket mean differences, not target rows.  It is therefore
   a descriptive bootstrap for an equal-weight aggregate over the observed
   bucket effects (and reflects their between-bucket spread); it does **not**
   quantify finite evaluated-row variation conditional on the run.  The pooled
   comparison interval has the same issue.  The within-bucket two-run
   intervals do resample target values, but only within already selected,
   different generated runs.  Replace the current blanket sentence
   "Resampling intervals cover finite evaluated-row variation conditional on
   the generated runs" with distinct descriptions of those two bootstrap
   targets, or omit the first/pool intervals if they are not needed.

2. Narrow the q=.01 versus q=.0001 inference.

   The reported signs are real for these two inputs: four `k=3` contrasts are
   positive and all six `k>=4` contrasts are negative; the within-bucket
   resampling signs also have the stated directions (except 3x5 crosses zero).
   But this is one independent generated run at each q, with different seeds,
   populations, and selected order statistics.  It cannot *discriminate
   against* a universal monotone selection-pressure curve, nor establish a
   `k=3`/`k>=4` reversal or saturation.  It is only a run-specific,
   hypothesis-generating sign pattern compatible with heterogeneity.  Change
   the paragraph beginning "Thus this evidence discriminates" accordingly.
   A common-q, multi-seed design (or repeated runs at both q levels) is needed
   for that discrimination.

3. Add selection-plan provenance to `metadata.json` and the report's input
   list.

   The hashes currently identify the feature/evaluation tables but not the
   frozen `selection-plan.json` files that establish the pre-target rule,
   candidate counts, seed, rounding, and baseline policy.  Record paths and
   hashes for `/tmp/sys-ds-extreme-scalar-rejection-proposer-1m-ridge-sum/selection-plan.json`
   and
   `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/selection-plan.json`.
   The 1M selected-before-target cache/hash would be a useful additional
   identity link, though its plan hash is the material minimum.

## Downstream-use boundary

After repair, the packet supports this narrow decision: low ridge-sum has a
strong, stratified descriptive association in the retained data and a
pre-target selected-versus-matched-baseline lift in the one 1M run; these make
an 8-target endpoint-path smoke a reasonable plumbing/feasibility probe.
The smoke must remain explicitly non-confirmatory: it cannot be advertised as
validating the pooled association, a universal low-ridge rule, a
selection-pressure curve, endpoint exceedance, or a `sys > 1` probability.

Review scope: source/provenance, joins, selection semantics, q definitions,
overlap/enrichment arithmetic, bootstrap targets, and cross-run interpretation.
No repository or packet analysis output was modified; this file is the review
record.

## Final repair check: accepted for the named owner-local use

Rechecked after repair.  All three required repairs are satisfied:

- `REPORT.md` now correctly distinguishes the retained re-ranking bootstrap,
  the bucket-effect/contrast bootstraps, and the per-bucket target-value
  resampling; it no longer calls the first two target-row sampling intervals.
- The q=.01 versus q=.0001 text now treats the sign split as run-specific and
  hypothesis-generating, explicitly rejects an inference of monotonicity,
  reversal, or saturation, and names the repeated common-q design needed for
  discrimination.
- `metadata.json` contains both frozen selection-plan paths and SHA-256 hashes,
  which match the current files, plus the 1M selected-before-target cache hash.

Acceptance is limited to retention as an owner-local research packet and to
motivating a clearly non-confirmatory 8-target endpoint-path smoke under the
boundary stated above.  It is not accepted as evidence for an endpoint claim,
`sys > 1` probability, or selection-pressure mechanism.
