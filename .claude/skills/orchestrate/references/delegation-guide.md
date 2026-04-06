# Delegation Guide

Reference file for orchestration agents. Read this to decide what and how to delegate.

## Agent types

| Type | Model | Use for |
|------|-------|---------|
| `general-purpose` | inherits | Default. Implementation, writing, debugging. |
| `Explore` | sonnet | Codebase search, file discovery, pattern finding. Read-only. |
| `review-rust` | sonnet | Check .rs files against project conventions. |
| `review-proof` | opus | Shallow correctness check on math.tex proofs. |
| `review-formalization` | opus | Check lemma↔code correspondence. |
| `review-claims` | sonnet | Verify factual claims against data/code/.bib. |
| `review-thesis` | sonnet | Check thesis .tex conventions. |
| `review-python` | sonnet | Check .py files against conventions. |
| `review-figures` | sonnet | Check .py→.tex→.png figure chain. |
| `session-search` | sonnet | Find past decisions in session JSONL transcripts. |

For `general-purpose` agents, set `model` explicitly: `"sonnet"` for mechanical work, `"opus"` for reasoning-heavy work.

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

Read the data file at second-order/curvatures.jsonl to understand the schema.
Read /workspaces/msc-math/crates/figure_config.py for the figure styling setup.
Read the logbook at second-order/logbook.md for what figures are needed.

Produce figures showing curvature along flat directions. Use `from figure_config import setup, FIGSIZE_SINGLE; setup()`.
Run the script with `cd /workspaces/msc-math/crates/exp-hko-local-maximum/second-order/ && uv run analyze.py`.
Report: figures produced, any unexpected findings in the data.
```

### Run pre-merge reviews (parallel, sonnet/opus)

```python
# Spawn all review agents in parallel on changed files
Agent(subagent_type="review-rust", prompt="Review these files: [list]. Report findings.", model="sonnet", run_in_background=True)
Agent(subagent_type="review-proof", prompt="Review /path/to/math.tex. Report findings.", model="opus", run_in_background=True)
Agent(subagent_type="review-claims", prompt="Verify claims in /path/to/logbook.md. Report findings.", model="sonnet", run_in_background=True)
# ... wait for all to complete, then synthesize
```

### Write math.tex proof (opus, worktree)

```
Write a proof of [lem:cap-derivative] in /workspaces/msc-math/crates/library/src/algorithms/math.tex.

The lemma states: the derivative of the EHZ capacity with respect to dual vertex a_k is [formula].
Read the function `capacity_derivatives_a()` in /workspaces/msc-math/crates/library/src/derivatives.rs to understand what the code computes.
Read the existing proofs in the same math.tex file for style and notation conventions.

Write the proof. Wrap in \begin{unverified}...\end{unverified} since Jörn hasn't verified it.
Build: `cd /workspaces/msc-math/crates && latexmk` — fix any errors.
Report: proof approach (1-2 sentences), any gaps or assumptions you're unsure about.
```

### Debug test failure (opus, worktree)

```
The test `test_projection_solver_sign` in /workspaces/msc-math/crates/library/src/kkt/tests.rs is failing.

Error message: "assertion failed: beta[3] > 0.0, got -1.2e-15"

Read the test, read projection_solver.rs, diagnose the root cause.
Fix the issue. Run `cd /workspaces/msc-math/crates/library/ && cargo test --release --lib` to verify.
Report: root cause, what you changed, test results.
```

### Regenerate experiment data (sonnet, no isolation)

```
Regenerate data for the random-sample experiment.
Run: `cd /workspaces/msc-math/crates/ && cargo run -p exp-sys-landscape --bin random-sample --release`
Then run the analysis: `cd /workspaces/msc-math/crates/exp-sys-landscape/random-sample/ && uv run analyze.py`
Report: how many records generated, any errors, figure files produced.
```

## Patterns

### Parallel independent agents

Use when sub-tasks don't depend on each other. Set `run_in_background: true` for all but the last.

Example: reviewing 3 independent files → 3 parallel review agents.

### Sequential dependent agents

Use when agent B needs agent A's result. Spawn A (foreground), read result, use it to construct B's prompt.

Example: "explore codebase for X" → result → "implement Y based on what was found."

### Fan-out then synthesize

Spawn N parallel agents, collect all results, then synthesize yourself (don't delegate synthesis — it requires judgment about what matters).

Example: run 5 review agents in parallel → collect findings → filter/prioritize → report to Jörn.

### Retry on failure

If an agent produces poor results: read why, write a better prompt with more context or constraints, spawn a new agent. Don't send the same prompt twice.

## Example decomposition: implement + test + document a library function

Session-scoped task: "Add `foo()` to the library with tests and math.tex proof."

Decomposition:
1. **Explore** (sonnet, background): find where similar functions live, what patterns they follow
2. **Implement** (opus, worktree): write the function + basic test, referencing explore results
3. **Write proof** (opus, worktree): write math.tex entry, referencing the implementation
4. **Review-formalization** (opus, background): check that proof matches code
5. **Review-rust** (sonnet, background): check code conventions
6. **Fix findings** (opus, worktree): address review findings
7. **Cargo test** (sonnet): run full test suite, report results

Steps 1 can run in parallel with nothing. Steps 2-3 are sequential (3 needs 2's output). Steps 4-5 run in parallel after 2-3. Step 6 after 4-5. Step 7 after 6.

## Example decomposition: run experiment + analyze + write logbook

Session-scoped task: "Run the foo experiment, analyze results, update logbook."

Decomposition:
1. **Explore** (sonnet): read run.rs and logbook.md to understand the experiment
2. **Run** (sonnet): execute the binary, collect output
3. **Analyze** (opus, worktree): write or update analyze.py, run it, produce figures
4. **Write logbook** (opus, worktree): update logbook.md with findings from steps 2-3
5. **Review-claims** (sonnet): verify logbook claims against data
6. **Review-figures** (sonnet): check figure quality
7. **Fix findings** (opus, worktree): address review findings

Steps 1-2 sequential. Step 3 after 2. Step 4 after 3. Steps 5-6 parallel after 4. Step 7 after 5-6.

## What NOT to delegate

- Decomposition decisions (that's YOUR job)
- Judging whether an agent's result is good enough
- Deciding what to do next based on results
- Communication with Jörn
- Synthesizing multiple agent results into a coherent picture
