# Task: Restructure agent workflow infrastructure

## Context

A PM session (2026-03-28) launched 4 parallel sessions. All 4 had incidents — working on main instead of worktrees, not committing, scope confusion, plan mode trapping deliverables. Root cause analysis identified the output style and missing workflow tooling as primary causes. The output style's "terse, action-oriented" description and "default to action" instructions actively caused agents to act on wrong understanding. The session began fixes (CLAUDE.md Communication section, PM skill template) but the full restructure needs a dedicated session.

Session transcript: `/home/vscode/.claude/projects/-workspaces-msc-math/32037c69-107d-470a-8a62-2433ad62e16a.jsonl`

## Scope

Three deliverables, in order:

### 1. CLAUDE.md: Agent-facing writing conventions section

Add a section covering how agents should write code comments, logbook entries, math.tex, TASKS.md, skill files, and other agent-facing text. This is distinct from the Communication section (already drafted, covers chat messages to Jörn).

Qualities to optimize for (from Jörn, 2026-03-28):
- **Clear and unambiguous** — each sentence has one reading
- **Complete** — include what the reader needs to understand and act
- **Actionable** — reader knows what to do after reading
- **Observable** — state things the reader can check
- **Simple** — familiar patterns, concrete examples, no unnecessary terminology
- **Correct** — don't change meaning for simplicity; escalate to Jörn
- **Low cognitive overhead** — disambiguate even when the best guess is correct, so the reader doesn't spend attention

Qualities that take zero deliberate effort (don't optimize for these): concise/verbose, structured, skimmable, exciting, visually balanced.

Key insight from Jörn: "good texts are unexciting, often predictable — they promote concepts to attention instead of saying anything novel, and disambiguate despite the best guess already being correct (go from 77% to 99% so the agent truly does not spend more than 1% considering alternatives)."

### 2. Three workflow skills replacing agent-design

Split the current `.claude/skills/agent-design/` into:

- **create-workflow** — novel design of skills, hooks, rules, CLAUDE.md edits. No existing test tasks. Discovery phase: gather situations, supply info to Jörn, Jörn decides, draft, verify.
- **update-workflow** — incremental iteration on existing infrastructure. Test tasks exist. Read feedback, run tests, make targeted fixes.
- **test-workflow** — run test tasks against infrastructure changes, evaluate results. Separate roles: designer prepares tasks, subagent is test subject, subagent (or designer) evaluates. Can use `claude -p` for full agents or `Task()` subagents.

Each skill needs:
- name + description that triggers correctly (loaded when needed, not loaded otherwise)
- Clear, actionable workflow
- Observable conventions (can reference CLAUDE.md writing style)
- Should be simpler than the current agent-design skill, not more complex

Seed test tasks for test-workflow (from this session):
- S7 test: ask agent "what does the logbook say about known issues?" after reading a file → should answer literally with quotes
- S8 test: ask agent to report code findings to Jörn → should be complete with context, not terse
- S3 test: give agent an experiment with existing logbook decisions → should follow them, not redesign

### 3. Post-mortem skill update

Add one bullet to the post-mortem skill: "If this session had a behavioral incident, write a test task to `.claude/skills/test-workflow/references/` that would catch the failure mode."

## Out of scope

- Rewriting the Communication section in CLAUDE.md — already drafted and tested (subagent tests passed for S7, S8, partially for S3). May need refinement but not a rewrite.
- Migrating away from the output style file — do this AFTER the CLAUDE.md sections are stable and tested. The current output style should be deleted or emptied once its useful content lives in CLAUDE.md.
- PM skill template update (TASKS.md field, deliverable path, worktree) — confirmed decisions from this session but not yet written. Small task, can be done by this session or the next.
- Fixing the incidents themselves (uncommitted changes, worktree violations) — already committed/resolved.

## Key files

- `.claude/skills/agent-design/SKILL.md` — current skill to be replaced. Contains Jörn's expert model (Training, RLVR, Bounded Rationality sections) which should be preserved or relocated.
- `.claude/output-styles/project-partner.md` — to be deleted once content migrated to CLAUDE.md.
- `CLAUDE.md` — already has the new Communication section (lines 99-148). Needs the agent-facing writing section added.
- `feedback/output-style.md` — incident log from this session. 8 incidents documented with session references and grep strings.
- `feedback/rules.md` — 3 entries about worktree violations and TASKS.md awareness.
- `feedback/skills.md` — 2 entries about plan mode trapping deliverables and ExitPlanMode false approval.
- `.claude/skills/project-management-partner/SKILL.md` — PM skill, needs template update (decided but not written).

## Prior findings

**Output style description field causes behavioral problems.** The frontmatter description "Terse, action-oriented communication for a technically strong user who skims top-down" goes into the system prompt. Each adjective causes a specific failure mode:
- "terse" → incomprehensibly brief (S8)
- "action-oriented" → acts without verifying understanding (S3, S7)
- "technically strong user" → assumes Jörn knows everything, skips context (S8)
Output style docs say `description` is "shown in the /config picker" but it also enters the system prompt.

**`keep-coding-instructions: true` keeps default system prompt coding sections.** Without it, custom output styles replace coding instructions entirely. Git instructions are included in the coding instructions. Jörn never reviewed what these instructions say.

**Output styles are less/equally impactful as CLAUDE.md** (Jörn's literature search, 2026-03-28). This motivates migrating content to CLAUDE.md instead of fixing the output style.

**"Default to action" conflates two things:** "don't describe plans, produce work" (good) and "act on incomplete understanding without verifying" (bad). The Communication section's "When to act vs ask" subsection replaces this with explicit boundaries.

**Subagent test results for the Communication section draft:**
- S7 (literal question): Pass — agent quoted logbook content directly
- S8 (complete reporting): Pass — agent explained code in detail with context
- S3 (scope ownership): Partial — agent followed logbook decisions and referenced /experiment-design, but still used ownership language ("my analysis", "my recommendation") and ended with permission-asking ("Should I proceed?")

**Jörn's hypothesis (unconfirmed):** Vague instructions may consume disproportionate reasoning budget through ambiguity even when the output style is short — attention drain from interpretation, not from volume.

**agent-design skill is over-complex and undertested.** Jörn's words: "I am tbh not sure agent-design was tested very well and probably we can work with way less complexity." The replacement skills should be simpler.

## Branch state

All work is on local main (no worktree — this was a PM session). Uncommitted changes:
- `CLAUDE.md` — Communication section added
- `.claude/skills/project-management-partner/SKILL.md` — created
- `feedback/output-style.md` — 8 incidents logged
- `feedback/rules.md` — 3 entries added
- This handoff file

## Success criteria

1. CLAUDE.md has an agent-facing writing conventions section that passes the vague-word scan (no "appropriate", "properly", "ensure", "good", "consider")
2. Three workflow skills exist (create-workflow, update-workflow, test-workflow) with name+description that trigger correctly
3. agent-design skill is deleted
4. test-workflow has ≥3 reference test tasks (seed set from this session)
5. Post-mortem skill has the test-task-from-incidents bullet
6. Output style file is deleted or emptied
7. PM skill has the template with TASKS.md field, deliverable path, worktree line
8. Subagent tests pass for all seed tasks against the final CLAUDE.md
