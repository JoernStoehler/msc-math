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

### 2026-03-30 — Post-mortem template says "Save to" test-tasks/ without format guidance

Post-mortem skill template section 6 (regression test candidate) says:

> Extract: the context the agent had, the user message, what happened (good or bad), what the correct behavior is. Save to `.claude/skills/test-workflow/references/test-tasks/`.

Agent followed "Save to" literally and wrote a test case file without checking existing files in that directory for format conventions. Jörn rejected it: "That is not a valid test case. You are not knowledgeable about creating test cases."

**Suggestion:** The template should say something like "Note the candidate for a future /test-workflow session" instead of "Save to" — or at minimum say "check existing test cases for format before writing."

### 2026-03-30 — Pre-merge checklist didn't catch worktree violation

Agent ran /pre-merge after committing directly to main (violating explicit "Work in a worktree" instruction). The pre-merge report said "nothing needs Jörn's review" without flagging that the process instruction had been violated. Jörn said "Merge" based on the clean report, then discovered the problem.

The pre-merge skill checks builds, tests, data freshness, content, and TASKS.md — but doesn't check whether the work was done according to process instructions (worktree, branch naming, etc.). A "process compliance" check — "Did you follow the instructions in the original prompt?" — would have caught this.

**Suggestion:** Add a check to pre-merge: "Were all explicit process instructions from the task prompt followed? (worktree, branch name, commit conventions, etc.)"

### 2026-04-01 — Pre-merge content checks not linked to review subagents

Agent ran /pre-merge and handled content checks ("All new factual claims verified", "New math.tex content has proofs", "Logbook entries cite sources inline") by grepping for "GAP" and eyeballing a few lines. Didn't launch any review subagents despite CLAUDE.md saying "Use review agents proactively before presenting work" and despite review-proof, review-claims, review-formalization agents existing for exactly these checks. Only launched them after Jörn asked about it.

**Root cause:** The pre-merge skill lists content checks as a checklist but doesn't say how to do them. The agent defaulted to shallow manual checking instead of using the purpose-built review agents. The connection between "verify factual claims" and "launch review-claims agent" wasn't made.

**Suggestion:** Pre-merge content checks section should explicitly say which review subagents to launch:
- "All new factual claims verified" → launch review-claims on logbook.md and any files with new claims
- "New math.tex content has proofs" → launch review-proof on math.tex
- "Cross-references resolve" → launch review-formalization on the module
- Run these as parallel background agents while doing the bash build/test checks

### 2026-04-02 — Pre-merge skill text had stale paths during session

The pre-merge skill shown to the agent after `/pre-merge` still referenced `cd crates/` and `cd experiments/` — the old paths — even though the skill file itself had been updated earlier in the session (by a subagent). This is likely because the skill content was loaded from the SKILL.md file at invocation time and the subagent's edit happened within the same session. The agent adapted and ran the right commands anyway, but a less experienced agent might have followed the stale instructions.

**Not actionable as a skill fix** — this was a one-time issue caused by editing the skill file and then invoking it in the same session. The skill file is now correct.

### 2026-04-02 — Mechanical experiment fixes should check whether experiment is active

Post-migration audit fixed 8 broken experiment binaries via subagents. One of them (`gradient-search`) is superseded by `sys-search` (documented in TASKS.md 100 lines below the gradient-search section). Neither the main agent nor the subagent checked whether the experiment was still active before fixing it. Jörn caught it.

**Suggestion:** When fixing experiment code mechanically (API migration, import updates), check TASKS.md and the experiment's logbook.md for supersession/deprecation status before presenting the fix as meaningful work. A 30-second read avoids presenting stale work to Jörn.
