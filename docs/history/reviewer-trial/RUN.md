# Reviewer trial run record

Start: 2026-09-18 12:19 UTC / 14:19 Europe/Berlin. Deadline: 13:19 UTC / 15:19 Europe/Berlin.
Durable agent: codex:thread:01a0b475-3ecd-7021-be3f-34bce8004b81; Herdr w26:p7.
Human active-effort ceiling: 30 minutes; recorded estimates below are not instrumented durations.

## Separation

The original DS text and its qualified human annotations are development material. The supplied-feedback revision is a transfer text with no prior human verdict, but shares topic, authoring lineage and development feedback. It is not independent topic generalization. Fresh evaluation workers get only a frozen prompt and source, without labels or development conversation. Predictions must be saved before human requests. No manuscript edits.

## Human effort

Initial state at 12:19 UTC: no requests yet. Subsequent requests and responses are recorded below.

## Initial execution

Three prompts frozen and executed by fresh workers with no conversation inheritance: generic baseline, precision/reader fit, mathematical relevance. Development workers only saw original DS + qualified labels. DS evaluation source is the supplied-feedback revision. Two further fresh workers run baseline and relevance on an HKO chapter with local historical labels withheld.

12:21–12:22 UTC: microbatch 1 requested, estimated 1–2 minutes active human time; response pending. Four purposive passages, two predicted keep/two problem. Manual parent predictions are separate from worker evaluations.

12:25 UTC: Round 1 DS outputs: baseline and precision each report normalization-wording and control-definition concerns; relevance reports opposing-path description and control-definition concerns. Relevance endorses the opening that the other two flag, a useful disagreement. Topic-transfer predictions frozen before historical labels are read by a separate scorer. Added an explicit passage-classification comparison; candidate selection draws from known agent concerns, endorsements and ordinary passages, without human transfer labels. This comparison is exploratory adaptation after inspecting agent outputs, not an untouched prompt comparison.

12:26 UTC: Historical comparison returned: among seven selected local HKO objections, baseline partially matches one, misses six; relevance directly matches one (borderline partial/direct), misses five, and explicitly endorses one human-rejected rhetorical feature. These are selected-label counts, not accuracy. Unadjudicated additional concerns are not false positives. HKO is now available for further development, and cannot be reused as fresh evaluation for adapted prompts.

## Round 2 separation and execution

After HKO scoring, its labels became development evidence for the narrow qualifications specialist. Its developer read original DS development source/labels and the HKO comparison/source, not revised DS or pentagon-transfer. The fresh qualifications evaluator then read only its frozen prompt and one evaluation text per run. Revised DS is fresh to those workers but previously inspected by parent and other evaluators; pentagon-transfer had no known human writing verdict. Baseline independently reviewed pentagon-transfer. Both found no concerns there within their respective scope. Absence of flags is not human endorsement.

Reader-question prompt was written after seeing HKO misses. It and baseline were then run independently on historical DS c002 without detailed c002 labels. Parent knew only the broad dataset summary (comfortable prose, rejected organization), disclosed as partial historical exposure. Historical scorer sees predictions only after round-2-freeze.json.

All evaluator/developer workers were spawned with no inherited conversation and explicit file restrictions; model settings were inherited without overrides. These are single runs, not stability estimates. General session harness still applies; no claim of contamination-free evaluation.

Second microbatch F/J/K/L requested after classification predictions were frozen, estimated 80 seconds plus response mechanics. No human reply yet. Classification was candidate-conditioned, not autonomous detection. F was not flagged by open reviews; J was shared concern; K was relevance-only concern and classifier KEEP; L was baseline-endorsed and classifier KEEP.

The qualifications-DS run happened after microbatch 1 was requested but before a human answer was received. Its concern overlaps microbatch C. Any later agreement on C is a secondary overlap, not a prediction frozen before that request. The other qualifications concern has not yet been requested from the human.

12:30 UTC: Launching final automatic historical check: prerequisite reconstruction versus baseline on c004 section 2. Audience overridden consistently to MSc mathematics without prior symplectic/contact theory. Parent knows broad historical rejection, not detailed labels. Predictions will freeze before scoring. Other held-back dataset material remains unused.

2026-09-18T12:35:36.833089+00:00: Microbatch 1 and F responses received. A useful, B contextually problematic/unclear, C clear problem, D insufficient context; F confirms seed-placement concern. Full paragraph context check requested for B using already frozen open-review endorsements. Narrow reproduction-detail prompt developed from F and historical code-detail objections. Existing qualification prompt retained unchanged. Next source alternate-ds-transfer.md chosen from different authoring route before parent reads contents, shares scientific source lineage. Human judgments on initial microtests are now development data and must not be rescored as new tests of adapted prompts.

## Human exposure and revised effort budget

Jörn received short agent progress summaries while microtests were pending. In particular, the qualification review's significance-test concern was mentioned before his C response. Microbatch 3 was introduced as two flagged details and one retained qualification, without assigning those labels explicitly to IDs. These are not blinded human adjudications. Predictions were frozen before requests except the separately disclosed secondary qualifications/C overlap. Prior human familiarity with the same broad defects also remains relevant.

A–D/F/J/K/L responses received. F and C confirm concerns, A and L supply local positive judgments, B remains contextual pending full paragraph, D unjudgeable, J diagnosis/remedy match partial, K substantive usefulness with local wording concerns. Provisional active effort allowance for received comments approximately 5–9 minutes (not measured). B context check and M/N/O transfer questions add about 2–3 minutes if brief; total remains comfortably below 30-minute ceiling, but do not use original 5-second recognition hope as measured fact.

B full-context follow-up: Jörn gives local paragraph PASS and high content value. Initial apparent miss is resolved; preserve both reactions and do not count B as a confirmed false endorsement. Context packet design is empirically consequential.

2026-09-18T12:42:58.692472+00:00: M confirms autonomous code-label concern; N confirms negative-alternative concern but challenges the proposed retained clause. Launch unchanged qualification prompt and generic baseline on proof-dominant chapter 4, seeking cross-subject transfer and possible false alarms. No prompt retuning from M/N. Parent knows broad historical borderline-PASS summary but has not read local chapter 4 feedback.

O response redirects to upstream reproducibility repair. User explicitly authorizes issue reports to main for research-maintenance gaps that burden prose. Sent initial report through Herdr helper; a bounded read-only worker is preparing concrete evidence. No data/code mutation or capacity computation authorized/executed by this trial.

P/Q human responses retain both clauses flagged as likely concerns by unchanged qualification specialist: two confirmed false alarms. Restrict conclusions accordingly; DS positive detection did not establish cross-subject discrimination.

2026-09-18T12:53:19.505235+00:00: Final proof microtests received: P/Q retain (two confirmed false alarms); R needs more context (unscored); S retain (agreement). Closing early because the scoped recommendation is now clear: reproduction-detail candidate has limited positive transfer evidence; qualification prompt should not be broadly adopted; no general acceptance gate. Optional active-time estimate requested while finalizing. No further content judgments requested.

## Completion

Closed 2026-09-18T12:54:42.973773+00:00; elapsed approximately 35.7 minutes of the authorized 60. The proof transfer provides a decisive boundary for the qualification prompt, so the trial ends early. All requested content judgments are recorded; D/R remain legitimately unscored for context. Optional human active-time estimate was offered and had not yet been answered at closure. Provisional total allowance 10–18 minutes, uninstrumented; no measured compliance claim.

Eighteen evaluator runs over seven texts. Exact sources and twenty-nine frozen manifest entries verified; report links resolve and git diff --check passes. Independent scoring audit corrected stale report status. No manuscript/data/code/installed-guidance changes; no expensive computation. Maintained trial worktree was supplied by main and was not created or removed by this task. Temporary duplicate directories created during this task were removed.

Final recommendation: keep reproduction-detail review as an experimental scoped flagger with one newly confirmed non-seed detection; do not adopt the qualification reviewer generally after two proof-text false alarms. Neither silence nor suggested edits establish human acceptance. The upstream provenance/schema issue was delivered to main in commit 12869c30.

### Human time estimate received at handoff

Jörn estimates “prob 10-45s per question? unsure about question count”. Fifteen unique passages plus B context follow-up make sixteen judgments: about 3–12 minutes by that estimate, plus incidental reading/coordination. This supersedes the agent’s provisional allowance as preferred evidence; no precise measured active duration is claimed. Exact account: human/time-budget.md.

## Post-trial interpretation correction

Jörn correctly challenged the inference from “two false alarms” to rejecting broad use. The earlier closing/adoption recommendation above is superseded by evaluation/qualification-utility.md. Complete inventory: unchanged qualification prompt reviewed four excerpts, approximately 7,431 whitespace-separated source text/formula tokens, and issued seven flags: two human-confirmed concerns, two confirmed false alarms, three unresolved. Two false alarms were in the eight-page quadratic chapter. Their existence does not establish that a human-triaged flagger has negative utility; actual benefit and burden were not measured. Both narrow prompts remain candidates for a scoped net-benefit test. No new trial or human judgments requested by this correction.
