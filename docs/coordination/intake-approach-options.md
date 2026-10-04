# Intake approaches for discussion

Status, 3 October 2026: **unselected candidates, not established remedies**.
Author: fresh-context agent `/root/intake_approaches`; this concise rendering
was prepared by the receiving root. The agent received the original user
request and a bounded account of the withdrawn attempt. It did not inherit the
parent conversation. Its framing still came from the parent, so freshness is
not independent evidence of correctness. No candidate probe has been launched.

Source request: [original user message](../history/coordination/20261003-project-repair-user-report.md).
The session is for intake/triage, with resolution later. Missing causal
diagnoses or validated remedies do not themselves make intake unsuccessful.

## Different starting points

| Candidate | Intended useful output | Main uncertainty | Failure detector / reason to revise |
| --- | --- | --- | --- |
| Account for the reported concerns first | Concern coverage and dispositions, evidence pointers, related concerns and open questions | Whether descriptions are useful to a receiving thread; shared causes and current applicability | Entries collapse consequential distinctions, lose concerns during grouping, or merely restate input without helping the intended consumer |
| Reconstruct one concrete incident first | Request, available information, action, user intervention, resulting state and evidence gaps | Representativeness, accessibility and whether the behavior still occurs | Trace exceeds its evidence; transcript recovery cannot affect triage; one incident is promoted into a general cause |
| Test one resolution handoff with a fresh consumer first | A brief containing reports, prior attempts, uncertainty, boundaries and a first investigation; consumer identifies missing context | Whether one brief covers the project; whether starting correctly leads to successful resolution | Consumer needs parent-only knowledge, treats hypotheses as facts, repeats failed remedies or silently substitutes implementation for investigation |

These can be combined; they are not mutually exclusive architectures. The
starting point changes what information is obtained first and where effort is
spent. Whether their results are valuable remains to be checked.

## Agent recommendation and untested assumptions

The returned recommendation was a small concern-coverage slice and one incident
trace, potentially concurrent, followed by a fresh-consumer test of a resulting
brief. Starting all three at full scale would add integration work before
testing whether their outputs help. No numerical cost or savings is established.
Concern coverage risks broad context growth; incident reconstruction risks
costly recovery; consumer tests add worker/context/latency and review costs.
Avoid making Jörn reconstruct evidence that agents can recover.

Known limitation: the agent did not establish which of these approaches had
already been tried, under what conditions, or why they failed. Novelty and
advantage over prior attempts remain unknown. Relevant sources should be
retrieved before claims about those matters are made.

The untested assumptions include accessible representative episodes, useful
omissions exposed by fresh consumers, and preparation/review costing less than
continuing in the parent. User reports, source evidence, agent interpretations,
attempted remedies and observed outcomes need distinct attribution. Unknown
outcomes must remain unknown; a confident agent summary does not settle them.

## Placement and revision

The returned placement recommendation: fresh agents for bounded brainstorming,
evidence reconstruction and consumer tests; this conversation for Jörn's
corrections, comparison, tradeoffs and integration. That does not select a
long-running coordinator architecture. Keep the parent returns concise and
source-grounded; do not copy the entire conversation into every worker or hide
contrary evidence merely to shorten a brief.

If a probe produces one of the stated failure signs, revise its scope or
starting point rather than treating the initial method as a permanent premise.
These are candidate-specific observations to discuss, not new universal
approval gates or automatically authorized execution.
