# Independent source acceptance — 4 October 2026

Owner: `/root/acceptance`, independent of operational integration. The accepted artifact and final review identity are bound by [acceptance-manifest.json](acceptance-manifest.json) and [acceptance-pdf.md](acceptance-pdf.md). Review starts from base `52b1526c8ff5613a400f2f4028b4290f35841b21` through preserved checkpoint `fe3475b3`; final source identities are recorded in the candidate manifest. No human PASS, complete formal verification, university submission or public release is inferred.

## Findings and disposition

| Severity | Finding | Status and acceptance boundary |
| --- | --- | --- |
| High API-contract defect | `general_qp_action_window` inherited curvature pruning that preserves global maxima but can remove positive stationary words inside a wider action window. Before correction, the current-code hypercube reproduction returned only 2 of 142 exact-positive words at the complete spectrum endpoint. | Repaired and independently accepted; see [numerical review](acceptance-numerical.md). This establishes wider-window undercoverage, not an incorrect scalar capacity. Historical development report's 128 pruned positives is a different measurement. |
| Medium evidence navigation | Non-HKO PROOF/RESULT historical `evidence/` references were not direct checkout paths. | Repaired and inspected: README/PROOF/PROVENANCE identify the nested archive and the later verification directory. Fresh exact predicates and the separate independent audit passed; retained evidence is present, not lost. |
| Low navigation | Visualization README references absent former thesis paths; retrospective target-full README omits later 90-body repair. | Repaired and inspected: current Chapter 12 path, Git custody for former prose, and explicit sibling 90-body repair route. Original observations and failures remain historical. |
| Low helper defect | Singular flow-tube solver skipped contradictory zero coefficient rows before classifying a fixed line. | Repaired and independently accepted; genuine-polytope reachability was not established. See the numerical report. |
| Low PDF fidelity | A near-empty contents page and ligature-converted command dashes impeded the assembled reader artifact. | Disposition and exact rendered identity are recorded in the PDF report. |
| No new mathematical blocker found | Changed foundations, product reduction, variation and HKO/non-HKO implication interfaces. | Bounded source review only, described below; final PDF review separate. |

## Fresh mathematical/source coverage

The acceptance reader independently checked the Chapter 13 cube example using Python `Fraction`, enumerating all 24 support/order values: 20 zero, two `1/8`, two `-1/8`; the winners are `(0,4,2,6)` and `(1,5,3,7)`, with action 4. This agrees with the retained `adversarial:square_product_exact_zeros` row in `experiments/dev-quadratic-program/tools/product_closure_route/sample5.jsonl`.

The newly named DS maximum was joined by `poly_id` between the provenance table and all 14,336 compressed comparison rows: `random_5x6_s3_168`, id `14ecfbc81f34240a8cced0e48bc6875015f1cfa6ac8d156094b624ff05f44444`, legacy ratio `0.86258589584944`, current numerical ratio `0.8625858958494405`, both capacity scalars `4.821447361515128`, interval `[4.821447361515127,4.821447361515128]`. The 14,335 interval coverage is 14,289 accepted scalars plus 46 wider intervals; `random_F8_s3_45` remains unresolved. Capacity certification does not certify volume or the full ratio. The manuscript preserves these boundaries.

The non-HKO upper-section argument was checked through its mixed-direction estimate: a positive gradient relation of rank 22 gives linear decrease when `||y|| >= M z²`; weighted negative `zz` curvature gives decrease in the complementary region. Constant rank uses identities along the two-parameter equality family, not only a base rank. Adding 15 symmetry directions yields a full 40-dimensional chart. The printed `-13/432` is curvature of feasible upper sections, not of actual capacity. Equality and fixed-ten-facet scope match the product calculation and analytic argument. The acceptance reader reran the complete exact verifier to a fresh temporary directory in 12.53 seconds: all checks passed, including 624 product orders, 24 sections, ranks 22 and 40, rational-family identities and the displayed curvature. `RUN.json` binds exact source hashes; the retained copy is [non-hko-acceptance/exact/](non-hko-acceptance/exact/), covered by [verification-manifest.json](verification-manifest.json). The separate independent triangulation/direct-solve run in [non-hko-verification/independent.json](non-hko-verification/independent.json) recomputed 960 gradient entries and 48 simplices, with 12 tight cases and gap 1/14. This recomputes algebra and does not replace the analytic implication.

A separate read-only mathematical reviewer (`/root/acceptance/mathematical_review`, report at 00:36:14 UTC) found no high/medium defect in:

- Chapters 02–04: Hausdorff continuity sandwich, boundary recovery, simple-minimizer dependency, billiard lift and block surgery, six-facet reduction. Existence and classification are distinguished.
- The Rudolf dependency: official journal Definition 2 and Theorem 1 supply the strong-billiard formula for arbitrary convex factors and at most three bounces in dimension two; the manuscript reverses the opposite characteristic convention consistently. Source: <https://link.springer.com/article/10.1007/s10884-022-10228-0>.
- Chapter 06: KKT nonsingularity and implicit-function continuation under stated constraint-rank and definiteness hypotheses.
- Chapter 07: finite-certificate to neighborhood reasoning, including non-HKO continuation, positive relation and symmetry chart.
- Chapters 09–11: changed reduction explanations and continuity at the rotation threshold; full older packets were not rerun.
- Chapter 13 scalar-pruning logic and cube counts. All 88 retained product-audit records sum to 1,835 weights, 173,496 objective intervals, and 23,559 raw sign mismatches, with zero recorded interval/ternary disagreements.

Prior source reports retained in `../2026-10-04-thesis-run/custody.md` are reused only at their stated HKO/products/summary scope. Their PDF snapshots are not accepted as the final PDF. No historical clean build or finite numerical comparison is promoted to unrestricted mathematical correctness.

## Required-area coverage map

The exact-PDF companion adds physical PDF pages and fresh visual-review depth. Presence in this map means the area is represented and its claim boundary was inspected, not that all prose and supporting code received exhaustive review.

| Project-facts item 8 | Selected source | Evidence owner and claim boundary |
| --- | --- | --- |
| 8.1 HKO local result | Chapter 07 and HKO fragments; Appendix B | `experiments/hko-local-maximum/theorem/`; non-HKO witness/verifier/PROOF. Fixed ten facets; explicit equality families; algebra plus neighborhood argument. |
| 8.2 Pentagon product side result | Chapters 09–10 | Selected analytic rotation proof; `formal/pentagon-affine-products/`; earlier Sage certificate has separate scope. |
| 8.3 Search/data science | Chapter 08; Appendix A | `experiments/sys-datascience/methods/`, retrospective comparison/repair; finite samples, historical-target provenance and volume caveat. |
| 8.4 Generalized Reeb/HK finite foundation | Chapters 03–04 | Selected definitions, HK formula, realization and product reduction; `formal/hk2017-qp-*`. |
| 8.5 First-order perturbation | Chapter 06 | Feasible upper sections, regular KKT branches, envelope caveats; separate finite implementation checks. |
| 8.6 Numerics/exactness | Chapter 13 | General/product development audits; production correspondence; exact action-window repair reviewed separately. |
| 8.7 Code/data/reproducibility | Chapter 14 | INSTALL, artifact contracts and producer runbooks; plain checkout differs from external artifacts and historical public revision. No clean-environment reproduction asserted. |
| 8.8 AI artifacts | Disclosure; Chapter 15 | Attributed firsthand account and contribution boundaries; historical personal proofreading is not current final acceptance. |
| 8.9 Visualization | Chapter 12 | `experiments/visualization/`; selected figures are explanatory computed views, not theorem evidence. |
| 8.10 CH2021/flow graph | Chapter 05 | `formal/flow-graph-*`, `crates/symplectic/src/algorithms/flow_graph/exact_search.rs`; conditional regularity theorem and finite implementation evidence. Fresh theorem/genericity/runtime correspondence review completed; see the numerical report. Singular helper repair is outside the theorem’s regularity class. |
| 8.11 Preliminaries | Chapter 02 | Normalized facet rows, convex/symplectic conventions, EHZ continuity; bounded source review above. |

## Acceptance outcome and limits

No high- or medium-severity source finding remains open within this review. The two code repairs, evidence navigation and final assembled-PDF inspection have independent acceptance records. A literal local-path check found all 52 source occurrences (47 distinct paths) resolving; this does not materialize external artifact datasets or validate every linked file. The all-eleven-area physical-page map, rendered inspection depth and claim transitions are in the PDF companion.

This is an independent agent acceptance of a bounded candidate, not Jörn’s PASS. It does not certify every mathematical proof, every citation’s complete contents, every implementation input, statistical generalization, clean-environment reproduction, missing historical execution provenance, or a public frozen release. The unresolved DS body, floating-point volume, historical optimizer provenance and external-data requirements are disclosed in the manuscript. No further concrete consequential correction emerged from the final sequential reader pass; broad new research or rerunning the full empirical population was not justified by the findings.
