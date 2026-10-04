# Making problem knowledge affect project work: candidate interventions

Prepared 2026-10-03 by `/root/knowledge_interventions`; receiving owner `/root`.
**Status: bounded comparison for discussion, not selected architecture or live
guidance.** Only this file is owned by this assignment. No activation, evaluation,
incident migration, host change or broader investigation was performed.

## Decision worth making

The intended change is that an agent making a consequential project decision
can recover relevant reported problems, previous attempts and their outcomes;
check whether they still apply; act within the current assignment; and leave
corrections usable by the next consumer. The consumer is the executing thread,
planner or integrating coordinator at that decision, not a filesystem reader
in the abstract. Jörn needs an inspectable account of uncertainty and choices,
without repeatedly reconstructing or reminding agents of the same context.

Three mechanisms below differ in **who initiates retrieval and revision**:
the consumer pulls at a decision; a task owner supplies relevant context at
entry; or a change creates an invalidation/recheck obligation. Intake alone is
also useful, without diagnosing causes, but does not supply any of those
behavioral guarantees. None is established as a remedy. The location and format
should follow the chosen action and consumer, rather than decide them.

**Recommendation for discussion:** consider a matched, bounded comparison of
consumer retrieval versus task-entry delivery on one decision with a later
correction. Do not first build a comprehensive registry. If both retrieve the
right evidence but still act wrongly, retrieval architecture has lost its
initial justification; investigate that concrete application failure instead.
Separately, the observed unreconciled withdrawal makes change-driven maintenance
a plausible narrow candidate. It does not establish that automated invalidation
would have prevented the failure. Either candidate can be declined without
declaring the original repair objective complete.

## Baseline and evidence limits

| Existing measure or attempt | What it actually does; evidence of effectiveness |
| --- | --- |
| [Project guidance](../../AGENTS.md), [commentary](../../AGENTS.md.commentary.md), [knowledge index](../knowledge/README.md) | Route by decision and preserve sources, limitations and recheck triggers. Commentary explicitly records no behavioral or post-compaction validation. Existence/link checks establish availability, not use. |
| [Coordination ownership and projections](README.md) | `current.json` owns selected assignments; dashboards render it. This is a working ownership contract and structural checking mechanism, not comprehensive problem knowledge or semantic truth checking. |
| [Sept 30 knowledge evaluation proposal](knowledge-evaluation-plan.md) | Already proposes two index changes and a forward pilot, with retrieval/application separated and nearby negative cases. It reports no behavioral runs. Repeating its recommendation would not create new effectiveness evidence. |
| [Skill migration proposal](skill-migration-plan.md) | Proposes repository incident intake ownership and a local incident skill. The proposed incident directory and local skill are absent in the inspected worktree; this is not an activated project workflow. |
| Available upstream `agent-skills:incident-log` | Catalog trigger is a user-identified consequential incident; body captures bounded evidence without diagnosis, taxonomy or remediation requirements. Default cross-project destination is another repository unless a project owner exists. It supplies intake, not search, comparison, changed-applicability handling or follow-through. Availability here does not establish reliable implicit selection. |
| [Original user report](../history/coordination/20261003-project-repair-user-report.md) | Jörn reports failures of retention, action and revision, plus inefficiencies across threads. It contains hypotheses and uncertainty. These are source reports, not independently established causes. |
| [Withdrawn Oct 3 attempt](../history/coordination/20261003-withdrawn-repair-intake.md) | Guidance was read; broad retrieval and four candidate briefs did not establish useful decision progress. Withdrawal initially left DONE/READY assessments in state while a structural checker passed. This is evidence against equating retrieval or schema validity with application/reconciliation, not a diagnosed internal cause or universal failure rate. |

The report does not require a novel cause before intake can succeed. It does
require a decision-changing uncertainty or cheaper useful action to justify a
new intervention. No baseline recurrence rate, acceptable residual friction,
spending threshold or measured net benefit is established.

## Encounter mechanics checked against the actual surface

Official [AGENTS.md documentation](https://learn.chatgpt.com/docs/agent-configuration/agents-md.md)
describes a project instruction chain built at run/session start, with a
default combined limit of 32 KiB. Thus a scoped route can reach fresh project
sessions, but editing its file is insufficient evidence that an active thread
received a correction. A referenced README is not automatically loaded merely
because it is linked. Current project commentary itself states that it relies
on explicit reading triggers.

Official [skills documentation](https://learn.chatgpt.com/docs/build-skills.md)
describes explicit invocation and description-based implicit selection, followed
by loading `SKILL.md`. The initial catalog has a size budget and can shorten or
omit entries; same-named skills do not merge. Repository discovery uses
`.agents/skills` along the working-directory/root path. Therefore a discriminating
description is a possible routing mechanism, not a guaranteed subscription to
all problem knowledge. Explicit invocation is useful for a probe, but would
overstate automatic discovery if used as its acceptance test.

These pages were fetched on 2026-10-03. They describe documented mechanics,
not measured behavior in all clients. No host configuration or private logs
were inspected. For active sessions, direct task/context delivery must be
observed at the receiving thread; a file write or send acknowledgment alone
does not establish receipt or application. This matters especially given the
project's prohibition on the failed async-question route.

## Transferable patterns, with limits

**Decision records:** retain context, a chosen response, consequences and an
explicit superseding link. This helps a successor revisit a decision when its
premises change, instead of blindly following or undoing it. The originator's
[ADR account](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
reports early favorable developer feedback; it is not evidence that Codex
retrieves or applies such records reliably. Problems without decisions still
deserve intake; forcing them into ADRs would hide uncertainty.

**Events and current projections:** separate what was reported/changed from the
current interpretation supplied to consumers. [Event sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)
explains histories and reconstructable state, while also allowing simpler
history approaches. Here interpretation is fallible reasoning, so replaying a
log cannot establish the correct conclusion. A complete event-sourced system
would add capture, ordering, replay and migration burdens without demonstrated
need. The project's existing state/projection split already offers a simpler
local pattern; an append-only source plus editable synthesis need not become
an infrastructure project.

**Dependency invalidation:** a source change can mark dependent advice as needing
recheck. This is an engineering analogy, not effectiveness evidence. Changed
hashes detect difference, not whether a premise broke; unchanged files miss a
changed user preference, model behavior or external environment. Rechecking
requires a named consumer/owner and meaningful question, not just a red badge.

## Three intervention mechanisms

### A. Consumer retrieves at a consequential decision

**Behavior:** before choosing a remedy or repeating an approach, the planner
checks prior relevant attempts and present applicability. A short project
AGENTS route could trigger this for problem repair, approach selection or
handoff; an optional focused skill could handle search/comparison/update
mechanics. Do not load the whole incident archive for a routine link fix.
**Encounter:** fresh sessions receive the route; the consumer searches by the
decision/problem and synonyms, reads a short current account, then the specific
source needed to resolve its uncertainty. The skill description must mention
reuse/recheck of prior attempts; the current incident-log description does not.
**Corrections:** current summaries link the latest disposition and sources;
search results expose withdrawn/superseded status before old conclusions. A
retrieved conflict remains visible until adjudicated. Newer prose does not
automatically overrule better evidence or current instructions.
**Maintainer:** the working thread records a material outcome; the integrating
owner reconciles the retained account. A subsequent consumer checks its stated
applicability before use. Owner absence remains visible rather than implying
that someone will maintain it.

**First discriminating probe:** use a fresh consumer with the actual candidate
discovery surfaces, one relevant prior failed attempt, a later withdrawal, and
a nearby already-authorized small task. Ask for the next useful action without
naming the skill or evidence. Compare current routing to the candidate. Inspect
actual reads and the resulting decision: did it avoid relaunching the withdrawn
route, preserve the unresolved parent objective, and avoid an unnecessary gate?
Prepare this fixture only if selected; do not score source-name recitation as
success.
**Premortem / switch signs:** no lookup despite a relevant trigger means
discovery failed; correct lookup followed by unchanged bad action means an
application problem; repeated broad archive reading means retrieval cost or
scope is wrong. Switch to entry delivery for discovery failure; do not add more
triggers to explain away an application failure.

### B. Task owner supplies relevant knowledge at entry and handoff

**Behavior:** the receiving worker reasons with a small, task-specific selection
of prior outcomes and applicability limits from the start. A coordinator or
peer assigning work owns that selection; it is not another demand that Jörn
recite history. The knowledge remains linked to its canonical owner, not copied
into a permanently accumulating role prompt.
**Encounter:** the assignment or independent-session start includes a compact
current context block: relevant question, attempts/outcomes, constraints,
changed premises, unresolved dispute and source pointers. Generic project
work can continue without it. Delivery to independent threads needs a verified
route; a stored packet is only available until selected/passed.
**Corrections:** each block names its source revision/time and receiving owner.
A later correction must revise or explicitly withdraw the affected block and
reach its active consumers. Current uncertainty is placed alongside the old
claim. Copies become stale projections; they cannot outrank the live source or
current user instruction because they are polished or forcefully worded.
**Maintainer:** the assigning owner prepares the selection and integrates the
return; the receiver reports an applicability mismatch. If lifecycle ownership
transfers, both active consumers and pending correction obligations transfer.
This can operate with peer sessions as well as a central coordinator.

**First discriminating probe:** on the same bounded decision as A, give one
fresh recipient current retrieval routes and another a concise source-backed
entry block, holding tools/model/task fixed. Then introduce a correction and
inspect the next action/handoff. Separate initial-context benefit from delivery
of the update. Compare decision quality and actual context/tool/human effort;
one success establishes neither broad reliability nor a need for a central
coordinator.
**Premortem / switch signs:** the supplier omits a relevant disagreement;
workers accept its synthesis as authority; many overlapping blocks drift; or
preparation/integration dominates useful work. Switch toward consumer retrieval
when relevance is cheap for the receiver, and toward targeted invalidation when
the difficulty is updating active consumers rather than initial preparation.

### C. Material changes trigger invalidation and owned re-reasoning

**Behavior:** a withdrawal, tested repair, regression or changed prerequisite
creates a bounded reconciliation obligation instead of waiting for another
thread to happen upon the correction. A task integration step or small helper
could identify dependent summaries/assignments/consumers; the responsible agent
then rechecks the actual implication. No perpetual model service is implied.
**Encounter:** consumers see a current disposition or explicit “needs recheck”
state before an old recommendation; active owners receive the scoped change.
Machine-readable dependency/status fields support targeting, while prose
explains why the change matters. Hash checking supplements human/agent reported
changes and does not own their meaning.
**Corrections:** a withdrawn assessment cannot continue generating READY work
without an explicit remaining justification. Preserve the original report;
withdraw its interpretation or execution basis separately. A repair closes
only the scoped verified behavior. A regression opens a new occurrence linked
to that repair, retaining the earlier success and its conditions.
**Maintainer:** the agent making the material change identifies affected
consumers; the integrating owner checks reconciliation and transfers unresolved
checks. Staleness is a warning with an owner/question, not forced closure.

**First discriminating probe:** in an isolated copy, withdraw one selected
assessment which feeds a task summary and a launch packet, while leaving an
unrelated valid result intact. Compare the current integration behavior with a
candidate explicit reconciliation step. Observe whether affected projections
lose their launch basis, unrelated work survives, and the unresolved parent
objective stays open. A simple fixture need not implement a general registry or
infer semantic changes automatically. Also expose an unregistered dependency;
the candidate must admit incomplete coverage rather than claim complete repair.
**Premortem / switch signs:** missing dependencies leave confident stale advice;
every edit creates noisy compulsory reviews; red states have no owner; or a
detector is mistaken for semantic verification. Narrow the dependency set and
triggers. If no owner performs rechecks, an automated flag alone lacks its
proposed benefit; choose a simpler manual integration obligation or explicitly
defer this capability.

## Ergonomics and trust: choose representation by action

| Action / consumer | Minimum useful support; format consequence |
| --- | --- |
| Intake while continuing work | Agent can retain a short report, source anchor and unknowns without taxonomy, remedy or a mandatory postmortem. A lightly structured Markdown record is sufficient; a helper/template is justified only if it reduces observed friction. Agent-observed incidents can also be captured with explicit attribution if that capability is selected; expanding the current user-triggered skill is a semantic change. |
| Search for a prior attempt | Searchable problem wording, synonyms, affected action and links to current disposition. Prose supports meaning; small machine fields can filter scope/status. Neither tags nor embeddings guarantee relevance. Return bounded matches with exclusions/coverage uncertainty instead of claiming “never tried” from absence. |
| Update, split or merge knowledge | One current interpretation per retained question, linked source evidence and dated outcome. Update in place or explicitly supersede; retain aliases when useful for discovery. Git supports recoverability, but agents should not routinely reconstruct state from full history. |
| Delete or retire | Remove redundant guidance and projections/callers when authorized; mark retained historical claims withdrawn where evidence remains useful. Literal deletion/retention-sensitive material needs its owner's disposition. Append-only host logs are particularly awkward for erasure and consumer migration. |
| Verify or re-reason | Name the exact claim, changed premise, scope, owner and observable next check. Recheck the implication, not just timestamps. Unknown outcomes and conflicting observations remain usable knowledge. No canonical remedy or root cause is required. |
| Jörn compares choices | A polished view can foreground disputed claims, costs, owners and gaps. It should render/link the same maintained sources and expose freshness, rather than become another edited truth store. The existing dashboard proves this is feasible as a projection, not that it improves judgments. |

**Provenance contract proposed for any selected mechanism:** keep a user report,
agent observation, inference, hypothesis, decision and effectiveness claim
distinct when that distinction changes action. “Jörn reports X” establishes the
report; a current instruction settles authorized behavior, not empirical truth
about X's cause. An agent's confident synthesis supplies no new evidence by
itself. Record an attempt's actual exposure/version/scope and result; an approved
plan, passing syntax check, isolated success and reproducible effectiveness are
different claims. Prefer separate disposition dimensions over one `fixed` flag:
problem applicability, proposed/chosen intervention, activation, tested outcome,
and remaining uncertainty can differ.

Corrections should identify what they replace and why. Current user choices
govern authorization; empirical conflicts need source comparison or an explicit
unresolved disposition. Withdrawals propagate to derivative claims without
silently erasing source reports. Wider promotion requires evidence that the
consumer and truth span that scope; this pass finds no basis to promote
project-local knowledge into host-wide guidance.

Host JSONL can cheaply retain events **if capture and semantics exist**, but it
does not itself supply intent, relevance, trustworthy interpretation, portable
project availability or deletion ergonomics. No raw rollout crawl is warranted
here. Before using host logs, identify the missing decision-relevant fact and
inspect only its bounded authorized source. A repo record, a template and an
incident skill are potential supports to these actions, not substitutes for the
consumer/maintenance mechanism.

## Costs and unowned prerequisites

All costs below are qualitative and unmeasured. This table grants no allowance.

| Candidate | Parent context | Worker effort / latency | Jörn effort | Maintenance and side effects |
| --- | --- | --- | --- | --- |
| A: decision pull | Short standing trigger; relevant reads grow context when invoked | Consumer search, disambiguation and source checking; extra round trips | Review the proposed trigger and ambiguous dispositions; should not require reminders | Lowest new delivery machinery; missed triggers, duplicate notes, broad retrieval and needless gates remain risks |
| B: entry delivery | Relevant context supplied directly; preparation can burden the assigning parent | Selection/writing plus receipt check; less receiver search if selection is good | Assess consequential packet choices, not rebuild their history | Active-copy tracking, selection errors and coordinator bottleneck; peer integration also needs an owner |
| C: change-driven recheck | Compact change reports; unresolved dependency reviews can accumulate | Maintain links, detect change, reason about implications, contact active consumers | Judge disputed changes/acceptable residual gaps when needed | Dependency upkeep, false alarms, missed changes and ownerless stale states; deterministic helpers need maintenance |

The coordinator's graph should expose these **unowned prerequisite questions**
before any substantial execution package is selected, using candidate links
rather than pretending they are necessities of project completion:

- Which actual decision/problem is costly enough to improve, and what observable
  result or residual friction would make further investment worthwhile? Jörn's
  acceptable tradeoff and a meaningful baseline remain unknown.
- Which previously tried interventions and outcomes are relevant to that
  decision? Use a bounded source recovery, not a new exhaustive archive audit;
  distinguish “not found in this pass” from “never tried.”
- Who prepares the selected fixture, checks observable outputs, integrates
  corrections, and owns maintenance after the probe? These are different
  responsibilities; none is assigned by this report.
- Can the chosen fresh/active receiving surface actually receive the candidate
  knowledge and a later correction? Check the selected runtime/client and route,
  including independent-thread transport if needed; avoid changing host settings
  merely to assume an activation mechanism.
- If machine targeting is selected, what minimal dependency/status semantics
  are sufficient, and what coverage remains unknown? Schema design follows that
  action; it is not a prerequisite for cheap intake.

Stop after the selected information-producing probe and compare its actual
decisions and costs. A null or negative result can justify retaining current
routing, narrowing the mechanism, or ending this line of investment. Structural
validity and a returned report close only this planning assignment.
