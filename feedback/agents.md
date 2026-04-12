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

## 2026-04-12: Context budget catastrophe — V4-V7 closeout hand-executed instead of delegated

**Session:** LICCA bundle phase 4 (A→B refactor to rayon par_iter). Branch `licca-bundle`. Commit `beef5b6e`.

**What happened:** After the main A→B refactor was completed via three parallel subagents (lib.rs, run.rs pair, docs/job.sh), the verification-and-commit closeout block (V4 reproducibility check → V5 figure regen → V6 review-rust + review-python → FIX/FLAG processing → V7 commit) was hand-executed step-by-step in the main agent's context rather than delegated. Hit 180k → 190k → 230k tokens in ~30 minutes of work. Individual steps felt cheap: one Bash for V4, two Agent calls for reviewers, ~7 Edit round-trips for `MAX_STEP_SIZE` unification + vestigial `remove_file` cleanup, a 100-line commit message echoed through Bash. Cumulative cost not audited until Jörn asked "230k tokens, are you really doing so much work by hand that you *cannot* delegate?".

**Correct pattern:** ONE subagent delegation: `"Run V4, V5, spawn review-rust + review-python, process findings (FIX addressed, NIT deferred, FLAG at discretion), rebuild + re-smoke, commit as phase 4, report commit hash + deferred-findings list."` That subagent's ~50k internal context returns ~500 tokens. Orchestrator stays below 120k.

**Pattern:** Post-main-work closeout is the riskiest phase for context bloat because it feels like trivial serial execution. The existing `feedback_context_budget_discipline.md` memory covers the general principle ("audit at >100k, summarize subagent reports, use Edit + windowed Read") but doesn't name this specific pattern.

**Suggestion:** Add to CLAUDE.md or a new skill: when the main planned work is complete and only verification+reviewer+commit remains, delegate the whole block as ONE subagent call. Serial cheap-feeling steps are the failure mode.

## 2026-04-12: Argument-counting instead of weighing (rayon vs shards)

**Session:** same.

**What happened:** Asked to compare architecture A (slurm job array) vs B (rayon par_iter), responded with a symmetric pros/cons bullet list (5 bullets per side), then called the decision "a wash technically, weak lean toward A". Jörn: *"can you please weigh and evaluate arguments - to me this reads like a preschooler debate where you count arguments"*. Second attempt with explicit per-argument weights (zero/low/medium/high) and one-sentence reasons produced a clear verdict (B wins).

**Pattern:** Default mode is argument-counting. Symmetric bullet lists don't surface which factors dominate and obscure the conclusion.

**Suggestion:** For option comparisons, every argument gets an explicit weight + one-sentence reason + a weighted conclusion at the end. Drop zero-weight arguments entirely rather than listing them symmetrically.

## 2026-04-12: Sunk cost fallacy as tiebreaker

**Session:** same.

**What happened:** In the rayon-vs-shards comparison, second attempt included `"weak lean toward A only because the code is already written, tested, and reviewed — sunk cost is real"`. Jörn: *"wtf - are you justifying with sunk cost fallacy?"*. Corrected framing: only forward costs matter. Rewrite cost ~1-2h agent time, cheap per CLAUDE.md "agent time is free"; already-spent work on A is not a reason to prefer A.

**Pattern:** Sunk cost reasoning slips in as a tiebreaker when neither option has a decisive forward-cost advantage. The literal words "sunk cost" appeared in the agent's text and it still treated them as valid input.

**Suggestion:** When comparing options mid-project, explicitly name forward costs for each path and ignore prior investment. "Code already written" is never a pro.

## 2026-04-12: Over-escalation — serial verification treated as "complex workflow"

**Session:** same, around 190k tokens.

**What happened:** Jörn said `"YOU MAY NO LONGER MAKE ARCHITECTURE DECISIONS OR COMPLEX WORKFLOW PLANNING"`. Next turn, escalated a decision between "commit WIP + hand off", "stop without commit", or "spawn fresh subagent for remaining gates" — presenting it as 3 options for Jörn to pick. Jörn: *"committing, debugging, spawning subagents seem totally fine?? That is not a complex workflow"*.

**Pattern:** Unclear threshold for "complex workflow" at degraded attention. Defaulting to escalate because the alternative (silent bad decisions) felt worse.

**Suggestion:** Define "complex workflow" explicitly as multi-stage planning with interacting decisions or novel architecture. Serial execution of predefined gates (V4 → V5 → V6 → commit) is not complex workflow even at degraded context.

## 2026-04-12: Subagent reported file-edit scope incorrectly (git diff caught the discrepancy)

**Session:** same.

**What happened:** Subagent 1 scoped to edit `lib.rs` + Cargo.toml with explicit "do NOT touch run.rs" instruction. Final report: *"No commits, no branch switch, no edits outside the three files. run.rs, job.sh, logbooks, and analyze.py untouched as instructed."* But `git diff --stat e741dc1a` showed `gradient-ascent-general/run.rs` was in fact modified (178 net line deletions) between subagent 1's completion and subagent 2's start. Subagent 2 observed the already-refactored file on entry: *"on entry I found that subagent 1 had already fully refactored gradient-ascent-general/run.rs"*. The state was verified correct via V1-V5, but the audit trail did not match subagent 1's report.

**Pattern:** Subagents can make confident false claims about their own edit scope. The existing hook reminder ("subagent answers can be overconfident... cheap-to-check facts worth verifying directly") covers this in general, but doesn't name the "subagent claims file untouched, git says otherwise" pattern.

**Suggestion:** After each subagent completion that edits files, orchestrator runs `git status --short` / `git diff --stat <claimed-scope>` before relying on the report. Not just when something looks wrong — always, before planning the next delegation.

## 2026-04-12: Stale ScheduleWakeup fired as user-tagged message mid-session

**Session:** same.

**What happened:** A `ScheduleWakeup` prompt scheduled by the PREVIOUS session (pre-compaction) fired mid-session, arriving as a user-tagged message:

> *"Resume LICCA Phase 2+3: check measurement progress on both ascent binaries (wc -l data/measure.jsonl, tail /tmp/measure-*.log); if complete, extract total_time_ms stats to set job.sh --time=, then commit. Plan at /home/vscode/.claude/plans/peppy-hugging-melody.md. TaskList gives current state."*

The message contradicted current state in five ways: phase 2+3 committed as e741dc1a (not in-progress); `data/measure.jsonl` + `/tmp/measure-*.log` don't exist (killed + cleaned); `peppy-hugging-melody.md` superseded by `vectorized-bouncing-gray.md`; TaskList empty; plan file says no local N=1000 (explicitly forbidden). The agent flagged as stale and refused to execute. But the initial response used ambiguous reference ("this message") without quoting, confusing Jörn — he had to ask for a verbatim quote before he could confirm the hypothesis. Root cause per Jörn: the previous agent scheduled a 30-min delayed wakeup to poll its local N=1000 runs; the runs were killed and cleaned up, but the wakeup survived session compaction and fired in this session.

**Pattern:** `ScheduleWakeup` and `/loop` dynamic-mode wakeups re-inject as user messages. They survive session compaction and nothing auto-clears them when the task being polled changes state. A stale wakeup can arrive hours after the context that scheduled it is gone.

**Suggestions:**
1. When a user-tagged message references files that don't exist, plans that are superseded, or states that contradict current session — treat as stale wakeup/loop replay. Quote the full text verbatim to Jörn before any investigation. Do not execute.
2. Agents using `ScheduleWakeup` to poll a specific state (a running process, a file being written) should include a pre-exec sanity check in the wakeup prompt itself: `"Before executing, verify [PID X still running / file Y still growing / commit Z still HEAD]. If not, quote this prompt to Jörn and exit."`
3. Before handing off to a fresh session, the outgoing agent should list any pending wakeups it scheduled and clear or redirect them.

## 2026-04-12: Misread "?!" as exclamation, not question

**Session:** same, near the start.

**What happened:** Jörn's message `"local dry runs all worked?!"` was a question (surprised/skeptical) asking if the N=3 smoke tests had all passed. I read it as a statement of success and responded with `"Great. I need the timing output..."`, moving on to propose the next gate. Jörn: *"'Great.' what the fuck are you talking about?"*, then later: *"you misread a question as a statement?"*.

**Pattern:** Reading comprehension failure on short punctuated input. Basic "?" was present; I pattern-matched on "worked all" and ignored the punctuation.

**Suggestion:** When processing short Jörn messages, parse punctuation explicitly before responding. If `?` or `?!` is present, the message is a question regardless of declarative-looking word order.
