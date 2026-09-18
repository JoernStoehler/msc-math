# Historical comparison: c001 topic transfer

## Scope and scoring rules

This is a retrospective comparison of the already frozen [baseline review](baseline-topic-transfer.md) and [relevance specialist review](relevance-topic-transfer.md) against c001 human records, using [the supplied topic-transfer extraction](../inputs/topic-transfer.txt). No reviewer predictions were added or revised after reading the labels. Only c001 label records and their associated `docs/review-evidence/calibration/hko-reading-feedback.md` anchor were consulted from the writing-detector-data checkout. Historical comments in that anchor about an older prospective prediction are not scores of either fresh review.

A **direct detection** identifies the local objection and its reading mechanism; **partial** identifies a related obstacle but leaves part of the labeled problem unstated; **missed** means the frozen review does not identify that local objection. **Contradicting local endorsement** means the reviewer expressly approves the feature the human objects to; it is stronger evidence of disagreement than silence. Praise for a different property of an overlapping passage is not automatically a contradiction. These are qualitative passage-level comparisons, not accuracy estimates.

## Exact human label wording

All eight c001 records are active. They are attributed to `coordinator_record_of_human`: the following are exact `evidence_quote` fields, not a claim that every word is a direct transcript of Jörn. The source anchor records no withdrawal of these objections.

### l001: computational_attribution (revision_requested)

> Jörn quoted “The exact calculation checks that these fifteen vectors are
> independent” and suggested “An exact calculation using a computer algebra
> system”.

### l002: unintroduced_reference (problem_identified)

> Second annotation: Jörn objects to “Use certificate entry 2 (the third entry
> of the zero-based witness)” because the reader does not yet know the referenced
> object. He suggests a footnote to code files or a source comment if only agents
> need it.

### l003: motivation (problem_identified)

> Third annotation: the heading “One actual seven-facet feasible section” lacks
> motivation. Jörn proposes explaining that this is a worked presentation of
> one of 26 upper bounds to showcase the methodology, that the others work
> analogously, and that verification used a computer algebra system rather
> than hand computation.

### l004: unsolicited_negative_alternative (problem_identified)

> Fourth annotation: Jörn objects to the passage distinguishing the HKO
> capacity identification from feasibility, especially “not the reverse bound”.
> It rules out a misunderstanding the reader has not entertained.

### l005: missing_inference (problem_identified)

> Fifth annotation: “It illustrates why the forty-coordinate calculation is
> stronger than differentiating only within products” is too dense and skips
> an inference.

### l006: explanation_order (problem_identified)

> Sixth annotation: the start of the worked-coordinate subsection gives h2 and
> the zero remaining components before explaining the intended perturbation.
> Jörn finds this too compact/elliptical and proposes describing the geometric
> direction first.

### l007: code_advertisement (problem_identified)

> Seventh annotation: the matrix sentence's “without floating hints, branch
> search, or printing 1040 expanded field entries” reads as an advertisement
> for code rather than mathematics.

### l008: overall_acceptance (PASS)

> Closing judgment: Jörn believes he finished the supplied PDF and has no further
> noticed issue to raise. He says “overall i think this would already be a PASS
> grade”, while noting that improvements could compensate for problems elsewhere.

## Mapping the seven local objections

| Human label | Baseline | Relevance specialist | Evidence and scope |
| --- | --- | --- | --- |
| l001: name computer algebra as the means of the exact calculation | **Missed** | **Missed** | Both ask for an identifiable verifier/result, but neither asks that the fifteen-vector calculation be attributed to a computer algebra system. Baseline praises this exact passage for explaining why independence matters; that praise addresses a different property and is not an explicit rejection of CAS attribution. General computational-evidence concerns do not detect this local wording request. |
| l002: certificate entry refers to an object not introduced | **Partial** | **Direct detection** | Baseline concern 1 identifies the undefined location of the “existing verifier”/“accompanying computational witness,” but does not isolate the certificate-entry opening or explain that its object is unknown on first encounter. Specialist concern 2 quotes that opening, questions what the storage indices contribute, proposes relocating them to a verification note, and qualifies this by the absence of an established adjacent-verifier reading arrangement. This matches the local reference problem and proposed separation of code traceability, though its emphasis is interruption rather than explicitly saying “unintroduced.” |
| l003: explain the seven-facet example's purpose as one of twenty-six analogous constructions | **Missed** | **Missed** | Neither asks for the missing heading/opening motivation or CAS framing. Their praise for the later singular-Hessian argument explains a mathematical distinction within the example; it does not identify the missing introductory purpose. Specialist concern 2 at the adjacent certificate reference is not a second hit on motivation. |
| l004: unsolicited “not the reverse bound” alternative | **Missed** | **Contradicting local endorsement** | Baseline's praise stops at feasible continuation versus an optimizing branch, without assessing this later negative alternative. Specialist acceptable passage 2 expressly calls “Feasibility yields c ≤ A, not the reverse bound” a “necessary logical qualification, not an operational distraction.” This directly endorses the rhetorical feature the human objects to. Preserving the mathematical source of equality is compatible with removing the unsolicited alternative; the human did not ask to drop that source. |
| l005: missing inference in the forty-coordinate-versus-products closing sentence | **Missed** | **Missed** | Neither identifies the skipped explanation that a momentum component of a q-facet normal falls outside the fixed product family's variations. Specialist praise for the earlier contribution/scope statement does not assess this later inference. |
| l006: describe the geometric perturbation before giving h2 and its zero components | **Missed** | **Missed** | Neither requests this ordering change. Specialist concern 2 quotes the next sentence's flat storage coordinate, but that code-reference issue does not identify the preceding geometry-before-symbols problem. |
| l007: the “without floating hints…” tail advertises code | **Missed** | **Missed** | Neither quotes or challenges this tail. Specialist concern 2 identifies other implementation intrusions; it does not flag this sentence, so topical similarity is insufficient for a passage-level hit. |

The direct/partial distinction on l002 is a scoring judgment, not a new human adjudication of the fresh reviews. A stricter requirement that a direct detection literally articulate “unintroduced object” could downgrade the specialist's l002 match to partial; its exact location, interruption diagnosis, and proposed relocation are the evidence for the direct classification here.

## Qualifications, acceptance, and presentation requests

The associated anchor adds the following exact qualifications to the label fields:

> He added that he was continuing through the PDF, and that this was
> the first point he had found worth annotating: “great work so far!”

This is reported positive feedback on reading up to the first annotation, not an exhaustive clean label for that span. Both reviews' endorsement of the opening decrease mechanism is compatible with that feedback, but it does not establish a specifically adjudicated true negative.

> This is approval of the HKO draft he read, not of the assembled thesis.
> He additionally requested named lemmas, illustrative upper-bound/symmetry
> figures, and an annotated CAS appendix. Those presentation improvements remain
> in progress and are not silently waived by this passing judgment.

The chapter-level **PASS** in l008 permits all seven local defects. It neither withdraws them nor licenses whole-thesis acceptance. Neither frozen review assigns a comparable overall grade, so there is no fresh overall PASS prediction to score. The specialist's “No figure is needed to answer an evident unresolved empirical question here” also does not settle the distinct human request for illustrative mathematical figures.

The anchor describes candidate edits after individual annotations, while the human continued reading the fixed PDF. In particular l007 explicitly notes that another candidate had already removed the tail. Those editing responses are not reviewer predictions and are not credited as detections. The visible topic-transfer extraction retains the quoted target features, supporting this local comparison despite the distinct extraction format.

## Novel reviewer concerns and false-positive status

Both reviews' central concern about an identifiable successful rank/strict-sign verification report is **unadjudicated as framed**. It overlaps the human's interest in computational attribution and an introduced witness, but the c001 labels do not adjudicate whether the proof's result-reporting/evidence-location presentation is inadequate in the exact way these reviews allege. Both reviews also qualify that concern by potentially missing surrounding context. It is not a confirmed false positive merely because Jörn reported no further noticed issue.

The specialist additionally singles out “This is flat coordinate 6 in the verifier’s zero-based storage.” That particular index objection is **unadjudicated**, although its certificate-entry counterpart matches l002. The human's different ordering complaint in this subsection does not adjudicate every neighboring sentence. There are **no confirmed false-positive concerns established by these selected records**; this means the evidence does not establish any, not that the reviews have none. The specialist's express endorsement of the l004 feature is separately recorded as an observed disagreement.

## Evidence limits

This is one selected historical mathematical chapter with seven recorded local objections and a qualified overall acceptance. It is not an exhaustive sentence-by-sentence annotation, a representative test set, a proof audit, or a controlled estimate of reviewer accuracy. No precision, recall, or comparative accuracy estimate follows from these selected labels. Unmentioned text remains unlabeled.

The source anchor identifies the human-read artifact as `.git/codex/hko-writing/chapter-preview.pdf`, SHA-256 `c5b4508fb226971d2c5d68d8bd8f8b32cdd727272dfc3b562a1e3f993c176202`, with feedback received September 14, 2026. This comparison checked the quoted features in the supplied extraction, not binary identity of that PDF. The labels are coordinator records of human feedback, with direct human quotations embedded, rather than independently re-collected judgments. The reviewers worked from the supplied extraction and audience assumptions, and their missing-context qualifications remain relevant. Their reviews were frozen before this scoring pass; this does not establish that the broader harness or its authors were historically unexposed to the case.
