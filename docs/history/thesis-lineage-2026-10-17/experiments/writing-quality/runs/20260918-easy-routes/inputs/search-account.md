# Search and data-science account beyond the ridge identity

Source audit, 2026-09-18. This document proposes a scientific organization, not
historical chronology. It reports existing evidence; no producer was rerun.
Paths below are repository-relative. Packet READMEs determine interpretation,
and their generated artifacts determine numbers. Overall DS completion remains
an unresolved thesis-scope decision; this audit does not settle it.

## A scientific account a writer can use

There are two complementary search questions. Can cheap geometric descriptions
identify promising bodies before expensive capacity evaluation? Can local
refinement exploit the branch structure of capacity to improve random starts?
The retained work gives bounded positive answers to both, although neither
found a new positive source. A chapter that says only “many methods failed to
find counterexamples” would discard its most useful results.

The first question starts with a finite population of random polytopes. It
requires distinguishing a relationship observed after computing the target
from a rule that selects new candidates without knowing their targets.
Ordinary models and feature ablations show where the association lives;
frozen scalar selectors then test whether that information can actually enrich
new candidate pools. The second question uses local branch information rather
than population correlations. Its comparatively high attained values show why
one cannot identify the retained random-table maximum with a limit of the
search machinery. However, evaluator lineage differs between packets, so their
numerical maxima are not a controlled head-to-head method comparison.

The evidence also distinguishes explanations from useful correlations. Bounce
labels, capacity/volume decomposition and the two-bounce width formula clarify
part of the retained product structure. Explicit paths where a favorable ridge
feature worsens the target show why that feature is not a universal ascent
objective. This supplies both successful structure and instructive failures,
without pretending the complete mechanism has been identified.

## Claims ready for a bounded scientific account

| Result | Actual evidence and limits | Source owner |
|---|---|---|
| Finite random baseline | 4,096 generic rows and 10,240 products; no stored target above one; maximum 0.86258589584944. One seed and one support-height interval, with named facet/polygon ranges. Numerical target lineage must accompany mathematical interpretation. | `experiments/sys-datascience/README.md`; `methods/scan-sys-gt-1/`; `experiments/polytope-datasets/retained-lineage.md` |
| Ordinary feature methods | P2 runs lasso, elastic net, boosting regression/classification and family ablations; associated packets cover ridge/RF, rules, projections, clustering, anomalies and association screens. 45 active invariant numeric features, grouped holdout by source/facet count. Ridge-area features carry most of the held-out signal. This is not generated-candidate validation or exhaustive coverage of all methods. | `experiments/sys-datascience/methods/standard-baseline-p2/README.md`, its `artifacts/`, and `trusted-random-product-method-dispositions.md` |
| Selection before target evaluation | In the original 100k product-candidate packet, 30 selection sets yielded 485 unique selected candidates and a 1,675-row selected/control union. Only the union received target evaluations; maximum 0.867546058507634 and zero above one. A separate fresh 100k concentration validation passed its predeclared incremental-enrichment criterion. Generated count must not be reported as capacity-evaluated count. | `methods/extreme-scalar-rejection-proposer/README.md`, `artifacts/100k-promising-scalars/`, `artifacts/100k-ridge-concentration-validation/` |
| Transfer to a different named source | Frozen rho and ridge selectors enriched mean target versus disjoint controls in both `4x6` and `6x6` buckets on one separately area-normalized `factorial-both` source. 91 distinct evaluated targets, five overlaps, no positives. This is finite-design sub-threshold enrichment; the packet's `strong_transfer` label is not a population theorem or superiority claim. | `methods/alternative-source-transfer/POST-TARGET-ACCOUNT.md`, `artifacts/transfer-v1/analysis.json` |
| Symplectic orientation matters for a selected body | On the selected product champion, 164 compact-orientation evaluations improved 0.862586 to 0.878308. The selected generic champion did not improve. A further 216 noncompact points per body yielded no further improvement. Two post-selected bodies establish a witnessed effect and a sparse negative diagnostic, not a population frequency or optimum. | `experiments/sys-landscape/fixed-shape-orientation-search/README.md`, `analysis.json`, `global-analysis.json` |
| Branch-aware finite ascent works on a fixed panel | Twelve F10 runs accepted all eight moves, mean stored-target gain 0.011565, total measured compute 400.889s. Every run hit the cap and every endpoint scan still found an improving move. Historical schema-v1 legacy branch-selection evidence, not evidence for the migrated v2 implementation. | `experiments/sys-landscape/gradient-ascent-observed-general/artifacts/summary.json` and README |
| Branch history outperformed named optimizer implementations | Seven fixed policies on 64 matched F10 starts, 448 runs. Four-anchor branch history median best historical evaluator target 0.984783 (10–90% across starts 0.957033–0.998515); next transition prediction 0.981647 and finite-gap model 0.977159. No run reached one. Same nominal 1000ms start-new-work cutoff/128-call cap, not equal realized time. Manifest says held out but full separation from tuning is not established. | `experiments/dev-gradient-ascent/optimizer-comparison/README.md`, `artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/final-summary.csv` |

All `methods/` paths in this document abbreviate
`experiments/sys-datascience/methods/`.

The optimizer account already exists in
`thesis/08-black-box-datascience-finite-budget-optimization.tex`, including the
branch-envelope motivation, performance table, compute plot and endpoint
controls. Its central explanatory example is ascending the ridge of
`f(x,y)=y-|x|`: following either selected branch gradient differs from balancing
both branches. The account explicitly denotes the historical objective by
`hat(sys)` because candidate-family completeness is untested. A shared evaluator
can make different errors at the points each optimizer visits; its ranking is
not established for certified mathematical capacity. The runner permits an
atomic final round to overrun the nominal cutoff; 310/448 terminal charged
compute totals do so. Do not compress this to “seven methods compared for
exactly one second”.

## Product structure beyond a scalar correlation

`methods/product-bounce-mechanism/README.md` describes a 10,240-row post-target
analysis. Within polygon buckets, the three-bounce-label log-target contrast
is 0.8452, splitting exactly into 0.2086 from twice log capacity and 0.6366 from
negative log twice volume. Generator and volume-free ridge-distribution controls
change both magnitude and decomposition. This is descriptive bookkeeping using
an identity, not causal mediation.

Two proposed explanations fail useful checks. Both label classes have six-facet
global-winner supports, exactly 3q+3p, so the simplest “one label leaves more
facets inactive” explanation fails. The within-bucket upper decile has fewer
near-ties between the two- and three-bounce class minima, not more: 11.3% versus
21.1% at the packet's 0.01 log-gap cutoff. A general HKO-like near-intersection
explanation therefore does not fit these retained rows.

The width shortcut supplies genuine geometry:
`W2(P,Q)=min_{d in boundary(P-P)} h_{Q-Q}(d)`.
Its current proof owner is `formal/product-two-bounce-class.tex`; the packet
calls it agent-reviewed, not Jörn-reviewed. Its independent exact implementation
agrees with retained A2 on 20 deterministic rows plus three analytic fixtures.
The 10,240-row association analysis substitutes retained target-derived A2; it
is **not** a 10,240-row independent geometry verification. Across four adjustment
sets, the positive capacity contrast is more than accounted for by the
higher two-bounce geometry term, with lower-envelope takeover attenuating it.
This explains one component without predicting whether A3 beats A2.
Sources: `methods/product-bounce-width-shortcut/README.md`,
`artifacts/summary.json`, `artifacts/retained-association.json`.

## Remaining scientific choices and concrete gaps

1. **Choose the thesis's evaluator claim.** Historical random/search results
   can truthfully remain evidence about the recorded numerical objective.
   Promoting optimizer rankings to the mathematical sys requires a new
   validation design, not a prose substitution or a current-implementation
   smoke. Re-evaluating only final winners also does not validate the old
   trajectory/algorithm decisions. This is the largest possible new-work
   dependency in the search account.
2. **Select how much bounce structure earns its space.** The two-bounce
   theorem review status needs resolution before thesis theorem use. Full
   geometry validation of all 10,240 rows was stopped and is not required merely
   to report the 20-row validation. The missing general A3 mechanism is a
   substantive open question only if the chapter promises it.
3. **Integrate existing optimizer evidence before commissioning another
   optimizer.** The current question map emphasizes the older 12-run panel and
   warns that it does not fully reconcile later packets. A reader-facing account
   that omits the 448-run comparison would be materially incomplete despite an
   exhaustive scalar-method inventory.
4. **Pick a few informative displays.** The existing optimizer compute figure,
   a clearly stratified random/ridge display, and a small generated-selector
   versus control table can each answer a different scientific question.
   Exact selection of the latter two remains an authoring decision; more plots
   of the same table are not new empirical closure.
5. **No numerical method quota.** Project facts 30–34.2 require a substantial
   random/gradient account and honest dispositions. Fact 31.1 explicitly says
   the approximate 100-method historical expectation is not a count gate. The
   ledger contains real unimplemented families, including MADS/Nelder–Mead,
   trust-region and calibrated Bayesian methods. Their existence alone does
   not identify necessary thesis work; compare their likely scientific yield
   against the claims the thesis intends to make.

A practical minimum account can therefore be written from retained evidence:
finite random baseline → invariant association → frozen generated selection
→ local branch-aware optimization → geometric interpretation and demonstrated
limits. These arrows are a pedagogical sequence, not a claim about discovery
chronology or that every later method was caused by an earlier result.

## Audit depth

Read packet-local READMEs and project facts, the optimizer thesis subsection,
its generated final-summary CSV, and the twelve-run generated summary. Other
numbers above are packet-reported rather than freshly recomputed from raw rows.
No capacity producer, data rebuild, expensive test or external literature search
was run. This is a source-grounded scientific handoff, not independent
recertification of every experimental result.
