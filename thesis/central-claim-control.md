Status: thesis-wide content companion for central thesis claims. Not source
truth.
Purpose: make the next central thesis writing packet possible without hidden
overclaim or live chat clarification.
Overruled by: active thesis files, owner-local notes, experiment artifacts,
formal proof files, official sources, and accepted Jörn/Kai decisions.
Origin: copied from `/tmp/central-claim-control-packet.md` on 2026-06-01 after
review and fixes. The `/tmp` copy is disposable; this file is the durable
thesis-working surface.

# Central Claim-Control Packet

Scope: only `abstract`, `introduction`, `hko-local-maximum`,
`black-box-datascience`, and `conclusion`.

Stop rule: use this packet to start the central TeX drafting packet plus the
named prerequisite packets below. Do not expand it into a whole-thesis map.

Allowed review gates:
- `none: caveated/status wording`
- `Jörn math correctness`
- `Jörn thesis-scope/taste`
- `Kai theorem framing`
- `advisor/admin external fact`
- `cut/defer acceptance only if retained wording no longer depends on it`

## Claim Rows

| surface | claim | must-have area | support source | support strength | caveat/fallback | paragraph placement | next action | review gate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `abstract.tex` | HKO2024 is locally maximal among ten-facet polytopes up to translations, scaling, and linear symplectic maps. | HKO local result | `experiments/hko-local-maximum/README.md`; `experiments/hko-local-maximum/theorem/`; `formal/hko-feasible-section-upper-branches.tex` | exact feasible-section certificate verifies; formal implication line-checked and Jörn quick-reviewed | If final theorem wording/Kai review does not accept theorem-strength phrasing, say exact verified certificate plus line-checked implication, not final theorem. | result sentence | Draft only after final HKO theorem wording is settled. | `Jörn math correctness`; `Kai theorem framing` |
| `abstract.tex` | Product of two rotated regular pentagons gives the named positive structured `sys>1` result/formula. | pentagon product result | `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`; `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`; endpoint/symmetry sources: `formal/combinatorial-boundary-regularity.tex`, `formal/lagrangian-product-rotation-symmetry.tex`; routing: `experiments/regular-products/README.md`; drafting guide: `thesis/rotated-regular-polygons-content.md` | exact proof packet exists for the open half-domain; endpoint and mirror steps are ordinary thesis-writing arguments | Do not claim global classification beyond the checked pentagon-product decision problem. | result sentence after HKO | Draft from the rotated-regular-polygons companion and proof packet. | `Jörn math correctness` |
| `abstract.tex` | The closed method table records no new source of `sys > 1` examples and no candidate-proposer for finding one, beyond examples already explained by the HKO2024 construction and its symplectic images or controlled perturbations. | search/data-science result | `experiments/sys-datascience/README.md`; `experiments/sys-datascience/methods/README.md` | bounded empirical support with caveats; wording accepted by Jörn on 2026-06-03 | Do not say no `sys>1` examples. Do not state density/impossibility/exhaustive search. | result sentence after positive examples | Replace stale abstract bullet “did not yield any new polytopes with `sys>1`.” | `none: caveated/status wording` |
| `abstract.tex` | The thesis uses finite computations for the polytope questions treated here and uses exact/Sage verification where needed. | generalized Reeb orbit and HK2019 foundation; numerics/exactness story | `thesis/generalized-reeb-orbits-polytopes.tex`; `thesis/quadratic-program-algorithm-hk2019.tex`; `thesis/numerics.tex`; `experiments/verification/README.md` | method summary; details elsewhere | Keep abstract-level; do not promise public certified solver or a general reduction for all symplectic capacity questions. | methods sentence | Draft after method sections have support wording. | `none: caveated/status wording` |
| `abstract.tex` | AI was used for much of the thesis labor. | use-of-AI disclosure | `thesis/use-of-ai.tex`; `thesis/abstract.tex` scaffold | required disclosure, not result | Keep factual and defer details to AI-use section. | final methods/disclosure sentence | Draft concise disclosure sentence. | `none: caveated/status wording` |
| `introduction.tex` | State Viterbo’s conjecture and systolic ratio, then explain why the thesis studies polytopes in dimension four and low facet counts. | preliminaries needed for readability | `FACTSHEET.md`; `thesis/introduction.tex`; `thesis/preliminaries.tex`; `thesis/MAP.md` | scaffold-level; needs standard sources | Avoid claiming the restriction is exhaustive or canonical. | opening motivation | Draft after preliminaries notation is fixed enough. | `none: caveated/status wording` |
| `introduction.tex` | Present HKO2024 as the motivating known/surprising example and the thesis’ local-maximality target. | HKO local result | `thesis/introduction.tex`; `experiments/hko-local-maximum/README.md` | exact certificate and line-checked implication exist; final theorem wording/Kai review still open | Say local ten-facet question, not raw strict local maximum in ambient coordinates. | motivation/results overview | Tie to HKO section and final theorem wording. | `Jörn math correctness`; `Kai theorem framing` |
| `introduction.tex` | Present the search/data-science result as bounded evidence that the closed method table found no new source of `sys > 1` examples and no candidate-proposer, while acknowledging HKO2024-derived examples and known structured examples. | search/data-science result; pentagon product result | `experiments/sys-datascience/README.md`; `experiments/sys-datascience/methods/README.md`; `thesis/rotated-regular-polygons.tex` | bounded empirical support | Avoid “no `sys>1` examples”; contrast known pentagon-product/HKO-derived geometry with the absence of a new source or candidate-proposer. | results overview | Draft after method-table terminal states are known enough. | `none: caveated/status wording` |
| `introduction.tex` | Explain the thesis method chain: generalized Reeb orbits, HK2019 finite computation, first-order perturbations, numerics/exactness, and checked code/data. | generalized Reeb orbit and HK2019 foundation; first-order perturbation method; numerics/exactness story; code/data/reproducibility story | active thesis method scaffolds; `formal/sys-first-order-local-behavior.md`; `experiments/dev-quadratic-program/numerics-audit/README.md`; `experiments/verification/README.md` | method roadmap; not all proofs final | Do not overstate first-order theorem beyond generic/caveated route. | structure paragraph | Use as section-by-section reader guide. | `none: caveated/status wording` |
| `hko-local-maximum.tex` | Define the exact local question: HKO2024 among ten-facet polytopes modulo translations, scaling, and linear symplectic maps. | HKO local result | `thesis/hko-local-maximum.tex`; `experiments/hko-local-maximum/README.md`; `experiments/hko-local-maximum/theorem/README.md` | source-backed target | Do not claim strict local maximality in raw `R^40`. | decision problem subsection | Draft definitions and symmetry quotient first. | `Jörn math correctness` |
| `hko-local-maximum.tex` | Exact computation uses quartic field `Q(tan(pi/5))`, exact dual coordinates, symmetry tangent rank 15, and Sage verification surfaces. | HKO local result; numerics/exactness story | `experiments/hko-local-maximum/theorem/`; `experiments/hko-local-maximum/theorem/verification-summary.json` | exact feasible-section certificate verifies | Avoid `Q(sqrt(5))`; use the verifier's ordered `Q(tan(pi/5))` field contract. | Sage computation subsection | Write the exact verification interface from the feasible-section certificate. | `Jörn math correctness` |
| `hko-local-maximum.tex` | The theorem-facing certificate uses 26 feasible-section rows, exact row rank 25, symmetry tangent rank 15, and a positive exact convex relation summing the rows to 0. | HKO local result; first-order perturbation method | `experiments/hko-local-maximum/README.md`; `experiments/hko-local-maximum/theorem/`; `formal/hko-feasible-section-upper-branches.tex` | exact Sage certificate verifies; formal implication line-checked and Jörn quick-reviewed; final theorem wording/Kai review still open | If final theorem wording/Kai review does not accept theorem-strength phrasing, state the exact certificate and implication status without promoting it to a final theorem. | Sage computation subsection before empirical tests | Draft certificate interface from the feasible-section packet and preserve the theorem-wording gate. | `Jörn math correctness`; `Kai theorem framing` |
| `hko-local-maximum.tex` | Supporting empirical checks include first-order numerical bookkeeping, second-order checks, neighborhood sampling, and `M_11` ascent checks. | HKO local result | `experiments/hko-local-maximum/README.md`; `experiments/hko-local-maximum/empirical/README.md` | supporting evidence only | State these are sanity/support checks, not substitutes for exact calculation. | empirical tests subsection | Select only checks that support retained wording. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Search surface includes random generic polytopes, random Lagrangian products, fixed-F gradient ascent, product ascent, variable-F continuation, regular polygon probes, and datascience pipeline. | search/data-science result | `experiments/sys-datascience/README.md`; `experiments/sys-datascience/produce/README.md`; `experiments/sys-datascience/prepare/README.md` | source-backed current surface | Keep HKO-specific packets outside principal hostile-search table unless explicitly used as controls. | opening / rows subsection | Define row families and caveats. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Random generic/product samples and fixed-F/variable-F ascent did not find a new source of `sys > 1` examples; regular pentagon-product geometry remains the known positive structured contrast. | search/data-science result; pentagon product result | `experiments/sys-datascience/prepare/README.md`; `experiments/sys-datascience/produce/README.md`; `thesis/rotated-regular-polygons.tex` | bounded empirical support | Do not claim no positive examples. Do not claim exhaustive search or density theorem. | result table intro | Draft table text from local dataset/method rows. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Table-column regression, classification, PCA/clustering, supervised alternatives, and residualized checks record no candidate-proposer, caveat-only diagnostics, or a current redo/abandoned state. | search/data-science result | `experiments/sys-datascience/methods/README.md` | mixed: source-backed method rows plus downgraded caveats | `stat-sanity` is downgraded and must not carry thesis numerical null/permutation claims unless a repo-owned method packet is added later. | methods subsection / appendix pointer | Draft from the closed method table; optional future rows should not block this wording. | `none: caveated/status wording` |
| `black-box-datascience.tex` | If a method records a validated new `sys > 1` row outside the known HKO2024-derived source or records a candidate-proposer, unrelated method churn stops and the lead is escalated. | search/data-science result | `experiments/sys-datascience/README.md`; `experiments/sys-datascience/methods/README.md` | process/interpretation rule | This is not a thesis result unless such a lead exists. | result-types subsection | Keep as method-policy caveat, not main claim. | `none: caveated/status wording` |
| `conclusion.tex` | Summarize HKO local result at the support strength actually achieved. | HKO local result | HKO rows above | exact feasible-section certificate verifies; final theorem wording/Kai review still open | Theorem only if final wording/Kai review accepts theorem-strength phrasing; otherwise exact-certificate and implication-status wording. | first conclusion paragraph | Draft last from HKO final wording. | `Jörn math correctness`; `Kai theorem framing` |
| `conclusion.tex` | Summarize pentagon product as positive structured result and search/data-science as finding no new source of `sys > 1` examples and no candidate-proposer beyond the HKO2024-derived source. | pentagon product result; search/data-science result | `thesis/rotated-regular-polygons.tex`; `experiments/sys-datascience/README.md`; `experiments/sys-datascience/methods/README.md` | mixed positive + bounded negative | Keep the distinction explicit: positive known structured examples vs no new source/candidate-proposer. | results summary | Draft from final regular-polygon and method-table wording. | `none: caveated/status wording` |
| `conclusion.tex` | Summarize algorithmic/support contributions: generalized Reeb-orbit foundation, HK2019 computation, first-order perturbations, numerics/exactness, CH2021/tube status, visualization, code/data, AI-use. | generalized Reeb orbit/HK2019 foundation; first-order perturbation method; numerics/exactness story; code/data/reproducibility story; use-of-AI disclosure; visualization as exploration; CH2021/flow-graph/tube algorithm story | `thesis/generalized-reeb-orbits-polytopes.tex`; `thesis/quadratic-program-algorithm-hk2019.tex`; `thesis/first-order-perturbations.tex`; `thesis/numerics.tex`; `thesis/flow-graph-algorithm-ch2021.tex`; `thesis/visualization-3d.tex`; `thesis/visualization-3d-content.md`; `thesis/published-code-data.tex`; `thesis/use-of-ai.tex`; `formal/sys-first-order-local-behavior.md`; `experiments/dev-quadratic-program/numerics-audit/README.md`; `experiments/visualization/README.md`; `crates/symplectic/src/algorithms/flow_graph/tube-algorithm-legacy-source-note.md`; `experiments/verification/README.md` | mixed; section-dependent | Do not imply every algorithm is finished at full proof/implementation strength. Use status/caveat wording for sections whose support is partial. | methods summary | Fill after supporting section packets settle support strength. | `none: caveated/status wording` |
| `conclusion.tex` | Future work should include HKO theorem-wording consequences if final review weakens the claim, method-table positive leads if any, and caveated algorithm/numerics extensions. | HKO local result; search/data-science result; CH2021/tube; numerics | HKO and hostile rows above; `tasks/planning-notes.md` | depends on final retained wording | Do not move must-have content to future work silently; future work only for pieces not needed by retained claims or explicitly caveated. | final paragraph | Draft after claim-support audit. | `Jörn thesis-scope/taste` |

## Immediate Follow-Up Packets

1. HKO theorem-control packet.
   - Input: HKO rows in this file plus `experiments/hko-local-maximum/`.
   - Output: theorem target, exact subclaims, artifact-to-subclaim map,
     feasible-section certificate interface, fallback wording.
   - Current packet surface:
     `experiments/hko-local-maximum/README.md` and
     `experiments/hko-local-maximum/theorem/README.md`.
   - Stop: no broad compute unless final review rejects or changes the
     feasible-section certificate route.

2. Hostile method-table closeout packet.
   - Input: data-science rows in this file plus
     `experiments/sys-datascience/README.md` and
     `experiments/sys-datascience/methods/README.md`.
   - Output: terminal state for each thesis-used method row. Current state:
     `endpoint-residualized-regression` has no current LICCA evidence packet
     and must be redone or explicitly abandoned before thesis use;
     `stat-sanity` is non-load-bearing caveat evidence.
   - Stop: no method churn beyond thesis-used rows.

3. Central TeX drafting packet.
   - Input: this file after HKO and hostile blocker rows are settled enough.
   - Output: draft central prose with caveats before details.
   - Stop: no global polish.

4. Rotated-regular-polygons section packet.
   - Current entry point: `experiments/regular-products/README.md`.
   - Writing companion: `thesis/rotated-regular-polygons-content.md`.
   - Exact proof packet:
     `experiments/regular-products/pentagon-rotation-formula-proof/`.
   - Output: final supportable wording for the pentagon-product result/formula.
   - Stop: provide wording usable by abstract/introduction/conclusion; do not
     broaden to all regular polygon products unless the section evidence
     supports it.

5. Supporting-section status packets.
   - Input: the surfaces named in the conclusion support row.
   - Output: support strength and caveated/status wording for CH2021/tube,
     visualization, code/data, AI-use, numerics, first-order, generalized Reeb
     orbit, and HK2019 sections.
   - Stop: enough wording to avoid unsupported promises in the conclusion.
