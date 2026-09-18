# Reviewer-development trial

**Corrected result:** both narrow prompts remain plausible human-triaged flaggers. The qualification reviewer issued seven flags over four excerpts (about 7,400 source text/formula tokens): two human-confirmed concerns, two false alarms, three unresolved. The two false alarms alone do not establish that it is unhelpful. Net human benefit was not measured; automatic repairs and acceptance gating are unsupported. See the [surface and utility accounting](evaluation/qualification-utility.md).

Trial window: 2026-09-18 **12:19–13:19 UTC / 14:19–15:19 Berlin**. Human active-effort ceiling: 30 minutes. Completed 12:54 UTC after approximately 36 minutes, ending after the proof-transfer check; the subsequent utility interpretation was corrected below. No manuscript edits or reviewer activation.

## What is worth retaining

| Workflow | Concrete evidence | Boundary |
| --- | --- | --- |
| [Reproduction-detail reviewer](development/reproduction-detail-prompt.md) | After seed-detail feedback, independently found the unused name “factorial-both” in another draft. Jörn confirms removal because the name serves no purpose there. | One newly adjudicated non-seed finding; same research lineage. Keep mathematically consequential sampling/evidence restrictions. |
| [Unraised-qualification reviewer](development/qualifications-prompt.md) | Independently flags the denial that the pentagon formula predicted the original observations; Jörn confirms the unraised alternative. Also identifies the significance-test denial before that answer, but after its request. | Its suggested retained clause remains unclear to Jörn. On proof prose it falsely flags two clauses Jörn retains. This creates review cost, but does not by itself establish negative utility as a flagger. |
| [Passage classifier](development/classification-prompt.md) | Identifies the seed-placement problem missed by open reviews; retains the face-area definition clarification that Jörn accepts. | Candidate locations were supplied. This shows judgment, not autonomous discovery. |

A practical next use is a short scoped pass that returns exact spans and the reader cost, followed by a context-aware decision about each flag. Preserve qualifications that change a mathematical inference. Treat silence as unknown. No skill was installed and no maintained agent guidance was changed.

## What failed or needs caution

- **Broad relevance:** on the historical HKO chapter, one of seven recorded local objections is matched (direct/partial boundary), five are missed, and one human-rejected qualification is expressly praised. Baseline only partially matches one. These are selected-label comparisons, not accuracy estimates.
- **Reader-question reconstruction:** finds local motivation/order problems in historical DS but misses the requested discovery-to-insight organization. Baseline misses that organizational complaint entirely.
- **Prerequisite reconstruction:** finds analytic vocabulary issues but misses both recorded preliminaries prose objections. Its new concerns are unadjudicated, not confirmed false alarms.
- **Location is not diagnosis:** all initial reviewers flag the control passage J, mostly seeking more explanation. Jörn questions why the implementation detail deserves attention. K is a useful observation with local wording defects; the reviewers do not fully match that judgment.
- **Repair is separate:** for N, deleting the detected negative clause leaves another clause Jörn calls slop. Do not automatically apply the reviewer’s proposed fix.

## Context changed an apparent miss

The isolated sentence B initially looked unclear to Jörn. After seeing its actual correlation paragraph, he called the text **PASSing and high-value**. That agrees with the frozen reviewer endorsements. Record a local paragraph PASS, not a whole-draft verdict. D remains unjudgeable without its referents. The accepted definition explanation L also shows that apparent formula restatements are not automatically unwanted.

## Repairing the research base can be the right action

For O’s historical-data disclaimer, Jörn prefers repairing repository reproducibility to preserving caveat prose. The [issue report sent to main](issue-reports/historical-ds-provenance.md), commit `12869c30`, identifies two distinct gaps: a recoverable feature-schema mismatch and missing historical execution guarantees. Its next step is artifact retrieval, hash/geometry joins and witness inventory with **zero capacity calls**. Importing a schema cannot certify an old execution. This trial changed no datasets and launched no expensive reruns.

## Evidence and limits

Eighteen single-run reviewer evaluations cover seven texts, with three separate developer workers and historical scoring after predictions froze. Evaluators started without inherited conversation and received specified source/prompt files; model settings were inherited unchanged. All texts share thesis/project lineage. Revised DS and the alternate draft are related to the development material. Parent knew historical overall summaries; detailed historical labels were withheld from evaluators. No representative sampling, repeatability, model comparison, or contamination-free holdout is claimed.

Human judgments are purposive and contextual. Some agent progress summaries were visible before answers, so the trial is not blinded. Jörn estimates 10–45 seconds per question. Sixteen passage judgments imply roughly **3–12 minutes**, plus incidental reading/coordination; this is subjective, not instrumented. See [time accounting](human/time-budget.md). Original per-question time estimates were optimistic when Jörn chose to explain his reasoning. The ceiling remains 30 minutes.

- [Human scorecard](evaluation/human-comparison.md): detections, misses, partial matches, acceptance and upstream actions separated.
- [Exact responses](human/): spelling, qualifications and context follow-ups retained.
- Historical comparisons: [HKO](evaluation/historical-comparison.md), [DS narrative](evaluation/narrative-historical-comparison.md), [preliminaries](evaluation/preliminaries-historical-comparison.md), [quadratic chapter](evaluation/quadratic-historical-comparison.md).
- [Run record](RUN.md): timing, exposure boundaries and adaptations; [source manifest](inputs/manifest.json), freeze manifests under evaluation/, and [integrity checks](evaluation/integrity-check.md).

There is no overall precision, recall, false-alarm rate or PASS-prediction estimate. Unmentioned historical concerns remain unadjudicated. Both narrow prompts justify further bounded evaluation as human-triaged flaggers. On the final proof set, P/Q are false alarms, R remains unjudgeable, and S is a correctly retained clarification. Neither reliable cross-chapter human-time saving nor negative utility is established.
