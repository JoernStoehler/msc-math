# Project Tracker

Unified tracker for thesis, experiments, and infrastructure.
Format: `## [status] Group` / `### [status] [date] Item`. Body only when header isn't enough.
Run `bash scripts/tasks-toc.sh` for section index.

**What the thesis should say:** `RESULTS.md`.
**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.
**Maintenance:** Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts) — run the command instead.
**Dependencies:** thesis/ is stale and will be restructured — most thesis work is blocked on restructuring decisions. Work on `library/`, `formal/`, and `experiments/` is independent and can proceed now.

## Schedule — 18 days to deadline (2026-04-30)

Rough shape of Jörn's plan as of 2026-04-12. Ordering is by hard dependencies, not priority. Tags in parentheses are `[Jörn]` / `[agent]` / `[both]`.

**Binding constraint**: Jörn's planning bandwidth, not agent wall-clock. Agents run fast enough that most task sequences complete within a day; the 18-day span exists because Jörn has to stop and re-plan between sequences. Bundles that minimize Jörn's re-planning cost dominate bundles that maximize agent parallelism.

### Today + tomorrow (2026-04-12 Sun + 2026-04-13 Mon)
- **Tube algorithm write-up** (Jörn): first sketch a correct rotation formula + proof → unblocks tube benchmark work. Agents can't do useful tube work before this.
- **Research results strengthening** (both): more experiment ideas, empirical falsification of conclusions, cleaner evidence presentations, extra evidence types even for high-confidence conclusions.
- **Library documentation/architecture shape decision** (Jörn + agent): pick form (colocated README vs architecture.md vs beefed-up file headers vs just math.tex). Agent runs a docs audit to test whether existing state already covers it.
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
- `job.sh` — `#SBATCH` headers, LICCA paths, production N. Jörn scps and submits; the slurm skill (`.agents/skills/slurm/`) has the template + resource-table requirement.

Artifacts per experiment: `job.sh` + `job-smoke.sh` + `data/smoke.jsonl` (locally-regenerable, committed) + `data/licca.jsonl` (produced on LICCA, scp'd back, committed via git LFS). Analyze.py reads whichever is newer / whichever is specified.

## [open] HKO2024 local maximality

Main conjecture: HKO2024 is a local maximum of the systolic ratio. Potentially publishable alongside thesis.
Key files: `experiments/hko-local-maximum/`, `thesis/handwritten-notes.md`.
Literature: BBLM2023 classifies smooth local maximizers (only ball for k=1). Polytope case genuinely open.
HKO2024 lives in multiple ambient spaces (LP(5,5), LP(6,5), F=10, F=13, convex bodies) — local max in one space != local max in a larger space.

### [done] [2026-04] 1a. First-order analysis in a_i space (gradient-analysis)
- Rank 25 in R^40, 15 flat directions. LP confirms 0 in conv(150 per-orbit gradients).
- `experiments/hko-local-maximum/gradient-analysis/logbook.md`

### [done] [2026-04] 1e. Second-order analysis along flat directions
- All 15 basis + 100 random curvatures negative (-0.31 to -0.02). Supports local maximality.
- `experiments/hko-local-maximum/second-order/`

### [done] [2026-03] 1b. Facet-splitting (F=10 to F=11)
- 536 cuts, all decrease sys.
- `experiments/hko-local-maximum/facet-splitting/`

### [done] [2026-03] 1b. Cut-and-ascent (cut then gradient ascent)
- 0/20 trials improved over HKO2024.
- `experiments/hko-local-maximum/cut-and-ascent/`

### [done] [2026-03] 1c. Subdifferential LP in (n,h)-space
- Phase C LP confirms 0 in conv(per-orbit gradients). Superseded by 1a (a_i space, no gauge).
- `experiments/hko-local-maximum/subdifferential-lp/`

### [done] [2026-03] 1d. Lagrangian boundary mapping
- Characteristic radius ~0.035, anisotropic (7x aspect ratio), ~10^-31 volume fraction.
- `experiments/hko-local-maximum/lagrangian-boundary/logbook.md`

### [done] [2026-03] Perturbation neighborhood (LP(5,5) random perturbations)
- 100 random perturbations all retain sys>1 (min 1.002, max 1.033). HKO highest.
- `experiments/hko-local-maximum/perturbation-neighborhood/`

### [active] [group:licca] LICCA-scale F=10 neighborhood falsification
- Scale the 100-seed perturbation-neighborhood experiment to 10k+ perturbations with 3 step-size buckets (small/medium/large). Honest falsification attempt. Expected: no sys>HKO (strengthens conjecture). Real outcome: whatever the data says.
- **Worktree pointer:** `licca-bundle @ 786de68c`. Contains phase 4 (A→B refactor: rayon `par_iter` + shared helpers in `exp-sys-landscape/src/lib.rs`) + phase 4.5 (crash-safe trace-first write + deterministic `finalize_ascent_output`) + V8 NIT / V6 finding polish. Third-party reviewer ("V8") returned READY for phases 4 and 4.5.
- **Best guess as of 2026-04-12 (not binding — verify before trusting):** the worktree is probably a net-positive starting point. The refactor is the main reproduction cost and has a READY verdict; rebuilding might be better if you find structural problems the V8 reviewer missed, but our rough estimate is that auditing + fixing the known issues below is cheaper than rebuilding from `main`.
- **Known issues found in spot-checks 2026-04-12 (verify still apply before acting):**
  1. All 3 `job.sh` files use `./target/release/<bin>` but set `CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target`, which looks like it makes the binary path wrong. Likely fix: `"$CARGO_TARGET_DIR/release/<bin>"`. Spot-checked on `gradient-ascent-general/job.sh`; not verified on the other two.
  2. No `cargo build` step in any `job.sh`. Unclear whether Jörn prebuilds on LICCA or this is an oversight.
  3. `--time=00:00:01` tripwire with CLI-override ritual. Open question whether to keep or bake in real values; smoke n=3 gives a rough ~3h for `sys-*`, ~30m for perturbation, but that's a thin sample.
  4. `logs/` directory created inside the script but SLURM opens its log file at submit time — possible timing bug, unverified on actual LICCA submission.
- **Ownership (Jörn override 2026-04-12):** the prior "Owned by licca-bundle agent" / "Post-LICCA follow-up unowned" split is superseded — one session now owns end-to-end (audit → fix → smoke → present scp+sbatch → wait → `analyze.py` → figures → logbook → `RESULTS.md` updates → `/pre-merge` → merge). Previous split was producing failed transfers.
- Re-plan trigger: after LICCA runs return, re-evaluate density/falsification claims (`TASKS.md:44`).

### [Jörn] [group:hko] Verify h-space proof
- Danskin + symmetry + Euler homogeneity argument. ~15 min.
- `experiments/hko-local-maximum/gradient-analysis/logbook.md` lines 151-156

### [Jörn] [group:hko] Verify second-order math.tex proposition
- Non-smooth second-order sufficiency proof sketch needs rigor check.
- `formal/hko-local-maximum/second-order.tex`

### [future] [group:hko] Higher-F perturbation validation (F=10→12, F=10→13)
- RESULTS.md claims empirical validation "up to 13-facet" — currently aspirational, only 11-facet done
- Extends facet-splitting and cut-and-ascent to add 2-3 facets simultaneously
- Suggested in `experiments/hko-local-maximum/cut-and-ascent/logbook.md` line 40

### [future] [group:hko] F-refinement convergence (increasing F as smooth approximation)

### [future] [group:hko] Convex-body direction (Minkowski smoothing K + eps*B^4)
- Needs scoping: can billiard algorithm handle non-polyhedral bodies?

### [future] [group:hko] Structural explanation for 0 in conv
- Why does pentagon geometry force this? Golden ratio, order-10 symmetry.

## [open] Novel sys>1 polytopes

Stronger conjecture: HKO2024 may be (up to perturbation/symplectomorphism) the only sys>1 case.

### [done] [2026-04] 2a. Rotated regular products
- Only 5x5 at theta=18deg achieves sys>1 among 3<=n,m<=6 (6deg resolution) + 7x7 separately. 7x7 peaks at 0.917. Mixed 7-pairs not tested.
- `experiments/sys-landscape/rotated-regular-products/`

### [done] [2026-04] 2b. Gradient ascent from random starts
- General: 10 seeds, best sys=0.9005. Products: 12 seeds, best sys=0.9127. No sys>1.
- `experiments/sys-landscape/gradient-ascent-general/`, `experiments/sys-landscape/gradient-ascent-products/`

### [done] [2026-03] 2c. Perturbation neighborhood (structurally different sys>1?)
- 100 random perturbations all retain sys>1 but none exceed HKO2024.
- `experiments/hko-local-maximum/perturbation-neighborhood/`

### [done] [2026-04] 2d. Variable-F ascent (F to F+1)
- 90 trials. F=10 local maxima often improve at F=11 but marginal; no sys>1.
- `experiments/sys-landscape/variable-f-ascent/`
- Successor baseline for the next continuation line: `research/sys-landscape/design/witness-search-program.md:67-75`

### [open] [group:witness-search] Witness oracle instrumentation + benchmark bank
- Upgrade exact witness search from "best permutation only" to a reusable local-structure oracle: top-`m` / within-gap returns, incumbent warm starts, near-active witness metadata, runtime diagnostics.
- Bundle the benchmark bank into the same session; do not track it as a separate item.
- Pointer: `research/sys-landscape/design/witness-search-program.md:22-38`

### [future] [group:witness-search] Witness reuse + safe prefilter calibration
- Quantify trust radius for local witness caches and benchmark safe pruning via `U_A(K) < 1`.
- Compare minimizer-only, top-`m`, within-gap, parent-cache, and hybrid witness sets.
- Fold permutation-neighborhood search and warm-start benchmarking into this line, not separate tracker headers.
- Pointer: `research/sys-landscape/design/witness-search-program.md:40-53`

### [future] [group:witness-search] Reduced-model ascent on witness sets
- Soft-min / log-sum-exp reduced-model ascent first; min-norm convex-hull QP second if the first pass is promising.
- Acceptance criterion: compare against exact-evaluate-every-step on the same seeds; report best exact `sys`, exact-call count, and wall-clock.
- Pointer: `research/sys-landscape/design/witness-search-program.md:55-65`

### [future] [group:witness-search] Witness-guided F→F+1 continuation
- Replace random facet addition with witness-guided vertex splitting and witness lifting into the child problem.
- Compare directly against `variable-f-ascent/` and `exp-hko-local-maximum/cut-and-ascent/`.
- Pointer: `research/sys-landscape/design/witness-search-program.md:67-75`

### [future] [group:witness-search] Symmetry-family search
- Search low-dimensional orbit-union families instead of only generic iid proposals.
- Use the reuse, prefilter, and reduced-model machinery inside those families.
- Keep combinatorial/order-type diagnostics as supporting logging inside this line.
- Pointer: `research/sys-landscape/design/witness-search-program.md:77-83`

### [future] [group:witness-search] Box-pruning on structured families
- Downstream of the symmetry-family line: use witness upper bounds to prune parameter boxes once a productive family exists.
- Pointer: `research/sys-landscape/design/witness-search-program.md:77-83`

### [done] [2026-03] Random sampling (general + products + calibration)
- Random polytopes max sys=0.578. Random products max sys=0.794 (6x6).
- `experiments/sys-landscape/random-sample/`, `experiments/sys-landscape/random-product-sample/`, `experiments/sys-landscape/rejection-calibration/`

### [future] Regular Lagrangian product formula fitting
- Dense (n, m, theta) sweep. Fit sys(n, m, theta). Does formula predict sys>1 only for 5x5?
- Partial data in `experiments/sys-landscape/rotated-regular-products/`

### [active] [group:licca] LICCA-scale massive ascent sampling (density probe)
- Scale `gradient-ascent-general/` (10 → 10k+ seeds) and `gradient-ascent-products/` (12 → 10k+ seeds).
- **Research question (load-bearing for RESULTS.md main-result-1 at `RESULTS.md:9–10`):** does the density of sys>1 local maxima in M_F actually support "no new examples"? Current seed counts are too small to claim the density is low.
- **Worktree pointer:** same as the F=10 item above — `licca-bundle @ 786de68c`. The refactor covers both `sys-*` binaries (V8 READY for phases 4+4.5 + polish).
- **Best guess + known issues + ownership override:** same as the F=10 item above. Same 4 `job.sh` bugs apply to `sys-gradient-ascent-general` and `sys-gradient-ascent-products`. Verify before acting. Rebuild is still on the table if the worktree turns out to be in worse shape than the F=10 entry's spot-checks suggest.
- Each family produces histogram + bucket counts at sys>0.95/0.99/1.00.
- Re-plan trigger: results back → update `RESULTS.md` density claim.

### [future] [group:licca] Combinatoric-changing step sizes on LICCA
- Beyond fixed-F ascent — let random walks flip facet combinatorics mid-trajectory.
- Deprioritized until fixed-F LICCA sampling returns; if fixed-F finds nothing, this is the natural next step.

### [future] Analytical formula for sys(P_5 x R(theta) P_5)
- Kai asked for this. Requires by-hand orbit analysis guided by empirical data.

## [open] sys landscape structure

sys as a continuous function on polytope space, no privileged threshold.

### [done] [2026-03] 3a. Omega hypothesis (small symplectic area -> high sys?)
- Falsified. Zero correlation (rho=-0.02).
- `experiments/combinatorial-cells/omega-hypothesis/`

### [done] [2026-03] 3b. Combinatorial boundary behavior
- Random cells convex, product cells non-convex (0% vs 100% transition failures).
- ~F boundaries per gradient step. Orbit facets 2x wider than non-orbit.
- sys continuous, gradient jumps up to 70deg at orbit switches (3%/boundary).
- `experiments/combinatorial-cells/` (cell-widths, boundary-characterization, gradient-discontinuity, convexity, multiple-crossings)

### [done] [2026-03] 3c. sys distribution for random polytopes
- No random polytope exceeded sys=0.80.
- `experiments/sys-landscape/random-sample/`, `experiments/sys-landscape/random-product-sample/`

### [done] [2026-03] 3d. Gradient validation
- Per-orbit gradient validated (slope=2.00) across 12 polytope types.
- Direction-filtered subdifferential is a negative result.
- `experiments/numerics/gradient/` (`numerics/`, `numerics-edge-cases/`, `numerics-subdifferential/`)

### [future] Systematic landscape analysis
- Gradient flow convergence, local maxima below sys=1, random noise effects.
- Partial data in gradient-ascent experiments.
- Witness-search successor line: `research/sys-landscape/design/witness-search-program.md:55-83`

## [open] Computing capacity

Instrument development. Results promote to `library/`.

### [done] [2026-03] 4a. Algorithm comparison (ablation, benchmark, profiling)
- A2 pruning: ~1078x speedup at F=10. Construction dominates for F<=10 (80-92%).
- `experiments/verification/algorithm-comparison/`

### [done] [2026-03] 4c. Capacity axiom validation
- All 6 axioms pass. 112/112 orbit-recovery polytopes pass.
- `experiments/verification/correctness/`

### [done] [2026-03] 4b-partial. Q error and KKT inertia
- 1.13M nodes, worst E=2.9e-11. Empirically exact at f64.
- Eigenvalue inertia formula holds for 6/7 polytopes, 5 mismatches are threshold artifacts.
- `experiments/numerics/q-error/`, `experiments/numerics/kkt-inertia/`

### [open] [group:numerics] 4b. Numerical error bounds (verify-numerics)
- math.tex Parts I+II complete. Proven Q error bound, eta bound for well-conditioned problems.
- 14 previously-failing tests now pass (329 pass, 0 fail).
- Rationale for current state: degenerate orbits are never capacity-achieving, so final capacity comes from well-conditioned orbits with proven low error. Gap remains for publication.
- Open: Part III (f64 algorithm description), eta bound for LP null-space search (39 violations on natural data with near-zero eigenvalues), GAP in cor:taylor-structure proof (needs Jörn).
- `experiments/numerics/error-bounds/`, `experiments/numerics/error-bounds/algorithm-notes.md`

### [open] [group:numerics] Projection solver
- 5-step algorithm: (1) solve equality constraints → (m-5)-dim affine space, (2) project H → reduced Hessian, (3) eigendecompose → null directions, (4) beta>0 as LP on projected null space, (5) recover multipliers.
- Basic implementation in `kkt/projection_solver.rs`. Needs mathematical rigor + ablation comparison.
- `experiments/numerics/error-bounds/algorithm-notes.md`

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
- `experiments/crosspolytope/`

### [future] Crosspolytope optimality proof
- Minimizing orbit has clean structure (uniform beta, max omega). Symmetry argument may avoid exhaustive enumeration.

### [blocked] [group:tube] Tube vs HK2017 benchmark
- Blocked on: Jörn writing down the tube algorithm + rotation formula + correctness proof (first task in the tube subtree).
- Once formula + proof exist: one session wires the formula into `library/src/algorithms/tube/mod.rs` (the current file is a misleadingly-named wrong placeholder), then a sibling session runs a benchmark harness that reads the polytope database, runs tube + HK2017 per entry, compares c_EHZ values + wallclock + memory, produces a report.
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
- `thesis/tube-algorithm-notes.md`

### [Jörn] [group:numerics] appendix-numerical.tex (5 TODOs)
- Continuity of c_EHZ on polytopes, simplicity assumption, billiard pruning, three-valued verdict, unverified numerical statement.

### [blocked] [group:tube] Tube rotation formula implementation
- Current code is a misleadingly named placeholder that is wrong (not CH2021, not any correct formula).
- Need to implement a correct rotation formula. Not necessarily CH2021 — we have different basis vectors.
- Performance on F>10 polytopes untested.
- Blocked on: Jörn reviewing the math (proofs for what formula is correct given our basis).
- `library/src/algorithms/tube/mod.rs`

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
- Scaffold note: `scratch/math-writeup-scaffold.md` (778 lines, grep-verified).
- Counts: 69 unverified blocks (unchanged from 2026-04-07), 41 `TODO: JÖRN` markers, 10 GAP markers, 100 theorem-like environments across 18 files.
- Top 4 ranked hard-labor items: `prop:capacity-piecewise-smooth`, `lem:cap-derivative`+`lem:vol-derivative`, `prop:prefilter-bound`, `prop:capacity-symplectic-product` (library GAP).
- Ahead of 2026-04-13 schedule. Kai briefing item (below) can consume it once LICCA Sunday preliminaries are back.

### [open] [group:writeup] Kai meeting prep briefing (Tuesday 2026-04-14)
- Terse `.md` with checkpoints Jörn reads ~10 min before the Kai call and drives the meeting from. Not a prose doc for Kai — Jörn drives verbally.
- Organized by decision: locked / empirically strong but unproven / genuinely open / options for closing each gap / recommended priority.
- Synthesizes RESULTS.md + TASKS.md + logbooks + math write-up scaffold + any LICCA Sunday preliminaries.
- Output: `kai-briefing-2026-04-14.md` at a migrated repo path outside the deleted documentation root.
- Scheduled: 2026-04-13 Mon, after the math write-up scaffold lands.

### [done] [2026-04-12] Thesis figure consistency check
- Thesis figure audit: 0 `\includegraphics` refs across 16 `thesis/**/*.tex` files, `thesis/assets/` does not exist. Rerun after experiment writeups and thesis restructuring land.
- Note: before the first figure lands, Jörn picks an asset-provenance convention (sidecar `.source` files / manifest / sync script) and adds it to `AGENTS.md`. See "Hand-drawn figures" + "Figure inventory" below.

### [future] [group:figures] Hand-drawn figures (Jörn)
- Concept illustrations: polar of 2D polytope; 0/1/2/3-facets; fake 3D polytope with Reeb vectors and closed/open trajectories; decompositions of `gamma: R → R^4` drawn as `gamma: R → R`; etc.
- Agent scope: figure inventory only (list, not produce), and only after narrative structure stabilizes so we know which concepts get illustrations and where.

### [future] [group:figures] 4D → R^3 projection viz tool
- RESULTS.md minor deliverable ("a visualization of the 4d geometry on a computer screen").
- Agent scaffolds the tool (reads a polytope + Reeb orbit data, outputs rendered view); Jörn drives design decisions (interactive vs static, rendering backend, which projection).
- Not urgent — post-Kai, after thesis narrative structure stabilizes enough to know what needs illustrating.

### [future] [group:figures] Figure inventory
- Compile per-chapter list of figures with provenance tags (hand-drawn / code-generated / viz-tool / whiteboard photo).
- Blocked on: Jörn picking which concepts get illustrations and where they go. After those calls exist, agent scope is mechanical inventory.

### [open] [group:submission] Final assembly (checklist-driven, ≤2026-04-30)
- Abstract, bibliography check (includes verifying agent-produced bib entry at `thesis/bibliography.bib` line 151), figure quality review, proofreading, print formatting.
- Pre-submission slice (agent-doable): repo freeze + tag, build reproducibility verification, bibliography citation-to-key integrity, figure presence check, university format compliance if documented anywhere.
- Submission slice (Jörn): physical print, university portal upload (credentials), handin signatures. **No defense.**
- After all content is stable.

## [open] Code quality + alignment

### [done] [2026-04-07] Code cleanup (session)
- Completed: step_bound duplication extracted to exp-sys-landscape/src/lib.rs (cut-and-ascent gets inline copy); products-vs-random split; wiggle strength justified + documented; `[lem:dual-vertex-qp]` proof drafted + Jörn-approved (Lemma 37 in `formal/main.pdf` p11); math.tex stubs audited (53 stubs + 69 unverified blocks as of 2026-04-07).
- Remaining future note: gradient-ascent + multiple-crossings overlap dedup is blocked until gradient ascent stabilizes into library. Until then, copy-edit between experiments is correct.
- Known side effect: step_bound upgrade (omega_0 detection) changes experiment behavior if re-run. Existing JSONL not regenerated.

### [open] [group:paranoia] Paranoia: numerical claims (first pass merged 2026-04-12)
- First pass merged: `paranoia-numerics` branch, 19 files fixed across experiment logbooks + `experiments/numerics/error-bounds/tests.rs` + `experiments/numerics/unknown-predicates/main.rs`. Session report at `paranoia-numerics-report.md`.
- Remaining sub-items (needs Jörn decision, then follow-up to agent):
  - `experiments/verification/orbit-recovery/`: 4 polytopes missing from dataset (112→108), `solution_dim` hardcoded to 0 in `main.rs`, error magnitudes from different algorithm version
  - `experiments/verification/algorithm-comparison/profiling/`: per-test durations zeroed in JSONL, 3 historical runs absent — data pipeline broken

### [done] [2026-04-12] [group:paranoia] Paranoia: conjectures + interpretations
- Flag-only audit merged: 42 ranked flags (belief 5 / causal 11 / unhedged 12 / interpretation 13 / conjecture 1).
- Top flags cluster in `formal/library/{geom,algorithms}.tex` and unverified lemmas in `formal/verification/algorithm-comparison/ablation.tex` + `formal/numerics/gradient/numerics-subdifferential.tex`.
- Jörn reads async when write-up capacity frees up.

### [open] [group:writeup] Thesis-code alignment
- Full list: `thesis/migration-findings.md`
- Tube rotation increment: current code is a misleadingly named placeholder, not CH2021. Need to implement a correct rotation formula (not necessarily CH2021 — we have different basis vectors).
- KKT notation: decided — use code's symmetric convention (eigenvalue decompositions pop out). Propagate to thesis.
- Accumulator pattern: thesis is stale, will be rewritten. Not a separate issue.
- qp_assembly dual-vertex: decided — thesis should just use a_i (h=1, a=n). The "equivalence" is trivial: substitute h=1, confirm |n|=1 was never used. Per-proof mechanical substitution.
- KktResult->Solution bridge: needs investigation — unclear whether this is actually a thesis-code alignment issue.
- Note: thesis-side propagation (KKT notation, accumulator, qp_assembly) is blocked on thesis restructuring.

### [open] [group:writeup] Dual-vertex parameterization (a_i migration)
- Library API done. Most experiment migration complete. Math.tex migration complete.
- `formal/library/algorithms.tex` uses a_i throughout.
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
- Jörn partially reviewed Defs 1–13 of `formal/library/geom.tex` (`library/src/geom/review-notes.md`).
- Consolidate Defs 1-2 (symplectic form). Add Def for HKO2024 + Thm for false Viterbo's conjecture.
- Clarify H-representation irredundancy. Fix Defs 12-13 (area/volume are algorithms, not definitions).
- Consider splitting into `math_geometry.tex`, `math_symplectic.tex`, `math_reeb.tex`.

### [done] [2026-04-12] [group:docs] Library architecture docs audit
- Library docs audit: existing headers + per-module math.tex cover architecture mostly held; 0 blockers, 7 gaps, 3 nits across `lib.rs`, `kkt/`, `algorithms/` umbrella, `algorithms/tube/`.
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
- Deliverables: `papers/citation-index.md` (verified theorem index), paper download workflow now lives in `.agents/skills/paper-download/SKILL.md`, 4 PDFs in `papers/`.

### [done] [2026-04-07] Delete superseded experiments
- Directories deleted 2026-04-03. Reference cleanup done 2026-04-07: removed gradient-search from gradient-ascent-general/products code comments, rules examples, stale cleanup note.

## [open] Infrastructure + tooling

### [open] Orchestration pattern test
- Testing orchestration / delegation guidance on real thesis tasks.
- Baseline commit `f8044b35`. Dry run confirmed delegated-agent mechanics work.
- Next: post-mortem on the orchestration workflow and whether the Codex-first split changed anything that future sessions should preserve.

### [open] [group:codex-migration] Codex CLI migration
- Port msc-math workflow into native Codex repo paths. Scaffold merged into main; the remaining work is cleanup and verification of the Codex-first state.
- Old `codex-migration` worktree notes are now archival, not the active source of truth.

### [open] [group:repo-layout] Mechanical stale-path cleanup outside onboarding
- Goal: clean stale post-migration references in non-onboarding files; do not change mathematical claims or experiment conclusions.
- Search first:
  - `rg -n 'crates/|docs/|thesis/assets|AGENTS\.new\.rules\.md|scripts/codex-cloud|codex-cloud\.md|review-(rust|python|thesis|claims|figures|proof|formalization)' thesis formal experiments library research TASKS.md RESULTS.md .devcontainer scripts`
  - `rg -n 'logbook\.md|math\.tex' thesis formal experiments library research TASKS.md RESULTS.md`
- Likely first-pass files from 2026-04-15 scan:
  - `thesis/tube-algorithm-notes.md`
  - `thesis/appendix-rewrite-notes.md`
  - `library/src/geom/review-notes.md`
  - `experiments/numerics/error-bounds/algorithm-notes.md`
  - `formal/**/*.tex` headers that still say root `math.tex`, colocated `math.tex`, or missing `logbook.md`
- Preserve provenance comments when they identify copied code history; update only paths that a future agent would treat as live instructions.
- Acceptance: scan has no live stale-path hits outside explicitly historical/provenance wording; changed Markdown/TeX/Rust comments still point to existing files or clearly say the referenced source is historical.

### [done] [2026-04-15] [group:library] Remove HK2017 capacity fixture, keep profiling bench
- Decision: delete `library/tests/fixtures/capacity_dataset.json` and `library/src/algorithms/hk2017/generate_capacity_fixtures.rs`. Broad HK2017 validation belongs in `experiments/verification/correctness/`; library tests keep small live smoke/regression checks for literature values, conformality, symplectic invariance, pruning agreement, and billiard agreement.
- `library/benches/profiling.rs` is still the Criterion source cited by `research/verification/design/algorithm-comparison/benchmark.md` for phase profiling and micro-benchmarks; keep `library/Cargo.toml` bench metadata unchanged.
- Updated `experiments/verification/algorithm-comparison/profiling/analyze.py` and refreshed its generated profiling artifacts so the benchmark design notes no longer point at deleted fixture tests.
- Convention update: `AGENTS.md`, `$rust-conventions`, and `$experiment-conventions` now state the boundary between fast crate tests and slow validation experiments.
- Verification for the branch: `cargo test -p symplectic --release --lib`; `cd library/ && cargo clippy --lib -- -D warnings`; `cargo test -p dev-capacity-validation --bin axioms-correctness --release`; `cargo build --workspace --release`; `uv run analyze.py` in `experiments/verification/algorithm-comparison/profiling/`; `cd thesis/ && latexmk && ./check-build.sh`; `cd formal/ && latexmk`. No bench metadata changed, so `cargo bench -p symplectic --bench profiling --no-run` is not required.

### [done] [2026-04-15] Design session-focus skills
- Goal: replace the TODO scaffolds in `.agents/skills/project-management-focus/SKILL.md` and `.agents/skills/research-focus/SKILL.md` with stable session-scope workflows.
- Scope distinction: focus skills describe what the top-level session keeps active, owns directly, and surfaces to Jörn. They should not duplicate artifact conventions such as Rust, thesis TeX, formal math, Python, or experiment rules.
- `project-management-focus` owns `TASKS.md`, task graph maintenance, PM surfaces for Jörn, decomposition, bundling, ownership, and agent/Jörn division of labor. `research-focus` owns research framing and method/evidence surfaces. `subagent-delegation` replaces the old `.agents/skills/orchestrate/SKILL.md` mechanics.
- Output: ready-to-use skill files committed on 2026-04-15; future changes should come from observed use, not speculative polish.

### [future] Revisit focus and delegation skills after live use
- Trigger: after Jörn has tried `project-management-focus`, `research-focus`, and `subagent-delegation` on real thesis work.
- Ask Jörn whether `research-focus` supported useful research sessions, whether `project-management-focus` kept `TASKS.md` useful without adding maintenance drag, and whether `subagent-delegation` kept top-level verification explicit.
- Expected update: small wording fixes if the workflows mostly worked; larger refactor only for observed failure modes from real sessions.

### [done] [2026-04-12] variable-f-ascent merge (closed as stale)
- Experiment 2d results are on main per the `[done] [2026-04]` entry in "Novel sys>1 polytopes". No `variable-f-ascent` branch or worktree exists at 2026-04-12.
- The logbook-reframing sub-task (RQ2 Path D should read as a sanity check, not a positive finding) is a small copy-edit that can happen as part of experiment writeup or ad hoc; not worth tracking as its own item in the 18-day push.

### [done] [2026-04-07] Database cleanup
- KKT panics → TypeCViolation/ConstraintViolation variants with eprintln warnings. catch_unwind removed from 6 files. Source populated for 170 DB records. Data regenerated for 4 combinatorial-cells experiments.
- Known TODOs left for future solver refactor:
  - Callers use `if let Feasible(...)`, silently skipping TypeCViolation/ConstraintViolation same as Infeasible. Revisit when solver math matures and variants carry different error bound semantics.
  - `Source::LagrangianProduct` uses 0.0 for circumradius/rotation on random products (n1/n2 are correct). No code uses these fields for reconstruction; fix when Source enum is extended.

### [done] [2026-04-12] Worktree audit
- Worktree audit: one non-main worktree, `paranoia-numerics` — `unmerged-wip`, 5 ahead / 37 behind main, 19 files, matches `[active]` Paranoia numerics session. Jörn to decide rebase-and-merge vs continue-accumulating.

### [done] [2026-04-12] Stale branch cleanup
- Branch audit: 5 branches fully merged with 0-file diffs (`citation-verification`, `citation-verification-d`, `database-cleanup`, `delete-api-reference`, `housekeeping-triage`) — safe `git branch -d` candidates. 1 unmerged: `numerical-story-expand` (1 ahead / 9 behind, +458/-160 in `thesis/numerical-story.md`). Jörn decides per-branch.

### [done] [2026-04-12] Delete api-reference/
- Never used organically by agents; agents read source directly. Removed `api-reference/`, the old API extract workspace member, the pre-commit hook, and stale project-doc references.

### [done] [2026-04] Polytope database
- `library/src/database.rs`, JSONL format, 1198 entries. 6 experiments migrated.

### [done] [2026-03] Polytope4D optimization
- 31x speedup at F=10 via integer-scaled arithmetic + f64 prefilters.
- Remaining: Jörn verifies `prop:integer-cramer`, f64 threshold soundness.

### [done] [2026-03] Slurm skill
- `.agents/skills/slurm/SKILL.md`

### [done] [2026-04] Orchestration infrastructure
- Original `/orchestrate` skill, delegation guide, cheatsheet. Live replacement split: `project-management-focus` for task graph and PM surfaces, and `subagent-delegation` for delegation mechanics.

## [done] Historical

19 infrastructure and cleanup tasks completed March-April 2026 (migrations, audits, reorgs, convention fixes). Details in git history.

## [future] Ideas

Items not tied to a specific research question. See also `experiments/verification/algorithm-comparison/ablation/`.

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
- Scaffolded, not implemented. `experiments/sys-landscape/gradient-ascent-dev/`
