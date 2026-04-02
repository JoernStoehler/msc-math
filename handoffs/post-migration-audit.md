# Post-Migration Audit Findings

Audit date: 2026-04-02. Covers: CLAUDE.md, .claude/ rules/agents/skills/hooks/settings, TASKS.md, IDEAS.md, handoffs/, feedback/, math.tex, Cargo.toml, experiments structure.

## Scoring

- **EVI** (Expected Value of Information): Jörn-minutes saved across the project lifetime by investigating/fixing this item. Accounts for frequency of encounter, confusion caused, downstream errors.
- **ECD** (Expected Cost of Discussion): Jörn-minutes to resolve this item. Low = agent can fix with a one-line approval; high = needs Jörn's judgment or design input.

---

## Findings

### 1. Workspace manifest location is wrong in CLAUDE.md
**EVI: 30 | ECD: 1**

CLAUDE.md line 17–18 says:
```
crates/                    all Rust code (library + experiments)
  Cargo.toml               workspace manifest (members: library, exp-*)
```

`crates/Cargo.toml` does not exist. The workspace manifest is at the **repo root** `/workspaces/msc-math/Cargo.toml`. Every agent that reads this layout will look in the wrong place. Also, the parenthetical `(members: library, exp-*)` omits `database`, `crosspolytope`, `visualization`.

**Fix:** Move `Cargo.toml` line out of the `crates/` indent, update description. Or add `Cargo.toml` at repo root level in the tree.

---

### 2. `.claude/memory/` in project layout — directory doesn't exist
**EVI: 8 | ECD: 1**

CLAUDE.md line 67: `.claude/memory/` — actual directory is `.claude/agent-memory/`. The auto-memory system (managed by Claude Code itself) lives at `/home/vscode/.claude/projects/.../memory/`, not here. Agents looking for project-level memory will search the wrong path.

**Fix:** Rename to `agent-memory/` in the layout, or note that this is Claude Code's built-in path and not a project directory.

---

### 3. Three workspace crates undocumented in CLAUDE.md
**EVI: 12 | ECD: 2**

`crates/crosspolytope/`, `crates/database/`, `crates/visualization/` are workspace members but appear nowhere in CLAUDE.md's project layout. The experiments rule (`experiments.md` line 32) does mention them as "Standalone: `crosspolytope`, `visualization`" but `database` is absent even there.

- `crosspolytope`: single-task computation (has logbook, run.rs, math.tex, data)
- `visualization`: interactive HTML viewer (has logbook, run.rs, math.tex, viz/)
- `database`: stub lib for future sigma cache (just lib.rs)

**Fix:** Add a brief line in the project layout under `crates/`, or note that experiments.md is the authoritative list.

---

### 4. combinatorial-structure/math.tex not included in root math.tex
**EVI: 5 | ECD: 2**

`/workspaces/msc-math/crates/exp-sys-optimization/combinatorial-structure/math.tex` exists with real content (defines combinatorial type, boundary events, proves continuity) but has no `\input` in root `math.tex`. Cross-references from this file won't resolve. All other experiments with math.tex files are included.

**Fix:** Add section + `\input` to root math.tex, or mark as intentionally excluded with a comment.

---

### 5. IDEAS.md blockers section outdated
**EVI: 4 | ECD: 1**

Lines 117–119: "Library has no derivative API (∂c_EHZ/∂a_i) — see TASKS.md `dual-vertex-parameterization`". But `capacity_derivatives_a()` and `volume_derivatives_a()` now exist (per TASKS.md line 290, migration completed 2026-03-30). Agents reading IDEAS.md will think derivatives are blocked.

**Fix:** Update the blockers section or delete it if no longer needed.

---

### 6. 8+ stale migration handoffs
**EVI: 6 | ECD: 1**

These handoff files are for completed migration work and will confuse agents scanning for active tasks:
- `migration-done-checklist.md` (Mar 16)
- `migration-target.md` (Mar 16)
- `migration-process.md` (Mar 16)
- `kkt-module-spec.md` (Mar 16)
- `kkt-rework-spec.md` (Mar 16)
- `tube-notes.md` (Feb 23)
- `tube-algorithm-plan.md` (Feb 25)
- `tube-spec.md` (Feb 23)
- `session-crate-math-tex.md`, `session-experiment-logbook-migration.md`, `session-test-data-pipeline.md` (Mar 17)

**Fix:** Delete or move to an `archive/` subdirectory. Agent can do this with one approval.

---

### 7. Old `experiments/` paths in active handoffs
**EVI: 8 | ECD: 1**

`verify-numerics-perturbation-chain.md` and `verify-numerics-algorithm.md` (both Apr 1, actively used) reference pre-migration paths like `experiments/verify-numerics/math.tex`, `experiments/correctness/`, etc. These paths don't exist post-migration — the correct paths use `crates/exp-*/`.

**Fix:** Find-and-replace `experiments/` → `crates/exp-*/` in these files.

---

### 8. `.claude/prompts/` and `.claude/output-styles/` undocumented
**EVI: 2 | ECD: 1**

Both directories exist but are absent from CLAUDE.md's `.claude/` layout. `prompts/` has one file (`rational-pipeline-mismatch-prompt.md`). `output-styles/` is empty. Low impact since they're rarely referenced.

**Fix:** Add to layout if they're intended to persist, or delete if they're migration artifacts.

---

### 9. Experiments rule has hardcoded experiment list
**EVI: 4 | ECD: 3**

`.claude/rules/experiments.md` lines 29–36 hardcode every experiment group and its subdirectories. This list will silently go stale when experiments are added/removed. No guidance on maintaining it.

**Fix options:** (a) Add a comment "update this list when adding experiments", (b) remove the list and have agents discover dynamically via `ls`, (c) keep as-is and accept the staleness risk. Needs a design call.

---

### 10. test-workflow skill references nonexistent directory
**EVI: 3 | ECD: 1**

`.claude/skills/test-workflow/SKILL.md` lines 69–70 instruct agents to save test tasks to `.claude/skills/test-workflow/references/test-tasks/` — this directory doesn't exist.

**Fix:** Create the directory, or update the skill to use a different location.

---

### 11. Feedback files reference memory entries that contradict post-mortem guidance
**EVI: 3 | ECD: 3**

`feedback/agents.md` and `feedback/rules.md` reference "memory entries" as if they're maintained. But `post-mortem/SKILL.md` line 64 says "Do NOT write to agent memory — memory entries go stale and are not maintained." Contradictory guidance about whether agent memory should be used.

**Fix:** Decide: is `.claude/agent-memory/` a maintained system or not? Align all references.

---

### 12. Plan file mechanism not explained in CLAUDE.md
**EVI: 5 | ECD: 2**

CLAUDE.md line 169: "Update the plan file as you work — it survives compaction." But CLAUDE.md never explains what the plan file is, where it lives, or how to create/read it. Agents encountering this for the first time must infer from context.

**Fix:** Add one sentence explaining that Claude Code creates plan files at `/home/vscode/.claude/plans/` when entering plan mode, and that they persist across context compaction.

---

### 13. "main .tex includes" phrasing ambiguous
**EVI: 2 | ECD: 0.5**

CLAUDE.md line 73: "Module-level files (mod.rs, main .tex includes)" — reads as "main.tex" (the thesis file) rather than "the main .tex include for each module."

**Fix:** Rephrase to "Module-level files (mod.rs, module-level .tex)" or similar.

---

### 14. Two experiment run.rs files use wrong doc comment style
**EVI: 1 | ECD: 0.5**

`exp-numerical-analysis/kkt-inertia/run.rs` and `exp-numerical-analysis/q-error/run.rs` use `///` instead of `//!` for the file header. CLAUDE.md says "Every source file has a header explaining purpose and context (Rust: `//!` doc comments)". 20/22 other experiment files are correct.

**Fix:** Change `///` to `//!` in both files.

---

### 15. 8 experiment binaries don't compile
**EVI: 10 | ECD: 5**

Per `feedback/rules.md` (2026-04-02): 8 experiment binaries reference post-migration APIs that changed (`KktOutcome`, `Polytope4D` methods). This blocks `cargo build --workspace` and any experiment-level `cargo test`. The broken binaries include generate_seeds, gradient_search, sys_search, combinatorial_boundaries, gradient_descent, sys_optimization, visualization, omega_obstacle.

**Fix:** Either fix each binary's API calls, or gate them behind feature flags so workspace-wide builds succeed. Likely tracked in TASKS.md already — check `dual-vertex-parameterization` task.

---

### 16. `profiling` experiment has minimal structure
**EVI: 1 | ECD: 1**

`crates/exp-algorithm-comparison/profiling/` has only `analyze.py` and `logbook.md` — no `run.rs`, no `math.tex`. The experiments rule says "not all experiments have all files" so this is valid, but it's the only experiment without any `.rs` file. Might be a Python-only analysis that could be documented differently.

**Fix:** If intentional, no action needed. If it should have a run.rs, note as TODO.

---

### 17. Root `Cargo.lock` not in project layout
**EVI: 1 | ECD: 0.5**

`Cargo.lock` exists at the repo root (normal for a workspace) but isn't listed in the CLAUDE.md project layout. Low impact — agents know what Cargo.lock is.

**Fix:** Optional — add to layout tree if completeness is valued.

---

### 18. TASKS.md figures-in-math.tex cleanup never done
**EVI: 3 | ECD: 3**

TASKS.md line 55 lists "Known offenders" where figures/tables live in math.tex instead of logbook.md (ablation, gradient-descent, lagrangian-products, etc.). Marked as "Low priority" but never addressed. This muddies the separation between math.tex (proofs) and logbook.md (results).

**Fix:** Either do the cleanup or explicitly accept that some math.tex files contain figures/tables and update conventions.

---

### 19. Agent memory file has brittle line-number reference
**EVI: 2 | ECD: 0.5**

`.claude/agent-memory/plan/project_verify_numerics_status.md` references `saddle_point_solver.rs:549`. Line numbers break on any edit. The CLAUDE.md style guide says to use function names or labels, not line numbers, in persistent text.

**Fix:** Replace with function name reference.

---

### 20. `Cargo.lock` and build artifacts in `.gitignore`?
**EVI: 2 | ECD: 1**

`math.aux`, `math.log`, `math.out`, `math.toc` exist at the repo root — these are LaTeX build artifacts. Check whether `.gitignore` covers them or if they're intentionally tracked.

**Fix:** Verify .gitignore. If they shouldn't be tracked, add to .gitignore and remove.

---

## Summary by EVI threshold

| EVI ≥ 10 | Items 1 (30), 3 (12), 15 (10) |
|----------|------|
| EVI 5–9 | Items 2 (8), 4 (5), 5 (4→round up), 6 (6), 7 (8), 12 (5) |
| EVI 2–4 | Items 8, 9, 10, 11, 13, 14, 18, 19, 20 |
| EVI < 2 | Items 16, 17 |

Agent-actionable (ECD ≤ 1, no design judgment needed): items 1, 2, 5, 6, 7, 8, 10, 13, 14, 17, 19
Need Jörn's judgment: items 3, 4, 9, 11, 12, 15, 16, 18, 20

## Resolution Log

- **1** ✅ Fixed: moved Cargo.toml to repo root in layout, added Cargo.lock, updated description
- **2** ✅ Fixed: `.claude/memory/` → `.claude/agent-memory/` with accurate description
- **3** ✅ Fixed: added crosspolytope/, database/, visualization/ to project layout
- **4** ✅ Fixed: added `\section{Combinatorial Structure}` + `\input` to root math.tex
- **5** ✅ Fixed: updated IDEAS.md blockers to reflect derivative API exists
- **6** ✅ Fixed: deleted 13 stale handoffs (migration-*, kkt-*, tube-*, session-*, planning-agent-brief). Kept `migration-thesis-findings.md` (10 unresolved items)
- **7** ✅ Fixed: updated all old paths in verify-numerics-perturbation-chain.md, verify-numerics-algorithm.md, tube-algorithm.md, handoff-geom-math-review.md. Deleted stale cross-reference-audit.md and experiment-deduplication.md
- **8** ✅ Fixed: added `prompts/` to CLAUDE.md layout (dropped empty `output-styles/`)
- **9** ✅ Fixed: replaced hardcoded list with pointer to `crates/` + guidance to scan sibling experiments before adding/editing
- **10** ✅ Fixed: created `.claude/skills/test-workflow/references/test-tasks/`. Skill audit in progress for similar issues in other skills.
- **11** ✅ Fixed: removed "Do NOT write to agent memory" from post-mortem skill; memory systems are used and maintained
- **12** ✅ Closed: system prompt already explains plan files; agents have never shown confusion about this
- **13** ✅ Fixed: "main .tex includes" → "math.tex"
- **14** ✅ Fixed: `///` → `//!` in both kkt-inertia/run.rs and q-error/run.rs
- **15** ✅ Fixed: 8 binaries updated to new library API (`KktOutcome` enum, `dual_vertices_f64()`, `capacity_derivatives_a`). Full workspace compiles clean
- **16** ✅ Closed: intentional Python-only profiling utility, no action needed
- **17** ✅ Fixed: added Cargo.lock to layout tree
- **18** ✅ Convention updated in math-tex.md rule; cleanup task updated in TASKS.md with full file list. 15 experiments need figures/tables moved to logbook.md
- **19** ✅ Fixed: replaced `saddle_point_solver.rs:549` with `SaddlePointSolver::verify_solution()` and `[lem:q-error-bound]`
- **20** ✅ Verified: all root math.tex artifacts are in .gitignore and untracked. No action needed.
