---
name: triage
description: Use when working with GitHub issues, triaging the issue board, scoping tasks, or writing issue content. Contains the issue lifecycle, template sections, authoring guidelines, and triage session workflow.
---

# Issue management and triage

## Issue lifecycle

Issues go through a structured lifecycle so that agents start work only on well-scoped tasks, and failed sessions produce reusable learnings rather than wasted effort.

### Stages

1. **Capture** (label: `draft`) — An idea comes up. Agent creates an issue with at least Goal and Context filled in. Most sections may be rough or empty. Creating issues is cheap.

2. **Refine** (label: `draft`) — Over one or more triage sessions, Jörn and agent discuss. Agent proposes edits to the issue body in chat; Jörn steers; agent publishes. Sections get filled in: Background, Deliverable, Scope, Sources, Acceptance criteria. Open questions get resolved — answers move into the appropriate section.

3. **Approve** (label: `approved`) — Jörn judges the issue is ready. The goal is worth pursuing, scope is clear, open questions resolved. The issue is now the prompt for an agent session.

4. **Session** (label: `in-progress`) — Agent reads the issue, follows the session workflow. Jörn provides mathematical direction during the session. If scope turns out to be wrong or the task is blocked, agent tells Jörn immediately — they re-scope together or abort. Agent does not silently produce something different from what was asked.

5. **PR + merge** — Jörn creates PR, merges. Mechanical step.

6. **Close** (label: `done`) — Follow-up ideas captured as new issues.

If a session fails: agent reports what it tried and learned. Issue goes back to `draft` for re-scoping. No work is lost — the branch exists.

### Issue template

These sections give an agent the information it needs to complete a task autonomously, and give Jörn the structure to review scope quickly.

Issues use these sections:

- **Goal** — What this achieves for the thesis. One or two sentences.
- **Background** — Domain knowledge needed. Link to papers, files, issues — don't repeat their content.
- **Context** — Why this constitutes progress toward the thesis. What it unblocks.
- **Deliverable** — What the agent produces. Describe substance, not form — the agent decides files and structure.
- **Scope** — What's in, what's out, and why. Each exclusion has a reason.
- **Sources** — Papers, code, specs. Flag trustworthiness.
- **Acceptance criteria** — Measurable. Two kinds: external (serves the project) and internal (quality bar).
- **Notes** — Preliminary findings, known risks, suggested sub-issues.
- **Open questions** — Uncertainties needing Jörn's input. Resolve via edits; once empty and Jörn approves, the task is ready.

### Authoring guidelines

Known failure modes when writing issues:

- **Unclear wording.** Prefer an extra sentence over a vague word. If a term could mean two things, say which.
- **Misleading confidence.** If something is unreviewed, mark it — and mark EVERY such item. Labeling one item "unreviewed" implies the rest ARE reviewed.
- **False facts.** Don't claim relationships unless verified. Don't call X a "specialization" of Y unless it literally is.
- **Misrepresenting process.** Don't claim something is approved when it isn't. Represent decision state accurately.
- **Over-constraining implementation.** Don't prescribe file names or structure. Constrain only what has external consequences (API surfaces, conventions, mathematical correctness).

## Triage sessions

Triage keeps the issue board accurate and actionable, preventing agents from working on stale or blocked tasks.

The issue board should reflect reality after the session: audit open issues against repo state, close completed ones, capture new work, refine drafts, prioritize, prepare top items for approval.

Decision authority during triage — agent's call: reading, summarizing, drafting proposed edits (shown to Jörn in chat, not published directly). Jörn's call: whether an issue is worth pursuing, scope, priority, labeling `approved`, approving edits for publication.

Present findings in batches, not one issue at a time.
