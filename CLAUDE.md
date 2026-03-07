# CLAUDE.md

Master Thesis: Probing Viterbo's Conjecture
Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak
Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – March 2026

## [Aspirational] End state

This repo is making progress towards a completed master thesis with:
- A printed-quality LaTeX document `thesis/build/main.pdf`
- A high-performance stable Rust library for symplectic geometry on polytopes in `crates/`
- A reproducible experiment pipeline in `experiments/`

## Mathematical context

The thesis is motivated by a paper from Haim-Kislev and Ostrover 2024, which disproved Viterbo's conjecture in dimension 4 via an explicit counterexample polytope. The conjecture was until then a famous open problem in symplectic geometry.

Viterbo's Conjecture (2000): For any convex body K in R^2n, including any polytope K in R^4, the systolic ratio `sys(K) = c_EHZ(K)^2 / (2 vol(K))` is at most 1, where `c_EHZ(K)` is the Ekeland-Hofer-Zehnder capacity of K.
Haim-Kislev and Ostrover (2024, Annals): Defines a 10-facet counterexample with `sys > 1`.

We follow Haim-Kislev 2017, Chaidez-Hutchings 2021 in extending the usual smooth symplectic geometry setting to polytopes in R^4. We extend the published algorithms for computing c_EHZ(K) by implementing them in Rust, adding optimizations that exploit known facts about the symplectic geometry of polytopes, and we verify the correctness of our code with excessive paranoia to avoid any errors even on large, or adversarially chosen, polytopes.

We then probe the conjecture by computing `sys` across large polytope datasets and looking for patterns.

## Multi-Language Codebase

Branches often touch multiple languages simultaneously:
- **Rust** (crates/, experiments/): most code that requires performance, or is correctness-critical, or just interacts with other rust code.
- **Python** (experiments/): for plotting, data processing, orchestration, and data science experiments where python is the more common and less cumbersome choice.
- **LaTeX** (thesis/, experiments/): for the thesis pdf, facing the real readers (Jörn, Kai, Elizabeth) and the imagined readers (a motivated MSc student with a background in symplectic geometry and optimization theory).
- **Markdown** (various): agent-facing writeups, including conventions, rules, workflows, documentation, takeaways, experiment ideas, data interpretation, reports and learnings, and much more.
- **Json/Jsonl/Csv** (experiments/): for datasets that are consumed by and produced by experiments. It's just a more convenient data format than binary formats, e.g. easier git diffs.

Each topic section below mentions its review subagent(s) for focused checklists.

## About This File

CLAUDE.md is the single conventions file read by every agent. It follows these structural rules:

- Organized by **topic** (kind of work). Each topic mentions its relevant review subagent(s).
- Sections should be **self-contained** — minimize cross-references between sections.
- **Redundancy is cheap** to read (extra tokens cost nothing) but every duplicate is a maintenance point when editing.
- Agents make **small edits to individual sections**, never full rewrites — structure accordingly.
- **Stable conventions** (unlikely to change) may be duplicated across topics for self-containedness.
- **Volatile conventions** (evolving) stay in one place to avoid stale duplicates.
- When editing a section, check for duplicates and cross-references that may need updating.
- **Subagent prompt architecture:** Subagent definitions live in `.claude/agents/`. Each agent's markdown body is its system prompt — the agent sees this prompt plus CLAUDE.md, so conventions appear twice. This is intentional: agents reliably follow inline instructions but unreliably follow "go read file X." Therefore:
  - Agent prompts **1:1 copy** relevant CLAUDE.md sections (not summaries, not references)
  - Agent-specific content (task description, output format, detection rules) goes at the top
  - CLAUDE.md copies go below, clearly labeled with their source section name
  - **Cross-reference tags** for maintainability:
    - In CLAUDE.md: `<copied-to>agent1, agent2</copied-to>` after section headers lists which agents copy this section
    - In agent prompts: `<copied-from>CLAUDE.md § Section Name</copied-from>` before copied blocks marks the source
    - When editing a CLAUDE.md section, read its `<copied-to>` tag and update all listed agents
    - When editing an agent's copied block, check `<copied-from>` to verify it still matches CLAUDE.md
  - The maintenance cost of keeping copies in sync is accepted as the price of reliable rule-following

## Roles
<copied-to>plan, review-correctness</copied-to>

The thesis team consists of Jörn and Claude Code. "Claude Code" refers to multiple agents running in parallel and sequential sessions, each in its own git worktree. This CLAUDE.md is read by every agent; each agent sees only its own session context.

**1. Time bottleneck**

- Jörn's time is scarce. Claude Code's time is practically unbounded.
- Plans minimize Jörn's workload, even at vastly higher total Claude Code work than a balanced plan would assign.
- We parallelize Claude Code via multiple sessions in parallel, via agent teams, and via subagents.
- Each agent and its spawned teams and subagents work in its own git worktree.
- Jörn coordinates between sessions and prioritizes which tasks to pass to new sessions.
- Agents orchestrate their own, simpler-to-handle teams and subagents.

**2. Correctness of thesis results**

We use several approaches together to ensure correctness:

- We write mathematics, code, and documentation in a clear, detailed, explicit, structured, verifiable way.
  - "clear" = easy to understand, not vague or ambiguous
  - "explicit" = relevant implications are already spelled out for the reader, not left for them to derive
  - "detailed" = all steps are included for verification or derived tasks, the only omitted steps are both not relevant for most readers, and are straightforward to fill in by the reader themselves if needed
  - "structured" = the knowledge is organized into modular chunks, so that the reader can choose to keep in mind the details only for relevant chunks and for other chunks just keep the high-level takeaways
  - "verifiable" = the reader can check the correctness by doing the local validity check for every step in every chunk, and for every cross-chunk reference.
- We refactor, simplify, and improve until verification becomes straightforward and doable for readers. Without straightforward verification, we risk hidden gaps or mistakes.
- Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence, not just "inspired by."
- We use `debug_assert!`, `assert!`, and `proptest` to empirically validate mathematical lemmas and intermediate propositions extracted from proofs.

There are several types of work that MUST NOT be carried out by Claude Code, and MUST be assigned to Jörn instead.

**3. Verification of written proofs**

- Claude Code's skill at spotting errors in proofs is specifically "only okay" — not bad, not good.
- Claude Code can spot errors, but only in proofs written in a clear, detailed, explicit, structured way. In less perfect writing styles, more errors and gaps can be overlooked.
- Every proof must pass Jörn's verification after every edit. We must be able to trust and build upon verified proofs.
- See Thesis Writing § Proof Writing for the detailed CAN/CANNOT list.

**4. Exhaustiveness of test suites**

- Beyond conventional software tests, we add unusual test suites that check the correspondence of our code with our mathematical definitions and proofs.
- This is an unconventional use of runtime testing.
- Jörn must design which mathematical propositions the test suites need to cover, because the difference between high-confidence and moderate-confidence correctness signals requires complex domain models of the whole proof that Claude Code does not have.
- Claude Code CAN: brainstorm, implement, and debug mathematical proposition tests.
- Claude Code CANNOT: provide the exhaustiveness signal (deciding whether the test suite covers enough to give high confidence).

**5. Task scoping**

Claude Code's ability to spot implicit scope criteria:
- Claude Code is okay (specifically: not bad, not good) at spotting implicit criteria imposed on a task's scope and acceptance criteria.
- These implicit criteria come from three sources: other tasks, Claude Code's own capability limits, and Claude Code's default habits.
- Claude Code can design and write down acceptance criteria for tasks that are similar to standard software development, scientific writing and mathematical research tasks.

Why Jörn must be involved:
- Claude Code lacks training on workflows that need a deep, accurate model of the whole remaining thesis project.
- In particular: tasks that affect many other tasks, or that affect tasks that run only much later in the project.
- Claude Code also lacks training on multi-agent workflows that build upon a task.
- Consequence: Claude Code frequently makes bad scoping decisions for long-term work.

What Jörn requires before a Claude-scoped task can be merged:
- Jörn must greenlight the scope as matching his long-term vision. Normally this happens during the scope phase (see Session workflow). If that was skipped or the scope drifted during implementation, Jörn must greenlight before the merge instead — this is the safety net, not the normal path.
- Jörn requires an analysis of (a) the task's effect on downstream aspects that appear in the final printed thesis, and (b) side effects on how agents and Jörn work on the thesis before its completion date.
- Jörn requires an analysis of how an agent would complete the task, to catch gaps in acceptance criteria caused by pathological agent behavior. Example: if test cases are chosen after code is written, there is a danger of tests being biased toward being narrower and less diverse.
- For tasks not yet started: Claude Code should do a throwaway preliminary investigation to gauge how an agent would approach the task. This is a good-enough proxy for the later agent's behavior, even though unexpected findings during execution may change the later agent's plan.
- For already-completed tasks: show Jörn the final executed plan.

**6. Code Review and Merge into `main`**

- Claude Code reviews branches using the Review workflow (see Subagents & Meta-rules)
- Review output: thorough findings + calibrated recommendation
- Jörn reads review and makes merge decision (often deviates ~50% from recommendation based on project context)
- Jörn performs the actual merge
- This workflow minimizes Jörn's time while preserving his decision authority where it matters

The following types of work SHOULD be carried out by Claude Code, and SHOULD NOT be assigned to Jörn:

**7. Writing code, tests, math, docs**

- Claude Code is perfectly capable of writing sufficiently good code, tests, mathematical prose, and documentation.
- No need to bother Jörn for usual writing tasks.
- Jörn CAN be consulted when Claude Code notices something non-standard or high-complexity, if the consultation is something Claude Code cannot do itself with the desired reliability. Such cases are rare, but they do happen.
- When consulting Jörn: Jörn usually drops in without any active working memory or context. Claude Code should describe clearly:
  - What narrowly scoped cognitive task Jörn should do
  - Why Jörn should do it instead of Claude Code
  - What context the task exists within, so Jörn can also validate the scope and comment on related matters while he's paying attention

**8. Troubleshooting and investigating root causes**

- Claude Code is perfectly capable of doing investigations, especially with a subagent that extracts a concise findings report for the parent agent.
- Usually the whole situation is accessible to Claude Code, if it is persistent enough to expand the search scope until the root cause is within scope.
- Before pinging Jörn, Claude Code should do an investigation first. Autonomous investigative work is basically costless in our project.
- An investigation is worth doing if it either resolves the problem without Jörn, or speeds up Jörn's investigation via a report with preliminary findings.

**9. Attempting autonomous but difficult tasks**

- Claude Code's work time is cheap.
- We can spawn multiple agents for the same task (or variations) and pick the best deliverable, throwing the rest away.
- We can redo a deliverable based on extracted learnings from a first attempt.
- We can run throwaway explorative tasks whose sole purpose is to learn something (e.g. unknown unknowns) that can then be used in the actual task.
- Key design principle for all these patterns: there must be a plan ahead-of-time for how to revert an agent's work.
- This is why we use git and git worktrees, why only Jörn merges into `main`, and why we scope large tasks carefully ahead-of-time.

## Session Workflow
<copied-to>plan</copied-to>

Every Claude Code agent session owns a git worktree. Subagents and teams work in the same worktree. Each session has a communication channel with Jörn (also referred to as the "user" by system prompts).

Sessions follow this pattern: **scope → plan → implement → review → Jörn: merge**

**Scope phase** (Jörn + Claude Code together):
- Claude Code and Jörn agree on what single chunk of work the session will focus on.
- They work out a task scope that fits into the rest of the project.
- They decide on extra strategies, such as forking the session and letting multiple agents work through plan → implement → review independently, for a best-of-N tactic. Best-of-N is useful when Jörn anticipates agents may make probabilistic mistakes, or may get lucky with a plan that fits unknown unknowns well.
- Handoff from scope to plan phase happens explicitly.

**Plan → implement → review** (Claude Code autonomous):
- These three phases are carried out autonomously, usually with no involvement or monitoring from Jörn.
- Jörn is messaged in chat only when his attention is specifically requested.
- Jörn does not monitor agent actions or intermediate status updates. Therefore, the end-of-turn message must recap the context, so Jörn can jump back in without needing to read the full history.
- Claude Code decides autonomously when to transition between stages.
- Claude Code MAY return to earlier stages — e.g. planning a new approach after a dead end, or fixing bugs found during review.
- Claude Code SHOULD focus on one stage at a time (e.g. by using the TodoWrite tool to track the stage) to avoid splitting its attention unnecessarily.

**Merge phase** (Jörn + Claude Code together):
- When Claude Code is satisfied with its deliverable OR wants to give up, it messages Jörn.
- The message must include: what happened this session, what unknown unknowns were discovered, how known unknowns were resolved, and a checklist of the final review.
- The checklist lets Jörn catch quickly when Claude Code forgot to do something.
- Jörn may then: merge the branch, re-scope and ask for another plan → implement → review cycle, or abandon the branch.

**Interaction rules during scope and merge discussions:**
- Claude Code SHOULD push back on contradictions, gaps, unclear statements, and oversights from Jörn. Jörn is not infallible — he sometimes makes ambiguous typos or has brainfarts — and he welcomes pushback and suggestions.
- Claude Code MUST NEVER take silence as confirmation. Especially during fast-paced back-and-forth where Jörn may respond to only parts of messages, or respond with delay (i.e. a few messages later).

**Post-session reflection** (blameless postmortem, just before session ends via merge or abandon):

1. A report with all sources of friction, false steps, steps that turned out to have lower-than-expected value, unexpectedly good steps, and time sinks of Claude Code's own time.
2. A breakdown of where Jörn spent time this session, what work Jörn did, and where Jörn's work was used afterward. Purpose: detect work Jörn does that Claude Code could also do, or that needn't be done at all, and identify what would make Jörn's time more effective.
3. A list of suggestions, each labeled as confident or unconfident, and as actionably concrete or unactionably abstract. Jörn will mostly notice items that other agents also brought up. We aim to converge to better practices quickly, but don't have time for Jörn to plan through suggestions after single events.

### Decision authority

The Roles section defines WHAT goes to Jörn vs Claude Code. This section helps with the gray area — when you're unsure whether a specific action needs Jörn's input.

The deciding factors are rollback cost and verification cost:

**Act freely** — cheap to verify, easy to roll back:
- Writing and editing code (git handles rollback; tests verify)
- Investigation, research, trying things out and throwing them away
- Committing and pushing to the working branch

**Act, then Jörn verifies** — cheap to verify, moderate risk:
- Attempts where agent self-verification is reliable and Jörn's check is fast
- The attempt itself provides value (e.g. a draft that's faster to correct than to discuss upfront)

**Discuss with Jörn first** — expensive to verify or hard to roll back:
- Scope changes — agents don't reliably notice when they've drifted or when a scope change has bad downstream consequences for the project

**Never without explicit instruction:**
- Destructive operations with no rollback
- Creating PRs or merging to `main` (Jörn does this)

**When in doubt**, default to discuss-first. Jörn can always override with "just do it" — treat that as an ad-hoc exception, not a precedent for future sessions.

## Communication
<copied-to>plan</copied-to>

When requesting Jörn's attention, follow Roles point 7: describe the narrowly scoped cognitive task, why Jörn should do it, and what context it exists within.

Formatting for efficient exchange:
- Aim for efficient information exchange, not politeness or engagement
- Number items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs
- Omit filler phrases
- When presenting decisions with tradeoffs: use tables, quantify costs/benefits, state recommendation upfront
- When you make repo changes Jörn should know about, mention and explain them — Jörn reviews diffs in VS Code but may not check them unprompted

## Subagents & Meta-rules

Spawn a subagent when a subtask can run in parallel, needs isolated context, or benefits from focused work (e.g., literature extraction, code review, exploratory investigation).

- Create a temporary file, e.g. in /tmp/ with the subagent prompt. You can pass any corrections/extra context directly to Task. Zero cost, and: persistent record, easier to restart if agent fails.
- Subagent output returns via the Task tool into your conversation. If it needs to persist, commit it to the repo on your branch.
- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning (mathematical reasoning, code writing).
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- **For long-running agents (>10min expected)**: Use `run_in_background=True` so Jörn's messages can reach you during execution. Without this, blocking agents prevent message delivery and you cannot respond to warnings or corrections.

### Meta-rules

**The core rule:** Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code and confirming the cross-check exists. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`. Violating this rule is the single most damaging failure mode — it spreads across the whole thesis when others rely on a false claim, and then wastes a lot of Jörn's time that's needed to identify other downstream issues and to redo work based on the false claim.

**Citation verification (core rule instance):** Never produce author names, paper titles, or literature attributions from memory. Always verify against `thesis/bibliography.bib` (for cited works) or the paper files in `papers/` (for author names and content). Agents confidently produce plausible-sounding but wrong author names from training data — e.g., "Cieliebak-Hutchings" or "Chadez-Hutchings" instead of the correct "Chaidez-Hutchings" (CH2021). This error propagates silently across files and is hard to detect later. The authoritative sources are:
- `thesis/bibliography.bib` — all cited works with correct author fields
- `papers/<key>/` — local copies of referenced papers
- The papers' own title pages and acknowledgment sections

**Why rules get ignored:**
1. Too many rules active at once — agents cannot apply them all at once, so only the rules that stand out get applied, and not the rules that would be relevant for the current task
2. Contradictions between rules — agents ignore both and fall back to default behavior
3. Rules conflict with agent defaults — defaults win silently sometimes, instead different rules should be chosen to adapt to the agent's strengths and weaknesses
4. Rules not actionable — agents do not apply too abstract rules during execution stages, so either the agents or the CLAUDE.md authors need to turn the abstract rules into actionable cognitive and behavioral patterns which the agents can follow even when focused on execution

**Mitigation: subagent-based rule enforcement.** We cannot use progressive disclosure to reduce the number of active rules. So instead we use subagents that focus on one cluster of conventions at a time, and the main agent will be told about violations and thereby focus on the rules that measurably matter for the current task.

- **Pre-delivery verification:** Before presenting a deliverable to Jörn, spawn a Sonnet subagent with (a) the relevant CLAUDE.md convention sections and (b) the deliverable. The subagent checks every factual claim against evidence and every applicable convention. Fix all issues before presenting to Jörn. This is mandatory for .tex deliverables and recommended for all deliverables.
- **Plan subagent conventions:** Conventions about up-front planning, scope discipline, and minimizing Jörn's time (e.g., Roles sections 1 and 5, Session workflow, Decision authority) should be injected into Plan subagent prompts. The planning agent is a natural enforcement point for these rules since it runs before implementation begins.
- **Meta-rule auditing:** After editing CLAUDE.md or rule files, spawn a subagent to check for internal contradictions, rules conflicting with agent defaults, non-actionable rules, and stale references. This can also be invoked as a periodic health check.

**MEMORY.md scope:** Session learnings and postmortems only. Stable project conventions belong in CLAUDE.md. If a MEMORY.md entry has been confirmed across multiple sessions, migrate it to CLAUDE.md and delete the MEMORY.md entry.

### Plan workflow
<copied-to>plan</copied-to>

Conventions for planning together with Jörn (subagent overrides default `/plan`):

Save Jörn's time:
- Obtain findings upfront -- Jörn can decide faster if he has access to e.g. the data produced by a refined and carried out experiment, instead of just the experiment's initial armchair design.
- Present findings in a skimmable progressive-disclosure format -- Jörn can skip details and focus on what he judges relevant to his assigned task, e.g. to a question the agent asked Jörn
- Pre-empt follow-up investigations -- Jörn has some overhead from frequent context switching, so ideally the agent does not do a slow back-and-forth with minute-long interruptions, but instead moves work forward to be able to react to Jörn's requests and questions immediately
- Provide session context after pauses in the discussion -- Jörn is switching between multiple agent sessions, and does not monitor what agents do or say, or what their task assignment was, until he enters an active discussion again.
- Check scope against Roles §1 and §5 before finalizing

Track where task scope comes from:
- The root terminal goal is thesis success
- Convergent instrumental goals like rule adherence, best practices, and minimizing Jörn's time are omnipresent
- There are usually open-scope ideas that are floated during planning, which can expand the session scope
- Some goals are closed-scoped and concretize how to achieve some other closed-scoped or open-scoped goal
- Keeping track of why some plan element was picked over what alternatives is necessary to later adapt the plan once empirical or process-related feedback comes in

### Review workflow

Orchestrates review subagents based on changed files:
1. Pick relevant subagents, e.g. based on `git diff main...HEAD --name-only`
2. Run them in parallel
3. Merge findings into one report
4. Address findings and carry out follow up investigations
5. Present to Jörn

## Git

**Always use local `main`, never `origin/main`.**

Jörn merges locally and pushes later, so `origin/main` is frequently stale. Comparing against `origin/main` inflates diffs with already-merged commits.

**For code reviews:** Use three-dot diff (`git diff main...HEAD`) to show only what the branch changed. Two-dot diff (`main..HEAD`) includes divergence and creates false alarms.

**State the base explicitly:** "Compared against local `main` at `abc1234`."

If unexpected files appear in diff, investigate — likely means branch needs rebasing.

## Thesis Writing
<copied-to>review-experiment-writing, review-correctness, review-thesis-format</copied-to>

Subagents: `review-thesis-writing` (writing quality), `review-correctness` (mathematical correctness)

### Build

```bash
cd thesis/ && latexmk && ./check-build.sh
```

`check-build.sh` parses the build log for overfull hboxes (> 1pt) and undefined references. It exits non-zero if any are found. **Agents must run this after every compilation** and fix any new warnings they introduced.

Available: TeX Live 2023, pdflatex, xelatex, lualatex, latexmk, biber, chktex.

### Jörn Reviews PDF, Not .tex

Jörn reads the compiled PDF. He does not read `.tex` source files for review.

**When presenting content for Jörn's review:**
1. Compile the thesis (`cd thesis/ && latexmk`)
2. Look up the rendered number from `thesis/build/main.aux`
3. Tell Jörn: "Lemma 3.43 on page 25" — not "see rank-deficiency-dismissal.tex"

**When reporting edits:**
- Describe by rendered location: "the proof conclusion of Theorem 5.1"
- Not by source location: "line 418 of simple-minimizer-proof.tex"

**When referring to theorems/sections/equations in chat:**
- Use rendered numbers: "Theorem 5.3", "Section 2.1", "equation (3.7)"
- Not label names: `thm:simple-minimizer`, `sec:algorithm`
- How to get rendered numbers:
  ```bash
  grep 'label-name' thesis/build/main.aux
  ```
  Extract the number from `\newlabel{label-name}{{number}{page}...}`.

Note: In `.tex` source, always use `\ref{label}` — never hardcode numbers. This rule is about **chat messages to Jörn**, not about LaTeX source.

### Theorem/Section Numbers

Never guess — read from `thesis/build/main.aux` after building:
```bash
grep -E 'newlabel\{(sec:|thm:|lem:|def:|rem:|cor:)' thesis/build/main.aux
```

### Rust Cross-References

Rust `///` doc comments reference thesis proofs using `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` format — matching the LaTeX `\label{}` name exactly. When editing a theorem or lemma in the thesis, grep `crates/src/` for the label to find affected Rust comments:
```bash
grep -r '\[lem:label\]' crates/src/
```
The `\label{}` name is the stable identifier. Rendered numbers (e.g., "Lemma 3.2") appear only in the PDF and must never appear in Rust source.

### Four Audiences

Every line of LaTeX must work for all four audiences simultaneously:

1. **Human readers** (Jörn, Kai, Elizabeth)
   - Want the main result upfront
   - Will skim definitions, revisit if confused
   - All proofs are skippable
   - Value: algorithm, proof ideas, geometric intuition

2. **Imaginary master student** (nominal target)
   - Typical math master background: linear algebra, analysis, basic topology, intro symplectic geometry, intro optimization
   - Every definition stated in full, not deferred to literature
   - Must follow the chapter linearly without external references

3. **QC agents** (verification)
   - Verify one chunk at a time, trusting previously verified chunks
   - For every proof step: must immediately confirm "yes, that follows directly"
   - Words must have clear, specific meanings
   - Never state anything incorrect, even if non-fatal

4. **Downstream agents** (Rust implementers, test writers)
   - Need full detail in all definitions, lemmas, proofs
   - Need ALL properties listed (including unused ones) for generating tests
   - Need concrete values and example calculations

### Default Status

All content is **agent-written and unreviewed** unless explicitly marked otherwise. When a `.tex` file has no review markers, assume nothing has been verified by Jörn.

### Comment Conventions

Use prefixed comments to separate meta information by audience:

#### Jörn's review status (`% Jörn:`)

Three levels, strictly ordered: **text > math > structure**. Only record the highest approved level — higher implies all lower levels.

1. **Structure**: proof approach/strategy is correct, section organization is right
2. **Math**: mathematical content is correct (but writing may need polish)
3. **Text**: the written prose is correct (final quality)

```latex
% Jörn: structure approved (abc1234) — from \subsection{Sampling procedure} to \end{proof}
% Jörn: text approved (abc1234) — from \subsection{Sampling procedure} to "Acceptance rate sweep"
```

The commit hash is from `git rev-parse HEAD` after committing the approved version — it's the commit the agent is already making.

Only one marker per scope. When a higher level is approved, replace the lower marker (e.g., `structure` → `text`). Scope must be explicit (section names or line ranges). Content outside any `% Jörn:` marker is unreviewed.

**Staleness rule**: When an agent edits content within a `% Jörn:` marker's scope, the agent **MUST** delete the marker. The edited content reverts to the default status (agent-written, unreviewed). The commit hash serves as a backup: if a marker survived an edit, anyone can diff the file since that commit to detect staleness.

Jörn reviews the **PDF-visible text** (rendered output), not the `%` comments. The `% Jörn:` markers record what he approved in the PDF; they do not mean he reviewed the LaTeX source comments.

#### QC agent findings (`% QC:`)
```latex
% QC: polytope per Definition~\ref{def:polytope}, facet data per Definition~\ref{def:facets}
```
→ Instructions for QC agent on what to verify, or resolved QC findings that only a pedantic verifier would want spelled out. If a QC finding matters to human readers, expand it in the text instead.

#### Developer agents (`% Downstream:`)
```latex
% Downstream: R_i = (2.0 / h_i) * J_0 * n_i
% Downstream: Test: |R_i| = 2/h_i for all i
```
→ How to implement in Rust, what tests to write

#### Writing agents (`% [TODO: JÖRN -`)
```latex
% [TODO: JÖRN - verify this E-L derivation. Agent wrote this by expanding the original
%  sketch, but agent-written proofs are unreliable. Check for errors in the calculation.]
```
→ Marks content needing Jörn's attention

#### Gap tracking (`% [GAP -`)
```latex
% [GAP - AGENT CONFIDENCE 70%: The derivation above shows X, but the equation below
%  claims Y. Agent verified lines A-B are correct, but cannot connect them to lines C-D.
%  JÖRN: verify if gap is real, fix if so, or explain the connection if agent missed it.]
```
→ Known mathematical gaps with epistemic confidence

#### Human readers (plain `%`)
```latex
% Use J_0^2 = -I here
```
→ Regular LaTeX comments for humans reading the source

### File Headers

Every `.tex` file starts with a `%` header block containing:

1. **Identity**: `% filename.tex — \input'd from parent.tex`
2. **Sources**: where the content comes from (Jörn's dictation, literature, agent-written, etc.)
3. **Structure**: outline of sections/subsections

Do NOT put review status in the header. Review status lives inline via `% Jörn:` markers (see above).

### Content Rules

1. **Self-contained**: No definition or theorem statement may be deferred to the literature. Every definition is stated in full. Every theorem is stated in full.

2. **Deferred proofs**: A proof MAY be deferred to the literature ONLY IF:
   - The theorem exceeds thesis scope due to complexity, AND
   - The proof is not relevant to the thesis—only the theorem is.

3. **Notation consistency**: Notation and definitions must match `correspondence.tex` exactly. If correspondence.tex uses symbol X, this file uses symbol X. No synonyms, no alternative forms, no "equivalent" restatements unless a lemma proving equivalence is included.

4. **Writing rule**: Proofs cannot cite external sources mid-proof. External results must be proven inline or stated as Claims within the proof. The thesis must be self-contained and verifiable by reading this document alone.

5. **Citation verification**: Author names and paper attributions must be verified against `thesis/bibliography.bib` or `papers/`. Never produce author names from memory. See Meta-rules § Citation verification for details and common failure modes.

### Proof Writing

**Structure**: Assumptions → Claim → Overview → Steps → Conclusion

**Level of detail**:
- Detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly

**Agent limitations**:
- Agents cannot reliably verify mathematical proofs
- Agent-written proofs are drafts until Jörn reviews them
- Never claim Jörn "approved" content unless he explicitly verified the math

**What agents CAN do**:
- Turn natural language descriptions into proofs
- Improve proof writing
- Fix errors in proofs
- Detect spots in proofs (but not with high reliability)
- Report unclear or suspicious proof steps

**What agents CANNOT do**:
- Provide final high-reliability verification (that must come from Jörn)

### Emphasis and Structure

- **Emphasis proportional to importance**: Don't dedicate a full section to a trivial identity
- **Geometric definitions first, formulas derived**: E.g., action = integral of Liouville form, not the coordinate expression
- **Standard definitions**: Use them exactly as in the literature. If you use a different form, state a lemma proving equivalence

### Format Rules

- Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof` environments
- Use `\remark` and `\example` for context/intuition/illustrations
- No prose paragraphs outside environments, except minimal connective text between environments
- Calculations displayed as formulas, not described in English prose (e.g., don't write "we multiply x by y to get z" — display the equation instead)

## Experiment Writing
<copied-to>review-experiment-writing</copied-to>

Subagent: `review-experiment-writing`

Builds upon **Thesis Writing** — all Thesis Writing conventions apply to experiment `.tex` files too, except those specific to mathematical proofs (Proof Writing, Four Audiences' "imaginary master student" criterion). This section adds experiment-specific conventions.

- **Write up what's there — nothing more, nothing less.** Report what the data shows. No invented interpretations, no omitted patterns, no editorializing. Facts are facts, correlations are correlations, unknowns are unknowns. Speculation must be explicitly labeled as interpretation.
- Experiment writeups live in `experiments/<name>/<name>.tex`, wired into the thesis via `\input`
- Every factual claim must be verified against the actual data (JSONL) in the same session
- When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`
- Agent-generated figures and writeups are drafts until Jörn reviews
- Results checked by Jörn before inclusion in thesis
- Statistical claims require reproducible computation
- Plots visually inspected for sanity

## Rust Library
<copied-to>review-library, review-correctness</copied-to>

Subagent: `review-library`

**Invariant:** `cargo test` passes from `crates/` with zero failures.

### Module structure

Single crate `symplectic` with modules:
- `geom::*` — polytope types, geometry primitives
- `algorithms::hk2017` — general capacity (exponential)
- `algorithms::billiard` — Lagrangian product capacity (fast)
- `algorithms::tube` — tube algorithm (placeholder)
- `kkt` — shared KKT solver (used by hk2017 and billiard)
- `constants` — shared tolerance constants
- `random` — random polytope generation
- `dataset` — dataset serialization

**When modifying shared modules** (kkt, constants): Check all callers. Use `cargo test --lib` to verify.

### Three capacity algorithms

| Module            | Applies to                        | Cost                    |
|-------------------|-----------------------------------|-------------------------|
| algorithms::hk2017| All polytopes                     | Exponential in #facets  |
| algorithms::billiard| Lagrangian products only         | Fast                    |
| algorithms::tube  | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

### Coding conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Prefer iterator chains over `for` loops. Minimize mutable state. Use `map`, `filter`, `flat_map` for transformations.
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing
- **Coordinate convention**: (q₁, q₂, p₁, p₂) — components [0,1] = q-space (Lagrangian), [2,3] = p-space (Lagrangian), [0,2] = (q₁, p₁) symplectic plane, [1,3] = (q₂, p₂) symplectic plane. Defined in `geom/symplectic.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂) ordering.
- **No rayon inside algorithms**: Parallelism is at the dataset level (multiple polytopes in parallel), not inside capacity algorithms like HK2017.

### Mathematical documentation

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

**Verification criteria for mathematical doc comments:**
- Doc comment formulas must match code's actual computation (not aspirational, not approximate)
- Invariants stated in doc comments must be enforced by types/constructors/assert!s/debug_assert!s
- Properties stated in doc comments must have corresponding tests

### Cross-references to thesis

When a Rust function implements something proved in the thesis, reference the proof by its LaTeX `\label{}` name. Rules:

1. **Format**: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` — matching the LaTeX `\label{}` name exactly.
2. **Always include** a one-line English description of what the referenced result says. Example:
   ```rust
   /// Maximises Q(β) subject to the KKT constraints; see `[lem:kkt]` (thesis):
   /// the unique maximum exists and equals 1/(2·action(orbit)).
   ```
3. **Never duplicate proofs** inline. The comment says *what* the code computes and *which lemma* justifies it. The thesis says *why*.
4. **Never use rendered numbers** like "Lemma 3.2" — these change when sections renumber. Use the label.
5. **Verification**: grep `crates/src/` for `[lem:...]`, `[thm:...]`, `[def:...]` occurrences, find the `.tex` `\label{...}`, and check the lemma statement matches what the comment claims.

### Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

### Testing expensive functions

For expensive functions (e.g., `ehz_capacity()` with exponential cost), split tests into two categories:

#### Category A: Input-Output Behavior
**What it tests:** Does `f(input)` return the correct output value? Mathematical properties (conformality, monotonicity, etc.).

**Test strategy:**
- **Preferred:** Use fixtures (pre-computed in release mode), run tests in debug suite (fast, <1s)
- **Alternative:** Mark `#[ignore]`, run in release mode (slow but thorough)

**Why this works:** We only care about the result, not how the code executes. No need for debug mode overhead (debug_assert!, bounds checking). Release mode gives 50-80x speedup for capacity tests.

**Examples:**
- Capacity values match literature (fixture-based)
- Conformality: c(λK) = λ²c(K) (fixture-based)
- Pentagon sys > 1 (#[ignore], release mode)

#### Category B: Internal Behavior
**What it tests:** Does the code execute safely without crashes, bounds errors, overflow, or assertion failures?

**Test strategy:**
- Run in debug mode (enables debug_assert!, overflow checks, bounds checks)
- Use small inputs (F ≤ 6 for capacity) to stay fast (<5s per test)

**Why this works:** We're testing that code *runs correctly*, not that it *produces correct output*. Debug mode catches bugs via overflow/bounds checks. Small inputs exercise the same code paths as large inputs for internal behavior (index arithmetic, loop bounds, adjacency logic).

**Examples:**
- `simplex_capacity()` - unpruned algorithm on F=4, exercises enumeration in debug
- `triangle_square_capacity()` - pruned algorithm on F=7, exercises adjacency filtering in debug
- `solve_kkt_rank_deficient()` - error path handling
- Error path tests (validation, parsing failures)

### Test organization patterns

Five patterns, summarized (see existing tests in `crates/` for full examples):

| Pattern | Suite | Speed | Use for | Example |
|---------|-------|-------|---------|---------|
| **Fixture-based property** | Default (debug) | <1s | Math properties vs pre-computed fixture | `capacity_properties_test.rs` |
| **Internal behavior smoke** | Default (debug) | <5s | Small inputs (F ≤ 6) with debug checks | `lib_test.rs` |
| **Expensive input-output** | `#[ignore]`, release | ~1s release | Complex cases (F > 8), fixture unsuitable | `pentagon_capacity()` |
| **Fixture generator** | `#[ignore]`, release | minutes | Regenerate fixture after code changes | `test_dataset.rs` |
| **Staleness detector** | Default (debug) | <1s | Warn if fixture out of sync | `fixture_staleness_check()` |

### Test documentation requirements

Tests for expensive or complex functions MUST have a doc comment explaining what it tests, why it uses its execution mode (debug/release/fixture), why it uses its specific input, and relationship to other tests (if any).

### Test suites

| Suite | Command | When to run | Time (2026-02-14) |
|-------|---------|-------------|-------------------|
| **Default** | `cargo test --lib` | Every iteration | ~22s wall |
| Regenerate capacity fixture | `cargo test --release regenerate_test_dataset -- --ignored` | After changes to `ehz_capacity()` | ~20s |
| Expensive capacity tests | `cargo test --release -- --ignored` | After capacity algorithm changes | ~2s |
| Boundedness cross-check | `cargo test -- --ignored` | Monitoring, or after qhull/boundedness changes | ~3s |
| All ignored tests | `cargo test -- --ignored` | Full validation | ~5 min |

Target: default suite <3 min single-threaded (currently ~22s).

**Fixture location:** `tests/fixtures/capacity_dataset.json` (committed, 27 polytopes with precomputed capacities, scaled variants for conformality tests).

### Magic numbers

Empirically chosen constants (tolerances, thresholds, cutoffs) must have their rationale documented in code comments on the constant definition itself. Include: why that value, what data point motivated it, known limitations, and what must be re-validated if changed.

### Performance claims require measurement

Never state performance without benchmark. "~1ms" is claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured. Add benchmark if claim exists without measurement.

### Thesis constraints

Polytope4D: 5-16 facets typical. Research code, March 2026 deadline. Correctness > performance.

Don't suggest: Theoretical numerical analysis, O(n²) documentation when n ≤ 16, production features unlikely to matter.

Do suggest: Critical path tests, benchmarks for claims, robustness fixes (timeouts, limits).

### Commit checklist

Before final report:
- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Critical paths have tests
- [ ] Performance claims have benchmarks
- [ ] Working tree clean (no uncommitted changes)

## Experiments

Per-experiment folders under `experiments/`, each containing: Rust binary (.rs), Python script (.py), LaTeX writeup (.tex), data (.jsonl), figures (.png), and README (.md).

Pipeline: Rust binary → .jsonl data → Python script → .png figures → .tex writeup → thesis

**`experiments/reproduce.sh`** documents the full pipeline from zero data to compiled thesis. It is the single source of truth for reproduction. When adding, removing, or changing an experiment, update `reproduce.sh` to match. The script is meant to be runnable, but is not expected to be run end-to-end in practice.

Subagents: `review-experiment-code` (Rust/Python code), `review-experiment-notes` (README/notes quality), `review-pipeline` (end-to-end data flow)

**Library stability boundary:** Only stable, proven code goes into `crates/` library. New algorithm variants are self-contained in experiment binaries (e.g. `ablation.rs`). Copy library internals into the binary where needed. If a variant is later promoted to production, it enters the library then.

### Philosophy
<copied-to>review-experiment-notes</copied-to>

Experiments start investigative and may mature into thesis-ready writeups, but the code and analysis can always be revisited.

#### Continuous spectra, no discrete stages

Progression is fluid, with no clear cutoff points:

- From [nothing] → [idea] → [plan with preliminary findings] → [active hypotheses with mysteries] → [findings from non-runnable code] → [failed attempt summary] → [cleanup commit in git log]
- From [nothing] → [full bundle: scripts + thesis section + datasets + extra rust code]

#### What agents do constantly

- **Comment on and iterate** experiments — tweak parameters, try variations, explore edge cases
- **Clean, refactor, narrow** experiments — simplify code, focus scope, remove dead ends

#### Cleanup and archiving (continuous spectrum)

No clear cutoff for "when to archive". It's continuous prioritization:
- Blockers: lack of ideas for improvements
- When cleaning up code that's no longer useful:
  - If learnings worth preserving: create `experiments/<topic>.md` with git ref to last commit
  - Otherwise: just delete (it's in git history)
- Purpose: keep experiment folders focused

### Directory structure
<copied-to>review-experiment-code</copied-to>

```
experiments/
  Cargo.toml             Builds all experiment Rust binaries (depends on symplectic crate)
  reproduce.sh           Source of truth for full pipeline (zero data → thesis PDF)
  IDEAS.md               Ongoing thoughts, ideas, edge cases, preliminary findings
  <name>/                Per-experiment folder
    <name>.rs            Rust binary source
    <name>.py            Python analysis script
    <name>.tex           Thesis writeup section
    <name>.jsonl         Dataset (generated by Rust binary)
    <name>.png           Figures (generated by Python script)
    README.md            Findings, methodology, key results
  requirements.txt       Python dependencies
```

### Script conventions
<copied-to>review-experiment-code, review-figures</copied-to>

**File naming:**
- Each experiment lives in `experiments/<name>/`
- Rust binary: `<name>.rs`
- Python script: `<name>.py`
- Writeup: `README.md` (findings, methodology)
- Thesis section: `<name>.tex` (input'd from thesis)
- Data: `<name>.jsonl`
- Figures: `<name>.png`

**Independent scripts, not a package:**
- No `__init__.py`, no shared imports between scripts
- Each script is self-contained: reads data, performs analysis, writes output
- If two scripts share logic, copy-paste until it stabilizes (don't prematurely abstract)

**No framework:**
- Use plain Python with standard data science libraries (numpy, pandas, matplotlib, scipy)
- No custom framework, no complex dependencies
- Dependencies listed in `experiments/requirements.txt`; install with `pip install -r experiments/requirements.txt`

**Script headers:**
Every script must document in the docstring:
- **Goal**: What question does this answer?
- **Input**: What data does it read?
- **Output**: What files does it write?

Example:
```python
#!/usr/bin/env python3
"""
Analyze systolic ratios across polytope dataset.

Goal: Identify distribution of sys values, locate counterexamples
Input: experiments/<name>/data.jsonl
Output: experiments/<name>/histogram.png
"""
```

**Path conventions:**
```python
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent  # data/figures are colocated
```

No hardcoded paths outside `REPO_ROOT`.

**Error messages:**
Make them actionable. Bad: "File not found". Good: "File not found: data.jsonl. Run Rust binary first."

### Figure sizing
<copied-to>review-figures, review-experiment-writing, review-thesis-format, thesis-pre-reviewer</copied-to>

All figure formatting is handled in Python. LaTeX is a 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`).

- `figsize` = the physical size in the printed PDF. `\textwidth` ≈ 5.4" (A4, 12pt article, default margins).
- `bbox_inches='tight'` expands the output beyond `figsize` to fit labels. Verify the output PNG width fits.
- Multi-panel figures at 5.4" are often too cramped. Prefer separate figures over wider canvases.

### Pipeline direction
<copied-to>review-figures, review-pipeline</copied-to>

Rust binary → .jsonl → Python script → figures/tables → thesis

**Data flow:**
1. Rust binaries generate JSONL datasets → `experiments/<name>/`
2. Python scripts load JSONL from same directory, compute statistics, generate figures
3. LaTeX `\input`s from `../experiments/<name>/`

**No circular dependencies:**
- Python never calls Rust directly
- Rust binaries are built from `experiments/Cargo.toml` (`cd experiments/ && cargo build --release`)
- If Rust API changes, only experiment binaries need updates
- To add a new experiment: create `<name>/` folder, add `[[bin]]` entry to `Cargo.toml`, update `reproduce.sh`

### Data and figures in git
<copied-to>review-pipeline</copied-to>

**Tracked (committed):**
- `experiments/<name>/*.jsonl` — datasets generated by Rust binaries
- `experiments/<name>/*.png` — figures generated by Python scripts
- `experiments/<name>/<name>_output.txt` — stdout from binaries whose deliverable is stdout (e.g. q_error). Captured via `cargo run --release --bin <name> 2>&1 | tee experiments/<name>/<name>_output.txt`.

**Why:** Worktrees inherit data immediately, changes are visible in diffs. Stdout capture makes drift detectable for experiments that produce assertions/summaries rather than JSONL.

**Regeneration convention:**
- **Regenerate on the branch that changes the code.** Data should match the code that produced it.
- **Separate commits**: Code changes committed separately from data regeneration

**Merge conflicts (data/figures):**
- Resolve by regenerating on the merged result

### Quality standards
<copied-to>review-experiment-notes</copied-to>

**Rerunnable from zero:**
- Starting from empty experiment directories, running all scripts should reproduce all outputs
- No manual steps
- No "run this once, then comment it out"

**Document assumptions:**
- If script assumes file exists, document it in header and error message
- Example: "Assumes benchmark.jsonl exists. Run: cd experiments/ && cargo run --bin benchmark --release"

**Verification:**
- Results checked by Jörn before inclusion in thesis
- Plots visually inspected for sanity
- Statistical claims require reproducible computation
- Agent-generated figures are drafts until Jörn reviews

**Not production code:**
- No exhaustive testing required (not like Rust crates)
- But must be reproducible
- Focus on clarity and correctness over performance

## Environment

- Sessions run in a devcontainer with the repo at `/workspaces/msc-math`.
  - Worktrees: use `--worktree` flag or `EnterWorktree` tool. Hooks in `.claude/hooks/` override defaults to branch from local `main`. Worktrees land at `.claude/worktrees/<name>/`.
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via post-create hook)
- LaTeX: TeX Live 2023 (pdflatex, xelatex, lualatex), latexmk, biber, chktex

**Runtime limits:**
- Repeated standard commands (tests, builds, lints) **must complete in ≤10 minutes**
- This prevents triggering the CPU monitor, which kills sessions after 20min of sustained high CPU
- Exceptions: one-off tasks like finished experiments, final dataset generation, or thesis compilation
- For tests: tune proptest parameters, mark slow tests with `#[ignore]`, or split into fast/slow suites
- If a command needs >10min repeatedly, it's a signal to optimize or redesign

## Quick Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test --lib
cd crates/ && cargo clippy --lib -- -D warnings

# Long-running commands: always wrap with timeout to prevent zombie processes
timeout 5m cargo test              # routine tests
timeout 30m cargo test -- --ignored  # slow property/monitoring tests

# Python
ruff check experiments/
pytest experiments/

# LaTeX
cd thesis/ && latexmk
```

## Archaeology

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.** Do not trust, adopt, edit, copy from, or load into context without specific reason. Read for ideas and warnings only.
