# Sys-Datascience Process Learnings

Use: this file records incidents, Jörn process critiques, and workflow updates
that should influence future sys-datascience agent behavior. It is optimized
for surface scouts, topic owners, and prompt writers. Add entries when an
incident reveals a reusable failure mode or prompt/workflow improvement.

## 2026-07-04: End-To-End Workflow-Test Chain Worked

Observation: a fresh global scout selected `generated-candidate-proposers`, a
fresh topic lead produced a prioritized packet batch, a fresh executor ran the
10k promising-scalars workflow-test under `/tmp`, and a fresh reviewer accepted
the packet. This tested the scout -> topic lead -> packet executor -> reviewer
chain without treating the run as thesis evidence.

Update: the proposer topic now includes a ready 10k workflow-test prompt shape,
an explicit scalar-union versus conjunction-rule boundary, a 100k launch
checklist, and defaults for sharpening the two-feature selected-tail rescue.
The global ledger now distinguishes next spawn type from topic status.

Future prompt implication: scalable execution improves when packet prompts
store the prompt itself, state exact artifact locations, give numeric sanity
thresholds for "unexpectedly large", and say which generated summaries are
enough for interpretation. Without those details, executors can still succeed,
but reviewers must spend extra effort reconstructing intended behavior.

## 2026-07-04: Do Not Drain Launch Boards Without A Named Milestone

Observation: after promoting useful scalar-proposer and ridge-mechanism
artifacts, the parent session kept turning plausible next launch-board items
into active work. The individual outputs were often useful, but the session
lost a user-legible current objective. Jörn had to interrupt because he no
longer knew which subgoal the session was serving.

Update: `next-session-candidates.md` is a spawn/rescope/stop board, not an
execution queue. A session may use it to choose work, but before doing more
than a short triage it must state the current milestone in ordinary thesis
terms and close or park opened surfaces when the milestone changes. Scratch
outputs should be promoted, parked with a durable rediscovery hook, or
intentionally abandoned; do not leave `/tmp` as live state that the main
session must remember.

Future prompt implication: surface scouts and research-map stewards should
distinguish workflow-design tests from real research packets. If a packet was
selected for workflow value, mark `Workflow-test: yes`; if it becomes a real
research artifact, review/promote it through the packet lifecycle before using
it to update research beliefs. Do not keep launching follow-up packets merely
because the previous packet produced a plausible next action.

## 2026-07-03: Dry Runs Need Status And Launch Scaffolding

Observation: workflow-test subagents could understand the high-level process,
but repeatedly had to infer which topic files were active, parked, or ready for
ownership. A packet-executor dry run treated an under-specified packet idea as
the first available task and had to repair the scope itself. A reviewer dry run
had to infer `git show` and `git ls-tree` entry points for parked packet
artifacts.

Update: topic files now use owner-readiness/status, adjacent-topic pointers,
and ready-packet versus needs-sharpening sections where useful.
`next-session-candidates.md` carries the compact current session-decision
board; rediscovery-only ideas belong in `parked-and-rejected.md`.
Parked-packet review guidance now includes read-only commit inspection
commands.

Future prompt implication: when testing materials, it is valid to choose work
mainly for workflow information rather than research value. Mark that as a
workflow-test packet so its process findings can be kept while its research
outputs are discarded or redone under better materials.

## 2026-07-03: Belief Files Are For Use, Not Gating

Observation: the first version of `research-ledger.md` described
"accepted boundary beliefs." Jörn objected that beliefs should be stored with
pointers to why they are believed, not hidden behind an acceptance gate.

Update: the global ledger is now framed as recent belief state with
evidence/update traces. Topic files are working memory and cross-session
research communication. A belief can be useful while uncertain, speculative, or
tainted, as long as the uncertainty and source trace are visible.

Future prompt implication: do not ask agents to record only accepted results.
Ask them to record current beliefs, doubts, live hypotheses, evidence traces,
and what downstream use is allowed.

## 2026-07-03: File Design Should Follow Downstream Use

Observation: Jörn pointed out that agents, not Jörn, are the primary readers of
the topic files. A topic lead will read its own file after compaction or deep
dives to recover context. Other topic leads will read it to borrow hypotheses,
architectures, feature ideas, and evidence traces.

Update: coordination guidance now says each owned file should state what use it
is optimized for and how it is maintained. Formats are not fixed. Markdown is
only the current default because agents read and write it well.

Future prompt implication: when spawning a topic owner, ask it to maintain a
surface that fits its expected readers. If it uses an append-only log or split
files, it should say why and what cost tradeoff it chose.

## 2026-07-03: Branch Names Affect Spawn Quality

Observation: `sys-ds-random-method-integration` became misleading after the
scope broadened beyond random-method integration. Jörn flagged that the name
would steer fresh sessions toward too narrow a frame.

Update: the broad integration worktree was renamed to
`thesis-datascience-integration` at the time. That branch is now historical
scratch, not the live coordination surface.

Future prompt implication: branch, folder, and role names are prompt material.
Rename stale surfaces when they mislead new sessions.

## 2026-07-02: Partial Packets From Weak Scope

Observation: earlier subagents delivered useful but partial packets when the
prompt did not sufficiently encode scale, definition of done, or how the packet
would be used. Example: extreme proposer work initially did not reach the
intended 1e-4-style scale/context without follow-up.

Update: packet prompts should include the local question, downstream use,
scale/cost expectations, source files, artifacts expected, stopping condition,
and review standard. "Useful exploration" is not enough if the packet must
support a specific future decision.

Future prompt implication: be ambitious enough in one packet when context is
clear, especially if follow-up coordination is more expensive than agent time.

## 2026-07-02: Mixed Packets Need Taint Separation

Observation: the tail-hardening packet had useful zero-positive/EVT artifacts,
but review found its HKO-distance/flank interpretation compared
volume-normalized prepared duals to an unnormalized HKO inradius. The right
response was not "discard all tail work" and not "merge as-is"; it was to park
the packet with a blocker note and preserve usable pieces.

Update: coordination surfaces use `mixed/tainted` language. Future agents should
separate source-backed facts, model-sensitive inferences, and known-bad claims.

Future prompt implication: reviewers should identify which parts of a packet
remain useful after a defect, and what must be repaired before merging.

## 2026-07-02: Review Catches Integration-Readiness Errors

Observation: feature-cost timing and HKO ridge-area packets were conceptually
useful but initially had integration-readiness issues: untracked core files,
local absolute paths in artifacts, schema/README naming mismatch, and defaults
that mixed full input with smoke output.

Update: before promoting packets, check tracked file status, nonportable paths,
artifact regeneration, README/schema consistency, and whether defaults match
the documented scope.

Future prompt implication: packet reviewer prompts should explicitly ask about
tracked/untracked files, absolute paths, generated artifact portability, and
whether no-argument or documented commands do what their folder names imply.

## 2026-07-02: Smoke Evidence Should Stay Smoke

Observation: the HKO ridge-area packet was useful plumbing and smoke-scale
evidence, but not thesis-strength HKO-local evidence. It was committed on its
own branch and parked rather than merged into the datascience integration.

Update: smoke packets can be valuable, but their status must be clear. Do not
let clean code and reproducible smoke artifacts imply thesis-strength evidence.

Future prompt implication: ask reviewers to distinguish launchability,
plumbing, smoke evidence, exploration-grade evidence, and thesis-grade
evidence.
