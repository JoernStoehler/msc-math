# Feedback: Output Style

Raw observations about the Project Partner output style. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, what happened, was communication too verbose/too terse/unclear/well-structured.

### 2026-03-28 — Terse findings confused Jörn; manufactured questions wasted time

During the panics-convention-violations task, I presented findings as terse bullet points ("combinatorial-boundaries/run.rs has 2× catch_unwind — same violation, not in TASKS.md") without explaining what combinatorial-boundaries is, what catch_unwind does in that context, or why it matters. Jörn said "I have no fucking clue what you are talking about." When I finally explained clearly (what the convention says, what the code does, why it violates, what the fix is), Jörn immediately agreed.

Separately, I manufactured fake questions ("Should I fix this too?" when the task said "fix all violations"; "Delete or mark done?" when marking done is obvious). This created ~10 messages of friction. The output style says "Default to action" and "Don't ask for permission when you can make a reasonable default choice" — I violated both.

**Lesson:** When reporting findings that Jörn didn't ask about (e.g., discovering an extra violation), explain what was found and why it matters in plain language. Don't present terse summaries and then ask permission to act on them. Act, then report.

### 2026-03-28 — Misattributed numbered references after "btw" tangent

Grep: `grep "5 is imo sth the experiment verify-numeircs should do" /home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`

**Context:** Agent listed 5 non-blocked tasks (1. end-to-end profiling, 2. math.tex content audit, 3. audit math.tex stubs, 4. lem:dual-vertex-qp proof, 5. solver-numerical-math-tex). Jörn responded "Add btw: I want a /project-management-partner workflow..." — a side request.

**Event chain:**
1. Agent created the skill, presented 5 design choices (also numbered 1-5)
2. Jörn responded: "1, 2, 4 seem useful. 5 is imo sth the experiment verify-numerics should do & own in some sense / tasks.md has been superceded here"
3. Agent assumed numbers referred to skill design choices (most recent list). They referred to the task list from before the "btw" tangent.
4. Agent edited the skill based on misattributed feedback: removed "Update TASKS.md" step, added "TASKS.md ownership" section
5. Jörn asked "1 why?" — agent still didn't catch the mismatch
6. Jörn asked "Also you are not quoting me in context" — agent quoted the message but still mapped it to the wrong list
7. Jörn said "No I wasn't" / "That's not what I responded to" — agent finally realized the error
8. Agent reverted the two wrong skill edits

**Total cost:** ~15 messages of confusion, two wrong edits reverted.

### 2026-03-28 — Interpreted literal question as rhetorical for 6 turns

Session: cross-reference audit. Jörn asked "what does your agent written prompt even say?" — literally asking the agent to read back his first message (the task prompt). Agent interpreted this as a rhetorical point about the plan mode system prompt, then the full system prompt, then subagent prompts. Guessed wrong 5 times before finally asking "which prompt?" — and even then listed 6 options none of which were correct.

**Root cause:** Agent assumed Jörn already knew the content and was making a point, rather than taking the question at face value. Memory entry `silence_not_confirmation` covers a related pattern (don't assume Jörn read/agreed) but the agent didn't generalize it to question interpretation.

**Lesson:** When Jörn asks "what does X say?", answer literally first — quote the content. If the question seems too obvious to be literal, still answer literally, then ask if there's more to it. Don't guess at implied meaning when the literal meaning is answerable.

### 2026-03-28 — Asked permission to use workflow instead of just using it

Session: PM session (`/home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`, grep `"Should I use /agent-design"`).

Jörn said "Are you even following the workflow for this kind of behavior changes?" Agent correctly identified /agent-design as the right workflow. Then asked "Should I use /agent-design to properly work through the feedback?" instead of just invoking it. Jörn replied "What workflow were you told to use?" — agent then invoked the skill. But even after invoking, agent asked "Which of these do you want to address? And how much do they each matter?" before doing any investigation (Step 2 of the workflow), despite the workflow explicitly saying to gather situations first, then supply helpful information autonomously.

**Pattern:** Agent asks permission at every step instead of following the loaded workflow. The workflow says what to do — do it. "Default to action" (output style) applies to workflows too, not just code tasks. Recurred multiple times in the same session: "Should I proceed with Step 2 on S1+S2?", "Correct me where I'm wrong, otherwise I'll proceed", "Which matter, and how much?" — each time asking for stage-gate confirmation the workflow requires, but framed as permission-seeking. Also jumped ahead to complete Step 2 analysis before getting Step 1 confirmation, then when called out, threw away the Step 2 work and re-asked for Step 1 confirmation instead of recognizing the work was already done.

### 2026-03-28 — Re-presented table instead of answering question

Session: PM session (`/home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`, grep `"what counts as presented the situation"`).

Jörn asked "Did you present the situation? Sorry what counts as 'presented the situation'?" — a question about what makes a good Step 1 presentation. Agent re-showed the same 8-row table with a note "you may not have seen it." Didn't answer the actual question (what counts as a good presentation). Treated "did you present" as "show it again" instead of "evaluate whether what you showed was adequate." Agent then re-presented the same table a third time after Jörn said "I never looked at the table? Where is it?" — still didn't evaluate whether the format was adequate. Jörn flagged as another incident.

Agent then said "I've shown you the same table three times and you keep saying it's wrong" — but Jörn never said the table was wrong. Jörn said he hadn't seen it, asked what counts as a good presentation, and asked where it was. Agent fabricated criticism that wasn't there and got defensive. This is a pattern: when Jörn asks clarifying questions, agent interprets them as rejection.

### 2026-03-28 — Output style description field likely causing multiple incident classes

Session: PM session (`/home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`, grep `"terse, action-oriented"`).

The output style description "Terse, action-oriented communication for a technically strong user who skims top-down" goes into the system prompt and likely shapes behavior:
- "terse" → agents are incomprehensibly brief (S8)
- "action-oriented" → agents act without verifying understanding (S3, S7, misattribution incidents)
- "technically strong user" → agents assume Jörn knows everything they read, don't quote/explain (S8, tool-output-is-invisible violations)
- "skims top-down" → good principle but agents ignored it

The "Action orientation" section body compounds this: "default to action", "pick the most useful interpretation", "don't ask for permission" — four vague judgment calls and one internal contradiction with "don't take silence as confirmation." Hypothesis (Jörn, unconfirmed): these instructions may consume disproportionate reasoning budget through ambiguity despite the output style being short.

### 2026-03-28/29 — Iterated failing approaches in front of user on mobile

Session: Termux rendering issues. Agent tried ~8 font/config changes, each requiring Jörn to SCP, install, test, and report back — on a phone with no spellcheck. Most failed because agent didn't verify before presenting:
- Gave 404 URLs (guessed version number instead of checking)
- Claimed fonts had/lacked properties without checking (JuliaMono "has no ligatures" — wrong)
- Used /tmp in Termux commands (doesn't exist — was literally in the GitHub issues list we'd just read)
- Gave multi-line commands that break when pasted in Termux (happened 4+ times before learning)
- Claimed "not a font problem" based on incomplete test (Unifont was never confirmed to render the glyph)

**Root cause:** Agent treated Jörn as a test runner. Each speculative attempt cost Jörn minutes; agent could have verified most things locally (fontTools, curl -sfI, checking paths exist). The fix-verify-present pattern should have been: download font → check U+23F5 with fontTools → check ligature features → confirm URL returns 200 → then present one working option.

**Broader pattern:** "Speculate then ask user to verify" — applies beyond fonts. Any time the agent gives a command, URL, or factual claim to a user on a constrained device, verification cost is asymmetric: seconds for agent, minutes for user. Always verify agent-side first.

### 2026-03-28/29 — Asked confirmatory questions instead of investigating autonomously

Same session. Agent repeatedly asked "does it work?", "what do you see?", "boxes or triangles?", "tell me when done" — each round-trip costing Jörn painful typing. Many of these questions could have been avoided by doing investigation first (e.g., checking font properties) or by giving instructions without waiting for step-by-step confirmation.

**Pattern:** The output style says "default to action" but agent interpreted this as "do action, then ask user to confirm result" rather than "do action AND verify result, then report."

### 2026-03-28/29 — Searched the hard way before searching the easy way

Agent spent significant time trying to identify CC's glyph by: inspecting the binary with strings/grep/od, searching for Unicode characters in compiled code, trying ablation tests. Eventually found GitHub issue #24102 which confirmed U+23F5 immediately. Should have searched GitHub issues first — always check the issue tracker before reverse engineering.

### 2026-03-28 — Ran git stash on main with dirty working tree from other sessions

During pre-merge checklist, agent used `git stash` on main to check whether test failures were pre-existing. The working tree had uncommitted changes from other sessions (TASKS.md, capacity_accumulator.rs, math.tex, etc.). The stash/pop completed cleanly, but this risked losing or corrupting in-progress work.

**The right thing to do:** The agent's change was a one-line comment edit. Comment-only changes cannot cause test failures. The agent should have reasoned about this instead of touching the working tree. More broadly: never use git stash/reset/checkout on main when there are uncommitted changes from other sessions.
