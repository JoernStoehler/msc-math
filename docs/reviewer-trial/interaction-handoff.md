# Interaction handoff to main — retrospective, research remains paused

This answers main's targeted process-knowledge question. No experiments, subagents, user questions, or maintained interaction instructions were started or changed. I have not audited main's transcript, so I cannot independently establish what main forgot or violated.

## What was observed here

1. **A concrete passage and a bounded judgment elicited useful answers without a document review.** Questions identified the exact phrase/sentence, supplied local context, and asked keep/change/need context. Repairs and explanations were optional; Jörn sometimes supplied them anyway. M elicited a direct removal judgment; L and S elicited short keeps. This supports cheap targeted judgments, not a requirement for binary-only replies.
2. **Shortness alone was insufficient.** The first A–D bundle produced “not quite sure what hte question is”; D could not be evaluated because its references were missing. The intended question had to be clarified as whether he would change the passage for the reader. “Approve this map?” would lose both the concrete object and the intended judgment. That comparison is an inference, not an observed map test here.
3. **Supply the context; do not make him reconstruct it.** B initially looked unclear in isolation. Showing the real correlation paragraph changed his response to “great text of PASSing quality and high value wrt what content it presents!” Context can be one paragraph, not a whole chapter. D/R still needed more context and were not forced into labels.
4. **Recognition can be cheap.** Jörn estimated “prob 10-45s per question? unsure about question count.” There were 16 passage judgments, implying roughly 3–12 minutes by that estimate, not measured timing. No long PDF review was needed for those decisions.
5. **Allow an answer to change the action, not just a label.** For O he wanted the repository's reproducibility repaired and explicitly authorized reports to main about research-maintenance gaps that burden prose. I prepared and sent the concrete issue report instead of insisting on keep/remove or asking him to recover repository facts.

Evidence: human/microbatch-1.md (initial confusion, D, B reversal), human/microbatch-2.md (F/J/K/L), human/microbatch-3.md (M/N/O), human/microbatch-4.md (P/Q/R/S), human/time-budget.md.

## Two actual successful questions

### M: narrowly scoped phrase judgment

Actual request:

> M (~20 seconds): Would you keep or change the phrase “called factorial-both” here? The label is not used elsewhere in this subsection.
>
> “A separate transfer experiment used frozen rho and ridge selectors on a different, separately factor-area-normalized source, called factorial-both. Selected means exceeded disjoint-control means in both its 4×6 and 6×6 buckets, across 91 distinct evaluated bodies.”
>
> Only judge the code-label phrase; other problems can remain.

Choices: change/remove that phrase; keep that phrase; need more context.

Response: “remove, i see no purpose of providing a name that is never used (?)”. Path: human/microbatch-3.md, M. Success was a usable local judgment, not approval of the paragraph or proof of optimal UI.

**Adaptation required by main's current instruction:** put the prediction in the async item itself: “I expect we should remove this unused code label because it adds no mathematical information. Is that right?” State the consequence as removing this phrase only. This predicted-answer sentence was not part of the original request; do not rewrite history as if it was.

### B: context repair, not escalation to a full review

Actual request opening:

> Context check for B (~45 seconds): does the full paragraph below resolve your concern about the marked sentence, or would you still change it? This tests whether the snippet hid needed context; no rewrite needed.

Then the request included the complete paragraph beginning “The pooled rank correlation between R and the numerical systolic ratio is about −0.94,” including the rank definition, linear correlation, marked sentence, within-group results and figure reference. The exact source is inputs/transfer-source.md; locate that opening. Choices: still a writing problem; context resolves it; cannot judge yet.

Response: local PASS/high value, quoted above. Path: human/microbatch-1.md, “B: full-context follow-up”.

**Current adaptation:** include “I expect this paragraph resolves the ambiguity because it explains the two correlation measures; if you agree I will retain the paragraph.” Include the paragraph in the queue item, so he does not have to retrieve it or read preceding commentary.

## What was conveyed, and what was not

Main received trial updates about short async judgments, the B context reversal, human effort, upstream issue O, and later the correction that two false alarms alone do not establish negative utility. README records those methodological findings. I did **not** separately hand off the successful request shape as an interaction contract: exact object + enough context + one bounded judgment + optional explanation + ability to answer “need context.” This note fills that gap. I cannot say main forgot something it was never explicitly handed.

The following are **current instructions relayed by main in this handoff request**, not preferences established by this trial: Jörn reads only the async queue; final response means final PDF submission; files to his phone must use Taildrop; requests must expose the predicted answer so he can correct the model. My trial used async questions but also commentary and ordinary final responses, did not use Taildrop, and generally froze rather than consistently showed predictions. Its method is not authority to disregard the current instructions or to run blinded questions when a prediction is requested.

## Inferred application to the reported main failures

Put the whole decision in the async queue item: concrete excerpt/artifact, exact choice and consequence, predicted answer with a short reason, and realistic effort. Do not depend on commentary/final text for facts needed to answer. A small set of independently answerable cards is a plausible mobile-friendly tactic; optimal batch size was not tested. Keep authorized independent work moving while questions wait, without treating silence as approval. For a necessary long artifact use the now-required Taildrop route; do not turn a local wording judgment into a long mobile review by default. Reserve final submission for the actual final PDF under main's current contract. These are applications of the relayed instructions and observed interaction, not newly discovered user preferences.
