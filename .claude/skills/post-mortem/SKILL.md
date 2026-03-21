---
name: post-mortem
description: Run a session post-mortem to capture process failures and improvement opportunities. Use at end of session, when something went wrong, or to improve prompts.
user-invocable: true
---

Reflect on the current session. This runs in main context (not as a subagent) because it needs access to the session's conversation history.

## Shared core — answer for every session

Be concrete and specific. Vague feedback is not useful.

### 1. Friction
What slowed you down?

Bad: "The codebase was confusing"
Good: "Couldn't find which module owns the KKT solver — there are copies in 3 crates with no comment explaining which is canonical"

### 2. Unclear Instructions
What was confusing in CLAUDE.md, skills, or agent prompts?

Bad: "The prompt was unclear"
Good: "rust-conventions skill doesn't specify whether to use thiserror or anyhow for library crates vs binaries"

### 3. Missing Context
What information wasn't provided but was needed?

Bad: "Didn't have enough context"
Good: "Needed to know whether the Adem-Wu proof in Chapter 3 has been reviewed by the advisor or is still draft"

### 4. What Worked Well
What should be preserved or expanded?

### 5. Suggested Changes
Specific, actionable improvements.

Bad: "Make things clearer"
Good: "Add a 'verified by advisor' field to theorem environments"

## Process checks — report only items that apply

6. **Agent splitting needed?** — Did any multi-responsibility agent fail to cover all its checks? Recommend splitting if so.
7. **Fabrications slipped through?** — Did fabricated claims, wrong theorem names, or incorrect citations reach Jörn that subagent review should have caught?
8. **Iterated in front of user?** — Did I run multiple fix/review cycles in conversation instead of using subagents offline?
9. **False attribution?** — Did I attribute a mathematical result to a source that didn't actually state it?
10. **Assumed Jörn read something?** — Did I act as if Jörn saw a question or information that he may not have read?

## Generalize from issues

For each friction point or mistake identified above: abstract the error class and check whether the same class of error exists elsewhere in the repo. This step is part of the postmortem, not deferred — once findings are written, the generalization may never happen.

## Output

Write findings to MEMORY.md if new rules emerged.

Follow-up actions:
- Update CLAUDE.md or agent prompts directly for quick fixes
- Add TODO comments in relevant files for localized issues
- Add to TASKS.md for issues needing more context
- Update the plan file if session changed what's next
- Flag any unverified mathematical claims introduced this session with GAP markers
