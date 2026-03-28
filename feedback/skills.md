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
