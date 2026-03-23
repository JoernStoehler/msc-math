# TASKS

Master task list for thesis completion. **Deadline: March 31, 2026.**

**Current state (2026-03-22):** No thesis chapter is publishable yet. Experiments have data but writeups are noisy and the thesis doesn't tell a coherent story backed by results. Migration is complete and merged. Meta-layer refactored (skills over rules, conventions as review spec). Progress rate is on track for deadline.

**Priority order:** Migration merge → thesis coherence + experiment quality → code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.

**Maintenance rule:** When an agent completes a task, discovers a new task, or learns something that affects a task's implementation, update this file immediately. Don't defer. Context that isn't written down here or in the referenced files is context that will be lost.

**Escalation markers:**
- 🔴 Jörn decision required — agent cannot proceed alone
- 🟡 Agent can attempt, but should present result for Jörn review before building on it
- 🟢 Agent can do autonomously

---

## Maturity Map

What's settled vs placeholder across the project's major components.

| Component | Status | Notes |
|-----------|--------|-------|
| Rust library (crates/) | **Draft** | 73 files, 317 tests pass (26 ignored), Clippy clean. Tests migrated to inline modules. Cleanup (§2) done, solver refactors (§6) and tube algorithm (§7) still ahead |
| Migration scaffold | **Settled** | Merged to main. Complete. |
| Test data pipeline | **Settled** | 1.7s release / 17s debug. Scalar fixture loading, release-mode default. See §1. |
| Experiment data | **Settled** | All 18 experiments have data and produce results |
| Experiment writeups | **Draft** | All 18 experiments have logbook.md (migration complete). Content quality varies — noisy, no coherent story |
| Experiment shared code | **Not started** | 930–2347 LOC duplicated across 4 experiments; extraction planned (§6d) but not started |
| Thesis — tube algorithm chapter | **Draft** | 8 TODOs (5 need Jörn's math verification, 3 GAP markers from agent content) |
| Thesis — numerical appendix | **Draft** | 5 TODOs including unverified claims |
| Thesis — other chapters | **Placeholder** | No TODOs but Jörn doubts publishability; no chapter tells a coherent story yet |
| Thesis — story arc / structure | **Not started** | Jörn hasn't defined the central argument; highest-value decision pending |
| Thesis — final assembly | **Not started** | Abstract, intro, conclusion, bibliography, figures, proofreading — all deferred until content stabilizes |
| Thesis↔code alignment | **Placeholder** | 15 mismatches + 18 missing cross-ref labels identified; Jörn decides which side to fix (§3) |
| Convention skills / meta-layer | **Draft** | Refactored 2026-03-22: skills over rules, conventions as review spec, meta-* skills reorganized. Meta-foundations and meta-create-conventions Jörn-reviewed. Convention skill content not yet audited for cargo cult / unjustified items (found one: iterator chain preference was cargo cult, removed). |
| SLURM skill | **Not started** | Planned (§7b), awaiting example template from Jörn |

---

## 0. Migration merge ~~(blocking everything)~~ ✅ COMPLETE

**Merged** to main (commit 6680d0e, 2026-03-19). No longer blocking.

### What's done
- 73 .rs files rewritten (72 target + lib.rs), 0 empty stubs
- All waves (1-4) complete: geom, kkt, algorithms, tests, lib.rs re-exports
- Experiments updated (16 .rs files, 16 READMEs standardized)
- Meta-layer: skill contradictions fixed, archaeology skill dropped, lookup.sh added
- Convention skills updated for new module layout
- Clippy clean (--lib and --tests)
- Tests: 316 pass, 0 fail, 26 ignored

### What Jörn needs to review
1. **Convention skills** — full review, not just diffs
2. **15 thesis↔code mismatches** — see `handoffs/migration-thesis-findings.md`. Key decisions:
   - Label mismatch: `[thm:billiard-characterization]` vs thesis `\label{thm:billiard-completeness}`
   - Notation: KKT multipliers (λ,ν) vs (μ,ξ), sign flip, factor-of-2
   - Algorithm boxes missing steps the code implements (accumulator, adjacency pruning)
   - Tube: rotation increment is heuristic, not CH2021 Lem.2.21 implementation
3. ~~**18 cross-ref labels**~~ — RESOLVED (2026-03-22). All code labels reference colocated math.tex files per convention. ~~3 redundant `(thesis)` markers~~ removed in §2.
4. **qp_assembly dual-vertex formulation** — unverified mathematical equivalence with normals/heights formulation (has explicit TODO)
5. ~~**Experiment adjacency functions**~~ — DONE (§2 item 5). 6 experiments converted to library API.
6. **KktResult→Solution bridge** — `margin = min(beta)` + `classify_margin()`. Is this the right verdict mapping?

### Remaining gate checks (need CPU)
```bash
cd .claude/worktrees/migration-scaffold/crates
cargo nextest run --lib            # 316 pass, ~7min (slow due to fixture regen)
cargo clippy --lib -- -D warnings  # clean
cargo clippy --tests -- -D warnings  # clean
```

---

## 1. Test data pipeline restructuring — DONE

🟢 Agent can do autonomously. Load `data-pipeline` skill for conventions.

**Result:** Default suite runs in **1.7s wall** (`cargo test --release --lib`), **17s wall** (debug). Down from ~7 min originally.

**What was done:**
1. Fixture file exists at `tests/fixtures/capacity_dataset.json` ✅ (pre-existing)
2. `known_polytopes` uses `LazyLock` caching ✅ (pre-existing)
3. BigRational + nalgebra opt-level=3 in dev/test builds ✅ (pre-existing, 9d191b5)
4. SVD pre-filter skips ~70-80% of rational vertex enumeration subsets ✅ (pre-existing, edbcb6a)
5. **Scalar-only fixture loading** — `load_dataset_entries()` returns `Vec<DatasetEntry>` with JSON parse only, skipping the expensive `Polytope4D::new()` call. 9 of 10 fixture tests switched to this path.
6. **`catalog_determinism` and `fixture_staleness_check` marked `#[ignore]`** — only needed during fixture regeneration. `CATALOG_VERSION` tag provides O(1) staleness guard in the default suite.
7. **`capacity_monotonicity` marked `#[ignore]`** — only fixture test needing `Polytope4D` (for vertex containment checks). Loads full dataset locally when run with `--ignored`.
8. **`debug_assert!` → `assert!`** in KKT solvers — enables release-mode testing without losing invariant checks (Q-constancy, FM back-substitution). All checks are O(m²) or O(1), negligible vs the O(m³) operations they guard.
9. **Default test command changed to `cargo test --release --lib`** — the crate's own code is 10-40× faster in release. All documentation updated.

**Profiling (2026-03-22, post-optimization):**

| Mode | Wall time | Test count |
|------|-----------|------------|
| `cargo test --release --lib` | 1.7s | 314 pass, 29 skip |
| `cargo test --lib` (debug) | 17s | 314 pass, 29 skip |
| Incremental rebuild + test (release) | 13s | — |
| Incremental rebuild + test (debug) | 16s | — |
| Cold build + test (either) | ~44s | — |

**Depends on:** Migration merge (§0) ✅.

---

## 1b. Migrate remaining experiments to logbook format — COMPLETE

**Status (2026-03-22):** All 19 experiments have logbook.md. No README.md files remain. `test-profiling` was the last migration (`profile.py` → `analyze.py`, README.md → logbook.md).

---

## 2. Migration cleanup (small fixes) — DONE (2026-03-22)

1. ~~**"orbit" → "candidate" terminology**~~ — Already done during migration. No problematic uses remain.
2. ~~**Duplicate EPS constants**~~ — DONE. `EPS_EIGEN_FLOOR` consolidated to `kkt/mod.rs`, imported by both solvers. `EPS_EIGEN_THRESHOLD` / `EIGEN_CONDITION_TAU` kept separate (different matrix structures, documented why).
3. ~~**`Vec<Vec<bool>>` adjacency**~~ — DONE (2026-03-22, commit f073e13).
4. ~~**`build_augmented_system` allocation**~~ — Dropped. Speculative perf claim without profiling. O(m) allocation is negligible vs O(m³) eigendecomposition.
5. ~~**Experiment f64 adjacency copies**~~ — DONE. 6 experiments converted to use `polytope.vertex_adjacency()`, `build_transition_matrix()`, `is_feasible_cycle()`. ~11 local functions deleted, ~185 lines removed. ablation's specialized A3 variants kept.
6. ~~**Redundant `(thesis)` markers**~~ — DONE. 5 `(thesis)` suffixes removed from `saddle_point_solver.rs` label references (now all labels reference colocated math.tex per convention).

---

## 2b. Resolve convention contradiction: math in code vs thesis — RESOLVED

Convention decided (2026-03-17): One `math.tex` per module directory contains lemma statements + proofs (compiles to PDF for Jörn review). `.rs` doc comments contain code-math correspondence only (symbol mapping, label references, no proofs). Agents must read the module's `math.tex` before editing `.rs` files. `rust-conventions/SKILL.md` updated.

---

## 3. Thesis↔code alignment

🔴 Jörn decides which side to fix for each item. Agent applies the fixes.

**Blocked by:** ~~§2b (convention contradiction resolution)~~ resolved.

See `handoffs/migration-thesis-findings.md` for the full list. Highest priority items:

1. **Tube rotation increment** — code is a heuristic claiming to implement `[def:rotation-increment]`. Fix doc comment, or implement CH2021 Lem.2.21.
2. **KKT notation** — unify (λ,ν) vs (μ,ξ) across thesis sections and code.
3. **Accumulator pattern** — describe two-tier certified/uncertain tracking in thesis algorithm boxes (currently only in appendix A.3-A.4).
4. ~~**18 missing labels**~~ — resolved, see §0 item 3.

---

## 4. Thesis TODOs

🔴 Jörn verifies the math. Agent writes drafts, Jörn reviews.

### tube-algorithm.tex (8 TODOs)
Mostly agent-written from Jörn's notes. Needs Jörn's mathematical verification on:
- JÖRN Q1: Quaternionic formula for ψ in CH2021 Lem.2.21
- JÖRN Q2: TF_{ij} ⊂ n_i^⊥ ∩ n_j^⊥ equivalence
- JÖRN Q3: Rotation number — abstract vs computable definition
- JÖRN Q4: Closing requires two extension steps (code has resolved this; thesis doesn't)
- JÖRN Q5: Correctness proof sketch
- 3 GAP markers where agent added unverified content

### appendix-numerical.tex (5 TODOs)
- Continuity of c_EHZ on polytopes
- Simplicity assumption
- Billiard algorithm — does pruning by adjacency apply?
- Three-valued verdict framework description
- Numerical statement agent verified but Jörn hasn't checked

---

## 5. Experiment quality + thesis coherence

🔴 Jörn scopes what the thesis story is. Agent executes.

**Problem:** The thesis is currently a dump of results, not a coherent narrative. Experiments have data but writeups are noisy. There are low-hanging-fruit experiments that would provide real value but haven't been done, while some existing experiments add noise without advancing the argument.

**Experiment ideas on file:**
- `experiments/IDEAS.md` — research directions (symplectic classification, HKO neighborhood local maximality)
- `experiments/ablation/ideas-future.md` — pruning extensions (A1 vs F scaling, non-simple polytopes, UNKNOWN predicates, face lattice approach, exact skeleton via perturbation)
- Various experiment README "Known limitations" sections mention follow-up ideas

**What Jörn needs to decide:**
- What is the thesis's central argument / story arc?
- Which existing experiments support that story vs are background validation vs are noise?
- Which experiments from IDEAS.md should be run? Which new ones not yet written down?
- Which low-hanging-fruit experiments would provide the most value?
- What depth does each experiment need in the thesis? (full section / brief mention / appendix / omit)

**Current experiment status:** All 16 have Complete status and standardized READMEs. But "Complete" means "has data," not "writeup is publishable."

**Known gaps:**
- `sys-optimization` needs a redesign, not just modernization — didn't use proper gradients, didn't look at cuts
- `crosspolytope` Phase 2 TODO: update known_polytopes.rs (tracked in its logbook after migration)
- `hko-neighborhood` logbook created; open questions 5-7 from Jörn about neighborhood landscape, subdifferential after cuts, saddle vs local max
- Experiment .tex writeups need review against the coherent-story standard once Jörn defines the story

**Experiment ideas session (2026-03-22, in progress):**

Key existing results:

| Experiment | Finding |
|---|---|
| **crosspolytope** | First computed c_EHZ for 4D crosspolytope: c=4.0 (same as hypercube, its dual), sys=0.75. Exhaustive through m=12/16 with symmetry reduction. |
| **hko-neighborhood** | Evidence HKO2024 is local max: gradient ascent converges in 1 step (Δsys~5e-9), all facet-splitting cuts decrease sys. 44 degenerate orbits at same action. Subdifferential appears to contain origin. Normal gradient nonzero but constrained by feasibility boundary. |
| **lagrangian-products** | HKO is only sys>1 among regular polygon pairs (3≤n,m≤6, 6° resolution). Violation region for (5,5) spans ~13.5-22.5° rotation (~25% of period). |
| **pentagon-perturb** | All 100 random perturbations of HKO in LP(5,5) space retain sys>1 (min 1.002, max 1.033). HKO at 1.047 is highest. No dominant PCA direction. |
| **gradient-descent** | 995 polytopes, none reach sys>1. Lagrangian 5×5 reach higher sys (max 0.905) than general F=10 (max 0.870). Step-bound barrier at combinatorial type boundary prevents convergence; residual gradients O(1) at termination. GD currently sucks and fails hard. |
| **sys-optimization** | Gradient ascent from 140 starts, best sys=0.878. (h,n) steps outperform h-only (68%, mean +0.054 vs +0.034). Combinatorial type boundary is binding constraint. 15/140 outliers near orbit boundaries. |
| **omega-obstacle** | Hypothesis that small abs(ω₀) on ridges increases sys **falsified** by 4 independent tests on 953 polytopes. Orbit-specific ω has zero correlation with sys (ρ=-0.02, p=0.61). Orbits actually prefer LARGE abs(ω₀) transitions (median 0.54 orbit vs 0.36 non-orbit). KKT optimizer compensates by redistributing β — small ω doesn't make Q small because the optimizer selects orbits and weights that maximize Q regardless. HKO's Lagrangian ridges are a consequence of its construction, not a general mechanism. |
| **random-sweep** | 70 random polytopes (F=5-12), max sys=0.578 at F=11. Median increases with F (0.08→0.48) but within-F variance large. Random polytopes stay far from violation. |
| **random-product-sweep** | Random Lagrangian products max sys=0.794 (6×6). Higher-polygon pairs reach higher sys. HKO requires specific rotation angle, not reproduced by random orientations. |
| **ablation** | All 4 pruning variants agree on capacity (max diff <1e-8). A2 speedup exponential: ~8x at F=5, ~1078x at F=10. A3 adds nothing on simple polytopes but 98% reduction on non-simple. |
| **correctness** | All 6 mathematical axioms pass: algorithm agreement, literature values, conformality, symplectic invariance, perturbation stability, monotonicity. |
| **kkt-inertia** | Eigenvalue inertia formula holds for 6/7 polytopes. 5 hko_pentagon mismatches are threshold artifacts at machine epsilon. 1.13M nodes. |
| **orbit-recovery** | 112/112 polytopes pass all geometric checks. Closure/facet/action errors <1e-6 for F≤9. Base point generically unique (96.4%). |
| **q-error** | 1.13M nodes, worst error bound E=2.9e-11. Actual errors at machine epsilon. Algorithm empirically exact at f64 precision. |
| **unknown-predicates** | 29 UNKNOWNs, all Lagrangian products, all f64 noise. Random polytopes: zero. Phase 2 not needed. |
| **benchmark** | Post-optimization (§5c): construction negligible vs capacity for all F. Capacity ~4.26^F/facet. Practical limit F≤12. Pre-optimization: construction dominated for F≤10 (80-92%), crossover at F≈11. |

Observations (Jörn, 2026-03-22):
- Gradient descent showed how rare sys>1 solutions are, but GD currently sucks and fails hard (step-bound barrier at combinatorial type boundaries, poor convergence).
- Thesis writing postponed until end of week.
- Thesis will introduce both (n_i, h_i) and a_i = n_i/h_i but primarily work in a_i since n_i rarely appears without a 1/h_i factor.

New experiment ideas (Jörn, 2026-03-22):
1. **HKO2024 local maximality:** Show or strongly suggest that HKO2024 is a local maximum in the space of convex bodies. Would be a publishable result.
2. **Regular Lagrangian product analysis:** Dense (n, m, θ) sweep across wide range. Fit sys(n, m, θ) formula. Key question: does the fitted formula predict sys>1 only for P_5 ×_L R(θ) P_5?
3. **Dense random sweep on LICCA + gradient descent:** Large-scale search for new sys>1 polytopes not close to HKO2024. GD needs redesign first.

Blockers for experiment work:
- Library has no derivative API (∂c_EHZ/∂a_i) — needed for subdifferential analysis and gradient experiments. See §6a.
- Experiment gradient code (hko-neighborhood, gradient-descent, sys-optimization) is written in (n, h) space, not dual vertices. Needs rewriting after a_i-only KKT formulation is settled.
- a_i-only KKT formulation is an open math question for Jörn (rescale β so h disappears from constraints). See §6a.

(Session interrupted — more ideas TBD)

---

## 5b. End-to-end profiling of systolic ratio computation

🟢 Agent can do autonomously. Update `experiments/benchmark/`.

**Problem:** The existing benchmark experiment only times the capacity computation (permutation enumeration + KKT solve), not the full pipeline. `Polytope4D::new` (integer-scaled vertex enumeration, adjacency, omega signs) is not profiled but may dominate wall time for larger polytopes. There is no end-to-end breakdown showing where time goes from dual vertices to systolic ratio.

**Work items:**
1. Add end-to-end timing to the benchmark experiment, broken into phases:
   - `Polytope4D::new` (construction: vertex enum, incidence, vertex_adjacency, omega signs)
   - Permutation enumeration + adjacency pruning
   - KKT assembly + solve (LU/SVD)
   - Accumulator / orbit recovery
2. Produce a figure (stacked bar or similar) showing phase breakdown vs facet count
3. Update `benchmark/logbook.md` with findings

---

## 5c. Optimize Polytope4D construction — DONE (2026-03-23)

**Result:** 31x speedup at F=10 (84ms → 2.7ms). Construction now negligible vs capacity computation.

**Method:**
- Integer-scaled arithmetic (BigInt instead of BigRational) for vertex enumeration — avoids GCD normalization overhead
- f64 prefilters for bounded check and irredundancy — skips expensive exact arithmetic for most subsets
- Constructor cleanup (removed thin wrapper constructors, unified on `new()` and `from_f64()`)

**Before:** `Polytope4D::new` dominated end-to-end systolic ratio computation for F ≤ 10 (80-92% of total time). At F=10, construction was 84ms vs 17ms for capacity. Bottleneck was BigRational vertex enumeration via C(F,4) subset solves.

**Remaining:**
- Math verification of `prop:integer-cramer` by Jörn (integer-scaled Cramer's rule correctness proof)
- f64 threshold soundness evaluation (prefilter thresholds are empirically safe but not proven)

---

## 6. Code refactors (improve before thesis freeze)

### 6a. Dual-vertex parameterization (thesis notation change) — DIRECTION DECIDED

Direction (2026-03-22, Jörn): Thesis will introduce both (n_i, h_i) and a_i = n_i/h_i, but primarily work in a_i since n_i rarely appears without a 1/h_i factor. Thesis writing is deferred to end of week.

**Current state:** Code stores `dual_vertices` (a_i) as primary representation, computes (n_i, h_i) on demand. Math.tex files are mixed: `geom/math.tex` uses a_i in the H-representation definition, but `kkt/math.tex` and `algorithms/math.tex` use (n_i, h_i) throughout.

**Open question (Jörn):** The KKT system in `kkt/math.tex` is written in (n_i, h_i) with three separate objects (N, H, eta). Is there a clean a_i-only formulation? Likely requires rescaling β so h disappears from the constraints, which changes H. Jörn to work out the math — agents should not attempt the derivation.

**Library has no derivative API:** The library computes c_EHZ but provides no ∂c_EHZ/∂a_i (or ∂c_EHZ/∂anything). This should exist in the library — it's needed by multiple experiments and the subdifferential analysis is central to the thesis story around local maximality.

**Experiment code duplicates (n, h) gradient logic:** Three experiments independently implement ∂sys/∂h and ∂sys/∂n:
- `hko-neighborhood/run.rs` — ~2100 LOC, per-orbit gradients, gradient ascent, step bounds
- `gradient-descent/run.rs` — ~700 LOC, gradient ascent for Lagrangian products
- `sys-optimization/run.rs` — ~600 LOC, sensitivity + gradient iteration

All three work in (n, h) space, not dual vertices. Once the a_i-only KKT formulation is settled (open question above), these should be replaced by a library API computing ∂c_EHZ/∂a_i, with the experiments calling that instead of reimplementing.

**Ordering:** No longer blocking §6b.

### 6b. KKT projection-based solver

🟡 Agent implements, Jörn reviews math.

Implement second solver variant: solve constraints → project H → eigendecompose → LP for β>0. Keep augmented-system solver for comparison. See detailed spec in TASKS.md §"Identified refactors" below.

**Tip:** `kkt/projection_solver.rs` already exists from migration with a basic implementation. The refactor is to make it mathematically rigorous and add the ablation comparison framework.

### 6c. Unify β>0 feasibility as LP — IN PROGRESS (kkt-lp-refactor worktree)

🟢 Agent can do. Replace 1d/nd split with single LP formulation. See detailed spec below.

**Status (2026-03-22):** `kkt-lp-refactor` worktree has 17 commits (compiles clean, 20+ commits behind main). Not worth rebasing — start fresh worktree, using the old branch as a reference for decisions and analysis. Key content to salvage:
- Unified `find_positive_beta` function design (4 cases: already positive / no null dirs / 1D / LP via `microlp`)
- Near-null eigenvector Type A/B/C classification (scratch-near-null-analysis.tex) — Type A (β ≈ 0) causes LP unboundedness, Type C never observed across 4300 permutations
- Open question for Jörn: is filtering Type A directions mathematically justified, or just empirical?
- `thesis/appendix-numerical.tex` scaffold (A.1 input representation written, A.2–A.4 outlined)

Old worktree at `.claude/worktrees/kkt-lp-refactor/` — read-only reference, do not develop on it.

### 6d. Extract shared experiment code to library

🟢 Agent can do. See `handoffs/experiment-deduplication.md`.

4 experiments have 930-2347 LOC of duplicated instrumented KKT solver + derivative code. After migration, some of this is already in the library (adjacency, permutations). Remaining: instrumented solver, derivatives.

---

## 7. Tube algorithm completion

🟡 Agent implements, Jörn reviews math.

See `handoffs/tube-algorithm.md`. The migration created a fresh implementation in `algorithms/tube/mod.rs` (1175 lines) but:
- Rotation increment is a heuristic, not the real CH2021 formula
- Correctness proof is a sketch with GAPs
- Performance on F>10 polytopes is untested

---

## 7b. SLURM skill for LICCA cluster

🟢 Agent can create the skill. Jörn provides the example template.

Create `.claude/skills/slurm/SKILL.md` with `references/example.sh` that agents copy from when writing SLURM job scripts. Agents write the script + Rust binary; Jörn runs `ssh licca && sbatch job.sh` (agents have no SSH access).

The data-pipeline skill already describes when to offload. This skill covers *how*.

---

## 7c. Replace reproduce.sh with decentralized approach

🟢 Agent can do after experiment logbook migration.

`reproduce.sh` is stale (last updated during q-error merge). Instead of maintaining a single monolithic script, each experiment's logbook says what other experiments it depends on (if any) and how to run it. The full pipeline is: read all logbooks, resolve dependencies, run.

May still want a thin `reproduce.sh` that just reads logbooks and runs in dependency order. Or may not need it at all — manual reproduction following logbooks is fine for a thesis project.

---

## 8. Thesis chapters — all need work to be publishable

🔴 Jörn decides what "publishable" means for each chapter. Agent writes drafts.

No chapter is currently publishable. The thesis needs to become a coherent document that tells a story, not a collection of sections. This is the highest-value work after migration merge.

**Thesis .tex files with open TODOs:**
- `tube-algorithm.tex` — 8 TODOs (5 JÖRN questions, 3 GAP markers from agent-written content)
- `appendix-numerical.tex` — 5 TODOs (continuity, simplicity, billiard pruning, verdict framework, unverified numerical claim)
- Other chapters — no TODOs but Jörn doubts publishability

**What agents can do:** Draft rewrites, improve flow, verify claims against code/data, fix notation inconsistencies, improve figure quality. But agents cannot decide the thesis structure or story — that's Jörn's.

**Meta-layer cleanup:**
- 🔴 Jörn reviews all convention skills (not just diffs) — the "agent-usage expert" review
- 🟢 CLAUDE.md "Working notes (redistribute later)" section — items should move to skills or other locations
- 🟢 Devcontainer rebuild (Jörn, low priority) — Dockerfile updated with nextest + cargo-watch

**Final assembly tasks (after content is stable):**
- Abstract, introduction, conclusion
- Bibliography check (all citations verified against papers/)
- Figure quality review (all experiments, per python-conventions + figure-review agent)
- Proofreading pass
- Print formatting

## 9. Agent infrastructure (deferred — after thesis deadline)

- 🟡 ~~Split monolithic review agent into per-concern reviewers~~ — Done (2026-03-22). Generic review agent now loads ONE convention skill per spawn; math-review is a separate dedicated agent.
- 🟡 Audit convention skills for cargo cult / unjustified items. Found one so far (iterator chain preference — removed). ~25 conventions lack stated justifications, some may be wrong. Jörn reviews items flagged by agents. See session 2026-03-22 audit for the full list.

---

## Detailed refactor specs (referenced from §6)

### Unify `find_positive_beta_1d` / `find_positive_beta_nd` in kkt.rs

**What:** Replace both functions with a single LP-based approach. `maximize min_j β_j` subject to `β = β₀ + V·α` handles all null-space dimensions uniformly.

**Why:**
- 1d/nd split has no profiling justification
- nd "coordinate ascent" is ad-hoc, not a standard algorithm
- The nd path is the only untested code path

**Thesis/code tension:** Main thesis (`lem:rank-deficiency-dismissal`) proves rank-deficient pairs are redundant. Code searches null space for β>0 on *near*-singular systems (approximate rank deficiency). Not contradictory but needs explicit documentation:
- Exact: rank-deficient → discard (smaller pair dominates)
- Numerical: near-singular → pseudoinverse β₀ may have β_i < 0 from noise; null-space shift recovers feasibility without changing Q

**Scope:** Replace both functions, add appendix-numerical writeup, verify on regression tests.

### Audit Reeb vector factor — AUDITED, CLEAN (2026-03-14)

No wrong instances found. `reeb_vector()` renamed to `reeb_direction()` during migration.

### KKT solver: projection-based variant

See §6b above. Detailed 5-step algorithm:
1. Solve `(N^T | h^T) β = (0 | 1)` → (m-5)-dim affine solution space
2. Project H → (m-5)×(m-5) reduced Hessian H'
3. Eigendecompose H' → near-null = constant-action directions
4. β > 0 as LP feasibility on projected null space
5. Recover multipliers from Hβ + Nμ + hν = 0

Keep augmented-system solver for ablation comparison.

### Dual-vertex parameterization

See §6a above.

### Experiment code extraction

See §6d above and `handoffs/experiment-deduplication.md`.

### Solver numerical behavior math.tex 🟢

Create `crates/src/math.tex` (or per-module files) collecting numerical analysis results
for the solvers used in the crate: SVD backward stability, condition number bounds,
LU error bounds, eigendecomposition stability. Multiple modules (prefilter, constraint_solver,
orbit_recovery) currently use SVD without citing a shared error analysis.
