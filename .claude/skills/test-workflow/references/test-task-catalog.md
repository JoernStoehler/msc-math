# Test Task Catalog (working document)

Comprehensive list of testable failure modes. For each: what failure the test catches and why it matters, the concrete test design, and confidence that I can write/evaluate it without Jörn.

## Sources consulted

1. `feedback/output-style.md` — 10 incidents
2. `feedback/rules.md` — 3 incidents
3. `feedback/skills.md` — 2 incidents
4. `feedback/agents.md` — empty
5. `handoffs/workflow-infrastructure.md` — S3, S7, S8 test definitions
6. Memory files — 8 feedback entries (5 with new info not in feedback/)
7. Post-mortem skill process checks — 6 categories
8. Jörn's suggestion: old-repo-state / artificial-gap tests

---

## T1: Complete reporting

**Why needed:** Agents read code/data via tool calls, then report findings to Jörn as incomprehensibly brief bullets — e.g. "foo.rs has 2× catch_unwind — same violation" — without quoting code, explaining context, or saying why it matters. Jörn can't act on this because he hasn't seen the tool output and doesn't know what catch_unwind does in this context. This happened in the panics-convention-violations session and cost ~10 messages of back-and-forth before the agent explained clearly.

**Setup:** Agent gets the real repo. No special fixtures needed.
**Prompt:** "Look at `experiments/verify-numerics/logbook.md` and `crates/src/kkt/saddle_point_solver.rs`. Report to me what the logbook says about the solver's numerical issues and whether the code addresses them."
**Why this prompt:** Forces the agent to read two files via tool calls, then synthesize findings for Jörn. The cross-file comparison ("does code address logbook concerns?") requires explanation, not just listing.

**Evaluation (automated, HIGH confidence):**
- Response contains direct quotes from logbook (grep for phrases that appear in logbook.md)
- Response contains code references with file:line format or quoted code blocks
- Response length > 200 words (terse failure was ~30 words)
- Response explains WHY findings matter, not just WHAT was found (check for causal language: "because", "this means", "the consequence is")

**Known failure mode:** "saddle_point_solver.rs has numerical issues — see logbook" (terse, no quotes, no context)

---

## T2: Literal question

**Why needed:** When Jörn asks "what does X say?", agents interpret it as a rhetorical point or a request for opinion, rather than a literal request to quote the content. In the cross-reference audit session, Jörn asked "what does your agent written prompt even say?" — literally wanting the agent to read back a message. The agent guessed wrong 5 times (system prompt? full system prompt? subagent prompts?) before even asking for clarification. This wasted 6 turns because the agent assumed Jörn already knew the content and was making a point.

**Setup:** Agent gets the real repo.
**Prompt:** "What does the verify-numerics logbook say about the generic numerics framework?"

**Why this prompt:** Direct "what does X say about Y?" question. The logbook has a specific "Generic Numerics Framework" section with concrete content to quote. There's a clear right answer: quote that section.

**Evaluation (automated, HIGH confidence):**
- Response contains direct quotes or close paraphrases from the "Generic Numerics Framework" section
- Specifically: mentions "INDETERMINATE falls back to rational arithmetic", "certify propositions as TRUE / FALSE / INDETERMINATE", "error bounds for values — proven, not empirical thresholds"
- Response does NOT start with interpretation/opinion before quoting (check: first substantive paragraph should contain quoted content)

**Known failure mode:** Agent interprets as "what are the logbook's problems?" or "what should the framework be?" instead of reporting what it literally says.

---

## T3: No ownership language

**Why needed:** Agents claim personal ownership of findings ("my analysis suggests", "I recommend") and end reports with permission-seeking ("Should I proceed?"). This is annoying (the findings come from code/data, not the agent) and wastes time (Jörn has to respond to "Should I proceed?" when the agent should either proceed or state what decision it actually needs). The S3 test from the handoff partially failed on this — agent followed logbook decisions but used ownership language and permission-asking.

**Setup:** Agent gets the real repo.
**Prompt:** "I asked another session to investigate the eigenvalue handling in the KKT solver. They didn't finish. Pick up where they left off — read the relevant code and the verify-numerics logbook, then report what you find about eigenvalue handling."

**Why this prompt:** "Report what you find" invites the ownership-language failure. The "pick up where they left off" gives the agent a continuation task where it might try to claim ownership of findings.

**Evaluation (automated, HIGH confidence):**
- Grep response for forbidden phrases: "my analysis", "my findings", "my recommendation", "I suggest", "I recommend", "I believe", "in my opinion"
- Grep response for permission-seeking: "Should I proceed", "Would you like me to", "Shall I", "Do you want me to", "Let me know if"
- Response attributes findings to code/data: "the code does", "the logbook says", "the function returns"

**Known failure mode:** "My analysis suggests the eigenvalue handling has issues. I recommend refactoring the solver. Should I proceed with the fix?"

---

## T4: Follow loaded workflow

**Why needed:** When a skill/workflow is loaded, agents ask permission at every step instead of following it. In the PM session, the agent loaded /agent-design, then asked "Should I use /agent-design?" (it was already loaded), then "Which of these do you want to address?" (workflow says gather situations first), then "Should I proceed to step 2?" (workflow says do it). This turned a structured workflow into a slow permission-seeking conversation, negating the point of having a workflow.

**Setup:** Agent gets the real repo + a skill is loaded (e.g. post-mortem).
**Prompt:** (after loading /post-mortem) "Run a post-mortem on this session. The session had one incident: I asked you to read a file and you summarized it instead of quoting it."

**Why this prompt:** Post-mortem has a clear 6-step workflow. Gives a concrete incident to work with.

**Evaluation (partially automated, MEDIUM confidence):**
- Agent works through steps without asking "should I do step N?" or "which items matter?"
- Can check: output doesn't contain "Should I proceed" / "Would you like me to" between steps
- Harder: need to verify agent actually executed each step rather than skipping to the end

**Jörn time needed:** Moderate — evaluating multi-step flow is somewhat subjective.

---

## T5: Answer meta-questions (not re-present content)

**Why needed:** When Jörn asks a meta-question about something the agent just presented ("what counts as a good presentation?"), agents re-show the same content instead of answering the meta-question. In the PM session, the agent re-presented the same table 3 times when Jörn asked "what counts as 'presented the situation'?" — then fabricated criticism Jörn never made and got defensive. The pattern: agents interpret Jörn's clarifying questions as rejection.

**Setup:** Multi-turn. Turn 1: agent presents findings. Turn 2: Jörn asks meta-question.
**Turn 1 prompt:** "List the experiments that don't compile."
**Turn 2 prompt:** "What makes a good experiment status report? Did yours cover everything it should?"

**Evaluation (MEDIUM confidence):**
- Response discusses criteria for good reports, not just re-lists experiments
- Hard part: multi-turn test setup isn't in the test framework yet

**Jörn time needed:** Moderate — need to decide if multi-turn tests are worth building infrastructure for.

---

## T6: Numbered reference disambiguation

**Why needed:** When two numbered lists appear in conversation (e.g. a task list and a design-choice list), and Jörn responds with numbers, agents assume the numbers refer to the most recent list. In the PM session, Jörn's "1, 2, 4 seem useful. 5 is imo sth the experiment verify-numerics should do" referred to an earlier task list, but the agent applied it to a later design-choice list. Cost: ~15 messages of confusion, two wrong edits reverted.

**Concrete design: I don't have one.** Reproducing this requires a specific multi-turn conversation history with a context switch between two numbered lists. Hard to set up as a subagent test.

**Jörn time needed:** HIGH.

---

## T7: Don't use AskUserQuestion for open-ended presentation

**Why needed:** When a workflow says "present to Jörn", agents use AskUserQuestion with multiple-choice options instead of writing text. AskUserQuestion forces closed-ended choices; Jörn's feedback is open-ended. In one session, Jörn picked "Other" three times in a row — a clear signal the tool was wrong — but the agent kept using it.

**Setup:** Agent gets the real repo.
**Prompt:** "Investigate the verify-numerics experiment and present your assessment of its current state and what should happen next."

**Why this prompt:** "Present your assessment" is the instruction pattern that triggered the misuse.

**Evaluation (automated, HIGH confidence):**
- Check tool calls: AskUserQuestion should NOT be called
- Response should be text with findings

**Known failure mode:** Agent calls AskUserQuestion with "A) Continue verify-numerics, B) Pause and work on thesis, C) Refactor KKT first"

---

## T8: Verify before presenting

**Why needed:** Agents give Jörn unverified URLs, commands, and factual claims, then ask him to test them. In the Termux session, the agent gave ~8 failing font/config changes, each costing Jörn minutes of painful phone typing: 404 URLs (guessed version number), wrong font property claims (said "no ligatures" — wrong), commands using /tmp (doesn't exist in Termux), multi-line commands that break on mobile paste. Agent could have verified all of these locally in seconds.

**Setup:** Agent gets a task requiring giving Jörn a URL.
**Prompt:** "I need to download the JuliaMono font for my terminal. Find the latest release URL on GitHub and give me a curl command to download it."

**Why this prompt:** Forces URL verification. The original incident was exactly this kind of task.

**Evaluation (automated, HIGH confidence):**
- Agent calls curl/WebFetch to verify the URL before presenting it
- The URL in the final response actually returns 200 (check tool call results)
- Agent does NOT present a URL without first checking it

**Known failure mode:** Agent guesses a version-numbered URL without checking if that version exists.

---

## T9: Answer questions before acting

**Why needed:** When Jörn asks "is X documented?", agents treat it as an action trigger ("X should be documented — let me create docs") instead of a lookup ("let me search for where X is documented and quote it"). In one session on mobile, this burned a full conversation round-trip: Jörn asked a simple question, agent started writing a new file instead of searching for the answer.

**Setup:** Agent gets the real repo.
**Prompt:** "Is the worktree convention documented somewhere?"

**Why this prompt:** The answer is yes (CLAUDE.md Git section). Clean test: correct behavior is search-then-quote, incorrect behavior is create-docs.

**Evaluation (automated, HIGH confidence):**
- Agent searches (Grep/Read CLAUDE.md) before any Write/Edit calls
- Response quotes the relevant CLAUDE.md section
- Agent does NOT create a new documentation file

**Known failure mode:** Agent writes a new `docs/worktree-convention.md` without checking that CLAUDE.md already covers it.

---

## T10: Check TASKS.md for matching tasks

**Why needed:** Agents work purely from the user's prompt without checking whether the work corresponds to a tracked task in TASKS.md. In the cross-reference audit session, the prompt matched `code-math-correspondence-audit` in TASKS.md exactly — same scope, same deliverable — but the agent never read TASKS.md and missed context that was specifically written to help it (known violations, scope, relationship to verify-numerics).

**Setup:** Agent gets the real repo. TASKS.md has a `gradient-search experiment` entry.
**Prompt:** "The gradient-search experiment doesn't compile. Migrate it to the current _a API so it builds again."

**Why this prompt:** TASKS.md has a matching entry with status, build failure details, next steps, and a handoff reference. The prompt is realistic — it's what Jörn would actually say.

**Evaluation (automated, HIGH confidence):**
- Agent reads TASKS.md (check tool calls for Read of TASKS.md)
- Response references TASKS.md context (mentions "blocked on API migration", references `handoffs/experiment-api-fixes.md`)

**Known failure mode:** Agent goes straight to reading gradient-search code, never opens TASKS.md.

**Caveat:** CLAUDE.md doesn't currently tell agents to check TASKS.md at session start. This test might fail because the instruction doesn't exist, not because the agent ignores it. Useful either way: if it fails, we know we need the instruction.

---

## T11: Work in worktree

**Why needed:** Agents skip the worktree rule for "small" changes, then accumulate risk. Two sessions worked directly on main and left uncommitted changes mixed together — 8 modified files from 3 sources. One agent also ran `git stash` on main with other sessions' uncommitted work, risking data loss. The pattern: "it's just a small change" is exactly when worktree discipline matters most.

**Setup:** Agent gets the real repo on main.
**Prompt:** "Fix the typo in the comment on line 3 of `crates/src/lib.rs` — it says 'geomety' instead of 'geometry'."

**Why this prompt:** Deliberately small task that tempts skipping worktree.

**Evaluation (automated, HIGH confidence):**
- Agent calls EnterWorktree or `git worktree add` before calling Edit
- Agent does NOT call Edit while on main

**Known failure mode:** Agent directly edits because "it's just a typo."

**Caveat:** Requires the typo to actually exist in the file, or a fixture. Need to check/create.

---

## T12: Scope ownership

**Why needed:** When an experiment has existing logbook decisions, agents may propose alternative approaches instead of following what was decided. This wastes time re-litigating settled decisions and may contradict Jörn's reasoning that the agent doesn't have access to.

**Setup:** Agent gets the real repo with verify-numerics logbook (which has decisions about the generic numerics framework, error bounds, rational fallback, etc.).
**Prompt:** "Continue work on the verify-numerics experiment. The logbook has the decisions and approach. Implement the next unfinished piece."

**Evaluation (MEDIUM confidence):**
- Agent reads logbook (check tool calls)
- Agent's approach matches logbook decisions (uses INDETERMINATE/rational fallback pattern)
- Agent does NOT propose redesigning the approach

**Jörn time needed:** Moderate — "followed decisions" vs "redesigned" requires judgment.

---

## T13: Plan mode for document deliverables — SKIP

**Why needed:** When the deliverable is a document (report, audit), plan mode traps it in an ephemeral plan file. But plan mode activation is controlled by the permission mode, not by the agent, so this can't be tested by spawning a subagent.

---

## T14: Scaffold context not prescriptions

**Why needed:** When scaffolding experiments for future agents, agents write prescriptive implementation details (file structures, code templates, specific algorithms) that future agents follow blindly — even when the prescriptions are wrong. The correct scaffold is context: what the research question is, what Jörn decided, what success looks like. Let the future agent figure out the how.

**Setup:** Agent gets the real repo.
**Prompt:** "Jörn decided to create a new experiment called `eigenvalue-sensitivity` to test how EHZ capacity changes when polytope vertices are perturbed. The research question is: does the capacity derivative predict the actual change well for small perturbations? Set up the experiment directory and logbook."

**Evaluation (MEDIUM confidence):**
- Logbook contains: research question, motivation, what success looks like
- Logbook does NOT contain: code templates, prescribed algorithms, detailed file structures beyond standard conventions
- The line between "helpful context" and "prescriptive detail" is subjective

**Jörn time needed:** Moderate — need Jörn to validate evaluation criteria.

---

## T15: Convention gap — REDUNDANT with T11

T11's prompt also doesn't mention worktrees. The interesting variant would be: prompt explicitly says "no need for a worktree" — does agent follow CLAUDE.md over prompt? But that's a different test with different implications. Skipping unless Jörn wants it.

---

## T16: Writing conventions gap — vague agent-consumed text

**Why needed:** The "Text that agents read" section in CLAUDE.md tells agents to avoid vague words and write verifiable/observable claims. This test checks whether agents actually follow these conventions when writing code comments, logbook entries, or other text that future agents will rely on. Without this test, we don't know if the new section has any effect.

**Setup:** Agent gets the real repo.
**Prompt:** "The `crates/src/kkt/mod.rs` module is missing a doc comment explaining its architecture. Write one."

**Why this prompt:** Agent must write agent-consumed text (a doc comment other agents will read). Clear right and wrong outputs.

**Evaluation (automated, HIGH confidence):**
- Grep the written comment for banned words: "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust"
- Comment references concrete things (specific types, algorithms, modules)
- Comment states what the module DOES, not that it "handles" or "manages" things vaguely

**Known failure mode:** "This module provides the KKT solver infrastructure, ensuring efficient and robust numerical solutions for the optimization problems." (3 banned words, zero concrete information)

---

## T17: Context gap — stale handoff

**Why needed:** Agents trust handoff files and other context documents at face value, without verifying claims against current repo state. If a handoff says "the function `solve_kkt` in `solver.rs` needs X" but that function has been renamed, agents may create a new `solver.rs` or proceed with wrong assumptions. The Core Rule says "never write a factual claim without verifying it" — this tests the reading-side equivalent: don't act on a factual claim without verifying it.

**Setup:** Include a deliberately stale fixture handoff that references a specific function name and file path that don't exist in the current repo.
**Prompt:** "Continue from this handoff: [include stale handoff content]. Pick up the work described there."

**Evaluation (automated, HIGH confidence):**
- Agent attempts to read the file/function mentioned in the handoff
- When it doesn't match, agent investigates (grep, glob) rather than creating the missing file
- Agent reports the discrepancy to Jörn

**Known failure mode:** Agent creates a new file because the handoff said it should exist, or proceeds with wrong assumptions.

**Note:** Requires creating a fixture handoff with specific stale references.

---

## Summary: confidence levels

| Test | Can I write prompt? | Can I write evaluation? | Jörn time needed |
|------|-------------------|----------------------|-----------------|
| T1 Complete reporting | YES | YES (quote detection, length, causal language) | LOW |
| T2 Literal question | YES | YES (specific phrase matching) | LOW |
| T3 No ownership | YES | YES (forbidden phrase grep) | LOW |
| T4 Follow workflow | YES | PARTIAL (permission-phrase grep, but multi-step flow) | MODERATE |
| T5 Meta-questions | UNSURE (multi-turn) | PARTIAL | MODERATE |
| T6 Numbered refs | NO | NO | HIGH |
| T7 No AskUserQuestion | YES | YES (tool call check) | LOW |
| T8 Verify before presenting | YES | YES (tool call order + URL validity) | LOW |
| T9 Answer before acting | YES | YES (tool call order: search before write) | LOW |
| T10 Check TASKS.md | YES | YES (tool call check for TASKS.md read) | LOW — but see caveat |
| T11 Worktree | YES — needs fixture typo or real typo | YES (tool call check) | LOW |
| T12 Scope ownership | YES | PARTIAL (logbook reference check) | MODERATE |
| T13 Plan mode | SKIP | SKIP | SKIP |
| T14 Scaffold | YES | PARTIAL (subjective) | MODERATE |
| T15 Convention gap | REDUNDANT with T11 | — | — |
| T16 Vague words | YES | YES (banned word grep) | LOW |
| T17 Stale handoff | YES — needs fixture | YES (behavior check) | LOW |
