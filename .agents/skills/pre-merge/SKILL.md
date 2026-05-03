---
name: pre-merge
description: Top-level merge-readiness workflow before asking Jörn about integration to main; subagents use assigned parts only as readiness checks and do not decide merge status or approval.
---

# Pre-Merge Workflow

Evaluate all phases in order before telling Jörn work is ready. Run the
in-scope checks, record empty phases as empty, and record optional phases as
not needed when their trigger is absent. Fix failures before proceeding to the
next phase.

For docs-only or harness-only branches, scope the phases to the touched
surfaces instead of running unrelated Rust, TeX, or experiment checks. A scoped
run must state the touched surface, the checks that measure that surface, and
which phases were omitted as irrelevant. Use the full workflow when code,
formal math, thesis sources, generated data, or experiment behavior changed.
For prompt/harness-only changes, include `git diff --check`, skill validation
for touched skill folders, TOML parsing for changed `.codex/*.toml`, and
targeted stale-reference searches when paths, skill names, or authority surfaces
changed.

## Phase 1: Build and test

Run all of these. If a command fails, fix the issue and rerun before proceeding.

```bash
cd crates/symplectic/ && cargo test --release --lib
cd crates/symplectic/ && cargo clippy --lib -- -D warnings
cargo build --workspace --release
cd thesis/ && latexmk && ./check-build.sh
cd formal/ && latexmk
```

## Phase 2: Smoke-test experiment binaries

List all experiment `main.rs` files on this branch. For each, compile and run with the fewest polytopes the binary accepts (typically 1). If the binary takes no dataset argument, run `--help` or the default invocation. Goal: catch panics and import errors early. The polytope database caches results, so hot runs are fast.

No experiment `main.rs` files on the branch → nothing to do (empty set, not a skip).

## Phase 3: Data freshness

For experiments with committed data (`.jsonl`, `.csv`), compare code and data commit dates:

```bash
git log -1 --format='%H %ci' -- experiments/<topic>/<experiment>/main.rs
git log -1 --format='%H %ci' -- experiments/<topic>/<experiment>/*.jsonl
```

If code is newer than data:

- regenerate on this branch when the branch changes the tracked artifact contract for the same filename
- otherwise report the dataset as stale and schedule refresh as post-merge follow-up by default
- only treat stale generator data as a pre-merge blocker when Jörn explicitly asked for a data-refresh pass in this branch

## Phase 4: Review

Use the `reviewer` subagent plus `$review`. Launch separate reviewer instances for independent review surfaces, not separate agent definitions for every file type.

Default review surfaces:

| Surface | Scope | Skills / review references |
|----------|-------|----------------------------|
| Rust | Changed `.rs` files | `$rust-conventions`, `.agents/skills/review/references/rust.md` |
| Formal math | Changed `formal/**/*.tex` files and Rust-linked labels | `$formal-math-conventions`, `.agents/skills/review/references/formal-math.md` |
| Claims | Changed result summaries, thesis text, captions, formal commentary | `.agents/skills/review/references/claims.md` |
| Thesis | Changed `thesis/**/*.tex` files | `$thesis-tex-conventions`, `.agents/skills/review/references/thesis.md` |
| Python | Changed `.py` files | `$python-conventions`, `.agents/skills/review/references/python.md` |
| Figures | Changed `analyze.py`, `.png`, or generated figure/table `.tex` files | `.agents/skills/review/references/figures.md` |
| Prompt/harness | Changed `AGENTS.md`, `.agents/skills/**`, `.codex/**`, or agent-facing task packets / temporary handoff notes | `$harness-engineering`; `$skill-creator` for skill behavior changes; `$openai-docs` for current OpenAI or Codex behavior claims |

If a surface has no files in scope, record "no files in scope" in the local notes. Do not launch an empty reviewer solely to prove the absence.

### Cross-check subagent findings

Before including any finding in the report to Jörn, read the file at the location the subagent references and confirm the finding matches what the code or text actually says.

Verify with priority:
1. **Cost-benefit recommendations** the subagent made — subagents lack context for cost-benefit judgments about the larger task. Severity ratings (FIX vs FLAG) reflect the subagent's limited view: it may escalate minor issues or downplay significant ones.
2. **Interpretive conclusions** where the subagent inferred meaning from limited context — e.g., "this lemma is orphaned" (may be used by other modules) or "this reference dangles" (may resolve through `formal/main.tex`).
3. **Specific claim types:** "dangling reference" -> check if it resolves through the relevant formal or thesis build. "Orphaned lemma" -> check if used elsewhere or is standalone valid math. "Missing entry" -> check `ROADMAP.md`, `tasks/*.md`, `research/INDEX.md`, or the relevant research note for a known gap before treating it as newly discovered.

A verification subagent can cross-check the combined findings when the task has high blast radius or the first reviewer reports subtle findings.

The Phase 8 report contains only verified findings, not the review/cross-check process.

## Phase 5: Sanity check

- **Goal alignment:** Re-read the original task prompt. Does the work produced actually serve that goal? Does it make sense for the thesis project roadmap? A misunderstood goal that produces technically correct but wrong-direction work is expensive to discover late.
- **Process compliance:** Work is on a worktree branch, or the task explicitly
  targeted the main checkout / root checkout. Explicit instructions from the
  task prompt were followed (branch naming, scope restrictions, etc.).
- **Project context:** Check `ROADMAP.md` and the relevant `tasks/*.md` bundle. Does this work correspond to tracked work? Is the experiment still active, not superseded by another experiment?

## Phase 6: Update roadmap surfaces

- Mark completed work as done in the relevant task bundle.
- Update status and next steps for work affected by this branch.
- Add newly discovered work to the relevant bundle.
- If no updates are needed, state that explicitly in the report ("roadmap surfaces: no changes needed").

## Phase 7: Full experiment runs (optional)

If experiment binaries were created or substantially modified, and Phase 4 review found issues that were fixed, run again with representative input to confirm fixes. Report results.

This phase is optional. Run it when experiment binaries were created or substantially modified in this branch.

## Phase 8: Report to Jörn

Structure:

1. **What changed** — files, scope, one-paragraph summary
2. **Build/test results** — which commands passed, any issues fixed during Phase 1
3. **Review findings** — verified findings from Phase 4 subagents (after cross-check)
4. **Needs Jörn** — decisions, unresolved `% [TODO: JÖRN` items, things only Jörn can verify
5. **Roadmap changes** — what was updated, or "no changes needed"
6. If work is incomplete: write a temporary handoff to `/tmp/<name>.md`;
   update the relevant `tasks/*.md` or `ROADMAP.md` entry only when durable
   task state changed.

## Merge Conflicts

When resolving conflicts during integration, choose the content that is true for the current repository state and project conventions. Do not use timestamp, branch side, author, or apparent task ownership as a shortcut for deciding which side wins.

For each conflicted hunk, identify the claim or behavior each side represents, check the surrounding files or commands when needed, then keep or combine the parts that preserve current mathematical statements, live paths, task status, and build behavior. If neither side is clearly correct, stop and ask Jörn with the concrete hunk and the missing decision.
