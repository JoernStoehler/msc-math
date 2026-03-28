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
