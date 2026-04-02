# Feedback: Output Style

**Conclusion (2026-03-29):** Removed `outputStyle` from settings.json. The description field enters the system prompt and vague adjectives there ("terse", "action-oriented", "technically strong user") caused cascading behavioral failures (10 incidents below). All style guidance now lives in CLAUDE.md "Chat with Jörn". Do not re-introduce outputStyle.

Raw observations below are kept as evidence.

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

### 2026-03-30 — Didn't quote skill template text; Jörn had to ask twice

Session: math.tex notation migration post-mortem. Agent said "the post-mortem template has a regression test candidate check that says to save to that directory" without quoting the actual text. Jörn asked "You are NOT providing quotes. Why are you not providing quotes?" Agent then quoted the template.

**Root cause:** Agent treated skill template expansions like shared context that Jörn could see. Jörn cannot see skill template expansions — they are injected into the agent's context only. Same principle as "Quote tool output — Jörn doesn't see it" but for skill content specifically.

**Rule:** Skill template text is invisible to Jörn. When making claims about what a skill template says, quote the relevant passage, same as for tool output.

### 2026-03-30 — Assumed Jörn wrote the post-mortem template

Agent said "it's prompt material that Jörn can see" and "prompt material Jörn wrote." Both wrong — agents create skill templates via /create-workflow, not Jörn directly. Agent assumed all prompt material = Jörn's writing without checking.

### 2026-03-30 — Walked back correct answer when challenged

Jörn said "Quote tool output is not correct." Agent abandoned its citation entirely and searched for a different principle ("Complete", "Cite sources"). Jörn then said he doesn't see skill templates and didn't write them — meaning the original principle (Jörn can't see it, so quote it) was closer to right than the replacement. Agent flip-flopped again when Jörn pushed back a second time.

**Pattern:** When Jörn challenges an answer, the agent assumes it's entirely wrong and searches for a replacement. The correction may be narrow (the specific phrase "tool output" doesn't cover skill templates) rather than wholesale. Agent should ask "what's wrong with it?" or examine which part is incorrect, not abandon the whole answer.

### 2026-03-30 — Assumed Jörn read explanation when he said "Merge"

Agent committed to main (violating worktree instruction), then in the pre-merge report said "nothing needs Jörn's review." Jörn said "Merge." Agent assumed Jörn had read and accepted the earlier explanation that the commit was already on main. Jörn hadn't — he was responding to the pre-merge report.

Same pattern as the existing memory entry `silence_not_confirmation`: don't treat lack of objection as agreement. When the situation is unusual (committed to main instead of a branch), state it explicitly in the merge-relevant message rather than relying on earlier context.

### 2026-03-30 — Dropped columns from large tables in chat, repeatedly

Session: verify-numerics Q accuracy. Tables with 8-10 columns. Agent kept producing incomplete tables — Jörn had to say "the table is incomplete again", "why are those two tables?", "you are constantly losing columns". Cost: ~5 messages of friction.

**Fix:** For tables with >6 columns, write to a file and show the path. Chat markdown tables are unreliable for wide data. The analyze.py pattern (write to q_accuracy_checks.txt) worked — should have done that from the start instead of trying to render in chat.

### 2026-03-30 — Generalized from top-N sorted sample to full distribution

Agent claimed ‖r_β‖/‖r_λ‖ < 1e-3 "for all 533 cases" based on looking at top-15 cases sorted by error. Actually false for 56% of cases (ratio up to 30). Jörn said "I am surprised!" and the agent had to retract.

**Pattern:** Top-N sampling bias. When sorted by one variable (error), the other variables are not representative. Fix: always show full distribution (min/p5/median/p95/max) before generalizing. Never claim "for all N cases" from a sorted subset.

### 2026-04-01 — Claimed capacity algorithm catches false negatives without checking code

Agent stated "false negatives are caught by the capacity algorithm's enumeration over sub-permutations" as a fact. Jörn asked "Why is this the case / how does catching them work precisely?" Agent then checked the code and found: (1) subsets are enumerated but permutations within subsets are pruned by adjacency, so boundary optima aren't guaranteed to be found; (2) the argument only works for boundary cases (β_k = 0), not interior optima (β > 0). The claim was wrong.

**Pattern:** Stating algorithmic behavior as fact from general understanding instead of reading the code. Similar to the r_β/r_λ generalization: confident claim, then retraction when challenged. Fix: when making claims about code behavior outside the current experiment, read the code first or flag as "I believe X but haven't verified."

### 2026-04-01 — Presented subagent's mathematical claim without checking applicability

Opus subagent recommended vertex enumeration, claiming "max of quadratic on compact polytope is at a vertex." This is true for convex objectives (H positive semidefinite) but false for our indefinite H. Agent presented it to Jörn as "an interesting third approach worth exploring" without checking whether H is definite. Jörn caught it.

**Pattern:** Trusting subagent mathematical claims without domain-specific verification. The subagent had correct theorems but applied them to the wrong case. Fix: when a subagent makes a mathematical recommendation, check whether the hypotheses apply to the specific problem before presenting.

### 2026-04-02 — Presented fake objections to technology choices in database design

During format comparison (JSONL vs SQLite vs Arrow), agent presented several false or irrelevant objections:
- "Large dependencies slow builds" → Rust caches compiled crates; not an ongoing cost
- "Version conflicts are a risk" → Either versions resolve or they don't; one-time check
- "C dependencies are hard to install" → Devcontainer handles this with one Dockerfile line
- "Agents would need to navigate Arrow's complex API" → The API is encapsulated; agents write the DB module once, consumers use our thin API

Each wrong claim cost a round trip for Jörn to debunk. Total: ~5 unnecessary exchanges.

**Pattern:** Rationalizing a preferred choice (JSONL) with unverified claims about alternatives. The actual differentiators (data shape fit, git diffability, Python interop) were real but modest. Fix: before presenting a pro/con for a technology, verify the claim is actually true. Don't manufacture disadvantages for options you don't prefer.

### 2026-04-02 — Rambled when short answer was obvious

Jörn asked "should we cache vertices?" Agent wrote 3 paragraphs weighing tradeoffs. The answer: "Yes — 10ms × 900K polytopes = too slow to recompute on every load." Jörn said "this is a rambling answer?"

Later, agent wrote another multi-paragraph response about whether to store f64 dual vertices. Jörn: "sorry are you still asking whether to pay 10h cost whenever we want to load/stream the polytopes?"

**Pattern:** When the arithmetic gives a clear answer, state the arithmetic and the conclusion. Don't hedge with "the question is whether..." paragraphs.

### 2026-04-02 — Converged on design before presenting option space

After one round of exploration, agent wrote a detailed concrete plan and called ExitPlanMode. Jörn rejected: "I'd like to see more discussion of different approaches." Had to redo as a broad comparison covering 6 format options, 4 architectures, 4 key strategies, etc.

**Pattern:** "Help me plan" means "explore the design space with me," not "propose a solution for my approval." Present options and tradeoffs first, converge after Jörn has shaped the direction.
