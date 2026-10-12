# Generated Candidate Proposers

Use / maintenance model: seed-level current-belief map for a future topic owner
or packet prompt. Keep it compact until a topic owner takes responsibility.
When packets add evidence, link the packet and update the current belief rather
than copying generated metrics here.

Scope: methods that propose or filter unevaluated candidate polytopes before
`sys` is computed, including rejection-based scalar filters, multi-feature
filters, and feature-first generated-candidate pipelines.

Status block:

- topic-status: two bounded same-generator pool-screening rules reviewed
- spawn-status: parked for empirical execution
- next-role: demonstration/readiness owner
- next-action: assign distinct source roles to ridge and covariance-rho, if
  both are used
- review-gate: no proposer overclaim; generated candidates must be selected
  before `sys`; normal review before thesis use
- belief-update-owner: generated-candidate topic owner; research-map steward for
  cross-topic propagation
- last-reviewed: 2026-07-12 covariance technical and interpretation reviews
- source-of-truth: `../../methods/extreme-scalar-rejection-proposer/`
- stale-if: runner configs/artifact paths change, new generated-candidate
  evidence lands, or thesis wording needs proposer claims
- allowed-downstream-use: evidence for exact frozen same-generator,
  sub-threshold pool-screening rules; not threshold-directed evidence

Current belief: the reviewed ridge cascade and frozen low canonical
vertex-covariance-rho rule are two distinct same-generator pool-screening rules
with prospective sub-threshold enrichment evidence. Rho is competitive with,
not better than, ridge; neither reached `sys > 1`, establishes mechanism, or
transfers beyond its named random-product height law.

Owner-readiness/status: ready for bounded demonstration/readiness, with exact
terminology still a later Jörn choice. Sources:
`../../methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/`
and `../../methods/extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/`.

Current implementation boundary: the runner supports scalar-rule unions and a
typed two-stage per-bucket cascade with a stage-1 comparator. The cascade
interface records both frozen steps and rejects conflicting scalar-selection
configuration.

Evidence sources:

- `../../methods/extreme-scalar-rejection-proposer/README.md`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/README.md`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/role-summary.tsv`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/README.md`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selection-summary.tsv`
- `../../methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/review.md`
- `../../methods/extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/`
- `experiments/polytope-invariant-table/feature_cost.rs`, infrastructure for measuring feature cost

Candidate hypotheses:

- Ridge and low-rho selection may be competing descriptions of the same
  generator-local favorable geometry; the current comparison cannot decide.
- A lower scalar objective need not be a useful optimizer objective: the ridge
  endpoint paths reverse it locally, without proving a general Goodhart law.
- Product bucket matching is essential; pooled proposer evidence mostly
  measures bucket/combinatorial structure.

Current boundary decision:

- Do not launch a rescue, cutoff search, same-generator repetition, or scale-up
  merely to improve the story.
- Reopen only for a named transfer or another missing claim, with a frozen rule,
  target-free selection, stopping criterion, and independent review plan.

Adjacent topics to read:

- `geometric-feature-mechanisms.md`, for mechanism hypotheses behind ridge
  features.
- `tail-rare-event-modeling.md`, for rare-event scale-up caveats.
- `supervised-and-representation-methods.md`, for in-table methods that may
  suggest additional generated-candidate filters.

Cheap discriminators:

- Within selected low-ridge candidates, run association/rule mining against
  selected `sys` values only.
- Sweep all cheap ridge-area scalar summaries in the generated-candidate
  feature cache, but keep sys calls to selected unions.
- Compare per-bucket thresholds rather than pooled thresholds.

Ready packet prompts:

- Workflow-test 10k promising-scalars executor demo.
  Objective: test whether a fresh packet executor can run the staged generated
  proposer pipeline, preserve selection-before-`sys`, produce the expected
  artifacts, run the target-field audit, and keep observations quarantined. This
  is not thesis evidence. Use a `/tmp` run root such as
  `/tmp/sys-ds-workflow-test/generated-candidate-proposers/10k-promising-scalars/`.
  Save the executor prompt as `prompt.md` in the run root. Start from the
  durable 100k config shape and lower `candidates_per_bucket` to `1000`, which
  gives 10,000 candidates across the runner's 10 product buckets. For the known
  10k command shape, expect about 165 selected-or-baseline rows; treat a much
  larger union as a stop-and-review condition. Put `command-output.txt` in the
  artifact directory and `target-field-audit.txt` in the run root unless a
  packet prompt says otherwise. Bucket-matched interpretation for this
  workflow-test should use only generated runner summaries such as
  `selection-summary.tsv`; do not add new analysis unless reviewing the
  workflow requires it.
- Bucket-specific scalar threshold curve packet. Objective: vary per-bucket
  selection strength for the same cheap scalar family and report whether
  enrichment collapses smoothly or sharply. This can reuse the same runner if
  configs express the threshold grid.

Needs topic-owner sharpening:

- Two-feature selected-tail rescue packet. Missing choices: which selected-tail
  source run to mine, which second-feature family is allowed, how to avoid
  overfitting rules to already evaluated candidates, what counts as rescue, and
  whether the output is a new proposer config or only an explanatory diagnostic.
  Reasonable defaults for a topic owner to accept or replace: use the latest
  reviewed generated-candidate selected-tail artifact as source; restrict second
  features to cheap scalar fields already in `CandidateFeatureRow`; define
  rescue as a frozen second-feature split that improves selected mean/max or
  selected-above-baseline-p95 within buckets without collapsing to one bucket;
  produce a diagnostic first, not a deployable conjunction proposer.
  A 2026-07-04 topic-owner read recommends narrowing further before execution:
  choose at most one base ridge-magnitude feature, one second
  concentration/distribution feature, one direction, and one quantile rule;
  require leave-one-product-bucket-out or deterministic candidate-id split in
  the diagnostic phase; validate only on an independent seed after selection is
  frozen before `sys`. Stop if the rule works in only one or two buckets,
  restates the first ridge feature, stays below about `sys = 0.90` on 100k
  validation, or fails to improve over the scalar-only per-bucket boundary in
  most buckets. Escalate immediately on any independent `sys > 1` row.

Opportunity-cost notes: do not launch larger single-feature ridge-sum runs
unless a concrete thesis sentence needs sharper statistics.
