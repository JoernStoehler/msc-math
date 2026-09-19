# Human responses: alternate-draft microbatch 3

Source: inputs/alternate-ds-transfer.md. Predictions frozen in alternate-freeze.json; selection rationale in microbatch-3-selection.md. Human sees targeted spans with local context, without a requested whole-draft verdict. These are related-content transfer cases, not independent topic generalization.

## M: one-off code label

Exact response:

> remove, i see no purpose of providing a name that is never used (?)

Confirms the reproduction-detail specialist's exact concern and proposed removal/relocation of “called factorial-both.” The baseline had also flagged the larger paragraph's unexplained methods, but did not isolate removal of the one-off name. This is one fresh autonomously located, human-confirmed concern for a prompt developed from earlier code-detail and seed-placement evidence. It is not just a supplied-candidate classification. The human question explicitly stated the label was unused elsewhere in the subsection, a true contextual fact.

## N: retrospective versus prospective formula

Exact response:

> the first sentence btw is slop? "later" -- later than what / why is timing relevant? "descriptor" -- is this clear from context? and yeah the second sentence rules out sth that i don't quite expect a reader to guess? if you want to say "We ran the empirical study first and guess via function fitting the final formula we had to prove." then just write that?

Confirms the qualifications specialist's concern about ruling out an unexpected prospective-prediction alternative. Also raises problems in the preceding positive clause: unclear relevance/referent of “later,” and possible opacity of “descriptor.” The specialist proposed retaining precisely that clause as its minimal repair. Therefore detection succeeds at the marked clause while repair adequacy fails or remains incomplete. The descriptor concern is explicitly context-dependent. The human's illustrative direct chronology is not evidence that this was the actual scientific discovery sequence or authorization to assert it.

This is a new exact passage from an alternate draft, but a related rhetorical pattern to development labels and earlier C. It supports narrow transfer within that pattern, not independent-domain generalization.

## O: historical numerical provenance

Exact response:

> oh - the solution is to fix reproducibility in the repo! worst case we just rerun expensive calculations for the sake of restoring metdata? best case we run a throwaay migration script that fakes having run the new pipeline using old data?

Subsequent explicit coordination instruction:

> (wrt these kinds of hard-to-describe repo states: you can message main with issue reports about writeup needing a better research base to write up -- i.e. that there's high-value minor maintainance efforts that'd really make writing easier)

Interpretation: this is an upstream evidence/maintenance direction, not KEEP or a simple wording rejection. The human wants to resolve the repository condition making the caveat necessary. Both narrow reviewers explicitly retained the sentence under the supplied factual assumptions; their fixed-text review did not identify upstream repair as the useful next action. Do not score O as a confirmed false endorsement of a false sentence or as permission to silently remove a still-valid limitation.

The parent clarified that importing old results into a new schema can preserve their actual provenance but cannot create evidence that a new pipeline executed or certified them. It relayed the request to main through the provenance-preserving Herdr helper and started a bounded read-only issue report: identify artifacts and missing guarantees, whether saved artifacts permit revalidation, and the smallest recomputation genuinely needed. No capacities were rerun and no provenance records were fabricated.

This reveals an action-space limitation of fixed-text review: the best response to an awkward true qualification can be repairing the underlying research state. The separate issue report belongs to main's research coordination; it is not a manuscript edit or an extension of the trial into expensive computation.

## Effort

M response is short; allow 0.25–0.5 minute for reading and answering. N adds roughly 0.5–1.5 minutes. O adds approximately 0.5–1.5 minutes. These are uninstrumented budget allowances.
