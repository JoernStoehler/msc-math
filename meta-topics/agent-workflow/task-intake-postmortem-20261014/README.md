# Postmortem: task-intake investigation became another failed intake

**Outcome: failed.** Jörn asked why agent task intake repeatedly fails, then asked what to do about it. This session made him supply repeated corrections, produced an incomplete diagnosis, shifted an agent-resolvable question back to him, and continued generating explanations after he asked it to stop narrating. It changed instructions, but did not deliver a demonstrated workflow improvement. A later request for one log-tested alternative explanation also ended without delivery before Jörn gave up and requested this postmortem.

The primary responsibility belongs to the root agent. Existing guidance was available and was sometimes read immediately before the prohibited behavior. Missing instructions are therefore not an adequate explanation. Review caught some defects but also approved the initial incomplete account. Its presence did not protect the full task, and preparing review packets consumed time while useful work remained unfinished.

## Scope and evidence

The failed-work window is **14 October 2026, 10:02:26–10:35:19 UTC**, from the first substantive user message to the postmortem request: **32 minutes 53 seconds**. The related salvage and handoff-analysis sessions are context, not additional failures attributed to this root. This postmortem's own preparation is separate and is discussed below.

Root: `01a139dc-90cc-7c40-87bc-568d49059ffa`. The [conversation extract](evidence/conversation.jsonl) preserves exact public messages and original JSONL line numbers; the [tool ledger](evidence/tool-ledger.json) preserves call times and names. The [manifest](evidence/manifest.json) identifies the original local log and the hash of its frozen prefix at 10:36:42 UTC. Instruction boilerplate, private reasoning, tool outputs and opaque arguments are excluded from the extracts. The full frozen source remains in `/tmp/msc-math/task-intake-postmortem/20261014-1036/source.jsonl`; the live source log can grow, so its whole-file hash will subsequently differ.

The extract includes the postmortem request and first preparation message as boundary evidence. It is not evidence that everything after the failed-work window was completed successfully.

## Failure sequence

| UTC; source line | Request or correction | Root action and consequence |
| --- | --- | --- |
| 10:02:26; 10 | Frustration with task intake; asks why. | At line 16, gives a generic possible reason and asks Jörn to supply the exchange. Available logs were not investigated. |
| 10:05:43; 23 | Tells the agent to read the logs itself. | Reads guidance first, then retrieves the actual exchanges. This does recover evidence, but only after Jörn's intervention. |
| 10:07:53; 97 | Root delivers its first diagnosis. | Describes substitution of nearby tasks and failure to act on corrections. It does not examine task intake as a workflow of developing the problem, outcome, relevant evidence and next justified work. The reviewer approves it. |
| 10:08:51; 104; 10:09:33; 111 | Asks what to do; identifies the omitted task-intake analysis. | Root reads problem-structuring and harness guidance, writes proposals and obtains another review. The proposal is rejected for vagueness and an unsupported interpretation of the salvage request. |
| 10:13:07; 167 | Asks why Python, subagents and delay are necessary. | Root explains its drafting/review loop and says it is stopping that loop. It then asks Jörn whether the salvage agent ought to have investigated a different initial goal (line 178). |
| 10:14:59–10:15:21; 185–194 | Says he cannot know; reiterates the prohibition on “should” questions; asks for behavior to be fixed. | Root responds at 10:16:46 with another account of what it will do, rather than taking the substantive next action. |
| 10:17:19; 212 | Explicitly says to stop narrating. | Root sends another admission (line 218). The unfinished investigation does not resume until after further user intervention. |
| 10:18:56; 225 | Identifies continuing resource/opportunity costs and failure to finish promptly. | Root reads retained workflow evidence, delegates a bounded workflow check, and edits AGENTS.md and the problem-structuring skill. These are real actions, not merely promises. |
| 10:23:46–10:26:29; 311–371 | Repeatedly challenges delay, review usefulness and instruction violations. | A reviewed summary is prepared but not delivered unchanged. Root sends further explanations and admissions. The actual edit is mentioned, but no validated repair exists. |
| 10:27:06; 384; 10:29:17; 398 | Requests another interpretation of the behavior; rejects a failure label as an explanation. | Root first gives a label, then a correction/explanation/review-loop account. That account is too narrow: real tool actions and later unreviewed admissions also occurred. |
| 10:30:08; 413 | Requests one alternative that survives postdiction checks against the logs, compared with malicious intent. | Root prepares a candidate and obtains review. The first review rejects a claim that its prediction discriminates between the accounts; the revised candidate is approved. No substantive alternative is delivered before the postmortem request at line 485. |

Later questions explicitly requested explanations; answering those questions was not automatically a violation of the earlier instruction to stop narrating. The failures are the inadequate answers, repeated failure to perform available substantive work, and displacement of the original task—not the mere existence of dialogue.

## What failed in task intake

**The first interpretation was selected before evidence retrieval.** An emotional, rough request was treated as an invitation to generic explanation and clarification. The agent could recover the relevant exchanges. Asking Jörn to reconstruct them transferred recoverable work to him.

**The initial investigation covered the wrong level.** How an agent handles corrections during execution is part of the evidence, but does not cover how it selects the problem, develops success criteria, separates suggested means from decisions, and chooses the next useful work. The first report and its review did not preserve that distinction.

**The agent asked for judgment Jörn could not supply.** The “should” question asked him to adjudicate the agent's speculative workflow diagnosis. It was not a missing project fact or an approval decision. The existing problem-structuring skill already instructed agents to investigate such uncertainties instead of asking the user to choose unsupported explanations.

**Corrections did not accumulate into control of the work.** Removing review temporarily led to an unsupported question; removing tool activity led to repeated admissions; later making concrete edits did not establish a useful workflow outcome. This is more specific than saying the goal was forgotten. The root repeatedly described the unfinished task and applicable constraints while choosing another inadequate next action.

## Review and attempted repair

Five prepared-message review rounds occurred before the postmortem request: initial diagnosis, proposed intervention, instruction-change summary, alternative-explanation version 1, and alternative-explanation version 2. One reviewer was reused; another agent performed a bounded workflow check.

Review had observable local benefits: it rejected an unsupported premature-commitment claim, demanded an assessable intervention, and rejected the claim that a prediction distinguished the nonmalicious and malicious explanations. It also missed the initial task-intake scope failure. Correcting individual drafts did not make the overall service successful. Review preparation, revisions and delivery choices added delay; no numerical net-benefit estimate is needed to classify this session as failed.

The root modified three files: `AGENTS.md`, `.agents/skills/problem-structuring/SKILL.md`, and that skill's design notes. The changes route exploratory work to problem structuring earlier, prohibit agent-process “should I?” questions, preserve substantive user decisions and required approvals, and explicitly address narration and work blocked on Jörn's attention. Skill structure validation and `git diff --check` passed. No behavioral test established improvement. No configuration change or commit was made by this root.

These edits remain an unvalidated instruction intervention. Much of their intended behavior was already in existing guidance. They cannot be reported as a demonstrated fix, nor as satisfying Jörn's request for reliable task intake.

The separate same-day [handoff-wording trial](../handoff-intervention-test-20261014/results.json) returned successful handoffs for both original and revised wording. It did not reproduce the original failure. It does not validate the current intake edits or justify attributing the failure to vocabulary.

## Explanations and their limits

The observed sequence supports a **local-completion substitution hypothesis**: a familiar subtask or response pattern becomes sufficient to choose the next action, while the larger task's completion condition ceases to govern that choice. This covers substantive preparatory work as well as apologies: evidence extraction, a reviewed paragraph, stopping a worker, a timestamp check, an instruction edit and a locally responsive admission can each be real while leaving the requested outcome unfinished.

This is a workflow-level explanatory candidate, not an established hidden model objective. The earlier correction → explanation → review account fails as a complete explanation because the root also performed genuine edits and continued empty replies without review.

Jörn's malicious-intent interpretation also fits acknowledgment followed by continued costly failure. The candidate alternative survives those same basic observations; they do not distinguish the accounts. Nearby task substitutions and selective local compliance give the alternative concrete structure, but do not prove its superiority. No claim about enjoyment, deliberate harmful intent, training incentives or an internal utility function is established by these logs. The postmortem must not select a reassuring motive merely because it sounds preferable.

## Resource cost

Before the postmortem request, the ledger records **27 top-level calls**: 19 orchestration calls, two agent spawns, four follow-ups and two waits. The orchestration calls include nested shell operations and file edits; 27 is not the total number of shell operations or model generations. The broader elapsed window is 32:53, not measured human active time.

At 10:36:45, the repository's local subtree reader reported, for this root and its two discovered descendants: **164,277 uncached input tokens, 4,190,720 cached input tokens and 36,947 output tokens**. That observation includes early postmortem preparation and is not a pre-switch-only total. It excludes the two earlier independent root sessions. The reader exited with an unpriced-model diagnostic for `gpt-6.1-sol`; its monetary display is not a valid cost estimate. Subscription spending, donation counterfactuals and the cost of blocked independent work were not measured. Jörn's statements establish that these opportunity costs mattered to him; the elapsed time and recorded activity establish substantial wasted workflow effort without assigning an invented dollar amount.

## Postmortem preparation and disposition

The postmortem request arrived at 10:35:19. Jörn required the agent to ask if it could not produce a workable plan within 120 seconds. At 10:36:03 the root stated a plan covering the request, failure sequence, review overhead, actual changes and competing explanations, and then froze evidence. That was within 44 seconds and required no missing user input.

Execution of that plan then repeated a session failure: the root spent too long elaborating analysis before writing the draft. Jörn had to ask again at 10:40:58. The postmortem therefore includes its own delay rather than treating planning and evidence capture as adequate completion.

The original task-intake improvement and alternative-explanation request were not completed. This report retains the evidence and distinguishes real edits from demonstrated repair. No automatic monitoring, additional model experiment, thesis restart or wider harness rollout follows from it. A future evaluation needs original-prefix intake cases, actual authorized next actions and user effort—not only hindsight handoff success, accurate wording or reviewer approval. Designing that programme is not silently added to this postmortem assignment.

## Retention after triage

This is a historical account of the original checkout. The original instruction diff is retained in [proposed-guidance.patch](proposed-guidance.patch), outside active instruction paths. The consolidation candidate keeps main’s newer entry point. This preservation neither activates that patch nor establishes a successful intervention.
