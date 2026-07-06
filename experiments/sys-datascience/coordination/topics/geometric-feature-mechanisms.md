# Geometric Feature Mechanisms

Use / maintenance model: seed-level topic map for mathematical and geometric
idea generation. Optimize for hypotheses, discriminators, and evidence links,
not for a polished explanatory essay. A topic owner may split off proof notes
or experiment packets when a mechanism becomes concrete.

Scope: mathematical and geometric interpretation of invariant scalar features,
especially ridge symplectic two-face area summaries, combinatorial structure,
and how these might relate to `sys`.

Status block:

- topic-status: promoted diagnostic available; mechanism theorem absent
- spawn-status: reopen if current milestone needs mechanism wording or a frozen
  concentration-rule validation plan
- next-role: topic owner if reopened
- next-action: synthesize thesis-safe ridge mechanism wording or freeze one
  concentration-rule validation plan
- review-gate: diagnostic-only unless independently validated; no proposer
  claim from post-`sys` splits
- belief-update-owner: mechanism topic owner; research-map steward for
  cross-topic propagation
- last-reviewed: 2026-07-04 topic-owner update; 2026-07-06 workflow hardening
  pass added status metadata only
- source-of-truth: `../../methods/ridge-mechanism-discriminator/` plus feature
  code listed below
- stale-if: ridge feature definitions, discriminator artifacts, HKO reference
  evidence, or thesis mechanism wording change
- allowed-downstream-use: empirical mechanism diagnostics and prompt seed; not a
  theorem or validated candidate proposer

Current belief: ridge-area summaries are empirically important, but the project
does not yet have a mechanism theorem. Current claims should stay empirical
unless a mathematical argument is developed.

Owner-readiness/status: topic seed, ready for a topic research lead if Jörn
wants a mechanism-focused session.

2026-07-04 topic-owner update: a mechanism pass recommended a first execution
packet, the Ridge Mechanism Discriminator Table, before more mechanism prose or
scalar-proposer rescue. The packet should use existing retained and generated
artifacts to classify whether low ridge area is best read as magnitude signal,
concentration/distribution signal, product/combinatorial proxy, extreme-tail
Goodharting, or small-area-fraction story. It should not make a new proposer
claim.

2026-07-04 promoted diagnostic status:
`../../methods/ridge-mechanism-discriminator/` contains a compact
discriminator-table snapshot. It reruns from its script when the local
tail-rule artifacts and full 100k feature cache are present, and it tracks the
compact output tables for ordinary interpretation. Result: low ridge magnitude
still looks like a bucket-local empirical signal; pure product/combinatorial
proxy is too strong; concentration/distribution features are plausible
diagnostic seeds; small-area fractions look weak; extreme-tail Goodharting
remains a caution. Use this as diagnostic/mechanism evidence only, not
as a generated-candidate proposer validation.

Mechanism claims must name a measured object and a falsifier. Do not write
causal or trajectory-constraining prose unless the claim says what measured
quantity it refers to and what observation, proof check, or experiment would
weaken it.

Evidence sources:

- `../../prepare/invariant_features.rs`
- `../../prepare/features_face_symplectic.rs`
- `../../methods/statistical-associations/`
- `../../methods/extreme-scalar-rejection-proposer/`
- `../../methods/ridge-mechanism-discriminator/`
- `../../methods/hko-reference-coverage/`

Current ridge observations:

- Retained-table scalar association screens and generated-candidate proposer
  artifacts both point to volume-normalized symplectic two-face area summaries
  as high-`sys` enrichment features.
- The 1M generated-candidate per-bucket low-ridge run supports enrichment
  within product buckets, but does not show monotone extrapolation toward
  `sys > 1`.
- Pooled/global ridge conclusions are unsafe without product-bucket or
  combinatorial controls.
- HKO ridge-rank evidence may be useful for mechanism hypotheses, but parked
  HKO-distance/flank claims remain tainted until normalization is repaired.

Feature taxonomy:

| Feature family | Measured object | Main use | Main caveat |
| --- | --- | --- | --- |
| ridge magnitude summaries | sum/mean/std/quantiles of symplectic area over primal two-faces, often volume-normalized | candidate high-`sys` enrichment and mechanism seeds | may proxy product bucket or combinatorics |
| ridge concentration summaries | max share, top-3 share, entropy, effective face count | distinguish low-ridge selected candidates | not yet a validated proposer |
| count/combinatorial controls | facets, vertices, edges, ridges, incidence summaries | test whether ridge features add signal beyond shape size/type | can absorb real geometry if controls are too broad |
| provenance/bucket controls | source, product bucket, height range | prevent pooled artifacts | does not prove mechanism inside bucket |

Adjacent topics to read:

- `generated-candidate-proposers.md`, for the strongest current ridge-filter
  observations.
- `hko-reference-and-local-geometry.md`, for HKO-local ridge questions.
- `method-surface-expansion.md`, for how mechanism claims affect thesis method
  closure.

Candidate hypotheses:

- Low total two-face symplectic area marks a globally favorable geometry.
  Measured object: low volume-normalized sum of symplectic polygon areas over
  primal two-faces. Explains: retained-table association and generated-candidate
  low-ridge enrichment. Predicts: ridge residuals stay useful inside fixed
  buckets after combinatorial controls. Falsified by: enrichment collapsing
  under bucket/combinatorial controls.
- Ridge-area magnitude is mostly a product-bucket/combinatorial proxy.
  Measured object: ridge magnitude summaries compared against facet/ridge
  counts, incidence summaries, and product bucket. Explains: unsafe pooled
  behavior. Predicts: residual ridge-area rules add little after controls.
  Falsified by: strong residual enrichment inside fixed buckets.
- High `sys` prefers distributed small symplectic area rather than a few
  collapsed faces. Measured object: entropy/effective-face-count and
  max/top-3-share concentration summaries. Explains: possible variation inside
  low-ridge selected candidates. Predicts: concentration features split high
  selected rows from mediocre selected rows. Falsified by: no within-selected
  association after bucket control.
- Extreme low-ridge filtering Goodharts. Measured object: selected mean/max
  `sys` as the low-ridge threshold becomes more extreme. Explains: 1M
  enrichment without near-counterexample behavior. Predicts: threshold curves
  flatten or degrade in the extreme tail. Falsified by: monotone improvement
  with stronger filtering.
- HKO shares the ridge-area mechanism but not the current random-product support
  route. Measured object: HKO invariant-feature ranks and corrected HKO-local
  perturbation features. Explains: possible HKO/ridge relation. Predicts:
  HKO-local perturbations preserve unusual ridge magnitude near high `sys`.
  Falsified by: corrected HKO-local runs showing no stable ridge extremality.

Cheap discriminators:

- Compare ridge-area summaries against purely combinatorial controls within
  bucket.
- Inspect selected low-ridge candidates geometrically: face/ridge patterns,
  concentration, and degeneracy.
- Ask what ridge-area hypotheses predict for HKO perturbations and retained
  high-tail rows.

Ready packet prompts:

- Ridge mechanism discriminator table. `promoted-diagnostic`.
  Objective: use existing retained table artifacts and existing
  generated-candidate selected/evaluated rows to classify ridge hypotheses.
  Minimum output: per product bucket and generic facet bucket summaries for low
  sum/mean/max/std/q95; concentration add-on table for
  entropy/effective-count/top-share conditional on low magnitude; compact
  residual/control classification `survives`, `weakens`, `collapses`, or
  `ambiguous`; overlap matrix for 100k `promising-scalars` selected rows by
  scalar rule; explicit "no new proposer claim" boundary. Stop if all ridge
  effects collapse under controls and no concentration feature adds value; if
  one concentration add-on survives held-out or bucket splits, freeze exactly
  one rule for possible later validation instead of browsing more features.
  Current next action: use
  `../../methods/ridge-mechanism-discriminator/artifacts/current/hypothesis_rollup.tsv`
  and the method README as the compact source for mechanism follow-up design.
  Do not launch independent concentration-rule validation until one frozen rule
  and selection-before-`sys` validation plan are written.
- Ridge feature mechanism note. `topic-owner-ready`. Observations, possible explanations, and
  falsifying predictions. First deliverable: a compact note that lists the
  current ridge-area observations, at least three natural mechanism hypotheses,
  what each predicts outside the already-observed scalar association, and the
  cheapest experiment or proof check that would distinguish them. It should not
  claim a theorem unless it gives a proof route.
- Ridge residual control audit. `audit-ready`.
  Objective: decide whether ridge magnitude is independent signal or mainly a
  bucket/combinatorial proxy. Output should classify the main ridge magnitude
  features as survives, weakens, or collapses under source/facet/provenance and
  combinatorial controls.
- Low-ridge selected-tail split. `executor-ready after selected-tail source is named`.
  Objective: test whether ridge concentration summaries separate high selected
  low-ridge candidates from mediocre selected candidates. Output should be a
  per-bucket diagnostic table and frozen-rule recommendation, labeled
  diagnostic-only unless evaluated on independent generated candidates.
- Threshold-curve Goodhart check. `executor-ready after config scale is chosen`.
  Objective: distinguish monotone low-ridge improvement from extreme-tail
  Goodharting by varying low-ridge selection strength with target-field audit
  and bucket-matched summaries.
- HKO ridge-rank repair/review. `reviewer-ready`.
  Objective: decide whether HKO ridge extremality is reliable enough to inform
  mechanism hypotheses. Exclude parked HKO-distance/flank claims until
  normalization is repaired.

Needs topic-owner sharpening:

- Low-ridge candidate geometry inspection packet.
- Combinatorial-control packet for ridge-area associations.

Opportunity-cost notes: high thesis value if it produces clean explanatory
language or conjectures; low value if it becomes unfalsifiable prose.
