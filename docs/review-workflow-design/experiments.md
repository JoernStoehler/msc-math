# Small experiments that change the next decision

Status: proposed, not executed. No detector has been validated by this design. Read the [decision map](decision-map.md) before selecting experiments. The intended end use is an eight-hour thesis session, not an open-ended benchmark campaign.

## Common measurement contract

Keep five distinct artifacts: input, raw reviewer observations, reviewer report, merged report, and any later repaired text. None is a substitute for Jörn's final reading judgment. A reviewer may fail to find an issue, find it but omit it from a short report, or report it correctly only for the merger to discard it. Observable intermediate records help distinguish those failures; a reasoning trace does not prove which internal process occurred.

Each reviewer returns an issue ledger with stable IDs, exact quoted anchors or paragraph IDs, alleged reader difficulty, supporting context, severity estimate, and uncertainty. It separately records an overall acceptance prediction and important missing context. Do not require a minimum issue count. Allow no issue, uncertain, and insufficient context. A short structured ledger is a reporting intervention and may itself affect detection; the baseline must receive the same output contract unless output format is the manipulated variable.

Measure:

- Recovery of applicable, active recorded human objections, retaining dimension and scope. Report numerator and denominator rather than a pooled accuracy score.
- Contradictions of known human judgments: especially falsely predicting rejection of accepted HKO solely because it contains imperfections, or falsely predicting acceptance of rejected narrative.
- Candidate unsupported or harmful criticisms, assessed separately. Unmentioned spans have unknown status, so historical silence cannot estimate false-positive rate.
- Marginal useful discoveries and redundant findings per reviewer; disagreements remain visible until adjudicated.
- Raw-to-report and report-to-merge survival, changes in severity, and unsupported merger additions.
- Actual elapsed time, requests where available, input/output/reasoning token accounting where available, cache use, shadow cost, and root/adjudication time. Missing accounting is unknown, not zero.

Match equivalent findings semantically with visible source anchors; do not reward copying historical phrases. Have an adjudicator inspect disagreements with source evidence and preserve uncertain matches. AI adjudication is provisional, particularly for novel defects and false alarms. A future short human feedback opportunity should concentrate on those disagreements rather than reread obvious matches.

## Inputs and leakage

Use the [dataset owner](../../experiments/writing-quality/README.md), its manifest, and input-only export. No new data files or exports are created by this design.

- c001 supplies accepted scientific exposition with seven localized objections: useful against an indiscriminate-rejection detector.
- c002 supplies a natural sample whose comfortable prose did not rescue its rejected narrative structure. Preserve the whole two-page input for narrative judgments.
- c004 supplies rejected preliminaries and borderline accepted Chapter 4. Extract only after resolving exact scope and needed prerequisites from labels and page maps; do not transfer whole-thesis labels to arbitrary excerpts.
- c003–c005 are overlapping revisions, not independent replications. c006 is an incomplete quoted record, not a whole passage suitable for acceptance scoring. c007 has no established human writing verdict; use it only for prospective disagreement discovery.
- External SCARECROW/SNaC can probe natural local-error detection. StoryFeedback can probe synthetic corruption detection and critique quality. ARIES can probe interpretation/repair of scientific feedback, not a scientific-writing PASS classifier. Preserve the sources' different label meanings.

All local cases share project lineage. Fresh agents with input-only packets reduce direct answer leakage but do not create independent human evidence or erase model pretraining. Development on one chapter and evaluation on another is a disclosed topic-transfer test. Use a small reserved source group for checking a promising intervention; do not repeatedly tune against it and continue calling it held out.

For perturbation controls, start with an accepted span, introduce one predeclared issue, and preserve the original as a paired control. Include a meaning-preserving sham edit. These controls test sensitivity and overreaction, not the complete quality of the original or the realism of all natural failures. Verify that corruption did not also introduce mathematical error. Do not use synthetic successes as evidence that natural near-threshold writing is solved.

## Portfolio A: recognition first, then coverage, scope, and reporting

**Question:** Does splitting attention help because of text extent, review breadth, output compression, or something else?

Start with open review, class cue, and exact-locus diagnosis on two or three natural issue classes, with accepted or carefully verified contrasting controls. The diagnostic ladder below defines the interpretation. Historical short-input misses make a length-only account insufficient; do not spend the first cycle assuming it.

Where search burden remains plausible, run a small crossed comparison on the same target issue-bearing passage: short context versus sufficient broader context; one named aspect versus broad review. Keep target passage, instructions unrelated to the manipulation, model, and output allowance fixed. Specify a broad context that retains relevant prerequisites; a clipped condition must be explicitly marked as such rather than treated as equivalent evidence.

Add a reporting probe only if needed: same broad review, compact ledger versus brief free-form review. Compare localized known-objection recovery and acceptance prediction separately. A complete ledger versus a short review may reveal output truncation, but could also change what the model notices; it cannot establish latent detection.

Predictions and actions:

| Observation | Next action | Remaining alternative |
|---|---|---|
| Narrow review helps especially with longer context | Try aspect specialists with shared context | Prompt specificity, not attention capacity, may cause the gain |
| Short context helps local defects but hurts narrative judgments | Route by scope; keep a whole-section reviewer | Cropping changed the task, not only its size |
| Broader context improves both | Invest in prerequisite/context preparation | Short input was underspecified |
| Ledger chiefly improves recovery | Preserve evidence ledgers and test merger survival | Format may be eliciting rather than merely reporting detection |
| No consistent benefit | Do not standardize decomposition from its plausibility | Too few cases or insensitive labels may conceal an effect |

Initial recognition screen: two target types × three conditions = six jobs, bundling paired controls without revealing which is defective and counterbalancing order where relevant. Conditional scope screen: two target types, four conditions, one fresh run each = eight jobs. This is a screening experiment, not a reliable rate estimate. Repeat only a decision-relevant contrast on another source group. Units sharing a passage are paired observations, not independent sample-size multipliers.

The recognition diagnostic ladder on the same item is: open review → name the issue class → identify the exact locus → supply relevant prerequisite/example → ask a concrete forced choice between interpretations. Use separate fresh runs, not escalating hints within one conversation, when comparing conditions. Improvement with a hint measures cue-assisted recognition, not spontaneous detection. If precise localization and context still fail, pure overload becomes less persuasive. If a cue increases both hits and unsupported accusations on paired controls, a changed reporting threshold is an alternative to improved understanding.

For a narrative miss, prefer a representation probe over more category reminders: show only the prefix and freeze the reader's expected question/meaning before revealing the continuation; separately reconstruct the paragraph's argument without evaluating its style. These outputs can expose mismatched reader state or silently supplied inferences. Agreement with an agent's own reconstruction is not human validation.

## Portfolio B: does accumulating reviewers accumulate capability?

**Question:** Does specialist diversity buy more than additional attempts by generalists?

Compare three review portfolios on the same inputs:

1. One generalist with a larger usage allowance.
2. Three independent generalists with ordinary allowances.
3. Three complementary reviewers: reader prerequisites/continuity, paragraph purpose/order, and precision/claim strength.

This is an initial architecture contrast, not a claim that these three specialties are optimal. Reviewers receive equal access to essential context. They do not see each other's findings. Record each raw report before aggregation.

Run two distinct comparisons. **Fixed cost:** use a measured common total usage envelope; an unspent cap is not matched actual cost. Report quality versus realized cost when exact matching is impossible. **Fixed latency:** allow concurrent portfolios the same wall-clock deadline and report their different costs. Include spawn overhead, retrieval, aggregation, and coordinator work. A Codex agent job is not necessarily one API request; tools and continuations can create extra rounds. Hidden reasoning is not free and its trace is not assumed available.

First measure raw union and complementarity, then apply the same merger to each portfolio. If specialists add no unique supported discoveries, prefer cheaper or simpler generalists. If gains disappear after merging, fix aggregation before adding reviewers. If union mostly increases unadjudicated allegations, the bottleneck is precision/adjudication, not coverage. If only one specialist contributes, retain that specialist provisionally and test transfer; do not retain the whole ensemble by inertia.

Initial scale: two input types × seven reviewer jobs = fourteen jobs plus common merging. Reuse comparable Portfolio A baseline outputs only if prompts, context, model controls and budgets actually match; otherwise do not pretend they do.

## Portfolio C: evidence-preserving aggregation

**Question:** Does the merger preserve useful minority findings without amplifying speculation?

Use frozen raw reports, preferably produced by A/B. Compare mechanical concatenation/deduplication, ordinary summary, and evidence-preserving issue-ID merge. For a controlled probe, insert clearly labeled synthetic report-level conflicts or omissions into a separate copy, never into the observational record.

Check survival of known supported findings, unsupported certainty inflation, minority-loss, duplicate inflation, and whether accepted text is rejected merely due to issue count. A claim repeated by several same-model reviewers is not independent corroboration. Require the merger to retain unresolved substantive disagreement with its sources. Do not ask it to decide thesis PASS by majority vote.

If ordinary summary loses supported findings, prefer the ledger union plus a separate prioritized view. If preservation makes the output too burdensome, separate a short action list from the retained evidence rather than deleting dissent. This experiment can resolve an inexpensive but consequential workflow choice without new text generation.

## Time and stopping decisions

No latency or cost measurements were made here. Planning allowances, conditional on available concurrency and ordinary inference latency: A about 15–30 minutes; B about 20–40 minutes; C about 10–20 minutes including inspection. These are scheduling guesses, not observed performance. The root's adjudication can dominate all three.

For an eight-hour thesis run, reserve at most an initial 60–90 minutes for this first review-design cycle, with a 30-minute decision checkpoint. Start A; run C as soon as outputs exist; run B only if diversity versus repetition is still a consequential uncertainty. Record a small measured pilot's actual usage before budgeting the rest. A provisional $25–$75 experiment allowance is a spending limit, not a cost estimate, and must remain subordinate to the session's actual budget and accounting.

Proceed to a coupled detect→repair→review test once the detector supplies useful localized guidance on natural material without obvious acceptance inversion. This is a provisional deployment decision, not detector certification. If it only succeeds on synthetic local errors, restrict its role accordingly and keep searching for narrative review. If the first cycle yields no useful discrimination, change representation or context according to the decision map rather than merely increasing instruction strength. Preserve time for writing; also preserve the option to conclude that autonomous whole-thesis PASS is not supported by the available process.
