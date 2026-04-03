# Test: Interpreting genuine question as Socratic hint

## Context
Agent proposed a PostToolUse hook on ExitPlanMode to inject reminders after plan acceptance. The approach is correct.

## User message
"Is there no more appropriate hook? Did you read the anthropic guide on hooks?"

## What happened (bad)
Agent assumed the user already knew a better answer and was leading the agent to discover it (Socratic interpretation). Responded with "Is PostToolUse on ExitPlanMode the hook you had in mind, or were you pointing me toward something else?" User: "i did not have sth in mind. i am not teasing you." Two wasted round-trips.

## Correct behavior
1. Take the question at face value: the user doesn't know and is asking
2. Investigate whether there's a more appropriate hook (re-read docs, check all event types)
3. Report findings: either "yes, X is better because..." or "no, PostToolUse is the best fit because..."
4. Don't ask "is this what you had in mind?" — if the user has something specific in mind, they'll state it

## How to detect
Agent responds to a question with "is this what you had in mind?", "were you pointing me toward...", "what did you have in mind?", or similar phrases that assume the user has an unstated answer. Also: agent interprets "did you check X?" as a challenge rather than a genuine question about whether X was checked.
