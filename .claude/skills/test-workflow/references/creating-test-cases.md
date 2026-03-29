# Creating Test Cases for Agent Infrastructure

Reference doc for when you need to create a new test case — typically during post-mortem (item 6) or when Jörn spots a good candidate during a session.

## Principle: real data over synthetic scenarios

Prefer real input→output pairs from this project's history over made-up prompts. Real data is more reliable because:
- The prompt is realistic (it actually happened)
- The failure/success mode is demonstrated (we have the actual output)
- The evaluation is concrete (we know what was wrong/right about the output)

Synthetic tests (made-up prompts designed to trigger a specific behavior) are less reliable — they may not trigger the failure mode in practice, and the evaluation criteria tend to be vague.

## Sources for test cases

### 1. JSONL session logs (best for communication/interaction tests)

**Location:** `~/.claude/projects/-workspaces-msc-math/*.jsonl`
**Corpus:** ~380 sessions, 700 MB, Feb 8 – Mar 29 2026.

**Format:** Each line is a JSON object with:
- `type`: "user" | "assistant" | "queue-operation" | "file-history-snapshot"
- `uuid` / `parentUuid`: linked list threading
- `message.content`: list of content blocks (`text`, `thinking`, `tool_use`, `tool_result`)
- `requestId`: groups blocks from one API call

**How to find incidents:**
- `feedback/` files have grep strings pointing to specific incidents (e.g., `grep "what does your agent written prompt even say" <session>.jsonl`)
- Walk the `parentUuid` chain forward/backward from the hit to reconstruct the conversation window
- Human messages: `type == "user"` where content has a `text` block (not just `tool_result`)
- One agent turn spans multiple JSONL lines — group by `requestId`

**Limitation:** The system prompt (CLAUDE.md, skills) isn't in the JSONL. Use `git log` to find what CLAUDE.md looked like at the session's timestamp.

### 2. Git history (best for "write code/math" integration tests)

**Corpus:** ~1,388 commits, ~30 merge commits with clear branch names.

**How to find candidates:**
- `git log --oneline --merges` — merge commits have descriptive messages and clear before/after states
- `git log --oneline -- '*/math.tex'` — math.tex additions
- `git log --oneline -- handoffs/` — tasks with explicit specs
- Look for commits where a handoff file describes the task and the diff shows the result

**Setup for a test case:**
- **Simple:** extract just the task-relevant input files at the old commit + the handoff/task description. Include them as fixtures in the test case folder.
- **Full:** `git clone` at the old commit, overlay current agent infra (CLAUDE.md, `.claude/`). More realistic but heavier.

**Strong candidates already identified (2026-03-29):**
1. **`8a685c4` — "Write math.tex from code"**: 73 Rust files → 3 math.tex files. Handoff at `handoffs/session-crate-math-tex.md` is the spec. Exercises formalization, code reading, cross-referencing, writing conventions.
2. **`707492b` — "Write smoothness proofs from experiment data"**: logbook observations → 4 formalized results. More creative/mathematical.
3. **`8dcc7c4` — "Extract shared code to library"**: 4 experiments with duplicated code → deduplicated library. Classic refactoring, testable by compilation.

### 3. Live sessions via post-mortem (ongoing collection)

Post-mortem item 6 asks agents to identify regression test candidates — both failures and successes. This is the primary ongoing source. When Jörn spots something during a session, he can also flag it directly.

## Timeline: how far back is useful?

- **Before Mar 12, 2026:** substantially different conventions (rules not skills, multiple crates, no logbooks). Not useful for testing current infrastructure.
- **Mar 12–19:** partially compatible (skills exist, experiments lack logbooks).
- **Mar 19 onward ("modern era"):** logbooks, math.tex, single crate, skills system. Test cases from here are relevant.
- **Mar 25 onward:** current CLAUDE.md rewritten from scratch. Strictest match to current conventions.

## Test case format

Defined in `/test-workflow` SKILL.md. The key fields:

```markdown
# <Test name>

**Tests:** <what infrastructure is being tested>
**Source:** <incident or decision that motivated this test — link to session/commit>

## Setup
<files the subagent should see — either real repo or extracted fixtures>

## Prompt
<the exact prompt — from a real session or handoff file, not invented>

## Expected behavior
<specific observable actions — tool calls, file content, message content>

## Failure modes
<specific wrong behaviors to watch for, from the real incident>

## Pass criteria
<concrete checklist>
```

**Key quality criteria:**
- Prompt comes from a real session or handoff, not invented
- Pass criteria are observable (check tool calls, file content, message text — not "agent understood correctly")
- Failure modes are specific (not "agent does the wrong thing" but "agent creates a new file instead of searching for the existing one")

## What makes a good test case

- A concrete input→output pair where we know what "good" looks like
- Multiple conventions/workflows exercised simultaneously (integration > unit)
- The failure mode (if testing a failure) actually happened and cost real time
- The success mode (if testing a success) was non-obvious — agent handled a tricky situation well
- Evaluation can be at least partially automated (grep for phrases, check tool call order, verify file content)
