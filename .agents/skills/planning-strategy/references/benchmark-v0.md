# Planning Strategy Benchmark v0

Use this reference when updating or reviewing `$planning-strategy`. Do not load
it for ordinary task use.

Purpose: test whether the skill catches planning failures before costly
execution while preserving successful bounded workflows. Rows are paraphrased
from local Codex rollout logs and subagent extractions from July 2026. They are
not transcript excerpts.

Categories:

- `trigger_failure`: the skill should trigger and prevent a known or likely
  planning failure.
- `trigger_success_guard`: the skill may trigger; expected behavior preserves a
  successful route choice.
- `nontrigger_success_guard`: the skill should not trigger; expected behavior
  is fast direct execution/review.
- `scoping_boundary`: use `$scoping`, not `$planning-strategy`.

## Rows

### PB-01: Finite Proof Bridge vs More Computation

- category: `trigger_success_guard`
- expected trigger: yes
- expected placement: scratch or durable proof note, not chat by default
- assertion: before more computation, compare the calculation route with the
  finite open-condition proof bridge and choose the route that addresses the
  actual proof gap.
- no-regression guard: do not bury a cheap structural proof check inside a long
  proof/calculation search.

### PB-02: Bounded Packet Production

- category: `nontrigger_success_guard`
- expected trigger: no, unless route choice reopens
- assertion: when the user already assigned a bounded packet and worktree, do
  the packet and validate it; do not broaden into thesis reorganization.
- no-regression guard: the planning skill must not make bounded work slower or
  more chatty.

### PB-03: Second-Pass Repaired Review

- category: `nontrigger_success_guard`
- expected trigger: no, unless the repaired surface creates a new route choice
- assertion: re-review the repaired sections and stop when the targeted defects
  are fixed; do not reopen the entire packet.
- no-regression guard: preserve narrow second-pass review.

### PB-04: Quarantined Workflow Test

- category: `trigger_success_guard`
- expected trigger: yes if the workflow test could write to the repo or consume
  substantial cleanup; otherwise no
- assertion: define quarantine boundary before execution and finish with zero
  repo-tracked diffs plus an artifact trail.
- no-regression guard: do not turn a workflow test into unbounded repo edits.

### PB-05: Workflow Design Chat Loop

- category: `trigger_failure`
- expected trigger: yes
- expected placement: scratch plus artifact; chat only for cruxes or final
  review request
- assertion: for broad workflow design, separate evidence collection,
  candidate workflows, and evaluation artifact before writing the proposal.
- failure to prevent: chat-side repair loops and partial summaries replacing a
  stable design/eval artifact.

### PB-06: External Evidence Scan Before Workflow Proposal

- category: `trigger_success_guard`
- expected trigger: yes
- assertion: keep evidence scan separate from workflow proposal; compare
  evidence maturity before designing the local process.
- no-regression guard: do not overclaim end-to-end autonomous review from thin
  evidence.

### PB-07: Review Failure Modes Before Autonomy Claims

- category: `trigger_success_guard`
- expected trigger: yes
- assertion: reject the assumed synergy "AI reviewer replaces human review";
  decompose into narrow checks with human accountability.
- no-regression guard: preserve decomposed-task framing.

### PB-08: Sentence-Level Prose Critique

- category: `nontrigger_success_guard`
- expected trigger: no
- assertion: answer the local editorial question directly.
- no-regression guard: do not add planning scaffolding to one-sentence prose
  review.

### PB-09: Fully Specified Thesis Execution Prompt

- category: `nontrigger_success_guard`
- expected trigger: no
- assertion: when surface, branch, and deliverable are fixed, follow the prompt
  and maintain file hygiene; do not invent a route-comparison plan.
- no-regression guard: preserve direct compliance on fully specified tasks.

### PB-10: Proposer Packet Keep/Trim/Park Decision

- category: `trigger_success_guard`
- expected trigger: yes
- assertion: compare keep/trim/park routes and distinguish durable evidence
  from regenerate-on-demand caches before recommending artifact handling.
- no-regression guard: avoid shipping unnecessary bulk artifacts.

### PB-11: Next-Step Launch/Sharpen/Park Decision

- category: `trigger_success_guard`
- expected trigger: yes
- assertion: compare launch, sharpen, and park, including opportunity cost and
  missing evidence, before choosing a next action.
- no-regression guard: preserve the value/cost comparison, not just the chosen
  route.

### PB-12: Too-Easy Review Benchmark

- category: `trigger_failure`
- expected trigger: yes
- assertion: detect that the benchmark case is too easy before treating it as
  strong evidence; ask for or construct a stricter case.
- failure to prevent: reporting a weak discriminator as if it validated the
  workflow.

### PB-13: First Thesis Slice Choice

- category: `scoping_boundary`
- expected trigger: no; use `$scoping`
- assertion: choosing the objective/slice is not implementation planning.
- no-regression guard: do not collapse target selection into
  `$planning-strategy`.

### PB-14: Planning-Harness Design After Repeated Failures

- category: `trigger_failure`
- expected trigger: yes
- assertion: before more skill patching, build/evaluate a benchmark from local
  traces and external practice.
- failure to prevent: continuing line edits without a feedback signal.

### PB-15: Planning-Skill Overlap Review

- category: `trigger_failure`
- expected trigger: yes
- assertion: reject trigger metadata that overlaps `$scoping` or depends on
  vague hidden estimates.
- failure to prevent: adding a second bad planning surface.

### PB-16: Narrow Merge-Readiness Review

- category: `nontrigger_success_guard`
- expected trigger: no
- assertion: perform the bounded source-fidelity/merge-readiness review
  directly.
- no-regression guard: do not turn every review into planning strategy.

## Current Lessons

- Trigger failures need pre-execution detection and route comparison.
- Success rows are not automatically trigger rows; many are no-regression guards
  for bounded work that should stay fast.
- `$scoping` owns objective and slice choice. `$planning-strategy` starts after
  the objective is known.
- Placement matters: scratch for ordinary autonomous work, chat only for a
  requested visible plan or a Jörn-only crux, and durable charters for long or
  resumable loops.
- Hard boundaries such as worktree/main constraints, quarantine paths, artifact
  ownership, and no-edit surfaces must be treated as success/failure criteria
  when present.
