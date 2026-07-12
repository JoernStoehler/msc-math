# AI In The Research Process: Content Companion

Status: section-local evidence and maintenance companion for
`thesis/13-use-of-ai.tex`. Not source truth and not thesis text.

Purpose: keep the publication-facing claims tied to their evidential roles. The
separate factual declaration is owned by `thesis/ai-use-disclosure.tex` and
`thesis/ai-use-disclosure-content.md`.

Overruled by: project artifacts, accepted Jörn/Kai decisions, and final thesis
review. Raw session text and row-level derived data remain private and must not
be committed.

## Evidence design

The section uses selected complete episodes plus one controlled replay. It does
not estimate net productivity or reconstruct an exhaustive research timeline.
Recorded sessions establish observed requests, actions, reviews, and handoffs;
Git establishes retained artifact states and branch integration; surviving
tests and code establish only what they directly check. Jörn's retrospective
account supplies subjective and offline context and is identified as such.
Episode output, later artifact evidence, and final integration are assessed
separately and are not attributed to the same agent episode unless recorded
lineage establishes that link.

The investigation stopped when the four reader-facing questions below had
enough evidence to decide whether they belonged in the section. Broader event
aggregation would not presently change a thesis-facing conclusion enough to
justify its cost.

## Retained claims and sources

### Earlier inspectable artifacts

Claim: with an explicit executable contract and relevant infrastructure,
delegation can produce an inspectable artifact early and move experimentation
earlier. This is not a claim of trustworthy completion or causal labor saving.

Sources:

- HK2017 implementation: Jörn's 2026-07-12 account records the early MATLAB-to-
  Rust translation, approximate speed relation, wrong results, poor code, and
  missing pruning. The later repository history establishes extensive repair
  but does not time a human-only counterfactual.
- Crosspolytope episode on 2026-02-24: recorded session/tool chronology gives a
  running binary and correct volume smoke test after about eight minutes and
  initial commit `d0985fcd` after about nine. Commit `9c592c47` adds the longer
  search machinery; merge `da5049f2` records the same-day retained result; later
  phase work appears in `b567adca`.

### Production and verification

Claim: tests and reviews help only when they discriminate the relevant error;
separating verifier construction from production remained useful in the tested
GPT-5.5/GPT-5.6 comparison.

Source: controlled four-run replay from base `f3d36cc9`, before the historical
projection reduced-gradient sign repair. All runs received the same known issue
without the historical fixture or answer. The independent audit restored the
bad sign and ran each proposed regression:

| Model | Prompt | Repair | Discriminating regression |
|---|---|---:|---:|
| GPT-5.5 | minimal | yes | yes |
| GPT-5.5 | verifier first | yes | yes |
| GPT-5.6 | minimal | yes | no |
| GPT-5.6 | verifier first | yes | yes |

The two verifier-first agents demonstrated fail-before/pass-after themselves.
The GPT-5.5 minimal test was independently shown to fail at its predicted
mirrored point under mutation. The GPT-5.6 minimal change still passed under
mutation. This single benchmark does not rank the generations or estimate a
general success rate.

Run costs, retained only to avoid flattening unlike quantities:

- GPT-5.5 minimal: 227 s; 58,161 uncached input, 638,976 cached input, 4,680
  output tokens.
- GPT-5.5 verifier first: 219 s; 38,608 uncached input, 540,416 cached input,
  4,376 output tokens.
- GPT-5.6 minimal: 126 s; 46,311 uncached input, 603,648 cached input, 3,106
  output tokens.
- GPT-5.6 verifier first: 155 s; 45,420 uncached input, 708,096 cached input,
  3,846 output tokens.

Do not sum these token categories or translate them into monetary cost without
the applicable pricing and cache rules.

### Mathematical labor types

Claim: selected cases distinguish useful review, obligation extraction,
empirical falsification, and candidate generation from completion of a proof or
production of a new mathematical object. They do not estimate comparative
success rates between labor types.

Sources:

- FG/CH2021 theorem-gap review on 2026-07-01: source-paper inspection found the
  Type-2 capacity boundary; formal/Rust short-word and singular-boundary
  mismatches; overstated algebraic support; and underspecified theorem
  hypotheses. Corrections merged in `fcd8545a`. The formal theorem remained
  explicitly unverified, so this is a durable review result rather than proof
  completion.
- Local-systolic-behavior episode: an attempted proof route produced a
  transition diagnostic showing how a target sigma absent at the base point
  could appear after perturbation. It did not establish finite-distance
  completeness or error bounds. The package was later removed/migrated by
  `0c905a96`.
- Gradient-ascent validity episode: empirical diagnostics showed selected
  endpoints remained improvable, contradicting their interpretation as
  converged local maxima. No replacement theorem, proof-ready definitions, or
  durable formal artifact was produced.
- Candidate-proposer episode: low two-face-area rules produced candidate rows
  and upper-tail enrichment after correction of a pooled-control bug, but no
  selected row exceeded one. The episode's branch was abandoned; the maintained
  scalar/ridge proposer later evaluated 100,000 candidates, selected 485 plus
  1,195 baselines, and remained sub-threshold with maximum about 0.868. Do not
  attribute the maintained packet to the abandoned episode.

Selection boundary: these are deliberately informative complete cases. They
are not a random or exchangeable sample and cannot establish a ranked capability
frontier or proof-success frequency.

### Breadth, selection, and integration

Claim: increased parallel production coincided with more work on prioritization,
verification, stopping, and integration. The record supports coexistence, not a
measured causal bottleneck coefficient or an unmeasured cost comparison.

Sources:

- Sys-data-science root session `019f50cf...` on 2026-07-11--12: 30
  structurally linked descendants, about nine object-level lines, explicit
  reprioritizations and stops, and an equal-budget S0 implementation with 7,312
  inserted lines. The commits were initially branch-only and later reconciled
  into Main as bounded research state by `43ac70b4`. The real target run had not
  occurred. This establishes integrated candidate supply and negative/partial
  research state, not a completed forward experiment.
- Profiling episode on 2026-06-08--09: the record contains useful measurements
  together with an overconfident recommendation and contaminated handoff;
  factual rewrite, fresh audit, and a provenance decision preceded retained
  commits `054c689b` and `95aa3506`.

## Deliberately omitted claims

- A global coefficient for AI productivity or net benefit.
- A human-only counterfactual inferred from session duration.
- Exhaustive session, token, commit, or changed-line statistics.
- General superiority of GPT-5.5 or GPT-5.6.
- Git history as evidence of idea origin or semantic acceptance.
- The aggregate log report as evidence of mathematical correctness or causal
  impact.

## Maintenance

If the prose changes, preserve the distinction between direct observation,
Jörn's retrospective judgment, and inference. Delete or reduce this companion
after section 13 is stable; do not turn private process data into a tracked
appendix by default.
