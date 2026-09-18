# Human-label curation notes

These 37 JSONL records are bounded observations from existing feedback records, not independent examples sampled from a population. They cover six case IDs: c001 (8), c002 (3), c003 (13), c004 (6), c005 (1), c006 (6). Labels are deliberately not exhaustive: no annotation means unknown, not acceptable writing.

## Provenance and scope

- `c001`: HKO frozen seven-page chapter. Seven localized objections and one completed chapter PASS. The PASS permits those imperfections. Suggested additional figures, lemmas and CAS exposition are not converted into extra failures.
- `c002`: only the first, explicitly quoted two-page DS sample feedback. Sentence comfort and narrative failure remain separate. The longer structure request is one record rather than many pseudo-independent labels. Later opening revisions in the same source are excluded because they concern different text.
- `c003`: comments before the explicit switch to the 17:30 PDF in `whole-thesis-reading-1700.md`; these concern the 17:00 snapshot. Provisional introduction impressions stay provisional. Coordinator repair descriptions within a quoted source span are context, not additional human judgments.
- `c004`: comments after that switch, concerning the 17:30 snapshot. Chapter 4 has a completed **borderline PASS**, a positive audience assessment, and a withdrawn motivation objection. The withdrawn record is not an active defect. The Section 13.1 attribution correction is separate and remains active.
- `c005`: only the coordinator-reported human whole-thesis FAIL. This does not elevate the AI self-review's findings to human labels.
- `c006`: pentagon excerpt/feedback-only case. The originally served PDF was overwritten, so these records cannot identify a single exact complete reviewed chapter. The proof acceptance and exposition rejection are deliberately combined to preserve the qualification. No label is inferred for the later v2 chapter.

`reviewer_kind: human` describes whose judgment is reported, not the author of the record file. Most records have `attribution: coordinator_record_of_human`; only verbatim passages explicitly presented as the human's words have `direct_human_quoted`. Even the latter were preserved by a coordinator rather than independently recovered from the original message log. `evidence_quote` is an exact substring of the source Markdown, including line breaks where present. Optional `target_quote` is an exact manuscript fragment quoted in the feedback record, not a claim of full source-text reconstruction.

`status: active` means the feedback was not withdrawn; it does **not** mean the defect persists in today's thesis. Repairs described in the sources affect other revisions. The one `withdrawn` observation is retained to prevent accidentally treating it as an active negative label.

## Interpretation limits

The records mix acceptance, requested revisions, local defects, provisional impressions and a withdrawal; the judgment vocabulary intentionally preserves these differences. They are not 37 binary PASS/FAIL examples. Their `dimension` values are curator descriptions for navigation, not an exhaustive error taxonomy. Compound source observations remain compound when separating them would suggest unsupported independence.

Exact frozen artifact identities and availability belong to the case manifest. Group evaluation by artifact lineage/topic and feedback time rather than splitting these labels randomly. Reviewer predictions, later repairs, calibration notes and these labels leak the answers; do not expose them to a purportedly blind evaluator. Human acceptance of a chapter does not approve every selected excerpt independently, and mathematical proof approval is not prose approval.

## Checks performed

- Parsed 37 JSON objects with unique contiguous label IDs.
- Checked every evidence quote as an exact substring of its repository source.
- Confirmed c002 evidence ends before the revised-opening section.
- Confirmed c003/c004 evidence falls respectively before/after the snapshot-switch heading.
- Retained exactly one withdrawn record and no v2 human acceptance label.
