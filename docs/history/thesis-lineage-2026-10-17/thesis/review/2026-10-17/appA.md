# Appendix A adversarial review — 17 October 2026

## Scope and candidate

Independent reviewer `/root/review_appa` inspected the selected
`thesis/appendices/data-science.tex` (all 702 lines), its relevant definitions
in Chapter 8, and the Appendix A text of the frozen current candidate:
`/tmp/msc-math/thesis3/20261017/baseline.pdf`, SHA256
`76fb3ac4e48d10c0175b1f41f5977ae57fd0ee95a272467b422b2ad8d24dd236`.
Page locators below are the printed baseline pages, which equal physical PDF
page numbers. Appendix A spans pp. 88–100. Pages 88–89 were additionally
rendered and visually inspected; their mathematical symbols and references
are legible. No source edits, producer runs, model fits or capacity evaluations
were performed.

## Findings and severity

**No blocking mathematical defect, numerical mismatch or broken cross-reference
was found in the inspected material. No manuscript fix is proposed.** This is a
bounded independent review, not full re-execution of every retained analysis.

| Severity | Pages; source lines | Adversarial check and outcome |
|---|---|---|
| No defect found; mathematical review remains valuable | 88–89; lines 10–84 | Re-derived the equilateral-triangle capacity proof: closure leaves weights x,y with x+y=1/3; cyclic words have one, two or three runs; the two-run objective is 2u·v≤2; the alternating word attains 2(c0−c1); x=y=1/6 gives the printed capacity. Checked the ridge numerator, normalization, endpoint values, symmetry reduction and isotropic-triangle Gram identity. Their factors and signs agree. |
| No mismatch found | 89–90; lines 109–153 | Geometry summary supports 14,336 rows and 645,120 matching cells. Target summary supports 14,245 successful evaluations, 54+37 refusals, all three rounded maximum relative deltas and 5,583 historical capacities outside new intervals. Repair summary supports 37 interval-only cases plus 53 norm repairs, including 44 new accepted scalars: 14,245+44=14,289 scalars, 37+9=46 wider intervals, one unresolved. Floating volume and incomplete historical-execution provenance remain explicitly caveated. |
| No mismatch found | 90–91; lines 158–204 | Random-forest summary supports 39 features, 8,192/6,144 rows, R²=.885166 and MAE=.049459. P2 ablation TSV supports the different 8,704/5,632 split, ridge-only R²=.887228 and counts-only R²=.043147. The text correctly prevents interpreting these as a head-to-head model comparison. |
| No overclaim found in checked packet comparisons | 91–93; lines 232–404 | Checked the conditional-tail narrative, fitted Gaussian reference, paired height-change results, orientation allocation and covariance-balancing result against their packet accounts. Numerical outcomes agree at the printed precision. Conditioning, post-selection, fixed panels and control dependence are preserved; no confidence or causation claim is promoted from them. |
| Residual evidence limitation already disclosed | 94–98; lines 409–608 | Retained final-summary CSV and analysis validation support seven policies, 64 starts, 448 runs, 28,824 full evaluations and the three reported medians. The opening paragraph explicitly says that tuning/held-out separation is unresolved, and the body uses the historical numerical objective rather than certified capacity. Endpoint failures and finite-basis misses are not called local maxima. No new rerun is needed to keep this bounded claim. |
| No mismatch found | 98–99; lines 612–674 | Supported-v2 CEM report supports 1,344 distinct requests versus 1,536 arm memberships, the three fixed seeds, all successes and timing totals. The source revision, coordinate-dependent admission and unequal costs are explicitly disclosed. |
| No broken local target found | 88–100; all six reference occurrences and all repository-qualified path occurrences | Every Appendix A LaTeX reference has a source label. Every repository-qualified printed path exists, including Table 3 packet owners. The abbreviated tail path is qualified by the methods directory in the surrounding text. |

## Essential Jörn reading, ordered by risk

| Priority | Exact pages and claim | Why personal verification is useful | What “OK” looks like | Minutes |
|---|---|---|---|---:|
| 1 | pp. 88–89, A.1, exact formulas c(Tθ)=3√3/[4cos(π/6−d3)] and sys(Tθ)=½sec²(π/6−d3), with R=8√3cos d3 | This is an unrestricted analytic statement for the triangle rotation family, not a finite diagnostic. The critical steps are exhaustive cyclic-run classification, equality attainment and reduction of all angles. | Jörn can reproduce the one/two/three-run upper bound and the attaining word, agrees with the Q-to-capacity normalization, and accepts the angle symmetry and isotropic-triangle implication. This simultaneously settles Chapter 8’s triangle-family interpretation. | 15 |
| 2, can be combined with Chapter 8 acceptance | p. 94, A.8 first two paragraphs, optimizer comparison is about the recorded historical objective with unresolved tuning separation | Exact historical objective and provenance limits materially affect how the numerical ranking may be described. This is an authorial acceptance decision, not a demand for new computations. | Jörn accepts reporting this as a bounded implementation comparison and does not read the medians as certified capacity superiority or a fully clean held-out test. | 3 |

No personal line-by-line rereading of the remaining numerical appendix is
identified as necessary by this review. Its uncertainty warnings should remain.
The 15-minute proof item is the substantive Appendix A reading obligation; the
3-minute objective-scope item can be shared with the Chapter 8 review.

## Inspected supporting evidence

- `topics/empirical-studies/evidence-closure/triangle-profile.md` and Chapter 9’s printed Q convention.
- `topics/empirical-studies/retrospective-revalidation/{geometry-full/comparison.json,target-full/summary.json,repair/comparison.json}`.
- `topics/empirical-studies/sys-datascience/methods/prediction-ranking/artifacts/summary.json`.
- `topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/artifacts/feature-family-ablation.tsv`.
- `topics/empirical-studies/sys-datascience/methods/tail-dependence-feasibility/README.md`.
- `topics/empirical-studies/sys-datascience/methods/selection-mechanism/{gaussian-result.md,controlled-result.md,balance-result.md}`.
- `topics/empirical-studies/sys-datascience/methods/{paired-tangentialization,orientation-allocation}/README.md`.
- `topics/empirical-studies/sys-datascience/methods/diagonal-cem-pilot/supported-v2-report.md`.
- `experiments/dev-gradient-ascent/optimizer-comparison/README.md` and its `heldout-f10-64-finalists-19a8b4dfd-analysis/{analysis.json,final-summary-by-facet.csv}`; counted the corresponding retained 448 `runs.jsonl` rows.
- `topics/empirical-studies/evidence-closure/search-account.md`.

The full raw optimizer proposal/diagnostic surface, every scalar selector and
every resampling calculation were not independently replayed. Numerical summary
agreement is reported only for the supporting objects actually read above.
