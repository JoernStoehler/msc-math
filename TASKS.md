# Project Tracker

Unified tracker for thesis, experiments, and infrastructure.
Format: `## [status] Group` / `### [status] [date] Item`. Body only when header isn't enough.
See `.claude/rules/tasks.md` for full conventions. Run `bash scripts/tasks-toc.sh` for section index.

**What the thesis should say:** `RESULTS.md`.
**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.
**Maintenance:** Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts) — run the command instead.
**Dependencies:** thesis/ is stale and will be restructured — most thesis work is blocked on restructuring decisions. Work on crates/ (code, math.tex, experiments) is independent and can proceed now.

## Schedule — 18 days to deadline (2026-04-30)

Rough shape of Jörn's plan as of 2026-04-12. Ordering is by hard dependencies, not priority. Tags in parentheses are `[Jörn]` / `[agent]` / `[both]`.

**Binding constraint**: Jörn's planning bandwidth, not agent wall-clock. Agents run fast enough that most task sequences complete within a day; the 18-day span exists because Jörn has to stop and re-plan between sequences. Bundles that minimize Jörn's re-planning cost dominate bundles that maximize agent parallelism.

### Today + tomorrow (2026-04-12 Sun + 2026-04-13 Mon)
- **Tube algorithm write-up** (Jörn): first sketch a correct rotation formula + proof → unblocks tube benchmark work. Agents can't do useful tube work before this.
- **Research results strengthening** (both): more experiment ideas, empirical falsification of conclusions, cleaner evidence presentations, extra evidence types even for high-confidence conclusions.
- **Library docs/architecture shape decision** (Jörn + agent): pick form (colocated README vs architecture.md vs beefed-up file headers vs just math.tex). Agent runs a docs audit to test whether existing state already covers it.
- **LICCA Sunday compute window** (agent): massive ascent sampling + HKO2024 neighborhood falsification as large-N experiments.

### Tuesday 2026-04-14 — Kai meeting (hard gate)
- Monday agent prep: **math write-up scaffold** (theorem dependency graph, stub inventory refresh, hard-labor list) + **Kai briefing** (terse checkpoints for Jörn to drive the meeting from).
- Tuesday: Kai meeting decides "how to finish/wrap up the thesis". **Re-plan thesis completion scope after.**

### Eventually (Wed 2026-04-15 → mid-Apr)
- **Full math write-up pass** (Jörn-driven two-phase: high-level notes → paragraph structure + flag hard labor). Agents prep scaffolds, run empirical precursors for hard-labor items when possible.
- **Hand-drawn figures** (Jörn: polar of 2D polytope, 0/1/2/3-facets, fake 3D polytope with Reeb vectors, gamma: R→R^4 decompositions, ...).
- **4D→R³ projection viz tool** (agent: matches RESULTS.md minor deliverable).

### Post-draft stability (late Apr)
- **SWE polish** (agent): dev/exp stable code → library promotion, test completion + perf, documentation gaps, simplifications. Only after thesis draft is stable so experiment numbers aren't moving mid-write-up.
- **Figure inventory + QA** (agent after Jörn picks figures per chapter).

### Finally (by 2026-04-30)
- Print + upload + handin thesis + repo at university portal. **No defense.**
- Pre-submission checklist (agent): repo freeze + tag, build reproducibility, bibliography sanity, format compliance.

### Re-plan triggers
- **After Tuesday Kai meeting** → re-scope thesis completion, decide must-haves vs cuts
- **After tube rotation formula + proof land** → wire up tube benchmark harness + run cross-compare against HK2017
- **After LICCA Sunday runs return** → re-evaluate density / falsification claims
- **After math write-up scaffold lands** → Jörn breaks hard-labor items into agent-doable sub-tasks

### Conventions for LICCA experiments
Same binary runs locally and on LICCA. The binary takes explicit CLI args (`--n`, `--out`, ...); no `--mode` flag, no Rust-side `if licca {}` branching — all configuration lives in the shell scripts.

Each LICCA-bound experiment ships two scripts in its directory:
- `job-smoke.sh` — plain bash, no `#SBATCH`. Small N. Output path under the experiment dir (e.g. `data/smoke.jsonl`). Runs in this devcontainer. Agents run this as their own verification step before handing off.
- `job.sh` — `#SBATCH` headers, LICCA paths, production N. Jörn scps and submits; the slurm skill (`.claude/skills/slurm/`) has the template + resource-table requirement.

Artifacts per experiment: `job.sh` + `job-smoke.sh` + `data/smoke.jsonl` (locally-regenerable, committed) + `data/licca.jsonl` (produced on LICCA, scp'd back, committed via git LFS). Analyze.py reads whichever is newer / whichever is specified.

## [open] HKO2024 local maximality

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

### [active] [group:licca] LICCA-scale F=10 neighborhood falsification
- Scale the 100-seed perturbation-neighborhood experiment to 10k+ perturbations with multiple step-size buckets (small/medium/large). Honest falsification attempt.
- In-flight on `.claude/worktrees/licca-bundle` branch `licca-bundle @ e741dc1a` (commit `perturbation-neighborhood: refactor for LICCA (CLI + 3 eps buckets)`). Refactor is in place on the original experiment dir (Jörn-approved, overriding the earlier sibling-dir plan).
- Pipeline remaining for the owning session:
  1. Confirm smoke tests pass locally (`job-smoke.sh`, output under `data/smoke-eps-{0.001,0.01,0.1}.jsonl`) — the worktree already has smoke artifacts committed in the refactor commit.
  2. Reviewer subagent set up via `REVIEWER_PROMPT.md` on the worktree — wait for / collect its output, address FATAL/SIMPLIFY findings.
  3. Prepare `job.sh` for Jörn's scp + slurm submission. Slurm skill: `.claude/skills/slurm/`.
  4. After Jörn runs on LICCA and `data/licca.jsonl` returns (git LFS): run `analyze.py`, update `perturbation-neighborhood/logbook.md`, update RESULTS.md density/falsification claims, mark re-plan trigger at `TASKS.md:44` ("After LICCA Sunday runs return → re-evaluate density / falsification claims").
  5. Pre-merge check + merge to main.
- Bundled with the sibling LICCA ascent-sampling item below; both ship out of the same `licca-bundle` worktree.
- Expected outcome: no sys>HKO (strengthens conjecture). Real outcome: whatever the data says.

### [Jörn] [group:hko] Verify h-space proof
- Danskin + symmetry + Euler homogeneity argument. ~15 min.
- `crates/exp-hko-local-maximum/gradient-analysis/logbook.md` lines 151-156

### [Jörn] [group:hko] Verify second-order math.tex proposition
- Non-smooth second-order sufficiency proof sketch needs rigor check.
- `crates/exp-hko-local-maximum/second-order/math.tex`

### [future] [group:hko] Higher-F perturbation validation (F=10→12, F=10→13)
- RESULTS.md claims empirical validation "up to 13-facet" — currently aspirational, only 11-facet done
- Extends facet-splitting and cut-and-ascent to add 2-3 facets simultaneously
- Suggested in `crates/exp-hko-local-maximum/cut-and-ascent/logbook.md` line 40

### [future] [group:hko] F-refinement convergence (increasing F as smooth approximation)

### [future] [group:hko] Convex-body direction (Minkowski smoothing K + eps*B^4)
- Needs scoping: can billiard algorithm handle non-polyhedral bodies?

### [future] [group:hko] Structural explanation for 0 in conv
- Why does pentagon geometry force this? Golden ratio, order-10 symmetry.

## [open] Novel sys>1 polytopes

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

### [active] [group:licca] LICCA-scale massive ascent sampling (density probe)
- Scale `gradient-ascent-general/` (10 seeds → 10k+) and `gradient-ascent-products/` (12 seeds → 10k+) on LICCA.
- **Research question**: does the density of sys>1 local maxima in M_F actually support "no new examples"? Current seed counts are too small to claim the density is low.
- In-flight on `.claude/worktrees/licca-bundle` branch `licca-bundle @ e741dc1a` (commit `ascent-{general,products}: refactor for LICCA (CLI + per-seed RNG + no-db-update)`). In-place refactor on the original experiment dirs (Jörn-approved).
- Follow the pipeline in the F=10 item above (smoke → reviewer → job.sh → scp → analyze → merge). This item shares the same worktree and the same reviewer subagent.
- Each family produces histogram + bucket counts at sys>0.95/0.99/1.00.
- Re-plan trigger: results back → update RESULTS.md density claim, evaluate whether any further sampling is worthwhile.

### [future] [group:licca] Combinatoric-changing step sizes on LICCA
- Beyond fixed-F ascent — let random walks flip facet combinatorics mid-trajectory.
- Deprioritized until fixed-F LICCA sampling returns; if fixed-F finds nothing, this is the natural next step.

### [future] Analytical formula for sys(P_5 x R(theta) P_5)
- Kai asked for this. Requires by-hand orbit analysis guided by empirical data.

## [open] sys landscape structure

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

## [open] Computing capacity

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

### [open] [group:numerics] 4b. Numerical error bounds (verify-numerics)
- math.tex Parts I+II complete. Proven Q error bound, eta bound for well-conditioned problems.
- 14 previously-failing tests now pass (329 pass, 0 fail).
- Rationale for current state: degenerate orbits are never capacity-achieving, so final capacity comes from well-conditioned orbits with proven low error. Gap remains for publication.
- Open: Part III (f64 algorithm description), eta bound for LP null-space search (39 violations on natural data with near-zero eigenvalues), GAP in cor:taylor-structure proof (needs Jörn).
- `crates/dev-numerical-analysis/error-bounds/`, `handoffs/verify-numerics-algorithm.md`

### [open] [group:numerics] Projection solver
- 5-step algorithm: (1) solve equality constraints → (m-5)-dim affine space, (2) project H → reduced Hessian, (3) eigendecompose → null directions, (4) beta>0 as LP on projected null space, (5) recover multipliers.
- Basic implementation in `kkt/projection_solver.rs`. Needs mathematical rigor + ablation comparison.
- `handoffs/verify-numerics-algorithm.md`

### [open] [group:numerics] Beta-LP unification
- Replace `find_positive_beta_1d`/`find_positive_beta_nd` with single LP: maximize min_j beta_j subject to beta = beta_0 + V*alpha.
- Previous branch deleted (tip `7ca81b53` has salvageable design: unified function, Type A/B/C eigenvector classification).
- Thesis/code tension: thesis proves rank-deficient pairs are redundant (discard); code searches null space for beta>0 on *near*-singular systems (pseudoinverse beta_0 may have beta_i < 0 from noise; null-space shift recovers feasibility without changing Q). Not contradictory but needs explicit documentation.
- Open question for Jörn: is filtering Type A directions mathematically justified?

### [open] [group:numerics] Solver numerical math.tex
- Per-module math.tex for SVD, condition numbers, LU, eigendecomposition stability.
- Multiple modules use SVD without shared error analysis.

### [done] [2026-04] Crosspolytope capacity
- c_EHZ = 4.0 (same as hypercube), sys=0.75. Exhaustive search through m=13.
- `crates/crosspolytope/`

### [future] Crosspolytope optimality proof
- Minimizing orbit has clean structure (uniform beta, max omega). Symmetry argument may avoid exhaustive enumeration.

### [blocked] [group:tube] Tube vs HK2017 benchmark
- Blocked on: Jörn writing down the tube algorithm + rotation formula + correctness proof (first task in the tube subtree).
- Once formula + proof exist: one session wires the formula into `crates/library/src/algorithms/tube/mod.rs` (the current file is a misleadingly-named wrong placeholder), then a sibling session runs a benchmark harness that reads the polytope database, runs tube + HK2017 per entry, compares c_EHZ values + wallclock + memory, produces a report.
- Goal: empirically decide "switch to tube if/where it beats HK2017" and cross-compare excessively for correctness verification.
- Re-plan trigger: Jörn's write-up lands → stage the wire-in and harness work as concrete items.

## [open] Thesis

thesis/ is stale (see `thesis/handwritten-notes.md`). Most work here is blocked on restructuring decisions.
tube-algorithm.tex and appendix-numerical.tex TODOs are about math correctness, independent of restructuring.

### [Jörn] [group:writeup] Thesis restructuring
- Current content stale. Decisions needed: chapter structure, what content survives, what gets rewritten.
- a_i replaces (n,h). Sign conventions changed. Simplification theorem ordering changed.
- Blocks: S0, experiment writeups, experiments chapter, introduction, conclusion.
- See `thesis/handwritten-notes.md` for narrative notes.

### [Jörn] [group:tube] tube-algorithm.tex (8 TODOs)
- 5 Jörn questions (quaternionic formula, TF_ij equivalence, rotation number, closing steps, correctness proof).
- 3 GAP markers (agent-added unverified content).
- `handoffs/tube-algorithm.md`

### [Jörn] [group:numerics] appendix-numerical.tex (5 TODOs)
- Continuity of c_EHZ on polytopes, simplicity assumption, billiard pruning, three-valued verdict, unverified numerical statement.

### [blocked] [group:tube] Tube rotation formula implementation
- Current code is a misleadingly named placeholder that is wrong (not CH2021, not any correct formula).
- Need to implement a correct rotation formula. Not necessarily CH2021 — we have different basis vectors.
- Performance on F>10 polytopes untested.
- Blocked on: Jörn reviewing the math (proofs for what formula is correct given our basis).
- `crates/library/src/algorithms/tube/mod.rs`

### [blocked] [group:writeup] S0: Notation restructure
- Blocked on: thesis restructuring. Only applies to content that survives — new content will use a_i from the start.
- a_i replaces (n,h). Sign convention for Lagrange multipliers changed. Simplification theorem ordering changed.
- Propagate through all thesis .tex files.

### [blocked] [group:writeup] Experiment writeup drafts
- Blocked on: thesis restructuring (need chapter structure and framing decisions).
- Logbooks already contain factual summaries. Agent value-add: thesis-style prose from logbook + data.
- Agent cannot decide framing (how experiment serves thesis argument) — only Jörn can.
- "Just try it" for 1-2 well-defined experiments first (e.g., gradient-analysis, rotated-regular-products). Trash if output is just paraphrased logbook.

### [blocked] [group:writeup] Experiments chapter
- Blocked on: thesis restructuring, experiment writeup quality.
- `thesis/experiments.tex` has 1 TODO.

### [blocked] [group:writeup] Introduction
- `thesis/main.tex` has 1 TODO (write introduction).
- Blocked on: stable chapter content.

### [blocked] [group:writeup] Conclusion
- Blocked on: stable chapter content.

### [done] [2026-04-12] [group:writeup] Math write-up scaffold
- Handoff: `handoffs/math-writeup-scaffold-2026-04-12.md` (778 lines, grep-verified).
- Counts: 69 unverified blocks (unchanged from 2026-04-07), 41 `TODO: JÖRN` markers, 10 GAP markers, 100 theorem-like environments across 18 files.
- Top 4 ranked hard-labor items: `prop:capacity-piecewise-smooth`, `lem:cap-derivative`+`lem:vol-derivative`, `prop:prefilter-bound`, `prop:capacity-symplectic-product` (library GAP).
- Ahead of 2026-04-13 schedule. Kai briefing item (below) can consume it once LICCA Sunday preliminaries are back.

### [open] [group:writeup] Kai meeting prep briefing (Tuesday 2026-04-14)
- Terse `.md` with checkpoints Jörn reads ~10 min before the Kai call and drives the meeting from. Not a prose doc for Kai — Jörn drives verbally.
- Organized by decision: locked / empirically strong but unproven / genuinely open / options for closing each gap / recommended priority.
- Synthesizes RESULTS.md + TASKS.md + logbooks + math write-up scaffold + any LICCA Sunday preliminaries.
- Output: `handoffs/kai-briefing-2026-04-14.md`.
- Scheduled: 2026-04-13 Mon, after the math write-up scaffold lands.

### [done] [2026-04-12] Thesis figure consistency check
- Handoff: `handoffs/thesis-figures-audit-2026-04-12.md`. Degenerate baseline: 0 `\includegraphics` refs across 16 `thesis/**/*.tex` files, `thesis/assets/` does not exist. Rerun after experiment writeups and thesis restructuring land.
- Note: before the first figure lands, Jörn picks an asset-provenance convention (sidecar `.source` files / manifest / sync script) and adds to CLAUDE.md. See "Hand-drawn figures" + "Figure inventory" below.

### [future] [group:figures] Hand-drawn figures (Jörn)
- Concept illustrations: polar of 2D polytope; 0/1/2/3-facets; fake 3D polytope with Reeb vectors and closed/open trajectories; decompositions of `gamma: R → R^4` drawn as `gamma: R → R`; etc.
- Agent role: figure inventory only (list, not produce), and only after narrative structure stabilizes so we know which concepts get illustrations and where.

### [future] [group:figures] 4D → R^3 projection viz tool
- RESULTS.md minor deliverable ("a visualization of the 4d geometry on a computer screen").
- Agent scaffolds the tool (reads a polytope + Reeb orbit data, outputs rendered view); Jörn drives design decisions (interactive vs static, rendering backend, which projection).
- Not urgent — post-Kai, after thesis narrative structure stabilizes enough to know what needs illustrating.

### [future] [group:figures] Figure inventory
- Compile per-chapter list of figures with provenance tags (hand-drawn / code-generated / viz-tool / whiteboard photo).
- Blocked on: Jörn picking which concepts get illustrations and where they go. Agent role is mechanical inventory after those calls exist.

### [open] [group:submission] Final assembly (checklist-driven, ≤2026-04-30)
- Abstract, bibliography check (includes verifying agent-produced bib entry at `thesis/bibliography.bib` line 151), figure quality review, proofreading, print formatting.
- Pre-submission slice (agent-doable): repo freeze + tag, build reproducibility verification, bibliography citation-to-key integrity, figure presence check, university format compliance if documented anywhere.
- Submission slice (Jörn): physical print, university portal upload (credentials), handin signatures. **No defense.**
- After all content is stable.

## [open] Code quality + alignment

### [done] [2026-04-07] Code cleanup (session)
- Completed: step_bound duplication extracted to exp-sys-landscape/src/lib.rs (cut-and-ascent gets inline copy); products-vs-random split; wiggle strength justified + documented; `[lem:dual-vertex-qp]` proof drafted + Jörn-approved (Lemma 37 in `crates/main.pdf` p11); math.tex stubs audited (53 stubs + 69 unverified blocks as of 2026-04-07).
- Remaining future note: gradient-ascent + multiple-crossings overlap dedup is blocked until gradient ascent stabilizes into library. Until then, copy-edit between experiments is correct.
- Known side effect: step_bound upgrade (omega_0 detection) changes experiment behavior if re-run. Existing JSONL not regenerated.

### [open] [group:paranoia] Paranoia: numerical claims (first pass merged 2026-04-12)
- First pass merged: `paranoia-numerics` branch, 19 files fixed across experiment logbooks + `dev-numerical-analysis/error-bounds/tests.rs` + `unknown-predicates/run.rs`. Session report at `paranoia-numerics-report.md`.
- Remaining sub-items (needs Jörn decision, then handoff to agent):
  - `dev-capacity-validation/orbit-recovery/`: 4 polytopes missing from dataset (112→108), `solution_dim` hardcoded to 0 in run.rs, error magnitudes from different algorithm version
  - `dev-algorithm-comparison/profiling/`: per-test durations zeroed in JSONL, 3 historical runs absent — data pipeline broken

### [done] [2026-04-12] [group:paranoia] Paranoia: conjectures + interpretations
- Flag-only audit merged: `handoffs/paranoia-conjectures-2026-04-12.md`. 42 ranked flags (belief 5 / causal 11 / unhedged 12 / interpretation 13 / conjecture 1).
- Top flags cluster in `library/src/{geom,algorithms}/math.tex` and unverified lemmas in `dev-algorithm-comparison/ablation/math.tex` + `dev-gradient/numerics-subdifferential/math.tex`.
- Jörn reads async when write-up capacity frees up.

### [open] [group:writeup] Thesis-code alignment
- Full list: `handoffs/migration-thesis-findings.md`
- Tube rotation increment: current code is a misleadingly named placeholder, not CH2021. Need to implement a correct rotation formula (not necessarily CH2021 — we have different basis vectors).
- KKT notation: decided — use code's symmetric convention (eigenvalue decompositions pop out). Propagate to thesis.
- Accumulator pattern: thesis is stale, will be rewritten. Not a separate issue.
- qp_assembly dual-vertex: decided — thesis should just use a_i (h=1, a=n). The "equivalence" is trivial: substitute h=1, confirm |n|=1 was never used. Per-proof mechanical substitution.
- KktResult->Solution bridge: needs investigation — unclear whether this is actually a thesis-code alignment issue.
- Note: thesis-side propagation (KKT notation, accumulator, qp_assembly) is blocked on thesis restructuring.

### [open] [group:writeup] Dual-vertex parameterization (a_i migration)
- Library API done. Most experiment migration complete. Math.tex migration complete.
- `crates/library/src/algorithms/math.tex` uses a_i throughout.
- Remaining:
  - Jörn verifies `[lem:cap-derivative]` and `[lem:vol-derivative]` (marked `\begin{unverified}`)
  - (`[lem:dual-vertex-qp]` proof was completed under Code cleanup 2026-04; see line 241.)

### [open] [group:docs] math.tex stub / unverified inventory (baseline for write-up scaffold)
- Audited 2026-04-07: 53 explicit stubs + 69 unverified blocks. No proofs lost in migration — stubs were created as stubs, agent-written proofs added later.
- Refresh is Bundle G precursor (see "Math write-up scaffold" in Thesis section); that item also re-categorizes by hard-labor framing and adds theorem dependency graph.
- **High priority** (blocks thesis or code correctness):
  - `lem:cap-derivative` + `lem:vol-derivative` (algorithms/math.tex:709,781) — core gradient lemmas, also tracked in "Dual-vertex parameterization" below.
  - `prop:prefilter-bound` (geom/math.tex:789) — needs restatement in terms of computable `hat_kappa`. Factor-counting issue: tight bound is 5376, not 1344.
  - GAP in `prop:capacity-symplectic-product` (geom/math.tex:157) — `c_EHZ(A) = area(A)` for 2D convex bodies is unverified, dubious citation.
- **Medium priority** (thesis completeness):
  - 10 definition verifications in geom/math.tex (lines 84-325) — routine review.
  - `thm:conformality` + `thm:sympl-invariance` (algorithms/math.tex:107,120) — standard results, need proofs or citations.
  - 3 agent-written proofs needing review: `lem:positive-span`, `lem:vertex-enumeration`, `lem:bounded-triples` (geom/math.tex).
- **Low priority** (dev math, not publication path):
  - 11 stubs + 6 gaps in dev-gradient/ and dev-numerical-analysis/.

### [open] [group:docs] Geom math.tex restructure
- Jörn partially reviewed Defs 1–13 of `crates/library/src/geom/math.tex` (`handoff-geom-math-review.md`).
- Consolidate Defs 1-2 (symplectic form). Add Def for HKO2024 + Thm for false Viterbo's conjecture.
- Clarify H-representation irredundancy. Fix Defs 12-13 (area/volume are algorithms, not definitions).
- Consider splitting into `math_geometry.tex`, `math_symplectic.tex`, `math_reeb.tex`.

### [done] [2026-04-12] [group:docs] Library architecture docs audit
- Handoff: `handoffs/library-docs-audit-2026-04-12.md`. Hypothesis (existing headers + per-module math.tex cover architecture) mostly held: 0 blockers, 7 gaps, 3 nits across `lib.rs`, `kkt/`, `algorithms/` umbrella, `algorithms/tube/`.
- 5 doc-only fixes applied and merged to main (no source/algorithm changes). Notable: `algorithms/mod.rs` tube description rewritten from "(placeholder)" to an explicit wrong-rotation-formula warning; `algorithms/` umbrella gained a "correctness invariant" paragraph (overlapping algorithms must agree).
- Skipped as marginal: `derivatives.rs` cross-directory lemma cite (findable via absolute path), `kkt/mod.rs` formula-location nit, umbrella missing utility math-label cross-refs.

### [future] [group:polish] SWE polish (post-thesis-draft-stability bucket)
- Covers: `dev-*/exp-*` stable code → `library/` promotion, test suite completion + perf, documentation gaps, code simplifications (adopt standard patterns, pull in overlooked libraries, abstract/unabstract as helpful).
- **Do not start during the 18-day push**: rerunning experiments invalidates logbook numbers Jörn is about to write up, and abstraction changes break silent invariants nobody's testing.
- Subsumes (or overlaps with) existing `[open] Projection solver`, `[open] Beta-LP unification`, `[active] Thesis-code alignment` code-side items — those can all be folded into this bucket after Tuesday's Kai meeting clarifies must-ship scope.
- Schedule: dedicated closeout phase after draft is stable enough that experiment numbers aren't moving.

### [done] [2026-04-07] Citation verification pass
- Was 76 `[TODO: JÖRN]` markers total (not 54); 16 were citation lookups.
- 11 citation TODOs fully resolved (theorem numbers filled in, TODO removed).
- 6 partially resolved (wrong chapters corrected, exact theorem numbers added from PDFs).
- Remaining ~57 `[TODO: JÖRN]` markers are mathematical verification (verify statement/proof), not citation lookups.
- Deliverables: `papers/citation-index.md` (verified theorem index), expanded `papers/CLAUDE.md` (download/verify workflow), 4 PDFs in `papers/`.

### [done] [2026-04-07] Delete superseded experiments
- Directories deleted 2026-04-03. Reference cleanup done 2026-04-07: removed gradient-search from code comments (gradient-ascent-general/products run.rs), rules examples, stale handoff.

## [open] Infrastructure + tooling

### [open] Orchestration pattern test
- Testing `/orchestrate` skill + delegation guide on real thesis tasks.
- Baseline commit `f8044b35`. Dry run confirmed Agent() mechanics work.
- Next: post-mortem in `.claude/skills/orchestrate/references/design-space.md`.

### [done] [2026-04-12] variable-f-ascent merge (closed as stale)
- Experiment 2d results are on main per the `[done] [2026-04]` entry in "Novel sys>1 polytopes". No `variable-f-ascent` branch or worktree exists at 2026-04-12.
- The logbook-reframing sub-task (RQ2 Path D should read as a sanity check, not a positive finding) is a small copy-edit that can happen as part of experiment writeup or ad hoc; not worth tracking as its own item in the 18-day push.

### [done] [2026-04-07] Database cleanup
- KKT panics → TypeCViolation/ConstraintViolation variants with eprintln warnings. catch_unwind removed from 6 files. Source populated for 170 DB records. Data regenerated for 4 combinatorial-cells experiments.
- Known TODOs left for future solver refactor:
  - Callers use `if let Feasible(...)`, silently skipping TypeCViolation/ConstraintViolation same as Infeasible. Revisit when solver math matures and variants carry different error bound semantics.
  - `Source::LagrangianProduct` uses 0.0 for circumradius/rotation on random products (n1/n2 are correct). No code uses these fields for reconstruction; fix when Source enum is extended.

### [done] [2026-04-12] Worktree audit
- Handoff: `handoffs/worktree-audit-2026-04-12.md`. One non-main worktree: `paranoia-numerics` — `unmerged-wip`, 5 ahead / 37 behind main, 19 files, matches `[active]` Paranoia numerics session. Jörn to decide rebase-and-merge vs continue-accumulating.

### [done] [2026-04-12] Stale branch cleanup
- Handoff: `handoffs/branch-audit-2026-04-12.md`. 5 branches fully merged with 0-file diffs (`citation-verification`, `citation-verification-d`, `database-cleanup`, `delete-api-reference`, `housekeeping-triage`) — safe `git branch -d` candidates. 1 unmerged: `numerical-story-expand` (1 ahead / 9 behind, +458/-160 in `thesis/numerical-story.md`). Jörn decides per-branch.

### [done] [2026-04-12] Delete api-reference/
- Never used organically by agents; agents read source directly. Removed `api-reference/`, `crates/tools/api-extract/`, pre-commit hook, workspace member, CLAUDE.md references.

### [done] [2026-04] Polytope database
- `crates/database/`, JSONL format, 1198 entries. 6 experiments migrated.

### [done] [2026-03] Polytope4D optimization
- 31x speedup at F=10 via integer-scaled arithmetic + f64 prefilters.
- Remaining: Jörn verifies `prop:integer-cramer`, f64 threshold soundness.

### [done] [2026-03] Slurm skill
- `.claude/skills/slurm/SKILL.md`

### [done] [2026-04] Orchestration infrastructure
- `/orchestrate` skill, delegation guide, cheatsheet.

## [done] Historical

19 infrastructure and cleanup tasks completed March-April 2026 (migrations, audits, reorgs, convention fixes). Details in git history.

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
