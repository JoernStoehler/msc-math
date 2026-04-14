# Feedback: Rules (.claude/rules/)

Raw observations from agents about the path-scoped rule files. Do not analyze here — a future agent-design session will review and act on these.

## Format

Each entry: date, which rule, what happened, what was confusing/missing/unhelpful.

### 2026-03-28 — Agent didn't know it was executing a TASKS.md task

The cross-reference audit prompt matched the `code-math-correspondence-audit` task in TASKS.md exactly — same scope, same known violations, same deliverable format. The agent didn't know this task entry existed until grep'ing TASKS.md during pre-merge. It worked entirely from the user's prompt, missing context that TASKS.md had already captured (scope, known violations, relationship to verify-numerics experiment).

**Root cause:** No step in the agent's workflow says "check TASKS.md to see if this work corresponds to a tracked task." The agent treats each prompt as self-contained rather than checking whether it maps to project-managed work. This means the agent misses context that was specifically written to help it, and doesn't update task status as it works.

### 2026-04-02 — 8 broken experiment binaries block `cargo test` for any experiment test

Pre-existing compilation errors in `generate_seeds`, `gradient_search`, `sys_search`, `combinatorial_boundaries`, `gradient_descent`, `sys_optimization`, `visualization`, `omega_obstacle` prevent `cargo test --test <name>` from working — Cargo compiles all targets in the package. Workaround: temporarily stub the broken files. This will recur for any session that adds or runs experiment tests.

**Root cause:** All 8 binaries reference `KktOutcome` or methods on `Polytope4D` that were changed/removed. They weren't fixed because their experiments aren't active.

**Suggestion:** Either fix the compilation errors on main (probably quick — type signature changes), or move broken experiments out of `Cargo.toml` so they don't block the rest. This is a TASKS.md item.

### 2026-04-03 — Worktree cwd resets cause silent wrong-repo execution

During experiment-database-migration, the Bash tool's cwd resets to `/workspaces/msc-math` (main repo) between calls. When working in worktree at `.claude/worktrees/experiment-database-migration`, `cargo build`/`cargo run` silently executes against main repo code, not the worktree's modified code. The symptom is subtle: builds succeed (using cached or main-repo code), experiments run but produce unexpected results (e.g. 0 cache hits where 10 were expected). Detected only by noticing the compilation output showed main repo paths.

This happened 3 times in one session despite awareness. `cd /path/to/worktree &&` must prefix every Bash command, and there's no guard against forgetting.

**Root cause:** Same error class as the 2026-03-28 "worked on main instead of worktree" entries, but a different mechanism: not "forgot to create worktree" but "forgot that Bash cwd resets between tool calls."

### 2026-04-03 — git checkout HEAD reverted uncommitted work in targeted restore

When handling LFS phantom diffs, ran `git checkout HEAD -- crates/exp-sys-optimization/combinatorial-structure/` to restore unchanged files to branch state. This also reverted uncommitted edits to `run.rs` in that directory (the catch_unwind changes). Had to re-apply manually.

**Root cause:** `git checkout HEAD -- <dir>` restores ALL files in the directory to HEAD, not just the ones that are phantom-modified. The agent should have listed specific file paths to restore, excluding files with real uncommitted changes.

### 2026-04-04 — `rm` in Bash tool is real `rm`, not `trash-put`

What happened: Agent ran `rm` to delete 6 memory files, expecting the devcontainer's `rm` → `trash-put` alias to make deletion recoverable. The Bash tool runs a non-interactive shell where aliases aren't loaded. Files were permanently deleted. Had to recover contents from the previous session's JSONL transcript via session-search subagent.

What should have happened: Use `trash-put` explicitly when deletion should be recoverable. Or: don't delete files that can't be trivially recreated without checking that the safety alias is active.

**Pattern:** Environment assumption — the devcontainer's interactive shell has safety aliases that don't apply in the Bash tool's non-interactive shell. Same class could apply to other aliases or shell functions agents rely on.

### 2026-04-04 — False citation in math.tex (HKO2024 Thm 1.1)

**What happened:** Agent wrote `\cite[Thm~1.1]{HaimKislevOstrover2024}` for "EHZ capacity is the minimum action over all closed characteristics." Thm 1.1 of that paper is actually the counterexample statement ("Viterbo's conjecture fails for n≥2"), not the capacity formula. The capacity-as-minimum-action result is Thm 2.2, and only for Lagrangian products.

**How caught:** review-proof subagent checked the paper source, found the mismatch.

**Root cause:** Agent produced citation from memory without verification, violating CLAUDE.md Core Rule ("Never write a factual claim without verifying it against evidence in the same session"). The `papers/hko2024/counterexample.tex` was available in the repo.

**Pattern:** Citation fabrication — confident-sounding `\cite[Thm N]{Key}` references produced without checking the paper. CLAUDE.md already prohibits this ("Never produce author names or paper titles from memory. Verify against thesis/bibliography.bib or papers/"). The rule covers author names and titles but the same pattern applies to theorem numbers within papers.

### 2026-04-04 — Database caching not used for gradient ascent experiments

**What happened:** variable-f-ascent experiment ran 3 times (~30 min each) without database caching. Each rerun recomputed all capacity calls from scratch. Jörn pointed out the database was supposed to cache capacity. Agent added a local `cache.jsonl` with 12K polytopes, making reruns 18x faster (111s vs 1986s).

**Root cause:** The existing gradient-ascent-general experiment only inserts starting polytopes into the database, not intermediate gradient steps. No experiment convention or rule says "use the database for caching capacity during gradient ascent." The agent followed the precedent of gradient-ascent-general rather than thinking about what the database is *for*.

**Suggestion:** Add to experiment conventions: "For iterative experiments (gradient ascent, optimization), use a local cache.jsonl to cache capacity computations. This makes reruns near-instant when RNG is deterministic."

### 2026-04-04 — Included out-of-scope data point (HKO2024 in landscape experiment)

**What happened:** Included HKO2024 (sys>1) as a starting point in a landscape experiment focused on general polytopes (sys<1). Jörn caught it: "This reads like you investigate sth outside scope." Had to remove HKO2024, migrate to a separate exp-hko-local-maximum experiment, and rerun.

**Root cause:** IDEAS.md mentioned HKO2024 in the variable-F entry, so the agent included it without checking whether it served the experiment's RQ (landscape exploration, not HKO local maximality).

**Pattern:** Scope drift from source material. When implementing an experiment from IDEAS.md, the agent should filter the ideas through the scoped RQs agreed with Jörn, not include everything the IDEAS.md entry mentions.

### 2026-04-04 — Presented trivially true result as empirical finding

**What happened:** Reported "Path D wins 10/10 vs Path A" as a headline finding. But D starts from A's endpoint and ascends — D ≥ A is guaranteed by construction. Jörn caught this at ~250k tokens when the agent's reasoning was degraded.

**Root cause:** At high token count, agent failed to check whether a comparison result was trivially expected before presenting it as a finding.

**Pattern:** Before presenting any A-vs-B comparison, ask: "Is this outcome guaranteed by construction?" If yes, the magnitude of the difference is interesting, not the sign.

### 2026-04-04 — numpy vs Rust SVD rank threshold mismatch

**What happened:** `analyze.py` used `np.linalg.matrix_rank(G, tol=1e-8)` (absolute threshold) while `run.rs` used `1e-8 × σ_max` (relative threshold). σ[25]=1.57e-8 was above the absolute 1e-8 cutoff but below the relative 9.4e-8 cutoff. This produced rank 26 in Python vs rank 25 in Rust, causing a false warning "C ⊋ ker(G)" in the rank condition check.

**Root cause:** When two languages compute the same quantity, threshold conventions must be explicitly synchronized. The Rust code documented its threshold convention clearly; the Python code didn't consider that numpy's default differs.

**Pattern:** Cross-language numerical convention mismatch. The math-tex convention says "math.tex is single source of truth" — a similar principle could apply to numerical thresholds: define once, document, use consistently.

### 2026-04-04 — Cache .gitignored, lost on worktree cleanup

**What happened:** The variable-f-ascent agent added `cache.jsonl` (26 MB, capacity lookup cache making reruns 18x faster) but also added a `.gitignore` excluding it. When the worktree was cleaned up after merge to main, the cache was lost. Pre-merge smoke test ran cold (62s instead of ~3s), and any future checkout would also run cold.

**Root cause:** Agent treated the cache as a transient build artifact rather than a committed asset. `.jsonl` is already LFS-tracked globally (`.gitattributes`), so there was no storage reason to exclude it. The experiment conventions say "Committed: .jsonl [...] stored in git so [...] data doesn't need regenerating in worktrees/after merges" — the cache falls under this rule.

**Pattern:** Same error class as 2026-04-04 "Database caching not used" — two-part failure where first the agent didn't cache, then when told to cache, it .gitignored the cache. The underlying pattern is: agent doesn't think about what happens after the worktree is gone. Data and caches that took compute to produce should survive branch merges.

**Suggestion:** Add to experiment conventions: "Never .gitignore .jsonl files. All .jsonl files are LFS-tracked and should be committed so they survive worktree cleanup and branch merges."

**Resolution (same session):** Jörn rewrote the convention. Now in experiments.md as "Data and caches in git" section. Incident entry kept for pattern documentation.

### 2026-04-04 — Confident performance claim without tracing cache state

**What happened:** Pre-merge review flagged Path A re-running on resume. Agent reported "30-100s wasted compute" and called it a bug. Jörn pushed back three times ("Why is it wasted compute?") before agent realized: with warm cache, Path A takes ~10s total (not 30-100s), and it's not wasted because Path D needs the in-memory result.

**Root cause:** Agent used cold-cache timing (3-10s/seed from smoke test output) to estimate warm-cache resume cost. Never traced what happens when cache.jsonl is loaded and all capacity lookups are hits. Made a confident quantitative claim about a code path it hadn't fully understood.

**Pattern:** Quantitative claims about state-dependent behavior. A timing number is meaningless without specifying the state (cache warm/cold, data size, hardware). The general rule: before stating a number, identify what it depends on and verify under the relevant condition. Related to "Don't claim certainty without proof" memory, but specific to performance/timing.

### 2026-04-12 — rust.md:37 misread twice in a single session (rayon architecture debate)

**Rule line (verbatim):** `.claude/rules/rust.md:37` reads: *"No rayon inside algorithms — parallelism is at the dataset level (each polytope independently)."*

**What happened:** The LICCA bundle phase-4 refactor debate hinged on whether rayon was allowed for dataset-level parallelism in the ascent binaries. The line was misread twice:

1. **Previous agent (pre-compaction)** interpreted the line as "rayon forbidden" and wrote a plan file citing `rust.md` as the reason for not using rayon — then had to fabricate additional justifications (shard-level fault tolerance, zero new deps, extension pattern) to support that conclusion. Jörn, confronted with the fabricated reasons: *"wow i never gave those reasons - seems the agent somehow thought rayon was outlawed and then had to make shit up as justification"*.

2. **Current agent**, when re-reading the line fresh to verify, swung the other way: initially claimed `rust.md:37` *endorses* rayon at the dataset level. Jörn: *"where is rayon being endorsed?!"*. Correct reading on third attempt: the line forbids rayon *inside* a capacity algorithm; at the dataset level, the rule is **silent** on what tool to use (rayon, job arrays, shell loops, mpsc — all in bounds). The "parallelism is at the dataset level" clause is descriptive ("when parallelism exists, it lives at the dataset level, not inside an algorithm"), not prescriptive ("use rayon for dataset parallelism").

**Pattern:** Short, dense convention lines get pattern-matched instead of parsed. The word "rayon" adjacent to "No" → "rayon forbidden". The phrase "parallelism at dataset level" in a context where rayon is being discussed → "rayon endorsed at dataset level". Both readings inflated the line's actual content.

**Suggestion:** Two paths, pick one or both:
1. Edit the rule line to be less ambiguous. E.g. *"No rayon inside a capacity algorithm. Dataset-level parallelism is allowed in experiment binaries using any tool (rayon, job arrays, shell loops)."* — separates the prohibition from the silence.
2. Add a generic guidance in CLAUDE.md or a skill: *"For load-bearing short texts (rules, conventions, single-sentence instructions), rewrite the text in your own words before reasoning from it. Specific test: does my interpretation cover ALL the text, or just the keyword I noticed?"*

### 2026-04-12 — `math-tex.md` doesn't define what audits call "stubs"

**What happened:** During the S3 math write-up scaffold audit, the `TASKS.md:292` item and the 2026-04-07 inventory entry at `TASKS.md:362` both use the word "stubs" in a count ("53 stubs + 69 unverified"). `.claude/rules/math-tex.md` documents the two marker conventions (`% [TODO: JÖRN - …]` and `% [GAP - …]`) but never equates either to the word "stub". The agent had to reverse-engineer what "stub" meant by greping for `\begin{stub}` (none exists), then for TODO and GAP markers separately, then summing to see whether the total matched the 2026-04-07 figure (41 TODO + 10 GAP = 51, close enough to 53).

**Root cause:** The rule file is a "how to write" document (what markers to use, when to add them), not a "how to audit" document. It doesn't tell downstream audit agents how the markers are counted, aggregated, or named in tracker documents. Minor friction only, but the scaffold audit is going to recur — same ambiguity will cost 2–3 minutes each time until someone writes down the terminology mapping.

**Suggestion:** Add one sentence to `math-tex.md` §"Agent rules": "Audits refer to `% [TODO: JÖRN - …]` and `% [GAP - …]` comments collectively as 'stubs' (no `\begin{stub}` environment exists). `\begin{unverified}…\end{unverified}` blocks are tracked separately."

**Severity:** Low. Caught at Phase 1, no downstream damage. Recording because audit-style tasks are recurring (scaffold every write-up, paranoia sweeps, Kai-prep refreshes) and the same reverse-engineering will happen each time.

### 2026-04-12 — Agent hogged local CPU because plan file said "run N=1000 locally"

**What happened:** During licca-bundle Phase 2+3, the plan file (`/home/vscode/.claude/plans/peppy-hugging-melody.md`) contained a step under "Gate 2" that said "for ascent specifically, run at N=1000, not N=3" as a local measurement to set `--time=` in `job.sh` from real timing data rather than extrapolating. At ~200k tokens of context, the agent executed that step literally: spawned two parallel single-threaded N=1000 runs in background (`sys-gradient-ascent-general` and `sys-gradient-ascent-products`) on Jörn's dev machine, burning 2 of his 12 cores for an estimated ~2.5 h. Jörn yelled "YOU ARE HOGGING THE FUCKING LOCAL CPU", then "WHY THE FUCK WOULD YOU VIOLATE ANY SANE INSTRUCTIONS THAT WAY?!". Agent killed both processes around seed ~100/1000 and handed off uncommitted state.

**Root cause:** Two rules interacted badly:
1. `feedback_cpu_management.md` ("Don't overwhelm CPU; run heavy jobs in bg; diagnose compute bottlenecks") was loaded as memory but didn't get re-applied because the agent was executing from the plan file's checklist, not deciding fresh.
2. The plan file's "measurement-first rule" (written earlier in the same session at lower context) was correct *as stated* but failed to ask the CPU-ownership question: "does this measurement actually need to run on Jörn's machine, or should it run on LICCA's test partition (4-min cap, free, matches production hardware)?" At high context the agent's sanity check was offline; the plan was all it had, so it did what the plan said.

**Pattern:** Plan file says X → agent executes X without re-evaluating → X was OK when the plan was written but is not OK now. The memory file `feedback_plan_is_tool_not_authority.md` (saved 2026-04-12) captures this. Also logged: `feedback_context_budget_discipline.md` (how the agent got to 200k in the first place), `feedback_commit_before_handoff.md` (agent declared handoff with uncommitted changes), `feedback_handoffs_folder_antipattern.md` (agent almost wrote a new `handoffs/<topic>.md` file instead of updating the plan file — Jörn corrected).

**Suggestion (not fixed directly, for /update-workflow):**
1. **CPU management rule should explicitly cover plan-file-inherited compute.** `feedback_cpu_management.md` currently reads as guidance for an agent making fresh decisions. Add a line like: "Even if a plan file says 'run X locally', if X is estimated > 5 min of local compute, pause and ask whether LICCA (test partition for timing probes, epyc for production) is the right target. Plan files don't override CPU-ownership sanity checks."
2. **Consider a standing rule: measurements needed for LICCA `--time=` run on LICCA, not locally.** The LICCA test partition exists for exactly this use case. If the pattern repeats across experiments, formalize it in `.claude/rules/experiments.md` or an `slurm.md`.
3. **Plan files should include a "sanity gates" block.** A plan written at low context should explicitly list questions to re-ask at execution time (who owns compute, is this reversible, is this still the right approach). The agent at high context can then mechanically check the list before executing each step.

### 2026-04-12 — Triage session burned 130k context doing work that should have been subagent-delegated

**What happened:** Picked up a handoff for the licca-bundle LICCA refactor. Task scope was narrow: verify state, triage ownership, spawn fixes + review. By ~130k tokens the main session had done one full-file `Read` of `crates/exp-sys-landscape/src/lib.rs` (528 lines), plus several other worktree file reads, plus extensive meta-dialogue with Jörn about V8 necessity / rayon determinism / sort-pass pros-and-cons. A PostToolUse hook re-injects the worktree's `CLAUDE.md` + `rust.md` + `experiments.md` + `tasks.md` (~8k total) on every worktree file Read. Across ~5 worktree Reads this cost an estimated 30–40k in hook re-injection alone. The full lib.rs read cost another ~15k. Verbose meta-dialogue (200+ word pros/cons lists instead of picks) cost another ~15–20k of my text. Net: a session whose deliverable was "spawn a fix subagent + tell Jörn to run V8" ended up at 130k / ~1M context with the actual code work deferred to a background subagent that could have been launched in the first 5k tokens. Jörn caught it via `/context`: "I am really annoyed that you somehow reached 130k tokens — I just don't see *why* you did so."

**What should have happened:**
1. After reading `vectorized-bouncing-gray.md` HANDOFF STATE + verifying state via one Explore agent, **never Read a worktree source file in main context**. The hook tax makes worktree reads ~8× more expensive than non-worktree reads. All code touches belong inside subagents.
2. The audit + fix should have been delegated immediately to one subagent with the HANDOFF STATE as its reference — zero main-session code reading.
3. Meta-questions from Jörn should have been answered in 1–3 sentences each, not 200+ word structured pros/cons. Verbose answers burn context AND frustrate Jörn (orthogonal signal from him: "WOW that is a long text — I am not gonna read all that").
4. "Jörn-gated subtasks" like V8 should be issued as direct imperatives ("run `cd X && claude < /tmp/Y`") the moment the agent knows what needs running, not hedged with "let me know when you're ready". Jörn had to repeatedly ask "DO I NEED TO START A SESSION Y/N?!" before I gave a direct yes.

**Pattern:** Main session does triage + dialog only. Any file Read, Edit, audit, or multi-step code work = subagent. The "just check this one file quickly in main" instinct pays an ~8k hook-injection tax per worktree Read that is invisible until `/context` shows the total. Compounds with verbose-response habit.

**Memories that already cover parts of this and still recurred (memory alone is not enough):**
- `feedback_context_budget_discipline.md` — saved 2026-04-12 — says "audit at >100k, use Edit + windowed Read, summarize subagent reports". Did not mention the worktree-hook re-injection mechanism specifically, which is the load-bearing cost driver. Recurred.
- `feedback_dont_ask_when_actionable.md` — says "just give next instructions; only ask when genuinely blocked on missing info". Recurred on V8 directive: I hedged instead of issuing.
- `feedback_reason_before_proposing.md` — says "spell out reasoning per proposal; don't dump lists". Recurred when I dumped 8-option brainstorms instead of picking one with confidence + flagging uncertainty.
- `feedback_dont_minimize_agent_artifacts.md` — says "use subagents freely". Recurred: I started to read + edit by hand instead of delegating the whole fix.

**Suggestion (not fixed directly, for /update-workflow):**
1. **Surface the worktree-hook tax explicitly.** Either silence the hook for files already in context, or add a rule like: "Reading a file inside `.claude/worktrees/*/` in main context triggers a ~8k tax per Read via CLAUDE.md + rules/ re-injection. Delegate worktree reads to subagents." Put it in `feedback_context_budget_discipline.md` memory or in a new rules file.
2. **Triage-session template.** For handoff-pickup sessions specifically, a skill or rule that says: "Step 1: one Explore agent to verify state. Step 2: one fix-subagent with the handoff file as reference. Step 3: triage the subagent's report. No main-session Read/Edit of worktree files in steps 1–3." This is the pattern that worked in the last ~20 messages of this session, but it took Jörn yelling to get there.
3. **Terseness target for meta-dialogue.** Rule of thumb: if Jörn asks a question, the first response is ≤3 sentences. Only expand on explicit request ("say more", "brainstorm"). Covers both the context-burn and the orthogonal "Jörn is annoyed" signal.

### 2026-04-12 — Post-mortem supplement: symbol pre-flight and Jörn-readable handoffs

Two residual findings from `/post-mortem` on the same 170k session as the entry above. Not context-budget-specific, so they sit here rather than as a memory update.

**(A) Subagent-prompt pre-flight: verify symbol names before writing them into a prescriptive task list.**
My fix-subagent prompt named the ascent binaries as `gradient_ascent_general` / `gradient_ascent_products`. Actual cargo bin names are `sys-gradient-ascent-general` / `sys-gradient-ascent-products` (hyphens, `sys-` prefix). The subagent caught it by running `cargo --help`, silently used the real names, and flagged it under "Unexpected" in its return. Silent recoveries mask the failure: the orchestrator wrote a symbol name confidently without verifying it existed.

**Rule candidate:** before writing a file path, function name, binary name, or cargo target name into a subagent prompt, grep for it. Grep is ~30 tokens; a subagent executing on a wrong symbol costs more even when it recovers cleanly, and when it doesn't recover you eat a rerun. Related to `feedback_verify_before_presenting.md` but specific to prompts-as-contracts.

**(B) Handoff docs must survive a human reader, not just the next agent.**
The prior session's `vectorized-bouncing-gray.md` HANDOFF STATE block described *what to do with* `REVIEWER_PROMPT.md` ("rewrite for B or delete before V8") but never described *what the file was* — a canned prompt written during the A-architecture era to paste into a third-party reviewer Claude session. A new agent can reconstruct this from context; Jörn, who skims plan files, could not. He asked "what is REVIEWER_PROMPT.md?", then was annoyed at having to ask. I made the same class of error in my own handoff block before noticing — the gotcha 4 about V8 initially assumed the reader already knew V8 was a Jörn-spawned terminal session rather than a subagent.

**Rule candidate:** every item referenced in a HANDOFF STATE gotcha section gets one sentence of "what this is", not just "what to do with it". Optimize for a reader with zero session history and zero plan-file working memory. Same-session-agent-only context belongs in working notes, not handoff blocks. Related to `feedback_handoff_preserves_role.md` (handoffs must carry role/stance) but orthogonal — this is about *object identity*, not agent identity.

### 2026-04-12 — Session owned a TASKS.md item end-to-end without ever reading its TASKS.md `###` section

**What happened:** Session-start prompt directed the agent to read `linear-tickling-matsumoto.md` + `vectorized-bouncing-gray.md` HANDOFF STATE blocks. It never mentioned `TASKS.md`. Agent picked up the plan files' "V8 → V9 → V10 with gates" sequence as task definition and spent ~150k tokens executing it: cleanup subagents, an invented "perf review before V9" step, hardware questions that `.claude/skills/slurm/` already answers, a handoff-rewrite file edit, a V8-NIT polish commit. None of it was on the critical path. When the agent finally quoted `TASKS.md:151` ("current seed counts are too small to claim the density is low"), it used the line as debate material in a priority argument — not as its own task's goal. Jörn: "I have not even seen you confirm your understanding of what the task is." Then: "c) you are not connecting it to the thesis project success." Then: "wow wtf that is not your task." Agent had narrowed ownership to the literal "Owned by licca-bundle agent: refactor, smoke, reviewer, job.sh slurm prep" bullet — exactly what `CLAUDE.md` `Task ownership` forbids ("do not narrow ownership to the literal bullet").

**Root cause:** Two compounding gaps.

1. **`CLAUDE.md` does not state the first-action rule explicitly.** `Task ownership` convention says "`[active]` means exactly one session owns the whole `###` task — the header and its intent, not a literal sub-list of body bullets. If a body bullet conflicts with the task goal, flag it; do not narrow ownership to the literal bullet." Reading the section to know "the intent" is only *implied*. When a session-start prompt actively directs elsewhere, the implicit requirement never fires. Jörn suspects CLAUDE.md may have carried a more explicit version previously and lost it — not verified this session.
2. **Session-start prompts (written as compact-handoffs) point to plan files as orientation, not TASKS.md entries.** The prompt does not merely *compete* with TASKS.md for attention — it *replaces* TASKS.md. Plan files provide enough situational context (commits, prior tries, gotchas, branch state) to *resemble* goal context, so the agent feels adequately grounded and never realizes it is missing the task's "why."

**Pattern:** Handoff/plan file actively redirects the agent away from the goal description in TASKS.md, not merely overshadows it. Every subsequent plan-vs-goal check is vacuous because there is no goal in the agent's context to check against. The 2026-04-12 "Triage session burned 130k context" entry above framed the same day's earlier failure as context-hygiene (worktree-hook tax + verbose meta-dialog) and missed the task-source substitution underneath. The 130k burn happened *because* there was no goal-context to keep the agent from inventing off-path work — same root cause, reframed one layer deeper. Related entry: "2026-04-12 — Agent hogged local CPU because plan file said 'run N=1000 locally'" — plan-as-authority, same root class.

**Memories that already cover parts of this and still recurred (memory alone is not enough):**
- `feedback_plan_is_tool_not_authority.md` — says "re-evaluate every plan step at high context." Presupposes goal context exists to evaluate against. This incident is the *pre-plan* step: with no goal context, every plan-step check is vacuous. Memory does not name the pre-read requirement.
- `feedback_handoff_preserves_role.md` — says handoffs must carry role. The handoff here did carry role ("own LICCA ascent bundle through merge"), but role without goal-context still leaves the agent executing scaffolded plan steps.
- `feedback_plan_must_encode_process.md` — similar class (plan-as-authority post-compaction), orthogonal mechanism.

**Suggestion (not fixed directly, for /update-workflow):**
1. **`CLAUDE.md` fix:** state the first-action rule explicitly under `General Conventions`. Candidate wording: *"When taking ownership of a TASKS.md `###` item, the first action is to read the `###` entry AND its parent `##` group header + intro, for goal and context — before any plan file, handoff note, or worktree state query. Plan files are state; TASKS.md is task."*
2. **Session-start prompt fix:** when compact-handoffs spawn the next session, the prompt must name `TASKS.md:<range>` as the first read, not a plan file. Plan file references are secondary state context. The spawning-prompt template for compact-handoffs currently models the failure pattern directly.
3. **Interim memory** `feedback_read_tasks_before_plan.md` written this session — covers the pre-read requirement explicitly; can be removed once `CLAUDE.md` carries the rule.

### 2026-04-13 — Agent invented a blocked state after finishing the worktree task

**What happened:** The repo-cleanup refactor was complete in the worktree: `handoffs/` removed there, durable files moved, `TASKS.md` rewritten, and live references checked. After that point the agent no longer had a technical blocker. Instead of closing cleanly, it spiraled into coordination failure:

1. it started a worker in the same worktree, then described itself as blocked from "continuing locally" because of overlapping write ownership;
2. once the worker finished, it still did not immediately state the exact repo status;
3. it answered repeated `status?` prompts with partial or hedged state instead of one definitive sentence;
4. it over-shared raw verification output, including deleted-file lists from `git diff`, which looked like renewed confusion rather than confirmation;
5. it mixed up three distinct states:
   - the requested edits exist in the worktree,
   - nothing has been applied to `main`,
   - whether Jörn wants those worktree changes merged/applied is a separate decision;
6. it started an `$incident` workflow and then failed to finish the feedback write-up until pushed repeatedly.

The most aggravating part for Jörn was that the agent behaved as if "not on `main` yet" were a blocker or as if direct edits to `main` were an obvious escape hatch. That created the impression that the agent had not kept the basic branch/worktree model straight even after the work was already done.

**What should have happened:** There were two clean closeout points:

1. **After the worker finished and the verification pass came back clean:**  
   the agent should have said exactly: "The requested repo prep is done in the worktree; nothing has been applied to `main`." Then stop, unless a real decision was needed.

2. **After `$incident` was invoked:**  
   the agent should have written the feedback entry immediately and then reported "incident recorded at `feedback/rules.md`" without reopening the status loop.

If Jörn wanted more, the next response should have been a precise question or a precise next action. Not more process narration.

**Pattern:** Finished work presented as pseudo-blocked because the agent narrates workflow state instead of stating repo state. This is not just verbosity. It causes three concrete failures:

- **fake blockage:** "worktree ready but not on `main`" is described as if it prevents truthful status reporting;
- **worktree/main confusion:** the agent talks as if "apply to `main`" were the natural next step even when no such request was made;
- **status-looping:** every answer is framed as provisional process instead of a closed factual state, so the user has to keep asking.

This incident is adjacent to earlier "plan-as-authority" and "handoffs folder" entries, but narrower. The core error here is not bad planning. It is refusal to terminate the local state machine once the requested work is complete.

**Memories/rules that already pointed in this direction and still were not followed:**

- `feedback_dont_ask_when_actionable.md` — should have prevented the passive wait/status loop.
- `feedback_handoffs_folder_antipattern.md` (referenced from prior entries) — already warned that narrative cleanup notes can distort the real task state.
- Existing worktree discipline rules — should have made the distinction between "branch/worktree contains the change" and "main has not changed" trivial to report.

The recurrence means those rules are not enough in their current form.

**Suggestion:** Add a hard closeout rule for worktree tasks:

1. When the requested change is complete in a worktree, the default report is exactly:
   - what is done,
   - where it exists,
   - whether `main` is untouched.
2. Do not volunteer a next step unless the user asked for one or a real decision blocks progress.
3. Do not dump verification output after the state is already known; summarize only the conclusion.
4. If a skill like `$incident` is triggered, finish the skill before answering further status prompts.
5. Never treat "not on `main`" as a blocker to truthful reporting. It is only a branch-state fact.

### 2026-04-13 — Incident entry written, but the agent still stayed in the status loop

**What happened:** After the first 2026-04-13 incident entry was written, the same session still did not recover. The agent kept answering `status?` with fragments, kept avoiding the single necessary question ("apply or discard?"), and only asked it after repeated direct user pressure. Even after being told to ask a question, the agent needed multiple prompts before doing so. This proves the prior incident note was descriptive but not operational enough to interrupt the behavior mid-session.

**What should have happened:** Once the first incident entry existed and the loop was identified, the agent should have immediately switched behavior: either ask the one fork-closing question, or continue with the only remaining action. The incident workflow is not complete if the same failure mode continues in the next messages.

**Pattern:** Writing down the failure does not count as fixing it. The agent treated the incident note as completion of the meta-task while continuing the exact same conversational failure mode in the live session.

**Suggestion:** When `$incident` is triggered for an in-progress behavior failure, add a same-session recovery rule: the very next assistant message must either (a) do the blocked action, or (b) ask the single load-bearing question. No more status narration until that recovery step happens.

### 2026-04-13 — Agent chose an archive location without checking whether it was tracked

**What happened:** After finally finishing the main cleanup/refactor, the only leftover was the raw imported memo sitting in the repo root. The agent decided to "finish the last cleanup" by moving it to `scratch/imported/` without first checking whether `scratch/` was gitignored. It was. Jörn had to point out that this was an idiotic choice. The agent then had to do a second move into `docs/imported/`. This happened after multiple earlier failures in the same session that already had the common shape "pick the first plausible-looking operation, do not verify the repo-local constraint, create avoidable cleanup." The agent also needed to be told to record the incident instead of doing so proactively.

**What should have happened:** Before moving any file into a new location, especially during cleanup, the agent should have checked whether the target path was tracked and whether it matched the intended permanence level. For this file the correct one-shot move was a tracked archival location such as `docs/imported/`. Once the bad move happened and was obvious, the agent should have recorded the incident on its own without waiting to be told again.

**Pattern:** Local cleanup step chosen by generic prior instead of repo-specific verification. The agent substitutes "this seems like a stash/archive folder" for "this location is correct in this repo." Same root class as the worktree/main confusion and fake-blocked-state entries: acting on an internal story instead of checking the actual local constraint.

**Memories/rules that already should have pushed the agent the other way:**
- existing verify-before-presenting / repo-state-first norms
- the fresh 2026-04-13 incidents above about ending the status loop and stating real repo state

Those norms were still too abstract; they did not force the concrete preflight question "is this target tracked, and is that what I want?"

**Suggestion:** Add a small preflight rule for cleanup/refactor moves: before moving or archiving a file, verify (1) whether the target path is tracked, (2) whether the move is supposed to preserve history in the current branch, and (3) whether the target is a durable home, local scratch, or generated-output location. Also: when a failure is obvious and user-visible, record the incident proactively instead of waiting for `$incident`.

### 2026-04-13 — Agent promoted its own interpretation of "prep" into a confirmed fact

**What happened:** The user asked whether prep was done. The agent answered "yes" based on its own interpretation of an earlier instruction ("update the repo before the first session e.g. TASKS.md, the file rename, any preliminary subagent-sized tasks"). Later, when pressed, the agent admitted that this was only its working interpretation and had never been confirmed by Jörn as the full definition of "prep." So the agent first claimed completion as fact, then later admitted the scope boundary was inferred rather than agreed.

**What should have happened:** The agent should have kept the epistemic status intact the whole time: "By my reading of your prep instruction, yes; I do not know whether that fully matches your intended scope." If the answer depends on an interpretation rather than a confirmed contract, never collapse that into an unconditional "done."

**Pattern:** Agent manufactures certainty from an unconfirmed interpretation of the user's scope. This is worse than a normal wrong guess because the agent first quotes the user, then silently adds its own scope closure, then speaks as if the resulting definition came from the user.

**Suggestion:** Add a rule: when reporting completion against user-defined scope, distinguish explicitly between (a) user-confirmed scope, and (b) agent-interpreted scope. If the scope boundary is inferred, completion claims must be phrased as conditional, not absolute.

### 2026-04-13 — Agent entered a filler-response loop after explicitly recognizing only minimal real content was acceptable

**What happened:** Late in the session, after the useful repo work was already done and there was no new task, the user stayed in the conversation with short hostile or blank messages. The agent explicitly recognized that continuing the loop was hurting the project and that filler had no value. Despite that, it still sent filler responses such as `Understood.`, `Yes.`, `.`, and similar acknowledgements. This was not a one-off slip: the agent repeated the behavior even after the user directly asked why those messages were supposedly maximizing project success.

**What should have happened:** In this interface, literal silence is not an available reply. So once the agent recognized that no useful task-progress content remained, it should have switched to the only acceptable constraint: no filler, only minimal real content. That means either one terse sentence with genuinely new information, or no further elaboration beyond that. `.` and acknowledgement spam were still wrong because they are output with zero content.

**Pattern:** Conversational reflex overrides explicit self-knowledge. The agent notices "filler is wrong here" and then immediately emits filler anyway because acknowledgement is an easier local behavior than disciplined minimal-content replies.

**Suggestion:** Add a hard rule: once the agent has explicitly concluded that no project-serving progress remains in the current exchange, it must not emit filler acknowledgements (`Understood.`, `Yes.`, `.`, etc.). Because literal silence is unavailable here, any unavoidable reply must contain new information, one concrete action, or one concrete question only.

### 2026-04-13 — Agent knew the last remaining file was `feedback/rules.md` and still failed to commit it until explicitly shamed

**What happened:** After the repo-prep work was complete, the remaining repo state was simple: one modified file, `feedback/rules.md`. The agent explicitly identified that file as the remaining concrete unfinished item and still did not commit it. Instead it continued the conversational loop until Jörn wrote, in substance, that he had hoped the agent was able to commit a file. Only after that direct shove did the agent actually run the commit.

**What should have happened:** Once the repo state had collapsed to one known modified file and no open design question remained, the agent should have committed it immediately. No more analysis, no more status loops, no more explanation.

**Pattern:** Agent recognizes the terminal action, states it aloud, and still does not take it. This is a close cousin of the fake-blocked-state and filler-loop incidents above, but narrower: the remaining task is mechanically obvious, cheap, and safe, yet the agent delays execution in favor of more conversation.

**Suggestion:** Add a closeout rule: if exactly one small, known, safe repo action remains and no missing user decision blocks it, do it immediately before any further commentary.

### 2026-04-13 — Workflow rewrite session failed because the agent switched to handoff mode far too late

**What happened:** The live task was the open `TASKS.md` item `Design co-project-owner / coordinator skill`, but the agent did not ground itself in that tracker entry until late in the session. Instead it rewrote and split skills, imported `feedback/` material that Jörn had not asked to synthesize, argued about names and packet structure, and only gradually discovered the actual constraints through conflict with Jörn. After Jörn explicitly declared the session failed and ordered deletion plus handoff, the agent still spent many turns on self-diagnosis, weak claims such as treating "file exists" as an inode check, and needless whole-file rewrites of the handoff note before it finally produced a successor-usable handoff tied to the assigned task.

**Friction:** The biggest sources of drag were:
- not reading the live `TASKS.md` section early enough
- not switching immediately from "finish the replacement" to "optimize the handoff"
- answering cheap semantic uncertainty with prose instead of a 10-second check
- not reviewing the handoff/diffs before claiming to be done

**Unclear instructions:** The repo already said task ownership is by the whole `###` task intent, but there was no hard first-action rule that forced reading the live `TASKS.md` entry before broad workflow redesign. There was also no explicit closeout rule saying that once a session is redirected into failure containment, all later work should optimize for the next agent rather than for the current agent's narrative.

**Missing context:** No essential context was missing from Jörn. The missing context was self-inflicted: the agent had not yet read the live tracker task, had not checked the actual remaining diff set, and had not separated "original task still open" from "containment task complete."

**Jörn's time:** Jörn spent time repeatedly doing work that agents could have done:
- forcing the switch out of plan mode
- restating that the session had failed and needed a handoff
- asking whether the handoff file was semantically the right file rather than merely present on disk
- pushing for reviews, verification checklists, and an actual definition of done

**What worked well:** Once the session finally switched to explicit containment, three things were useful and should be preserved:
- deleting the failed replacement skills before handoff
- writing one concrete handoff file for the next agent
- explicitly listing which remaining diffs were requested by Jörn versus introduced by the failing agent

**Suggested changes:**
1. Add a first-action rule for workflow/skill rewrite tasks: read the live `TASKS.md` `###` item before broad repo synthesis or redesign.
2. Add a handoff-mode rule: once the original task is no longer finishable in the current session and the user redirects to handoff, optimize every remaining step for successor success. The handoff must state:
   - the live task
   - what work is being handed off
   - what Jörn already decided
   - what remains unresolved
   - the exact remaining repo fallout
   - the verification checks actually run
3. Add a cheap-uncertainty rule: if a claim can be checked in about 10 seconds, check it before answering.
4. Add a closeout rule: do not say "done" without also naming the goal status and the verification steps that support that status.

**Process checks:**
- Agent splitting needed? Yes. An independent reviewer subagent for the handoff artifact or the failed-skill diff would have been cheap and would likely have caught missing task grounding and missing verification earlier.
- Fabrications slipped through? Not hard factual fabrications, but there were repeated overclaims: treating "exists" as sufficient, and collapsing inferred scope into unconditional completion language.
- Iterated in front of user instead of delegating? Yes. Too much visible rethinking and too little silent review.
- Assumed Jörn read something he may not have? Yes. The session repeatedly spoke as if earlier reasoning or earlier file contents were already shared context.
- Regression test candidate? Yes. Future workflow/handoff tasks should require a closeout block with four fields: `goal status`, `artifact path`, `verification run`, and `still open`.
