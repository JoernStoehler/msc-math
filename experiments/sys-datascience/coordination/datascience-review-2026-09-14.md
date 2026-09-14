# Data-science review: evidence, opportunities, and exploration readiness

Date: 2026-09-14. Read-only investigation followed by this requested report.
This is a source-linked diagnostic and planning account, not a new empirical
result, launch authorization, or exhaustive codebase audit. No new target
evaluations, model fits, artifact downloads, or expensive computations were run.

## Executive diagnosis and subsequent scope correction

The repository contains a substantive chain of evidence: invariant descriptors
predict stored numerical targets; frozen selectors enrich fresh candidates below
the threshold; stronger proxy conditioning does not reliably improve that
enrichment or produce counterexamples. The thesis currently makes less of this
chain than the retained evidence permits.

**Jörn clarified the organizing narrative after the initial review:** the
subject is not teaching or cataloguing data-science methods. It is the meta
approach of using AI to explore the mathematical problem broadly with data
science and seeing whether the problem yields to that approach. Methods and
their outcomes are evidence about that attempt.

**The initial review overfavored consolidation.** Existing results justify
reporting what happened, but do not establish a strong reason to stop the
overall approach. Individual failed continuation criteria do not close other
methods, representations, or competent redos of inadequately executed work.
Confusing code/docs and failed execution must not be counted as scientific
negative results. Jörn regards another five hours as potentially inexpensive,
and emphasized that five hours of elapsed time could include roughly twenty
parallel agent experiments after shared preparation, followed by consolidation.
That is a possible architecture, not a measured resource estimate or an
authorized execution packet.

The review supplies enough context to investigate such a portfolio. It does
**not** yet supply enough operational evidence to choose the best twenty
experiments, guarantee one-hour preparation, or conclude that new methods would
mostly be ceremony. Repeated estimators on the same features are a weak bet;
genuinely different approaches and useful redos remain insufficiently audited.

## Coverage and evidence standard

Five parallel reviewers covered different responsibilities: packet inventory;
scientific reading of the thesis; reproducibility/provenance; bounded follow-up
opportunities; and duplicate/dependent evidence. The coordinator reconciled
selected conclusions against primary code and compact artifacts, including RF
metric definitions, P2 contracts, prospective result JSONs, and an HKO reporting
contradiction. This is broad source coverage, not independent validation of all
producer implementations or mathematical proofs.

All paths below are relative to `experiments/sys-datascience/` unless stated.
Packet-local artifacts take precedence over dated coordination summaries.

The [retained numerical lineage account](../../polytope-datasets/retained-lineage.md)
is the target trust boundary. The historical 14,336-row table establishes stored
numerical targets, not certified capacities. Its loader copies capacity,
volume, and sys, and assigns backend labels itself. Historical source code
allowed cache reuse; there is no matching clean-run/cache-freshness manifest.
The July certified-producer migration cannot certify the earlier June payloads.

## Current evidence map

| Question | Data and method | Result | Epistemic status / readiness |
| --- | --- | --- | --- |
| Did the retained sample contain recorded sys > 1? | 4,096 generic rows, 512 each F=5–12; 10,240 products, 1,024 each of ten polygon-pair buckets; seed 42, heights [.8,1.2]; predicate scan | Zero; max .8625859, median .3108414, p99 .7521020 | Finite historical table. Ready with explicit numerical qualification; no global absence or prevalence claim |
| Do features predict targets? | RF: 39 features, train/test 8,192/6,144. P2: 45 features, 8,704/5,632. Grouped holdout source:facet count | RF R² .885166, MAE .049459. P2 GB R² .878424, MAE .052034; ridge-only .887228 versus counts-only .043147 | In-table prediction; different splits and schemas prevent direct ranking of algorithms |
| Does ridge association survive bucket conditioning? | Same 14,336 rows, 18 buckets; within-bucket fractional ranks | Ridge-sum correlation −.944; all 18 signs negative, range −.997 to −.841 | Descriptive association; useful figure, no mechanism identification |
| Do frozen scalar rules enrich fresh candidates? | Original 100k candidate pool; 30 selection sets; 1,675 unique evaluated union | 485 unique selected, 1,195 baseline, five shared; max .867546; no >1 | Prospective numerical selection; historical evaluated-target.v2 |
| Does concentration add incremental enrichment? | Independent 100k seed; 2,490 targets; frozen cascade versus stage-one complement | Mean increment +.033992, positive 8/10 buckets; max .902384 | Passed predeclared descriptive criterion, not a significance test or threshold success |
| Does covariance selection work prospectively? | Two fresh 50k pools; 1,436 unique targets; rho/ridge/control arms | Rho−control +.333123; rho−ridge +.014488, interval [−.014926,.043902] | Enrichment, no established rho superiority; shared controls and 64 selected overlaps |
| Does low ridge transfer to generic polytopes? | 10k target-blind F10 candidates; 100 selected and 100 disjoint controls | Means .625707 vs .338570; difference +.287137 [.249767,.324117]. Hardening difference −.031761 [−.094574,.033126] | Coarse transfer succeeds; harder-conditioning continuation criterion fails, not proof that hardening is harmful |
| Does selection transfer to another law? | factorial-both, one seed, two buckets, 6,400 eligible source rows, 91 unique targets | Rho−control +.258728; ridge−control +.225355; no >1 | Prospective finite-design transfer; clean evaluator identity, shared controls and five selector overlaps |
| Which explanations survive? | Retained bounce/anatomy work, controlled orientation witnesses, designed endpoint paths | Orientation changes targets at witness level; directional ridge mediator fails; smaller ridge is not universally better | Mixed descriptive, controlled numerical, and separately proof-owned evidence |
| How effective is finite-budget optimization? | 64 matched F10 starts × seven policies = 448 traces | Best terminal median about .9848; none ≥1; paired uncertainty retained | Historical heuristic evaluator; development isolation unproved; separate companion experiment |
| Do local probes establish local maximality? | Selected-body panel and controls; 1,482 valid probes | Targets resist tested probes; positive controls improve; other endpoint audits find missed ascent | Bounded observations only; dirty producer state prevents exact historical replay |

Primary references:

- [P2 README](../methods/standard-baseline-p2/README.md) and its
  `artifacts/{summary.json,regression-metrics.tsv,high-tail-classification-metrics.tsv,feature-family-ablation.tsv}`.
- [RF summary](../methods/prediction-ranking/artifacts/summary.json) and
  [analyzer](../methods/prediction-ranking/analyze.py).
- [Atlas analysis](../methods/conditional-tail-atlas/artifacts/analysis.md),
  `ridge-extreme-tail.tsv`, and `feature-rank-correlation.tsv` in that directory.
- [Scalar proposer](../methods/extreme-scalar-rejection-proposer/README.md),
  especially artifact directories `100k-promising-scalars`,
  `100k-ridge-concentration-validation`, and `covariance-rho-frozen-validation`.
- [Generic analysis](../methods/generic-ridge-tail-stage1/artifacts/stage1/analysis.json)
  and [target evaluator account](../methods/generic-ridge-tail-stage1-target/README.md).
- [Alternative-source account](../methods/alternative-source-transfer/POST-TARGET-ACCOUNT.md)
  and `artifacts/transfer-v1/{analysis.json,result-manifest.json,target-evaluations.jsonl}`.
- [Main chapter](../../../thesis/08-black-box-datascience.tex),
  [optimizer subsection](../../../thesis/08-black-box-datascience-finite-budget-optimization.tex),
  [local screen](../../../thesis/08-black-box-datascience-local-maxima-check.tex),
  and [appendix](../../../thesis/a-datascience-results.tex).

## Valuable existing work

- Actual transitions from post-target exploration to frozen selection, then
  fresh-seed and alternative-source tests. These are more informative than a
  catalogue of fitted models.
- Matched baselines, combinatorial ablations, U(2) invariance controls, HKO
  recovery, and structured positive-control probes address named alternatives.
- The generic stage-one stop criterion produced an interpretable outcome:
  coarse enrichment without evidence that stronger conditioning improves it.
- Existing uncertainty includes generic bootstrap/Wilson summaries, covariance
  effect intervals, alternative-source resampling, and paired optimizer
  checkpoint/final intervals. Adding another bootstrap is not automatically new
  information.
- Exact triangle and two-bounce arguments have formal owners. Numerical tests
  should remain support rather than be promoted into proof premises.
- Negative mechanism findings constrain simplistic explanations and are worth
  retaining even if they are not central chapter sections.

## Unclear, under-presented, or misleading work

### Metric and inference mismatches

The RF analyzer's `top_decile_enrichment` is precision among predicted-top-decile
rows relative to the **test-target** q90, not fold lift. Its value is .603252.
P2 uses train q90=.569765; test positive rate is .190696. Its classifier has
AUC .935973, AP .703431, precision .741135, lift 3.88647. Different split seeds,
features, thresholds, and estimands prevent an apples-to-apples model ranking.
The RF ten-permutation p=1/11 is a coarse sanity check, not strong inference.

The metadata baseline includes post-evaluation bounce labels and held-out
categorical levels. Its weakness does not eliminate source confounding.
Within-bucket association and feature-family ablations are complementary guards.
Ridge area / sqrt(volume) and sys=capacity²/(2 volume) share normalization;
this is not automatically target leakage, but it complicates mechanism claims.

Alternative-source bootstrap intervals condition on realized arm×bucket panels,
not source/seed variation. The permutation output 0.0 means zero exceedances
among 10,000 draws, not p=0. Covariance intervals use t(df19) across twenty
seed×bucket effects, not twenty independent seeds. The generic hardening
interval includes zero: its continuation criterion failed, but a negative true
effect was not established.

### Concrete reporting defects and weak diagnostics

- The atlas says HKO's ridge sum is below every retained value. Its source
  [HKO summary](../methods/hko-reference-coverage/artifacts/summary.json) records
  HKO 8.94427191 and retained minimum 8.92591789. The false sentence is hard-coded
  in `conditional-tail-atlas/analyze.py` around line 246. Correct the sentence
  and generated account before using that interpretation.
- Atlas extreme-band maxima compare 140 lowest-1% rows against 4,312 rows in
  the 20–50% band. Maxima do not isolate conditioning from sample opportunity.
- Zero top-25 anomaly overlap with the top 2% target tail is weak evidence:
  random ranking expects only .5 overlaps, with approximately .60 probability
  of zero. Demote it or ask a more informative ranking question.
- A distribution scan's assumed support (0,1) is a model restriction, not a
  mathematical fact usable to rule out >1.
- Some theorem-packet README statuses lag later formal work. Reconcile with the
  formal owner instead of treating an old empirical status as current proof status.

### Duplicate versus distinct evidence

- EDA, association, prediction, covariance, bounce, and tail models mostly reuse
  the same retained table. They have different endpoints, not independent data.
- The [feature quotient](../methods/statistical-associations/artifacts/feature-family-quotient.json)
  checks fourteen declared identities. Some P2 ablations are identical
  complements because there are only two families. Feature count is not a count
  of independent measurements.
- Ridge mean/sum selections coincide in fixed product buckets. Other highly
  correlated ridge summaries can select different extremes; do not merge those
  selection tests solely because correlations are high.
- Original 100k and 1M ridge pools share seed 271828 and nested generation.
  Read-only cache joining found 21 equal evaluated IDs with exactly equal sys.
  The 1M pool has only 466 evaluated targets: 166 selected and 300 baseline.
  It is not a million-target negative result.
- The hypothetical 1M budget in `tail-survival-1m-posterior` is a third object:
  sensitivity analysis of retained data, not that generated pool.
- Generic anatomy reuses its 200 targets and 142 product 5x5 targets from the
  rho packet (42 rho-only, 42 ridge-only, eight overlap, fifty controls).
- Independent concentration adds a fresh seed; rho adds two fresh seeds and a
  different descriptor; alternative-source transfer adds a source law. Arms
  within each experiment still share controls and sometimes selected rows.

## Additional inspected structural results

These deserve preservation but do not all belong in the main narrative.

- `canonical-vertex-covariance`: 14,334/14,336 accepted; pooled correlation
  −.963917, within bucket+vertex-count ranks −.965526. Two numerical rejects.
  This is post-target description; the separate frozen rho packet supplies
  prospective evidence.
- `generator-orientation-target-pilot`: eight bases × identity/U(2)/SO(4).
  U(2) maximum difference 5.55e−16; SO(4) absolute difference ≥.01 on 7/8,
  median .212909. Ridge correlation −.8333 but only 5/8 opposite signs fails
  the conjunctive mediator gate. An earlier exposed panel was discarded after
  a hash typo/path-depth error; retained rerun provenance discloses this.
- `product-bounce-distribution`: 3,979 two-bounce and 6,261 three-bounce labels;
  means .23671/.39763. Labels are target-derived, not proposer features.
- `product-bounce-mechanism`: log-sys association decomposes into capacity and
  volume terms; adjusted associations remain observational. Upper-tail near-tie
  rates do not support a simple A2/A3 near-tie explanation.
- `product-bounce-class-degeneration`: 9,455 complete rows, 3,559 shared-support
  rows, 10,471 pairs. Shared-support rate 37.6%, 49.3% near |g|≤.01, versus
  shuffled mean 12.6%. Pair multiplicity is not independent sample size.
- `product-bounce-width-shortcut`: independent exact numerical check is 20/20
  fixed rows plus three analytic fixtures. The 10,240-row association reuses
  target-derived A2. The full independent exact audit was stopped; do not imply
  its completion or silently reopen it.
- `product-triangle-bounce-classification`: 1,024 strict-sign retained rows;
  718 available domination comparisons all pass; 20k rational stress tests,
  no strict counterexample, 27 boundary equalities. Formal proof owns theorem.
- `residual-exemplar-seeds`: ten inspection records, fourteen unique bodies;
  no consistent incidence statistic across four discordant pairs. An apparent
  orbit-class difference was permutation duplication, not new mechanism.
- `ridge-endpoint-path`: eight frozen points; 3x6 q01 sys .9509718, endpoints
  .75/.50. Designed paths refute universal monotonic proxy interpretations,
  not population or causal statements. `ridge-symmetry-completion` adds two
  points and a mathematical path bound; more numerical densification is not
  needed for that bound.
- `tail-survival-1m-posterior`: retained backtests and prior sensitivity already
  show extrapolation fragility; a reported million-budget probability spans
  .0367–.986 across priors. More fitted distributions cannot resolve this by
  methodological volume alone.

## Blockers and smallest unblockers

| Issue / category | Claim or operation blocked | Minimal response |
| --- | --- | --- |
| Historical numerical lineage / provenance | Certified-capacity interpretation | Keep recorded-target qualification; recover actual historical manifest if available, or separately budget certified evaluation |
| Registered large artifacts locally absent / materialization | Selected raw-data reruns | Materialize only needed bundle; absence is not broken code |
| Registered table lacks six active ridge columns / schema | Current 45-feature consumers | Retrieve validated derivative or budget a feature rebuild; no new target evaluation needed |
| Derivative historically retained in scratch / artifact retention | Convenient repeatability | Locate/register verified copy before recomputing |
| Hard-coded HKO claim / implementation and exposition | Specific support comparison | Compute comparison from source summary and correct generated prose |
| Dirty local-screen producer lacks patch/hashes / provenance | Exact historical replay | Preserve archival finite-observation boundary; clean rerun would be new evidence |
| Optimizer tuning isolation unproved / experimental provenance | Confirmatory method-selection generalization | Keep matched descriptive comparison; rerun cannot repair historical selection |
| Scratch-only old distribution outputs / artifacts and docs | Exact old-fit citations | Prefer retained sensitivity packet unless missing output serves a unique claim |
| Two-bounce exact audit only twenty rows / scientific denominator | Full-table independent exact-validation claim | State true denominator and formal proof status |

The provenance reviewer checked 26 registered paths across six bundles: source
dataset 0/3 present, invariant table 0/1, covariance frozen raw packet 0/18,
alternative-source full source 0/1, bounce distribution 0/1, degeneration 0/2.
These were absent paths, not broken symlinks. Compact summaries and many frozen
selection/target artifacts remain locally available. Remote availability and
credentials were not tested; no secret contents were read.

P2 documents old table SHA-256 `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`
versus current derivative `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`.
The previous validated rebuild took 364.92 seconds wall with substantial
parallel CPU use and prompted Jörn's objection. Its historical retained path
was `/tmp/sys-ds-p2-current-full.verify.bVbiYw`; continued existence was not
established. Materializing the old registered table does not alone unlock
current-schema analyzers.

The cheap `check-retained-lineage.py` guard passed: 14,336 unique provenance
identities, source counts and registry identities agree. This does not validate
capacities or remote bytes. No current compilation failure was established;
compilation health remains untested.

Alternative-source transfer has materially stronger execution provenance than
the June table: clean evaluator commit `5a5736687dcd8ad10f4a682266fa24d1fe067efc`,
source/lock/backend digests and 91 retained targets. That is still numerical
transfer evidence, not automatic certified capacity computation. Generic target
evaluation records its historical capacity_auto/pruned-HK route separately.

## Ranked existing-evidence easy wins

These rankings optimize near-term clarity and bounded information, **not** the
value of a fresh twenty-agent research round. That second ranking remains open.
Costs are planning estimates of implementation/review time, not measured runs.

| Rank | Payoff, inputs, bound and cost | Outcome branches and retained value | Shared risks |
| --- | --- | --- | --- |
| 1 | Claim/method/evaluator ledger from manifests, scripts, summaries; 3–5h, no scientific computation | Success: central claims reconcile. Partial: peripheral gaps. Failure: contradictions become a precise claim-boundary ledger | Packet-to-claim mapping and historical provenance |
| 2 | Prospective-effect display from existing concentration/rho/generic/transfer verdicts; 3–5h, reuse intervals, no pooled meta-analysis | Success: enrichment plus hardening limits clear. Partial: source-specific heterogeneity. Failure: exclude incompatible contrast with reason | Shared targets, controls, selectors, evaluator families |
| 3 | One explanatory figure from atlas TSVs and generic selected/control/hardening summaries; 2–4h | Success: readable empirical arc. Partial: simpler separate panels. Failure: corrected table instead of misleading figure | Same ridge hypothesis and retained data as other displays |
| 4 | Consolidate exact redundancies and metric definitions using quotient artifacts/analyzers; 2–3h | Success: shorter defensible methods account. Partial: retain genuinely distinct selections. Failure: mark incomparable metrics explicitly | Documentation accuracy, not new numerical certification |
| 5 | Budget-consistent optimizer display from existing curves and paired intervals; 1–2h | Success: stable finite-budget ordering. Partial: similar-performance tier. Failure: checkpoint coverage restricts comparison | Same optimizer evaluator and uncertain tuning isolation |
| 6 | Optional equal-opportunity tail sensitivity; 2–4h, ≤2,000 resamples | Positive: descriptive deficit persists. Null: opportunity explains reversal. Ambiguous: sparse/weight-sensitive bands. Procedural failure: count/hash mismatch retained | Same table, normalization and ridge hypothesis |
| 7 | Optional component accounting from existing sixteen-start optimizer traces; 3–6h | Positive: stable capacity/volume pattern. Null: compensation. Ambiguous: gauge dependence. Procedural failure: missing state join | Optimizer lineage and geometric normalization |

### Exact proposed new computation: equal-opportunity tail sensitivity

Question: does the atlas's apparent extreme-proxy ceiling survive equal sample
opportunity? Existing aggregate maxima cannot answer this because band counts
differ. Minimum inputs: stable ID, bucket, ridge sum and stored target; newer
six columns and raw geometry are unnecessary.

Freeze 0–1%, 1–2%, and 2–5% bands and two contrasts; within each bucket draw
equal counts without replacement, limited by its smallest band. Report q90 and
maximum distributions under fixed bucket weights and one weighting sensitivity.
Stop after 2,000 resamples, or immediately on unreconciled counts/hash. No
adaptive bin search or target calls. Positive/null/ambiguous outcomes refine
only the empirical conditioning interpretation, not a population endpoint.
Procedural failure leaves the original descriptive plot usable with count
labels and the limitation stated. This is not independent replication.

### Exact proposed new computation: optimizer component accounting

Question: under declared normalization, does recorded progress primarily involve
capacity growth, volume reduction, or compensation? Existing summaries lack this
decomposition. Minimum inputs: already present sixteen-start longer-budget
branch-history evaluation traces, capacities, volumes, states, IDs and clocks.

Use initial, one-second and terminal best states. Verify
`Δlog(sys) = 2Δlog(capacity) − Δlog(volume)`. Distinguish best-so-far from
current algorithm states. Declare one geometric normalization and compare raw
coordinate accounting because the components depend on scaling. Stop at sixteen
starts, three checkpoints and one figure; no target calls or automatic expansion.
Positive supports explanatory accounting; null means compensation; ambiguous
gauge dependence limits it to a bounded observation; failed joins yield an
exact missing-link record. None establishes causal mechanism.

## Chapter-story alternatives, updated to Jörn's intended narrative

1. **AI-assisted exploration that obtains traction but does not solve the
   search problem.** Show the bet, breadth of competent attempts, progression
   to prospective tests, gains, and unresolved obstacles. Recommended narrative.
   The reason for ending effort is not yet scientifically settled.
2. **Controlled negative search benchmark.** Closest to current manuscript;
   defensible but undersells enrichment and risks turning many methods into a
   repetitive zero-count account.
3. **Finite-budget optimization and limits of local evidence.** Strong companion
   story, but shifts attention from the broad data-science attempt and depends
   more on historical evaluator/tuning limitations.

The meta-approach account must distinguish evidence about the mathematical
problem from evidence about AI: these experiments show what this AI-assisted
attempt achieved, not that AI outperformed human exploration or that another
attempt cannot succeed.

## Next investigation: launchable parallel portfolio, not yet selected

The initial consolidation recommendation is a useful available packet, but
does not settle Jörn's subsequent question about another broad exploration
round. A targeted readiness/opportunity audit should determine:

- Can an agent load suitable data and run a baseline in minutes? Test a named
  consumer path rather than broadly clean the codebase.
- Which promising questions never received competent execution, and which are
  genuinely answered? Trace failed attempts to primary outputs and exact causes.
- What target-evaluation throughput and CPU/memory budget can support concurrent
  experiments? Twenty agents do not imply twenty affordable parallel solvers.
- Which approaches ask different questions or use different representations,
  rather than changing estimators over the same columns?
- Which minimal shared adapter or materialized dataset unlocks which named
  experiments? Refactoring should have concrete consumers.

A possible elapsed-time architecture is one hour enabling work, three hours
independent experiments, one hour consolidation. Its feasibility is unmeasured.
Workers need not cross-learn during execution, but should share input/target
contracts, controls, resource limits, and a result schema so consolidation can
distinguish scientific negatives from procedural failures.

The next deliverable should have more candidates than launch slots, each with:
question and changed claim; prior competent/incompetent coverage; exact inputs;
minimum enabling changes; compute budget; controls; stopping rule; positive,
null, ambiguous and procedural-failure interpretation; useful retained output;
shared dependencies; and a fallback. This report has not performed that audit.
No claim that the best twenty experiments are known is warranted.

## Remaining unknowns and uninspected work

Inspected starting points include all eleven user-named thesis, README,
coordination, lineage and memory documents, plus the appendix. Reviewers followed
packet-local compact JSON/TSV artifacts, selected per-row caches, manifests,
feature/evaluator implementations and registry paths. The coordinator directly
checked key comparisons rather than adopting all reviewer judgments verbatim.

Not done: expensive model/target reruns, Rust compilation, remote materialization,
independent historical capacity validation, full proof review, complete visual
QA, exhaustive raw-row reconciliation, or an operational timing experiment.
Geometric duplicates across differently named IDs remain unchecked. Exact
historical execution state, remote availability, and some replay dependencies
remain unresolved. The practical ease of candidate methodology experiments and
the best independent portfolio remain major known unknowns.

No code/docs repairs described above have been made by this review. The only
requested write is this persistent report. Existing unrelated `.codex/config.toml`
and `tmp/` worktree changes were left untouched.
