# Project Tracker

Unified tracker for thesis, experiments, and infrastructure.
Format: `## [status] Group` / `### [status] [date] Item`. Body only when header isn't enough.
See `.claude/rules/tasks.md` for full conventions. Run `bash scripts/tasks-toc.sh` for section index.

**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.
**Maintenance:** Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts) — run the command instead.
**Dependencies:** thesis/ is stale and will be restructured — most thesis work is blocked on restructuring decisions. Work on crates/ (code, math.tex, experiments) is independent and can proceed now.

## [open] Q1: HKO2024 local maximality

Main conjecture: HKO2024 is a local maximum of the systolic ratio. Potentially publishable alongside thesis.
Key files: `crates/exp-hko-local-maximum/`, `thesis/handwritten-notes.md`.
Literature: BBLM2023 classifies smooth local maximizers (only ball for k=1). Polytope case genuinely open.
HKO2024 lives in multiple ambient spaces (LP(5,5), LP(6,5), F=10, F=13, convex bodies) — local max in one space != local max in a larger space.

### [done] [2026-04] 1a. First-order analysis in a_i space (gradient-analysis)
- Rank 25 in R^40, 15 flat directions. LP confirms 0 in conv(150 per-orbit gradients).
- `crates/exp-hko-local-maximum/gradient-analysis/logbook.md`

### [done] [2026-04] 1e. Second-order analysis along flat directions
- All 15 basis + 100 random curvatures negative (-0.31 to -0.02). Supports local maximality.
- `crates/exp-hko-local-maximum/second-order/`

### [done] [2026-03] 1b. Facet-splitting (F=10 to F=11)
- 536 cuts, all decrease sys.
- `crates/exp-hko-local-maximum/facet-splitting/`

### [done] [2026-03] 1b. Cut-and-ascent (cut then gradient ascent)
- 0/20 trials improved over HKO2024.
- `crates/exp-hko-local-maximum/cut-and-ascent/`

### [done] [2026-03] 1c. Subdifferential LP in (n,h)-space
- Phase C LP confirms 0 in conv(per-orbit gradients). Superseded by 1a (a_i space, no gauge).
- `crates/exp-hko-local-maximum/subdifferential-lp/`

### [done] [2026-03] 1d. Lagrangian boundary mapping
- Characteristic radius ~0.035, anisotropic (7x aspect ratio), ~10^-31 volume fraction.
- `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md`

### [done] [2026-03] Perturbation neighborhood (LP(5,5) random perturbations)
- 100 random perturbations all retain sys>1 (min 1.002, max 1.033). HKO highest.
- `crates/exp-hko-local-maximum/perturbation-neighborhood/`

### [Jörn] Verify h-space proof
- Danskin + symmetry + Euler homogeneity argument. ~15 min.
- `crates/exp-hko-local-maximum/gradient-analysis/logbook.md` lines 151-156

### [Jörn] Verify second-order math.tex proposition
- Non-smooth second-order sufficiency proof sketch needs rigor check.
- `crates/exp-hko-local-maximum/second-order/math.tex`

### [future] F-refinement convergence (increasing F as smooth approximation)

### [future] Convex-body direction (Minkowski smoothing K + eps*B^4)
- Needs scoping: can billiard algorithm handle non-polyhedral bodies?

### [future] Structural explanation for 0 in conv
- Why does pentagon geometry force this? Golden ratio, order-10 symmetry.

## [open] Q2: Novel sys>1 polytopes

Stronger conjecture: HKO2024 may be (up to perturbation/symplectomorphism) the only sys>1 case.

### [done] [2026-04] 2a. Rotated regular products
- Only 5x5 at theta=18deg achieves sys>1 among 3<=n,m<=6 (6deg resolution) + 7x7 separately. 7x7 peaks at 0.917. Mixed 7-pairs not tested.
- `crates/exp-sys-landscape/rotated-regular-products/`

### [done] [2026-04] 2b. Gradient ascent from random starts
- General: 10 seeds, best sys=0.9005. Products: 12 seeds, best sys=0.9127. No sys>1.
- `crates/exp-sys-landscape/gradient-ascent-general/`, `gradient-ascent-products/`

### [done] [2026-03] 2c. Perturbation neighborhood (structurally different sys>1?)
- 100 random perturbations all retain sys>1 but none exceed HKO2024.
- `crates/exp-hko-local-maximum/perturbation-neighborhood/`

### [done] [2026-04] 2d. Variable-F ascent (F to F+1)
- 90 trials. F=10 local maxima often improve at F=11 but marginal; no sys>1.
- `crates/exp-sys-landscape/variable-f-ascent/`

### [done] [2026-03] Random sampling (general + products + calibration)
- Random polytopes max sys=0.578. Random products max sys=0.794 (6x6).
- `crates/exp-sys-landscape/random-sample/`, `random-product-sample/`, `rejection-calibration/`

### [future] Regular Lagrangian product formula fitting
- Dense (n, m, theta) sweep. Fit sys(n, m, theta). Does formula predict sys>1 only for 5x5?
- Partial data in `crates/exp-sys-landscape/rotated-regular-products/`

### [future] Massive random search on LICCA
- Random polytopes with gradient descent and combinatoric-changing step sizes.
- Also: Lagrangian products (density near HKO not negligible), HKO neighborhood.

### [future] Analytical formula for sys(P_5 x R(theta) P_5)
- Kai asked for this. Requires by-hand orbit analysis guided by empirical data.

## [open] Q3: sys landscape structure

sys as a continuous function on polytope space, no privileged threshold.

### [done] [2026-03] 3a. Omega hypothesis (small symplectic area -> high sys?)
- Falsified. Zero correlation (rho=-0.02).
- `crates/exp-combinatorial-cells/omega-hypothesis/`

### [done] [2026-03] 3b. Combinatorial boundary behavior
- Random cells convex, product cells non-convex (0% vs 100% transition failures).
- ~F boundaries per gradient step. Orbit facets 2x wider than non-orbit.
- sys continuous, gradient jumps up to 70deg at orbit switches (3%/boundary).
- `crates/exp-combinatorial-cells/` (cell-widths, boundary-characterization, gradient-discontinuity, convexity, multiple-crossings)

### [done] [2026-03] 3c. sys distribution for random polytopes
- No random polytope exceeded sys=0.80.
- `crates/exp-sys-landscape/random-sample/`, `random-product-sample/`

### [done] [2026-03] 3d. Gradient validation
- Per-orbit gradient validated (slope=2.00) across 12 polytope types.
- Direction-filtered subdifferential is a negative result.
- `crates/dev-gradient/numerics/`, `numerics-edge-cases/`, `numerics-subdifferential/`

### [future] Systematic landscape analysis
- Gradient flow convergence, local maxima below sys=1, random noise effects.
- Partial data in gradient-ascent experiments.

## [open] Q4: Computing capacity better

Instrument development. Results promote to `crates/library/`.

### [done] [2026-03] 4a. Algorithm comparison (ablation, benchmark, profiling)
- A2 pruning: ~1078x speedup at F=10. Construction dominates for F<=10 (80-92%).
- `crates/dev-algorithm-comparison/`

### [done] [2026-03] 4c. Capacity axiom validation
- All 6 axioms pass. 112/112 orbit-recovery polytopes pass.
- `crates/dev-capacity-validation/`

### [done] [2026-03] 4b-partial. Q error and KKT inertia
- 1.13M nodes, worst E=2.9e-11. Empirically exact at f64.
- Eigenvalue inertia formula holds for 6/7 polytopes, 5 mismatches are threshold artifacts.
- `crates/dev-numerical-analysis/q-error/`, `kkt-inertia/`

### [active] 4b. Numerical error bounds (verify-numerics)
- math.tex Parts I+II complete. Proven Q error bound, eta bound for well-conditioned problems.
- 14 previously-failing tests now pass (329 pass, 0 fail).
- Rationale for current state: degenerate orbits are never capacity-achieving, so final capacity comes from well-conditioned orbits with proven low error. Gap remains for publication.
- Open: Part III (f64 algorithm description), eta bound for LP null-space search (39 violations on natural data with near-zero eigenvalues), GAP in cor:taylor-structure proof (needs Jörn).
- Known bug: projection solver sign bug at `kkt/projection_solver.rs:93`.
- `crates/dev-numerical-analysis/error-bounds/`, `handoffs/verify-numerics-algorithm.md`

### [open] Projection solver
- 5-step algorithm: (1) solve equality constraints → (m-5)-dim affine space, (2) project H → reduced Hessian, (3) eigendecompose → null directions, (4) beta>0 as LP on projected null space, (5) recover multipliers.
- Basic implementation in `kkt/projection_solver.rs`. Needs mathematical rigor + ablation comparison.
- `handoffs/verify-numerics-algorithm.md`

### [open] Beta-LP unification
- Replace `find_positive_beta_1d`/`find_positive_beta_nd` with single LP: maximize min_j beta_j subject to beta = beta_0 + V*alpha.
- Previous branch deleted (tip `7ca81b53` has salvageable design: unified function, Type A/B/C eigenvector classification).
- Thesis/code tension: thesis proves rank-deficient pairs are redundant (discard); code searches null space for beta>0 on *near*-singular systems (pseudoinverse beta_0 may have beta_i < 0 from noise; null-space shift recovers feasibility without changing Q). Not contradictory but needs explicit documentation.
- Open question for Jörn: is filtering Type A directions mathematically justified?

### [open] Solver numerical math.tex
- Per-module math.tex for SVD, condition numbers, LU, eigendecomposition stability.
- Multiple modules use SVD without shared error analysis.

### [done] [2026-04] Crosspolytope capacity
- c_EHZ = 4.0 (same as hypercube), sys=0.75. Exhaustive search through m=13.
- `crates/crosspolytope/`

### [future] Crosspolytope optimality proof
- Minimizing orbit has clean structure (uniform beta, max omega). Symmetry argument may avoid exhaustive enumeration.

## [open] Thesis

thesis/ is stale (see `thesis/handwritten-notes.md`). Most work here is blocked on restructuring decisions.
tube-algorithm.tex and appendix-numerical.tex TODOs are about math correctness, independent of restructuring.

### [Jörn] Thesis restructuring
- Current content stale. Decisions needed: chapter structure, what content survives, what gets rewritten.
- a_i replaces (n,h). Sign conventions changed. Simplification theorem ordering changed.
- Blocks: S0, experiment writeups, experiments chapter, introduction, conclusion.
- See `thesis/handwritten-notes.md` for narrative notes.

### [Jörn] tube-algorithm.tex (8 TODOs)
- 5 Jörn questions (quaternionic formula, TF_ij equivalence, rotation number, closing steps, correctness proof).
- 3 GAP markers (agent-added unverified content).
- `handoffs/tube-algorithm.md`

### [Jörn] appendix-numerical.tex (5 TODOs)
- Continuity of c_EHZ on polytopes, simplicity assumption, billiard pruning, three-valued verdict, unverified numerical statement.

### [blocked] Tube rotation formula implementation
- Current code is a misleadingly named placeholder that is wrong (not CH2021, not any correct formula).
- Need to implement a correct rotation formula. Not necessarily CH2021 — we have different basis vectors.
- Performance on F>10 polytopes untested.
- Blocked on: Jörn reviewing the math (proofs for what formula is correct given our basis).
- `crates/library/src/algorithms/tube/mod.rs`

### [blocked] S0: Notation restructure
- Blocked on: thesis restructuring. Only applies to content that survives — new content will use a_i from the start.
- a_i replaces (n,h). Sign convention for Lagrange multipliers changed. Simplification theorem ordering changed.
- Propagate through all thesis .tex files.

### [blocked] Experiment writeup drafts
- Blocked on: thesis restructuring (need chapter structure and framing decisions).
- Logbooks already contain factual summaries. Agent value-add: thesis-style prose from logbook + data.
- Agent cannot decide framing (how experiment serves thesis argument) — only Jörn can.
- "Just try it" for 1-2 well-defined experiments first (e.g., gradient-analysis, rotated-regular-products). Trash if output is just paraphrased logbook.

### [blocked] Experiments chapter
- Blocked on: thesis restructuring, experiment writeup quality.
- `thesis/experiments.tex` has 1 TODO.

### [blocked] Introduction
- `thesis/main.tex` has 1 TODO (write introduction).
- Blocked on: stable chapter content.

### [blocked] Conclusion
- Blocked on: stable chapter content.

### [open] Thesis figure consistency check
- Conditional: only makes sense if current thesis .tex content is kept rather than rewritten.
- Verify every `\includegraphics` in `thesis/**/*.tex` points to an existing file in `thesis/assets/`.
- Check whether asset files are stale copies of regenerated crates/ originals (compare timestamps/content).
- Report broken references and stale copies. Does not decide what new figures to create — that's a thesis-writing decision.

### [open] Final assembly
- Abstract, bibliography check (includes verifying agent-produced bib entry at `thesis/bibliography.bib` line 151), figure quality review, proofreading, print formatting.
- After all content is stable.

## [open] Code quality + alignment

### [active] Code cleanup (session launched 2026-04-07)
- [done] step_bound duplication: enriched version (omega_0 + degeneration detection) extracted to exp-sys-landscape/src/lib.rs. cut-and-ascent gets inline copy.
- [done] Products-vs-random split: already done — separate experiments each process one source dataset.
- [done] Wiggle strength justification: 0.05 justified (~40% of narrowest median cell width 0.124, deliberately strong). Documented on all 4 constants.
- [done] Draft `[lem:dual-vertex-qp]` proof: Lemma 37 in crates/main.pdf p11. Jörn approved, promoted from unverified.
- [done] math.tex stubs audit: 53 explicit stubs + 69 unverified blocks. No proofs lost in migration — stubs were created as stubs. Priority items: lem:cap-derivative, lem:vol-derivative, prop:prefilter-bound, GAP in prop:capacity-symplectic-product.
- [future] gradient-ascent + multiple-crossings overlap: dedup blocked until gradient ascent stabilizes into library. Until then, copy-edit between experiments is correct.
- Note: step_bound upgrade (omega_0 detection) changes experiment behavior if re-run. Existing JSONL not regenerated.

### [active] Paranoia: numerical claims (session launched 2026-04-07)
- Verify every numerical claim in experiment logbooks against actual JSONL data and code output.
- Flag mismatches, fix obvious ones, report the rest.

### [active] Paranoia: conjectures + interpretations (session launched 2026-04-07)
- Flag-only. Audit all conjectures, "we believe" statements, causal claims, unhedged conclusions.
- Produce ranked list of "most embarrassing if wrong."
- No fixes — Jörn reviews the list when writing capacity frees up.

### [active] Thesis-code alignment
- Full list: `handoffs/migration-thesis-findings.md`
- Tube rotation increment: current code is a misleadingly named placeholder, not CH2021. Need to implement a correct rotation formula (not necessarily CH2021 — we have different basis vectors).
- KKT notation: decided — use code's symmetric convention (eigenvalue decompositions pop out). Propagate to thesis.
- Accumulator pattern: thesis is stale, will be rewritten. Not a separate issue.
- qp_assembly dual-vertex: decided — thesis should just use a_i (h=1, a=n). The "equivalence" is trivial: substitute h=1, confirm |n|=1 was never used. Per-proof mechanical substitution.
- KktResult->Solution bridge: needs investigation — unclear whether this is actually a thesis-code alignment issue.
- Note: thesis-side propagation (KKT notation, accumulator, qp_assembly) is blocked on thesis restructuring.

### [active] Dual-vertex parameterization (a_i migration)
- Library API done. Most experiment migration complete. Math.tex migration complete.
- `crates/library/src/algorithms/math.tex` uses a_i throughout.
- Remaining:
  - Jörn verifies `[lem:cap-derivative]` and `[lem:vol-derivative]` (marked `\begin{unverified}`)
  - Write `[lem:dual-vertex-qp]` proof (a_i vs (n,h) QP equivalence)

### [open] Audit math.tex stubs for lost proofs
- Audited 2026-04-07: 53 explicit stubs + 69 unverified blocks. No proofs lost in migration — stubs were created as stubs, agent-written proofs added later.
- **High priority** (blocks thesis or code correctness):
  - `lem:cap-derivative` + `lem:vol-derivative` (algorithms/math.tex:709,781) — core gradient lemmas, also tracked in "Dual-vertex parameterization" below.
  - `prop:prefilter-bound` (geom/math.tex:789) — needs restatement in terms of computable `hat_kappa`. Factor-counting issue: tight bound is 5376, not 1344.
  - GAP in `prop:capacity-symplectic-product` (geom/math.tex:157) — `c_EHZ(A) = area(A)` for 2D convex bodies is unverified, dubious citation.
- **Medium priority** (thesis completeness):
  - 10 definition verifications in geom/math.tex (lines 84-325) — routine review.
  - `thm:conformality` + `thm:sympl-invariance` (algorithms/math.tex:107,120) — standard results, need proofs or citations.
  - 10 Higham/GVL citation numbers (geom + error-bounds) — see "Citation verification pass" below.
  - 3 agent-written proofs needing review: `lem:positive-span`, `lem:vertex-enumeration`, `lem:bounded-triples` (geom/math.tex).
- **Low priority** (dev math, not publication path):
  - 11 stubs + 6 gaps in dev-gradient/ and dev-numerical-analysis/.

### [open] Geom math.tex restructure
- Jörn partially reviewed Defs 1–13 of `crates/library/src/geom/math.tex` (`handoff-geom-math-review.md`).
- Consolidate Defs 1-2 (symplectic form). Add Def for HKO2024 + Thm for false Viterbo's conjecture.
- Clarify H-representation irredundancy. Fix Defs 12-13 (area/volume are algorithms, not definitions).
- Consider splitting into `math_geometry.tex`, `math_symplectic.tex`, `math_reeb.tex`.

### [open] Citation verification pass
- 54 `[TODO: JÖRN - verify]` markers across `crates/**/math.tex` files.
- Most are reference lookups (Higham chapter, GVL section, Wedin/Weyl theorem numbers).
- Agent can look up papers and pre-verify; Jörn reviews flagged problems only.

### [done] [2026-04-07] Delete superseded experiments
- Directories deleted 2026-04-03. Reference cleanup done 2026-04-07: removed gradient-search from code comments (gradient-ascent-general/products run.rs), rules examples, stale handoff.

## [open] Infrastructure + tooling

### [active] Orchestration pattern test
- Testing `/orchestrate` skill + delegation guide on real thesis tasks.
- Baseline commit `f8044b35`. Dry run confirmed Agent() mechanics work.
- Next: post-mortem in `.claude/skills/orchestrate/references/design-space.md`.

### [open] variable-f-ascent merge
- Experiment complete, logbook needs reframing (RQ2 Path D is trivial by construction, reframe as sanity check).
- Branch at `.claude/worktrees/variable-f-ascent/`. Run `/pre-merge` then merge.

### [open] Database cleanup
- Leftover from database implementation: combinatorial-* `catch_unwind` removal, Lagrangian product `Source` lookup, combinatorial-* data regeneration.
- Mechanical code changes + re-running data pipelines.

### [open] Worktree audit
- 4 active worktrees: `api-extract`, `dev-tube-summary` (Apr 4), `numerical-story`, `verify-numerics-part3` (Apr 4).
- Agent investigates each: what's on the branch, is it merged, what's the diff vs main. Report to Jörn.
- **Do not delete any worktree or branch.** Jörn decides what to keep/merge/discard.

### [open] Stale branch cleanup
- 9 branches not in worktrees: `crosspolytope-phase2`, `feedback-triage-3-6`, `hko-second-order`, `math-tex-audit`, `remove-kkt-panics`, `session-search-agent`, `update-pre-merge`, `variable-f-ascent`, `api-extract-rs`.
- Agent checks each for unmerged work, reports. Jörn says delete or keep per branch.

### [open] Evaluate api-reference/ usefulness
- `api-reference/` + `api-extract` crate were built so agents can read stripped .rs files (no bodies/tests/privates).
- As of 2026-04-07: zero organic agent usage found in session history. Agents prefer reading source directly.
- Decision: keep for now, re-evaluate in ~1 week, delete if still unused.
- Check command: `grep -rn "api-reference/library" ~/.claude/projects/-workspaces-msc-math/*.jsonl | grep -v CLAUDE.md | grep -v MEMORY.md | grep -v memory/`
- If no new hits: delete `api-reference/`, `crates/tools/api-extract/`, pre-commit hook in `.pre-commit-config.yaml`, workspace member in `crates/Cargo.toml`, CLAUDE.md references.

### [done] [2026-04] Polytope database
- `crates/database/`, JSONL format, 1198 entries. 6 experiments migrated.

### [done] [2026-03] Polytope4D optimization
- 31x speedup at F=10 via integer-scaled arithmetic + f64 prefilters.
- Remaining: Jörn verifies `prop:integer-cramer`, f64 threshold soundness.

### [done] [2026-03] Slurm skill
- `.claude/skills/slurm/SKILL.md`

### [done] [2026-04] Orchestration infrastructure
- `/orchestrate` skill, delegation guide, cheatsheet.

## [done] Completed tasks (historical)

### [done] [2026-03-19] migration-merge (commit 6680d0e)
### [done] [2026-03-22] test-data-pipeline
### [done] [2026-03-22] logbook-migration
### [done] [2026-03-22] migration-cleanup (commit f073e13)
### [done] [2026-03-17] convention-contradiction
### [done] [2026-03-14] reeb-vector-audit
### [done] [2026-03-22] review-agent-split
### [done] [2026-03-22] experiment-code-extraction (~2035 LOC removed from 4 experiments)
### [done] [2026-03-23] reproduce-decentralization (deleted reproduce.sh, 19 logbooks have "How to run")
### [done] [2026-03-24] meta-layer-refactor
### [done] [2026-03-26] procedural-rewrite (5 rules, 1 output style, 8 skills, 9 agents)
### [done] [2026-03-28] code-math-correspondence-audit (170 cross-refs, 3 mismatches in saddle_point_solver.rs)
### [done] [2026-03-28] q-error-threshold (subsumed by verify-numerics)
### [done] [2026-03-28] convention-violations (catch_unwind removed, panic comments rewritten)
### [done] [2026-04-03] experiment-reorg (dev-*/exp-* lifecycle split, 3 experiments -> 10 focused units)
### [done] [2026-04-04] math.tex content audit (13 files audited, 9 stubbed, 4 trimmed, -1142 lines)
### [done] [2026-04-06] collaboration-skill (superseded by orchestration-pattern-test)
### [done] [2026-04-03] gradient-search, sensitivity-analysis, large-scale-descent (deleted, superseded)
### [done] [2026-04-04] boundary-crossing-search (split into gradient-ascent-general + gradient-ascent-products)

## [future] Ideas

Items not tied to a specific research question. See also `crates/dev-algorithm-comparison/ablation/ideas-future.md`.

### [future] Symplectic classification of simplices
- Source: Fickel (supervised by Cieliebak). Two 2n-simplices equivalent under affine symplectomorphism iff 2D subsimplices have identical symplectic areas.
- Could enable equivalence-class reduction for sweeps.
- Deprioritized (2026-03-13): for random sampling, equivalence classes and raw polytopes give similar diversity.

### [future] Gradient ascent with variable facet count (F to F+1)
- Add barely non-redundant a_{F+1}, then fixed-F gradient ascent on F+1.
- Warm-start via sigma-list reuse rejected (complexity not worth it).
- Partially explored: `exp-sys-landscape/variable-f-ascent/`, `exp-hko-local-maximum/cut-and-ascent/`.

### [future] Dimension scaling study
- How does max-achievable-sys scale with F for random polytopes? Scattered data exists, no systematic study.

### [future] dev-gradient-ascent scaffolds (step-calibration, strategy-comparison)
- Scaffolded, not implemented. `crates/dev-gradient-ascent/`
