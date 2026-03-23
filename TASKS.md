# TASKS

Master task list for thesis completion. **Deadline: March 31, 2026.**

**Current state (2026-03-23):** No thesis chapter is publishable yet. Experiments have data but writeups are noisy and the thesis doesn't tell a coherent story. Migration complete and merged. Meta-layer refactored. Progress rate on track.

**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.

**Maintenance rule:** Update this file immediately when completing, discovering, or learning something that affects a task. Context not written down here is context that will be lost.

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

## thesis-todos

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

## experiment-quality

The thesis is currently a dump of results, not a coherent narrative. Experiments have data but writeups are noisy.

**Jörn needs to decide:**
- What is the thesis's central argument / story arc?
- Which existing experiments support that story vs are background validation vs are noise?
- What depth does each experiment need in the thesis? (full section / brief mention / appendix / omit)

**Known gaps:**
- `sys-optimization` needs a redesign, not just modernization — didn't use proper gradients, didn't look at cuts
- `crosspolytope` Phase 2 TODO: update known_polytopes.rs (tracked in its logbook)
- `hko-neighborhood` Phase C (2026-03-23) proved first-order local max in F=10 (n,h)-space via LP. Remaining: 16 flat directions need second-order analysis; F=11 and convex-body directions untested (Phases E, F in logbook)

**Experiment ideas:** See `IDEAS.md` (root).

Depends on: Jörn scoping the thesis story. Derivative-related experiments also depend on dual-vertex-parameterization (library derivative API).

---

## thesis-chapters

No chapter is currently publishable. The thesis needs to become a coherent document that tells a story, not a collection of sections.

**Thesis .tex files with open TODOs:**
- `tube-algorithm.tex` — 8 TODOs (5 JÖRN questions, 3 GAP markers)
- `appendix-numerical.tex` — 5 TODOs
- Other chapters — no TODOs but Jörn doubts publishability

**What agents can do:** Draft rewrites, improve flow, verify claims against code/data, fix notation inconsistencies, improve figure quality. Agents cannot decide thesis structure or story.

**Meta-layer cleanup:**
- Jörn reviews all convention skills (not just diffs) — includes auditing for cargo cult / unjustified items (~25 conventions lack stated justifications)
- CLAUDE.md "Working notes" section — items should move to skills or other locations
- Devcontainer rebuild (low priority) — Dockerfile updated with nextest + cargo-watch

**Final assembly (after content is stable):**
- Abstract, introduction, conclusion
- Bibliography check (all citations verified against papers/)
- Figure quality review
- Proofreading pass
- Print formatting

Depends on: experiment-quality (thesis story), thesis-todos (math verification).

---

## end-to-end-profiling

The existing benchmark experiment only times capacity computation, not the full pipeline. `Polytope4D::new` is not profiled but may dominate wall time for larger polytopes.

**Work items:**
1. Add end-to-end timing to benchmark experiment, broken into phases (construction, permutation enumeration, KKT solve, orbit recovery)
2. Produce a figure showing phase breakdown vs facet count
3. Update `benchmark/logbook.md` with findings

---

## polytope-construction-optimization

`Polytope4D::new` dominates end-to-end systolic ratio computation for F ≤ 10 (80-92% of total time). At F=10, construction is 83 ms vs 17 ms for capacity. Bottleneck is exact rational arithmetic (BigRational vertex enumeration via C(F,4) subset solves). SVD pre-filter already skips 70-80% of subsets.

**Data:** See `experiments/benchmark/logbook.md`, `profiling/phase_breakdown.png`, flamegraphs at F=9 and F=11.

**Goal:** Reduce construction time. No correctness compromises.

Depends on: end-to-end-profiling (for baseline data).

---

## dual-vertex-parameterization

Direction (2026-03-22, Jörn): Thesis will introduce both (n_i, h_i) and a_i = n_i/h_i, but primarily work in a_i.

**Update (2026-03-23, Jörn):** Refactor all code & math to use a_i instead of (n_i, h_i). Since we never use ||n||=1 anywhere, a_i is equivalent to assuming h_i=1 and n_i=a_i with no unit-norm constraint. Advantages:
- Gradient ∇_{a_i} sys is a single 4-vector per facet — no separate ∂/∂h and ∂/∂n, no tangent projection to T_{n_k}S³
- KKT constraint simplifies: Σ a_{σ(i)} β_i = 0, Σ β_i = 1 (build_qp already uses this)
- Reeb vector R_i = 2J a_i (instead of 2/h_i · J n_i)
- Eliminates 10D gauge freedom (radial directions) that complicates the Phase C LP test

**Motivation from Phase C (2026-03-23):** The LP test for HKO local maximality works in R^{50} ambient with 10 gauge directions, requiring careful bookkeeping (40 effective DOF, tangent projection). In a_i-space this would be a clean R^{40} LP with no gauge.

**Current state:** Code stores `dual_vertices` (a_i) as primary representation, computes (n_i, h_i) on demand. `build_qp` already uses a_i. But `build_augmented_system`, `derivatives.rs`, and all experiment gradient code use (n_i, h_i).

**Work items:**
1. **Math:** Write the a_i-only KKT formulation and gradient formulas in math.tex. The formulas are: ∂A/∂a_k = envelope theorem on Q(β) = (1/2) Σ_{i<j} β_i β_j ω₀(a_{σ(i)}, a_{σ(j)}), constraint A^T β = 0, 1^T β = 1. Capacity A = 1/(2Q). (Jörn to verify the derivation.)
2. **Library API:** Add `pub fn capacity_derivatives_a(...)` to `derivatives.rs` returning ∂c_EHZ/∂a_k ∈ R^4 per facet.
3. **Experiment migration:** Replace (n,h) gradient code in hko-neighborhood, gradient-descent, sys-optimization with library API calls.
4. **Math.tex migration:** Update `kkt/math.tex` and `algorithms/math.tex` from (n_i, h_i) to a_i.

**Library derivative API:** The library computes c_EHZ but provides no ∂c_EHZ/∂a_i. Needed by multiple experiments and the subdifferential analysis for local maximality.

**Experiment code duplication:** Three experiments independently implement ∂sys/∂h and ∂sys/∂n in (n, h) space (~2100 + 700 + 600 LOC across hko-neighborhood, gradient-descent, sys-optimization) plus Phase C's Python reimplementation. Once a_i-only formulation is settled, replace all with a library API.

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

## slurm-skill

Create `.claude/skills/slurm/SKILL.md` with `references/example.sh` for writing SLURM job scripts targeting LICCA cluster. Agents write scripts + Rust binaries; Jörn runs `ssh licca && sbatch job.sh`.

Depends on: Jörn provides example template.

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
