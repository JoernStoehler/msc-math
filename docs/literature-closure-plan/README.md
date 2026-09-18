# Literature closure: bounded plan and one verified correction

Checked 18 September 2026. Planning branch `research/literature-closure-plan`, based on research/review-workflow-design `e33c3f0f`. Production remains paused. This is a focused inventory, not a completed literature survey or a new proof audit.

## Immediate result

The active candidate introduction still asserted that equality of EHZ and cylindrical capacity for four-dimensional convex bodies is open. This is false. The narrow correction and its bibliography entry are committed alongside this plan. They apply to the recovered introduction actually selected by the interrupted production main.tex as well as by this clean branch. No production worktree has been modified.

Abbondandolo–Edtmair–Kang, [arXiv:2412.01777v1](https://arxiv.org/html/2412.01777v1), Corollary 1 proves the capacity equality for **every convex body in R4**. The stronger surface-of-section theorem has a uniformly convex hypothesis; it must not be imposed on the capacity corollary. The revised sentence states the capacity result and its relevance to our computations, and removes the misleading “Edtmair instead” transition. This does not identify Gromov width with EHZ. Abstract-page history lists only v1, dated 2 December 2024.

The clean candidate built successfully with `sh thesis/candidate/build.sh` (86 pages). This validates the citation integration, not the interrupted candidate assembly, mathematics, or prose acceptance.

## Source boundaries

Actual interrupted source: `/tmp/msc-math-thesis-review-20260917/thesis/candidate/main.tex`. It selects recovered introduction, preliminaries, flow, variation, HKO, DS opening-v2, conclusion, plus replacement QP/pentagon/numerics and other existing overlays. The clean research branch does not contain all those interrupted drafts. Consult the assembly-source-map workstream before applying changes to anything other than the identical recovered introduction and additional.bib patched here.

Background diagnosis: `docs/process-audit/independent/reconciliation.md`, especially “premature closure,” and `docs/review-evidence/pro-review-2026-09-15/thesis_review/04_sources_and_scope.md`. Prior review findings are pointers, not fresh verification. This pass directly checked the primary sources below for the consequential corrections; it did not reproduce the full prior audit.

## Consequential claim inventory

| Priority / claim | Thesis surface and present status | Closure task |
|---|---|---|
| P0: EHZ = cylindrical in R4 | Recovered introduction §1.1: false open claim; **corrected in this branch** | Integrate the two-file correction; check final abstract/introduction do not reintroduce stale wording. |
| P0: Which sampled product families can exceed sys=1? | DS opening-v2 §8.1 uses ten groups, 3≤k≤m≤6; search narrative does not separate proven exclusions | BMP2026 Theorem 1.2 directly verifies arbitrary quadrilateral factor and affinely regular hexagon factor. Section 4 treats triangles. Map those scopes to actual generator outputs, then revise interpretation of retained search runs. |
| P1: Fixed-ten-facet HKO result versus existing nonsmooth literature | Intro §1.2, HKO opening, conclusion; smooth Zoll comparison exists, directly relevant 2025 paper absent | Compare exact perturbation spaces/conclusions with Haim–Kislev 2025 Example 1.12 and Proposition 1.13. Do not equate cuts additivity, cuts extremality, generalized Zoll, and local systolic maximality. |
| P1: Product six-facet / three-bounce reduction | Interrupted QP §4.3 cites Rudolf Theorem 1; further product-QP reduction accepted by Jörn | Pin final journal Definition 2/Theorem 1 and check nonsmooth hypotheses/normalization against imported step. Distinguish imported billiard theorem, project reformulation, and independent product vertex-support proof. Preserve accepted theorem strength. |
| P1: HKO endpoint value and pentagon provenance | Improved pentagon derives profile using HKO endpoint values; AI proof provenance is present | Pin HKO final text/value; compare newly claimed contribution to endpoint result. Preserve independent analytic proof provenance in disclosure; do not call a rederived known endpoint novel. Priority search only if final text claims publication priority. |
| P1: CH2021 import versus project flow theorem | Flow opening imports local passage geometry; project adds conditional exhaustive search and chamber-genericity | Technical-closure owner checks hypotheses and actual implication. Literature owner supplies accurate theorem/version and attribution, not another overlapping proof audit. |
| P2: Foundational finite formula and generalized characteristics | QP imports HK Theorem 1.1; Reeb chapter draws on variational sources | Resolve exact source/version/pinpoint, particularly prior Q03 AAO lemma-number question. This is distinct from checking the self-contained derivation is logically complete. |
| P2: Background connections | Intro AKO2014 Mahler equivalence; AB2023 Zoll optimality; Edtmair2024 surface-of-section characterization | Verify precise hypotheses, capacity normalization, and topology of local optimality against final statements. No defect established by this pass beyond the corrected transition. |
| P2: AI reflection literature inference | AI chapter cites Guo et al. 2025 for outcome-feedback generalization | Match its precise sentence to paper evidence; prohibit jumping from a training result to established explanation of this thesis's agent behavior. Lower priority than mathematical thesis claims. |
| P2: Bibliography metadata and availability promises | Candidate build resolves legacy/bibliography.bib plus recovered additional.bib | HKO metadata already correctly cites Annals 203(2),603–622,2026; do not “fix” it back to preprint year. Check other stale “to appear” entries only if actually cited. Publication/repository availability claims go to integration owner; no publication is authorized. |

**Family inference already justified at this scope:** of the ten side-count groups, 3×3, 3×4, 3×5, 3×6, 4×4, 4×5, 4×6 are excluded from above-one discovery by triangle/quadrilateral results. The three remaining groups are *not excluded by these particular results*, which is not proof that no other theorem excludes a subfamily. A random six-sided polygon is not necessarily affinely regular. An arbitrary orthogonal mixing of q/p can destroy the product hypothesis; do not transfer the exclusion without checking the transformation. Historical runs in excluded families remain useful for controls and subthreshold geometry.

**Novelty distinction:** “we prove X” can honestly describe a thesis proof without asserting first publication of X. Global “first/new/previously unknown” wording requires a separate search. Neither a clean search nor absence of hits certifies priority. This pass found no reason to weaken the accepted local theorem or remove retained research areas.

## Work units and stopping conditions

These are judgment-based effort ranges, not a measured schedule.

1. **DS theorem-to-generator mapping (30–60 agent-minutes).** Inputs: DS evidence packet, actual generator law, BMP text. Output: short family table and exact corrections to interpretation. Stop when every retained above-one search family is either covered by a specific theorem with matched hypotheses or explicitly outside the checked exclusions. Escalate complexity if an input is a projection/mixed symplectic coordinate transform rather than an actual Lagrangian product. Can run independently of HKO comparison.
2. **HKO related-work comparison (45–90 agent-minutes).** Inputs: project theorem and HK2025 definitions/Example1.12/Proposition1.13. Output: claim/hypothesis comparison and a short paragraph suitable for intro or HKO. Read neighboring results and recent citing/author work for counterevidence to any proposed current-open or first-result statement. Stop with supported comparison; unresolved priority remains explicitly unresolved rather than a silent novelty claim. If a newer theorem subsumes the project theorem, return that concrete evidence before rewriting framing.
3. **Imported-theorem citation contract (45–90 agent-minutes, separable by HK/Rudolf/CH/AAO).** Coordinate with technical closure to avoid duplicated proof review. Output: selected theorem number/version, hypotheses, exact imported conclusion and transformations to project conventions. Stop on complete implication mapping, not on matching title/abstract. Any missing nonsmooth/generalized applicability is a substantive mathematical issue, not citation noise.
4. **Final claim sweep (20–40 agent-minutes after integration).** Search assembled source and PDF for open/unknown/new/first plus contextual synonyms and misleading future-work suggestions; inspect introduction, conclusion, captions and disclosure. Validate citations resolve to the checked edition. Stop after consequence-bearing claims have owners/support; do not recrawl every bibliography item.

Parallel now: units 1 and 2 plus source/version part of 3. Dependent: writing final comparison after technical theorem scope is settled; final sweep after authoring. No new human review required to execute these checks. Request scientific judgment only if the project scope actually changes.

## Primary sources consulted in this pass

- [AEK2412.01777v1](https://arxiv.org/html/2412.01777v1), introduction Corollary1 and surrounding capacity discussion; current abstract version history checked. Supports the applied correction.
- [HKO2405.16513v3](https://arxiv.org/html/2405.16513v3), current primary text opened; abstract version history checked (v3 20 November 2025). [Annals final page](https://annals.math.princeton.edu/2026/203-2/p05) independently verifies existing bibliography metadata and publication 1 March 2026. Endpoint proof was not reaudited here.
- [BMP2603.12495v1](https://arxiv.org/html/2603.12495v1), Theorem1.2, §4 triangular case, §7 affinely regular hexagons. Preprint status; no claim of refereed publication. Source supports family exclusions, not arbitrary-hexagon exclusion.
- [HK2511.16644v1](https://arxiv.org/html/2511.16644v1), Example1.12, Proposition1.13 and Remark1.15. Checked distinction between numerically suggested behavior and stated cut-additivity proposition. Full comparison remains work unit2.
- [Rudolf worm-problem article](https://link.springer.com/article/10.1007/s00605-022-01806-x) appeared in targeted current product search; Proposition4.1 and cited finite characterization were inspected as discovery context only, not used to close a thesis claim.

Searches included current Viterbo/Lagrangian-products/polygon and symmetric-case terminology, not only “open problem”. Search results are leads; unassessed claims of defects on aggregators do not overturn accepted results. A bounded search is not exhaustive coverage. Final-date-sensitive assertions must be checked again only if later evidence or substantially later completion makes that relevant.

## Scope left deliberately open

No exhaustive priority determination, no proof revalidation of all cited papers, no blanket certification of current literature status. No modifications to active/main dirty work, no rewritten DS/HKO chapter, no public/university submission. This branch is an active integration handoff and should be removed after its unique committed work is integrated, not discarded before that.
