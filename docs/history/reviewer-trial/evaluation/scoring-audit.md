# Independent interpretation audit

Audited the current human comparison, microbatches 1–3, run record and README on 2026-09-18. Also read the alternate-draft selection record and freeze manifest for the M–O timing and selection boundary. This is an interpretation audit, not an independent hash verification or new reviewer evaluation. P–S remain pending and unscored.

## Findings

The current [human comparison](human-comparison.md) and human-response interpretations support their bounded conclusions. No new confirmed false alarm, general accuracy estimate or deployment claim follows from this audit.

| Question | Supported interpretation and limit |
| --- | --- |
| Source and label exposure | Fresh evaluator workers were restricted to frozen prompts and sources, but the texts share project lineage, prompts were adapted from earlier feedback, and the parent knew some historical summaries. Human adjudication was not blinded: progress summaries preceded some answers, including C, and reviewer endorsement was disclosed before B's follow-up. Worker separation does not remove those limitations. |
| M | An autonomously located exact-span concern received human confirmation, with a matching removal remedy. The human question supplied the true fact that the name was unused in the subsection. This is useful narrow transfer from earlier code-detail evidence, not a representative estimate of discovery or repair performance. |
| N | The negative-alternative concern was confirmed, while the proposed retained positive clause itself drew criticism. Detection at the marked clause succeeded; an adequate final repair was not established. Its rhetorical pattern overlaps development evidence, including C. |
| B | Initial concern was conditional on missing context. The full paragraph received an explicit local PASS. Preserve both judgments; do not count B as a confirmed false endorsement or extend the paragraph PASS to the manuscript. |
| C | The manual prediction preceded the request. The qualifications review preceded the answer but followed the request; its overlap is secondary evidence. The human had also seen a summary mentioning the concern. Neither timing nor exposure supports treating it as a blinded, prospectively requested test of that specialist. |
| J and K | Span overlap is not full diagnostic or remedy agreement. J's reviewers mainly wanted more explanation while the human questioned excess detail. K's substantive observation was useful, but the classifier missed wording defects and the relevance review's larger requested expansion was not endorsed. |
| O | The preferred action repairs the upstream evidence and reproducibility state. Fixed-fact retention does not answer that action request, but O is not a binary false endorsement or permission to erase a currently valid caveat. |
| Selective labels | Historical counts and purposive microtests support local matches and misses only. Unmarked passages are unknown; unadjudicated concerns are not false positives. No precision, recall, overall accuracy or automatic-repair competence is established. |

## Concrete documentation corrections

At audit time, README lagged behind the response records: its classification and qualifications rows still described adjudication as pending, its evidence/workflow summary omitted M/N, and its pending-decision and effort paragraphs reflected earlier responses. Refresh these from the received judgments and distinguish remaining P–S judgments from completed cases. Its evaluator-run count also predates the alternate-draft and quadratic runs; recompute from the run inventory before retaining a numerical total.

RUN.md is chronological, so earlier pending statements are understandable as history. The unqualified “No requests yet” under its initial Human effort heading is nevertheless easy to read as current status; label it as the initial state or replace it with a pointer to the later effort record. Keep effort allowances explicitly uninstrumented and avoid presenting summed estimates as measured active time.

These are documentation synchronization issues. They do not change the bounded interpretations in the current human comparison.

## Parent disposition after this audit snapshot

README was rewritten and synchronized with received judgments, eighteen evaluator runs/seven texts, and updated effort estimates. RUN now labels “No requests yet” as the initial state. P–S were subsequently adjudicated: two false alarms, one unscored context request, one retained clarification. Original frozen reviews remain unchanged.
