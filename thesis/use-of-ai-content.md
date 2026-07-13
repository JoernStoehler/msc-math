# AI In The Research Process: Content Companion

Status: section-local evidence and maintenance companion for
`thesis/13-use-of-ai.tex`. Not source truth and not thesis text.

Purpose: keep the publication-facing claims tied to their evidential roles. The
separate factual declaration is owned by `thesis/ai-use-disclosure.tex` and
`thesis/ai-use-disclosure-content.md`.

Overruled by: project artifacts, accepted Jörn/Kai decisions, and final thesis
review. Raw session text and row-level derived data remain private and must not
be committed.

## Publication status and known shortcomings

As of Jörn's 2026-07-13 review, this section is **provisional and not accepted
as final publication prose**. The separate disclosure page has been accepted.
Jörn found the proposed Section 13 conclusions too abstract, banal, and close to
surface observations. In particular, generic advice such as “use tests,”
“prioritize,” or “review agent output” is not a thesis-worthy finding merely
because a project episode illustrates it.

Known missing or incomplete work:

1. **Observed agent cost versus observed result.** The case matrix records
   outcomes but does not join most episodes to rollout-tree resources. Existing
   infrastructure can recover model mixture, lineage size, elapsed span,
   uncached input, cached input, output, and API-equivalent shadow cost for many
   Codex cases. Those unlike token categories must remain separate. Human active
   time is mostly unavailable and must not be inferred from wall-clock session
   duration. The sign replay is currently the only section case with complete
   per-run telemetry.
2. **A result/value ladder.** “Produced,” “committed,” and “integrated” do not
   mean mathematically correct or thesis-valued. A future comparison should
   distinguish at least: candidate produced, executable, checked by a relevant
   discriminator, interpreted correctly, integrated as bounded state, retained
   as thesis evidence, and accepted by Jörn/Kai where required.
3. **The ragged frontier across mathematical labor.** The selected cases do not
   compare proof search, conjecturing, formalizing intuitions, explanation,
   coding, code/data interpretation, mathematical review, experiment design,
   or research taste systematically. They establish examples, not relative
   effectiveness, efficiency, or failure profiles.
4. **Comparable repeated work.** Apart from the one-bug four-run replay, the
   packet does not select repeated or continued tasks across GPT-5.5 and GPT-5.6
   using skills, keywords, lineage, or shared artifacts. It therefore cannot
   distinguish task effects from model, prompt, review topology, or project-era
   effects.
5. **Review independence.** A fresh agent or additional review count is not an
   external oracle. Shared prompts, sources, models, and proxies can produce
   correlated blind spots. The current packet has examples of source-anchored,
   mutation-based, and alternative-route checks but no general analysis of
   which review structures caught which errors.
6. **Naturalistic productivity analysis.** The absence of a randomized
   human-only counterfactual does not make observed cost--result analysis
   impossible. A future model may compare the observed task distributions for
   Jörn and agents, fit or falsify workflow hypotheses, and use those estimates
   cautiously for counterfactual reasoning. The current section instead
   retreats too often from causal overclaiming to non-quantitative description.
7. **Coverage and selection.** Cases were chosen retrospectively for explanatory
   and outcome diversity, not sampled prospectively or representatively.
   Session coverage, offline work, external tools, lineage gaps, semantic
   acceptance, and first-origin uncertainty remain limitations. The exact sign
   replay prompts were encrypted; its reconstructed contracts, one run per
   cell, and lack of randomization preclude model/prompt effect estimates.
8. **Thesis narrative.** The current section overweights defensibility and
   generic workflow recommendations. It has not yet earned a strong central
   interpretation of what the project reveals about current agents on a
   mathematician's daily work. Its length and case detail should be reconsidered
   after, not before, the missing cost--result and labor-frontier analysis.

Useful existing inputs for a future session are
`experiments/ai-use/reports/ai-research-workflow-case-matrix.md`,
`experiments/ai-use/reports/project-efficiency-analysis-2026-07-12.md`, the
token/lineage scripts documented in `experiments/ai-use/README.md`, and the
private complete-case artifacts named by the case matrix. Do not restart by
collecting a large transcript dataset; first join the existing selected cases
to resource accounting and test whether that changes a thesis-relevant
conclusion.

## Evidence design

The section uses selected complete episodes plus one structured four-run replay.
It does not estimate net productivity or reconstruct an exhaustive research timeline.
Recorded sessions establish observed requests, actions, reviews, and handoffs;
Git establishes retained artifact states and branch integration; surviving
tests and code establish only what they directly check. Jörn's retrospective
account supplies subjective and offline context and is identified as such.
Episode output, later artifact evidence, and final integration are assessed
separately and are not attributed to the same agent episode unless recorded
lineage establishes that link.

Cases were selected after observing project history for explanatory and outcome
diversity; they are neither prospective nor representative. The bounded pilot
was enough to reconstruct episodes but not enough to establish that broader or
more systematic comparison lacks thesis value. Jörn's later review identified
cost--result accounting and the mathematical-labor frontier as material missing
questions.

The working principles use Jörn's 2026-07-12 account of delegation, verifier
design, failure ledgers, selective salvage versus restart, human-attention cost,
breadth-first search, and model-version non-transfer. The record checks these
against selected episodes; it does not turn them into universal prescriptions.

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
GPT-5.5/GPT-5.6-sol comparison.

Source: structured four-run replay from base `f3d36cc9`, before the historical
projection reduced-gradient sign repair. All runs received the same known issue
without the historical fixture or answer. The exact prompts are encrypted and
unrecoverable; the task contracts are a contemporaneous reconstruction. The
independent audit restored the bad sign and ran a semantically faithful
reconstruction of each proposed regression:

| Model | Prompt | Repair | Discriminating regression |
|---|---|---:|---:|
| GPT-5.5 | minimal | yes | yes |
| GPT-5.5 | verifier first | yes | yes |
| GPT-5.6-sol | minimal | yes | no |
| GPT-5.6-sol | verifier first | yes | yes |

The two verifier-first agents demonstrated fail-before/pass-after themselves.
The GPT-5.5 minimal test was independently shown to fail at its predicted
mirrored point under mutation. The GPT-5.6-sol minimal change still passed under
mutation. There was one run per cell, no randomization, and no within-condition
replication. This case study therefore identifies neither a causal prompting
effect nor a model-generation effect, and it does not estimate a general
success rate.

Run costs, retained only to avoid flattening unlike quantities:

- GPT-5.5 minimal: 227 s; 58,161 uncached input, 638,976 cached input, 4,680
  output tokens.
- GPT-5.5 verifier first: 219 s; 38,608 uncached input, 540,416 cached input,
  4,376 output tokens.
- GPT-5.6-sol minimal: 126 s; 46,311 uncached input, 603,648 cached input, 3,106
  output tokens.
- GPT-5.6-sol verifier first: 155 s; 45,420 uncached input, 708,096 cached input,
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
- Triangle--hexagon interval proof candidate: an agent derived an explicit
  generalized three-bounce-billiard argument for
  `sys <= (3/4) sec(delta)^2 <= 1` on the fundamental rotation interval,
  conditional on the cited capacity--billiard interface, and a separate
  range-norm lower bound for endpoint equality. A fresh agent independently
  checked the geometry, normal-cone signs and indexing, action and volume,
  endpoint nonsmoothness, lower bound, and theorem boundary. Commits `a0d4414e`
  and `dc966b07` were reconciled into Main by `43ac70b4`. The artifact explicitly
  remains unreviewed and unapproved by Jörn; interior equality with the secant
  branch is not proved. Section 13 therefore presents this as a substantive
  proof candidate rather than an accepted thesis result.
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

Narrative boundary: the section does not need one privileged conclusion. Its
case-backed findings concern different stages and labor types and may remain
separately useful without being compressed into a global verdict on AI's value.

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

## Claims still forbidden by current evidence

- A global coefficient for AI productivity or net benefit without an explicit
  value model.
- A human-only counterfactual inferred directly from session duration.
- General superiority of GPT-5.5 or GPT-5.6-sol from the one-bug replay.
- Git history as evidence of idea origin or semantic acceptance.
- The aggregate log report as evidence of mathematical correctness or causal
  impact.

These restrictions do not forbid per-episode resource accounting, comparison of
observed result stages, naturalistic workflow models, or matched future trials.
Exhaustive token/commit/line statistics are not a required output, but targeted
cost accounting for the cases used in thesis prose is missing evidence rather
than a forbidden analysis.

## Maintenance

If the prose changes, preserve the distinction between direct observation,
Jörn's retrospective judgment, and inference. Delete or reduce this companion
after section 13 is stable; do not turn private process data into a tracked
appendix by default.
