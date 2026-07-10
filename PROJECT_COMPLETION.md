# Project Completion

Status: tracked control anchor for the `project-completion-integration` branch.
Update this file whenever evidence, ownership, a crux, or a milestone changes.
It is not source truth for mathematics or experiments.

## Objective And Milestones

Complete all three project deliverables:

1. a defensible printed-quality thesis PDF;
2. the durable Rust crates that the thesis and repository promise;
3. the reproducible experiment pipeline and evidence used by the thesis.

The finished project also includes the advisor, submission, and archive state
needed to hand in the thesis. Completion has three explicit milestones:

- **M1 -- ready for Kai:** Jörn accepts a full PDF for Kai review.
- **M2 -- ready for hand-in:** Kai's feedback has been incorporated and all
  remaining submission requirements are ready.
- **M3 -- project complete:** the thesis has actually been handed in and the
  accepted code/data/archive actions are complete.

This control anchor targets M3. A clean branch, a PDF build, chapter drafts,
M1, or M2 is progress but not project completion.

The previous June 2026 deadline is stale and is not an active planning
constraint. No replacement hard deadline is currently recorded.

## Source Truth And Status Language

Use these sources in descending local authority:

- Jörn's current messages and accepted Jörn/Kai decisions;
- `FACTSHEET.md` for still-current Jörn-confirmed facts;
- active `thesis/*.tex`, owner-local mathematical/formal sources, experiment
  artifacts and READMEs, code/tests, and official submission sources;
- content companions and maps as navigation, not evidence;
- recovered branches, session logs, and `/tmp` as provenance or salvage
  sources, not trusted current state.

Status terms in this file mean:

- **recovered:** labor was preserved; no correctness or relevance verdict;
- **internally reviewed candidate:** named local checks/reviews passed, but any
  stated Jörn/Kai/external gate remains open;
- **accepted:** the named stakeholder gate was explicitly passed;
- **complete:** the surface satisfies its downstream use and no named gate
  remains.

Refresh the status table and crux register after every integration or decisive
review before selecting another expensive packet.

## Hard Scope Constraints

The content areas in `FACTSHEET.md` item 8 are mandatory. They may be
reorganized or honestly caveated, but may be cut or materially weakened only
after a newer explicit Jörn scope decision. In particular:

- do not weaken or cut HKO because its proof route is expensive;
- do not replace the non-generic arbitrary-polytope first-order issue with a
  generic smooth-branch statement when retained wording depends on it;
- do not reduce the data-science standard repertoire to a few representative
  families without dispositioning known feasible methods;
- do not prewrite the data-science outcome as negative before row/method
  closure; a positive lead may be escalated or stated as future work;
- numerics may be minimal where it is not load-bearing, but the exact/f64/
  indeterminate/Sage boundary retained by the thesis must be true;
- flow graph/CH2021 remains required, at the support strength actually reached;
- code/data/reproduction and AI-use disclosure are required in substance.

HKO's required trust shape comes from `FACTSHEET.md` items 20--25: Rust may
generate a witness, SageMath must verify it in a standard human-readable way,
LaTeX must justify the verifier algorithm and show that the certificate implies
the theorem, and the final framing must be acceptable to Jörn/Kai. The target
is the exact local statement in the HKO sources, not raw ambient strict local
maximality.

The long-horizon reproduction target comes from `FACTSHEET.md` items 35--38.4:
source-to-data-to-figure-to-PDF instructions; byte-identical expectations or an
explicit accepted comparison class; a devcontainer/local route; the original
LICCA route where relevant; and approximate original timings/machine facts.

## Operating Boundary

- `main` remains blocker-free and read-only until Jörn approves a reviewed
  integration.
- This branch is the consolidated completion surface. Substantial independent
  packets use child worktrees/branches and return as bounded reviewed commits.
- Old worktrees, branches, logs, and `/tmp` are not execution contexts.
- Prefer a fresh session at a phase boundary when a durable handoff exists and
  the previous phase's internals are irrelevant. Do not force a reset when it
  would discard needed state, and do not use repeated compaction as the planned
  workflow.
- Recovered files keep their original epistemic status. Generated outputs,
  mathematical claims, and prose must pass owner-local gates.
- Harness work is permitted only by an explicit harness task and must answer an
  observed failure. A workflow, skill, prompt, or checklist is never project
  completion and should not be created when its expected thesis value is below
  its design/review cost.
- Ask Jörn for mathematical/stakeholder cruxes and elevated LICCA/mail/admin
  actions, not agent-accessible repo work.

The loop may report blocked only when a named Jörn or external action lies on
the critical path, all agent-accessible preparation is complete, and the exact
request, prepared artifact, and current state are recorded here.

## Recovery Checkpoint

Checked 2026-07-10 against integration base `49fc7e59`.

Recovered into this branch:

- HKO candidate from `hko-finish-freeze-v3`/`8c38094c`, excluding the bundled
  unapproved `hko-sage-verifier-review` skill. Source truth:
  `experiments/hko-local-maximum/README.md`,
  `experiments/hko-local-maximum/theorem/`,
  `formal/hko-feasible-section-upper-branches.tex`, and
  `thesis/07-hko-local-maximum*.tex`. The exact Sage verifier, HKO thesis build,
  and formal build passed during the 2026-07-10 recovery audit. Status:
  internally reviewed candidate, not Jörn/Kai accepted. A subsequent
  integrated source/PDF review found no proof gap; it corrected one empirical
  sentence to say that no increase was found above numerical resolution. The
  remaining C1 gate is stakeholder acceptance of the theorem/framing,
  especially shortened Sage excerpts versus the external full verifier.
- Corrected data-science LICCA smoke design from `0a389f58`, retained only as a
  dormant reusable plan conditional on a fresh C3 decision.
- The formerly dirty `datascience-agent-memory` state, checkpointed as
  `bb0e538e`, was normalized at file level. The reviewed P2 packet and canonical
  method dispositions remain. Superseded wave syntheses, proposed compute,
  workflow evaluation, prompt/orchestration material, and stale registries were
  removed after their current source-backed boundaries were consolidated into
  `experiments/sys-datascience/coordination/exploration-handoff.md`.
- Data-science chapter and appendix drafts from `black-box-ds-review`; stale
  conflicting claim-control edits were not imported. Status: recovered prose
  source predating P2, not merge-ready thesis text.
- Stale June deadline removed from `AGENTS.md` and `FACTSHEET.md` by Jörn's
  explicit 2026-07-10 instruction.

The P2 packet at
`experiments/sys-datascience/methods/standard-baseline-p2/` has a durable input
contract: its current-schema prepared table is deterministically rebuilt from
the named tracked Git LFS producer objects and checked against recorded table
hashes. A duplicate prepared table is not retained solely for P2. Status:
reviewed retained-table evidence; no generated-candidate or broader-distribution
claim. On 2026-07-10 a fresh full rebuild reproduced both table hashes, and the
P2 summary plus four TSV artifacts reproduced byte-for-byte; the command record
matched after removing the verification-only output-directory argument.

Recovery is now bounded to the known ownerless HKO and data-science surfaces.
Do not mine `thesis-gpt55-*`, PaperOrchestra, old HKO writing-test/workflow
branches, `thesis-datascience-integration`, or other stale branches unless a
current required claim or artifact is missing and a focused comparison has
positive expected value. The unmerged stalled-session-recovery patch is failure
evidence, not a preferred fix.

## Required Surface

| Surface | Current state | Source route | Completion gate |
| --- | --- | --- | --- |
| Abstract | empty | central accepted thesis claims | write last from accepted wording |
| Introduction | heading only | `thesis/introduction-content.md` plus completed chapters | motivation, results, and reader map |
| Preliminaries | internally reviewed foundation candidate | `thesis/02-*`, `thesis/preliminaries-content.md`, cited papers | C13 and integrated reader review |
| Generalized Reeb orbits | internally reviewed foundation candidate | `thesis/03-*`, formal/cited sources | C13 and integrated reader review |
| Haim--Kislev QP | substantial draft | `thesis/04-*`, companion, HK2017 and product sources | convention/orientation and finite-enumeration acceptance |
| Flow graph/CH2021 | substantial conditional draft | `thesis/05-*`, companion, flow-graph crate/experiment/formal sources | final theorem/algorithm role and honest support wording |
| First-order perturbations | scaffold | `thesis/first-order-perturbations-content.md`, `formal/sys-first-order-local-behavior.md` | supported general method and arbitrary-polytope boundary |
| HKO local maximum | internally reviewed candidate | HKO sources in recovery checkpoint | integrated PDF and Jörn/Kai theorem/framing review |
| Data-science search | Phase 0 normalized; exploration not started | `experiments/sys-datascience/coordination/exploration-handoff.md`, method packets and producer contracts | research closure, demonstration, source-backed rewrite |
| Rotated regular polygons | substantial theorem/certificate draft | `experiments/regular-products/`, `thesis/09-*` | finite-enumeration gate, proof framing, empirical asset polish |
| 3D visualization | heading only | `experiments/visualization/`, thesis companion | concise exploratory exposition and selected assets |
| Numerics | heading only | numerics companion, audit/verification sources | truthful minimal exact/f64/indeterminate/Sage story |
| Published code/data | heading only | crates/experiments READMEs and reproduction commands | true public/reproduction/archive claims |
| Use of AI | heading only | `experiments/ai-use/`, thesis companion | factual Jörn-accepted disclosure |
| Conclusion | heading only | completed accepted chapters | results, limitations, and future work synthesis |
| Appendices | mostly empty; DS draft recovered | named chapter proof/evidence needs | only useful proof/reproduction/detail material retained |
| Rust crates | extensive existing code | `crates/MAP.md`, crate docs/source/tests | durable APIs/tests/docs required by thesis/repo promises |
| Experiment pipeline | extensive mixed state | `experiments/MAP.md`, owner-local producers/artifacts | claimed artifacts reproduce under named comparison rules |
| Bibliography/assets | partially integrated | thesis bibliography and producing owners | every retained citation/table/figure/reference resolves |
| Advisor/submission/archive | external state incomplete | `FACTSHEET.md`, `submit/`, current official sources, Jörn/mail state | Kai flow, registration, hand-in, accepted archive complete |

The consolidated integration PDF built cleanly at 58 pages on 2026-07-10 with
`cd thesis && latexmk && ./check-build.sh`.

The Clarke/simple-minimizer foundation packet was source-audited on 2026-07-10
against HK2017, AAO2014, the active symplectic conventions, and the local
HK2017 convention audit. The free-period multiplier and reconstruction factors
are checked both by explicit rescaling of HK2017 Lemma 2.1 and by the detailed
free-period multiplier derivation retained in the thesis; the latter follows
the proof of AAO2014 Lemma 5.2 while deriving the thesis normalization locally.
The simple-minimizer compactness and cyclic-merging steps are explicit. The
thesis and formal builds passed, and a bounded independent mathematical review
found no high-severity error after two citation/exposition findings were
corrected. The subsequent preservation audit compared the pre-audit active
text, the Jörn-approved legacy proof, the active companions, HK2017, and the
AAO2014 proof. It restored the lost step-by-step multiplier calculation without
presenting the nonsmooth multiplier rule itself as locally proved. C13 is
settled: cite the literature for attribution and analytic inputs while
retaining the detailed proofs and explanations already developed, accepted by
Jörn, and agreed with Kai for inclusion. Do not compress the existing proof
presentation into citations. Status: internally reviewed candidate; only the
ordinary integrated reader review remains. A fresh independent mathematical
review of the preservation repair found no error in the constraint
qualification, signs, homogeneity factors, or multiplier normalization; its
one optional source-precision observation was incorporated.
This packet does not change the simple-minimizer theorem strength used by the
QP, flow-graph, HKO, rotated-polygon, or first-order chapters.

## Dependency And Ownership Shape

1. Data-science Phase 0 normalization is complete. Start fresh exploration from
   `experiments/sys-datascience/coordination/exploration-handoff.md`; do not
   revive recovered coordination as execution state.
2. Review HKO as an integrated candidate. Reopen proof development only on a
   concrete mathematical/source finding.
3. Use separate durable handoffs for data-science recovery/freeze,
   exploration/research, demonstration/reproduction, and thesis writeup. The
   exploration phase must disposition the feasible standard repertoire and
   decide whether more data/producer axes change an exact retained claim.
4. Review Clarke/simple minimizers, then QP conventions and the product
   finite-enumeration theorem. These unlock reliable first-order, HKO, and
   pentagon exposition.
5. Finish the remaining required result, method, side-result, code/data, and
   disclosure sections from accepted sources.
6. Write introduction, conclusion, and abstract; then review the integrated PDF
   for mathematics, empirical interpretation, source strength, notation,
   citations, assets, and reader flow.
7. Complete reproduction/release, Kai review and corrections, current official
   submission requirements, registration/hand-in actions, and archive.

Independent child sessions may work in parallel only when they do not depend
on an unsettled claim. The integration controller owns dependency ordering,
merge readiness, and the crux register.

## Packet Minimum

A substantial child packet records the completion gate it advances, exact
inputs/status, outputs, prohibited stronger conclusions, checks that must pass,
stopping conditions, and next ownership. Omit a normally relevant check only
with a recorded reason. Add or revise durable workflow material only after an
observed recurring failure and a positive expected-value check.

## Crux And Uncertainty Register

Use `agent-checkable`, `Jörn`, `Kai`, `external`, or `settled`.

| ID | Status | Crux or uncertainty | What resolves it | Downstream effect |
| --- | --- | --- | --- | --- |
| C1 | Jörn/Kai | Accept HKO theorem wording, equality-orbit phrasing, local chart bridge, and computation-as-proof framing. | integrated PDF review against the named exact/formal sources | HKO, abstract, introduction, conclusion |
| C2 | Jörn/Kai | Is the Lagrangian-product finite-enumeration theorem sufficiently proved and cited? | focused source/math review | pentagon theorem and QP chapter |
| C3 | agent evidence, then Jörn | Which exact data-science claim is the target after satisfying the required method/data coverage: the current bounded retained random/product claim, or a broader producer-axis claim requiring LICCA work? | normalized claim ladder, method dispositions, missing-data assessment, costed alternatives | research stopping rule and LICCA requirement |
| C4 | Jörn | Should data science remain framed as a main result? | compare final supported claim with thesis architecture | title and thesis emphasis |
| C5 | agent-checkable, then Jörn math | What arbitrary-polytope first-order statement is supported beyond generic smooth branches? | formal/source audit with explicit theorem and fallback wording | first-order and HKO/search bridges |
| C6 | agent-checkable, then Jörn scope | What final role/support strength should flow graph/CH2021 have? | theorem/code/evidence audit with concrete chapter alternatives | flow-graph chapter and conclusion |
| C7 | settled | P2 current-schema input is source-reproducible from named tracked producer objects; no surviving `/tmp` input is required. | reopen only if producer objects, prepare schema, or artifacts change | P2 evidence promotion |
| C8 | Jörn | What AI-use disclosure length and emphasis is desired? | short source-backed alternatives | AI section and abstract |
| C9 | Jörn | Has Kai received a full PDF, and what review state is current? | Jörn/mail state when scheduling M1/M2 | external review sequencing, not a deadline assumption |
| C10 | Jörn | What final archive target and artifact set should be used? | prepared options near finalization | code/data section and release |
| C11 | agent external check + Jörn state | What are the current official submission requirements, and has the registration note been handed to the Prüfungsamt? | refresh official sources; ask only the private completed-action state | M2/M3 hand-in checklist |
| C12 | settled | The consolidated exploration handoff is the only active research-control surface; P1 decisions live in the canonical method-disposition ledger, the competing top-level ledger was removed, owner packets retain evidence, and `experiments/sys-datascience/LICCA.md` marks every retained cluster script dormant. | reopen only if a unique source-backed decision is shown missing from the handoff or owner | prevent recovered process bulk from driving work |
| C13 | settled | Cite the literature while retaining the detailed Clarke/simple-minimizer proofs and explanations already developed; Jörn is satisfied with them and Kai agreed to include the proofs already available. | Jörn decision, 2026-07-10 | do not replace the existing proof presentation with citation-only prose |

Open research under C3, C5, or C6 stops when the alternatives' retained wording
and evidence are explicit, another iteration is unlikely to change the thesis
choice enough to repay its cost, and any remaining stakeholder choice is put to
Jörn. One diagnostic, one method wave, or one theorem attempt is not closure by
itself.

Do not ask Jörn to answer the whole register. Ask one crux when it changes the
next expensive route and agent-accessible evidence is prepared.

## Insufficient Stopping Points

- a clean integration branch or PDF build;
- merging HKO or data-science drafts;
- completing one experiment/method wave;
- producing a writing workflow, skill, checklist, or status report;
- drafting every section without source/claim review;
- passing tests while retained thesis/repo promises remain false or unclear;
- reaching M1 or M2 without reaching M3.

## Completion Conditions

M3 is reached only when:

- every mandatory `FACTSHEET.md` item 8 area is present at an explicitly
  accepted support strength;
- every retained mathematical and empirical claim is supported at its stated
  strength, including the HKO, pentagon, data-science, and shared-foundation
  review gates;
- the full PDF has no silent placeholders and passes integrated mathematical,
  source, citation, notation, asset, and reader-flow review;
- the durable crate and experiment deliverables satisfy their retained API,
  evidence, documentation, and reproduction promises without requiring broad
  unrelated cleanup;
- source-to-artifact-to-PDF reproduction is documented and checked at each
  promised comparison level;
- AI-use, archive, and publication statements are true;
- Kai feedback and current official submission requirements are accounted for;
- the registration and hand-in actions are complete;
- the thesis has actually been handed in, the accepted archive/release actions
  are complete, and Jörn confirms project completion.
