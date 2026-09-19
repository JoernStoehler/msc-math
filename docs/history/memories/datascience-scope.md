# Data-science scope: evidence questions before new research

Source-reading orientation, 2026-09-07, baseline `921f3c02`; bounded retained-data
checks added 2026-09-11 at `2e36e9ca`.
This separates candidate work by what it would accomplish; it does not decide
what Jörn should accept or certify the retained results. No new experiments were run. September 11 checks compare reported aggregates
with retained records; they do not validate producer correctness or capacity.

## Scope already decided

Jörn calls data science moderately incomplete but has not selected which gaps
to finish versus report. Additional branch-aware versus nonsmooth optimizer
comparison is not interesting enough to delay submission even an hour. These
are user decisions recorded in [current work](todos.md), not conclusions from
the method ledgers.

The active [chapter](../thesis/08-black-box-datascience.tex) presents three
different contributions: a retained finite-table negative search, a frozen
generated-candidate scalar-filter test, and a seven-implementation historical
optimizer comparison. Its selected-body local screen is another bounded
observation. None requires establishing a generally successful search method
to be reported honestly. Whether this account is enough for the thesis remains
a scope choice, separate from whether its particular claims are supported.

## Candidate retained-claim checks

**Generated-proposer numerical qualification: wording repaired.** The
chapter's generated-candidate paragraph and the appendix's “Generated-candidate
scalar proposer” now identify the 1675 targets as historical evaluator values,
not current production capacity certificates. The
[scalar-filter README](../experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md)
explicitly identifies these frozen accepted targets as `evaluated-target.v2`,
not current v3 production certificates. Previously the chapter's heuristic-value
qualification referred only to the optimizer comparison. This is a source-based
wording repair, not a numerical recheck or evidence that a value is wrong.

**Retained-table evaluator lineage: partially unresolved.** September 11
read-only inspection counted 14,336 provenance rows in
`experiments/polytope-invariant-table/polytope-provenance-table.jsonl`: 4,096
`random_sample` rows labeled `ehz_capacity` and 10,240 `random_product_sample`
rows labeled `ehz_capacity_billiard`. Current `load_caches.rs` inserts these
labels; they do not establish historical solver or certification semantics.
The rows contain neither producer revision nor capacity-method/certificate
fields. Scalar tables were absent at inspected local paths. The trusted-filter
summary records maximum 0.86258589584944 and zero values above one, but expressly
does not validate capacity/volume. Current certified-versus-legacy contracts in
the producer README cannot establish the frozen table's historical contract.
Recover actual scalar/producer payload and matching provenance to narrow this
further; no sample recomputation is implied.

**Appendix diagnostic aggregates checked against retained records.** A bounded
subagent recomputed these on September 11 for “Detailed HKO calibration” and
“Selected finite-step failure audit” in `thesis/a-datascience-results.tex`:

- 16 HKO perturbations: gap recovery 0.9339957463–0.9999999922 and distance
  reduction 0.7788465885–0.9999744166; single-branch recovery
  −0.2211149374–0.4141014923. HKO control has zero accepted steps/gain.
- Five of eight endpoints improve, with stated gain range 1.57e−6–4.61e−5.
  Recounted 80 proposals, 52 losses with represented winner and positive
  prediction, and 40 with determinate unchanged geometry.
- KKT ratios 217.8466, 67.4055, 1.526416 round to the manuscript's values.
  All three recorded geometry comparisons agree at all 39 points; maximum
  relative volume discrepancy is 8.4091e−16.

Evidence: under `experiments/dev-gradient-ascent/`, the
`ascent-continuation/artifacts/hko-one-step-development-panel-20260729/` raw
summary and analysis CSVs, `ascent-continuation/artifacts/`'s
`top8-tuning-endpoints-one-step-20260729/raw/candidates.jsonl`, and
`endpoint-model-audit/artifacts/directional-decomposition-20260729/raw/audit.json`.
These checks support the aggregates, not producer correctness or causal claims.

**Editorial qualifications integrated September 11.** Finite differences at tested
radius 1e−8 approached the implemented derivative; finite samples do not establish
asymptotic convergence. The top failure still has about 0.99% action and 1.88%
branch-ratio relative errors. The appendix now states agreement at
the tested scale and these limits. For the frozen table, Its table statistics are now consistently described as recorded values,
matching the abstract/conclusion; filtering establishes membership, not
numerical certification. No evidence here establishes that the maximum is wrong.
`./thesis/check-build.sh` passed after these edits, refreshing the current PDF.
This checks PDF production, selected overfull boxes and references. A read-only
visual check of rendered PDF page 104 (both changed passages) found intact math
glyphs and no clipping, overlap or spacing defects. This is not full-manuscript
visual or scientific review.

## Already disclosed or resolved, not fresh research blockers

- Optimizer selection/tuning independence, heuristic evaluator and start-new-work
  time cutoff are already disclosed in the chapter and appendix; the existing
  optimizer pilot preserves their support boundary. More comparative research
  is not needed merely to restate this frozen experiment.
- The local screen's missing dirty producer state is disclosed in the chapter.
  Exact replay remains a support limitation, not an undiscovered task to rerun
  generic probes. Whether retaining the bounded observation is worthwhile is
  an editorial/evidence decision.
- [P2](../experiments/sys-datascience/methods/standard-baseline-p2/README.md)
  records the completed current-schema input rebuild and matching hashes. The
  downloaded prepared table lacks six active columns, so its documented
  rebuild route matters; this is not a reason to repeat the expensive check.

The older [question map](../experiments/sys-datascience/coordination/current-question-map.md)
contains broader generator, representation, mechanism, endpoint and rarity
questions. Its own opening says it is not fully reconciled with later packets.
Those questions describe possible new scientific results, not missing support
for every bounded claim above. Choosing more plots, methods or distributions
before identifying a reader-facing claim would reopen scope rather than close
an established evidence gap.
