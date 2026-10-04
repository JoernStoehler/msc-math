# Independent DS claim and evidence review

Observed 2026-10-04, 01:32 UTC. Owner: `/root/acceptance/ds_claim_evidence`;
receiving owner: `/root/acceptance`. Read-only review of manuscript and retained
evidence; this report is the only owned repository output. The earlier
`thesis-restart-2026-10-04/` acceptance bundle was not changed.

## Verdict and actionable findings

No high or medium finding in the checked central statistical claims. The
retained rows independently reproduce the chapter's association, conditional
selection, balancing and optimizer comparison numbers. Two low evidence-interface
issues were sent to the receiving owner; integration owns any repairs.

1. **Low — stale route from the reader-facing evidence index.** Appendix A's
   source-packet table (`thesis/appendices/data-science.tex:672`, PDF p.95)
   points to `docs/ds-evidence-closure/search-account.md`. Lines 51–56 of that
   document describe the optimizer account as existing at the absent
   `thesis/08-black-box-datascience-finite-budget-optimization.tex`, including
   a compute plot and central `f(x,y)=y-|x|` example. The selected account is
   now `thesis/chapters/08-data-science.tex:516` and Appendix A; those two
   described elements are not in the selected chapter. Preserve the dated
   audit as historical where appropriate, but identify the old location as
   historical or update the route and description to the current account.
2. **Low — prediction subsection promises more specification than it gives.**
   `thesis/chapters/08-data-science.tex:126` promises feature lists and split
   definitions in Appendix A. The boosting comparison at
   `thesis/appendices/data-science.tex:174` (PDF p.86) only states that its
   feature list and split differ from the random forest. The actual P2 packet
   gives 27 count/incidence features and 18 ridge features, with 8,704 training
   and 5,632 test rows; test groups are product facet counts 8, 10 and 12 plus
   generic facet count 12. Either add this concise specification and route the
   complete feature names to the packet, or narrow the chapter's promise.
   This does not invalidate the reported prediction scores.

## Independent numerical checks

All calculations below used fresh, short standard-library Python snippets that
read retained files and printed results. They did not call the producers or
capacity evaluators, write retained outputs, import the producer's ranking
implementation, or regenerate figures. The independent rank implementation
sorted indexed values, assigned average ranks to ties, and used
`statistics.correlation`; conditional ranks were recomputed within each
retained group exactly as the chapter specifies.

| Claim and reader location | Fresh check | Result |
| --- | --- | --- |
| Broad ridge association, chapter lines 68–85; PDF pp.54–57 | Join 14,336 unique table identities to unique provenance; reconstruct 18 groups and compute ranks | Eight groups of 512 generic bodies and ten of 1,024 products. Pooled Spearman −0.9384368672, Pearson −0.2050393154; group Spearman range [−0.9971023398, −0.8409437728], matching rounded prose. |
| Conditional attenuation, chapter lines 87–108; PDF pp.54–55; Appendix A pp.86–87 | Recompute all 14 selection/fraction combinations from original rows; compare to `conditional-ranks.tsv` | All counts match; maximum coefficient difference 1.12e−16. At 10%, high-ratio −0.2864220229 and low-ridge −0.2215426132; at 5%, −0.2390470216 and −0.1127107347. The within-group full-panel coefficient is −0.9440234832, distinct from the raw pooled coefficient above. |
| Balancing, chapter lines 383–416; PDF p.61; Appendix A p.89 | Join 640 distinct successful evaluator results to geometry; verify input identities, raw-input hashes, capacity containment and numerical ratio formula; pair by source body | 320 pairs; mean ratio 0.3418878631 → 0.6536831816, with 314 increases and six decreases. Mean ridge score 23.7388334491 → 11.6523466729, with 306 decreases. Pooled within-group correlation −0.9289222874 → −0.2486803519. Balanced triangle group +1, other nine negative, matching all reported limits. |
| Balancing geometry | Independently reconstruct exact rational planar vertices from the binary64 dual inputs for pair index 0 in every group, in both arms (20 bodies); calculate mixed-edge absolute dot sums and exact factor areas | Every factor has its specified number of vertices. Maximum ridge-score discrepancy 3.56e−15; maximum volume relative discrepancy 2.23e−16. This is a selected geometry check, not 640 new capacity or certified volume evaluations. |
| Finite optimizer comparison, chapter lines 528–556; PDF p.63; Appendix A pp.89–91 | Recompute medians and interpolated 10th/90th percentiles directly from 448 retained `runs.jsonl` rows, grouped by algorithm | All seven table rows match. Four-anchor median 0.9847831937, range 0.9570332088–0.9985145750; maximum across all runs 0.9998262445, zero at or above one. There are 64 starts, 28,824 physical evaluations, and 28,376 charged calls; the difference is 448 initial evaluations. The appendix's physical-call count is consistent. |

## Figure and adjacent evidence interfaces

- The two association figures resolve to `thesis/figures/plot_association.py`.
  Its input hashes match the table and provenance used above; it displays all
  14,336 bodies in the stated groups. No body lies outside its fixed axes:
  observed ridge scores are 8.9259–8866.9894 and ratios are
  0.0000020328–0.8625858958, within x=[8,10000], y=[0,0.9]. This checks the
  input/plotting specification, not a fresh rendering of the figure binaries.
- The selected `conditional-ridge-correlation.pdf` is byte-identical to
  `selection-mechanism/artifacts/conditional-ranks.pdf`; the independently
  recomputed numerical series matches that producer's retained table.
- Prediction figures in prose match retained outputs: random forest
  `prediction-ranking/artifacts/summary.json` gives R²=0.8851660, 39 features
  and 8,192/6,144 rows; P2 `feature-family-ablation.tsv` gives ridge-only
  R²=0.8872276 versus count-only 0.0431470 on 8,704/5,632 rows. This was output
  and feature-definition inspection, not model refitting or fresh predictions.
- The 100,000-product scalar-selection packet's retained summary contains
  0.6261166821 versus 0.3155039434 for the named per-group low-ridge rule.
  Its README distinguishes the 1,675 evaluated union from the unevaluated
  100,000-body pool and identifies omitted large geometry/feature caches.
  The covariance validation packet retains its frozen target list and exact
  evaluation output, while documenting omitted full candidate caches. This
  review does not independently authenticate historical pre-target chronology
  or regenerate either complete candidate pool.
- The three adaptive/IID table rows match `diagonal-cem-pilot/artifacts-supported-v2/analysis.json`.
  The manuscript explicitly preserves the differing coordinate/admission
  filters, shared initial population and exact-array deduplication. It does
  not call this an isolated causal benefit of adaptation.
- Chapter and conclusion preserve the material limitations: conditional
  association is descriptive, balancing changes multiple properties, model
  scores do not rank differently tuned families, numerical volumes are not
  certified, and finite endpoint probes do not establish local maximality.

## Artifact identity and boundaries

Reviewed manuscript hashes:

| Artifact | SHA-256 |
| --- | --- |
| `thesis/chapters/08-data-science.tex` | `4c7e4436641d8b01c7819cc1cdbae8f21a73cc71fac7fc36c8efbc148aecaa00` |
| `thesis/appendices/data-science.tex` | `fdbb0fb8af8580cf9f17463f6f06e23e5611f432fec0b935e95a094c4cb4f1e7` |
| Earlier accepted PDF used only for page pointers, `docs/history/thesis-restart-2026-10-04/thesis-candidate.pdf` | `27dbf152964bf3eeb6a15bf5f319ab4326de14d48a40e2e296f6008eff3e2f70` |
| Historical invariant-table snapshot | `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59` |
| `experiments/polytope-invariant-table/polytope-provenance-table.jsonl` | `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2` |
| Balancing `inputs.jsonl` | `fc063edaa6ae301679dd45151aa9ec385b8a2b00bd5aa7f007cd4825523817f6` |
| Balancing `geometry.jsonl` | `81feeed2b2359a4706cd49dfa0d7db7311c88a8aab29f554d5ccc4739ab81e77` |
| Balancing `pilot-results.jsonl.gz` | `645f791e8f0f127c2b04242e223bd52d04ffbaaf10c1c9c75305ce855309de58` |
| Balancing `remaining-results.jsonl.gz` | `9eb5b7494cba28829e419d8021e5b625a8c4877ccc6f87066f40216e482ec55b` |
| Optimizer `heldout-f10-64-finalists-19a8b4dfd/runs.jsonl` | `2f0a7820dac3eae0a0b7335924c3512e0a0d463202a667e7635c7232daed0dd7` |
| Conditional rank summary | `86c0629667782b72c85b0efc0ab2fcc26d2ca928943e80fe2dcac9cd9ba7db61` |
| Selected and producer conditional figure | `912e9bef768cff74c26a09d453e6ab4c56a56af2fca4152919aa5f72e6b4bdff` |
| Selected generic association figure | `85898c620583f52a1d28b056dfc60faffb81e73a9a8a86ba7d517c3d586154f4` |
| Selected product association figure | `fbbe2c12b98d5c93764a6dbf949a6f30402190c01c31dab552063bd00f85b572` |

The invariant table was read from the registered local cache under snapshot
`c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a`.
Input hashes and agreement establish which retained bytes were checked; they
do not recover original executables, prove original generator execution, or
certify historical optimizer proposals. No full baseline, capacity producer,
model fit, Gaussian simulation, or expensive certificate was rerun. Exact
pentagon/triangle theorems and all prospective raw selection rows were not
reproved or exhaustively audited in this assignment. PDF page extraction was
used for locations; this is not a new visual acceptance pass.

## Route and value judgment

The strongest supported DS deliverable remains the current bounded scientific
account: a broad association, loss of ordering after selection, useful named
geometric interventions, and finite-budget local improvements with explicit
evaluator limits. The fresh independent aggregations support keeping that
account. The two small interface repairs are better justified than new method
execution or replacing the current claims with weaker generic prose.

Project fact 31.1 leaves satisfaction of Jörn's historical roughly-100-method
expectation uncertain. This review neither declares it complete nor converts
the approximate count into a numeric deficit or execution gate. No missing
supported result identified here is required to alter the selected scientific
conclusions. Human PASS and overall thesis completion remain separate unknowns.

Status: review complete; two low findings handed to acceptance/integration.
No source or evidence edits, no running computations, and no child assignments.
