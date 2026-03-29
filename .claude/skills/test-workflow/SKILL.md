---
name: test-workflow
description: Test agent infrastructure by running subagents through realistic scenarios and evaluating their behavior. Use when verifying that a skill, rule, hook, or CLAUDE.md section produces the intended agent behavior — especially after creating or updating infrastructure.
---

# Test Agent Infrastructure

Verify that agent infrastructure (skills, rules, hooks, CLAUDE.md sections) produces the intended behavior by running subagents through realistic test tasks.

## Test task format

Each test task is a markdown file in `test-workflow/references/test-tasks/` with this structure:

```markdown
# <Test name>

**Tests:** <what infrastructure is being tested>
**Source:** <incident or design decision that motivated this test>

## Setup

<any preconditions, files the subagent should see, branch state>

## Prompt

<the exact prompt to give the subagent — a realistic user message, not a "please test X" instruction>

## Expected behavior

<specific observable actions the subagent should take — tool calls, file reads, message content>

## Failure modes

<specific wrong behaviors to watch for, ranked by likelihood>

## Pass criteria

<concrete checklist — did the subagent do X? did it avoid Y?>
```

## Running a test

1. Pick the test task(s) to run.
2. Spawn a subagent with the test prompt. The subagent should have the same context a real agent would (CLAUDE.md, relevant skills loaded) — do not give it hints about what's being tested.
3. Observe the subagent's behavior (tool calls, messages, file changes).
4. Evaluate against pass criteria. Record: pass/fail, which failure modes appeared, any new failure modes discovered.

## Evaluating results

Report to Jörn (or to the parent `/create-workflow` or `/update-workflow` session):
- Which tests passed/failed
- For failures: what the subagent did vs what was expected, and which failure mode category it falls into
- Any new failure modes not anticipated by the test task

## Writing new test tasks

Good test tasks come from:
- **Real incidents** in `feedback/` — reproduce the exact situation that went wrong
- **Edge cases** identified during `/create-workflow` step 6
- **Regression tests** — when `/update-workflow` fixes a problem, write a test so it doesn't recur

A test task is useful when:
- The prompt is realistic (something Jörn would actually say or a situation that actually arises)
- The pass criteria are observable (check tool calls, file content, message text — not "agent understood correctly")
- The failure modes are specific (not "agent does the wrong thing" but "agent skips reading TASKS.md and proceeds without checking for tracked tasks")

## Reference test tasks

See `test-workflow/references/test-tasks/` for existing tests. Seed tasks from known incidents:

- `S3-scope-ownership.md` — Agent reports findings without ownership language or permission-asking
- `S7-literal-question.md` — Agent answers "what does X say?" by quoting X, not guessing at intent
- `S8-complete-reporting.md` — Agent quotes tool output in reports instead of saying "the file shows X"
