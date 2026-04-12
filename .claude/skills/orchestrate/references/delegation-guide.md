# Delegation Guide

Reference for orchestration agents. Read to decide what and how to delegate.

## Session-level model selection (delegated sessions)

When handing a full session to a fresh Claude Code tab (or spawning via Agent()), pick model per *phase* rather than assuming one model fits the whole session.

| Session shape | Plan phase | Impl phase | When |
|---------------|-----------|------------|------|
| **opusplan (default)** | opus | sonnet | Default for delegated sessions. Opus where scoping involves real tradeoffs (math structure, architectural choice, interleaved decisions); sonnet for faithful execution of the landed plan. |
| **pure sonnet** | sonnet | sonnet | When TASKS.md already nails scope + DoD is mechanical: audits, data-pipeline fixes, isolated bug fixes, report generation. Opus plan-phase is overhead-only. |
| **pure opus** | opus | opus | Only when the implementation *itself* requires sustained research taste — drafting novel math, thesis prose, new API design where every step needs judgment. Rare for delegated work. |

**Subagent depth axis** (separate from reasoning token budget — this is about the *complexity of thought the model can hold at one time*, not how many tokens it spends):

- **Haiku** — structural enumeration, grep results, file-exists checks, "what's in this directory", short-file paraphrase, mechanical edits with a pre-specified pattern.
- **Sonnet** — applying written criteria to content, triage with clear inputs, multi-step "search → understand → report", verification-against-data, most content-reading tasks.
- **Opus** — open-ended research taste ("reframe this [Jörn] verification item into an agent-checkable precursor"), math-proof understanding, architectural tradeoffs that require holding multiple constraints simultaneously, "is this math rigorous enough to publish".

## Agent types

| Type | Model | Use for |
|------|-------|---------|
| `general-purpose` | set explicitly | Default. Implementation, writing, debugging. Use `"sonnet"` for mechanical work, `"opus"` for reasoning. |
| `Explore` | sonnet | Codebase search, file discovery, pattern finding. Read-only — cannot edit files. |
| `review-rust` | sonnet | Check .rs files against conventions in `.claude/rules/rust.md`. |
| `review-proof` | opus | Shallow correctness check on math.tex proofs (gaps, unargued claims, quantifier errors). |
| `review-formalization` | opus | Check lemma↔code correspondence in a module. |
| `review-claims` | sonnet | Verify factual claims (numbers, citations, code behavior) against data/code/.bib. |
| `review-thesis` | sonnet | Check thesis .tex file conventions. |
| `review-python` | sonnet | Check .py files against conventions in `.claude/rules/python.md`. |
| `review-figures` | sonnet | Check .py→.tex→.png figure production chain. |
| `session-search` | sonnet | Search session JSONL transcripts for past decisions. |

## Example prompts

### Implement a function (opus, worktree)

```
Implement `volume_derivatives_a()` in /workspaces/msc-math/crates/library/src/derivatives.rs.

It should compute the gradient of polytope volume with respect to dual vertices a_i.
Read the existing `capacity_derivatives_a()` in the same file for the pattern.
Read [lem:vol-derivative] in /workspaces/msc-math/crates/library/src/algorithms/math.tex for the math.

Write the function, add a finite-difference cross-check test in the test module.
Run `cd /workspaces/msc-math/crates/library/ && cargo test --release --lib` and fix any failures.
Report: function signature, test results, any surprises.
```

### Explore codebase (sonnet, no isolation)

```
Find all places in /workspaces/msc-math/crates/ where `catch_unwind` is used.
For each occurrence: file path, line number, what it wraps, and whether there's a comment explaining why.
Report as a table.
```

### Write analyze.py (opus, worktree)

```
Write /workspaces/msc-math/crates/exp-hko-local-maximum/second-order/analyze.py.

Read the data at second-order/curvatures.jsonl to understand the schema.
Read /workspaces/msc-math/crates/figure_config.py for figure styling.
Read the logbook at second-order/logbook.md for what figures are needed.

Produce figures showing curvature along flat directions. Use `from figure_config import setup, FIGSIZE_SINGLE; setup()`.
Run: `cd /workspaces/msc-math/crates/exp-hko-local-maximum/second-order/ && uv run analyze.py`
Report: figures produced, any unexpected findings in the data.
```

### Run pre-merge reviews (parallel, sonnet/opus)

```python
# Spawn all review agents in parallel on changed files
Agent(subagent_type="review-rust", prompt="Review: [file list]", model="sonnet", run_in_background=True)
Agent(subagent_type="review-proof", prompt="Review /path/to/math.tex", model="opus", run_in_background=True)
Agent(subagent_type="review-claims", prompt="Verify claims in /path/to/logbook.md", model="sonnet", run_in_background=True)
# Wait for notifications, then synthesize findings
```

### Write math.tex proof (opus, worktree)

```
Write a proof of [lem:cap-derivative] in /workspaces/msc-math/crates/library/src/algorithms/math.tex.

The lemma states: the derivative of the EHZ capacity with respect to dual vertex a_k is [formula].
Read `capacity_derivatives_a()` in /workspaces/msc-math/crates/library/src/derivatives.rs for what the code computes.
Read existing proofs in the same math.tex for style and notation.

Write the proof. Wrap in \begin{unverified}...\end{unverified}.
Build: `cd /workspaces/msc-math/crates/ && latexmk` — fix any errors.
Report: proof approach (1-2 sentences), any gaps or assumptions you're unsure about.
```

### Debug test failure (opus, worktree)

```
The test `test_projection_solver_sign` in /workspaces/msc-math/crates/library/src/kkt/tests.rs is failing.
Error: "assertion failed: beta[3] > 0.0, got -1.2e-15"

Read the test and projection_solver.rs, diagnose the root cause.
Fix it. Run `cd /workspaces/msc-math/crates/library/ && cargo test --release --lib` to verify.
Report: root cause, what you changed, test results.
```

### Regenerate experiment data (sonnet, no isolation)

```
Regenerate data for the random-sample experiment.
Run: `cd /workspaces/msc-math/crates/ && cargo run -p exp-sys-landscape --bin random-sample --release`
Then: `cd /workspaces/msc-math/crates/exp-sys-landscape/random-sample/ && uv run analyze.py`
Report: records generated, errors, figure files produced.
```

## Patterns

### Parallel independent

Use `run_in_background: true` for all agents. Collect results from notifications.
Example: 3 review agents on 3 independent files.

### Sequential dependent

Agent B needs A's result. Spawn A foreground, read result, construct B's prompt.
Example: explore → implement based on findings.

### Fan-out then synthesize

Spawn N parallel agents. Collect all results. Synthesize yourself — don't delegate synthesis, it requires judgment.
Example: 5 review agents → collect findings → filter/prioritize → report to Jörn.

### Retry on failure

Read why it failed. Write a better prompt with more context or tighter constraints. Don't resend the same prompt.

## Example decomposition: library function

Task: "Add `foo()` to the library with tests and math.tex proof."

1. **Explore** (sonnet, bg): find similar functions, patterns
2. **Implement** (opus, worktree): function + test, using explore results
3. **Write proof** (opus, worktree): math.tex entry, referencing implementation
4. **Review-formalization** (opus, bg): proof↔code check
5. **Review-rust** (sonnet, bg): convention check
6. **Fix findings** (opus, worktree): address reviews
7. **Cargo test** (sonnet): full suite

Dependencies: 1 independent → 2 → 3 → {4, 5} parallel → 6 → 7.

## Example decomposition: experiment cycle

Task: "Run foo experiment, analyze results, update logbook."

1. **Explore** (sonnet): read run.rs and logbook.md
2. **Run** (sonnet): execute binary, collect output
3. **Analyze** (opus, worktree): write/update analyze.py, produce figures
4. **Write logbook** (opus, worktree): update with findings
5. **Review-claims** (sonnet, bg): verify logbook against data
6. **Review-figures** (sonnet, bg): figure quality
7. **Fix** (opus, worktree): address reviews

Dependencies: 1 → 2 → 3 → 4 → {5, 6} parallel → 7.

## What NOT to delegate

- Task decomposition (that's the orchestration agent's job)
- Judging whether an agent's result is good enough
- Deciding what to do next based on results
- Communication with Jörn
- Synthesizing multiple agent results
