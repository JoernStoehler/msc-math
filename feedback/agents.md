# Feedback: Agents (.claude/agents/)

Raw observations from agents about review/planning subagents. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, which agent, what happened, what was confusing/missing/unhelpful. Include: did the agent trigger when expected? Did it produce useful output?

## 2026-03-30: review-proof on verify-numerics/math.tex

Triggered proactively on first draft, found 11 issues (4 high: missing assumptions, broken \ref, handwavy "second-order" claim, dropped second-order term in runtime bound). All addressed before presenting to Jörn. Good ROI — saved one Jörn round-trip.

## 2026-04-01: opus subagent for QP algorithm research

Subagent confidently recommended vertex enumeration ("max of quadratic on polytope is at a vertex"). Self-corrected mid-analysis (indefinite H breaks this), but the main agent presented the recommendation before verifying applicability. Jörn caught it. **Lesson:** When a research subagent makes a mathematical claim, the main agent must verify it applies to the specific problem before presenting. Subagents don't know domain-specific constraints (our H is indefinite).

## 2026-04-02: repo-wide path-update subagents missed file categories

**What happened:** Launched 5 parallel sonnet subagents for Phase 4 path updates (math.tex, Python, CLAUDE.md+rules, TASKS.md+handoffs, logbooks). Two categories of files were missed:

1. `.claude/skills/` and `.claude/agents/` — The CLAUDE.md+rules subagent was prompted with "CLAUDE.md and .claude/rules/*.md" but not `.claude/skills/` or `.claude/agents/`. Three skill files and one agent file had stale `experiments/` paths.

2. `.rs` doc comments in 8 files — The logbook subagent updated logbooks but the prompt didn't cover .rs files. The Python subagent covered analyze.py but left docstrings alone. No subagent was responsible for .rs doc comments.

Required two additional fix-up passes (one via subagents, one manual via sed).

**Error class:** Subagent scope gaps when partitioning work by file type. Each subagent's prompt defined a narrow file set, and files that didn't fit neatly into any category fell through the cracks.

**Suggestion:** For repo-wide find-and-replace tasks, add a "sweep" subagent whose job is to grep for remaining stale references across ALL file types after the targeted subagents complete, and fix anything they missed. Or: include a verification grep in each subagent's prompt and have them report (not fix) files outside their scope.

## 2026-04-03: used subagent to read 3 lines from a file

**What happened:** Needed to find agent names from lines 494-496 of a JSONL transcript. Launched a recover-context subagent to do this. When that wasn't enough, tried to launch a second subagent for the same file. Jörn rejected it and said to just read the file directly. A single `sed -n '494,496p' | python3 -c ...` command took 2 seconds and returned exactly what was needed.

**What should have happened:** The first recover-context subagent (to find which lines contained Agent calls) was justified — searching a 610-line JSONL for relevant entries is a lookup task. But once the lines were known (494-496), reading 3 lines is a direct operation, not a subagent task.

**Pattern:** Over-delegation. Jörn's framing: "Don't ask a librarian to find, read a book and report back some insight from the book. Ask them to find the book and then you read it." Use subagents to locate information, then read and interpret it yourself.
