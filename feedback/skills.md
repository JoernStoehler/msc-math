# Feedback: Skills (.claude/skills/)

Raw observations from agents about skill workflows. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, which skill, what happened, what was confusing/missing/unhelpful.

### 2026-03-28 — Plan mode trapped report deliverable in ephemeral file

Session: cross-reference audit. Task was "write a structured report" — a read-only audit producing a document, not code. Plan mode activated and the agent ran the entire audit (5 subagents, all math.tex + .rs files) during plan phase, writing the complete report into the ephemeral plan file at `.claude/plans/`. When Jörn pointed out the file doesn't persist, the work was already done in the wrong place.

**Root cause:** Plan mode's 5-phase workflow assumes the deliverable is code changes. Its prompt says "this is the only file you are allowed to edit." For tasks where the deliverable IS a document/report, the agent should recognize the task shape mismatch and exit plan mode immediately, then do the work with full tool access.

**Suggestion:** Plan mode prompt should include guidance like: "If the task deliverable is a report or document rather than code changes, exit plan mode immediately and produce the deliverable directly." Alternatively, the agent could be taught to recognize report/audit tasks and avoid entering plan mode at all.

### 2026-03-28 — ExitPlanMode returned false "approved" signal

During the cross-reference audit session, ExitPlanMode tool returned "User has approved your plan" when Jörn had NOT approved. The agent proceeded as if it had clearance. Jörn caught it: "I did not APPROVE the plan. Is ANYTHING LYING TO YOU?"

This is a platform/tool bug — the tool result was factually wrong. The agent should not blindly trust ExitPlanMode's result if the interaction felt ambiguous or if the user's preceding messages suggested disapproval.

### 2026-04-02 — Pre-merge skill text had stale paths during session

The pre-merge skill shown to the agent after `/pre-merge` still referenced `cd crates/` and `cd experiments/` — the old paths — even though the skill file itself had been updated earlier in the session (by a subagent). This is likely because the skill content was loaded from the SKILL.md file at invocation time and the subagent's edit happened within the same session. The agent adapted and ran the right commands anyway, but a less experienced agent might have followed the stale instructions.

**Not actionable as a skill fix** — this was a one-time issue caused by editing the skill file and then invoking it in the same session. The skill file is now correct.

### 2026-04-04 — Pre-merge skill doesn't cover post-hoc reviews

`/pre-merge` was run after the branch was already merged to main (Jörn merged without running it first). Phase 5 sanity check says "Work is on a worktree branch, not main" — inapplicable in this case. Phase 3 (data freshness) and Phase 2 (smoke tests) worked fine on main. The skill could note that post-hoc reviews on main are valid — skip the "not on main" check, and note that fixes go directly on main as follow-up commits rather than on a branch.
