# Handoff: Feedback Triage — Tasks 3-6

## Background

A feedback triage session (plan: `/home/vscode/.claude/plans/clever-napping-wilkinson.md`) processed 60+ agent incidents from `feedback/`. Tasks 1-2 are merged to main:
1. Pre-merge skill rewrite (`.claude/skills/pre-merge/SKILL.md`)
2. CLAUDE.md edits (Core Rule, Making Decisions, Message Style, What to avoid)

Tasks 3-6 were executed autonomously after compaction and reverted. This handoff restarts them.

## What failed in previous attempts

- First attempt: agent executed all 4 tasks without review after compaction. Test cases, memory deletions, feedback deletions — all done autonomously. Reverted.
- Second attempt (this session): agent loaded context, designed solutions independently, tried to get Jörn to rubber-stamp. Derailed because agent treated Jörn as reviewer rather than domain expert.

**The agent has no expertise in agent infrastructure.** Jörn has observed hundreds of agent sessions and knows which failure modes matter, what makes a good test case, and which behavioral rules actually change agent behavior. The agent's job is to present raw material and execute Jörn's decisions, not to design solutions.

## Tasks

### Task 3: Post-mortem SKILL.md — "Save to" wording

**Problem:** `.claude/skills/post-mortem/SKILL.md` line 50 says "Save to `.claude/skills/test-workflow/references/test-tasks/`." An agent followed this literally and wrote an invalid test case (feedback/skills.md 2026-03-30). The post-mortem skill shouldn't write test cases directly — that's /test-workflow's job.

**What to do:** Present the current text (line 50 and surrounding context) to Jörn. He decides the replacement wording.

### Task 4: Write 3 test cases

**Problem:** The previous plan selected 3 incidents to turn into test cases. That selection was never reviewed by Jörn.

**What to do:**
1. Read the existing test cases in `.claude/skills/test-workflow/references/test-tasks/` to understand format and quality bar
2. Read the feedback files to understand the full set of incidents
3. Present candidate incidents to Jörn — he selects which become test cases
4. For each selected incident: draft the test case, present to Jörn for review, iterate until approved
5. Jörn knows what makes a good test case. Ask him.

### Task 5: Retire memory entries

**Problem:** The previous plan said "retire 6 promoted memory entries" — but "promoted" was the previous agent's unverified conclusion. Some of these memories may NOT be covered by current CLAUDE.md.

**What to do:**
1. For each of the 6 memories, present: the memory content AND the CLAUDE.md text that supposedly covers it
2. Jörn decides which are truly redundant and can be deleted
3. Only delete what Jörn explicitly approves

The 6 memories: `feedback_run_code_you_create.md`, `feedback_verify_before_presenting_to_jorn.md`, `feedback_narrate_decisions_not_corrections.md`, `feedback_compare_then_choose.md`, `feedback_evaluate_with_criteria.md`, `feedback_dont_minimize_edits.md`

### Task 6: Delete processed feedback entries

**Problem:** The previous plan said "delete processed feedback entries" without specifying which. Deleting feedback entries is irreversible information loss.

**What to do:**
1. Present each feedback file's entries to Jörn with a summary of what (if anything) has been done about each
2. Jörn decides which entries are processed and safe to delete
3. Only delete what Jörn explicitly approves

## Pre-existing state

`feedback/rules.md` has 2 uncommitted entries on main (2026-04-04: post-compaction autonomous execution, `rm` vs `trash-put`). Commit these first or include in the work.

## Files to read at session start

```
# The previous plan (for context on what was tried)
/home/vscode/.claude/plans/clever-napping-wilkinson.md

# Infrastructure being modified
.claude/skills/post-mortem/SKILL.md                          (task 3)
.claude/skills/test-workflow/SKILL.md                        (task 4 — format)
.claude/skills/test-workflow/references/test-tasks/*.md      (task 4 — examples)

# Source material
feedback/rules.md                                            (task 4, 6)
feedback/agents.md                                           (task 4, 6)
feedback/skills.md                                           (task 3, 6)
feedback/output-style.md                                     (task 6)

# Memories to evaluate
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_run_code_you_create.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_verify_before_presenting_to_jorn.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_narrate_decisions_not_corrections.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_compare_then_choose.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_evaluate_with_criteria.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/feedback_dont_minimize_edits.md
/home/vscode/.claude/projects/-workspaces-msc-math/memory/MEMORY.md
```

## Process

Use /update-workflow. Work through one task at a time. For each task: present raw material to Jörn, he decides the fix, you execute, he reviews. Do not design solutions independently.
