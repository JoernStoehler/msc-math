# Qualification reviewer: surface, yield, and utility correction

Jörn challenged the conclusion that two false alarms make this reviewer unsuitable: absolute counts need the reviewed surface, useful findings, false-alarm burden, and intended workflow. That criticism is correct. The initial recommendation against broad use inferred too much from the existence of errors.

## Complete flag inventory

This is the same unchanged qualification prompt in four evaluation runs. Approximate length counts whitespace-separated source text/formula tokens, not model tokens or exact prose words. The two DS drafts share content and research lineage; these are not four independent samples.

| Reviewed input | Approximate length | Flags issued | Human-confirmed concerns | Confirmed false alarms | Unresolved |
| --- | ---: | ---: | ---: | ---: | ---: |
| Revised DS | 1,112 | 2 | 1 (C) | 0 | 1 unasked |
| Pentagon proof, four PDF pages | 1,433 | 0 | 0 | 0 | 0 flags; missed defects unknown |
| Alternate DS | 898 | 2 | 1 (N) | 0 | 1 unasked |
| Quadratic-program chapter, eight PDF pages | 3,988 | 3 | 0 | 2 (P/Q) | 1 context-insufficient (R) |
| **Total evaluated surface** | **7,431** | **7** | **2** | **2** | **3** |

C's reviewer output was frozen before the answer but after the human request, and the human saw a progress summary of that concern. N was frozen before its request. Neither has the status of an independently blinded validation item. Both nevertheless identify actual concerns the user endorsed. N's proposed repair was incomplete; detection benefit and repair quality differ.

All three flags from the quadratic chapter were presented for judgment: two were rejected and one was unresolved. Thus the two false alarms came from that eight-page chapter, **not a whole-thesis pass**. The other three runs add reviewed surface and two useful findings; the zero-flag pentagon run is not evidence that no defects were missed. No thesis-wide extrapolation is justified.

## What the counts do and do not imply

Of the four flags with a definite human concern/false-alarm judgment, two were useful and two were false alarms. That describes these judgments; selective elicitation, shared content and three unresolved flags prevent treating it as a general precision estimate. Recall is unknown. Review length and output caps also affect extrapolation.

A human-triaged concern generator can be worthwhile with false alarms. The relevant balance is the value of found defects and avoided rereading against the time spent inspecting all flags, resolving context, evaluating repairs, and any harm from accepting a bad suggestion. At Jörn's rough 10–45 seconds per question, adjudicating P/Q would amount to about 20–90 seconds, if those judgments were typical; their actual individual times were not measured. That burden alone does not establish negative utility. Conversely, the trial did not measure end-to-end savings or the cost of applying and checking edits.

## Corrected recommendation

Keep the qualification reviewer as a plausible **human-triaged flagger**, alongside the reproduction-detail candidate. Its overall utility remains unmeasured, not disproved. Do not automatically delete its flagged text or infer acceptance from silence. The historical misses remain relevant to acceptance gating, but they do not require a flagger to be comprehensive.

A useful next evaluation would run on a complete chapter or thesis section, adjudicate its entire flag list, sample unflagged text for important misses, and record accepted improvements and total human effort. That would compare actual benefit and burden on a defined surface. This correction does not launch a new trial or extend the completed trial's human budget.
