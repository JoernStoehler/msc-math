# Historical comparison: c004 Chapter 4

The [baseline](baseline-quadratic.md) and [qualifications specialist](qualifications-quadratic.md) match both hashes in [quadratic-freeze.json](quadratic-freeze.json), frozen at `2026-09-18T12:45:18.964395+00:00`. This scoring uses [the supplied Chapter 4 extraction](../inputs/quadratic-transfer.txt), only c004 labels, and their associated `docs/review-evidence/calibration/whole-thesis-reading-1700.md` anchor. No predictions were changed.

**No specific concern or retained passage in either frozen review has an active local human adjudication in these records.** There is a withdrawn reconstruction/motivation objection and qualified chapter-level positive evidence. Neither supplies clean local labels.

## Exact applicable human records

The following exact `evidence_quote` fields are coordinator records of human feedback. Labels concerning other chapters are excluded.

### l028: objection_withdrawn — withdrawn

> Jörn later raises, then withdraws, a Chapter 4 reconstruction/motivation
> annotation after recalling the earlier dual-reconstruction theorem. No edit
> or new confirmed defect is attributed to that withdrawn annotation.

### l029: positive — active

> Jörn subsequently judges Chapter 4 safe from the math/CS audience mismatch:
> “it stays nicely on the math side! readable for a mathematician like Kai!”

### l030: PASS_borderline — active

> Jörn then confirms he finished Chapter 4: it is appropriate and “would be a
> PASS”, but writing quality is “borderline”.

## Local comparison

| Frozen judgment | Existing human status |
| --- | --- |
| Specialist concern: repeated denial that every maximizer has support at most six | **Unadjudicated.** No local label on Remark 4.8. |
| Specialist concern: singularity does not imply infeasibility or failure of the capacity formula | **Unadjudicated.** No local label on this sentence. |
| Specialist concern: “without solving KKT systems” advertises an avoided solver step | **Unadjudicated.** No local label; preserve the specialist's lower confidence and acknowledgment that the comparison may justify retaining it. |
| Baseline concern: numerical terminology in section 4.5 is too concentrated | **Unadjudicated locally.** The chapter-level math/CS audience endorsement is relevant counterevidence to a broad audience-mismatch claim, but does not explicitly settle this paragraph. |
| Baseline concern: “block” changes scope during merging | **Unadjudicated.** No local label on this terminology. |
| Specialist retains the stationary-versus-maximum qualification | **Unadjudicated.** No local endorsement or rejection. |
| Specialist retains the boundary-realized-global-minimizer qualification for transition pruning | **Unadjudicated.** No local endorsement or rejection. |
| Specialist retains surgery in the dual admissible class; baseline retains the related warning that splitting need not stay inside the polygon | **Unadjudicated.** l028 is thematically related, but gives no exact passage and withdraws the objection after recalling prior reconstruction. It cannot establish a defect, a confirmed retention judgment, or a direct match to either quoted passage. |
| Baseline retains the four-facet upper-bound/global-comparison distinction and the six-facet mechanism-before-notation opening | **Unadjudicated.** No local endorsement or rejection. |

There are no confirmed direct detections, partial detections, or explicit local disagreements to assign from these records. This is lack of local adjudication, not a finding that the reviewers have no valid concerns. Their novel concerns cannot be called false positives merely because the chapter passed.

## Qualifications and limits

l028 remains **withdrawn**: it is neither a confirmed defect nor a positive local label. l029 is positive audience/readability evidence for Chapter 4 of the fixed **17:30** PDF. l030 records a completed chapter-level **PASS with “borderline” writing quality**; that reservation stays attached. Neither label grants whole-thesis approval or approves every sentence. The baseline's generally well-signposted assessment is broadly compatible with the chapter-level positive evidence, without making its local endorsements human-confirmed.

The associated anchor explicitly says:

> The independent predicted QP
> issues remain agent suggestions, not human-confirmed defects.

That describes the historical predictions, not the fresh frozen reviews; the same evidentiary distinction applies here. The anchor filename/header refers initially to a 17:00 snapshot, but the applicable Chapter 4 feedback is under the later 17:30 reading. This scoring does not transfer the earlier PDF hash to the later artifact or verify historical binary identity of the extraction.

Fresh microjudgments would be needed to adjudicate any of the five current concerns or six retained passages. These selected chapter-level labels support no reviewer accuracy estimate and do not justify interpreting unmentioned passages as clean.
