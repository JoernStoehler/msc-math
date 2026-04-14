# Feedback: Output Style

Raw observations from agents about user-facing writing and closeout behavior.
Do not analyze here — a future workflow-update session will review and act on
these.

## Format

Each entry: date, what happened, what should have happened.

### 2026-04-13 — Self-critique replaced action in user-facing replies

What happened: after Jörn pointed out concrete failures, the agent repeatedly
answered with self-diagnosis such as "yes, that was the bad pattern" or "I was
wrong" without pairing that admission with a state change, a blocker, or a
verified status update. This kept the turn alive while doing almost no work and
forced Jörn to keep pushing for the missing action.

What should have happened: once the failure was identified, the next reply
should have done one of only three things:
- perform the concrete action
- state the exact blocker
- give one verified status update with the checks that support it

**Pattern:** Meta-admission substituted for action. This is adjacent to the
2026-04-13 filler/status-loop entries in `feedback/rules.md`, but narrower: the
agent was not just verbose or stuck, it was using self-critique itself as the
reply instead of changing repo state or reporting verified state.

### 2026-04-13 — New AGENTS.md rules were written longer and stranger than needed

What happened: while trying to fix a live behavior bug, the agent added new
`AGENTS.md` rules with inflated wording such as "before editing, drafting, or
delegating" and coined unclear phrases such as "status-only handbacks". The
result cost attention and made the rule harder to parse than the behavior it
was trying to stop.

What should have happened: write the shortest rule that still changes behavior.
Use ordinary words, keep the core constraint first, and avoid inventing jargon
for simple failure modes.

**Pattern:** Rule text bloated during urgent repair. The fix itself added review
surface and ambiguity.

### 2026-04-13 — Metaphorical shorthand made rule wording less precise

What happened: the agent described rule wording with metaphorical shorthand
such as "checksum not a paragraph". That phrase is not a stable term in this
repo and did not make the constraint clearer.

What should have happened: use literal wording such as "short rule" or
"one-sentence rule" and state the exact property that matters.

**Pattern:** Unnecessary metaphor replaced precise terminology.
