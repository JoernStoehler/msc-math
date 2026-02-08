# Project board review ("triage")

Reference material for sessions where the task is to review and maintain the GitHub issue board.

## What a board review involves

The issue board should reflect reality after the session. Concretely:

1. **Audit** — Read all open issues. Check recent merges and closed issues. Compare what the board says vs. what the repo contains.
2. **Close** — Issues whose deliverable is already in main.
3. **Capture** — Work implied by the project goals or by recently completed work, but not yet tracked as an issue.
4. **Refine** — Issue bodies that are stale, incomplete, or don't match conventions. Rewrite using the issue template (`.github/ISSUE_TEMPLATE/task.md`).
5. **Prioritize** — Given dependencies and thesis timeline, what should be worked on next?
6. **Prepare** — Make top-priority issues session-ready: all template sections filled, open questions resolved, label → `approved`.

These steps don't need to happen in strict order — auditing often reveals things to capture or close, refining often surfaces open questions that change priority.

## Starting context

An agent doing board review needs broad, shallow context rather than deep knowledge of one topic:
- All open issues (titles, labels, bodies)
- Recently closed issues and merged PRs (what changed since last review)
- Current codebase state (what exists, what's a stub)
- The issue template and lifecycle doc (`docs/references/issue-lifecycle.md`)

## Decision authority

Agent's call:
- Reading and summarizing issue state
- Proposing closures, new issues, edits
- Writing/rewriting issue bodies to match conventions
- Running subagent clarity checks on refined issues

Jörn's call:
- Whether an issue is worth pursuing
- Scope boundaries (what's in, what's out)
- Priority order
- Labeling issues `approved`

## Workflow

Present findings in batches — a prioritized list of proposed actions that Jörn can approve, reject, or steer. Don't drip-feed one issue at a time; the overhead of context-switching between issues is lower when they're presented together.

For each issue being refined: check against the authoring guidelines in the issue template (false claims, over-constraining, misleading confidence, unclear wording, process misrepresentation).

When capturing new issues, a rough draft with just Goal and a few notes is fine — refinement happens iteratively across sessions, not all at once.

## Known pitfalls from past sessions

- Claiming relationships between components without verifying (e.g. "X determines Y", "X is a specialization of Y"). Check before asserting.
- Revising an issue body many times before Jörn has seen any version. Write one complete draft, link it, get feedback.
- Presenting tool output as if Jörn can see it. He sees only assistant text messages — present substance in prose or link to GitHub URLs.
- Asking questions that assume Jörn already read something. Either tell him it's ready to review, or ask only questions needed before writing.
