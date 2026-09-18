# Adversarial review: experiments and intervention search

2026-09-18. Independent review of the proposed next investigations; no experiment or manuscript change. Sources: [current position](current-position.md), the recognition and repair reports and frozen prompts/criteria, [experiment design](experiments.md), [architecture menu](architectures.md), [decision map](decision-map.md), [causal alternatives](causal-alternatives.md), [content audit](content-cruxes.md), and the dataset README. Harness-engineering guidance informed the distinction between outcome and intermediate metric. Cost ranges below are planning judgments, not measurements.

## Recommendation

First obtain an absolute human judgment on an existing DS repair through the review desk, if that judgment is not already available. Then compare one promising critique-led workflow with a direct authoring alternative on current natural material. Do not make another detector screen the automatic next step. A detector is valuable only if its contribution improves the eventual text enough to justify its extra work; the current search still gives it too much presumptive centrality.

The existing evidence supports trying informed editing. It does not establish that autonomous critique can discover the right direction, that supplied-direction editing reaches acceptance, or that either scales to a thesis. These are different uncertainties. The smallest next action should address the earliest uncertainty that would change the next investment.

## Four consequential objections

### 1. Withholding the verdict does not withhold the answer

The proposed purpose-informed DS review could receive exactly the missing discovery → patterns → interpretation → conjectures route that human feedback supplied. The [repair criteria](../../experiments/writing-quality/runs/20260918-repair/criteria.md) contain this route explicitly. A reviewer can then find a mismatch between source and specification without independently detecting the reader's difficulty. Calling its input “context” and hiding PASS/FAIL would not change that.

This can still be a useful intervention. A maintained editorial brief might be enough to make production work. The consequential question is whether this brief is available for future units at affordable cost, not whether supplying it is philosophically pure. If Jörn must identify every missing explanation and arrange every narrative first, the process is assisted execution and its human effort must be budgeted accordingly.

**Change the proposed test:** distinguish general reader/purpose context recoverable before passage-specific feedback from local editorial solutions derived from that feedback. Record the provenance of each supplied assertion. Use the first kind in a transfer test; call the second a supplied-direction repair test. Evaluate resulting prose against human acceptance, not just against compliance with the supplied route. A successful different structure must be allowed to pass.

**What would change my assessment:** a critique supplied only independently recoverable purpose identifies a new consequential problem on a different natural text, and its resulting edit is accepted without new passage-specific direction. Conversely, repeated “success” confined to briefs containing the missing inference should shift investment toward efficient human briefing, not autonomous detector claims.

### 2. Detector-first search may omit the better intervention

The menu is diverse at the level of review operations, but the favored next comparison still asks which critique representation to use. No reported comparison establishes that a critique stage helps more than asking an author directly to produce an acceptable explanation from sound content and purpose. The repair trial's “reconstruction” still gave its author the original prose and precise objections; convergence with narrow editing does not test authoring from a content representation without the old prose.

A stronger rival is: extract claims, warrants, mandatory evidence, reader prerequisites and section purpose; verify that packet; then draft without the rejected wording or paragraph order. A second rival is to preserve already accepted prose and concentrate changes on missing content and rejected sections. Neither requires a universal acceptance classifier. Both have risks: content extraction can omit material, and a fresh author can repeat the same defaults. They deserve one bounded comparison before more reviewer machinery is built.

Use the same factual packet and legitimate purpose for both branches; give original prose to the critique/edit branch and withhold its wording/order from the fresh-author branch. This intentionally changes a whole workflow, so interpret the result as a deployment comparison, not a causal proof of anchoring. Independently check claim and caveat preservation. Keep the unchanged baseline as an actual candidate. Relative preference alone cannot show any candidate passes.

**What would change my assessment:** critique-led editing reliably produces the accepted version while the alternative fails or creates greater verification burden. That would justify investing in critique. If direct authoring succeeds with less total work, stop making detector development a prerequisite for production.

### 3. The next result could be another cheap fact with no decision consequence

Purpose critique versus paragraph questions versus prefix expectations can yield interesting different reports even when every branch ultimately triggers the same rewrite. Successful prefix prediction may merely reproduce one conventional order; an unconventional but sound continuation need not be defective. Question reconstruction may silently supply missing coherence. Neither result is intrinsically evidence of improved prose.

Before running either, name the decision it can change. If both produce the same candidate repair, choose the lower-cost operation unless another documented benefit matters. Do not pursue their psychological explanation. If neither produces an accepted repair, additional reports do not justify scaling them. Measure the total cycle through preparation, content checking, editing, rendered review and human judgment; the recorded 108-second recognition window establishes cheap review generation, not cheap successful revision.

There is an inexpensive missing observation already waiting: the existing DS reconstruction's absolute acceptability. The desk's prepared one-minute HKO option is convenient but HKO already passed as a chapter. A favorable HKO local preference would give much weaker evidence about crossing a narrative acceptance threshold than a DS judgment. This is a prioritization suggestion to the desk, not authority to interrupt Jörn or replace its interaction.

### 4. Short-cycle success cannot itself supply a production-readiness threshold

The current-position note correctly calls its threshold underspecified. Adding a fixed number of successful short samples would not fix the main problem: their coverage of the actual remaining work is unknown. Historical fragments, particularly selected known defects, need not resemble current chapters that combine new proofs, numerical provenance, and revised cross-references.

A useful readiness demonstration is one current assembled chapter with at least one consequential transition to another chapter, including its actual evidence and references. Record which edits were made, what remained unresolved, the time spent on verification and revision, and whether human acceptance required a new editorial direction. Use that result with an inventory of the remaining heterogeneous obligations, not pages-per-minute multiplication alone. Integration, scientific source changes, and human-review queueing are serial constraints even when authors run concurrently.

One successful demonstration licenses a scoped production strategy; it does not certify the whole PDF. One failure should identify an unresolved class, not prove all autonomy impossible. Resume decisions should say explicitly what unsupported transfer is being accepted and which failure would stop the run.

## A small sequence with rival predictions and action rules

Do not launch all rows together. These are proposals, not authorization. Route any human request through the existing desk and reuse feedback already collected.

| Action | Rival observable outcomes | Decision consequence | Estimated incremental effort and latency |
|---|---|---|---|
| **A. Judge the existing DS reconstruction in its complete two-page scope.** Ask absolute acceptability for that scope first; distinguish missing scientific content from prose failure. Use baseline comparison only if useful afterward. | Supplied direction is sufficient: the draft passes. Execution remains inadequate: it still fails for prose. Or content constraints make the desired narrative impossible without more evidence. | Pass: prioritize independent direction/transfer. Prose failure: improve authoring before detector architecture. Content failure: resolve or truthfully delimit that obligation before using the sample as a writing test. | No new authoring. Desk preparation roughly 5–10 agent minutes; Jörn roughly 3–6 minutes for the text and judgment, longer if he chooses detailed feedback. Calendar latency depends on availability. No need for exhaustive labels. |
| **B. One paired end-to-end transfer on current natural material**, gated by A. Use legitimate purpose, a checked content packet, original baseline, critique→edit, and fresh authoring. Choose a different substantive section and include necessary preceding context. | Critique adds useful direction; fresh authoring bypasses inherited structure; both fail; or baseline already passes and changes are unnecessary. | Deploy the successful lower-burden route provisionally. If only supplied historical direction works, estimate/ask about human briefing rather than declaring autonomy. If baseline passes, preserve it. If all fail, change the authoring intervention or reconsider resumption; do not automatically add reviewers. | Roughly 20–40 agent minutes including packet preparation, checking and rendering; about 5–10 human minutes for a modest packet. Reduce packet size or stage candidates if this exceeds the desk's available attention. No model-cost estimate inferred from prior review-only costs. |
| **C. Current chapter plus boundary demonstration**, only after a promising route. Use the actual assembled source and a representative content integration. | Local success transfers; seams and new evidence cause failure; or acceptable output takes too many revision cycles. | Successful, affordable cycle: use measured throughput plus remaining-obligation inventory for a scoped resumption proposal. Seam failure: add explicit integration ownership. Excessive revision/human direction: revise the completion forecast before spending the production budget. | Roughly 45–90 agent minutes as a planning allowance, strongly dependent on source readiness; 10–20 human minutes for a selected chapter and boundary, with full-chapter reading potentially longer. Measure actual cost instead of treating this allowance as a forecast. |

These human-effort estimates are not imposed requirements. Jörn may prefer a faster verdict, read more carefully, or decline another pilot. The review desk should surface that choice early once further preparation depends on whether he wants it. A proxy agent verdict cannot be relabeled as his acceptance.

## Omitted context and other necessary work

- **Current artifact ownership comes before sample selection.** The content audit reports an assembled source still mixing frozen and replacement chapters. Identify the candidate source and evidence state before commissioning prose that may immediately become obsolete. This is a narrow dependency check, not permission to restart all technical work.
- **Scientific purpose can depend on unresolved content.** DS's intended discovery narrative cannot be fulfilled by promising conjectures or rigorous explanations absent from the available results. Resolve the distinction between a motivating research program and completed achievements in the content packet. Better prose cannot close this evidence gap.
- **Preservation is an intervention.** Accepted HKO is positive evidence for leaving much of a chapter alone. A criterion that induces gratuitous restructuring there is costly even if it also repairs DS. Avoid making “more explanatory” or “matches the outline” an unconditional objective.
- **The proposal documents have different ages.** Older causal-alternative guidance says repair only after a useful detector; current-position guidance correctly permits coupled development. Likewise old first-cycle budget allocations are not the current authorization state. Root should identify the active proposal explicitly when delegating, so a worker does not revive superseded gates or time limits.
- **Preparation has a cost even before the production clock starts.** Parallel reviews can be inexpensive while the root's synthesis and Jörn's context switching accumulate. Once surviving hypotheses imply the same next intervention, stop investigating the distinction. The opportunity cost is delivery delay and human attention, not only the nominal eight-hour allocation.

The strongest reason to continue workflow search is that informed edits are cheap enough to try. The strongest reason to narrow that search now is that human acceptance of the resulting text, and the need for further human editorial direction, remain unmeasured. Those observations should choose the next branch.
