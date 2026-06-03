Status: durable control packet for central thesis claims. Not source truth.
Purpose: make the next central thesis writing packet possible without hidden
overclaim or live chat clarification.
Overruled by: active thesis files, research notes, experiment artifacts, formal
proof files, official sources, and accepted Jörn/Kai decisions.
Origin: copied from `/tmp/central-claim-control-packet.md` on 2026-06-01 after
review and fixes. The `/tmp` copy is disposable; this file is the durable
project reference.

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
| `abstract.tex` | HKO2024 is locally maximal among ten-facet polytopes up to translations, scaling, and linear symplectic maps. | HKO local result | `research/hko-local-maximum-status.md`; `research/hko-local-maximum.md`; `research/hko-local-maximum-exact-clarke.md` | target theorem; not closed today | If Packet 3 does not close, say strong exact/numerical support and certificate-in-progress, not theorem. | result sentence | Draft only after HKO claim row below is settled. | `Jörn math correctness`; `Kai theorem framing` |
| `abstract.tex` | Product of two rotated regular pentagons gives the named positive structured `sys>1` result/formula. | pentagon product result | `thesis/rotated-regular-polygons.tex`; `research/sys-landscape.md`; `research/INDEX.md` | support exists in scaffold/research; exact wording pending Sage/proof writeup | Do not claim global classification beyond the checked pentagon-product decision problem. | result sentence after HKO | External dependency: final wording must come from the rotated-regular-polygons section packet before central TeX finalization. | `Jörn math correctness` |
| `abstract.tex` | Search/data-science work did not produce a new useful explanation or general search rule beyond known structured examples. | search/data-science result | `research/sys-landscape.md`; `research/sys-landscape-toolbox-audit.md`; `research/sys-landscape-datascience/idea-ledger.md` | bounded empirical support with caveats | Do not say no `sys>1` examples. Do not state density/impossibility/exhaustive search. | result sentence after positive examples | Replace stale abstract bullet “did not yield any new polytopes with `sys>1`.” | `none: caveated/status wording` |
| `abstract.tex` | The thesis uses finite computations for the polytope questions treated here and uses exact/Sage verification where needed. | generalized Reeb orbit and HK2019 foundation; numerics/exactness story | `thesis/generalized-reeb-orbits-polytopes.tex`; `thesis/quadratic-program-algorithm-hk2019.tex`; `thesis/numerics.tex`; `research/verification.md` | method summary; details elsewhere | Keep abstract-level; do not promise public certified solver or a general reduction for all symplectic capacity questions. | methods sentence | Draft after method sections have support wording. | `none: caveated/status wording` |
| `abstract.tex` | AI was used for much of the thesis labor. | use-of-AI disclosure | `thesis/use-of-ai.tex`; `thesis/abstract.tex` scaffold | required disclosure, not result | Keep factual and defer details to AI-use section. | final methods/disclosure sentence | Draft concise disclosure sentence. | `none: caveated/status wording` |
| `introduction.tex` | State Viterbo’s conjecture and systolic ratio, then explain why the thesis studies polytopes in dimension four and low facet counts. | preliminaries needed for readability | `thesis/introduction.tex`; `thesis/preliminaries.tex`; `research/INDEX.md` | scaffold-level; needs standard sources | Avoid claiming the restriction is exhaustive or canonical. | opening motivation | Draft after preliminaries notation is fixed enough. | `none: caveated/status wording` |
| `introduction.tex` | Present HKO2024 as the motivating known/surprising example and the thesis’ local-maximality target. | HKO local result | `thesis/introduction.tex`; `research/hko-local-maximum-status.md` | target theorem not closed | Say local ten-facet question, not raw strict local maximum in ambient coordinates. | motivation/results overview | Tie to HKO section and fallback wording. | `Jörn math correctness`; `Kai theorem framing` |
| `introduction.tex` | Present the search/data-science result as bounded evidence about failed general search rules, while acknowledging known structured `sys>1` examples. | search/data-science result; pentagon product result | `research/sys-landscape.md`; `research/sys-landscape-toolbox-audit.md`; `thesis/rotated-regular-polygons.tex` | bounded empirical support | Avoid “no `sys>1` examples”; contrast known pentagon-product geometry with failed broader search rules. | results overview | Draft after method-table terminal states are known enough. | `none: caveated/status wording` |
| `introduction.tex` | Explain the thesis method chain: generalized Reeb orbits, HK2019 finite computation, first-order perturbations, numerics/exactness, and checked code/data. | generalized Reeb orbit and HK2019 foundation; first-order perturbation method; numerics/exactness story; code/data/reproducibility story | active thesis method scaffolds; `research/sys-first-order-local-behavior.md`; `research/numerics.md`; `research/verification.md` | method roadmap; not all proofs final | Do not overstate first-order theorem beyond generic/caveated route. | structure paragraph | Use as section-by-section reader guide. | `none: caveated/status wording` |
| `hko-local-maximum.tex` | Define the exact local question: HKO2024 among ten-facet polytopes modulo translations, scaling, and linear symplectic maps. | HKO local result | `thesis/hko-local-maximum.tex`; `research/hko-local-maximum-status.md`; `research/hko-local-maximum.md` | source-backed target | Do not claim strict local maximality in raw `R^40`. | decision problem subsection | Draft definitions and symmetry quotient first. | `Jörn math correctness` |
| `hko-local-maximum.tex` | Exact computation uses quartic field `Q(tan(pi/5))`, exact dual coordinates, symmetry tangent rank 15, and Sage verification surfaces. | HKO local result; numerics/exactness story | `research/hko-local-maximum-exact-clarke.md`; `experiments/hko-local-maximum/exact-clarke/`; `widened-seed-witness.json`; `widened-seed-witness-verification.json` | Packet 1 closed; Packet 3 partial | Avoid `Q(sqrt(5))`; say Sage may use larger exact field if needed. | Sage computation subsection | Write subroutine chain from exact-Clarke note. | `Jörn math correctness` |
| `hko-local-maximum.tex` | Final theorem certificate needs active-gradient rank 25, kernel dimension 15, and kernel equal to symmetry tangent space. | HKO local result; first-order perturbation method | `research/hko-local-maximum.md`; `research/hko-local-maximum-status.md`; `research/sys-first-order-local-behavior.md` | target certificate; not closed today | If incomplete, state as certificate target/current blocker, not completed theorem. | Sage computation subsection before empirical tests | Create HKO blocker packet mapping subclaims to artifacts. | `Jörn math correctness`; `Kai theorem framing` |
| `hko-local-maximum.tex` | Supporting empirical checks include first-order numerical bookkeeping, second-order checks, perturbation, facet-splitting, cut-and-ascent, and neighborhood checks. | HKO local result | `research/hko-local-maximum.md`; `experiments/hko-local-maximum/README.md` | supporting evidence only | State these are sanity/support checks, not substitutes for exact calculation. | empirical tests subsection | Select only checks that support retained wording. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Search surface includes random generic polytopes, random Lagrangian products, fixed-F gradient ascent, product ascent, variable-F continuation, regular polygon probes, and datascience pipeline. | search/data-science result | `research/sys-landscape.md`; `research/sys-landscape-toolbox-audit.md` | source-backed current surface | Keep HKO-specific packets outside principal hostile-search table unless explicitly used as controls. | opening / rows subsection | Define row families and caveats. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Random generic/product samples and fixed-F/variable-F ascent did not find a new broad search route; regular pentagon-product geometry remains the known positive structured regime. | search/data-science result; pentagon product result | `research/sys-landscape.md`; `research/sys-landscape-toolbox-audit.md` | bounded empirical support | Do not claim no positive examples. Do not claim exhaustive search or density theorem. | result table intro | Draft table text from audit rows. | `none: caveated/status wording` |
| `black-box-datascience.tex` | Feature/regression/classification/PCA/clustering/supervised alternatives mostly give negative or caveat-only evidence; endpoint residualized regression is not thesis-bearing as-is. | search/data-science result | `research/sys-landscape-toolbox-audit.md`; `research/sys-landscape-datascience/idea-ledger.md` | mixed: some source-backed, some redo/downgrade | `endpoint-residualized-regression` must be repaired or cut/futured; `stat-sanity` should not be load-bearing until source-truth repaired. | methods subsection / appendix pointer | Run hostile method-table closeout packet before final prose. | `none: caveated/status wording` |
| `black-box-datascience.tex` | If a method gives a useful pattern or positive lead, unrelated method churn stops and the lead is escalated. | search/data-science result | `research/sys-landscape-datascience/idea-ledger.md`; `tasks/planning-notes.md` | process/interpretation rule | This is not a thesis result unless such a lead exists. | result-types subsection | Keep as method-policy caveat, not main claim. | `none: caveated/status wording` |
| `conclusion.tex` | Summarize HKO local result at the support strength actually achieved. | HKO local result | HKO rows above | depends on Packet 3 outcome | Theorem if certificate closes; otherwise support/certificate-in-progress wording. | first conclusion paragraph | Draft last from HKO final wording. | `Jörn math correctness`; `Kai theorem framing` |
| `conclusion.tex` | Summarize pentagon product as positive structured result and search/data-science as failure to find broader explanation/search rule. | pentagon product result; search/data-science result | `thesis/rotated-regular-polygons.tex`; `research/sys-landscape.md`; `research/sys-landscape-toolbox-audit.md` | mixed positive + bounded negative | Keep the distinction explicit: positive known structured examples vs no new general rule. | results summary | Draft from final regular-polygon and method-table wording. | `none: caveated/status wording` |
| `conclusion.tex` | Summarize algorithmic/support contributions: generalized Reeb-orbit foundation, HK2019 computation, first-order perturbations, numerics/exactness, CH2021/tube status, visualization, code/data, AI-use. | generalized Reeb orbit/HK2019 foundation; first-order perturbation method; numerics/exactness story; code/data/reproducibility story; use-of-AI disclosure; visualization as exploration; CH2021/flow-graph/tube algorithm story | `thesis/generalized-reeb-orbits-polytopes.tex`; `thesis/quadratic-program-algorithm-hk2019.tex`; `thesis/first-order-perturbations.tex`; `thesis/numerics.tex`; `thesis/flow-graph-algorithm-ch2021.tex`; `thesis/visualization-3d.tex`; `thesis/published-code-data.tex`; `thesis/use-of-ai.tex`; `research/sys-first-order-local-behavior.md`; `research/numerics.md`; `research/visualization.md`; `research/tube-algorithm.md`; `research/verification.md` | mixed; section-dependent | Do not imply every algorithm is finished at full proof/implementation strength. Use status/caveat wording for sections whose support is partial. | methods summary | Fill after supporting section packets settle support strength. | `none: caveated/status wording` |
| `conclusion.tex` | Future work should include unresolved exact HKO certificate pieces if not closed, method-table positive leads if any, and caveated algorithm/numerics extensions. | HKO local result; search/data-science result; CH2021/tube; numerics | HKO and hostile rows above; `tasks/planning-notes.md` | depends on final retained wording | Do not move must-have content to future work silently; future work only for pieces not needed by retained claims or explicitly caveated. | final paragraph | Draft after claim-support audit. | `Jörn thesis-scope/taste` |

## Immediate Follow-Up Packets

1. HKO blocker packet.
   - Input: HKO rows in this file plus `research/hko-local-maximum*.md`.
   - Output: theorem target, exact subclaims, artifact-to-subclaim map, Packet 3
     missing rows, fallback wording.
   - Stop: no broad compute until this map exists.

2. Hostile method-table closeout packet.
   - Input: data-science rows in this file plus
     `research/sys-landscape-toolbox-audit.md` and
     `research/sys-landscape-datascience/idea-ledger.md`.
   - Output: terminal state for each thesis-used method row; repair/cut decision
     for `endpoint-residualized-regression`; repair/downgrade decision for
     `stat-sanity`.
   - Stop: no method churn beyond thesis-used rows.

3. Central TeX drafting packet.
   - Input: this file after HKO and hostile blocker rows are settled enough.
   - Output: draft central prose with caveats before details.
   - Stop: no global polish.

4. Rotated-regular-polygons section packet.
   - Input: `thesis/rotated-regular-polygons.tex`; `research/sys-landscape.md`;
     relevant Sage/proof artifacts for the pentagon-product decision problem.
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
