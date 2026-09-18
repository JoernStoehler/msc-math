# Historical comparison: c002 narrative transfer

## Scope and frozen evidence

This compares [baseline-narrative.md](baseline-narrative.md) and [reader-question-narrative.md](reader-question-narrative.md) with the c002 human records for [the supplied two-page extraction](../inputs/narrative-transfer.txt). Both review hashes match [round-2-freeze.json](round-2-freeze.json), frozen at `2026-09-18T12:29:30.958320+00:00`. No reviewer predictions were added or revised during scoring. Only c002 records from the historical labels file and their associated `docs/review-evidence/human-feedback/ds-reading-2026-09-14.md` anchor were read for this scoring task.

**Direct** means the review identifies the labeled concern and its mechanism at the relevant scope. **Partial** means it identifies a related component but not the full objection. **Missed** means it does not identify the objection. **Unadjudicated** means the human record does not settle a reviewer concern. Explicit endorsement of a human-rejected feature would be recorded as disagreement, separately from silence. Positive prose evidence and negative organizational evidence are assessed separately.

## Exact human labels

All three c002 labels are active and attributed to `direct_human_quoted`. The following reproduce the exact `evidence_quote` fields, including spelling and qualifications; no withdrawal is recorded.

### l009: sentence_level_readability (positive)

> it is actuallyl comfortable to read, but it lacks structure. i.e. writing quality is good / passing,

### l010: narrative_structure (negative)

> but narrative is now too chaotic to follow. it starts okay/mediocre as an introduction.

### l011: narrative_structure (revision_requested)

> i think there should be subsections maybe? like, we need to establish first in the motivating opening of the chapter that we use random polytopes (reference to a later subsection wrt what distributions we sample) and compute a colorful basket of different symplectic invariants and non-invariants that we come up with. Then we basically forget about the semantics, and simply use data science to look for low-entropy patterns detectable using stnadard data science methodology, patterns that we predict occur when there's deeper matehmatical insights that cause these patterns. The hope is that we find something, then reconstruct a semantic interpretation of what we have found in the form of a fuzzy conjecture, and look for ways to proof a rigorous conjecture and extract from the proof the insights we care about.

## Structural concerns

| Human evidence | Baseline | Reader-question specialist | Reason |
| --- | --- | --- | --- |
| l010: narrative “too chaotic to follow”; introduction “okay/mediocre” | **Missed** | **Partial** | Baseline raises missing experimental evidence and error interpretation, but identifies no organizational failure. The specialist flags purpose-after-construction for the ridge descriptor and an unexplained choice of tangent-polygon modification, and proposes ordering changes. These are genuine local narrative obstacles, but it describes the route as “largely accessible” and does not identify the broader organizational failure the human reports. |
| l011: reorganize around discovery from random objects and varied descriptors, through statistical pattern detection, to semantic interpretation and conjectures/proof-directed insight | **Missed** | **Missed at the requested structural scope** | Neither review recovers this discovery-to-insight organizing sequence, the broad basket of invariants and non-invariants, or the separation of motivating opening from later distribution detail. The specialist's two local motivation repairs do not amount to that architecture. Its l010 partial credit must not be counted again as detection of l011's specific revision direction. |

The tentative “subsections maybe?” in l011 is preserved as a suggestion, not promoted to a categorical heading requirement. Neither review proposes subsections, but the more consequential mismatch is the requested organizing account of the research. The human's hoped-for conjecture-to-proof sequence is a desired explanatory framework; it is not evidence that the research completed that sequence or proved a conjecture.

The specialist's accessible-route framing is in tension with the severity of l010. It is not an explicit endorsement of the existing organization as a whole: the review itself proposes structural changes. Its praise for two particular transitions therefore does not establish a direct local contradiction, because c002 does not individually reject those transitions.

## Comfortable prose is a separate positive result

| Human evidence | Baseline | Reader-question specialist | Limit |
| --- | --- | --- | --- |
| l009: “comfortable to read” and writing quality “good / passing” | **Compatible local endorsements; no equivalent whole-sample judgment** | **Compatible accessibility judgment** | Baseline praises three specific passages; the specialist calls the route “largely accessible” and retains two transitions. These can coexist with comfortable sentences. They do not establish a full-chapter PASS, resolve l010, or justify accepting the organization. |

The human expressly separates sentence-level reading comfort from narrative quality in the same feedback. Treating the positive label as an overall acceptance would erase that distinction. Conversely, the structural criticism does not justify labeling all sentences difficult. The specialist's phrase “route is largely accessible” is broader than a sentence-level claim and should not be counted as an exact prediction of l009's qualified judgment.

## Local concerns and unadjudicated predictions

There are no separately labeled sentence-level defects in c002 for this frozen sample. The following reviewer concerns therefore cannot be scored as direct human-confirmed local detections or confirmed false positives:

| Frozen concern | Historical status |
| --- | --- |
| Baseline: selection evidence lacks definitions of controls, selection method, and comparison outcome | **Unadjudicated.** The human's organizational rejection does not resolve these experiment-reporting details. |
| Baseline: numerical accuracy is not tied to reported changes | **Unadjudicated.** Comfortable prose does not adjudicate whether computational error matters for these comparisons. |
| Specialist: ridge descriptor purpose arrives after construction | **Unadjudicated as a specific local defect; related partial structural evidence under l010.** The human requests a broader motivating opening but does not specifically endorse moving the low-ridge association ahead of the definition. |
| Specialist: choosing equal heights lacks an actual motive | **Unadjudicated as a specific local defect; related partial structural evidence under l010.** The broad structural criticism does not specifically identify this heuristic choice. |

Baseline's conditional missing-context observation about “AI-assisted exploration” is likewise not adjudicated by these labels. No confirmed false-positive concern can be established from this record; that is a statement about missing adjudication, not proof that all reviewer concerns are valid. Related local obstacles can support a partial match to a broad complaint without becoming separately confirmed local defects.

## Associated-anchor qualifications and evidence limits

The source anchor identifies the human-read artifact as `sample.pdf`, SHA-256 `01d63be3d72999230e98c0b5fb6f89dcba33d532188d83894f4ad01a9f62c58c`, with feedback received September 14, 2026. Its scope qualification is:

> Applies to this sample, not blanket assessment endorsement or full-manuscript PASS.

The anchor later records feedback on two revised openings. Those are different artifacts and are excluded from scoring these frozen reviews. To preserve the qualifications without applying later judgments to the original sample, the anchor says:

> The following is the coordinator's September14 relay of Jörn's feedback, not an independently recovered verbatim user message:

It then limits that feedback to “revised opening only, not approval of remainder.” For the later opening-v2 it says:

> Approval is specific to the opening sentence, not the paragraph or remainder.

Those later opening judgments neither withdraw the original structural objection nor create additional original-sample defects. The original c002 labels already supply the relevant positive/negative distinction and desired revision direction.

This is a selected historical sample, not exhaustive annotation or an independent proof/empirical-method audit. The supplied extraction and associated feedback establish the comparison's content; this scoring did not verify binary identity with the original PDF. No accuracy, precision, or recall estimate follows from these three selected labels. The record is sufficient to show that the specialist noticed local motivation/order problems the baseline did not, while neither review recovered the human's requested chapter organization. It does not establish comparative performance on other chapters or that the broader harness was historically unexposed to this case.
