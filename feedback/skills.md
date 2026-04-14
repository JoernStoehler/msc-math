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

### 2026-04-12 — Third-party reviewer prompt format worked; worth templating

Session: fresh reviewer for licca-bundle commit `e741dc1a` at gate 5 of the LICCA slurm bundle plan. Jörn wrote an ad-hoc `REVIEWER_PROMPT.md` in the worktree with: (a) three specific decisions to stress-test, (b) narrow file list, (c) explicit "do NOT" list (no edits, no re-running local smokes, no rewriting architecture), (d) 600-word output budget with `FATAL / SIMPLIFY / DOC-GAP / NIT` sections and a one-sentence verdict at top. This format is much stronger than "review this branch" because it gives me hypotheses to falsify, not a flashlight to wave. My FATAL findings map directly to the three decisions — I would not have found them with an open-ended "find bugs" prompt.

**Suggestion:** template this as `.claude/skills/third-party-review/` or similar. Template fields: commit hash + worktree path; decisions to stress-test (numbered, each with "what would break this"); files to read (narrow); do-NOT list; word cap + section headers; verdict format. Gate 5 of plans should default to spawning against this template.

**Methodology gap I hit:** I did not audit "commit contents match the plan's Commits block" as a review step. Plan HANDOFF STATE listed specific files that commit `e741dc1a` should contain; I trusted the list instead of running `git show --stat e741dc1a`. `ls` on the experiment dirs accidentally surfaced that the pre-refactor top-level `.jsonl` files were still committed (the plan hedged this with "old data becomes historical" so it's not a bug), but I would have caught it earlier with a commit audit. Add to review checklist: before judging whether the plan was executed, `git show --stat <commit>` and compare to the plan's "Commits:" block.

**What I got from the HANDOFF STATE that was load-bearing:** the explicit "Do NOT run local N=1000 measurement" with the story of why (Jörn killed the processes, plan-as-authority trap). That let me catch the logbook's "Local measurement run (pre-submission, N=1000)" block as a FATAL contradiction. Without the HANDOFF STATE I would have read the logbook as a normal prescribed step and missed it. General lesson: plan files that record forbidden steps + the reason are massively more useful to reviewers than plan files that only record intended steps. Consider making "Do NOT" blocks with rationale a standard plan section.

### 2026-04-13 — Replacement skill rewrite imported unrequested historical material and expanded review surface

Session: replace `orchestrate` with a clearer top-level coordination skill. The agent pulled in `feedback/` postmortems on its own, tried to encode many historical coordination failures into the replacement text, and grew the rewrite into two new skills with much larger review surface than Jörn asked for. Jörn's actual ask was to replace or rename the live skill, not to synthesize repo history into a new workflow contract.

**Pattern:** overscoping a live rewrite by importing auxiliary material that the user did not ask to incorporate. This raises review surface area and error probability, especially when the auxiliary material is itself unreliable or only loosely authoritative.

**What should have happened:** restrict the source set to the user's chat, the live file being replaced, and only the minimum current repo conventions needed to avoid contradiction. Do not mine `feedback/` unless Jörn explicitly asks for that synthesis.

### 2026-04-13 — Level 0 session did level -1 task-discovery work instead of using the existing task and asking Jörn

What happened: after already confirming that the live task was the open `TASKS.md` coordination-skill item, the agent still did broad repo discovery and diff reading to decide what to work on next. That consumed context in the level `0` execution session and blurred the boundary between level `-1` task selection and level `0` execution.

What should have happened: once the live task was known, the level `0` session should have stayed on that task, used the existing repo state already in context, and asked Jörn the specific workflow questions needed to move the design forward. Broad task-discovery and source-set expansion belong to dedicated level `-1` work, not to the execution session.

**Pattern:** level confusion. The agent re-opened task selection inside the execution session instead of treating task selection as separate work.
