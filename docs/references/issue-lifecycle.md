# Issue lifecycle

How ideas go from a remark in conversation to completed, merged work.

For the issue template, see `.github/ISSUE_TEMPLATE/task.md`.

## Stages

### Capture

An idea comes up during a triage session, a work session, or any conversation. An agent creates a GitHub issue using the task template. Most sections are rough or empty — Goal and Context are filled in, Open questions lists what we don't know yet. Label: `draft`.

Creating issues is cheap. It's better to capture ten ideas and discard eight than to lose two good ones.

### Refine

Over one or more triage sessions, the issue gets edited in place. Jörn and the agent discuss the task in chat. The agent edits the issue body to reflect what they agree on — Jörn reviews by reading the issue on GitHub.

They fill in Background, sharpen the Deliverable, negotiate Scope, identify Sources, and write Acceptance criteria. As Open questions get answered, their answers move into the appropriate sections — the question is removed from Open questions and the answer is written into whichever section it belongs to.

The issue body is the single source of truth — it always reflects the current best understanding, not a history of revisions. If a decision changes, the issue body changes.

When Jörn and the agent disagree during refinement, Jörn decides. He's the project owner.

### Approve

When Jörn judges the issue is ready — the goal is worth pursuing, the deliverable is clear enough to attempt, the scope won't cause the agent to wander, and the open questions are resolved or non-blocking — he changes the label to `approved`.

This is a gate: no agent session starts without Jörn's approval. The issue is now the prompt.

### Session

Jörn spawns an agent session with the issue as context. The agent reads the issue, then follows the standard session workflow from CLAUDE.md: scope (push back on problems), plan (decompose into steps), implement (write code/proofs/docs), review (re-read the result as a whole). The agent commits and pushes to a feature branch. Label: `in-progress`.

Throughout the session, Jörn provides mathematical direction, answers questions, and makes judgment calls the agent can't make alone.

If the agent discovers the scope is wrong or the task is blocked mid-session, it tells Jörn immediately. They either re-scope together (the agent may update the issue with Jörn's agreement) or abort the session. The agent does not silently produce something different from what was asked.

If the session fails entirely, the agent reports what it tried, what it learned, and what went wrong. The issue goes back to `draft` for re-scoping, or gets closed with a note. No work is lost — the branch exists and the issue documents what happened.

### PR and merge

Jörn creates a PR from the agent's branch, reviews it, and merges to main. If Jörn requests corrections, the agent fixes them on the same branch (in the same session or a follow-up session) — no new issue is needed for PR feedback.

This is Jörn's second gate — code and proofs get human review before landing on main.

### Close

The issue is closed. Any follow-up ideas that emerged during the session are captured as new issues. The idea has found closure — it's no longer actively on anyone's mind because the work is done and integrated.

### Triage

Triage sessions happen regularly. Either Jörn or an agent can initiate one. During triage, they review open issues: close completed ones, refine draft issues, reprioritize, capture new ideas as issues, and identify what's ready for approval. Triage is where the backlog stays healthy.

## Labels

Labels track lifecycle state:

- `draft` — issue exists, not yet ready for a session
- `approved` — Jörn approved, ready for agent session
- `in-progress` — agent session active
- (closed) — done, merged, or abandoned with a note

## Key properties

**The issue body is always current.** Edited in place, not appended to. An agent reading the issue at any point sees the current state, not a thread of revisions it has to reconstruct.

**Labels track state, not content.** The labels tell you where in the lifecycle the issue is. The content tells you what the task is.

**Every gate has a gatekeeper.** `draft → approved` is Jörn's call. `branch → main` is Jörn's call (via PR review). The agent operates freely between gates but doesn't cross them unilaterally.

**Failure is expected and recoverable.** An agent session might fail — the scope was wrong, the task was harder than expected, the approach didn't work. This is normal. The agent declares what it tried and what it learned. The issue goes back to `draft` for re-scoping, or gets closed with a note. No work is lost because the branch exists and the issue documents what happened.

**Ideas are cheap, sessions are expensive.** Creating an issue costs nothing. Running an agent session costs Jörn's time and attention. The lifecycle is designed so that cheap work (capturing, refining issues) happens first, and expensive work (agent sessions) only happens after the task is well-understood.

## Example: happy path

**Spark.** During a triage session, Jörn says: "We need to write down the HK2017 algorithm before we can implement it."

**Capture.** The agent creates issue #12:
- Goal: "Write an implementation-ready description of the HK2017 algorithm."
- Context: "The thesis needs to compute EHZ capacities. HK2017 is the most general algorithm. Can't implement without a clear spec."
- Open questions: "What format? How detailed do correctness arguments need to be? What notation?"
- Everything else empty. Label: `draft`.

**Refine.** Over two triage sessions, Jörn and the agent chat. The agent edits #12:
- Background filled in: the paper reference, the MATLAB implementation, the R^4 restriction.
- Deliverable clarified: "A writeup of the algorithm specialized to R^4 — definitions, theorem statement, pseudocode, correctness arguments, and what's needed from geom2d/geom4d."
- Scope negotiated: "In: the algorithm and its correctness. Out: implementation choices, V-to-H conversion, performance."
- Sources identified: the paper (in repo), the MATLAB implementation (external), archaeology specs (untrusted, pitfalls only).
- Acceptance criteria written: "Other agents can implement from this writeup alone. Subagent clarity checks pass. Jörn accepts the math (proofs are drafts until reviewed)."
- Open questions resolved: notation → use the paper's. Correctness depth → proof sketches, Jörn reviews.

**Approve.** Jörn reads #12, agrees the scope is right, labels it `approved`.

**Session.** Jörn spawns a session. The agent reads the paper, discusses the math with Jörn, writes the algorithm description, runs subagent clarity checks, commits and pushes.

**PR and merge.** Jörn creates PR, reviews, requests one correction to the QP derivation, agent fixes it on the same branch, Jörn merges.

**Close.** Issue #12 closed. Follow-up: "implement HK2017 in Rust" captured as new issue.

## Example: failure and recovery

**Session.** An agent is implementing the billiard algorithm. Halfway through, it discovers the algorithm needs a polygon intersection routine from geom2d that doesn't exist yet and isn't trivial to write.

**Agent tells Jörn immediately**: "I can't complete the billiard implementation without polygon intersection in geom2d. This is a separate piece of work — should I implement it here or split it?"

**Jörn decides**: "Split it. Capture a new issue for polygon intersection, finish what you can for billiard without it, and note the dependency."

**Agent**: Creates issue "geom2d: polygon intersection" (label: `draft`). Updates the billiard issue to note the dependency. Commits the partial billiard work, pushes.

**Result**: The billiard issue goes back to `draft` (scope needs updating to account for the dependency). Polygon intersection gets refined and scheduled separately. No work is lost, the dependency is tracked, and Jörn wasn't surprised.
