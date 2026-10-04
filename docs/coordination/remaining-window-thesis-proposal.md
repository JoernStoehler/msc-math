# Remaining-window thesis proposal

Prepared 3 October 2026; revised after rejection of the arbitrary four-worker
cap. **Unapproved.** Hard finish:
4 October 04:00 Berlin time / 02:00 UTC. This is a concrete candidate-production
proposal, not a promise of a thesis grade or human acceptance.

## What the run would leave behind

1. **A readable odd-polygon result, with proof and consequences.** Explain the
   existing claim that every odd regular planar factor with n >= 5 and every
   centrally symmetric convex partner has capacity given by the width formula,
   a sharp systolic bound strictly below 1, and the specified equality partners.
   Explain the necessary asymmetry bound for a potential counterexample and why
   triangles fall outside the result. Reuse the selected pentagon candidate as
   the special case; keep the broader theorem visibly a candidate. Its argument
   has an independent agent check; human acceptance and novelty are unestablished.
2. **A checked local-maximality section for Chapters 6–7.** Reconstruct the
   fixed-normal active-orbit balance argument, including why an uncovered facet
   permits improvement. Explain the stationary-base identity, actual capacity
   versus feasible upper sections, and the chamber/completeness restrictions.
   If a consequential implication fails reconstruction, return the strongest
   supported section and the exact unresolved implication.
3. **One complete certified numerical example for Chapters 13–14.** Use the
   existing known-simplex fixture and public capacity API. Retain represented
   input bits, returned interval, exact comparison, command and output receipt;
   explain what establishes global capacity. Return a small reusable producer
   and evidence directory, rather than merely another test PASS.
4. **A combined candidate PDF and concrete insertion patch.** Include necessary
   surrounding text, repair the affected reader bridges, and inspect the actual
   assembled PDF. Do not silently promote candidates or human acceptance.

## Work inventory, sources and ownership

Recommend **sixteen worker assignments plus a coordinating root**, using included
Codex and installed local tools. No paid APIs or new remote compute. The count
follows the split below; it is not measured capacity or demonstrated optimality.
No production workers have been launched.

Each content owner writes only `thesis/candidates/remaining-window/unit-NN/`:
replacement text, necessary context, source map and insertion boundary. Shared
chapter copies belong to the integrators.

| Owner | Actual owned output | Existing starting point |
|---|---|---|
| P1 | Odd-regular theorem, equality/asymmetry consequences, selected pentagon corollary | `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md`; selected symmetric candidate |
| P2 | General width/four/six-block explanation and exact triangle obstruction | `formal/pentagon-affine-products/research.tex`; Chapter 10 and triangle appendix |
| P3 | Stationary-base identity and KKT/section explanation | `formal/capacity-derivatives.tex`; weaker identity in September 15 mathematical audit; Chapter 6 |
| P4 | Active-orbit balance and facetwise coverage account | `formal/active-orbit-facet-coverage.tex`; Chapters 6–7 |
| P5 | 26-section/25-dimensional certificate explanation and predicate-to-lemma map | Chapter 7, `thesis/appendices/hko-certificate.tex`, `hko_core.py`; backlog section 2 |
| P6 | Surviving support-row, billiard, splitting and closure-replacement bridges | Chapters 2/4; backlog section 1 |
| P7 | Known-simplex producer, retained input/result/receipt and certified example | `crates/symplectic/tests/public_capacity_api.rs`; owned new `experiments/remaining-window-capacity-example/` |
| P8 | Figure/result/evidence links and truthful selected-claim reproduction instructions | Chapters 12–14; topic READMEs and retained artifacts |
| P9 | Current DS and ten-facet human-concern disposition, necessary replacements only | Chapter 8; backlog sections 5/7 and current human-review sources |
| P10 | Claim-to-evidence map and candidate opening/conclusion paragraphs | Current manuscript and `docs/project-facts.md`; supported final scopes |
| R11 | Independent polygon/product-foundation proof and source reconstruction | P1/P2/P6 sources and final text |
| R12 | Independent variation/HKO implication reconstruction | P3/P4/P5 sources and final text |
| R13 | Independent API and evidence-contract review | P7/P8/P9 sources, outputs and final text |
| R14 | Independent assembled-reader review and correction requests | P6/P10 and combined reading context |
| I15 | Mathematical units 1–6 integrated in staged Chapters 2/4/6/7/9/10 | Owned mathematical chapter copies and fragment manifest |
| I16 | Units 7–10 integrated in staged Chapter 8/12/13/14 and opening/conclusion; combined build | Owned evidence/prose copies, combined `main.tex`, PDF and insertion patch |

**Receiving final-delivery owner: this task-design root**, conditional on explicit
execution approval. Root supervises dependencies and reconciles coordination.
I15/I16 perform integration rather than routing every edit through root; they
must explicitly accept ownership at launch. R14 independently checks assembled
output. Reviewers do not accept their own content. No silent promotion occurs.

## Schedule, alternatives and risks

Check live ownership at launch; fix insertion locations immediately. Reviewers
start from sources and integrators prepare chapter architecture immediately,
without waiting for all drafts. Request first supported drafts within 45 minutes.
Freeze substantial scope by 00:45 UTC;
start protected assembled review by **01:00 UTC**, finish by **02:00 UTC**.
Assemble incrementally; finish supported units before broadening them. Stop
already-repaired units and move released ownership to outstanding reviews.
P3 feeds P5; P1/P2 inform P6; P7 feeds P8; P10 follows supported final scopes.
I15's fragments feed I16's combined build. No expensive certificate rerun.

| Configuration | Work funded | Limitation |
|---|---|---|
| 4 workers | Three producers and one reader | Most inventory omitted/grouped; all integration bottlenecks root; no basis that it suffices |
| 8 workers | Four grouped production streams, two grouped reviewers, two integrators | Products; variation/HKO; foundations/DS; numerical/evidence become longer serial streams; review scope concentrated |
| 16 workers | Ten content units, three source/proof reviewers, assembled reader and two integrators above | More coordination and handoffs; current load/quota telemetry does not establish throughput |

Sixteen is recommended for the concrete supported inventory. It does not prove
optimality, full delivery or PASS. Four was previously proposed without capacity
evidence and should not be imposed as a cap.

The odd-polygon source is suitable for explanation rather than another extension
search. Its generalization remains unselected. The variation note is marked
unverified and needs reconstruction. Neither finite tests nor a PDF build
establish all mathematical implications or prose quality. Time, shared source
ownership and integration workload are the remaining delivery risks. No full
rewrite, new theorem search or COVER-L campaign is included.
