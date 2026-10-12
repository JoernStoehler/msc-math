# Cleanup workflow design — 10 October 2026

Owner: current `/root` in worktree `f7e0`. Baseline: `844fc5be`, detached HEAD.
This is design work, not authorization to resume historical thesis programmes.

## Outcome and boundaries

G1 — uphill, high confidence in attributed requirement, updated 15:56 UTC:
design an overnight cleanup workflow for a cold-start `gpt-6.1-sol` medium root,
cheaper Sol/Luna workers, this worktree and a handover prompt. User permits
design-phase branch edits. First 15-minute familiarization/report window starts
15:46 UTC; first report due about 16:01 UTC. User then intends to provide a
60-minute design window, with a second decision-quality report at about 17:01
UTC if it starts immediately. Overnight execution is a subsequent session.

G2 — uphill, high confidence: research usability and an agent-facing thesis may
be the overnight deliverable; human-writing capability may remain unresolved.
However the user's explicit grading criterion is whether tomorrow this enables
a credible workflow completing the human thesis within seven days, roughly
28 hours of Jörn's involvement, and $10k API-equivalent token capacity (weekly
quota/resets). These are future planning constraints, not current spending
authorization or measured available balances. Include human planning, review,
correction, source explanation and authoring in effort accounting. Do not
silently substitute cosmetic cleanup or harmless bounded activity for G2.

G3 — uphill, high confidence: support multiple sessions and meaningful ongoing
Jörn participation. User does not expect this design session to establish that
agents can design the human-writing workflow without his help.

C1 — downhill: preserve scientific claims, provenance, retained evidence and
existing user choices. A passing build or local reviewer is not thesis PASS.
C2 — uphill: test whether a feedback mechanism can recognize wrong-goal and
incomplete results before relying on it; its own sufficiency is a question.
C3 — uphill: gather accessible inputs before autonomous execution, distinguish
unsearched, unmaterialized, unavailable and human-only information.
C4 — uphill: operational knowledge needs uncertainty detection, current source
lookup and actual outcome checks; stored Codex advice can become stale.
C5 — uphill: messages must expose actionable uniquely identified requests;
noise that hides them, missed delivery, and abandoned parent objectives are
workflow failures. Do not invent questions once the work is actually complete.

## Current user inputs and unresolved questions

- Q1 resolved then refined: research usability/agent thesis improvement can be
  overnight success without solving human writing, conditional on G2.
- Q2 resolved at 15:58 UTC: yes. Make the entire repo readable for agents.
  User expects a curated, somewhat linear agent-facing thesis with contents,
  searchability and cross-references, linking down to source code, datasets,
  figures and notes, with links back up. A file-tree solution is acceptable
  if it actually supplies this explanatory function; Rust annotations alone
  are not presumed adequate. Progressive disclosure is the selected intent,
  not a mandate for a particular filename or physical tree.
- Q3 resolved at 15:59 UTC: Jörn can stay at the keyboard if a concrete trial
  judgment sounds valuable. Supply its purpose, exact object and reading-time
  estimate; do not consume attention merely to obtain another approval label.
  Historical feedback will be used first.
- User reports over 30 failed workflow attempts across two months, irrespective
  of agent cost; best known quality/speed workflow is Jörn writing everything.
  Treat this as adverse prior evidence, not a quantified failure probability.
- Human-facing writing, research friction, operational Codex understanding and
  communication failures are coupled but distinct; testing one does not certify
  all others.

## Candidate interventions and dependencies

P1 — uphill: standard source/knowledge cleanup with actual downstream consumer
tests; depends on G2 and observable research tasks. Compare physical reorg,
semantic source repair, and a coherent explanation before selecting a layout.
P2 — uphill: agent-facing mathematical synthesis; depends on source/evidence
coverage and consumer reconstruction/continuation tests, not just link checks.
P3 — uphill: calibrated writing support for a human/agent process; depends on
P1/P2 where information is the bottleneck. Do not assume information repair
solves composition or recognition of poor writing.
P4 — uphill: cold-start execution and recovery; depends on explicit pending
decisions, accessible inputs, operational checks, integration capacity and
multi-session handover tests.

## Bounded discovery assignments

Three parallel read-only Astra investigations completed: `goal_feedback`
(prior evidence and feedback), `repo_probe` (representative tasks and runtime
knowledge), `adversarial_design` (failure modes and evidence burden). Each had
a 4–6 minute allowance, no delegation, root integration ownership.
`cold_reader`: fresh Sol 6.1 medium, <=4 minutes, read-only current-repo research
question on the pentagon result. Tests baseline discoverability/explanation;
does not validate overnight reliability or writing quality.

## Decision log and pre-registered expectations

15:56 UTC: Q1's answer is recorded above. Q2 anticipated agreement 0.8, Q3
availability unknown (subjective 0.5); these are predictions, not acceptance.
The first root response wrongly serialized discovery and postponed clarification.
User corrected both parallelism and the interpretation of cleanup. Root now
retains original completion objective while separating its bottlenecks.
Before reading cold_reader output: predict it can recover the basic analytic
proof but may miss already-existing symmetric-partner/general odd-n extensions
(0.6); one success/failure only informs task choice, not an aggregate rate.


## Phase-1 extension and human review surface (16:18 UTC)

User extended familiarization by15minutes at16:06UTC; current deadline16:21UTC.
The 60-minute design phase has not started. Q4 was deferred, not rejected as a
permanent direction. Q5 identifies the first report's principal failure: no
clear problem/solution-space decomposition for human evaluation. A revisable
standalone HTML review draft now owns the human-facing analysis; this structure
file remains agent-facing. `review.html` separates required outcomes (AND),
alternative interventions, selection arguments, evidence and feasibility.

Q6 resolved: substantial human authoring is acceptable, but Jörn estimates
writing everything himself at four weeks. Do not call that a seven-day fallback
or infer a working-hour count from the calendar estimate.
Q7 resolved: repeated rewording, detailed reconstruction of repository content
and unclear phrases cost time. Understanding an already clear proof and making
it more relevant/readable for a less familiar reader is not much of a problem.
No measured time fractions supplied.
Q8 resolved: seven-day endpoint is ready to send to Kai, not advisor turnaround.
Q9 prepared: review missing/wrong problem branches, alternatives and dependencies
in D2/D3/D4/D6/D7 of the HTML. Q4 remains the later design-phase decision.

Three independent follow-ups examined standard workflow families, current
manuscript obligations and defects in the first selection argument. They found
that many source repairs already exist; human acceptance is unknown, not a
known whole-candidate FAIL. A harder DS optimizer interpretation case is a
candidate transfer task. Root must not use the convenient pentagon side-result
as a proxy for all eleven required human-thesis areas or the entire research
corpus. The design must explain actual acceleration toward the seven-day
outcome, not redefine success as merely a credible plan.

Human review file is served through review-files at http://100.70.188.20:58037
(eight-hour session). Main-page screenshot inspected; HTTP bytes match the
source file. This verifies presentation and serving only, not the analysis.


## Revised checkpoint: inventory and early cleanup (10 October, 16:31–17:07 UTC)

Jörn found the proposal too ambitious to evaluate and selected concrete early
cleanup: knowledge inventory, simplification and a first curated agent guide.
Whole-crate refactoring, cross-crate API movement and note conventions remain
in scope, without a selected physical layout. Sol/Luna workers surveyed
foundations, product/local-max results, empirical work, software and guidance;
a separate physical census covers tracked paths. Root reconciled overlaps,
assembled the user view and integrated bounded fixes. See `docs/inventory/`.

The current discussion surface is the inventory's `index.html`, shared under
CURRENT-repository-inventory.html. Q9 and the prior decision draft are
superseded for this checkpoint. Q10 asks result organization versus code-first
priority; Q11 asks continued use of the provisional goal map; Q12 asks useful
result units/grouping; Q13 offers a concrete explanation sample to locate
remaining human reconstruction cost. These are not blanket approval questions.
The overnight handover and its restart/evaluator trials remain unfinished.

Browser runtime discovery returned no available browser during the inventory
page check. Serving, HTML anchors, data and JavaScript can be checked; no new
visual/UI acceptance is claimed. The earlier phase-1 page's screenshot check
must not be transferred to the inventory page.


## Parallel revision, 10 October 18:16 UTC

Jörn corrected Q10's either/or premise: geometry consolidation, foundational
mathematics, dataset simplification and tests/benchmarks can proceed in parallel.
Root removed inventory completion as a global prerequisite. Three bounded
read-only follow-ups supplied concrete first units, integrated in
`docs/inventory/parallel-plan.md` and the user view. The guide/inventory is
maintained alongside those streams. Shared APIs, schemas and assumptions create
local dependencies; unrelated work continues. Q11–Q13 remain optional feedback.
No implementation or overnight execution occurred in this revision.


## 12 October: layout-pattern comparison reopened (06:16 UTC)

Jörn asks for the actual pattern space, particularly nesting and lifecycle
tradeoffs between reading, incremental work and deciding what deserves retention.
The first report compared explanatory architectures but did not complete physical
layout/convention exploration. Root then prematurely recommended a domain hybrid
and subsequently topic ownership. Neither recommendation selects a migration.

G4 — uphill: give a concrete catalogue of materially distinct packet structures,
lifecycle/versioning/retention choices, and their combinations. Distinguish
previously considered candidates from alternatives added now. This is a design
comparison, not a claim that every option has been tested or that time proves
completeness. No human choice is needed to enumerate the alternatives.

C6 — user steering: topic packets can hold code, definitions/proofs, exploratory
notes, producer-owned datasets and figures; artifact genre does not require a
top-level ownership split. Keep packet location, internal decomposition, reading
order, dependency relationships and lifecycle distinct in the comparison.

D8 — open: how much physical nesting should express topic/subproblem membership
versus independently addressable packets and explicit links.
D9 — open: inexpensive incremental records versus maintained explanatory accounts;
when and what to promote, retire, or retain for evidence.
D10 — open: stable locations/status metadata versus lifecycle directories and
immutable run/version packets; preserve provenance and consumers when moving.

Bounded read-only contributions: adversarial_design owns missing nesting patterns;
agent_guide owns lifecycle/retention patterns. Each has three minutes, no edits or
descendants; root integrates, maintains this map and delivers the catalogue.
No migration, adoption of headers, or resumed thesis work is authorized here.

Catalogue axes now covered: flat/topic/method/question/attempt/result/obligation
nesting; shared foundations and explicit graph relations; pipeline and run
structure; notebook/living/literate/assembled/frozen reading accounts; stable
paths versus status lanes; Git/snapshot/workspace/external-artifact lifecycle;
contain/link/pin/bundle dependency choices; granularity evolution and multiple
retention outcomes. These are not a measured architecture ranking. The
substantive comparison is retained in
`meta-topics/repository-organization/organization-design.md`; disposable exact review packets
were moved to the temporary recovery collection documented in
`meta-topics/agent-workflow/message-review-retention.md`.


## 12 October 06:27 UTC: analytical ranking withdrawn and redone

User clarified the repo's main consumer is Codex across thousands of threads
and subagents. Root overgeneralized this into mandatory arbitrary-entry,
handoff, concurrency and explanatory-layer requirements, without establishing
their importance or relative costs. User requested a better report if merely
communication failed, otherwise renewed analysis. Root selected renewed analysis.
The earlier priority ordering is not retained as a justified recommendation.

G5 — uphill: rank material unresolved design choices by expected consequence,
uncertainty/difficulty and Jörn's marginal knowledge. Separate established
consumer/goal from candidate mechanisms. Actual per-packet uses between
material changes, repeated reconstruction/error consequences and maintenance
cost matter; nominal thread count does not establish frequency, concurrency,
entry modes or desired duplication. Re-read primary archival-consumption
results and writing-pilot reconstruction logs. Adversarial_design independently
challenges the ranking in a two-minute read-only assignment. No new consumer
trial, migration or universal conventions selected.

Revised analysis completed at06:31UTC. Primary evidence supports specific
discovery and dated-account problems, not an established folder remedy.
Priorities: scientific boundaries, current meaning/lifecycle, net explanation
value, retention. Jörn's marginal judgment is highest on disputed scientific
relationships and status; agent trials/inspection own retrieval and maintenance
cost assessment. A separate assembled reviewer approved the exact report;
the disposable review packet's removal is recorded in
`meta-topics/agent-workflow/message-review-retention.md`. This supersedes
the earlier unsupported priority ordering; architecture remains unselected.


## 12 October 06:59 UTC: retained operational comparison selected

User corrected the analysis again: concrete structure/search/workflow/resource
comparisons are required, not only scientific questions. Metadata-aware lookup
can change the value of file organization; broad relevance discovery can be
a compression task assigned to a subagent; topic organization can be rewritten
when expansion-heavy work becomes reading-heavy. User suggests a matrix across
agent tasks, concerns, tools/behavior, resources and available context, including
critical-path time versus triage noise. These are candidate tradeoffs, not
measured model claims.

User also challenged root-heavy repeated brainstorming. Root has dispatched
two fresh-context bounded workers with only a compact project feature brief,
not earlier candidate reports. Fresh structure comparison and fresh workflow
comparison each own one scratch analysis and have four minutes, no descendants.
Root owns factual checks, comparison/integration and delivery. User requests a
retained agent-consultable report, and a short human check-in limited to the
most consequential feedback. Earlier undelivered message candidates are drafts.
No migration, live behavioral convention or retrieval-tool implementation is
authorized by this report task.

Final retained reference: `meta-topics/repository-organization/organization-design.md`, linked
from inventory. Two fresh analyses and separate fresh synthesis completed; root
checked supplied features, restored critical-path/net-benefit distinctions, and
obtained independent assembled review. Prior rankings are superseded for this
task. One optional within-authorized-resource latency/spend preference is the
human feedback focus; no required input, migration or instruction activation.
