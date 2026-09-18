# Competing explanations for missed writing defects

Status: independent design research, 2026-09-18. No model runs or thesis changes.
Purpose: choose useful review interventions within an eight-hour thesis-work budget.
This is a hypothesis register and experiment proposal, not established psychology of LLMs.

## Initial hypothesis register — frozen before historical diagnostics

The list below was written before opening `docs/writing-quality-research` or
`experiments/writing-quality/README.md`. Later observations refine it below without
rewriting this initial register. Order is not a probability ranking.

1. **Missing domain or writing knowledge.** The model cannot recognize a defect even
   when shown the exact passage and asked a neutral, relevant question. Supplying
   definitions or examples should help more than simply allocating another pass.
2. **Knowledge available, selection absent.** The model can distinguish good from bad
   versions when prompted but fails to choose the relevant question during open review.
   A class cue should outperform a generic request for more effort.
3. **Execution capacity.** The relevant test is selected but not completed over all
   instances. Smaller workloads or externally enumerated instances should improve
   coverage even with identical criteria; merely repeating the criteria should not.
4. **Presentation-dependent access.** Source markup, notation, typography, ordering,
   and nearby distractors change what is noticed. Equivalent presentations should
   change detection; knowledge deficits should persist across these changes.
5. **Finding-to-report loss.** Concrete findings exist in an observable intermediate
   artifact but disappear during ranking, deduplication, compression, or final drafting.
   Removing a report cap or preserving a ledger should recover them without rereading.
6. **Conservative reporting criterion.** Ambiguous reader problems are withheld when
   false accusations seem expensive. Changing the requested certainty or loss tradeoff
   should move both true and false positive rates, unlike an unqualified skill gain.
7. **Familiarity masks reader difficulty.** Repeated exposure or broad background makes
   omitted motivation or definitions feel obvious. A fresh reader or an explicit
   reconstruction from text alone should recover omissions; more background may hurt.
8. **Insufficient global context.** An apparently poor local passage is explained later,
   or a locally fluent passage serves the wrong chapter purpose. Full context or a
   chapter dependency map should improve these judgments, whereas isolated snippets
   should help only local coverage and may increase false positives.
9. **Wrong task objective.** “Review” is interpreted as correctness, copyediting, praise,
   or author assistance rather than reader comprehension. Changing the output task to
   reconstructing the argument or identifying reader decisions should change findings.
10. **Reference substitution.** The model silently supplies the argument it expects,
    instead of auditing the argument actually written. Literal evidence extraction,
    counterfactual variants, or asking where each inference is licensed should expose it.
11. **Normalization and authority.** Polished notation, confident claims, or author
    prestige cause suspicious prose to be accepted. Swapping authority cues while holding
    text fixed should change detection if this matters.
12. **Issue competition.** Obvious grammar or mathematical errors consume review effort
    or displace subtler framing defects. Removing easy defects or separating classes
    should improve subtle-defect recall at fixed length and budget.
13. **Correlated reviewer errors.** Several nominal specialists repeatedly inspect the
    same cues or accept the same assumptions. Their union has little added coverage;
    independently varied tasks or representations should yield more marginal findings.
14. **Aggregation failure.** Specialists find problems but a final reviewer discards,
    mismerges, misprioritizes, or contradicts them. Independent source-linked findings
    survive better than prose summaries; better detectors alone do not fix readiness.
15. **Ambiguous target / evaluator disagreement.** A “miss” is a disputed taste or
    readership choice, not failed detection of a stable defect. Explicit audience and
    intended use should improve agreement; changing prompts alone cannot establish truth.
16. **Repair rather than detection is hard.** A reviewer sees a problem but cannot find
    a safe fix and therefore omits it. Requesting diagnosis without a fix should increase
    recall; a separate repair search can then exploit a reliable class-specific detector.
17. **Boundary and scope errors.** The necessary evidence is split across files or a
    retrieval decision excludes it. Supplying paired claims and their support should help
    more than extra reasoning over the same incomplete excerpt.
18. **Stochastic discovery / rare activation.** An effective test is occasionally applied
    with no stable identifiable trigger. Repeated identical reviews should add coverage;
    repeated controlled comparisons are needed before attributing a single gain to a cue.
19. **Premature closure.** A coherent initial account or several easy findings ends the
    search. A required independent alternative account or a second empty-ledger pass
    should expose missed defects more than extra elaboration of the first account.
20. **Hidden dependency among fixes.** The review identifies sentences independently,
    missing that restructuring one passage eliminates several complaints or creates new
    ones. Reviewing the repaired whole should change issue validity and readiness even
    if every local diagnosis was individually defensible.

These are behavioral hypotheses, often compatible rather than exclusive. Descriptions
such as “attention,” “activation,” or “closure” do not establish neural mechanisms.
Neither an assistant's explanation of its miss nor its claim to have checked something
is introspective evidence. Only observable tasks, outputs, manipulations, and resulting
text support the comparisons proposed here.

## What the retained evidence changes

**Observed records, with their boundaries.** The [dataset README](../../experiments/writing-quality/README.md)
reports 37 bounded human observations across seven cases, overlapping revisions and
selective feedback. These are not independent binary grades. The following statements
come from preserved feedback/prediction records rather than fresh experiments:

- The [HKO prediction](../../experiments/writing-quality/local/evidence/docs/review-evidence/calibration/hko-reading-prediction.md)
  names three concerns. The [human feedback](../../experiments/writing-quality/local/evidence/docs/review-evidence/calibration/hko-reading-feedback.md)
  matches one of seven annotations to that prediction and gives the chapter an overall
  PASS. Nearby or generic predictions do not count as detecting a different objection.
  This separates annotation recall from readiness and makes unknown false positives
  important: the other predicted issues were not individually rejected by the human.
- The [DS comparison](../../experiments/writing-quality/local/evidence/docs/review-evidence/calibration/ds-sample-comparison.md)
  records comfortable local prose but an unpredicted central narrative failure in a
  two-page sample, despite the coordinator having project context. This weakens a
  **length-only** explanation; it does not rule out breadth, wrong objective, context
  selection, or an unstated intended narrative.
- The [five-passage pentagon diagnostic](../../experiments/writing-quality/local/evidence/docs/pentagon-chapter-v2/review.md)
  records detection of two rejected features but a miss on implementation-first framing.
  Short input alone did not solve that case. A later explicit criterion and approval of
  revised prose are not an independent successful retest: passage, prompt, and exposure
  changed, and no human judgment establishes the revised chapter's writing quality.
- The [local inventory](../writing-quality-research/local-datasets.md) records a math-only
  preflight that excluded style. Its silence is not evidence that a requested prose
  review failed. Reviewing the actual assignment is necessary before explaining a miss.

**Behavioral refinements.** Preserve the whole hypothesis register. The short-input
misses make pure document-length overload a poor sole account. Criterion mismatch and
selection remain plausible, but the evidence cannot distinguish them from missing
knowledge of the intended audience or narrative. The passing HKO chapter shows that
excellent annotation recall is not necessary for a PASS; the DS sample shows that local
fluency is not sufficient. Neither observation estimates a detector's safe stopping rate.

The [detector research](../writing-quality-research/detectors.md) and
[authoring research](../writing-quality-research/authoring-workflows.md) supply secondary
pointers, not new replication here. Their reported distinctions motivate the designs
below: taxonomy prompts can change salience differently from correctness; rubric scores
can contain signal without calibrated overall judgments; useful critique need not yield
a successful repair. No empirical effect sizes from those papers are imported into
predictions for this thesis.

**Neural/training speculation.** Familiarity-based likelihood, learned reward for agreeable
reviews, representational interference, and training-distribution gaps could each produce
several of the behavioral patterns above. These records identify none of them. We do not
need to choose among them to test whether a cue, representation, ledger, or task split
helps. An agent's retrospective “I focused on X” is another generated report, not a
measurement of the process that caused the original miss.

## Discriminatory comparisons

Each comparison needs the same source version, an explicit audience, a recorded review
scope, and identical available evidence except for the nominated treatment. Match total
review cost where feasible; report wall time, total work, and human adjudication effort
separately. A longer or more expensive treatment is a workflow comparison, not an isolated
causal estimate. Fresh separate contexts avoid teaching later conditions the answer.

| Rival accounts | Minimal manipulable comparison | Divergent observable predictions | Main ambiguity / disconfirming result |
|---|---|---|---|
| Missing knowledge vs missing selection vs localization/search | On the same issue class, compare open review, class-only cue, neutral exact location plus class, then a concrete worked example from a different passage. Use nearby acceptable variants too. | Cue rescue supports selection; location-only rescue points toward search burden; example-only rescue supports missing usable task knowledge. Persistent exact-location failure undermines a pure search explanation. | Cues can teach the criterion or lower the reporting threshold. An exact location reveals suspicion. Include equally suspicious clean controls; no rescue does not prove knowledge absent. |
| Recognition vs reporting criterion | Request either firm consequential defects or all plausible reader obstacles, preserving the same issue classes and output allowance. Independently ask a neutral comparison of original and a verified repaired variant, with order swapped. | Threshold change raises both correct reports and harmful/unsupported reports; better discrimination improves variant comparisons without merely flagging more text. | Pairwise recognition does not establish independent discovery, and repairs can leak obvious cues. Confidence claims alone do not identify threshold. |
| Execution burden vs issue competition vs output loss | Cross short/long input with narrow/broad review scope; keep a short target identical inside longer versions. Record an explicit per-section finding ledger before a final summary. | Execution/competition predicts fewer target findings in the ledger as scope grows; reporting loss predicts stable ledger findings but poorer survival in summary. Easy distractor defects should hurt subtle recall more under competition. | Length also adds context and output demand. Use irrelevant-but-plausible padding and useful context separately; a breadth effect alone does not identify capacity. The ledger itself changes the task. |
| Missing context vs familiarity-induced completion | Same target with no context, necessary factual context, or the same factual context plus an author-provided interpretation. Ask for passage-supported reconstruction. | Necessary context helping favors evidence insufficiency; interpretation worsening detection while facts are fixed favors adoption of the author's account. | More context also costs capacity. No difference may reflect a weak manipulation. Accurate reconstruction with an omission still unreported points toward criterion/task choice. |
| Reader-order failure vs inherent incomprehensibility | Compare full chapter review with sequential reading that records what is defined before each section; hold final available facts fixed. | Sequential review may expose definitions or purpose supplied only later; full-context review may excuse them. An actually absent explanation should fail in both. | Later explanations may be deliberate and acceptable. Task requires audience-sensitive judgment; sequence is an intervention, not a literal simulation of a human. |
| Surface fluency/authority vs content-based judgment | Keep proposition and discourse structure fixed while varying source/presentation or asserted authorship, separately; use source text and verified rendered views. | Authority-sensitive judgments change with asserted author; presentation-sensitive detection changes with typography/markup. Stable judgments weaken these explanations locally. | Rewriting surface fluency can change the defect. PDF extraction errors are tool failures, not prose defects; inspect relevant displays. Do not bundle author cue and formatting. |
| Repair difficulty suppresses diagnosis vs detection inability | Compare diagnosis-only review with diagnosis-plus-repair under the same input budget; then supply a fixed verified diagnosis to an independent editor. | Extra diagnoses in the first condition suggest repair/report coupling. Good detection with poor supplied-diagnosis repair isolates a downstream problem. | Diagnosis-only may permit lower-quality complaints. More reports count only if their validity survives verification. |
| Specialist benefit vs simple repeated sampling | Equal-cost union of broad repeats versus class specialists, with identical coverage and finding format; compare marginal verified findings as reviewers accumulate. | Specialists helping their assigned classes beyond repeats supports targeted task selection. Similar unions favor additional sampling over role labels. Low marginal yield in both indicates correlated blindspots or saturation. | Specialists can quietly receive better evidence/examples. Shared misses do not prove shared internal mechanism. Independence of sessions does not imply independent errors. |
| Aggregation loss vs weak detectors | Give identical frozen finding ledgers to two aggregation tasks: unconstrained evidence-preserving union and prioritized short report. Match issue identities afterward. | Findings present before merging but absent afterward establish observable merge/report loss. If present but ranked low, the failure is salience. If all survive and readiness remains wrong, investigate acceptance calibration. | Source claims still need adjudication. A lossless union may overwhelm the human; evaluate useful prioritization as well as retention. |
| Ambiguous readership vs unstable ability | Hold text fixed and state two genuinely different reader prerequisites/purposes, then repeat under a fixed purpose. | Systematic audience-dependent differences may be appropriate, while instability under one audience suggests unreliable judgments. | Agreement is not truth. Do not manufacture a preferred audience just to relabel a historical miss as success. |

## Small experiments worth doing within the thesis budget

Do not execute the full matrix before helping the thesis. The first experiments should
answer routing decisions, not estimate a complete causal model.

1. **Recognition/search diagnostic, first priority.** Choose two or three consequential
   classes from development evidence: missing purpose before machinery, missing inference,
   and narrative order. For each, use a natural failed passage and a nearby acceptable
   or carefully repaired contrast with its context. Compare open review, class cue, and
   exact-locus diagnosis in separate contexts. Start with one replicate per condition as
   a screening exercise; repeat only a result that changes the workflow decision. Require
   exact source anchors and consequences, not matches to issue vocabulary. This is too
   small for reliability certification. It can show whether specialist search is worth
   trying or whether criterion/examples need development first.
2. **Cost-matched coverage, conditional on usable recognition.** Compare a union of
   generalist repeats with specialists on one unchanged chapter. Assign specialists
   complementary operations (reader reconstruction, argument warrants, purpose/order),
   not merely differently named identities. Preserve every anchored finding and measure
   incremental verified issues, major known misses, contradictions, and review cost.
   Compare both local and chapter-scale defects. If specialization adds no useful coverage,
   stop adding roles; try the missing-context or criterion hypothesis instead.
3. **Aggregation audit, whenever multiple reports exist.** No further review runs are
   needed to identify omissions between frozen individual reports and their final union.
   Track each finding through retained/merged/rejected states with reasons. Evaluate
   top-priority omissions separately from raw retention. This inexpensive test isolates
   an observable stage and prevents detector improvements from being lost downstream.
4. **Repair trial only after a useful detector exists.** Supply a verified issue to an
   editor; retain original/no-change, local edit, and structural rewrite when appropriate.
   Recheck the target class independently and check mathematical preservation and newly
   introduced prose problems. A class-specific detector can guide candidate search even
   if it cannot certify whole-chapter readiness. Improvement requires actual successful
   repair, not an explanation that repeats the critic.

A proposed eight-hour allocation is at most 60–90 minutes for screening and choosing a
workflow, roughly five hours for review/repair of the most consequential thesis defects,
and the remainder for integration, mathematics regressions, rendered reading, and a
bounded human-facing decision packet. These are planning limits, not measured runtime
predictions. Stop mechanistic exploration when two surviving explanations recommend the
same useful action. Stop a treatment early if it consumes the repair budget without
adding verified consequential findings. Failure of the screen means no validated shortcut;
it does not justify either declaring the thesis ready or spending all eight hours on
benchmark construction.

## What can and cannot be inferred

**Keep five outcomes separate.** (1) Does the complaint describe a real issue? (2) Is it
important for this reader? (3) What known issues were missed? (4) Did the edit improve the
text without mathematical damage? (5) Is the whole artifact ready? An accurate class
specialist can support (1) and a search over (4), while offering little evidence for (5).
The union of specialists is a candidate review workflow; it does not logically cover
unassigned classes or interactions between individually sound passages.

**Shared causes and non-identifiability.** A prompt cue simultaneously changes task
interpretation, retrieved examples, reporting criterion, and output allocation. Shortening
input changes context, number of opportunities, and reading order. Repeated reviews add
computation and stochastic diversity. Therefore success under those manipulations identifies
a useful treatment before it identifies a unique explanation. Report which rivals survive.
An explicit intermediate ledger establishes a stage boundary only for the workflow that
produced it; it cannot reveal unreported thoughts in the original one-shot review.

**Null results.** No treatment difference can mean either shared competence, shared
failure, inadequate manipulation, too few cases, or an insensitive outcome. Inspect
absolute errors and controls. If both conditions catch every known issue, the cases may
be too easy; if neither does, test a located concrete distinction before increasing
review breadth. If added reviewers find nothing, distinguish genuine saturation from
correlated omission using known hidden objections. A finding absent from selective human
feedback remains unadjudicated, not automatically a false positive.

**Leakage and measurement.** Historical objections are development evidence for this
report. Reusing them can test task responsiveness but cannot establish unseen-case
reliability. The local dataset has no independent lineage split; disclose transfer within
the same thesis project. Freeze prompts and the scoring interpretation before comparisons;
keep labels away from evaluators. Do not reward a generic nearby critique as a hit. Retain
withdrawn objections and chapter-level PASS labels with their original scope. If new
human adjudication is necessary, request a compact set of concrete disputed consequences,
not exhaustive labeling of every model complaint.

**Practical recommendation.** First locate the bottleneck behaviorally: recognition,
search, reporting, prioritization, repair, or final acceptance. Use a specialist when it
adds verified coverage at acceptable cost, preserve its evidence through aggregation,
and use the detector to compare repairs only within its demonstrated scope. Keep a
separate final reading of the integrated argument: accumulating locally correct reviews
cannot by itself establish the reader-facing whole.
