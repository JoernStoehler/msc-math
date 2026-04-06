# Orchestration Patterns — Design Space

## Goal

Find orchestration patterns that improve three things (in priority order):

### 1. Jörn-time spent on avoidable work (highest priority)

Jörn's time goes to: fixing agent mistakes, explaining things agents could figure out,
telling agents what to do when a smarter/better-contexted agent could decide on its own.

Orchestration can help by:
- **Making agents smarter** (cleaner context → better reasoning → fewer mistakes → less
  Jörn-time fixing).
- **Avoiding entire mistake classes.** Example: agents often commit to one approach without
  comparing alternatives, then sunk-cost-fallacy their way into fixing a bad result instead
  of admitting the approach failed. Jörn has to learn the hard way through chat that this is
  happening, then tell the agent to stop, hand off to a new session, and explicitly ask the
  new agent to compare approaches under supervision. An orchestration pattern that builds in
  approach comparison before execution avoids this class entirely.
- **Reducing supervision overhead.** Some patterns need less Jörn-involvement per unit of
  useful work than others.

### 2. Agent intelligence ↔ context quality (the main technical lever)

Agent reasoning quality degrades as irrelevant tokens accumulate in context (see "Why context
management matters" below). The operational levers:
- **Assign smaller task lists to a single session** — less work = less context accumulation.
- **Switch more between multiple sessions** — refresh context at natural phase boundaries.
- **Delegate more of a session's work to subagents** — subagent context doesn't pollute parent.
- **Add tools that cut down on irrelevant tokens in context** — a new direction. Ideas:
  - Chat extraction from sessions (strip tool calls, keep human↔assistant messages) as an
    alternative to resuming full sessions or lossy compaction summaries.
  - Rust codebase documentation — if agents can read symbols + doccomments instead of full
    function bodies, they need fewer tokens to understand the codebase.
  - Other token-efficient information retrieval mechanisms.

### 3. Total Anthropic API budget (lowest priority)

Not currently a binding constraint. Shorter, cleaner sessions will naturally burn less API
budget anyway. But subagent delegation has a real cost — subagent tokens are billed on the
same account, just not in the parent context. Worth tracking but not optimizing for directly.

## How this document works

Working document. Created 2026-04-05 in a research session with Jörn.

- Each pattern has 4 sections: training priors, phases/roles, proposed variant, experience.
- "Experience" sections are placeholders — filled after trying the pattern.
- Future sessions: pick a pattern, prepare materials, try it, fill in the experience section.

## Why context management matters

### The degradation model

As tokens accumulate in context, two things happen:
1. **Intelligence/attention degrades non-linearly.** There is some regime where adding tokens
   is mild, then a steeper drop. The mechanism: the model's attention is a finite resource.
   More tokens in context = more candidates competing for attention = less attention per
   relevant token. This is not about running out of window — it's about dilution.
2. **Speed degrades roughly linearly.** More input tokens = proportionally slower output.

**Token count is a reasonable proxy** for degradation, unless the mixture is unusual. But
per-token harm varies by content type:
- **Blatantly irrelevant** (e.g., build output from 3 phases ago): lowest harm per token,
  but still nonzero — even obviously irrelevant content is a drain on attention.
- **Stale/outdated** (e.g., previous iteration's code): roughly same as irrelevant. Models
  are trained on multi-version contexts, so they handle this okay. Mainly harmful by volume.
- **Non-blatantly irrelevant** (e.g., tool output that *looks* related to current task but
  isn't needed): higher harm. The model can't cheaply discount it, so it competes for
  attention with genuinely relevant content.
- **Confusing/wrong** (e.g., agent-written false proof not flagged as such): similar harm to
  non-blatantly-irrelevant. The model may reason from wrong premises.

The practical upshot: **don't overthink content classification, just minimize total tokens.**
Orchestration should aim for "what's the minimum context needed for this phase?" rather than
surgically removing harmful tokens.

### Evidence

- **GSM-Symbolic** (Mirzadeh et al. 2024): Adding a single irrelevant-but-plausible clause
  causes up to 65% performance drop across all SOTA models. Stale context is actively harmful.
- **Anthropic** (official docs): "A clean session with a better prompt almost always
  outperforms a long session with accumulated corrections."
- **RULER benchmark** (2024): Most models fail complex tasks at half their claimed context
  length. (Pre-Opus-4.6 — absolute thresholds may differ, but "usable < claimed" likely holds.)
- **Jörn's observation** (Opus 4.6, 1M window): >200K tokens → barely functional for wrap-up
  discussions, and fails entirely at high-intelligence tasks like task delegation/decomposition.
  Confounded by selection bias — reaching 200K requires hard-to-decompose tasks that are
  independently difficult.
- **Lost-in-the-middle** (Jörn's observation, Opus 4.6): Seems fine for recall ("have you
  read something relevant to X?"). But instructions should be at the start or end of context.
  Practical implication: pivoting a session to a new task at high token count is bad — the
  new task's instructions land in the middle, surrounded by irrelevant context from the old task.
  This is already covered by "don't start a new task at 100K+ tokens."

**Caveat:** Most published benchmarks are pre-Opus 4.6. Absolute token thresholds from older
models don't transfer directly. The qualitative mechanism (attention dilution) is model-general,
but where exactly the non-linear drop-off hits for Opus 4.6 on 1M context is unknown.

### The "PhD under time pressure" model

A useful approximation for managing agents: Claude Opus 4.6 is like a person with PhDs in
every domain but under enormous time pressure — even "thinking carefully" happens under a tight
budget. The model has broad expert knowledge (Rust, LaTeX, proof techniques, symplectic
geometry basics, etc.) but limited deep-reasoning bandwidth per response.

**Specific weaknesses** (things NOT to give agents unsupervised):
- Writing proofs under time pressure → sloppy, skips steps, makes sign errors
- Understanding complex software libraries (as opposed to simple ones) → error-prone,
  misses compound conditions and edge cases
- Deep symbolic modeling in complex domains → can't reliably hold a complex formal system
  in working memory across many steps

Note: the "time pressure" is a convention (cost optimization), not a hard limit. We could
in principle let agents think for hours. But in practice the budget is tight.

Implications for orchestration:
- **Narrow tasks → concentrated budget.** A subagent doing "read these 3 files and summarize
  the algorithm" has its full reasoning budget focused on that task. A session agent juggling
  exploration + implementation + correctness + workflow + communication splits the same budget.
- **Agents execute decomposed tasks well, but designing decompositions is expensive.** Knowing
  when a decomposition has failed requires meta-reasoning that competes with task execution for
  the same attention budget.
- **Jörn's role as supervisor is cheap** (a few sentences of steering) while the agent's
  execution is expensive-but-bounded. The orchestration design should keep decomposition
  decisions with Jörn and give agents well-bounded execution tasks.

### Feasibility constraint: agent training priors

Orchestration patterns must use concepts agents already have behavioral priors for. Teaching
a novel coordination protocol costs attention budget that competes with task execution. If an
agent spends 20% of its effective capacity understanding its role, it has 80% left for work.

**Likely strong priors** (agents do these well — inferred from behavior, not confirmed
training data):
- "Here's a task/issue, produce a solution" (likely RLVR: SWE-bench, issue→PR)
- "Here's a spec, implement it" (likely pre-training: docs→code)
- "Read X, summarize for purpose Y" (likely pre-training: research assistant patterns)
- "Review this for correctness" (likely pre-training + RLHF: code review, paper review)
- "This is in scope, that isn't" (likely SWE project management training data)
- "Write a proof of lemma X given definitions Y" (likely math training data)

**Likely weak/no priors** (agents struggle or burn budget on these):
- Multi-agent peer coordination (probably not in RLVR training)
- Self-managing context/compaction (novel to Claude Code, no training data)
- Detecting own decomposition failure (requires expensive meta-reasoning)
- Novel orchestration protocols that need in-context explanation

The **form** (GitHub PR, plan file, JIRA ticket) doesn't matter much — what matters is the
**concept + behavioral strategy** the form activates. "Complete this PR" and "execute this
plan file" activate the same prior: glance at task description, glance at verification
pipeline, produce artifact. **Reward-hacking risk:** agents have a trained tendency to
optimize for passing the verification pipeline, even at the cost of correctness. An
orchestration pattern with automated verification may see agents gaming the checks rather
than doing the underlying work well.

### The universal protocol

All patterns reduce to the same underlying protocol: "here's a prompt/description of
something to achieve, here's some rules/conventions, here's some feedback mechanism; now go
to work." The orchestration question is how to structure each of these three components and
when to refresh them (new session, new subagent, compaction).

If agents are told to first develop a feedback mechanism (e.g., write tests before
implementing), they can do that — but building the feedback mechanism itself costs ~50K tokens
of context, of which only ~1K (the resulting tests/checks) are needed afterward. This is a
concrete example of context accumulation during setup phases.

### Interface complexity as decomposition criterion

A decomposition is good when its **interface is narrow**: small input context, small output
summary. The internal work can be arbitrarily complex — it's discarded after the phase
completes.

Concrete hierarchy of search patterns ranked by interface width (from narrow to wide):

| Pattern | Input interface | Output interface |
|---------|----------------|------------------|
| grep → scattered read | keywords | line:content pairs |
| delegated keyword discovery | vague topic | keyword list → snippets |
| full semantic search | question + large text | semantic summary |
| code iteration | spec + current state | working code (history discarded) |
| experiment execution | run command, collect | numbers + interpretation |
| proof attempt | conjecture + definitions | proof or "stuck at X" |

Each is a **natural unit of delegation** — agents execute them well because they match
training data. The key: previous iterations / internal work are NOT part of the output
interface.

**Design method:** Inventory what concepts agents already understand, then compose patterns
from those building blocks. Don't start from an ideal orchestration and try to teach it —
start from what agents know and build up.

### Practical numbers

- Fixed overhead per session: ~20K tokens (system prompt, tools, agents, memory, skills).
- Agent Teams: designed for parallel independent work, not sequential phases. Community
  consensus: subagents suffice for most work. Not explored further here.
- Subagent token cost: billed on the same account, just not in parent context. A subagent
  reading a 5K file and returning a 500-token summary costs ~6K+ total to save 4.5K in
  parent context. **Default should be "just read the file directly."** Subagents only earn
  their cost when doing significant work beyond reading — searching across many files,
  synthesizing multiple sources, running code. Using subagents to summarize known files is a
  telephone game: introduces errors/misinterpretations and burns API budget for marginal
  context savings.

### Research-day phase decomposition

A higher-level view of how research work naturally decomposes into phases, each of which maps
to one of the patterns below:

```
gather todo list (Pattern 6: TASKS.md population)
  → prioritize highest value-over-cost questions (Jörn, possibly with agent)
    → expand a question:
        - propose actionable computational experiments (Pattern 5: task map)
        - propose actionable formalization + proof attempts (conjecturing)
        - propose actionable writeup/distillation to simplify the project surface
      → execute something actionable:
          - formalize a vague statement until one conjecture is provable and useful
          - write and run experiment code
          - iterate on a code folder (previous versions NOT needed as context —
            the coding agent should be compacted/refreshed periodically)
            → report results (narrow interface back up the chain)
```

Each level is a natural decomposition boundary. Each indentation level could be a session
boundary or a subagent boundary, depending on complexity.

---

## Pattern 1: Chat + Execute (no plan)

### Training priors

Agents know this from **interactive coding assistance** — the dominant pattern in pre-training
data (Stack Overflow answers, chat-based coding help, pair programming transcripts). Also
heavily reinforced in RLHF: user asks → agent does → user reacts → agent adjusts.

Variants agents likely recognize (based on what they do well, not verified training data):
- Pair programming (continuous back-and-forth, small changes)
- "Fix this bug" / "add this feature" (single request → execution → done)
- Exploratory coding ("let's try X... ok now try Y...")

### Phases / roles

```
Jörn: request/steer ←→ Agent: understand + execute
      (interleaved, no formal phase boundary)
```

**Jörn's role:** Continuous steering. Provides task, reacts to output, redirects.
**Agent's role:** Understand request, execute, show results, adjust.

Phases overlap almost entirely — the agent is simultaneously understanding what to do and doing
it. No explicit planning phase, though the agent internally plans (extended thinking).

### Proposed variant for this project

Use as-is for **small, clear tasks** where:
- The task fits in one session without significant context pressure
- Jörn can verify results quickly (e.g., a single file change, a quick computation)
- No cross-session coordination needed

**Session boundary:** Session ends when task is done. No handoff needed.

### Experience

_Placeholder — this is the current baseline. Notes on when it works and when it breaks:_

---

## Pattern 2: Plan → Execute (within one session)

### Training priors

Agents know this from **SWE-bench / issue→PR workflows** (RLVR training). The behavioral
pattern: read task description → explore codebase → form plan → implement → verify. Also
from software engineering training data: design docs → implementation, RFC → code.

Variants agents likely recognize:
- GitHub issue → PR (probably strongest prior: task description + verification = PR acceptance)
- Design doc → implementation (agents seem to know this but often skip back to the design)
- TDD: write test → implement → pass (agents can do this when asked but don't naturally
  initiate it — unclear if this is a training gap or a prompt gap)

The plan-mode / bypass-permissions-mode toggle in Claude Code maps to this: plan mode forces
the agent to only write a plan file (= design phase), bypass mode lets it execute (= implement).

### Phases / roles

**Variant A: Single session**
```
Phase 1 (plan mode):     Jörn + Agent discuss → Agent writes plan file
Phase 2 (bypass mode):   Agent executes plan, Jörn monitors
  [optional: Agent hits predicted stuck point → EnterPlanMode() → re-plan → continue]
Phase 3 (end):           Agent runs /pre-merge → subagent reviews → fixes
```

**Variant B: Two sessions** (Jörn's most complex explicit orchestration)
```
Session 1 (plan mode):   Chat about research question → plan file v1
                          (research question + experimental approach + rough architecture)
Session 2 (plan mode):   Attach plan file v1 → agent investigates how to execute →
                          writes plan file v2 (concrete action plan, with EnterPlanMode()
                          scheduled at predicted stuck points)
Session 2 (bypass mode): Agent executes plan v2
  [at stuck point: EnterPlanMode() → compaction → re-plan tail → continue]
Session 2 (end):         /pre-merge → subagent reviews → fixes
```

The two-session variant is intermediate between Pattern 2 and Pattern 3 — Session 1's context
is discarded, but Session 2 keeps its planning context during execution.

**Jörn's role:**
- Phase 1: Co-author of the plan (scope, approach, constraints). Approves plan.
- Phase 2: Monitor. Intervenes on failure or drift.
- Phase 3: Reviews /pre-merge output, gates merge.

**Agent's role:**
- Phase 1: Investigate (read code, explore), propose plan, incorporate Jörn's feedback.
- Phase 2: Execute plan. Internally decomposes into sub-steps. May delegate to subagents.
  ~90% execute, ~10% ongoing planning/adjustment.
- Phase 3: Quality control via review subagents. Fix findings.

**Phase overlap:** Phase 2 has significant overlap with planning — the agent reads code during
execution that informs how to proceed. This is valuable: the reading context is still available
when the agent acts on it. A strict phase split (plan in one session, execute in fresh session)
would lose this, requiring re-reading.

**Scheduled re-plan (EnterPlanMode):** For long action sequences where the head is clear but
the tail depends on what happens. The agent plans the head, executes it, then re-plans the tail
after compaction. This is a context refresh point — similar to starting a new session but
retaining a compaction summary of what was done.

### Proposed variant for this project

Current practice. Works for **medium-to-complex tasks** where:
- The task needs investigation before execution
- Jörn wants to review the approach before work begins
- The task is completable in one session (maybe with 1-2 re-plan points)

**Known issue:** Execution phase accumulates tokens fast (Read + Write + progress). By
mid-execution, context may be 150K+ and quality degrades for remaining high-intelligence
subtasks (like writing math.tex or making architectural decisions).

**Possible improvements to test:**
- More aggressive subagent delegation during execution (but see cost/telephone-game tradeoff)
- Smaller plan scope → more frequent plan→execute cycles → more context refreshes
- Explicit "checkpoint" discipline: agent writes state to plan file before heavy execution bursts

### Experience

_Placeholder. Current baseline for complex tasks. Known to work but token-hungry._

---

## Pattern 3: Plan → Clear → Fresh Execute

### Training priors

Agents know the **spec → implementation** pattern from pre-training (API docs → client library,
requirements doc → code, math problem statement → solution). The key: the spec is
self-contained enough that the implementer doesn't need the discussion that produced it.

An analogy: fresh context is like a new developer onboarding — read the docs, read the ticket,
start working. Whether agents have a specific prior for this or just handle it as "read then
do" is unclear.

### Phases / roles

```
Session 1 (plan mode):   Jörn + Agent discuss → Agent writes plan file
[/clear or new session]
Session 2 (bypass mode):  Agent reads plan file → executes
```

**Jörn's role:**
- Session 1: Same as Pattern 2 Phase 1.
- Between sessions: Triggers /clear or starts new session. May edit plan file.
- Session 2: Monitor, intervene on failure.

**Agent's role:**
- Session 1: Investigate, write plan. The plan must be self-contained — all context the
  executor needs must be written down, because the discussion context is gone.
- Session 2: Read plan, execute. Re-reads files as needed (fresh context = will re-read
  what Session 1 agent already read, but from clean state).

**Critical interface:** The plan file. If the plan is vague, Session 2 agent will flounder or
make wrong assumptions. If the plan is too detailed (prescribing exact code), it constrains
Session 2 agent's ability to adapt to what it finds.

**Plan granularity tension (open design question):** The plan should describe WHAT to achieve
and WHY, not HOW at the code level (existing feedback memory: "scaffold context not
prescriptions"). The executing agent should decompose into concrete actions itself — because
the reading/investigation done during that decomposition is context it needs while executing.

However, agents are often confused about whether to break the plan down further into a
low-level action sequence. People seem divided on this. An explicit split is possible:
- Phase A: plan-level decomposition (what + why + rough approach)
- Phase B: action-level decomposition (concrete steps, what to read first, how to verify)
- Phase C: execution

Whether B and C should be in the same session (B's reading context is useful during C) or
separate (B's context may pollute C at high token counts) is an open question to test.

### Proposed variant for this project

Use when **the plan is complex enough that execution will exceed ~100K tokens**, and the
planning discussion itself consumed significant context. The /clear boundary discards the
planning discussion but preserves decisions in the plan file.

**Session boundary:** /clear after plan approval (or new session with plan file path in prompt).

**Variant: `/clear` vs new session:**
- `/clear`: Same terminal, plan file path already known. Faster.
- New session: Fully fresh. Can also change permission mode, attach different files.
- New session with `--resume`: Doesn't help — resumes full context including stale parts.

**What the plan file should contain (minimum):**
- What to achieve (goal, success criteria)
- Why (context the executor needs to make judgment calls)
- What files to read/modify (starting points, not exhaustive)
- What's NOT in scope (prevents drift)
- How to verify (test commands, review checklist)

### Experience

_Placeholder. Jörn uses this occasionally. Key question: does the plan file quality justify
the re-reading cost?_

---

## Pattern 4: Failed Attempt → Handoff → Retry

### Training priors

Agents likely know **debugging from error logs / failed attempts** (pre-training: Stack
Overflow threads where the first answer didn't work, GitHub issues with back-and-forth). Also:
**code review → revision** (PR feedback → new commit).

The handoff file maps to a "bug report with reproduction steps + failed attempts" — agents
seem to handle these well. Jörn's framing: the handoff is "a v2 of the prompt" — not a
recovery document but a better-specified version of the original task.

### Phases / roles

```
Session 1:  Agent attempts task → fails or produces poor results
[Jörn deletes worktree, writes/edits handoff file]
Session 2:  Fresh agent reads handoff → attempts with learnings
```

**Jörn's role:**
- Diagnoses why Session 1 failed (or accepts agent's diagnosis)
- Decides to retry vs. abandon vs. re-scope
- Writes/edits the handoff file with clarifications and constraints
- Deletes the failed worktree (clean slate)

**Agent's role:**
- Session 1: Attempt the task. When stuck, write what was tried, what went wrong, what
  hypotheses remain (this becomes the handoff or feeds into it).
- Session 2: Read handoff. Avoid the same mistakes. Benefit from clarified scope/approach.

**Jörn's framing:** The handoff is "a v2 of the prompt" — the original task description plus
learnings and clarifications. Session 2 agent sees a better-specified task, not a recovery job.

### Connection to the sunk-cost mistake class

Pattern 4 is the **recovery mechanism** for the sunk-cost mistake class described in the Goal
section. The typical arc:
1. Agent commits to one approach without comparing alternatives
2. Approach produces bad results
3. Instead of admitting failure, agent starts fixing/patching the result while sticking to
   the approach (sunk-cost fallacy)
4. Jörn notices through chat that this is happening — often late, after significant wasted work
5. Jörn tells the agent to stop, deletes the worktree
6. Handoff to fresh session with explicit instruction: "compare approaches before committing"

**Prevention (not just recovery):** An orchestration pattern that builds in approach comparison
*before* execution would avoid step 1-4 entirely. This could be a variant of Pattern 2 or 3
where the plan phase explicitly requires listing and comparing 2-3 approaches before picking
one. Open question: does this add enough value to justify the planning overhead for every task,
or should it be reserved for tasks where the approach is genuinely uncertain?

### Proposed variant for this project

Current practice. Works well. The main improvement opportunity is in **handoff file quality**.

Existing handoff infrastructure: `handoffs/` directory, `/handoff` skill.

**Key quality criteria for the handoff:**
- What was the goal (not just what was attempted)
- What specific approach failed and WHY (not just "it didn't work")
- What the agent learned about the codebase/problem that a fresh agent wouldn't know
- What approach to try next (if known) or what to investigate first

### Experience

_Placeholder. Current practice. Generally works — the main failure mode is handoff files that
describe what was done but not why it failed._

---

## Pattern 5: Task Map → Independent Sessions

### Training priors

Agents know **project planning / work breakdown structures** from pre-training (project
management docs, sprint planning, roadmap documents). Also: **modular software design** —
decomposing a system into components with defined interfaces.

The task map itself is similar to a **README + architecture doc** that a new contributor reads
before working on one module. Agents are trained on "here's the system, you own this component,
don't touch the others."

Variants:
- Sprint backlog → individual tickets → individual PRs (SWE)
- Research agenda → individual experiments (academia)
- Thesis outline → individual chapters (academic writing)

### Phases / roles

```
Session 0 (task map creation):
  Jörn + Agent brainstorm research question → break into components
  Deliverable: task map document defining components + interfaces

Session 1..N (component execution):
  Each session reads the task map, works on ONE component
  Deliverable: completed component (code, data, writeup, etc.)

Session N+1 (integration, if needed):
  Agent reads all component outputs, resolves cross-component issues

[Optional: Session 0' (task map update):
  Agent reads task map + completed components, re-plans remaining work]
```

**Jörn's role:**
- Session 0: Co-author of task map. Defines the research question. Guides decomposition.
  Ensures components are session-sized and interfaces are low-complexity.
- Sessions 1..N: Selects which component to work on. Monitors. May work on components
  himself (e.g., math review).
- Integration: Decides when components are ready to integrate.

**Agent's role (Session 0):** Research scoper. Brainstorms how to make a question actionable
and verifiable. Proposes decomposition. Writes the task map document.

**Agent's role (Sessions 1..N):** Component implementer. Reads task map (full map, not just
own component — Jörn's point: "no interface description is perfect, it's better to say what's
beyond the interface as well" so the agent can disambiguate). Executes one component. Reports
results.

**Interface: the task map.** Must define for each component:
- What it produces (deliverable)
- What it needs from other components (inputs/dependencies)
- What's in scope and what isn't
- How to verify the component independently

**Phases are NOT strictly sequential** — Sessions 1..N can run in parallel (different
worktrees) if components are independent. Session 0' (re-planning) can happen mid-way when
early results change the plan.

### Proposed variant for this project

New pattern, not yet tried. Natural fit for **multi-experiment research questions** where
the question decomposes into independent computational/mathematical sub-questions.

Illustrative example (agent-generated, not validated by Jörn — actual decomposition may differ):
"Investigate local maxima of the EHZ capacity near the crosspolytope" might decompose into:
- Component A: gradient computation and verification (code + math.tex)
- Component B: numerical experiment on specific polytope families (code + data)
- Component C: theoretical analysis of when local maxima occur (math.tex)
- Component D: visualization of the capacity landscape (code + figures)

Each component would be a session-sized task. Components A and B might be prerequisites for C.

**Session boundary format for component sessions:**
- Input: task map path + component ID. Agent reads full map, focuses on one component.
- Output: completed component + brief report on what was done + any surprises that affect
  other components.

**Task map update triggers:**
- A component produced unexpected results that change the research question
- A component turned out to be much harder/easier than expected
- New components emerged from work on existing ones

**Open questions:**
- Should the task map be a single markdown file, or structured differently (e.g., TASKS.md
  entries, separate files per component)?
- Should a chat extraction from the task-map-creation session be attached to the map? This
  would give component sessions a compressed record of the discussion/reasoning behind the
  decomposition — more reliable than a summary, without the full session context.

**Task map update variants:** When the map needs updating (component failed, unexpected
results, re-scoping), two options:
- **Resume the creation session** — has the full discussion context. But that context may be
  stale/large.
- **New session reading only the map** — clean context, but lacks the reasoning that produced
  the map. The chat extraction idea helps here: attach it so the update session has the
  discussion without the tool call bloat.

### Experience

_Placeholder. Not yet tried. Key risk: task map quality. If decomposition is wrong, multiple
sessions execute the wrong thing before anyone notices._

---

## Pattern 6: TASKS.md Population → Deferred Execution

### Training priors

Agents know **backlog grooming / issue triage** from SWE training data. Writing task
descriptions that others will execute is a common pattern in project management docs,
GitHub issue templates, and sprint planning transcripts.

Also: **brainstorming → filtering → prioritizing** is a general reasoning pattern agents
handle well (list generation is cheap; evaluation/ranking is where they add value).

### Phases / roles

```
Session 0:  Agent explores, identifies work items, writes TASKS.md
[Jörn reviews, prioritizes, selects]
Sessions 1..N:  Each session picks one task and executes it
```

**Jörn's role:** Reviews the task list. Prioritizes. Selects what to work on and in what order.
May add/remove/rewrite tasks.

**Agent's role (Session 0):** Explore the problem space. Identify concrete, actionable tasks.
Write them up with enough context that a future session can execute them without the exploration
context.

**Differs from Pattern 5 (task map):** The tasks may be unrelated — they're a backlog, not
components of one decomposed problem. No defined interfaces between tasks.

### Proposed variant for this project

Current practice. Used when a session uncovers more work than it can do. The TASKS.md entries
serve as prompts for future sessions.

**Improvement opportunity:** Task descriptions in TASKS.md could be more structured — currently
they vary in quality. A template (goal, context, scope, verification) would help future
sessions.

### Experience

_Placeholder. Current practice. Works for scoping. Quality of task descriptions varies._

---

## Pattern 7: Progress-File Loop (Anthropic's Two-Agent Harness)

### Training priors

**Note: This pattern was not discussed with Jörn — it's from the research subagents' findings.
Training prior claims below are plausible but unverified.**

Agents likely know **iterative development with checkpoints** from SWE contexts: run tests →
fix one failure → run tests again → fix next failure. The progress file maps to a **TODO list
that gets checked off**.

Source: Anthropic engineering blog on building effective harnesses for long-running agents.

### Phases / roles

```
Setup:     Initializer agent creates environment + progress file (JSON) + feature list
Loop:      Coding agent reads progress → picks next item → implements → tests → 
           commits → updates progress → [compact/refresh] → repeat
```

**Jörn's role:** Defines the feature list (or reviews the initializer's version). Monitors.

**Agent's role:** Mechanical loop — one feature at a time. Each iteration is a mini
plan→execute cycle. The progress file is the interface between iterations.

**Key insight from Anthropic:** JSON for the progress file, not markdown — less likely to be
corrupted by the agent. Each iteration is independent: the agent reads current state from the
progress file, not from memory of what it did last iteration.

**Context management:** The loop naturally creates refresh points. After each feature, the
agent could compact or even /clear and resume from the progress file. This is the pattern's
main advantage — it converts a long session into many short independent iterations.

### Proposed variant for this project

Potentially useful for **implementation-heavy tasks** with many independent sub-items:
- Implementing a list of unit tests
- Fixing a list of clippy warnings
- Updating multiple experiment analysis scripts

Less useful for research tasks where the next step depends on understanding the previous
result (not just knowing it was "done").

**Adaptation for research:** Instead of JSON progress, use a structured markdown file that
includes not just "done/not done" but brief results. The next iteration reads the results
to decide what to do. This blurs into Pattern 5 (task map) with iteration.

### Experience

_Placeholder. Not yet tried. Anthropic reports it works well for mechanical coding tasks._

---

## Pattern 8: Review Phase (End-of-Session QA)

### Training priors

**Note: This pattern was discussed briefly (Jörn's /pre-merge workflow) but the full writeup
is agent-elaborated. The same-session vs fresh-session tradeoff and hybrid proposal are
agent opinions, not validated.**

Agents know **code review** well (pre-training + RLHF). Also: **peer review of academic
papers** — check claims against evidence, flag gaps.

Agents are biased toward their own code in the same session. Anthropic's docs note: "A fresh
context improves code review since Claude won't be biased toward code it just wrote."

### Phases / roles

```
[after execution phase of any pattern]
Review:   Main agent triggers /pre-merge → delegates to review subagents
          → reads findings → fixes issues
```

**Jörn's role:** Reviews the review output. Gates merge.

**Agent's role:** Delegates to specialized reviewers (review-proof, review-formalization,
review-claims, review-rust, review-python, etc.). Each reviewer has narrow scope. Main agent
synthesizes findings and fixes.

**Context tradeoff:** Doing reviews in the same session means the agent has implementation
context — useful for fixing unclear docs/comments where it remembers what it meant. But the
context is maximally polluted at this point (all execution history). A fresh review session
would have clean context but no implementation memory.

### Proposed variant for this project

Current practice. Two variants to compare:

**A. Same-session review (current):** Agent has implementation context. Better at fixing
"what I meant to write" issues. Worse at catching fundamental problems (biased by own work,
polluted context).

**B. Fresh-session review:** New session reads only the diff/files. Better at catching real
issues (fresh eyes, no bias). Worse at understanding intent behind code choices.

**Possible hybrid:** Same-session for quick fixes, fresh session for deep review of critical
math proofs.

### Experience

_Placeholder. Current practice is same-session. No comparison data with fresh-session review._

---

## How Patterns Compose

The patterns are not independent — they chain and nest:

- **Pattern 2 (plan→execute) failing → Pattern 4 (handoff→retry).** The most common
  composition. When execution fails or produces poor results, the recovery path is a handoff
  to a fresh session with learnings.
- **Pattern 5 (task map) uses Pattern 2 or 3 for each component.** The task map decomposes
  the problem; each component session follows plan→execute (Pattern 2) or plan→clear→execute
  (Pattern 3) internally.
- **Pattern 6 (TASKS.md) feeds individual tasks into Pattern 1, 2, or 3.** TASKS.md populates
  the backlog; each task is executed in its own session using whichever pattern fits its size.
- **Pattern 8 (review) is a phase inside Pattern 2 or 3**, not a standalone workflow. It
  happens at the end of execution.
- **Pattern 7 (progress loop) is an alternative execution strategy** within Pattern 2 — instead
  of executing the plan as one continuous phase, iterate through items with refresh points.

### Selecting a pattern

The selection is not formalized — Jörn picks based on the task. Rough heuristics:

| Task characteristics | Pattern |
|---------------------|---------|
| Small, clear, quick to verify | Pattern 1 (chat + execute) |
| Needs investigation, single session | Pattern 2 (plan → execute) |
| Complex plan, execution would exceed ~100K | Pattern 3 (plan → clear → fresh execute) |
| Previous attempt failed | Pattern 4 (handoff → retry) |
| Research question decomposable into independent sub-questions | Pattern 5 (task map) |
| Session uncovered more work than it can do | Pattern 6 (TASKS.md population) |
| Many independent mechanical sub-items | Pattern 7 (progress loop) |

These heuristics are Jörn's judgment, not rules. The boundaries are fuzzy.

### Future direction: formal comparison

This document could evolve into a more formal comparison grounded in established disciplines:
SWE project lifecycle, research methodology, team management. The patterns here map to known
concepts in those fields (sprint planning, advisor-student model, work breakdown structures),
but the mapping is informal. A formal comparison would make the design choices more explicit
and transferable.

---

## Primitives Reference

Building blocks that patterns compose from.

### Claude Code built-in commands

| Command | What it does | Effect on context |
|---------|-------------|-------------------|
| **/clear** | End session, start fresh in same terminal | Discards everything. New session has only CLAUDE.md + memory + system prompt (~20K). |
| **/compact** | Summarize context, continue with summary | Lossy — replaces full history with summary. Discards detail, exact quotes, tool output, nuance. Can pass custom instructions: `/compact "preserve X, Y, Z"`. Auto-fires at 95% by default. |
| **/resume** | Pick up a previous session | Restores full context from session JSONL. Useful for continuing after terminal close. **Note:** prompt cache has a 5-minute lifetime (1-hour option at 2x cost). After 5 minutes, /resume re-processes the entire context at full price — no cost advantage over extracting relevant parts via script and starting a fresh session. |
| **/fork** | Branch the current session | Creates a copy of current context in a new session. Both continue independently. Not discussed — utility for our workflows unknown. |
| **/<skill>** | Load a reusable text snippet / workflow | Injects skill template into context (~1-3K tokens). The skill text guides agent behavior for a specific task type. |
| **Plan mode** (Shift+Tab) | Agent can only write plan file | Changes system prompt. Agent thinks and discusses but cannot edit code or run commands. Forces planning before execution. |
| **Bypass permissions mode** (Shift+Tab) | Agent runs without permission prompts | Changes system prompt. Agent executes freely. No approval bottleneck. |

### Artifacts and persistence

| Primitive | Persists across | Token cost | What it discards |
|-----------|----------------|------------|-----------------|
| **Session** | Nothing (unless files written) | ~20K fixed overhead | Everything on /clear or exit |
| **Subagent (Agent())** | Nothing (returns summary to parent) | Subagent's full context (billed on same account, not in parent context) | All internal work; only summary enters parent |
| **Plan file** | Sessions (file on disk) | ~0.5-5K to read | Nothing — it's a file. Survives compaction (re-read from disk). |
| **Handoff file** | Sessions (file on disk) | ~1-3K to read | Discussion context that produced it |
| **TASKS.md** | Sessions (file on disk) | ~1-5K to read | Exploration context that identified tasks |
| **CLAUDE.md** | Sessions (always loaded) | ~4K (always paid) | Nothing — always present |
| **Memory (MEMORY.md + files)** | Sessions (always loaded) | ~5K (always paid) | Nothing — always present; 200-line index cap |
| **Skills** | Sessions (loaded on invocation) | ~1-3K per skill | Nothing — loaded fresh each time |
| **Git commit** | Repository | 0 | Nothing — full snapshot |
| **Worktree** | Isolated branch | ~0 (git overhead) | Nothing — full repo copy |

### Not-yet-built primitives

**Chat extraction**: Script to strip tool calls from session JSONL
(`~/.claude/projects/[hash]/[session-id].jsonl`), producing a compressed record of what was
discussed and decided. More reliable than compaction summaries because it preserves exact
human↔assistant messages. Cost: building the script + unknown how much context a future agent
can absorb from raw chat.

### Not explored

**Hooks** (trigger shell commands on events like tool calls, session start/end): Could
potentially automate some transitions (e.g., session-start hook that loads context, pre-compact
hook that saves state). Not discussed in this session — Jörn currently triggers transitions
manually.

---

## What to Try Next

### Testing methodology

1. **For each pattern:** a dedicated session prepares the required materials (skills, rules,
   templates), then tests the pattern on a concrete thesis project task. The session writes
   a report on how it went (what worked, what didn't, what was surprising).
2. **After several pattern-testing sessions:** a synthesis session reads all reports +
   interviews Jörn about his experience. Together they decide what patterns to
   keep/adjust/delete.
3. **Final result:** one or more skills that describe specific patterns/roles-inside-patterns,
   telling agents how to behave in their role. E.g., a skill for "research scoping session"
   that guides the agent through brainstorming with Jörn, decomposing into session-sized
   components, and writing a task map.

### Candidate patterns for first testing round

_To be selected. Each entry should specify:_
- _Which pattern (or combination)_
- _What concrete thesis project task to test it on_
- _What materials to prepare (skills, templates, etc.)_
- _What to observe / how to evaluate_

### Additional ideas for token reduction (orthogonal to patterns)

- **Chat extraction script:** build the JSONL → chat-only extraction tool. Test by attaching
  extracted chat to a task map as compressed discussion context.
- **Rust codebase documentation:** generate symbols + doccomments view so agents can understand
  the library without reading full function bodies.
- **Targeted compaction:** `/compact "preserve X, Y, Z"` with specific preservation
  instructions tuned per pattern/phase.

### Note on Patterns 7 and 8

Patterns 7 (Progress-File Loop) and 8 (Review Phase) were elaborated by the agent based on
research findings but Jörn has not engaged with or validated them. The training prior claims
and proposed variants in those sections are plausible but unverified. Review before testing.
