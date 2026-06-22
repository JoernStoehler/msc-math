# Trustworthy Computation Workflow Blocks

Use this reference when planning Rust work where exact/f64 implementations,
certified predicates, numerical error bounds, fallback, profiling, and
proof/code correspondence interact.

This is a menu of familiar workflow blocks GPT-5.5 already knows. It is not a
required order, not a complete method, and not a recommendation that every block
belongs in a given task. For a concrete task, pick the blocks whose value
exceeds their one-time, usage, maintenance, review, and runtime costs.

Use this as a poor man's brainstorm seed, not as sufficient search or
assessment. The list is medium-effort babble over standard workflow blocks,
low-effort pruning, and only repo-contextual rather than task-contextual. For
high-value work, gather the task context first, add task-specific and custom
strategies when returns justify it, and spend real effort assessing value, cost,
interaction effects, and combinations.

- **Mathematical specification first:** state the optimization problem,
  quantities, predicates, and success condition in mathematical terms before
  optimizing code structure around them.
- **Executable specification:** write a direct executable version of the
  intended behavior. In this repo, exact arithmetic often plays this role for
  f64 audits.
- **Reference implementation:** maintain a simple or trusted implementation
  that prioritizes semantic correctness over speed and can be compared against
  faster paths.
- **Specification by example:** choose concrete examples that express important
  cases, edge cases, and regressions. These examples are not proof, but they
  make the intended behavior legible.
- **Contract-first API design:** define preconditions, postconditions,
  non-success cases, and theorem-backed guarantees at API boundaries before
  callers depend on the behavior.
- **Design by contract:** keep those contracts visible near functions and check
  the executable parts with assertions, result variants, or tests.
- **Self-documenting code:** use names, types, and small functions so ordinary
  code communicates the domain operation without requiring a separate prose
  explanation.
- **Literate programming:** organize code and explanation so the reader can
  follow the reasoning behind the computation. In this repo, use this idea
  lightly: proof-sized reasoning usually belongs in `formal/` or research notes,
  with code comments naming the local proposition and reference.
- **Comment the why, not the what:** use comments for proof obligations,
  invariants, choices among plausible designs, and non-obvious numerical
  assumptions. Avoid comments that restate the syntax.
- **Proof/code correspondence check:** compare each theorem precondition and
  conclusion against the fields, branches, and constructors in the code.
- **Proof obligations list:** maintain a small list of claims the code relies
  on but has not yet proved, measured, or delegated to exact fallback.
- **Traceability matrix:** map math claims, code functions, test cases, data
  fields, and proof labels to each other when several surfaces must stay aligned.
- **Assumption audit:** list assumptions that are active in the implementation
  and classify them as validated, theorem-backed, measured, heuristic, or open.
- **Invariants checklist:** identify values that must remain finite,
  normalized, dimension-compatible, sorted, bounded, or otherwise constrained.
- **Failure-mode analysis:** name the ways the algorithm can fail, be
  inapplicable, return `Indet`, or require fallback before deciding how to encode
  those outcomes.
- **Separation of exploration and production:** allow wide diagnostic output
  while searching, then extract a narrow certifying production path after the
  policy and proof obligations are known.
- **Instrumentation-first debugging:** add structured fields that expose the
  intermediate quantities needed to distinguish hypotheses before changing the
  algorithm repeatedly.
- **Measure before optimizing:** use profiling or timing data before treating
  performance as the bottleneck.
- **Profiling-driven optimization:** optimize the measured hotspot, then rerun
  the profile to check whether the bottleneck moved or the optimization mattered.
- **Hypothesis-driven development:** state the candidate explanation or theorem,
  collect the fields needed to test it, then decide whether the result supports,
  falsifies, or fails to distinguish it.
- **Counterexample-guided refinement:** use failing rows to refine the theorem,
  instrumentation, solver, or fallback policy. Preserve important
  counterexamples as tests or data.
- **Differential testing loop:** repeatedly compare f64 and exact/reference
  paths on the same inputs while changing the f64 algorithm or certificate.
- **Property-based testing loop:** generate many inputs to search for violations
  of general properties or soundness claims.
- **Metamorphic testing loop:** transform inputs in ways that should preserve or
  predictably transform outputs, then check the relation.
- **Golden test capture:** once a failure or important edge case is understood,
  pin it with a regression test or fixture.
- **Characterization testing:** record current behavior before a risky refactor
  so accidental behavior changes become visible.
- **A/B implementation comparison:** keep two candidate algorithms available
  long enough to compare correctness, applicability, performance, and code
  complexity on the same cases.
- **Shadow mode:** run a new policy beside the old one without making it
  authoritative yet, then compare outputs and failures.
- **Table-driven policy evaluation:** evaluate many candidate policies over the
  same rows and report soundness, applicability, capacity effect, and fallback
  cost separately.
- **Ablation study:** remove or disable one component at a time to see which
  part of a method accounts for the observed behavior.
- **Bisect the computation:** compare intermediate exact and f64 quantities to
  locate where an error or branch divergence first appears.
- **Local spike:** build a small throwaway prototype to learn an API,
  theorem-candidate shape, performance bound, or diagnostic field set.
- **Parallel spikes:** try several independent designs in separate worktrees or
  subagents when alternatives are cheap and comparison is more informative than
  discussion.
- **Design review:** inspect an API or architecture for whether it expresses the
  actual domain contracts and avoids unnecessary abstraction.
- **Code review:** inspect the diff for bugs, regressions, missing tests, and
  contract violations.
- **Proof review:** inspect theorem statements, hypotheses, edge cases, and
  proof/code correspondence.
- **Data review:** inspect whether the generated rows, sampling, summaries, and
  provenance answer the intended empirical question.
- **Independent reviewer subagent:** ask a non-fork subagent to review named
  files for a named quality, and to list hotspots or concerns worth checking.
  This is useful when the review surface is bounded and the output is evidence,
  not an authoritative decision.
- **Decision subagent:** ask a subagent to choose among alternatives only when
  the required context, criteria, and evidence can be included compactly. This
  is often an anti-pattern for project-level decisions because missing context
  can dominate the answer.
- **Red-team review:** ask a reviewer to look specifically for false claims,
  unsound implications, hidden assumptions, and cases where the stated success
  criterion can be gamed.
- **Pre-mortem:** before committing to a design, ask how it could fail or become
  misleading even if the local tests pass.
- **Post-mortem:** after a failure, identify the missing evidence, confused
  distinction, or bad feedback loop that allowed it. Convert only recurring or
  expensive lessons into durable guidance.
- **Progressive disclosure:** keep always-loaded instructions short and move
  detailed inventories or checklists into references that are read only when
  relevant.
- **README-driven development:** use a short consumer-facing README or API note
  to force clarity about how a future caller should use a settled surface.
- **Decision record:** record an important accepted/rejected approach, its
  evidence, and its expected revisit condition when the decision affects future
  agents.
- **Stop-condition design:** define what evidence is enough to stop a loop,
  what evidence falsifies the current path, and what unresolved risk remains.
- **Fallback policy design:** decide which unresolved cases can be routed to a
  slower or exact path, and measure the correctness and performance effect of
  doing so.
- **Interface minimization:** after exploration, shrink the production API to
  the values, certificates, and outcomes callers actually need.
- **Garbage-collection pass:** remove or demote stale diagnostic surfaces,
  generated reports, obsolete policy variants, and misleading documentation
  after the durable result is known.
