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

### 2026-04-04 — numpy vs Rust SVD rank threshold mismatch

**What happened:** `analyze.py` used `np.linalg.matrix_rank(G, tol=1e-8)` (absolute threshold) while `run.rs` used `1e-8 × σ_max` (relative threshold). σ[25]=1.57e-8 was above the absolute 1e-8 cutoff but below the relative 9.4e-8 cutoff. This produced rank 26 in Python vs rank 25 in Rust, causing a false warning "C ⊋ ker(G)" in the rank condition check.

**Root cause:** When two languages compute the same quantity, threshold conventions must be explicitly synchronized. The Rust code documented its threshold convention clearly; the Python code didn't consider that numpy's default differs.

**Pattern:** Cross-language numerical convention mismatch. The math-tex convention says "math.tex is single source of truth" — a similar principle could apply to numerical thresholds: define once, document, use consistently.
