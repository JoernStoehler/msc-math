# Data-science scope: selected source versus confirmed expectation

Prepared 16 October 2026 from this checkout's retained sources. This comparison
completes the agent-preparable scope inventory; **TODO (Jörn): classify overall
scope sufficiency.** It supplies neither whole-chapter acceptance nor a new
experiment. The selected source is `thesis/main.tex` →
[`chapters/08-data-science.tex`](../chapters/08-data-science.tex) and
[`appendices/data-science.tex`](../appendices/data-science.tex), as identified by
[`thesis-work.md`, §5](../../meta-topics/project-management/thesis-work.md#5-data-science-prose-and-evidence-disposition).

## What the confirmed expectation requires

[`project-facts.md`, items 31–31.1](../../docs/project-facts.md#data-science-and-search)
(lines 266–290) distinguish three questions:

1. Applicable feasible methods were to be executed; skipped methods needed an
   actual cost, promise, redundancy or other disposition, rather than a few
   families silently standing for the repertoire.
2. The named retained-table method surface is now closed through executed,
   interface-covered, parked, future-work, rejected and out-of-scope statuses.
3. Whether this satisfies Jörn's intended broader repertoire is unresolved.
   “On the order of 100” is historical context, not an auditable inventory or
   a count to compare with ledger rows. Neither a completion verdict nor a
   numeric shortfall follows from the current ledger.

## Exact-source comparison

The manuscript develops selected findings; the ledger records the wider method
disposition. A method's presence in the ledger does not mean its results are
printed in the chapter, and a ledger status is not a fresh execution receipt.

| Confirmed obligation / method coverage | Present selected treatment | Retained support and boundary |
| --- | --- | --- |
| Explicit input and numerical target | Chapter labels the 4,096 generic and 10,240 product sample; appendix `app:ds-generation` and `app:ds-evaluation` identify sampling, source recovery, original numerical ratios and current capacity checks. | [Closure summary, “Active Data Slice”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-closure-summary.md#active-data-slice). The finite table has no recorded numerical `sys > 1`; this is not an arbitrary-distribution claim. Capacity intervals now cover 14,335/14,336 bodies; floating-point volume/ratio and `random_F8_s3_45` remain explicit limits. |
| Standard descriptive and statistical methods | Chapter's inverse association, conditional rank analyses and Gaussian reference; appendix `app:ds-selection`. | [Disposition ledger, “Baseline And EDA”, “Leakage, Validation, And Null Checks”, and “Statistical Associations”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-method-dispositions.md#baseline-and-eda) names target scan, EDA, Pearson/Spearman, factor tests, permutation/bootstrap and null checks as executed; confidence/error-control variants have explicit interface or park dispositions. These are finite descriptive/diagnostic findings, not a causal law or a calibrated general tail model. |
| Supervised prediction and rules | Main chapter reports ridge-only versus combinatorial-only boosted prediction. Appendix `app:ds-prediction` explains random forest, histogram gradient boosting, features and grouped holdouts, and points to the P2 packet. | [Ledger, “Supervised Prediction And Rules”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-method-dispositions.md#supervised-prediction-and-rules) records ridge, random forest, lasso, elastic net, boosted regression, logistic/boosted high-tail classification, shallow tree rules and feature-family ablation as executed. [P2, “Research Question” and “Current Status”](../../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/README.md#research-question) closes the named missing baselines. The appendix does not print every model's score; the packet retains them. No generated-candidate or threshold-crossing validity follows from this table ranking. |
| Unsupervised, density and anomaly coverage | PCA, k-means and isolation-style anomaly diagnostics are not developed in the selected reader narrative. | [Ledger, “Unsupervised, Density, And Anomaly Methods”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-method-dispositions.md#unsupervised-density-and-anomaly-methods) records these three as executed, with explicit park reasons for factor analysis, hierarchical/mixture clustering, density models, one-class methods and normalizing flows. Their omission from the narrative is visible here; no retained requirement says every executed diagnostic must be printed. |
| Novel random/gradient search remains a substantial method (project fact 30) | Chapter subsections “Using evaluated ratios to guide search”, “Competing branches in local search” and “A failed probe does not establish a local maximum”; appendix `app:ds-local-optimization`, `app:ds-probes`, `app:ds-adaptive`. Named comparisons include diagonal CEM/IID, branch gradients, finite-gap affine model, branch history with/without transition prediction, CMA-ES and coordinate poll. | [Appendix evidence index](../appendices/data-science.tex), `tab:ds-evidence-index`, lines 676–703, routes the search packets. The retained-table ledger excludes the older ascent/endpoint surface from its own slice; this is compatible with the manuscript giving bounded search results separately. Fixed implementations/budgets, admission confounding in CEM/IID, unrecovered historical optimizer executable/full traces and non-maximality of finite endpoints remain stated. |
| Skipped feasible families have an actual disposition | The chapter's geometric selection/deformation story is selective; it does not claim exhaustion of the standard repertoire. | [Ledger, “Disposition Vocabulary”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-method-dispositions.md#disposition-vocabulary) and named rows distinguish interface coverage (e.g. OLS, stepwise selection), low-value parks (e.g. splines/GAMs, SVMs, neural networks), future work (e.g. full MADS/Nelder–Mead, trust-region, Bayesian/surrogate and constrained/hybrid optimization), and rejected/out-of-scope targets. Existing optimizer infrastructure does not convert unfinished families into executed comparisons. |

## Evidence strength and remaining judgment

The P2 packet retains actual [regression metrics](../../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/artifacts/regression-metrics.tsv),
[classification metrics](../../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/artifacts/high-tail-classification-metrics.tsv)
and [feature-family ablations](../../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/artifacts/feature-family-ablation.tsv).
The inspected rows support the named lasso/elastic-net/boosted and high-tail
classifier coverage, the 8,704/5,632 split, and the appendix's rounded ridge-only
versus combinatorial-only scores, 0.887 versus 0.043. P2's interpretation is
retained-table coverage, not validation of a new candidate proposer.

The [P2 reproduction contract](../../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/README.md#command)
and [closure summary, “Current Gate”](../../topics/empirical-studies/sys-datascience/methods/trusted-random-product-closure-summary.md#current-gate)
expose the 39-feature historical versus 45-feature rebuilt-input distinction:
the registered prepared snapshot lacks six required ridge columns. The documented
rebuild, rather than an unchanged default download, is required for P2.
Some detailed tail-rule outputs remain outside the packet tree. This comparison
read retained source/summary/metric files; it reran no producer, model or solver,
and does not audit every execution artifact behind every ledger row.

The [scope owner, §5](../../meta-topics/project-management/thesis-work.md#5-data-science-prose-and-evidence-disposition)
records source concerns as dispositioned while selected-chapter human acceptance
remains unknown. Earlier rejected drafts do not establish rejection of this
selected version. Conversely, later revisions and agent checks do not establish
acceptance. The chapter's final paragraph keeps its conclusions finite: coarse
association, two opposite exact rotation-family laws and improvements under
fixed search budgets; no universal capacity law, general route to local
optimality, or numerical ratio above one produced by the reported random
samples/search comparisons.

**TODO (Jörn, scope only):** choose A, B or C below. This decision is separate
from mathematical/prose acceptance of the exact chapter and does not authorize
additional computation by itself. No option is recommended as Jörn's intended
scope is precisely the missing source fact.

> For the selected data-science chapter `thesis/chapters/08-data-science.tex`
> and appendix `thesis/appendices/data-science.tex`, the retained method ledger
> closes named standard methods through executed or explicit skipped/future-work
> dispositions, while the manuscript reports bounded geometric and search
> findings. Does that satisfy the intended thesis data-science scope?
> **A:** sufficient; **B:** sufficient with the stated finite-sample,
> unfinished-method and reproduction limits; **C:** one specific additional
> obligation is needed before submission (name the method/result or exposition
> obligation in one sentence). This unblocks the scope classification in
> `thesis/DISTANCE-TO-SUBMISSION.md`; a C answer enables a bounded follow-up
> specification rather than automatically launching experiments.

## Check performed

Compared the cited selected TeX passages and scope owners with the retained
ledger, P2 contract and three P2 metric TSVs; checked the linked local files and
heading anchors. This Markdown comparison changes no manuscript claim or TeX
input. The integrated PDF build is recorded by the receiving coordinator.
