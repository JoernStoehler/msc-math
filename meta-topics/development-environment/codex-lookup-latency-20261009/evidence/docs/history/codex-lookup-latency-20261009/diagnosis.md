# Diagnosis and bounded intervention

This concerns the repeated copy-formatting corrections and subsequent attempted
fixes on 9 October 2026. The incident intakes preserve the user complaints;
[test evidence](README.md) preserves successful and unsuccessful trials.

## Observed waste

- The root changed presentation before checking `/copy` selection behavior.
  Correcting that assumption required reading the picker, not changing the task
  or moving model settings into the launch prompt.
- The root repeatedly invoked separate message review during immediate
  corrections. The existing skill already exempted direct synchronous
  discussion. Its selected gate for prepared deliverables remains unchanged.
- The root searched broad guides and official docs even though the local
  implementation could answer the narrow question. The project entry point
  pointed to a long guide before exposing the source location. Several fresh
  agents independently followed that detour.
- Independent skill reads, repository searches and trace/logging work were
  often serialized across orchestration calls. The shell calls themselves were
  short; repeated model/tool rounds added delay. Examples and timestamps are
  retained in the native traces referenced by tests.json.
- The root wrote speculative guidance and an answer-specific source-map entry
  before establishing a behavioral baseline. Structural validation and message
  approval did not establish that the workflow was fixed. These additions were
  removed.
- The first trace inspection omitted custom-tool events and incorrectly
  reported no tool calls. Correcting that mistake added inspection and logging
  rounds. The intake retains the original claim and its correction.
- Some short-lookup agents sent readiness messages to the parent before the
  requested final answer. The final-copy trace at 09:07:21–09:07:52 exhibits this
  extra return path after three retrievals; it missed the deadline.

These are observable routing, scope and completion failures. They do not
establish an internal model-level explanation for every delay.

## Retained change and scope

The existing source-routing paragraph in project AGENTS.md now exposes the
already-available CLI source directory and contextual search, with revision and
live-behavior caveats. The existing collaboration section says a completed short
lookup returns its answer directly; interim private messages remain useful for
consequential blockers or decisions.

The installed OpenAI Docs skill and reference were restored to their original
contents after exploratory changes. The project message-review skill, model and
Codex config were not changed. There is no retained `/copy` answer in the source
map and no new lookup tool, daemon or service.

This intervention targets source discovery and completion overhead in this
project. It does not demonstrate corrected judgment across arbitrary future
tasks, make every message exempt from review, or guarantee a 20-second response
under all runtime conditions. The final independent tests, including their
limits and earlier regressions, determine the observed result.

## Independent diagnosis and stop, 09:18 UTC

A fresh read-only assessor inspected this packet without implementing or
running experiments. It identified the central failure as expansion of a small
correction into repeated guidance changes and fourteen timing trials. The
source lookup detour was subsidiary. It found further timing trials or a lookup
script unjustified without a concrete decision that their outcomes would
resolve. No model-level causal explanation was established.

The root stopped the trials. The retained project completion guidance now
requires delivering an already-supported correction before expanding a workflow
diagnosis and ties further checks to decisions they would change. Failed
acceptance tests remain failures; passing samples do not erase the failures.
This guard has not undergone a new behavioral evaluation; another unmotivated
timing trial would repeat the diagnosed failure.

The original 20-second fresh-agent test failed. Later passing samples establish
that the task can finish within that time in this environment, but the latest
two copy trials still failed (27.749 and 21.128 seconds). Reliable latency has
not been established. All trials and the narrower successful raw-command cases
are retained in tests.json. No further experiments are launched by this report.

## Activation withdrawn after Jörn's objection, 09:30 UTC

Jörn challenged both the placement of CLI details in AGENTS.md and the lack of
consultation about whether the changed guidance was good. The root removed all
its AGENTS.md amendments and preserved the diff in
[unactivated-AGENTS.patch](unactivated-AGENTS.patch). Earlier descriptions of
retained guidance above describe the then-active state, not the current state.
The original installed skill and review gate are also unchanged. Test evidence
does not authorize reactivation. No new intervention is selected or activated.
