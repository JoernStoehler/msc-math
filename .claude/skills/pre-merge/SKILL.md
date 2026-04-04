---
name: pre-merge
description: Mandatory workflow before presenting work for merge to main. Load when finishing a task and preparing to report to Jörn.
---

# Pre-Merge Workflow

Run all phases in order before telling Jörn work is ready. Every phase runs on every branch — do not skip phases because "no changes in this area." Fix failures before proceeding to the next phase.

## Phase 1: Build and test

Run all of these. If a command fails, fix the issue and rerun before proceeding.

```bash
cd crates/library/ && cargo test --release --lib
cd crates/library/ && cargo clippy --lib -- -D warnings
cargo build --workspace --release
cd thesis/ && latexmk && ./check-build.sh
pdflatex math.tex && pdflatex math.tex
```

## Phase 2: Smoke-test experiment binaries

List all `run.rs` files on this branch. For each, compile and run with the fewest polytopes the binary accepts (typically 1). If the binary takes no dataset argument, run `--help` or the default invocation. Goal: catch panics and import errors early. The polytope database caches results, so hot runs are fast.

No `run.rs` files on the branch → nothing to do (empty set, not a skip).

## Phase 3: Data freshness

For experiments with committed data (`.jsonl`, `.csv`), compare code and data commit dates:

```bash
git log -1 --format='%H %ci' -- crates/exp-<group>/<subdir>/run.rs
git log -1 --format='%H %ci' -- crates/exp-<group>/<subdir>/*.jsonl
```

If code is newer than data, regenerate on this branch.

## Phase 4: Review subagents

Launch all review subagents in parallel on the branch diff:

| Subagent | Scope |
|----------|-------|
| review-rust | Changed `.rs` files |
| review-proof | Changed `math.tex` files |
| review-formalization | Modules with both `.rs` and `math.tex` changes |
| review-claims | Changed `logbook.md`, thesis `.tex` with claims, `math.tex` |
| review-thesis | Changed thesis `.tex` files |
| review-python | Changed `.py` files |
| review-figures | Changed `analyze.py` files or changed `.png` files |

Launch all review subagents. If a subagent finds no files in scope, it reports "no files in scope" — that is the expected outcome, not a reason to skip launching it.

### Cross-check subagent findings

Before including any finding in the report to Jörn, read the file at the location the subagent references and confirm the finding matches what the code or text actually says.

**Trust without re-checking:** quotes and file:line references (agents are trained on these; low error rate).

**Verify with priority:**
1. **Cost-benefit recommendations** the subagent made — subagents lack context for cost-benefit judgments about the larger task. Severity ratings (FIX vs FLAG) reflect the subagent's limited view: it may escalate minor issues or downplay significant ones.
2. **Interpretive conclusions** where the subagent inferred meaning from limited context — e.g., "this lemma is orphaned" (may be used by other modules) or "this reference dangles" (may resolve via root `math.tex`).
3. **Specific claim types:** "dangling reference" → check if it resolves via root `math.tex` (cross-module refs do). "Orphaned lemma" → check if used elsewhere or is standalone valid math. "Missing entry" → check logbook/TASKS.md for "Part N not written" (known gap, not discovery).

A verification subagent can cross-check the combined findings — it has fresh eyes and no sunk-cost bias toward the original findings.

The Phase 8 report contains only verified findings, not the review/cross-check process.

## Phase 5: Sanity check

- **Goal alignment:** Re-read the original task prompt. Does the work produced actually serve that goal? Does it make sense for the thesis project roadmap? A misunderstood goal that produces technically correct but wrong-direction work is expensive to discover late.
- **Process compliance:** Work is on a worktree branch, not `main`. Explicit instructions from the task prompt were followed (branch naming, scope restrictions, etc.).
- **Project context:** Check TASKS.md — does this work correspond to a tracked task? Is the experiment still active (not superseded by another experiment)?

## Phase 6: Update TASKS.md

- Mark completed tasks as done (move to Completed section with date and one-line summary)
- Update status and next-steps for tasks affected by this work
- Add newly discovered tasks
- If no updates are needed, state that explicitly in the report ("TASKS.md: no changes needed")

## Phase 7: Full experiment runs (optional)

If experiment binaries were created or substantially modified, and Phase 4 review found issues that were fixed, run again with representative input to confirm fixes. Report results.

This phase is optional. Run it when experiment binaries were created or substantially modified in this branch.

## Phase 8: Report to Jörn

Structure:

1. **What changed** — files, scope, one-paragraph summary
2. **Build/test results** — which commands passed, any issues fixed during Phase 1
3. **Review findings** — verified findings from Phase 4 subagents (after cross-check)
4. **Needs Jörn** — decisions, unresolved `% [TODO: JÖRN` items, things only Jörn can verify
5. **TASKS.md changes** — what was updated, or "no changes needed"
6. If work is incomplete: write a handoff to `handoffs/<name>.md`
