# Method Surface Expansion

Use / maintenance model: active surface-scout workspace for identifying missing
method families, candidate topic-owner sessions, and considered-but-currently
low-value research directions. It may be more exploratory than other topic
files, but should still preserve why a seed is promising or not worth spawning.

Scope: broad idea generation over possible data-science methodologies,
experiment designs, and missing topic areas for the full sys-datascience slice.

Status block:

- topic-status: active surface-scout seed
- spawn-status: use for spawn/rescope/stop decisions before broad execution
- next-role: surface scout or design scout
- next-action: separate retained-contract wording from broader producer-axis and
  standard-method evidence needs
- review-gate: no blind new producer/model run until a thesis sentence and one
  producer or method axis are named
- belief-update-owner: surface scout or research-map steward
- last-reviewed: 2026-07-04 method-surface audit; 2026-07-06 workflow hardening
  pass added status metadata only
- source-of-truth: `../../methods/README.md`, current topic maps, and method
  disposition/checklist files listed below
- stale-if: thesis wording changes, new method packets land, or the retained
  random/product source contract changes
- allowed-downstream-use: session-design and method-surface prioritization; not
  a claim that all standard methods have been exhausted

Current belief: the method surface is adequate for narrow wording but not for
strong "all standard data-science methods" closure. The main value here is
identifying cheap high-information packets and deciding when a seed deserves a
topic owner.

Owner-readiness/status: active surface-scout seed. Use this when the immediate
question is which sessions to spawn/rescope/stop, not when a concrete packet is
already ready.

2026-07-04 method-surface audit update: the surface is strong enough for narrow
retained random/product wording, but not for broad claims that standard
data-science methods were exhausted or that random models beyond the retained
producer contract were covered. Recommended next method-surface work is a
standard-method closure plan before any new model runs. A broader
random-distribution wording/design scout is separate and should be design-only
until it names a thesis sentence and one producer axis.

2026-07-04 closure plan status: the standard-method closure plan is parked,
not live scratch. It recommended a tiny retained-table baseline packet only if
thesis wording needs stronger ordinary-method coverage than the named existing
packets. Proposed scope: lasso/elastic-net, gradient boosting, high-tail
classification, and feature-family ablation. Parked/rejected for now:
SVM/kNN/kernel, neural/autoencoder, Bayesian/GP, density/mixture/one-class,
direct-search/optimization, and broader random-distribution work. Do not track
the old `/tmp/sys-ds-standard-method-closure-plan.md` as active state; this
paragraph is the durable rediscovery hook.

Evidence sources:

- `../../methods/README.md`
- `../research-ledger.md`
- current topic maps in this folder
- `../../methods/trusted-random-product-method-dispositions.md`, current disposition
  ledger for retained random/product method families
- `../../methods/method-coverage-checklist.md`, broad recall checklist, not current
  source truth

Adjacent topics to read:

- all current topic files, because this topic exists to compare and expand the
  surface.

Candidate hypotheses:

- Most additional standard methods will confirm known structure rather than
  change the thesis result.
- A small number of missing methods or representations may find interactions
  that scalar filters missed.
- The largest value may come from better question generation rather than from
  running more models.

Current method-surface disposition table:

| Family | Current coverage | Evidence stage | Wording affected | Current disposition | Reopen trigger |
| --- | --- | --- | --- | --- | --- |
| target scan and trusted data checks | scan, row/provenance, schema, duplicate checks | current retained-table artifacts | retained random/product absence of `sys > 1` | covered | thesis text needs more detailed provenance statement |
| scalar associations and factor tests | Pearson/Spearman, source/facet/product tests, bootstrap/permutation | in-table explanatory evidence | structure in retained table | covered but not proposer validation | scalar effect becomes thesis-facing mechanism claim |
| supervised ranking/regression | ridge, random forest, metadata-only/grouped validation | in-table ranking evidence | ordinary retained-table methods | covered for narrow wording | broad standard-method wording needs lasso/boosting/classifier comparison |
| projection/anomaly | PCA, k-means, isolation-style anomaly diagnostics | in-table diagnostic evidence | visual/structural diagnostics | covered for narrow wording | projection packet shows unexplained high-tail structure |
| generated-candidate scalar proposers | feature-first random-product scalar filters before `sys` | generated-candidate exploration evidence | candidate-proposer claims | active topic | compact 100k packet is boundary evidence; reopen for two-feature rescue or non-product source |
| tail and rare-event models | tail summaries, exploratory fits, parked zero-positive/EVT packet | mixed/tainted | scale-up decisions and negative evidence | parked | scale-up wording matters or HKO taint is repaired |
| HKO reference/local geometry | HKO coverage and parked HKO ridge packet | smoke/mixed | mechanism and HKO-local wording | parked | HKO-local topic explicitly reopened |
| broader distributions and producer variants | height/facet/product/generic variants | limited or deferred | broader random-distribution claims | parked | thesis wants claims beyond current producer contract |
| missing ordinary baselines | lasso/elastic-net, boosting, GAM/splines, density/mixture/one-class variants | mostly not run | broad standard-method closure | audit first | thesis wording needs stronger ordinary-method breadth |

Cheap discriminators:

- Ask surface scouts to produce disjoint longlists of questions, hypotheses,
  and packet ideas, then compare overlap and novelty.
- Run method-surface audits against existing packet READMEs rather than raw
  code first.
- Track "considered but not promising" seeds explicitly enough to avoid
  repeating low-value searches.

Ready packet prompts:

- Standard-method closure baseline. `parked-conditional`.
  Objective: if thesis wording needs stronger ordinary-method coverage than
  the named existing packets, run one tiny retained-table baseline packet:
  lasso/elastic-net, gradient boosting, high-tail classification, and
  feature-family ablation. Do not run models until the exact thesis sentence is
  named. If thesis wording names only the already-run methods, keep this parked.
- Broader random-distribution wording/design scout. `sharpen-ready`.
  Objective: separate exact retained-contract wording from stronger random
  model wording. Consider height interval, generic facet-count range, product
  side range, independent retained-size rerun, Latin-hypercube or
  space-filling height design, and alternative random polytope models. Do not
  run new producers in the first pass; output launch/park/reject status and the
  smallest producer/prepare/method design only if one axis should launch.
- Missing methods audit with EV-style prioritization. `audit-ready`.
  First deliverable: a spawn/rescope/stop recommendation list for the full
  sys-datascience slice, with each candidate method family marked as launch now,
  park, reject for now, or needs topic-owner sharpening. Include why the answer
  would affect thesis wording or future packet choice.
- Claim wording ladder audit. `audit-ready`.
  Objective: separate possible thesis claims into retained-table absence,
  in-table structure, generated-candidate proposer, broader random-distribution
  claim, and broad standard-method coverage. Output should say which existing
  packets support each claim and which missing packet would be needed to
  strengthen it.
- Exploratory packet disposition bridge. `reviewer-ready`.
  Objective: make a compact disposition note for exploratory tail/distribution
  packets so scouts do not reread scratch-oriented READMEs merely to learn that
  they are not thesis-facing yet.

Needs topic-owner sharpening:

- Adversarial review of current method-surface closure wording.
- Topic-seed expansion pass over geometric, statistical, and search-algorithmic
  hypotheses.

Opportunity-cost notes: this work is valuable when it changes which sessions
are spawned next. It should stop before it becomes a large unsourced essay.
