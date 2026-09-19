# Relevance reviewer development

## Scope and evidence

Candidate prompt: [relevance-prompt.md](relevance-prompt.md). Developed from only [development-source.md](../inputs/development-source.md) and [development-labels.md](../inputs/development-labels.md). Audience: an MSc mathematician familiar with the named symplectic concepts, not presumed familiar with statistics or machine learning. The intended change is to detect costly omissions, misplaced detail and unexplained purposes that a correctness review can miss. This is a development hypothesis, not a validated acceptance predictor. No transfer text or historical material was consulted; no manuscript was edited.

## Failure hypotheses and known cases covered

| Hypothesis | Development evidence and qualification | Prompt response |
| --- | --- | --- |
| A reviewer can accept true prose that fails to communicate the mathematical contribution. | Label 1 questions the title and offers a tentative alternative; label 12 rejects the first two closing sentences, not the whole closing paragraph. Label 9 rejects the analytic-example/table reconciliation's placement, not its truth. | Ask what title, opening and closing contribute; distinguish relocation from deletion and relevance from truth. Do not adopt the suggested substitution of capacity for systolic ratio. |
| Explanation effort can be allocated inversely to the reader's needs. | Label 3 objects to early population detail and asks to present the phenomenon first, with uncertainty later. Label 4 finds the scaling explanation immediate for this audience. Label 6 expects correlation measures to need interpretation; its precision objection is tentative. | Compare unnecessary routine explanation with missing unfamiliar explanation; judge placement of details and useful precision, retaining needed limitations. Do not invent significance or prescribe a digit count. |
| Stable mathematical objects become hard to follow through indirect names. | Labels 5 and 8 object to “targets” and “the ratio.” Historical numerical values remain distinct from certified mathematical values. | Require identifiable quantity names without erasing numerical qualifications or banning all pronouns. |
| A reader can lose the purpose when the method changes. | Label 7 retains a request for visible empirical/theoretical separation and figures, but explicitly withdraws the pentagon-first ordering criticism. Label 11 asks for a clearer separate account of selection experiments; its proposed interpretation is a question. | Check the question answered at each transition and whether a figure would help; require textual support for experiment interpretations. No universal subsection rule or forced order of general and special formulas. |
| Reviewers can mistake a conditional criticism or local praise for a blanket verdict. | Label 2 reconsiders the opening as good after a random-maximum section. Label 10 calls failure of universal ascent a nice result and only tentatively suggests moving it to the paragraph opening. Overall “needs revision” does not negate these local judgments. | Evaluate supplied context, qualify missing-context concerns, and return anchored acceptable passages alongside concerns. Result-first structure is an option, not a universal requirement. |

## Likely false alarms

- Calling all sample counts, uncertainty distinctions or computational details irrelevant. They can determine the claim's meaning or serve a methods section; the objection is conditional on purpose and placement.
- Deleting an essential warning because it is neither a theorem nor a geometric explanation. A caveat that prevents a plausible misreading earns space even if a nearby aside does not.
- Explaining all mathematics less and all statistics more. Audience knowledge is a starting assumption; the actual formula, inference and unfamiliarity determine the need.
- Demanding result-first openings, figures, subsections or a fixed observation–proof–experiment sequence regardless of the text's function.
- Flagging every abbreviated quantity name or decimal expansion mechanically, or merging numerical estimates with certified quantities to simplify wording.
- Manufacturing five concerns or three positive judgments. The prompt caps concerns and permits an explicit shortfall when three defensible positives are unavailable.

## Untested scope and limits

Only one annotated development source informed this prompt. The mappings above show intended coverage, not independently measured recovery. Neither the prompt nor its false-alarm controls have been run on another text here. Unknowns include discrimination on already acceptable writing, reviewer disagreement, performance on proof-dominant sections, and context from adjacent sections. The five-concern cap may compress distinct defects; acceptable passages test local judgment, not overall acceptance. The prompt deliberately supplies no memorized source quotations or mandated defect inventory to a fresh worker. External scientific verification and manuscript revision are outside this review's remit.
