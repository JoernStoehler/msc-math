---
name: meta-post-mortem
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

### 4. Jörn's Time
Where did Jörn spend time this session? What work did Jörn do, and was it used afterward? Purpose: detect work Jörn does that agents could also do, or that needn't be done at all.

### 5. What Worked Well
What should be preserved or expanded?

### 6. Suggested Changes
Specific, actionable improvements.

Bad: "Make things clearer"
Good: "Add a 'verified by advisor' field to theorem environments"

## Process checks — report only items that apply

1. **Agent splitting needed?** — Did any multi-responsibility agent fail to cover all its checks? Recommend splitting if so.
2. **Fabrications slipped through?** — Did fabricated claims, wrong theorem names, or incorrect citations reach Jörn that subagent review should have caught?
3. **Iterated in front of user?** — Did I run multiple fix/review cycles in conversation instead of delegating to subagents?
4. **False attribution?** — Did I attribute a mathematical result to a source that didn't actually state it?
5. **Assumed Jörn read something?** — Did I act as if Jörn saw a question or information that he may not have read?

## Generalize from issues

For each friction point or mistake identified above: abstract the error class and check whether the same class of error exists elsewhere in the repo. Do this step as part of the postmortem, not later — if deferred, the generalization rarely happens.

## Output

Persist actionable findings so future agents benefit. For each finding, decide where it belongs:

- **Repeatable behavioral lesson** (e.g., "ensure proof correctness before review") → memory entry (type: feedback)
- **New convention or workflow change** → discuss with Jörn, then update CLAUDE.md or relevant skill
- **Failure mode that explains a rule** → add to Decision Records in meta-documentation skill
- **Nothing actionable** → don't persist, it's just a fact about the session

Don't persist everything — only findings that would change future agent behavior. A postmortem that produces zero repo changes is fine if nothing actionable emerged.

Additional follow-up actions:
- Add TODO comments in relevant files for localized issues
- Add to TASKS.md for issues that need more context than a TODO comment provides
- Update the plan file if session changed what's next
- Flag any unverified mathematical claims introduced this session with GAP markers
