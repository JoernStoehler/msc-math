# TASKS

Master task list for thesis completion. **Deadline: mid-April 2026.**

## How to use this file

- **Format:** Each task is a `##` section with status, description, next steps, and dependencies.
- **Maintenance:** Update when a task's status, blockers, or dependencies change. Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts, which binaries compile) — run the command instead. An agent acting on stale cached state has worse consequences than the seconds it takes to run `cargo test`.
- **Status dates:** Always include a date when updating status (e.g. "Status (2026-03-24)").
- **Completed tasks:** Move to the `## Completed` section at the bottom with a one-line summary.
- **Dependencies:** State what blocks a task. If it's Jörn-gated, say so explicitly.
- **Scope:** This file tracks WHAT needs doing. HOW is in skills, conventions, and logbooks.

**Current state (2026-03-28):** No thesis chapter is publishable yet. Experiments have data but writeups are noisy and the thesis doesn't tell a coherent story. Procedural layer fully rewritten and ready for testing. Progress rate on track.

**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.

---

## gradient-search experiment

**Status (2026-03-26):** Does not compile. Blocked on API migration.

Two-binary pipeline: `generate_seeds` → `seeds.jsonl` → `gradient_search` → `results.jsonl`. H-only gradient ascent with step-bound overshoot (crosses combinatorial boundaries) + wiggle. Interruptible/resumable. Branch: `gradient-search`.

**Build failure (2026-03-26):** Imports nonexistent `capacity_derivatives_h`, `volume_derivatives_h`, `normals_f64()`, `heights_f64()`. Library was refactored to dual-vertex `_a` API but this experiment was never migrated. Requires reworking derivative calls and step-bound computation to use `capacity_derivatives_a` / `volume_derivatives_a`.

**Next steps:**
1. **Migrate to `_a` API** — see `handoffs/experiment-api-fixes.md`
2. Run production computation (270 seeds, ~10 min locally, trivially parallelizable on LICCA)
3. Analyze results, produce figures

See `crates/exp-sys-optimization/gradient-search/logbook.md` for details.

---

## thesis-code-alignment

See `handoffs/migration-thesis-findings.md` for the full list. Highest priority items:

1. **Tube rotation increment** — code is a heuristic claiming to implement `[def:rotation-increment]`. Fix doc comment, or implement CH2021 Lem.2.21.
2. **KKT notation** — unify (λ,ν) vs (μ,ξ) across thesis sections and code.
3. **Accumulator pattern** — describe two-tier certified/uncertain tracking in thesis algorithm boxes (currently only in appendix A.3-A.4).
4. **qp_assembly dual-vertex formulation** — unverified mathematical equivalence with normals/heights formulation (has explicit TODO).
5. **KktResult→Solution bridge** — `margin = min(beta)` + `classify_margin()`. Is this the right verdict mapping?

Depends on: Jörn decides which side to fix for each item.

---

## math.tex content audit

**Status (2026-03-27):** Several experiment math.tex files contain not just mathematical proofs but also figures, result tables, and experiment writeups. This is wrong — math.tex should contain only lemma/theorem statements, proofs, definitions, and formal derivations. Prose, figures, and result discussions belong in logbook.md (for developers) or thesis/ (for examiners).

**Known offenders (from root math.pdf build):** experiments that include `\includegraphics` or data tables in their math.tex: ablation, gradient-descent, lagrangian-products, omega-obstacle, orbit-recovery, pentagon-perturb, random-product-sweep, random-sweep, sys-optimization, visualization.

**Fix:** Move non-proof content out of math.tex into logbook.md or separate files. Low priority — doesn't block anything, but makes the combined math.pdf noisy.

---

## 3b. Audit math.tex stubs for lost mathematical backing

When algorithmic lemmas were migrated from code doc comments to math.tex, some may have lost their connection to source material (papers, thesis definitions). Scan all math.tex `[TODO: JÖRN -` entries and check: was there ever a proof or citation? Did it get dropped during migration? Example: `lem:positive-span` and `lem:vertex-enumeration` in `geom/math.tex` have been proof-less stubs since their first commit — the thesis has Jörn-approved definitions of what a polytope is, but no proofs of the algorithmic facts the code relies on (positive spanning ↔ bounded, vertex enumeration correctness, irredundancy via affine rank).

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

Depends on: Jörn verifies the math. Agent writes drafts, Jörn reviews.

---

## hko-local-maximality

**Goal:** Show HKO2024 is a local maximum of sys. Potentially publishable alongside the thesis.

**Literature (2026-03-23):** BBLM2023 (arXiv:2303.13348, `papers/bblm2023/`, bib `BBLM2023`) classifies smooth local maximizers of c_k-hat in dim 4: they are rational ellipsoids. For k=1 (= c_EHZ, our case), only the round ball. Techniques (Clarke functional, Morse theory, C³ topology) are fundamentally smooth — don't extend to polytopes. **The polytope local maximality question is genuinely open.** HKO2024 paper (line 642) explicitly calls for "further study of non-smooth domains with Besse-type dynamics."

**What's done:**
- **h-space (normals fixed):** 0 ∈ conv(subdifferential) by symmetry + Euler homogeneity. Jörn has not verified the proof.
- **F=10 (n,h)-space:** Phase C LP confirms 0 ∈ conv(44 per-orbit gradients in R^40). First-order necessary condition for local max satisfied. 16 flat directions remain. Script: `crates/exp-hko-local-maximum/gradient-is-zero/phase_c_lp_test.py`.
- **F=11 facet-splitting:** 211 cuts all decrease sys (sampling-based, facet 5 incomplete: 11/200 rows).
- **LP(5,5) perturbations:** 100 random, all lower (pentagon-perturb experiment).

**Next steps (priority order):**
1. **Second-order analysis of flat directions** — compute sys along each flat direction via finite differences. Determines whether first-order necessary condition → actual local max. Agent-executable. This is the critical gap. Note (2026-03-26): the 16 flat directions were computed in (n,h)-space including gauge directions. After a_i migration, Phase C LP could be redone in clean R^{40} with no gauge — possibly fewer true flat directions. Consider waiting for migration or doing both.
2. **Jörn verifies h-space proof** — Danskin + symmetry + Euler homogeneity argument in `crates/exp-hko-local-maximum/gradient-is-zero/logbook.md` lines 151-156. ~15 min. Then formalize in math.tex.
3. **Complete Phase B** — facet-splitting now has 536 directions (regenerated 2026-03-26, was 212). All decrease sys.
4. **Convex-body direction** (Phases E/F in logbook) — Minkowski smoothing or F-refinement. Completely untested direction. Needs scoping: can billiard algorithm handle K+εB⁴?
5. **Structural explanation** — why does pentagon geometry force 0 ∈ conv? Relates to golden ratio β-structure, order-10 symmetry. Jörn's domain.

**Key files:** `crates/exp-hko-local-maximum/gradient-is-zero/logbook.md` (full analysis), `crates/exp-hko-local-maximum/gradient-is-zero/phase_c_lp_test.py` (Phase C script), `crates/exp-hko-local-maximum/gradient-is-zero/math.tex` (theory).

---

## experiment-quality

The thesis is currently a dump of results, not a coherent narrative. Experiments have data but writeups are noisy.

**Jörn needs to decide:**
- What is the thesis's central argument / story arc?
- Which existing experiments support that story vs are background validation vs are noise?
- What depth does each experiment need in the thesis? (full section / brief mention / appendix / omit)

**Known gaps:**
- `crosspolytope` Phase 2 TODO: update known_polytopes.rs (tracked in its logbook)
- `crates/exp-hko-local-maximum/gradient-is-zero` Phase C (2026-03-23) verified first-order necessary condition for local max in F=10 (n,h)-space via LP. See `hko-local-maximality` task for next steps.

**Gradient experiment redesign (2026-03-26, Jörn):** The three gradient experiments (`sys-optimization`, `gradient-descent`, `gradient-search`) evolved incrementally and overlap significantly. Replace with three cleanly scoped experiments:
1. **gradient-correctness** (Q5b/Q5c complete, merged to main 2026-03-28) — Per-orbit gradient validated (slope=2.00). Q5b: 12 polytope types. Q5c: direction-filtered subdiff is a negative result. KktOutcome enum, error handling convention, code-math audit. 14 tests fail due to wrong Q error bound math (lem:q-error-bound too loose). See logbook, TASKS.md q-error-threshold.
2. **combinatorial-boundaries** (complete, 2026-03-27) — Random cells convex, product cells non-convex (0% vs 100% transition failures). ~F boundaries per gradient step. Orbit facets 2× wider than non-orbit. Gradient-cell alignment favorable (r=0.52). sys continuous, gradient stable except at orbit switches (3%/boundary, up to 70° jump). See logbook.
3. **sys-search** (dev run complete, branch `sys-search`) — Gradient-based search for sys > 1. Dev run: 42 seeds, best sys=0.933, wiggle dominates overshoot. Next: landscape characterization and search strategy comparison. See `handoffs/sys-search.md`.

Dependency chain: #1 validates the gradient → #2 characterizes the obstacle → #3 applies the tool. #3 can start independently but benefits from #2's findings. Delete old experiments once new ones are confirmed better.


**Experiment ideas:** See `IDEAS.md` (root).

**New experiment ideas (2026-03-26, discussion with Jörn):**
- **lagrangian-search** (Phase 1 complete, 2026-03-27) — Measured the sys>1 region around HKO2024 via dense perturbation sweep (6500 samples, 13 ε-levels) and directional boundary probing (500 rays). Key findings: characteristic per-component radius ε*≈0.035, anisotropic boundary with 7× aspect ratio, shape governed by combinatorial orbit structure (not smooth geometry). Random sampling in full LP space is hopeless (~10⁻³¹ volume fraction). Phases 2-3 (guided search, novelty check) deferred. See `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md`.
- **Higher polygon Lagrangian products** — LP(5,7), LP(7,7) etc. untested, HKO came from LP(5,5). Partially covered by lagrangian-search Phase 1.
- **Dimension scaling** — how does max-achievable-sys scale with F for random polytopes? Scattered data exists but no systematic study.

Depends on: Jörn scoping the thesis story. Derivative-related experiments also depend on dual-vertex-parameterization (library derivative API, now mostly complete).

**Cross-experiment cleanup (2026-03-27):**
- **Step-bound code duplication:** `compute_step_bound` (incidence + ω₀ detection in a-space) exists in sys-optimization, combinatorial-boundaries, sys-search. sys-search version is missing ω₀ detection (43% of boundaries). Candidates: unify into library, or at minimum copy the enriched version from combinatorial-boundaries into sys-search.
- **Products-vs-random split:** Every gradient experiment should split analysis by source dataset. The 0%/100% convexity split is a fundamental structural difference that affects strategy choice. Could add a standard `source_dataset` analysis function to figure_config.py.
- **Wiggle strength justification:** sys-search uses 5% (from gradient-search, unjustified). combinatorial-boundaries provides per-facet cell widths (0.12–0.26) that could inform this. See combinatorial-boundaries logbook "Unused synergies" section.
- **sys-search + combinatorial-boundaries overlap:** The multi-boundary sweep with sys tracking (combinatorial-boundaries Pass 4) answers a sys-search question. If sys-search grows a similar capability, consider removing it from combinatorial-boundaries to avoid duplication.

---

## verify-numerics

**Status (2026-04-02):** Perturbation chain analysis complete. Infrastructure simplified. Merged to main.

**Done:**
- math.tex: Parts I+II complete (problem, structure, exact algorithm, perturbation chain error bounds)
- Projection solver with diagnostics (projection_solver.rs) + eta_k certification bound
- Exact rational solver (exact_solver.rs) for ground truth
- Saddle-point solver archived in saddle_point_solver.rs (dead code)
- Rust test infrastructure: testdata/ (30 cases/conjecture) + tests.rs (2 tests, both pass)
- Eigendirection scaling confirmed: |delta_alpha_j| ~ eps_mach / |gamma_j|
- Eta bound valid for well-conditioned problems (c = m^2 safety factor, zero violations)
- Proven Q error bound: |Q−Q*| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C). Zero violations on 45K natural polytope problems.
- β > 0 classification: zero false positives on 45K problems. 9 false negatives root-caused.
- Projection solver sign bug identified (crates/src/kkt/projection_solver.rs:93).

**Open:**
- Part III of math.tex (f64 algorithm description) not written
- Eta bound doesn't cover LP null-space search (39 violations on natural data with near-zero eigenvalues)
- GAP in cor:taylor-structure proof (needs Jörn)

**Priority:** Blocks other experiment development. All experiments rely on this machinery.

**Subsumes:** q-error-threshold, numerics portion of code-math-correspondence-audit.

**Location:** `crates/exp-numerical-analysis/error-bounds/`, handoff at `handoffs/verify-numerics-algorithm.md`

---

## code-math-correspondence-audit

**Status (2026-03-28):** Complete. Full report: `handoffs/cross-reference-audit.md`.

~170 cross-references audited across 25 files, citing 45 distinct labels. Results: ~160 OK, 3 MISMATCH (all in saddle_point_solver.rs, all known), 2 TODO (missing lemma labels, properly flagged in code).

**Mismatches (all in saddle_point_solver.rs):**
- M1 (critical): line 358 — lem:q-error-bound bound inapplicable to pseudoinverse (known, tracked in verify-numerics)
- M2 (high): line 477 — lem:well-defined: returned β_final differs from β₀ used for Q computation
- M3 (medium): lines 534-544 — lem:q-error-bound: code comment oversimplifies the proof's Step 4

**Missing labels (both properly TODO-flagged):**
- qp_assembly.rs:58 → lem:dual-vertex-qp (needs writing)
- tube/mod.rs:344 → lem:rotation-increment-approx (needs writing)

---

## q-error-threshold

**Status (2026-03-28):** Subsumed by verify-numerics experiment. Root cause and context preserved below for reference.

The Q error bound E = (9/2)||r||²/|λ_min| ([lem:q-error-bound]) uses |λ_min| of ALL eigenvalues but the code uses |λ_min| of RETAINED eigenvalues. This mismatch was never caught. The verify-numerics experiment will develop a correct error bound as part of its systematic treatment of the QP solver.

**Handoff:** `handoffs/q-error-bound-rework.md`. **Location:** `crates/src/kkt/saddle_point_solver.rs:537-567`

---

## convention-violations

**Status (2026-03-28):** Done.

- `crates/exp-sys-optimization/gradient-validation/run.rs`: catch_unwind already removed in prior session
- `crates/exp-hko-local-maximum/gradient-is-zero/run.rs`: catch_unwind already removed in prior session
- `crates/exp-sys-optimization/combinatorial-structure/run.rs`: 2× catch_unwind removed (unlisted, found during fix)
- `crates/src/kkt/saddle_point_solver.rs`: panic comments rewritten (deferred-work context, root cause, resolution path)
- `crates/src/algorithms/capacity_accumulator.rs`: stale doc comment fixed (gap case returns None, not panic)

---

## thesis-chapters

No chapter is currently publishable. The thesis needs to become a coherent document that tells a story, not a collection of sections.

**Thesis .tex files with open TODOs:**
- `tube-algorithm.tex` — 8 TODOs (5 JÖRN questions, 3 GAP markers)
- `appendix-numerical.tex` — 5 TODOs
- Other chapters — no TODOs but Jörn doubts publishability

**What agents can do:** Draft rewrites, improve flow, verify claims against code/data, fix notation inconsistencies, improve figure quality. Agents cannot decide thesis structure or story.

**Final assembly (after content is stable):**
- Abstract, introduction, conclusion
- Bibliography check (all citations verified against papers/)
- Figure quality review
- Proofreading pass
- Print formatting

Depends on: experiment-quality (thesis story), thesis-todos (math verification).

---

## end-to-end-profiling

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

## dual-vertex-parameterization

Direction (2026-03-22, Jörn): Thesis will introduce both (n_i, h_i) and a_i = n_i/h_i, but primarily work in a_i.

**Update (2026-03-23, Jörn):** Refactor all code & math to use a_i instead of (n_i, h_i). Since we never use ||n||=1 anywhere, a_i is equivalent to assuming h_i=1 and n_i=a_i with no unit-norm constraint. Advantages:
- Gradient ∇_{a_i} sys is a single 4-vector per facet — no separate ∂/∂h and ∂/∂n, no tangent projection to T_{n_k}S³
- KKT constraint simplifies: Σ a_{σ(i)} β_i = 0, Σ β_i = 1 (build_qp already uses this)
- Reeb vector R_i = 2J a_i (instead of 2/h_i · J n_i)
- Eliminates 10D gauge freedom (radial directions) that complicates the Phase C LP test

**Motivation from Phase C (2026-03-23):** The LP test for HKO local maximality works in R^{50} ambient with 10 gauge directions, requiring careful bookkeeping (40 effective DOF, tangent projection). In a_i-space this would be a clean R^{40} LP with no gauge.

**Current state (2026-03-30):** Library API is done. Most experiment migration complete. Math.tex migration complete.

- ✅ `capacity_derivatives_a()` and `volume_derivatives_a()` exist in `derivatives.rs`, tested (FD cross-check)
- ✅ `build_qp` uses a_i internally
- ✅ 4/5 gradient experiments migrated: sys-optimization, hko-neighborhood, gradient-descent, omega-obstacle
- ❌ gradient-search: imports nonexistent `_h` functions (does not compile)
- ✅ math.tex migration: `kkt/math.tex` and `algorithms/math.tex` use a_i throughout (commits b9eedda, 4afefb9, 0ff6c6d)
- ❌ Jörn verification: `[lem:cap-derivative]` and `[lem:vol-derivative]` in sys-optimization/math.tex marked `\begin{unverified}`
- ❌ `[lem:dual-vertex-qp]`: TODO in qp_assembly.rs:58-61 — prove a_i QP formulation recovers same optimal action as (n,h)

**Remaining work items:**
1. **gradient-search migration** — see `handoffs/experiment-api-fixes.md`. Agent task, small.
2. **Jörn verifies derivative lemmas** — `[lem:cap-derivative]`, `[lem:vol-derivative]` in `crates/exp-sys-optimization/sensitivity-analysis/math.tex`.
3. **Write `[lem:dual-vertex-qp]` proof** — mathematical equivalence of a_i and (n,h) QP formulations. Agent drafts, Jörn verifies.

---

## projection-solver

Implement second KKT solver variant. Keep augmented-system solver for comparison.

5-step algorithm:
1. Solve `(N^T | h^T) β = (0 | 1)` → (m-5)-dim affine solution space
2. Project H → (m-5)×(m-5) reduced Hessian H'
3. Eigendecompose H' → near-null = constant-action directions
4. β > 0 as LP feasibility on projected null space
5. Recover multipliers from Hβ + Nμ + hν = 0

`kkt/projection_solver.rs` already exists with a basic implementation. The refactor is to make it mathematically rigorous and add ablation comparison.

Depends on: Jörn reviews math.

---

## beta-lp-unification

Replace `find_positive_beta_1d` / `find_positive_beta_nd` with a single LP-based approach. `maximize min_j β_j` subject to `β = β₀ + V·α` handles all null-space dimensions uniformly.

**Why:**
- 1d/nd split has no profiling justification
- nd "coordinate ascent" is ad-hoc, not a standard algorithm
- The nd path is the only untested code path

**Thesis/code tension:** Main thesis (`lem:rank-deficiency-dismissal`) proves rank-deficient pairs are redundant (exact rank deficiency → discard, smaller pair dominates). Code searches null space for β>0 on *near*-singular systems — pseudoinverse β₀ may have β_i < 0 from noise; null-space shift recovers feasibility without changing Q. Not contradictory but needs explicit documentation.

**Status (2026-03-22):** Previous `kkt-lp-refactor` worktree has 17 commits (20+ commits behind main). Not worth rebasing — start fresh, using old branch as reference. Key content to salvage:
- Unified `find_positive_beta` function design (4 cases)
- Near-null eigenvector Type A/B/C classification
- Open question for Jörn: is filtering Type A directions mathematically justified?

Old worktree at `.claude/worktrees/kkt-lp-refactor/` — read-only reference.

---

## solver-numerical-math-tex

Create per-module math.tex files collecting numerical analysis results for the solvers: SVD backward stability, condition number bounds, LU error bounds, eigendecomposition stability. Multiple modules (prefilter, constraint_solver, orbit_recovery) currently use SVD without citing a shared error analysis.

---

## tube-algorithm

See `handoffs/tube-algorithm.md`. Migration created a fresh implementation in `algorithms/tube/mod.rs` (1175 lines) but:
- Rotation increment is a heuristic, not the real CH2021 formula
- Correctness proof is a sketch with GAPs
- Performance on F>10 polytopes is untested

Depends on: Jörn reviews math.

---

## collaboration-skill

**Status (2026-03-26):** Deferred. Need evidence from real work sessions before designing.

Evidence gathered so far (from session log analysis):
- 57% of agent-era sessions use zero subagents — under-delegation is the main problem
- Sessions just stop without cleanup (0/362 self-initiated postmortems)
- Subagent prompting is fine — subagents self-serve skills and rules via shared system prompt
- Handoff quality is good when created, but creation is bursty not routine

**Next steps:** Run real work sessions with the new procedural layer. Observe whether "use proactively" descriptions + CLAUDE.md delegation nudge address under-delegation. Collect feedback in `feedback/`. Design the skill based on observed failures, not predictions.

---

## Completed

- **migration-merge** — DONE (2026-03-19, commit 6680d0e)
- **test-data-pipeline** — DONE (2026-03-22)
- **logbook-migration** — DONE (2026-03-22)
- **migration-cleanup** — DONE (2026-03-22, commit f073e13)
- **convention-contradiction** — DONE (2026-03-17)
- **reeb-vector-audit** — DONE (2026-03-14)
- **review-agent-split** — DONE (2026-03-22)
- **experiment-code-extraction** — DONE (commit 8dcc7c4). Extracted derivatives, facet volumes, KKT multipliers to library. ~2035 LOC removed from 4 experiments.
- **reproduce-decentralization** — DONE (2026-03-23). Deleted `reproduce.sh`; all 19 logbooks have "How to run" sections.
- **slurm-skill** — DONE. `.claude/skills/slurm/SKILL.md` exists.
- **meta-layer-refactor** — DONE (2026-03-24). Simplified procedural layer, created agent-design workflow skill.
- **procedural-rewrite** — DONE (2026-03-26). Full rewrite: 5 rules, 1 output style, 8 skills, 9 agents, CLAUDE.md updated. Old unvetted skills/agents deleted. Collaboration skill deferred. Feedback collection set up.
