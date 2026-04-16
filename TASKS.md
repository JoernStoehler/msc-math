# Project Tracker

Unified tracker for thesis, experiments, and infrastructure.
Format: `## [status] Group` / `### [status] [date] Item`. Body only when header isn't enough.
Run `bash scripts/tasks-toc.sh` for section index.

**What the thesis should say:** `RESULTS.md`.
**How thesis success is measured:** `VERIFICATION.md`.
**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.
**Maintenance:** Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts) — run the command instead.
**Dependencies:** thesis/ is stale and will be restructured — most thesis work is blocked on restructuring decisions. Work on `library/`, `formal/`, and `experiments/` is independent and can proceed now.
**Post-Kai scope (2026-04-15):** Kai agreed on 2026-04-14 that the two main result blocks in `RESULTS.md` are sufficient to finish the thesis project. Extra polish mainly decides whether the project also has publication-grade insights others will want in written form. Jörn wants focused polish and agent-pattern trials, not an obligation to cover every remaining surface. Development work is worth doing through about 2026-04-21; after that, stop expanding and prioritize thesis completion, assembly, and submission.

## Schedule — 18 days to deadline (2026-04-30)

Rough shape of Jörn's plan as of 2026-04-12. Ordering is by hard dependencies, not priority. Tags in parentheses are `[Jörn]` / `[agent]` / `[both]`.

**Binding constraint**: Jörn's planning bandwidth, not agent wall-clock. Agents run fast enough that most task sequences complete within a day; the 18-day span exists because Jörn has to stop and re-plan between sequences. Bundles that minimize Jörn's re-planning cost dominate bundles that maximize agent parallelism.

### Today + tomorrow (2026-04-12 Sun + 2026-04-13 Mon)
- **Tube algorithm write-up** (Jörn): first sketch a correct rotation formula + proof → unblocks tube benchmark work. Agents can't do useful tube work before this.
- **Research results strengthening** (both): more experiment ideas, empirical falsification of conclusions, cleaner evidence presentations, extra evidence types even for high-confidence conclusions.
- **Library documentation/architecture shape decision** (Jörn + agent): pick form (colocated README vs architecture.md vs beefed-up file headers vs just formal library files). Agent runs a docs audit to test whether existing state already covers it.
- **LICCA Sunday compute window** (agent): massive ascent sampling + HKO2024 neighborhood falsification as large-N experiments.

### Tuesday 2026-04-14 — Kai meeting (gate closed)
- Meeting happened. Kai agreed the two main results are sufficient for the thesis; remaining development/polish is optional upside, not required coverage.
- Development cutoff: spend at most until about 2026-04-21 on code/experiment polish or AI-work-pattern trials. After that, only development that directly fixes thesis correctness or submission blockers should continue.
- The planned Kai briefing file was not produced and is now obsolete; preserve the decision above instead of reconstructing the briefing.

### Eventually (Wed 2026-04-15 → mid-Apr)
- **Full math write-up pass** (Jörn-driven two-phase: high-level notes → paragraph structure + flag hard labor). Agents prep scaffolds, run empirical precursors for hard-labor items when possible.
- **Hand-drawn figures** (Jörn: polar of 2D polytope, 0/1/2/3-facets, fake 3D polytope with Reeb vectors, gamma: R→R^4 decompositions, ...).
- **3D visualization figure integration** (agent: existing visualization pipeline + thesis figure/write-up integration; Jörn decides what the figures should show).

### Polish window (2026-04-15 → about 2026-04-21)
- **Agent-attacked polish surface:** evidence hygiene, local validation experiments, code/thesis alignment pruning, and bounded AI-work-pattern trials. Goal is not full coverage; goal is to see which remaining surfaces Codex can close without high Jörn load.
- **Development constraint:** avoid new research branches that need Jörn to interpret before they can be written. Prefer self-verifying outputs: refreshed notes, small validation datasets, figures/tables, and failing/succeeding checks.

### Post-polish stability (after about 2026-04-21)
- **SWE polish becomes freeze-only:** code changes should fix correctness, reproducibility, or submission blockers. Defer broad library promotion, performance refactors, and abstractions unless they unblock thesis text.
- **Figure inventory + QA** (agent after Jörn picks figures per chapter).

### Finally (by 2026-04-30)
- Print + upload + handin thesis + repo at university portal. **No defense.**
- Pre-submission checklist (agent): repo freeze + tag, build reproducibility, bibliography sanity, format compliance.

### Re-plan triggers
- **After tube rotation formula + proof land** → wire up tube benchmark harness + run cross-compare against HK2017
- **After LICCA Sunday runs return** → re-evaluate density / falsification claims
- **After math write-up scaffold lands** → Jörn breaks hard-labor items into agent-doable sub-tasks

### Conventions for LICCA experiments
Same binary runs locally and on LICCA. The binary takes explicit CLI args (`--n`, `--out`, ...); no `--mode` flag, no Rust-side `if licca {}` branching — all configuration lives in the shell scripts.

Each LICCA-bound experiment ships two scripts in its directory:
- `job-smoke.sh` — plain bash, no `#SBATCH`. Small N. Output path under the experiment dir (e.g. `data/smoke.jsonl`). Runs in this devcontainer. Agents run this as their own verification step before handing off.
- `job.sh` — `#SBATCH` headers, LICCA paths, production N. Jörn scps and submits; the slurm skill (`.agents/skills/slurm/`) has the template + resource-table requirement.

Artifacts per experiment: `job.sh` + `job-smoke.sh` + experiment-specific smoke/licca JSONL paths. Current examples: `sys-*` uses `data/smoke.jsonl` and `data/licca.jsonl`; HKO perturbation uses `data/smoke-eps-*.jsonl` and `data/licca-eps-*.jsonl`. Analyze.py reads whichever production/smoke paths that experiment documents.
Before `sbatch`, create the experiment-local `logs/` directory because SLURM opens `#SBATCH --output=logs/%x-%j.out` before the script body runs.

## [open] HKO2024 local maximality

Main conjecture: HKO2024 is a local maximum of the systolic ratio. Potentially publishable alongside thesis.
Key files: `experiments/hko-local-maximum/`, `thesis/handwritten-notes.md`.
Literature: BBLM2023 classifies smooth local maximizers (only ball for k=1). Polytope case genuinely open.
HKO2024 lives in multiple ambient spaces (LP(5,5), LP(6,5), F=10, F=13, convex bodies) — local max in one space != local max in a larger space.

### [done] [2026-04] 1a. First-order analysis in a_i space (gradient-analysis)
- Rank 25 in R^40, 15 flat directions. LP confirms 0 in conv(150 per-orbit gradients).
- `research/hko-local-maximum/design/gradient-analysis.md`

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
- `research/hko-local-maximum/design/subdifferential-lp.md`

### [done] [2026-03] 1d. Lagrangian boundary mapping
- Characteristic radius ~0.035, anisotropic (7x aspect ratio), ~10^-31 volume fraction.
- `research/hko-local-maximum/design/lagrangian-boundary.md`

### [done] [2026-03] Perturbation neighborhood (LP(5,5) random perturbations)
- Historical single-eps artifact `pentagon-perturb.jsonl`: 101 recorded perturbations all retain sys>1 (min 1.0142, max 1.0472). Current smoke/LICCA pipeline is tracked separately below.
- `experiments/hko-local-maximum/perturbation-neighborhood/`

### [active] [group:licca] LICCA-scale F=10 neighborhood falsification
- Scale the 100-seed perturbation-neighborhood experiment to 10k+ perturbations with 3 step-size buckets (small/medium/large). Honest falsification attempt. Expected: no sys>HKO (strengthens conjecture). Real outcome: whatever the data says.
- **Post-Kai priority:** optional publication-grade polish, not required for thesis sufficiency. If LICCA results are back before about 2026-04-21, integrate them; if not, keep the existing local evidence and state the large-scale run as pending/future.
- **Handoff commit:** `fc7991e6` fixed the LICCA script readiness layer from the data-freshness packet; current checkout uses `experiments/...` package paths, not old `exp-*` deployment paths.
- **Script readiness state (2026-04-15):** fixed the known `CARGO_TARGET_DIR` binary-path bug by running `"$CARGO_TARGET_DIR/release/hko-perturbation"`; added `job-smoke.sh`; kept build outside production `job.sh` with an executable preflight error that prints the exact `cargo build` command. Before LICCA submission, Jörn runs `cd ~/msc-math && CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target cargo build --release -p exp-hko-local-maximum --bin hko-perturbation`, then `cd experiments/hko-local-maximum/perturbation-neighborhood && mkdir -p logs && sbatch ... job.sh`.
- **Open LICCA-side check:** confirm that `~/msc-math` on LICCA has the same current repo layout. If it still has an old `~/msc-math/crates/exp-*` deployment copy, update that copy or switch to the current repo layout before submitting.
- **PM caveat:** the data-freshness packet that produced `fc7991e6` shifted productively into LICCA script fixes and smoke checks; it did not produce the full cross-experiment freshness/rerun matrix. See the open data freshness task below before treating all stale-evidence questions as planned.
- **Ownership (Jörn override 2026-04-12):** the prior "Owned by licca-bundle agent" / "Post-LICCA follow-up unowned" split is superseded — one session now owns end-to-end (audit → fix → smoke → present scp+sbatch → wait → `analyze.py` → figures → logbook → `RESULTS.md` updates → `/pre-merge` → merge). Previous split was producing failed transfers.
- Re-plan trigger: after LICCA runs return, re-evaluate density/falsification claims (`TASKS.md:44`).

### [Jörn] [group:hko] Verify h-space proof
- Danskin + symmetry + Euler homogeneity argument. ~15 min.
- `research/hko-local-maximum/design/gradient-analysis.md`

### [Jörn] [group:hko] Verify second-order formal proposition
- Non-smooth second-order sufficiency proof sketch needs rigor check.
- `formal/hko-local-maximum/second-order.tex`

### [future] [group:hko] Higher-F perturbation validation (F=10→12, F=10→13)
- `RESULTS.md` records F=12/F=13 validation as pending/future evidence for the broad HKO2024 local-maximality conjecture; only F=11 checks are currently done.
- Extends facet-splitting and cut-and-ascent to add 2-3 facets simultaneously
- Suggested in `research/hko-local-maximum/design/cut-and-ascent.md`

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
- Post-Kai priority: optional AI-work-pattern / publication-polish trial through about 2026-04-21. Do not treat this as required thesis coverage. Stop if the first session does not produce a reusable oracle surface, benchmark report, or clear negative finding without high Jörn interpretation load.
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
- Compare directly against `experiments/sys-landscape/variable-f-ascent/` and `experiments/hko-local-maximum/cut-and-ascent/`.
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
- **Research question:** does the density of sys>1 local maxima in M_F actually support "no new examples"? Current seed counts are too small for a strong density claim.
- **Post-Kai priority:** optional strengthening of a thesis-sufficient result, not required to finish the thesis. If results are back before about 2026-04-21, integrate them; otherwise weaken density wording and leave the run as pending/future.
- **Handoff commit:** same as the F=10 item above, `fc7991e6`; the data-freshness packet landed LICCA script fixes and smoke runners, not a full data-rerun matrix.
- **Script readiness state (2026-04-15):** fixed the known `CARGO_TARGET_DIR` binary-path bug for both `sys-*` scripts; added local `job-smoke.sh`; kept the 1-second `--time` tripwire, so Jörn must submit production with `sbatch --time=02:00:00 job.sh` after the test-partition dry run. Build commands are `cd ~/msc-math && CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general --bin sys-gradient-ascent-products`, followed by `cd experiments/sys-landscape/<experiment> && mkdir -p logs && sbatch ... job.sh`.
- Each family produces histogram + bucket counts at sys>0.95/0.99/1.00.
- Re-plan trigger: results back → update `RESULTS.md` density claim.

### [future] [group:licca] Combinatoric-changing step sizes on LICCA
- Beyond fixed-F ascent — let random walks flip facet combinatorics mid-trajectory.
- Post-Kai: defer beyond thesis unless Jörn explicitly reopens research development before the 2026-04-21 cutoff. If fixed-F LICCA finds nothing, this remains a future-research next step, not a thesis blocker.

### [future] Analytical formula for sys(P_5 x R(theta) P_5)
- Standalone mathematical result in `RESULTS.md`: explain the shape of the pentagon rotation curve.
- Jörn knows the symbolic enumeration algorithm. Missing work: run it in a CAS over a field such as `Q(sin(theta), cos(theta), sqrt(5))`, then write the proof cleanly.
- Scope target is `P_5 x R(theta) P_5`; no general formula for higher `(n,m)` pairs is required. Higher-pair function fitting remains useful only if it reveals recognizable shapes and later conjectures.

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

### [open] [group:landscape] Feature regression + local-maxima pattern search
- Post-Kai priority: closure-blocking for the hostile-landscape thesis wording, but bounded in method scope and effort. Do not invent novel tools here; throw standard data-science methods at the available datasets and see whether any transferable signal actually appears.
- Run regression/classifier methods on random polytopes using Euclidean and symplectic feature data. More importantly, run the same checks on local maxima found by ascent.
- Candidate outcomes: a transferable signal gives a conjecture or guided search strategy; no signal or only non-transferable structure supports the hostile-landscape conclusion.
- Dependencies: random/polytope datasets are available now; use current local-maxima datasets immediately and extend to LICCA-returned local maxima if those artifacts arrive in time.
- Acceptance: produce a bounded standard-method pass over random samples and ascent-found local maxima, report cross-validated predictive performance plus feature importance or failure mode, state whether any signal transfers between the two regimes, and update `RESULTS.md`.
- Stop condition: if a real signal appears, surface it for Jörn's mathematical interpretation before turning it into a conjecture; if the standard-method pass yields no transferable signal, record that negative result and stop; do not open a novel method-development line here.

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
- All 6 axioms pass. Orbit-recovery validation exists, but its historical 112-row summary is stale against the current shared cache; see the open orbit-recovery packet in the paranoia section below.
- `experiments/verification/correctness/`

### [open] [group:library] Capacity/orbit result API architecture
- Problem: the public library API exposes capacity-focused entry points (`ehz_capacity`, `ehz_capacity_unpruned`, `billiard_capacity`) whose result stores one certified best permutation and beta. Several experiments need richer algorithm output: all certified candidate orbits, all minimum-action simple orbits within tolerance, near-active witnesses, pruning/solver diagnostics, and recovered primal trajectories. Today that richer output exists only as copied experiment instrumentation or module-internal traversal.
- Durable design note: `research/repo-maintainability/design/hk2017-result-api-plan.md`.
- Architecture note from the first repo-level doc pass: the current layering is also conceptually messy. `EhzResult` wraps `CapacityResult`; geometric-orbit recovery is a separate pass (currently named `OrbitRecovery` in code); derivatives are separate low-level functions consuming orbit/KKT ingredients rather than a named report object. Later cleanup should decide whether this layering is intentional or accidental complexity.
- Execution status on `capacity-result-api-exec`: Packet 1 scaffold landed as `library/src/algorithms/orbit_search.rs` with shared enums/types for the new result layer. Packet 2 is complete for the saddle-point-backed collector goal: there is a shared `solve_orbit_sigma(...)` primitive, a shared `collect_legacy_capacity(...)` seam for the current solve/classify/track/finalize loop, internal exact-fallback / guarantee-mode helpers, and public `OrbitSearchResult` collectors `hk2017_minimum_orbits(...)`, `hk2017_minimum_orbits_unpruned(...)`, and `billiard_minimum_orbits(...)`. Packet 3 is in progress: `library/src/derivatives.rs` now exposes `OrbitGradientA`, `ClarkeSubdiffA`, `DerivativeError`, a `KktResult`-level derivative helper, an `OrbitKktData`-level derivative helper, and primitive Clarke directional-derivative helpers; the migrated buildable packages now include `exp-combinatorial-cells`, `exp-sys-landscape`, `exp-hko-local-maximum` (`hko-second-order`, `hko-cut-and-ascent`), and the first truly subdifferential-heavy binary `dev_numerics_subdifferential`. The current open limitation is still explicit projected-backend support: `OrbitSearchError::UnsupportedBackend` reports that `library/src/kkt/projection_solver.rs` does not yet expose the payload/error-bound contract needed by the shared result layer.
- Specific result-shape issue to discuss explicitly: returning only one best orbit is likely the wrong boundary for the richer API. If the library can prove the minimum-orbit list is nonempty, consumers that only want one representative can take the first item themselves, while richer consumers still get the full minimum-action set without re-enumeration or copied instrumentation.
- Specific follow-up to keep in scope: move Clarke-subdifferential support into the library. The required operation is the directional derivative surface `d -> D_d capacity = min_i <d, grad capacity_sigma_i>` for a supplied list of minimum-action orbits. The data type may stay primitive for now, e.g. a list of per-orbit `grad capacity_sigma_i` objects in the same order as the input sigma list, rather than a heavyweight abstract subdifferential type.
- Post-Kai priority: discuss/design before implementing broad library changes. A small API design session is useful before the all-minimum-orbit validation task; full implementation is optional polish before about 2026-04-21 and future work after that unless it fixes thesis reproducibility.
- Candidate design surface: one shared orbit/result layer for `ehz_capacity`, `ehz_capacity_unpruned`, and `billiard_capacity`, with separate search frontends for general search, pruning policy, and Lagrangian-product-specific enumeration/validation. Thin convenience wrappers may remain only as staging aids, not as the design goal.
- Decisions to make: which data belongs in `library/` result types vs experiment-owned report rows; whether recovered primal trajectories belong in capacity results or in a separate recovery pass; whether derivatives should stay as low-level functions or gain a higher-level report surface; whether Clarke-subdifferential support should operate on gradients only or also evaluate `d -> min_i <d, g_i>` directly; how tolerance for "all minimum" is represented; how billiard-specific input validation / bounce-count conveniences sit on top of the shared result layer; and which APIs are public vs `pub(crate)` until stabilized.
- Acceptance check for the design/execution session: the durable design note and execution packets name the public functions/types, migration path for copied experiment instrumentation, verification commands, explicit non-goals for the thesis push, and the required doc updates (`TASKS.md`, `ARCHITECTURE.md`) plus the rerun surface that will demonstrate the refactor actually worked. Current verification on `capacity-result-api-exec` includes the Packet 2 collector checks `cargo build -p symplectic --release`, `cargo test -p symplectic --release minimum_orbits -- --nocapture`, `cargo test -p symplectic --release --lib`, plus the Packet 3 checks `cargo test -p symplectic --release derivatives::tests -- --nocapture`, `cargo build -p exp-combinatorial-cells --release`, `cargo build -p exp-sys-landscape --release`, `cargo build -p exp-hko-local-maximum --release`, and `cargo build -p dev-gradient --release --bin dev_numerics_subdifferential`.
- Stop condition: if the design requires resolving tube correctness, changing solver semantics, or committing to a broad future-proof public API guarantee beyond the thesis-push surface, defer the implementation and keep validation experiment-local.

### [open] [group:verification] All-minimum simple-orbit validation (local-first)
- Goal: empirically verify the method that recovers all simple minimum-action orbits, not only the capacity value or one best permutation. This is a verification/polish task for the post-Kai polish window, not a LICCA-bound search by default.
- Current code state: public `hk2017::ehz_capacity` returns one best permutation; several experiments have copied instrumented enumeration code that collects all valid or near-optimal orbits. `orbit_recovery::recover_and_verify` validates one `(sigma, beta, capacity)` result at a time, so the validation session depends on either an experiment-local adapter or the library API design task above.
- Local dataset default: use diversity from existing local artifacts before inventing new data: known polytopes, `experiments/verification/correctness/correctness.jsonl`, cached experiment polytopes with rational dual vertices, and a small stratified F<=10 sample. LICCA is only relevant if Jörn explicitly asks for rare-event or high-F stress, not for the first validation pass.
- Acceptance check: for each selected polytope, enumerate certified simple orbits, identify all minimum-action orbits within an explicit tolerance, cross-check the minimum action with `ehz_capacity`, recover each minimum orbit, and report counts plus max closure/on-facet/inside/action errors. Known symmetric cases should record expected multiplicities when they are already known from local evidence.
- Stress-test escalation: if this becomes a stress test, broaden it into a capacity-and-lemma validation bundle instead of stressing only orbits. The broader bundle should cover capacity agreement, all-minimum orbit recovery, pruning/adjacency assumptions, beta-positivity classification, and the intermediate lemmas used by the capacity pipeline.
- Stop condition: if tolerance choice changes the minimum-orbit count, if a recovered minimum orbit fails geometry checks, or if the required helper API would become a library design decision, stop and surface the evidence for Jörn.

### [done] [2026-03] 4b-partial. Q error and KKT inertia
- 1.13M nodes, worst E=2.9e-11. Empirically exact at f64.
- Eigenvalue inertia formula holds for 6/7 polytopes, 5 mismatches are threshold artifacts.
- `experiments/numerics/q-error/`, `experiments/numerics/kkt-inertia/`

### [open] [group:numerics] 4b. Numerical error bounds (verify-numerics)
- `formal/numerics/error-bounds.tex` Parts I+II complete. Proven Q error bound, eta bound for well-conditioned problems.
- 14 previously-failing tests now pass (329 pass, 0 fail).
- Rationale for current state: degenerate orbits are never capacity-achieving, so final capacity comes from well-conditioned orbits with proven low error. Gap remains for publication.
- Open: Part III (f64 algorithm description), eta bound for LP null-space search (39 violations on natural data with near-zero eigenvalues), GAP in cor:taylor-structure proof (needs Jörn).
- Post-Kai priority: publication polish and thesis confidence, not a prerequisite for the two main thesis results. Before about 2026-04-21, agents may close self-verifying pieces such as stale notes/tests; after that, leave explicit caveats or cut proof ambitions rather than opening new solver work.
- `experiments/numerics/error-bounds/`, `experiments/numerics/error-bounds/algorithm-notes.md`

### [open] [group:numerics] Projection solver
- 5-step algorithm: (1) solve equality constraints → (m-5)-dim affine space, (2) project H → reduced Hessian, (3) eigendecompose → null directions, (4) beta>0 as LP on projected null space, (5) recover multipliers.
- Basic implementation in `kkt/projection_solver.rs`. Needs mathematical rigor + ablation comparison.
- Post-Kai: defer broad promotion/refactor unless it becomes part of the local validation or stale-note cleanup before about 2026-04-21.
- `experiments/numerics/error-bounds/algorithm-notes.md`

### [open] [group:numerics] Beta-LP unification
- Replace `find_positive_beta_1d`/`find_positive_beta_nd` with single LP: maximize min_j beta_j subject to beta = beta_0 + V*alpha.
- Previous branch deleted (tip `7ca81b53` has salvageable design: unified function, Type A/B/C eigenvector classification).
- Thesis/code tension: thesis proves rank-deficient pairs are redundant (discard); code searches null space for beta>0 on *near*-singular systems (pseudoinverse beta_0 may have beta_i < 0 from noise; null-space shift recovers feasibility without changing Q). Not contradictory but needs explicit documentation.
- Open question for Jörn: is filtering Type A directions mathematically justified?
- Post-Kai: documentation of the tension is useful polish; implementation unification is deferable unless a bounded session can finish and verify it before about 2026-04-21.

### [open] [group:numerics] Solver numerical formal writeup
- Per-module formal files for SVD, condition numbers, LU, eigendecomposition stability.
- Multiple modules use SVD without shared error analysis.
- Post-Kai: write only the pieces that directly support thesis text or current validation. Defer full per-module numerical formalization after about 2026-04-21.

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

### [future] [group:writeup] AI/process reflection
- `RESULTS.md` process result: analyze how AI agents contributed to the project, including counterfactual impact, useful failure modes, and lessons for future mathematical software work.
- Jörn owns the framing and counterfactual judgment. Agents can gather repo history, session notes, task records, and concrete examples once the main thesis content is stable.
- Acceptance: thesis section/appendix draft exists with evidence-backed examples, or Jörn explicitly cuts the process reflection from the thesis.

### [done] [2026-04-12] [group:writeup] Math write-up scaffold
- Scaffold note: `scratch/math-writeup-scaffold.md` (778 lines, grep-verified).
- Counts: 69 unverified blocks (unchanged from 2026-04-07), 41 `TODO: JÖRN` markers, 10 GAP markers, 100 theorem-like environments across 18 files.
- Top 4 ranked hard-labor items: `prop:capacity-piecewise-smooth`, `lem:cap-derivative`+`lem:vol-derivative`, `prop:prefilter-bound`, `prop:capacity-symplectic-product` (library GAP).
- Ahead of 2026-04-13 schedule. Kai briefing item (below) can consume it once LICCA Sunday preliminaries are back.

### [done] [2026-04-15] [group:writeup] Kai meeting gate closed; briefing obsolete
- The planned `kai-briefing-2026-04-14.md` was not produced before the meeting. Do not reconstruct it after the fact.
- Outcome recorded in the schedule section: Kai agreed the two main `RESULTS.md` result blocks are sufficient for thesis completion; optional polish runs only through about 2026-04-21 unless it fixes thesis correctness or submission blockers.

### [done] [2026-04-12] Thesis figure consistency check
- Thesis figure audit: 0 `\includegraphics` refs across 16 `thesis/**/*.tex` files, `thesis/assets/` does not exist. Rerun after experiment writeups and thesis restructuring land.
- Note: before the first figure lands, Jörn picks an asset-provenance convention (sidecar `.source` files / manifest / sync script) and adds it to `AGENTS.md`. See "Hand-drawn figures" + "Figure inventory" below.

### [future] [group:figures] Hand-drawn figures (Jörn)
- Concept illustrations: polar of 2D polytope; 0/1/2/3-facets; fake 3D polytope with Reeb vectors and closed/open trajectories; decompositions of `gamma: R → R^4` drawn as `gamma: R → R`; etc.
- Agent scope: figure inventory only (list, not produce), and only after narrative structure stabilizes so we know which concepts get illustrations and where.

### [future] [group:figures] 3D visualization write-up + figure integration
- `RESULTS.md` standalone mathematical result: visual inspection of 3D projections of 4D polytope geometry and Reeb dynamics did not reveal a usable geometric pattern.
- Tooling exists in the visualization pipeline; remaining work is choosing which screenshots/assets belong in the thesis and writing the negative-result/communication framing.
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
- Completed: step_bound duplication extracted to `experiments/sys-landscape/src/lib.rs` (cut-and-ascent gets inline copy); products-vs-random split; wiggle strength justified + documented; `[lem:dual-vertex-qp]` proof drafted + Jörn-approved (Lemma 37 in `formal/main.pdf` p11); formal stubs audited (53 stubs + 69 unverified blocks as of 2026-04-07).
- Remaining future note: gradient-ascent + multiple-crossings overlap dedup is blocked until gradient ascent stabilizes into library. Until then, copy-edit between experiments is correct.
- Known side effect: step_bound upgrade (omega_0 detection) changes experiment behavior if re-run. Existing JSONL not regenerated.

### [open] [group:paranoia] Paranoia: numerical claims (closure pass 2026-04-15)
- First pass merged: `paranoia-numerics` branch, 19 files fixed across experiment logbooks + `experiments/numerics/error-bounds/tests.rs` + `experiments/numerics/unknown-predicates/main.rs`.
- The old top-level audit report has been retired after migration into current tracker/design-note ownership. The long pre-migration body was mostly stale: the rotated-products path bug, numerics testdata path bug, and unknown-predicates dataset-label bug are fixed; most other old flags are either closed in current design notes or reduced to the six live packets below.
- Profiling closed as a live issue: `experiments/verification/algorithm-comparison/profiling/logbook.jsonl` keeps the bad 2026-04-04 zero-duration row as history, but the 2026-04-15 `f5d4ba18` row + `profile.jsonl` are the first usable post-fixture-removal baseline.
- 2026-04-16 tracker reduction: the checkpoint cleanup closed the design-note-only rows for combinatorial-cells convexity, numerics error-bounds prose, crosspolytope timing wording, and cut-and-ascent timing wording. The remaining live rows are orbit recovery plus the LICCA-scale perturbation neighborhood run.
- Live follow-up packets:
  - `experiments/verification/orbit-recovery/`: choose dataset scope (170 cached vs 112 historical vs smaller validation set), compute real `solution_dim`, regenerate `orbit-recovery.jsonl` + figure, and refresh `research/verification/design/orbit-recovery.md` plus the stale historical capacity-axiom summary above. Cross-check this against the shared-cache/data-flow audit below before deciding that the 170-row mirror should become the thesis-facing baseline. Jörn only if dataset scope is thesis-facing.
  - `experiments/hko-local-maximum/perturbation-neighborhood/`: split historical `pentagon-perturb.jsonl` findings from current `data/{smoke,licca}-eps-*.jsonl` analyzer outputs; update stale task/design min/max wording.
- Closed in checkpoint `9d55e7f8`:
  - `experiments/combinatorial-cells/convexity/`: design note now matches committed JSONL (2800 rows, 2661 successful midpoint constructions, 1558/1558 product transition failures, 0/1103 random transition failures).
  - `research/numerics/design/error-bounds.md`: stale M1 wording marked historical; stale `make smoke` / `make full` commands removed.
  - `research/crosspolytope/design/main.md`: elapsed-time wording reconciled to the committed `1095.1s` JSONL result vs historical `1112.8s` console/table total.
  - `research/hko-local-maximum/design/cut-and-ascent.md`: stale `~10s per trial` scale-up estimate removed.
- Stop condition for this closure bundle was reached: more than 2-3 live fixes remain, so they are split as packets instead of being fixed in this reconciliation task.
- Post-Kai priority: high-value polish because it protects thesis claims. Before about 2026-04-21, prefer shallow evidence repairs and stale-note fixes; after that, weaken or qualify claims instead of rerunning broad experiments.

### [open] [group:paranoia] Data freshness and rerun matrix
- Source packet: `/tmp/4.md` asked for a prioritized table of evidence gaps with columns `claim`, `current data`, `missing data`, `local vs LICCA`, `estimated runtime`, `job/script readiness`, and `thesis impact`. The finished work instead landed the useful LICCA script-readiness commit `fc7991e6` and did not create the full matrix.
- Do not redo the LICCA script audit first: `fc7991e6` added `job-smoke.sh`, fixed `CARGO_TARGET_DIR` binary paths, ran the three smoke scripts, and updated the active LICCA handoff notes. Remaining LICCA-side check is external: current repo layout under `~/msc-math` on LICCA before `sbatch`.
- Current matrix snapshot (2026-04-16):

| row | current data | missing data / blocker | local vs LICCA | runtime / readiness | thesis impact | recommendation | deadline bucket |
| --- | --- | --- | --- | --- | --- | --- | --- |
| orbit recovery | `polytopes.jsonl` mirror exists with 170 rows; no committed `orbit-recovery.jsonl`; `main.rs` still writes `solution_dim: 0`; design note still presents the old 112-row run | Jörn-facing dataset-scope choice; regenerated dataset, figure, and real `solution_dim` | local-first | local binary + analyzers exist; no LICCA script needed; blocked by scope choice, not infrastructure | verification/polish task for orbit-recovery and all-minimum-orbit work | weaken/reword now, then rerun locally after scope is fixed | fix before 2026-04-21 if thesis-facing scope is chosen, otherwise weaken/reword |
| perturbation neighborhood | historical `pentagon-perturb.jsonl` kept; committed smoke files exist for three eps buckets; LICCA production files absent | production-scale 10k-per-bucket run | needs LICCA | `job.sh` + `job-smoke.sh` exist; smoke path already verified; production script budgets 30 min and only needs the external LICCA repo-layout check | load-bearing for the pending large-N HKO neighborhood falsification claim in `RESULTS.md` | needs LICCA | fix before 2026-04-21 only if LICCA results come back in time; otherwise leave pending/future |
| convexity | committed `combinatorial-boundaries-convexity.jsonl` has 2800 rows and the design note matches it | none for current thesis wording | local if ever rerun | binary + analyzer exist; no current need to execute | supports the hostile-landscape / non-convex-cell interpretation, but current committed data already covers the claim | do not rerun because it would only move already-synced counts/figures | defer/future |
| numerics error-bounds note | current binaries/tests exist; stale note wording already fixed | none for the paranoia row; deeper solver work remains a separate numerics task | local if reopened | local commands documented; not a LICCA surface | prose clarity only; not a missing evidence artifact after the checkpoint cleanup | do not rerun because the stale issue was note-only | defer/future |
| crosspolytope timing | committed `crosspolytope.jsonl` records `1095.1s`; note already labels `1112.8s` as historical console/table output | none for current thesis wording | local if intentionally recomputed | binary exists, but rerun is a long recomputation of an already-established standalone result | standalone established result already in `RESULTS.md`; recomputation risks moving a non-essential timing number | do not rerun because it would move a non-load-bearing timing figure | defer/future |
| cut-and-ascent timing | committed `cut-and-ascent.jsonl` has 20 preliminary rows; stale estimate already removed from the note | no trusted current scale-up timing estimate; bigger run would become a new experiment scope | local if reopened | binary exists; per-trial budget in code is 180s, but no current tracker requirement to measure it | current thesis use is only the empirical `0/20 improved` evidence; larger-F validation is already future work | do not rerun now; keep the current preliminary claim and defer broader sampling | defer/future |

- Seed rows from the paranoia closure shortlist: orbit recovery, perturbation neighborhood, combinatorial-cells convexity, numerics error-bounds note, crosspolytope timing, and cut-and-ascent timing. Add non-paranoia stale evidence only when it affects `RESULTS.md` or an active thesis claim.
- Immediate PM consequence from the matrix: only two rows still need action in the paranoia bundle. Orbit recovery needs a scope decision plus a local rerun path; perturbation neighborhood needs the external LICCA submission step. The remaining four rows should not absorb another rerun session unless a separate research reason appears.
- Repo-wide follow-up discovered during pre-merge on 2026-04-16: sweep other experiment binaries for default or `--smoke` code paths that still overwrite production `.jsonl` outputs or production-side cache overlays. Target outcome: smoke reruns write to untracked `smoke-*.jsonl` / `smoke-*.csv` style outputs unless the caller explicitly requests a canonical refresh.
- Stop condition: if the matrix recommends a new large experiment family rather than a rerun of an existing package, stop for Jörn's thesis-priority decision.
- Post-Kai deadline rule: the matrix should classify rows into `fix before 2026-04-21`, `weaken/reword`, or `defer/future` in addition to local vs LICCA. Do not let the matrix create a new required coverage obligation.

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
- Post-Kai: use the polish window for pruning and correctness alignment. After about 2026-04-21, only fix items that would make the thesis wrong or unreproducible; leave code-side cleanup as future work.

### [open] [group:pm] Repo maintainability / architecture program
- Durable planning note: `research/repo-maintainability/design/main.md`
- Purpose: gather repo facts first, then prepare the Jörn review surface for the broad maintainability refactor. Separate `observed facts`, `open architecture decisions`, and `candidate execution packets` before freezing a multi-session DAG.
- Seeded facts already recorded in the note: before this session there was no top-level `ARCHITECTURE.md`; experiments already depend on deep library paths; topic packages already have `src/lib.rs` helper crates; the 170-row polytope cache is mirrored in three identical files; `variable-f-ascent` cache is intentionally local.
- Discovery artifacts now written:
  - `research/repo-maintainability/design/repo-facts.md`
  - `research/repo-maintainability/design/import-surface-inventory.md`
  - `research/repo-maintainability/design/shared-helper-inventory.md`
  - `research/repo-maintainability/design/data-flow-inventory.md`
  - `research/repo-maintainability/design/docs-navigation-inventory.md`
  - `research/repo-maintainability/design/execution-constraints-inventory.md`
- Current documentation method: facts first, architecture prose second. The consolidated current-state fact base is `research/repo-maintainability/design/repo-facts.md`; `ARCHITECTURE.md` should be derived from it instead of mixing discovery with policy.
- `ARCHITECTURE.md` now carries both the component/code architecture and the current persisted-data architecture. The separate `DATAFLOW.md` trial was dropped as unnecessary structure for the current repo size. `AGENTS.md` remains the short repo map; `ARCHITECTURE.md` is intentionally descriptive and still light on policy where the repo has not decided it yet.
- Current phase: architecture-doc pass and capacity/orbit API decision surface reviewed. Next step is execution-packet planning and worktree setup for the approved shared result-layer direction (`hk2017`, `hk2017_unpruned`, `billiard` sharing one orbit/result layer with separate search frontends).
- Discussion order for the next phase: keep the design note as the source of approved API direction, then write and execute bounded packets incrementally instead of freezing the whole DAG upfront.
- Next PM action: commit the durable design notes, create a dedicated feature worktree, and start packetizing the refactor around shared core types/search frontends/consumer migration.
- Acceptance check: later sessions can resume from `TASKS.md` plus the note without chat history; the note names discovery packets, Jörn decision points, execution-packet template, and the current safe resume point.
- Stop condition: if the note starts implying API or data decisions that Jörn has not reviewed, keep them as options in the note instead of promoting them to tracker facts.

### [open] [group:library] Experiment-to-library algorithm surface audit
- Purpose: decide which experiment-grown algorithms are stable enough to promote, wrap, extract to a topic-local helper, or explicitly leave experiment-owned before the 2026-04-21 development cutoff. This is a triage/design task, not a broad refactor permission.
- Evidence surface:
  - Rich HK2017 enumeration appears repeatedly as copied `ehz_capacity_instrumented` / `enumerate_all_orbits` code in HKO, combinatorial-cells, and gradient-validation experiments. This is the highest-value library API gap and is tracked directly by the "Capacity/orbit result API architecture" task.
  - `sys = c^2/(2 vol)` and `d_sys/da` are recomputed in several experiments using library `capacity_derivatives_a` and `volume_derivatives_a`. A small `systolic_ratio` / `sys_derivatives_a` helper could reduce drift, but only after Jörn is comfortable with the derivative lemma status.
  - Combinatorial-cell step-bound detection (`compute_step_bound_detailed`, including incidence, omega_0 sign, and dual-vertex degeneration events) is stable enough for shared experiment code; library promotion should wait until there is a public "combinatorial type/cell" API design.
  - Topic packages already have `src/lib.rs` entry points (`exp-combinatorial-cells`, `exp-hko-local-maximum`, `exp-numerics-gradient`), but some of those helper crates are still empty while shared routines remain copied across binaries. Extracting to `experiments/<topic>/src/lib.rs` is often cheaper and safer than immediate library promotion.
  - Projection-solver diagnostics and exact-QP validation are richer in `experiments/numerics/error-bounds/` than in the public library API. The library now has the fixed projection solver and exact KKT solver; stale experiment-local comments/copies should be pruned or relabeled before adding more solver APIs.
  - Experiments already import deep library paths such as `hk2017::permutations`, `hk2017::orbit_recovery`, and `kkt::saddle_point_solver`, so the audit should record which deep paths are intended expert surfaces versus accidental internals that later agents should avoid depending on.
  - Gradient ascent, wiggle/overshoot escape, add-facet/variable-F ascent, rotated-product sweeps, and crosspolytope symmetry reduction are publishable experimental methods or special computations, not general library algorithms for the thesis push.
- Recommended before about 2026-04-21: do a short design pass for rich capacity/orbit reports; optionally add a tiny `sys` helper if it unblocks validation or write-up; classify repeated helpers as `library`, `topic-local helper`, or `per-binary local`; and clean stale duplicate solver comments if they risk confusing agents. Defer broad migration of ascent/search heuristics, combinatorial-cell APIs, and crosspolytope-specific symmetry code.
- Acceptance check: a design note or short patch names each candidate as `promote now`, `extract to topic lib`, `experiment helper only`, `document stale copy`, or `future`, records the intended stable import path for anything shared, and gives one verification command per promoted or extracted API. If no code is promoted, close by linking this audit from the broad SWE polish bucket.
- Stop condition: if a candidate changes mathematical claims, proof obligations, or public solver semantics, stop for Jörn rather than promoting it as polish.

### [open] [group:data] Experiment data-flow audit and cache plan
- Purpose: map which experiments can reuse polytope/capacity/sigma datasets and which ones need experiment-owned intermediate data. This is the data-flow analogue of the algorithm-surface audit; do not start by moving `.jsonl` files.
- Current cache evidence:
  - `library/src/database.rs` defines `PolytopeRecord` with rational dual vertices, rational vertices, optional volume/capacity, and optional `sigmas`; callers own path policy and there is no canonical mutable shared cache.
  - `experiments/sys-landscape/cache.jsonl`, `experiments/combinatorial-cells/polytopes.jsonl`, and `experiments/verification/orbit-recovery/polytopes.jsonl` are byte-identical on 2026-04-15 (`sha256sum` `8679b89763a10bf1380410f288845f03bcdc8e365035aa31235ff00c9cc07363`), 170 rows each, with volume, capacity, and one best sigma (`sigma_gap_cutoff = 0.0`).
  - `experiments/sys-landscape/variable-f-ascent/cache.jsonl` is larger (1685 rows) and intentionally local: it stores many intermediate gradient-step polytopes so the shared family cache does not become a transient search log.
- Data-shape classes:
  - **Minimum action only:** random/product sampling, perturbation sweeps, rotated-product sweeps, correctness smoke data, and many landscape plots can reuse rows with dual vertices, volume, capacity, and sys.
  - **Best sigma at the minimum:** orbit recovery, omega-obstacle gradients, gradient-ascent derivative steps, and some combinatorial-cell experiments can use a cached best permutation plus a cheap single-perm KKT solve for beta.
  - **All tied or near-minimum sigmas:** HKO sensitivity/second-order work, subdifferential experiments, all-minimum-orbit validation, and witness-oracle work cannot be served by the current one-sigma cache; they need the richer capacity/orbit report API or an experiment-owned extension with a nonzero gap cutoff.
  - **Non-minimum intermediate nodes:** numerical error-bounds, Q-error/inertia, algorithm ablation, and solver benchmarks need per-`(S, sigma)` matrices, solver verdicts, or timing variants. These should stay in custom datasets rather than a shared polytope catalog.
  - **Path-dependent search traces:** gradient-ascent traces, variable-F paths, and LICCA shard outputs are analysis artifacts, not reusable polytope catalogs except for their final polytopes.
- Recommended before about 2026-04-21: write a short data-flow note or tracker packet naming one canonical source for the 170-row polytope catalog, the allowed consumer paths, and the fields each consumer may trust. Prefer documenting or scripting mirror refresh over changing all experiments at once.
- Acceptance check: produce a table with columns `experiment`, `input polytopes`, `reusable fields`, `missing fields`, `source dataset/cache`, `output dataset`, `reuse allowed?`, and `stale/drift risk`; verify identical cache mirrors with `sha256sum`; name any `.jsonl` files that should be future-only, mirrored, or regenerated.
- Stop condition: if an optimization would change committed data values, alter thesis-facing figures, or merge transient search states into a shared cache, stop for Jörn's thesis-priority decision.

### [open] [group:docs] Agent-facing architecture and navigation guide
- Goal: give future agents a repo-level map for the common "where does this live?" questions across `library/`, `experiments/`, datasets, and verification commands. Current orientation is split across `AGENTS.md`, `TASKS.md`, `library/src/lib.rs`, `library/src/database.rs`, and per-package `src/lib.rs` headers; there is no top-level architecture guide today.
- Preferred output: top-level `ARCHITECTURE.md` that answers the frequent navigation questions first and links outward to module headers, formal files, and tracker packets instead of duplicating them. This is documentation and boundary-setting work, not a broad refactor task.
- Minimum contents:
  - workspace map (`library/`, `experiments/`, `formal/`, `research/`, `thesis/`) with which surfaces are thesis-facing, experiment-owned, or infrastructure.
  - stable/simple library entry points versus expert deep import paths versus intentionally experiment-local helpers.
  - data-flow map: canonical shared polytope catalog, mirrored caches, experiment-owned transient datasets, and JSONL/LFS rules.
  - code-placement rules: when to promote to `library/`, when to extract to `experiments/<topic>/src/lib.rs`, and when to keep logic per-binary.
  - "start here" verification commands for common agent tasks (build one package, run one experiment smoke, check a data mirror).
- Dependency note: this guide should consume conclusions from "Capacity/orbit result API architecture", "Experiment-to-library algorithm surface audit", and "Experiment data-flow audit and cache plan". A first draft may land earlier only if it marks open decisions explicitly instead of guessing them.
- Acceptance check: a new agent can answer the common navigation questions without source spelunking: where to compute capacity/volume/sys, where shared experiment helpers belong, which caches are canonical, which import paths are intended to be stable, and which artifacts are generated-only.
- Stop condition: if the guide starts freezing undecided API or data policy, link the open task packet instead of inventing a rule.

### [open] [group:writeup] Dual-vertex parameterization (a_i migration)
- Library API done. Most experiment migration complete. Math.tex migration complete.
- `formal/library/algorithms.tex` uses a_i throughout.
- Remaining:
  - Jörn verifies `[lem:cap-derivative]` and `[lem:vol-derivative]` (marked `\begin{unverified}`)
  - (`[lem:dual-vertex-qp]` proof was completed under Code cleanup 2026-04; see line 241.)

### [open] [group:docs] Formal stub / unverified inventory (baseline for write-up scaffold)
- Audited 2026-04-07: 53 explicit stubs + 69 unverified blocks. No proofs lost in migration — stubs were created as stubs, agent-written proofs added later.
- Refresh is Bundle G precursor (see "Math write-up scaffold" in Thesis section); that item also re-categorizes by hard-labor framing and adds theorem dependency graph.
- **High priority** (blocks thesis or code correctness):
  - `lem:cap-derivative` + `lem:vol-derivative` (`formal/library/algorithms.tex`:709,781) — core gradient lemmas, also tracked in "Dual-vertex parameterization" below.
  - `prop:prefilter-bound` (`formal/library/geom.tex`:789) — needs restatement in terms of computable `hat_kappa`. Factor-counting issue: tight bound is 5376, not 1344.
  - GAP in `prop:capacity-symplectic-product` (`formal/library/geom.tex`:157) — `c_EHZ(A) = area(A)` for 2D convex bodies is unverified, dubious citation.
- **Medium priority** (thesis completeness):
  - 10 definition verifications in `formal/library/geom.tex` (lines 84-325) — routine review.
  - `thm:conformality` + `thm:sympl-invariance` (`formal/library/algorithms.tex`:107,120) — standard results, need proofs or citations.
  - 3 agent-written proofs needing review: `lem:positive-span`, `lem:vertex-enumeration`, `lem:bounded-triples` (`formal/library/geom.tex`).
- **Low priority** (dev math, not publication path):
  - 11 stubs + 6 gaps in dev-gradient/ and dev-numerical-analysis/.
- Post-Kai: before about 2026-04-21, agents may close routine verifications or convert stale claims into explicit caveats. After that, do not chase full formal coverage; make remaining unverified blocks visible to thesis writing or cut them from the publication path.

### [open] [group:docs] Geom formal file restructure
- Jörn partially reviewed Defs 1–13 of `formal/library/geom.tex` (`library/src/geom/review-notes.md`).
- Consolidate Defs 1-2 (symplectic form). Add Def for HKO2024 + Thm for false Viterbo's conjecture.
- Clarify H-representation irredundancy. Fix Defs 12-13 (area/volume are algorithms, not definitions).
- Consider splitting into `math_geometry.tex`, `math_symplectic.tex`, `math_reeb.tex`.
- Post-Kai: do not perform a broad split unless it directly helps the thesis by 2026-04-21. Prefer narrow corrections to definitions and labels.

### [done] [2026-04-12] [group:docs] Library architecture docs audit
- Library docs audit: existing headers + per-module formal files cover architecture mostly held; 0 blockers, 7 gaps, 3 nits across `lib.rs`, `kkt/`, `algorithms` umbrella, `algorithms/tube`.
- 5 doc-only fixes applied and merged to main (no source/algorithm changes). Notable: `algorithms/mod.rs` tube description rewritten from "(placeholder)" to an explicit wrong-rotation-formula warning; `algorithms/` umbrella gained a "correctness invariant" paragraph (overlapping algorithms must agree).
- Skipped as marginal: `derivatives.rs` cross-directory lemma cite (findable via absolute path), `kkt/mod.rs` formula-location nit, umbrella missing utility math-label cross-refs.

### [future] [group:polish] SWE polish (post-thesis-draft-stability bucket)
- Covers: `dev-*/exp-*` stable code → `library/` promotion, test suite completion + perf, documentation gaps, code simplifications (adopt standard patterns, pull in overlooked libraries, abstract/unabstract as helpful).
- **Do not start broad polish during the thesis push**: rerunning experiments invalidates logbook numbers Jörn is about to write up, and abstraction changes break silent invariants nobody's testing.
- Post-Kai cutoff: through about 2026-04-21, allow bounded polish that yields a self-verifying artifact. After that, this bucket becomes future work except for correctness, reproducibility, or submission blockers.
- Subsumes (or overlaps with) existing `[open] Projection solver`, `[open] Beta-LP unification`, and code-side thesis-alignment items once thesis submission is no longer at risk.

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

### [done] [2026-04-15] [group:repo-layout] Mechanical stale-path cleanup outside onboarding
- Cleaned stale post-migration references in non-onboarding Markdown, TeX, Rust comments, and scan-caught Python diagnostics; no mathematical claims or experiment conclusions changed.
- First-pass cleanup covered `thesis/tube-algorithm-notes.md`, `thesis/appendix-rewrite-notes.md`, `library/src/geom/review-notes.md`, `experiments/numerics/error-bounds/algorithm-notes.md`, and formal headers.
- Follow-up experiment Rust pass removed old wave/subagent migration TODO scaffolding and stale pre-migration crate-path references from experiment comments; the experiment/library migration-wording scan returns no matches.
- Research-note migration coverage audit: `/tmp/1-answer.md` reports 33/33 live `experiments/**/main.rs` directories have live `research/**/design/*.md` notes, 0 live experiments need migration, and the only `logbook*` path is the generated profiling history file. The full matrix was a one-time audit aid; do not preserve it unless a future formal-file coverage task needs row-level evidence.
- Preserved copied-code provenance comments and historical deletion/audit notes by marking them historical or leaving already explicit provenance wording.
- Verification: stale-path scans now report only explicitly historical/provenance hits; changed live references point to existing `formal/`, `library/`, `research/`, `.devcontainer/`, or experiment paths.

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
- Partially explored: `experiments/sys-landscape/variable-f-ascent/`, `experiments/hko-local-maximum/cut-and-ascent/`.

### [future] Dimension scaling study
- How does max-achievable-sys scale with F for random polytopes? Scattered data exists, no systematic study.

### [future] dev-gradient-ascent scaffolds (step-calibration, strategy-comparison)
- Scaffolded, not implemented. `experiments/sys-landscape/gradient-ascent-dev/`
