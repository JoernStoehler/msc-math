# Independent interpretation review

Date: 2026-07-12

Status: **ACCEPT** for bounded same-generator prospective-selection evidence.
No material repair is needed before interpretation. This review is independent
of the packet's technical/provenance review and uses only cheap recomputation
from the retained 1,436-row evaluation caches.

## Evidence reviewed

The source truth for this assessment is:

- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/pre-target/frozen-selected-candidates-before-sys.jsonl`;
- the two retained `evaluation/seed-*-sys-evaluation-cache.jsonl` files;
- `evaluation/covariance-rho-validation-verdict.json`;
- `evaluation/TECHNICAL_REVIEW.md`;
- the per-seed plans and target-field audit under `pre-target/`; and
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/analyze_covariance_rho_validation.py`.

`SHA256SUMS` in the packet root identifies these artifacts and the frozen
producer, configs, assembler, audit, and verdict reader.

## Validity and denominators

The manifest identity and freeze boundary pass. Its SHA-256 agrees with the
pre-target summary and generated verdict. The retained target-field audit
covers the two direct geometry-only feature inputs and the combined manifest.
The low-rho direction was chosen from earlier post-target discovery evidence,
so this is an independent holdout validation on fresh geometry, not a
theory-first discovery or a globally unselected hypothesis.

Cheap cache-level recomputation found exactly 1,436 unique manifest rows and
1,436 unique evaluation rows, with identical candidate-ID sets and no missing,
extra, or duplicate row. Candidate ID, polytope ID, bucket, arm memberships,
selection feature/direction/value, and rule values agree between the manifest
and evaluation caches. All retained `sys`, capacity, and timing values are
finite.

Each arm has 500 memberships, with 25 memberships in every one of the 20
seed-by-bucket strata. The rho and ridge arms overlap in 64 rows and the
control is disjoint, hence `500 + 500 + 500 - 64 = 1,436` unique evaluations.
Shared rho/ridge rows are evaluated once and correctly enter both arm means.
The rho-ridge comparison is a within-stratum arm-mean contrast, so this overlap
is handled as shared information rather than as independent target work.

The frozen plans record the required ridge comparator: bottom 1% by normalized
ridge-area sum followed by bottom 50% by maximum share within that tail. The
pre-target technical review verified the cascade against the full target-free
pool. The retained 1,436 rows preserve its arm labels and rule values but, by
themselves, cannot re-prove ranks against the omitted pool rows.

## Frozen decision and heterogeneity

The generated verdict owns the detailed metrics. Independent recomputation
reproduced its estimands and gates exactly:

- rho-control is `0.3331231475`, with frozen two-sided 95% interval
  `[0.3097480168, 0.3564982782]`;
- both seed aggregates are positive, all ten seed-pooled bucket effects are
  positive, and all 20 individual stratum effects are positive;
- the individual rho-control effects range from `0.259639` to `0.440029`, so
  every stratum also exceeds the predeclared `0.08` aggregate threshold;
- leave-one-bucket-out estimates remain between `0.328452` and `0.341256`;
- rho-ridge is `0.0144879956`, with interval
  `[-0.0149255563, 0.0439015475]`; and
- ridge-control is `0.3186351519`, with interval
  `[0.2792396706, 0.3580306332]`.

The primary success gate therefore passes robustly. The gatekept conclusion is
that rho is competitive with ridge under the frozen `-0.05` margin, not that it
is better than ridge.

The comparator has real bucket heterogeneity. Thirteen of 20 stratum contrasts
favor rho. The rho-ridge sign agrees across seeds for nine of ten buckets:
ridge is favored in both seeds for `3x4`, `3x5`, and `3x6`, while rho is
generally favored from `4x5` upward. This is a diagnostic observation only. It
does not authorize a post-hoc bucket subset or a revised selection rule.

The generated intervals use a Student-t summary of the 20 fixed
seed-by-bucket effects. The ten buckets are fixed and there are only two
producer seeds, so these are the frozen scoring intervals, not strongly
calibrated confidence intervals for arbitrary unseen seeds or distributions.
This qualification does not change the bounded success decision: the estimate
clears the effect threshold by a large margin and every individual stratum is
positive.

## Maxima, failures, and escalation

There are no missing or nonfinite target results and all bounce counts are two
or three. The unique global maximum is the ridge-only value
`0.9002763913`; the rho maximum is `0.9000949659`. Each selected arm has one
membership above `0.9`; no row exceeds `1` or the frozen previous generated
maximum `0.9509718381`. Independent `sys > 1` verification is therefore not
triggered.

The generated verdict reports counts above the previous maximum but does not
emit the numerical threshold. The frozen value is the reader constant in
`analyze_covariance_rho_validation.py`. Including that value in a future
regenerated verdict would improve self-containment, but this is not a validity
repair and does not justify patching the generated JSON.

## Exact claim boundary

Supported:

- prospective generated-candidate evidence that this exact bottom-0.5%
  low-canonical-vertex-covariance-rho rule enriches `sys` relative to the
  frozen disjoint control on the specified random-product height law; and
- under the predeclared gate, this rho rule is competitive with the frozen
  ridge cascade on that law.

Prohibited:

- rho beats ridge;
- low rho predicts, approaches, or provides a calibrated probability of
  `sys > 1`;
- transfer beyond the random-product generator with heights in `[0.8,1.2]`;
- an arbitrary-random-polytope claim or a capacity theorem;
- a causal or independent geometric-mechanism conclusion from the algebraic
  covariance decomposition;
- a direction flip, favorable-bucket subset, altered cutoff, or another
  post-target refinement presented as frozen validation; and
- describing the discovery-plus-validation process as globally preregistered.

## Belief and portfolio update

Rounded decision beliefs, not generated frequentist outputs:

- probability that an exact-design fresh replicate on this height law again
  gives positive low-rho enrichment: about `0.98`;
- probability that its equal-stratum mean enrichment exceeds `0.08`: above
  `0.99`;
- probability that low rho remains within `0.05` of the ridge cascade on this
  law: about `0.85`;
- probability that low rho is genuinely better than ridge overall: only about
  `0.65`, insufficient for that claim;
- transfer to another generator or height law: essentially no update;
- covariance as an independent mechanism or theorem: no update; and
- this selector alone as a promising `sys > 1` escalation: below `0.1`, given
  the retained maxima and failure to exceed the previous generated maximum.

The packet changes the global research ledger locally: generated-candidate
proposers are no longer supported only as explanatory in-table signals. This
exact low-rho proposer has prospective same-generator enrichment validation
and is a credible bounded demonstration or thesis-result candidate.

It does not change the top-level Viterbo portfolio. There is no counterexample,
stronger tail evidence, transfer result, mechanism result, or theorem. More
same-generator covariance repetitions, scale-up, cutoff search, or bucket
subset work has negative marginal value. Further covariance experiments are
worth reopening only for a named missing claim, most plausibly transfer to a
materially different generator, with a newly frozen rule and stopping
criterion. Current priority should be durable interpretation and
feature-completion, not further covariance search.

## Review coverage

This pass checked hash/freeze evidence, manifest-cache identity, membership and
denominator accounting, frozen estimands and gates, overlap handling, ridge
comparator semantics, cross-seed and bucket heterogeneity, maxima and failures,
claim strength, marginal empirical value, and portfolio effect. No additional
review agent was used because this document records the already independent
cache-level interpretation review.
