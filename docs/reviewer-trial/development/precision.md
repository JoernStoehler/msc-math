# Precision specialist: development record

## Scope and evidence

Candidate prompt: [precision-prompt.md](precision-prompt.md). Developed only from [development-source.md](../inputs/development-source.md) and [development-labels.md](../inputs/development-labels.md), using the harness-engineering-v2 skill. No transfer text or historical dataset was read. This is a bounded specialist for reader fit and visible mathematical content, not scientific verification or acceptance prediction. Paragraph numbers below count prose paragraphs after the title, as in the human anchors; displays belong to their surrounding paragraph.

The source's human verdict is “Needs revision before acceptable.” That verdict does not turn every passage into a defect. The prompt remains a trial artifact, not an activated standing instruction.

## Hypotheses

1. Requiring an explicit reader obstacle should find reference and jargon problems that a correctness audit can overlook, while discouraging preference-only rewriting.
2. Checking what a sentence contributes in context should distinguish weak result visibility from useful abstract transitions. Requiring three defended acceptable passages may reduce indiscriminate criticism; this is untested and could instead produce formulaic praise.
3. An explicit mathematical reader profile should catch the asymmetry between explaining elementary scaling and presuming familiarity with correlation measures or ML vocabulary.
4. A five-concern cap with duplicate grouping should keep the review actionable. It may lose coverage when many independent issues compete; this specialist does not claim to reproduce all twelve annotations.

## Known development cases

| Anchor | Evidence and expected treatment |
|---|---|
| Title, “Ridge geometry as a guide to candidate search” | Human prefers a title communicating the obtained result. Detect weak result visibility; the suggested alternative and its use of “capacity” are tentative, not authoritative terminology. |
| ¶1, “Random sampling can do more than return the largest systolic ratio encountered.” | Initial low-information objection was reconsidered: good if following a section on maximizing the sampled systolic ratio. A concern must be conditional on absent context, or this may be accepted as orientation. |
| ¶2, “The initial population comprised 4,096…” | Human wants the phenomenon before sampling detail. Relevant to visibility, not authority to suppress numerical limitations or fabricate statistical significance. |
| ¶3, “since both the face integrals … scale quadratically” | Human considers this immediate for an MSc student. Candidate mismatch of explanation level, not a mathematical error or a universal ban on scaling explanations. |
| ¶4, “stored targets”; ¶6, “the ratio” | Human objects to indirect quantity names. Prefer explicit systolic-ratio wording while preserving the historical numerical-estimate distinction. A locally recoverable referent can still impose avoidable tracking effort. |
| ¶4, “Spearman rank correlation … Pearson correlation” | Human does not expect reader familiarity. The following ordering-versus-linearity sentence partially explains the meaning; evaluate whether it suffices. Digits criticism is tentative; precision alone is not an error. |
| ¶5, “Products make that information more concrete.” | Human withdrew the pentagon-first/general-later sequencing objection and accepted keeping general and specialized formulas together. Retained concern: visibly signal the shift from empirical pattern to mathematical explanation. Do not label the whole transition empty by default. |
| ¶6, “This is an analytic example outside the finite random table…” | Human judges the aside out of place, not false. Assess its local contribution; do not discard the scientific distinction merely to shorten text. |
| ¶7, “Small ridge sum is therefore not a universal ascent objective.” | Explicit positive human judgment: “nice result.” Moving it to the paragraph opening is tentative. Preserve the result and distinguish a placement suggestion from a content defect. |
| ¶8, “Predictive models and feature ablations … held-out signal” | Strong reader-fit case: unexplained ML language and vague contribution. Human's account of filtering new data to test persistence is a proposed interpretation, requiring source checking before adoption. |
| ¶9, “Geometry-guided selection and local refinement consequently answer different questions.” | Human finds the first two closing sentences irrelevant to mathematicians and ineffective closure. Scope does not extend automatically to all remaining sentences. A true distinction may still fail to foreground the section's mathematical result. |

## Negative cases and false alarms

Useful candidate positives include ¶3's explicit definition of the ridge descriptor and facewise absolute values; ¶5's mixed-rectangle formula and explanation of edge pairings; and ¶7's limitation on a universal ascent objective. Only the last has explicit human praise. Unmarked passages are not certified acceptable. A reviewer may defend other passages with textual evidence.

Avoid extrapolating this reader's scaling objection to all short explanations, banning pronouns with clear referents, treating every abstract transition as slop, or imposing result-first sentences mechanically. Statistical qualifications can be necessary even when poorly placed. The prompt cannot establish which surrounding chapter context exists, verify experiments, or resolve mathematical truth from stylistic evidence.

## Validation and frozen boundary

Desk-checked the prompt against the cases above: it specifies the reader, four target concern types, exact anchors, a maximum of five concerns, three reasoned acceptable passages, and contextual uncertainty. It explicitly guards the conditional opening and withdrawn ordering objection without memorizing the source's vocabulary. This is design inspection on development evidence, not an independent behavioral test or proof of improved acceptance prediction. The parent owns subsequent frozen-prompt testing and the coherent commit; these two files are the complete specialist deliverable.
