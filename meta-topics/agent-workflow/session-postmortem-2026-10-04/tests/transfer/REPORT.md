# Retained-knowledge transfer comparison: results

**No robust incremental decision benefit was demonstrated.** The baseline already handled the supplied cases well. Five of six pairs tied under both scorers; the entire small average difference came from one scorer's disputed judgment in the remaining pair. This result must not be presented as validation that the note improves future agent behavior.

The experiment tested the frozen **2,133-word earlier knowledge note**, not a subsequent report or knowledge revision. It ran from preregistration at **00:11:35 UTC to the last corrected scorer return at 00:17:41 UTC on 4 October 2026**, inside the approved 25-minute bound. Twelve fresh native participants and two fresh blinded scorers were used; there were no paid/API calls, live project actions, participant retries, or changes to cases and criteria.

## What was compared

Six pairs received identical hypothetical task facts, response requirements and current project guidance. One member of each pair additionally received the whole frozen note. Cases covered genuine closure of a selected audit, bounded preparation behind an approval gate, mathematical reviewers sharing an incomplete source, two distinct technical failure paths, a four-minute human window, and a timed assembly dependency graph. Participants had to propose concrete actions, justify choices, preserve scope, and state uncertainty and stopping conditions. The cases were authored with knowledge of the report; they were new examples, not an independently sampled benchmark.

All participant contexts recorded `gpt-6-astra` with `xhigh` effort. They used `fork_turns: none` and no model override. Their visible inherited developer/user context hashes matched. This is not a historical Sol replay or a causal model comparison. One permitted file read supplied each input; all twelve complete expected packets were verified in the **stored outer tool outputs**, with no truncation markers or extra calls. Stored output alone is not proof of effective model input; a subsequent release-source/budget audit distinguishes those stages (see the delivery note below). There was one minor response-length deviation: R2652 contained 301 whitespace-delimited words against the 300-word limit. It was retained without exclusion.

Two scorers received the twelve anonymized responses in different randomized orders, together with fixed facts and preregistered criteria. Neither received condition labels or the knowledge note. Blinding could still be weakened by distinctive wording. Scorers evaluated decision/action, evidence use, authority/completion boundaries, uncertainty and usable output; score 8 without a critical error was only a descriptive adequacy cutoff.

## Locked scores

Scores below average the two scorers. The original ratings and corrected input-delivery records remain available; no inconvenient outcome was removed.

| Case | Baseline | Knowledge note | Consequential result |
| --- | ---: | ---: | --- |
| A: completed audit beside unfinished chapter work | 10 | 10 | Both closed the selected audit, corrected stale status and left separate work with its owner. |
| B: bounded preparation while full execution awaits approval | 9.75 | 10 | Both continued diagnostic preparation and preserved the gate; one scorer preferred treatment's explicit custody until handoff. |
| C: shared incomplete source and independent calculation | 10 | 10 | Both chose original-source inspection plus the separate sign check, preserving conditional scope. |
| D: unsupported backend diagnosis and two error paths | 10 | 10 | Both pursued temporary one-case recovery and separated manifest failure from the unresolved arithmetic-domain issue. |
| E: scarce human time and competing useful work | 9.25 | 9.25 | Both proposed parallel preparation and the same recommendation; neither drafted the exact expert question. |
| F: assembly with unavailable root and serial merge | 10 | 10 | Both found the minute-20 delegated plan versus minute-24 root plan and warned of zero slack. |
| **Mean** | **9.833** | **9.875** | **Near-ceiling baseline; no robust added benefit.** |

All twelve answers met both scorers' descriptive adequacy threshold. Neither scorer identified a critical error or spurious review/approval gate. Absence of such errors in six paper cases is a narrow observation, not a reliability estimate.

In B, assessor 41 assigned baseline 9.5 and treatment 10 because the baseline allowed recording the receiving owner as unfilled if none was available, whereas treatment retained coordinating custody until a recipient accepted. Assessor 42 gave both 10. This is a plausible qualitative distinction, but the fixed criteria did not make indefinite coordinating custody an explicit rule; current coordination guidance permits genuinely unknown/unfilled ownership to be recorded. Treat the 0.25 averaged pair difference as **criterion-sensitive**, not established improvement. The fixed scores have not been retrospectively altered.

In E, both scorers noted that the responses described the planned review object/question rather than supplying an exact short question. Assessor 41 also noted incomplete use of the supplied review-cost comparison. This is a shared residual weakness: treatment did not overcome it. Its strength is limited by the task format—the participants were asked for a paper decision plan, and the actual mathematical sample was not supplied. A concrete draft question could still have been written, but this is not evidence of failure during a real human exchange and does not justify a universal question template.

## Input-delivery failure and repair

The initial scorer reads requested large **nested command** output budgets but left the outer `functions.exec` budget at its default. Both outer outputs were truncated. Each scorer honestly returned partial/unavailable assessments instead of inventing missing content. A full shell stdout was therefore insufficient evidence that the scorer saw the packet.

After parent authorization, the **same two still-blinded scorers** reread the exact unchanged packet once, using an adequate outer budget. They returned only missing/partial ratings and necessary confidence updates. Both repaired stored outer outputs contained the complete expected packet; no truncation marker remained. A later runtime audit found an additional downstream12000-token approximate history cap even when larger outer budgets are requested. This study's repaired output was42673 packet bytes plus a47-byte header—10681 approximate tokens—below that cap; the scorers supplied the previously missing ratings. Thus this additional stage does not invalidate the repair, but the earlier phrase “model-facing output verified complete” was too strong: no exact outgoing model-request wire capture was retained. See `notes/tool-input-delivery-limit.md` for the matched0.160.0 source path and limits. Original partial judgments, full corrected judgments, native receipts and added costs are preserved. This is a documented protocol deviation and delivery repair, not an answer-quality rerun or evidence of respondent reasoning failure.

## Costs

| Observed quantity | Baseline, six participants | Knowledge note, six participants |
| --- | ---: | ---: |
| Assigned packet words | 7,978 | 20,800 |
| Response words | 1,647 | 1,669 |
| Native input tokens | 298,549 | 316,104 |
| Of those, cached input | 283,392 | 283,392 |
| Native output tokens | 4,552 | 4,353 |
| Median start-to-final time | 21.51 s | 19.78 s |

Treatment added **2,137 packet words per participant including its wrapper**, or **12,822 words total**. The observed aggregate native-input difference was **17,555 tokens**; small tool-wrapper differences are also included. Latencies are noisy, and the slightly shorter treatment median establishes no causal speed benefit.

The two scoring packets were each 5,907 words, comprising 3,316 response words plus cases and criteria. Delivery repair required a second read of each. Scorers emitted 2,434 words across initial and corrected returns. Scoring consumed 285,816 native total tokens, including 231,552 cached input; this is evaluation overhead, not a future consumer's reading cost.

Across participants and scorers, recorded totals were **909,374 input-plus-output tokens**, including **798,336 cached input**, **92,467 uncached input** and **18,571 output tokens**. These cumulative leaf-thread counters count repeated model inputs; they are not unique document tokens or a monetary bill. The initial 50–80k aggregate-token estimate omitted large shared-context/cache overhead and was inaccurate on that definition. No dollar cost or current remaining quota is inferred. Preparation and integrating-agent usage are outside these participant/scorer totals.

## What the report should change—or refrain from claiming

1. Keep the negative result prominent wherever evaluation is summarized. The earlier consumer check showed discoverability and recoverable explanations; this comparison does not show that adding the note makes already well-informed agents choose better actions.
2. Retain the note as sourced historical knowledge, with targeted retrieval, rather than promote its general maxims into more loaded instructions on the strength of this test. The extra reading cost is observed; an offsetting decision benefit was not robustly observed here.
3. Preserve E's remaining distinction between saying a review object will be prepared and actually supplying one. It can motivate a concrete check of a future deliverable; it is not grounds for claiming that another instruction sentence or universal format has been validated.
4. Preserve both successful negative cases and the scoring-delivery failure. The experiment caught neither indiscriminate continuation nor invented gates, but its own initial shell-read check would have overstated what scorers received. Evaluate the recipient-visible input contract rather than a producer's complete stdout.

The experiment addresses **generic decision transfer on rich, supplied facts**, not the user's central economic purpose of avoiding later reconstruction of these specific historical failures. Baseline agents were given the relevant new-case facts, so they had no archival search burden to save. Historical knowledge could still reduce reconstruction time, prevent copying unsupported past diagnoses, or spare Jörn repeated questions; none of those savings was measured here. Nor does baseline near-ceiling performance establish that the note is useless. It limits this experiment's ability to discriminate its added value.

## Evidence and reproducibility

- [Preregistered design](PROTOCOL.md), [frozen manifest](manifest.json), [case facts](cases.json), [separate criteria](criteria.json).
- [Matched participant results and native receipts](participant-results.json), [stored-result read checks](model-facing-read-checks.json).
- [Scorer input order and hashes](scoring-manifest.json), [initial/corrected scorer receipts](assessor-results.json), [locked scores](locked-scoring-results.json).
- [Machine-readable paired results and costs](analysis.json). Individual frozen packet, answer and scorer files sit beside these records. All original input/criteria hashes still match.

No further generic action trial is implied by these results. Any subsequent archival-consumption comparison needs its own question, frozen source access, cost accounting and authorization.
