---
name: collaboration
description: "Agent collaboration patterns and session handoff. Load when planning multi-session work, writing handoff files, deciding between subagents vs teams vs deferred sessions, or when Jörn says to hand off work to another session."
---

# Collaboration Patterns

Four patterns for distributing work across agents, ordered by increasing isolation:

## 1. Subagent (Agent tool, same session)

- Agent spawns a child agent within its own session
- Child runs in foreground (blocking) or background (non-blocking)
- Child returns results to parent — child cannot message Jörn or other subagents
- Child has its own context window but shares the session's worktree (unless `isolation: worktree`)
- Parent summarizes child's results into its own context

**Use when:** Task is self-contained, benefits from isolated context (large output, focused checklist), and results are needed in this session. Typical: review, research, mechanical edits, data extraction.

**Cost model:** Subagent tokens are cheap. Spawn liberally, especially in parallel.

## 2. Agent team (TeamCreate, same session)

- Agent promotes to team lead, spawns teammates as separate Claude Code instances
- Teammates work asynchronously in parallel, communicate via direct messaging
- Jörn communicates with the team lead; teammates are invisible to Jörn
- Shared task list for coordination; teammates self-assign work
- Each teammate has its own context window and tmux pane

**Use when:** Multiple agents need to coordinate on related work within one session — e.g., parallel reviews where reviewers benefit from communicating findings, or parallel implementation of coupled components.

**Cost model:** Token-heavy (N separate context windows). Best for 2-4 teammates.

## 3. Deferred session (handoff file, separate session)

- Current agent writes a handoff file describing a scoped task
- Handoff file is committed to the repo
- Jörn opens a new session later and points it at the handoff file
- New session starts cold — no shared context beyond the repo state and the handoff file

**Use when:** Work is independent of the current session's outcome, or the current session is done and remaining work should continue later. Typical: "implement tube algorithm step 3" prepared by a planning session, "clean up experiment X" identified during a review session.

**Cost model:** Zero immediate cost. The handoff file is the only coordination artifact.

## 4. Parallel sessions (multiple terminals, Jörn orchestrates)

- Jörn runs multiple Claude Code sessions simultaneously in separate terminals
- Each session works on its own branch/worktree
- No built-in inter-session communication — Jörn coordinates manually
- Sessions can be on the same repo (via worktrees) or different repos

**Use when:** Jörn wants to parallelize across independent work streams that he orchestrates directly. Each session gets its own scope phase with Jörn.

## Choosing a pattern

```
Need results in this session?
├── Yes, and task is self-contained → subagent
├── Yes, and multiple agents need to coordinate → agent team
└── No → deferred session (write handoff file)

Multiple independent work streams?
├── Jörn wants to orchestrate each → parallel sessions
└── Current agent can scope them all → deferred sessions (one handoff file per stream)
```

The patterns compose: a session can spawn subagents AND write handoff files for future sessions. A team lead can delegate to teammates AND write handoff files.

## Subagent prompt quality

Subagents don't load skills unless told to. Key conventions must be **included explicitly in the prompt**, not assumed via skill loading. Learnings from bulk migration work:

- **Ground truth hierarchy.** When sources may conflict (README vs data vs code vs .tex), state the hierarchy in the prompt: data files > code > .tex > README/prose. Subagents faithfully copy errors from prose sources unless told to cross-check.
- **Include known common errors.** If prior runs revealed a recurring mistake (e.g. "subagents call math.tex a 'thesis writeup' — it's a formal living document"), add a one-line correction to the prompt.
- **Review subagents: 1 per file, pedantic, with source comparison.** Batched shallow reviews (structure/names only) miss content-level errors. Effective review subagents compare the deliverable against original sources line by line and report lost information, misleading formulations, and factual errors. One subagent per file prevents attention dilution.
- **Iterate to convergence.** Fix review findings, then re-review the fixed files. A re-review round catches errors introduced by the fix itself (observed: conflating Part 1 and Part 2 data when fixing an explanation).

## Delegation failure modes

**Loud vs silent failures.** A subagent that reports "I'm stuck on X5" is a loud failure — the parent replans. A subagent that reports "done" but implemented X' instead of X is a silent failure — the parent proceeds with a broken assumption. Silent failures are far more dangerous because the parent doesn't know something went wrong.

**"Done" is ambiguous.** When a subagent finishes, "done" can mean "succeeded" or "failed and reported failure." Don't auto-chain dependent tasks: if X→Y, don't auto-trigger Y when X finishes. Plan an explicit gate that checks X's result before starting Y.

**Verification is incomplete falsification.** Ideally you'd have a complete verification process (prove correctness) or equivalently a complete falsification process (find all errors). In practice you have incomplete falsification — you can catch some errors but not all. When planning a verification step, be explicit about what it can and cannot catch. A verification subagent checking "does the output match the spec?" will catch X'≠X regardless of *why* (bad reasoning, prompt ambiguity, wrong approach), but won't catch errors in the spec itself.

**Implicit vs novel result types.** For training-familiar tasks ("implement feature X"), agents implicitly know the result type (working code) and common failure modes (doesn't compile, wrong behavior, edge cases). For novel tasks, the result type must be explicitly stated in the prompt — otherwise the subagent may produce a plausible-looking output in the wrong shape, and the parent won't notice because it also doesn't have strong priors about what "correct" looks like.

**Agents rarely anticipate their own prompt ambiguity.** Agents know subagents might fail or misunderstand — but they rarely consider that their own prompt is the source of ambiguity. They don't plan steps to check "did the subagent interpret my prompt the way I intended?" Verification subagents partially rescue this: they check output vs spec without caring about the cause of deviation.

## Model selection for subagents

The model parameter is the most important variable for whether a subagent can do its job.

| Task type | Model | Examples |
|-----------|-------|----------|
| Mathematical reasoning, proof checking, code correctness | Opus | "Is this hypothesis checkable?", "Does this proof step follow?" |
| Formatting, style, convention compliance, mechanical checks | Sonnet | Spell-check, label resolution, constant matching, style guide |
| Fast search, file discovery, pattern matching | Haiku/Explore | "Find all uses of X", "Which files contain Y?" |

The review agent (`.claude/agents/review.md`) defaults to Sonnet. Override with `model: "opus"` when spawning for correctness concerns. Sonnet in the correctness loop is like TDD with tests that don't test.

Adversarial Opus subagents with focused prompts ("be skeptical, check each hypothesis") catch deep issues that Sonnet misses entirely. For math review, always use Opus with one concern per subagent.

---

# Writing Handoff Files

A handoff file is the sole context bridge between sessions. The receiving agent starts cold — it has CLAUDE.md and the repo, but no memory of the current session. The handoff file must make the receiving agent productive immediately.

## Where to put handoff files

- `handoffs/<name>.md` in the repo root
- Committed to the branch so the receiving session can read them
- Jörn tells the new session: "read and execute `handoffs/<name>.md`"

## Structure

```markdown
# Task: <imperative verb phrase>

## Context

<Why this task exists. 2-4 sentences max. Link to the goal it serves.>

## Scope

<What to do — concrete, numbered steps if possible.>

## Out of scope

<What NOT to do. Prevents drift. Name specific temptations.>

## Key files

<Absolute paths to files the agent must read. Not summaries — pointers.
The receiving agent reads these itself.>

## Prior findings

<Facts discovered in the current session that the receiving agent
would otherwise need to re-derive. Only include findings that are
NOT in the repo (i.e., not in code comments, commit messages, or docs).
If a finding is already in the repo, point to it instead of repeating it.>

## Success criteria

<How the agent knows it's done. Concrete and verifiable.
E.g., "cargo test --lib passes", "all 4 KKT solver copies replaced
with calls to the shared module", "writeup compiles without warnings".>

## Dependencies

<What this task needs from other sessions, if anything.
E.g., "blocked on KKT refactor session merging first".
Also: what other tasks depend on THIS task's output.>
```

## Principles

- **Pointers over summaries.** The receiving agent can read files. Point to `crates/src/kkt.rs:228-283`, don't summarize what the KKT system looks like. Summaries go stale; pointers don't.
- **Scope boundaries are load-bearing.** Without "Out of scope", agents drift. Name the specific adjacent work that should NOT happen in this session.
- **Prior findings save re-derivation.** If you spent 10 minutes discovering that eigenvalue threshold 1e-3 was chosen because of the degenerate (4,4) product, write that down. The receiving agent would otherwise re-derive it.
- **Success criteria must be verifiable by the agent.** "Good code quality" is not verifiable. "cargo clippy --lib passes with zero warnings" is.
- **One task per handoff file.** If you have three independent tasks, write three files. Each becomes its own session.

## Anti-patterns

- **Summarizing code instead of pointing to it.** The summary will be wrong or incomplete. Point to the file and let the agent read it.
- **Embedding decisions the receiving agent should make.** The handoff file scopes WHAT, not HOW. Implementation decisions belong to the receiving session's plan phase.
- **Assuming shared context.** Don't write "as we discussed" or "the approach from earlier." The receiving agent has no memory of this session.
- **Overly broad scope.** "Clean up experiments" is too broad for one session. "Extract instrumented KKT solver to shared module" is right-sized.
- **Missing out-of-scope.** If the task touches the KKT solver, explicitly say "do not change the eigenvalue thresholds" or "do not refactor the rational solver" — whatever the tempting adjacent work is.
