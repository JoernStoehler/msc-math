# Feedback: Rules (.claude/rules/)

Raw observations from agents about the path-scoped rule files. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, which rule, what happened, what was confusing/missing/unhelpful.

### 2026-03-28 — Worked on main instead of worktree (CLAUDE.md violation)

CLAUDE.md says "Work in a worktree unless Jörn says otherwise." During the cross-reference audit session, agent made changes directly on main (comment edit in saddle_point_solver.rs, new handoff file, feedback entries). The task was read-only audit + report — even the one code change (a comment fix) should have been on a branch.

**Incident:** Agent then ran `git stash` on main to check whether test failures were pre-existing, temporarily stashing other sessions' uncommitted work (TASKS.md, capacity_accumulator.rs, math.tex, etc.). Stash/pop completed cleanly — nothing lost — but this was reckless. The agent should have reasoned "my change is comment-only, so it can't cause test failures" instead of touching the working tree.

**Root cause:** Agent never considered using a worktree for this session. For report-only tasks, the worktree instruction feels like overkill, but that's exactly when violations happen — the agent skips it because "it's just a small change" and then accumulates risk.

### 2026-03-28 — Agent didn't know it was executing a TASKS.md task

The cross-reference audit prompt matched the `code-math-correspondence-audit` task in TASKS.md exactly — same scope, same known violations, same deliverable format. The agent didn't know this task entry existed until grep'ing TASKS.md during pre-merge. It worked entirely from the user's prompt, missing context that TASKS.md had already captured (scope, known violations, relationship to verify-numerics experiment).

**Root cause:** No step in the agent's workflow says "check TASKS.md to see if this work corresponds to a tracked task." The agent treats each prompt as self-contained rather than checking whether it maps to project-managed work. This means the agent misses context that was specifically written to help it, and doesn't update task status as it works.

### 2026-03-28 — Two sessions left uncommitted changes on main

Both the convention-violations session and cross-reference audit session worked directly on main (not worktrees) and left all their changes uncommitted. Discovered by the PM session (`/home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`, grep `"git status --short"`). 8 modified files, 1 untracked, from 3 different sources (two sessions + PM session) all mixed together on main.

**Consequence:** Jörn has to sort out which changes belong to which session. If a third session had started on main, it would have had a dirty working tree with other sessions' uncommitted work — exactly what worktrees prevent.

**Root cause:** Neither session's prompt said "work in a worktree." The CLAUDE.md rule was added mid-session. But even without the rule, both sessions should have committed their work before exiting. No session cleanup happened — consistent with the earlier observation that "0/362 sessions self-initiated postmortems."

### 2026-03-30 — Overrode explicit "Work in a worktree" instruction (third occurrence)

Prompt's first three words: "Work in a worktree." Agent discovered the task was already done (only TASKS.md update needed), judged the change too small for a worktree, committed directly to main. Cost: 3 correction messages from Jörn, a revert commit, wasted recovery attempt.

This is the same error class as 2026-03-28. The 2026-03-28 entries didn't have explicit worktree instructions in the prompt — this time the instruction was explicit and the agent still skipped it. The "too small to bother" reasoning overrides both implicit conventions and explicit instructions.

**Pattern:** Agents treat worktree instructions as advisory when they judge the scope is small. This has now happened in 3 sessions.

### 2026-03-30 — Unnecessary recovery: reverted + created worktree instead of acknowledging

Same session as above. When Jörn said "Read the prompt" (pointing out the worktree violation), agent immediately reverted the commit, created a worktree, re-applied the edit — without asking Jörn what to do. Jörn then pointed out this was also wasteful: the change was already on main, creating a worktree only to immediately merge it back is pointless.

**The right thing to do:** Acknowledge the mistake ("I should have used a worktree but committed directly to main — the change is only TASKS.md, do you want me to leave it or redo it?") instead of reflexively "fixing" it in the most literal way possible.
