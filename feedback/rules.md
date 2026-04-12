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
