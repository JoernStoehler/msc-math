# Project Tracker

Unified tracker for thesis, experiments, and infrastructure.
Format: `## [status] Group` / `### [status] [date] Item`. Body only when header isn't enough.
Run `bash scripts/tasks-toc.sh` for section index.

**What the thesis should say:** `RESULTS.md`.
**How thesis success is measured:** `FINAL-VERIFICATION.md`.
**Priority:** Thesis coherence + experiment quality > code refactors. Code refactors only matter if they unblock thesis content or experiment correctness.
**Maintenance:** Record decisions and reasons — these can't be derived later. Don't cache derivable state (build status, test counts) — run the command instead.
**Dependencies:** `thesis/` is stale and needs a retained-content structure before broad writing. Work outside `thesis/` is useful now only if it changes retained thesis claims, prevents a false claim, improves reproducibility for cited artifacts, or saves enough Jörn-time to justify the supervision.
**Post-Kai scope (2026-04-15):** Kai agreed on 2026-04-14 that the two main result blocks in `RESULTS.md` are sufficient to finish the thesis project. Standalone results Kai discussed are worth including, but usually as current-state inclusions rather than new improvement programs.

## Finish Mode — current delta to done (target 2026-05-14)

Current plan state as of 2026-04-24:

- Phase 1 done-state basis accepted by Jorn on 2026-04-24. Do not keep
  elaborating the done state by guessing which content or presentation labor is
  worth doing.
- Target: finish the scoped master-thesis project by 2026-05-14. Finishing by 2026-05-07 is plausible but not assumed.
- No current hard 2026-04-30 deadline is used for planning. Old April schedule rows are historical and superseded by this section.
- The binding resource is Jörn's 40h/week and especially long contiguous Jörn-time blocks, not Codex subscription budget or agent wall-clock.
- Agent labor is cheap but not free: each packet must avoid wasting Jörn-time through unclear scope, missing verification, or ad-hoc review burden.
- Scope-control rule: cool-to-have work is future/follow-up unless it improves the thesis enough to justify the calendar delay and Jörn-time cost. This is a live guardrail because Jörn is easy to nerd-snipe into interesting non-thesis work.
- Reorientation rule: do not encode large conditional trees. Schedule deliberate reorientation points when new information arrives, and propagate surprises through the plan rather than continuing from stale assumptions.

Current result scope:

- Thesis spine: HKO2024 local maximality and the hostile sys-search landscape.
- Standalone results discussed with Kai should appear if they can be included from current evidence with low incremental work: crosspolytope capacity, visualization as negative mathematical exploration, and pentagon-rotation status. Do not improve them beyond current state unless Jörn explicitly says the thesis payoff is worth the delay.
- LICCA large runs are ready-to-use but not thesis obligations. Reopen them only if Jörn chooses the external action or results are already available with low integration cost; otherwise leave the related claims pending/future or weaken wording.
- Broad architecture, API, and code-polish programs are future/follow-up unless they fix a false thesis claim, cited reproducibility, or a direct writing blocker.

Current external/admin facts:

- Final closure means no further direct repo-related master-thesis action remains; the final GitHub archive/read-only action is the last direct repo action.
- University handin requirements are now indexed in `thesis/submission/README.md`
  with downloaded MNTF forms under `thesis/submission/forms/`. TODO(Jörn):
  verify the exact Prüfungsamt copy count, form names, USB/CD contents, and
  upload mechanics from the current Ausgabebescheid / checklist.
- TODO(Jörn): hand in the already-filled Bachelor-/Masterarbeit registration
  form after Elizabeth agrees/signs; earliest expected date from current state
  is Monday 2026-04-27.
- Jörn and Kai want repo preservation outside GitHub so the project is not lost
  if GitHub disappears or Jörn deletes the account. Current named candidate:
  Zenodo, from Kai's mail; see `thesis/submission/README.md` for source links
  and the shallow alternatives pass. TODO(Jörn/Kai): choose the actual
  preservation destination(s) before the final archive step.
- arXiv upload and outreach mails to Haim-Kislev, Ostrover, and similar
  researchers are post-Kai-review dissemination candidates. They are not
  master-thesis closure blockers unless Jörn and Kai explicitly promote one of
  them into the closure checklist.
- No defense talk is part of the thesis project.
- Jörn is the final clarity/usefulness judge in `FINAL-VERIFICATION.md`; Kai and Elizabeth feedback can still mean "not done yet" if they flag a real blocker.

Immediate backchain buckets for the next planning step:

- Current-state and Jorn-knowledge migration: fill
  `FINISH.md` with repo-state signals and Jorn's implicit project knowledge
  before selecting content/presentation labor.
- Done-state closure: keep `FINAL-VERIFICATION.md` aligned with archive/no-more-repo-work semantics only when new external facts require it.
- Current-state reset: clear stale active ownership, classify open work as mainline thesis / contingent during writing / future-follow-up.
- Thesis structure and writing: choose retained chapter structure, then write from already-chosen content.
- Claim-strength freeze: compress HKO, hostile-landscape, numerical appendix, and standalone result wording to what current evidence supports.
- Reproducibility and artifact truth: verify only cited or promised artifacts; do not chase every historical orphaned dataset unless it affects a retained claim.
- Final assembly: build PDF, bibliography/cross-reference/proofread checks, print/USB/forms/upload, final repo tag, backup copies, GitHub archive.

### Historical April 2026 schedule

The previous 2026-04-12 to 2026-04-30 schedule is superseded. Durable decisions
from it are preserved above and in the rows below: Kai accepted the two main
result blocks as thesis-sufficient, optional polish must not expand the thesis
scope by default, and thesis completion now dominates broad research/code polish.

### Conventions for LICCA experiments
Same binary runs locally and on LICCA. The binary takes explicit CLI args (`--n`, `--out`, ...); no `--mode` flag, no Rust-side `if licca {}` branching — all configuration lives in the shell scripts.

Each LICCA-bound experiment ships two scripts in its directory:
- `job-smoke.sh` — plain bash, no `#SBATCH`. Small N. Writes untracked smoke output under `${TMPDIR:-/tmp}` by default and deletes it on exit unless the script documents an explicit keep flag. Runs in this devcontainer. Agents run this as their own verification step before handing off.
- `job.sh` — `#SBATCH` headers, LICCA paths, production N. Jörn scps and submits; the slurm skill (`.agents/skills/slurm/`) has the template + resource-table requirement.

Artifacts per experiment: `job.sh` + `job-smoke.sh` + experiment-specific production JSONL paths. Smoke outputs are execution-only unless the experiment explicitly documents an analyzer path for them. Current examples: `sys-*` production runs still target experiment-owned JSONL while `job-smoke.sh` uses temp `smoke-*.jsonl`; HKO perturbation keeps `data/licca-eps-*.jsonl` for LICCA output and treats smoke output as temp-only.
Before `sbatch`, create the experiment-local `logs/` directory because SLURM opens `#SBATCH --output=logs/%x-%j.out` before the script body runs.

## [open] HKO2024 local maximality

Main conjecture: HKO2024 is a local maximum of the systolic ratio. Potentially publishable alongside thesis.
Key files: `experiments/hko-local-maximum/`, `research/hko-local-maximum-status.md`, `research/hko-local-maximum.md`, `research/hko-local-maximum-exact-clarke.md`.
Literature: BBLM2023 classifies smooth local maximizers (only ball for k=1). Polytope case genuinely open.
HKO2024 lives in multiple ambient spaces (LP(5,5), LP(6,5), F=10, F=13, convex bodies) — local max in one space != local max in a larger space.

### [done] [2026-04] 1a. First-order analysis in a_i space (gradient-analysis)
- Rank 25 in R^40, 15 flat directions. LP confirms 0 in conv(150 per-orbit gradients).
- `research/hko-local-maximum.md`

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
- `research/hko-local-maximum.md`

### [done] [2026-03] 1d. Lagrangian boundary mapping
- Characteristic radius ~0.035, anisotropic (7x aspect ratio), ~10^-31 volume fraction.
- `research/hko-local-maximum.md`

### [done] [2026-03] Perturbation neighborhood (LP(5,5) random perturbations)
- Historical single-eps artifact `pentagon-perturb.jsonl`: 101 recorded perturbations all retain sys>1 (min 1.0142, max 1.0472). Current smoke/LICCA pipeline is tracked separately below.
- `experiments/hko-local-maximum/perturbation-neighborhood/`

### [future] [group:licca] LICCA-scale F=10 neighborhood falsification
- Remaining data/evidence refresh packet for the HKO local-maximality numerics surface: scale the perturbation-neighborhood experiment from the current local evidence to 10k+ perturbations with 3 step-size buckets (small/medium/large). Honest falsification attempt. Expected: no sys>HKO (strengthens conjecture). Real outcome: whatever the data says.
- **Finish-mode status (2026-04-24):** optional publication-grade polish, not required for thesis sufficiency. The job is ready enough to reopen if Jörn chooses the external LICCA action, but it is not active thesis work by default. If no result is already available with low integration cost, keep the existing local evidence and state the large-scale run as pending/future.
- **Handoff commit:** `fc7991e6` fixed the LICCA script readiness layer from the data-freshness packet; current checkout uses `experiments/...` package paths, not old `exp-*` deployment paths.
- **Script readiness state (2026-04-15):** fixed the known `CARGO_TARGET_DIR` binary-path bug by running `"$CARGO_TARGET_DIR/release/hko-perturbation"`; added `job-smoke.sh`; kept build outside production `job.sh` with an executable preflight error that prints the exact `cargo build` command. Before LICCA submission, Jörn runs `cd ~/msc-math && CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target cargo build --release -p exp-hko-local-maximum --bin hko-perturbation`, then `cd experiments/hko-local-maximum/perturbation-neighborhood && mkdir -p logs && sbatch ... job.sh`.
- **Open LICCA-side check:** confirm that `~/msc-math` on LICCA has the same current repo layout. If it still has an old `~/msc-math/crates/exp-*` deployment copy, update that copy or switch to the current repo layout before submitting.
- **PM caveat:** the data-freshness packet that produced `fc7991e6` shifted productively into LICCA script fixes and smoke checks; it did not produce the full cross-experiment freshness/rerun matrix. See the open data freshness task below before treating all stale-evidence questions as planned.
- **Ownership:** no live session owns this row after the 2026-04-24 finish-mode reset. If reopened, one session should own the end-to-end path (audit → fix → smoke → present scp+sbatch → wait → `analyze.py` → figures → logbook → `RESULTS.md` updates → `/pre-merge` → merge); the previous split produced failed transfers.
- Re-plan trigger: after LICCA runs return, re-evaluate density/falsification claims against the finish-mode scope above.

### [Jörn] [group:hko] Verify h-space proof
- Danskin + symmetry + Euler homogeneity argument. ~15 min.
- `research/hko-local-maximum.md`

### [Jörn] [group:hko] Verify second-order formal proposition
- Non-smooth second-order sufficiency proof sketch needs rigor check.
- `formal/hko-local-maximum/second-order.tex`

### [open] [group:hko] Exact Clarke-subgradient checker for HKO2024
- Preferred route for the intended `RESULTS.md` `M_10` theorem: prove that the Clarke-flat directions in dual-vertex coordinates are exactly the infinitesimal symmetry directions, using exact linear algebra instead of the current second-order numerical evidence route.
- Design note: `research/hko-local-maximum.md`
- Durable execution tracker: `research/hko-local-maximum.md`
- Current Packet 1 artifacts: `experiments/hko-local-maximum/exact-clarke/hko-geometry.json`, `experiments/hko-local-maximum/exact-clarke/hko-symmetry-tangent.json`, `experiments/hko-local-maximum/exact-clarke/numerical-minima-summary.json`
- Current execution policy: prefer the larger exact-computation route with the simplest trustworthy setup. Use symmetry/paper reductions first as validation, bookkeeping, and explanation surfaces; make them load-bearing only if the larger exact route fails to become thesis-ready.
- Backend contract: keep the checker backend-agnostic as long as it emits a trusted witness artifact. Current default is SageMath for exact number fields and exact linear algebra; the future Rust number-field backend is an explicit candidate if it gives the same witness contract with better performance. SymPy remains acceptable for smaller exact scaffolding and witness generation when performance is sufficient.
- Current concrete Sage surface: `experiments/hko-local-maximum/exact-clarke/build_widened_seed_witness.py` now emits `widened-seed-witness.json`, and `experiments/hko-local-maximum/exact-clarke/verify_widened_seed_witness.sage` replays exact field/geometry/symmetry/representative-row checks on that single artifact. This is a Packet 3 witness-contract milestone, not the final theorem checker.
- Current exact Sage finding: on the present widened representative surface, the representative rows annihilate all `15` committed symmetry generators exactly. The earlier affine/scaling mismatch came from the representative-row formula, not the symmetry tangent encoding. The widened representative row matrix still has right-kernel dimension `29`, so the remaining obstruction is active-row multiplicity rather than affine-symmetry compatibility.
- Input contract: theorem input may come from a larger exact finite candidate or active surface if the backend can certify the relevant action minima/gaps and emit exact row/rank/kernel witnesses. The checker must not depend on floating-point KKT solves or empirical active-set discovery in the final proof artifact, but numerical planning surfaces are acceptable for choosing what to exactify next.
- Current context: the HKO billiard sigma-word surface has `50,400` raw block words and `6,240` directed-feasible sigma words, versus `717` currently valid KKT orbits and `150` exact minima. In the current sympy probe, the `6240` route is too slow to treat as the default theorem path (`~14h` lower-bound projection before positivity/gap certification), but this is a backend limitation rather than a combinatorics impossibility.
- Current symmetry-quotiented combinatorics count: the `6,240` directed-feasible sigma words collapse to `628` cyclic representatives modulo the order-10 HKO symplectic symmetry group; that is the most relevant finite input size for a Sage-first representative-based route before exact KKT/action pruning.
- Current Sage feasibility signal: the first exact Sage KKT probe on a `100`-representative sample from that `628`-orbit surface projects to about `1.84s` for the full front-end linear-solve stage. So the old SymPy-based `~14h` objection does not apply to the current Sage representative-first route at this KKT front-end rung.
- Suggested revisit point: if the Rust number-field work lands and gives a much faster exact backend than the current sympy route, reevaluate the `6240` directed-feasible sigma surface as a self-contained exhaustive theorem input. That route is currently blocked by exact backend cost, not by the size of the finite sigma set itself.
- Current reduced-route obstruction: one exact endpoint representative plus one exact midpoint representative give only `20` symmetry images, so that ultra-small reduced surface cannot reach active-matrix rank `25`. Current widening state: `5` six-facet representative permutation-orbit classes and `6` midpoint-style seven-facet representative permutation-orbit classes are exactified; only `2` asymmetric seven-facet representative classes remain unresolved on the current numerical planning surface.
- Acceptance: exact artifact exists for the active first-order certificate on the candidate/minimizer surface, together with the symmetry tangent-space basis and any extra argument needed if the final writeup compresses that certificate to a `rank/kernel` summary; thesis/formal wording can then cite this as the first-order quotient-space proof surface.
- Stop condition: if the larger exact route cannot be made correct+trusted with a manageable backend/witness setup, record the obstruction explicitly, then decide whether a more compressed paper/symmetry route is actually needed rather than opening that extra setup complexity by default.
- Dedicated-session split:
  - Session A: keep exactifying the widened active-row representative surface and converge on a correct+trusted witness contract for the large exact route.
  - Session B: implement the exact checker backend and machine-readable witness artifact (`G`, rank, kernel, symmetry inclusion/equality certificates), using Sage or the Rust exact backend depending on readiness/performance.
  - Session C: integrate the exact result into `formal/` / thesis wording and reclassify the second-order note as supporting evidence if the first-order proof closes.

### [future] [group:hko] Higher-F perturbation validation (F=10→12, F=10→13)
- `RESULTS.md` records F=12/F=13 validation as pending/future evidence for the broad HKO2024 local-maximality conjecture; only F=11 checks are currently done.
- Extends facet-splitting and cut-and-ascent to add 2-3 facets simultaneously
- Suggested by the continuation notes in `research/hko-local-maximum.md`

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
- General: 10 seeds, best sys=0.9030. Products: 12 seeds, best sys=0.8727. No sys>1.
- `experiments/sys-landscape/gradient-ascent-general/`, `experiments/sys-landscape/gradient-ascent-products/`

### [done] [2026-03] 2c. Perturbation neighborhood (structurally different sys>1?)
- 100 random perturbations all retain sys>1 but none exceed HKO2024.
- `experiments/hko-local-maximum/perturbation-neighborhood/`

### [done] [2026-04] 2d. Variable-F ascent (F to F+1)
- 90 trials. F=10 local maxima often improve at F=11 but marginal; no sys>1.
- `experiments/sys-landscape/variable-f-ascent/`
- Successor baseline for the next continuation line: `research/sys-landscape.md`

### [open] [group:witness-search] Witness oracle instrumentation + benchmark bank
- Upgrade exact witness search from "best permutation only" to a reusable local-structure oracle: top-`m` / within-gap returns, incumbent warm starts, near-active witness metadata, runtime diagnostics.
- Bundle the benchmark bank into the same session; do not track it as a separate item.
- Finish-mode status: optional AI-work-pattern / publication-polish trial. Do not treat this as required thesis coverage. Stop if the first session does not produce a reusable oracle surface, benchmark report, or clear negative finding without high Jörn interpretation load.
- Pointer: `research/sys-landscape.md`

### [future] [group:witness-search] Witness reuse + safe prefilter calibration
- Quantify trust radius for local witness caches and benchmark safe pruning via `U_A(K) < 1`.
- Compare minimizer-only, top-`m`, within-gap, parent-cache, and hybrid witness sets.
- Fold permutation-neighborhood search and warm-start benchmarking into this line, not separate tracker headers.
- Pointer: `research/sys-landscape.md`

### [future] [group:witness-search] Reduced-model ascent on witness sets
- Soft-min / log-sum-exp reduced-model ascent first; min-norm convex-hull QP second if the first pass is promising.
- Acceptance criterion: compare against exact-evaluate-every-step on the same seeds; report best exact `sys`, exact-call count, and wall-clock.
- Pointer: `research/sys-landscape.md`

### [future] [group:witness-search] Witness-guided F→F+1 continuation
- Replace random facet addition with witness-guided vertex splitting and witness lifting into the child problem.
- Compare directly against `experiments/sys-landscape/variable-f-ascent/` and `experiments/hko-local-maximum/cut-and-ascent/`.
- Pointer: `research/sys-landscape.md`

### [future] [group:witness-search] Symmetry-family search
- Search low-dimensional orbit-union families instead of only generic iid proposals.
- Use the reuse, prefilter, and reduced-model machinery inside those families.
- Keep combinatorial/order-type diagnostics as supporting logging inside this line.
- Pointer: `research/sys-landscape.md`

### [future] [group:witness-search] Box-pruning on structured families
- Downstream of the symmetry-family line: use witness upper bounds to prune parameter boxes once a productive family exists.
- Pointer: `research/sys-landscape.md`

### [done] [2026-03] Random sampling (general + products + calibration)
- Random polytopes max sys=0.739. Random products max sys=0.794 (6x6).
- `experiments/sys-landscape/random-sample/`, `experiments/sys-landscape/random-product-sample/`, `experiments/sys-landscape/rejection-calibration/`

### [future] Regular Lagrangian product formula fitting
- Dense (n, m, theta) sweep. Fit sys(n, m, theta). Does formula predict sys>1 only for 5x5?
- Partial data in `experiments/sys-landscape/rotated-regular-products/`

### [future] [group:licca] LICCA-scale massive ascent sampling (density probe)
- Scale `gradient-ascent-general/` (10 → 10k+ seeds) and `gradient-ascent-products/` (12 → 10k+ seeds).
- **Research question:** does the density of sys>1 local maxima in M_F actually support "no new examples"? Current seed counts are too small for a strong density claim.
- **Finish-mode status (2026-04-24):** optional strengthening of a thesis-sufficient result, not required to finish the thesis. Reopen only if Jörn chooses the LICCA action or results already exist with low integration cost; otherwise weaken density wording and leave the run as pending/future.
- **Handoff commit:** same as the F=10 item above, `fc7991e6`; the data-freshness packet landed LICCA script fixes and smoke runners, not a full data-rerun matrix.
- **Script readiness state (2026-04-15):** fixed the known `CARGO_TARGET_DIR` binary-path bug for both `sys-*` scripts; added local `job-smoke.sh`; kept the 1-second `--time` tripwire, so Jörn must submit production with `sbatch --time=02:00:00 job.sh` after the test-partition dry run. Build commands are `cd ~/msc-math && CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general --bin sys-gradient-ascent-products`, followed by `cd experiments/sys-landscape/<experiment> && mkdir -p logs && sbatch ... job.sh`.
- Each family produces histogram + bucket counts at sys>0.95/0.99/1.00.
- Re-plan trigger: results back → update `RESULTS.md` density claim.

### [future] [group:licca] Combinatoric-changing step sizes on LICCA
- Beyond fixed-F ascent — let random walks flip facet combinatorics mid-trajectory.
- Finish-mode status: defer beyond thesis unless Jörn explicitly reopens research development. If fixed-F LICCA finds nothing, this remains a future-research next step, not a thesis blocker.

### [open] Analytical formula for sys(P_5 x R(theta) P_5)
- Standalone mathematical result in `RESULTS.md`: explain the shape of the pentagon rotation curve.
- Scout workspace landed on 2026-04-18:
  `research/sys-landscape.md`,
  `formal/sys-landscape/pentagon-rotation-formula.tex`,
  `experiments/sys-landscape/pentagon-rotation-formula/`.
- Current conjecture:
  `sys(theta) = ((5 + 2 sqrt(5)) / 10) sec^2(theta)` on `0 <= theta <= pi/10`,
  mirrored by `theta -> pi/5 - theta` on the second half of the fundamental domain.
- Current proof state: the draft writes out the active `2`-bounce `sec` / `sec^2` branch calculation and isolates the remaining blocker as the `3`-bounce exclusion lemma.
- Pre-SageMath merge handoff landed on 2026-04-18:
  `handoffs/p5-rotation-sagemath-handoff.md`.
- Next step after this merge: continue from the SageMath migration branch, port the current CAS witnesses and branch descriptors, and use that surface to finish the `3`-bounce exclusion instead of extending the SymPy-only proof worktree.
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

### [done] [2026-04-18] [group:landscape] Feature regression + local-maxima pattern search
- Post-Kai priority: closure-blocking for the hostile-landscape thesis wording, but bounded in method scope and effort. Do not invent novel tools here; throw standard data-science methods at the available datasets and see whether any transferable signal actually appears.
- Run regression/classifier methods on random polytopes using Euclidean and symplectic feature data. More importantly, run the same checks on local maxima found by ascent.
- Default dataset boundary: omit packets constructed near HKO2024 from the main modeling surface so the analysis does not learn the spoiler "start near the one known counterexample"; HKO-local packets may appear only as separately labeled controls or sensitivity checks.
- Durable data contract before modeling: `research/sys-landscape.md` records the normalized `poly_id`/`state_id` core-plus-enrichment dataset shape over an ad hoc wide table or a fake full state graph, and fixes `poly_id` as a hash of canonical exact dual vertices.
- Candidate outcomes: a transferable signal gives a conjecture or guided search strategy; no signal or only non-transferable structure supports the hostile-landscape conclusion.
- Dependencies: random/polytope datasets are available now; use current local-maxima datasets immediately and extend to LICCA-returned local maxima if those artifacts arrive in time.
- First implementation packet: add a `sys-*` converter that loads the existing producer JSONLs, enriches them, and writes the datascience tables without forcing an intermediate full state graph.
- Landed in this worktree as the `sys-dataset` pipeline; the current output surface is `polytope-table.jsonl` plus `observation-table.jsonl`, with `282` rows in each table on the refreshed committed producer data. The old `287` trace-event count is still useful only as an internal aggregate folded into observation-level trajectory features, not as a produced table.
- Bounded pass landed in `experiments/sys-landscape/datascience/methods/feature-pattern-search/` with `feature_geometry`, `feature_skeleton`, `feature_omega`, sigma-local `feature_orbit`, and observation-keyed `feature_trajectory` blocks plus refreshed summary plots. The markdown ledger surfaces now exist in `research/sys-landscape-toolbox-audit.md` and `research/sys-landscape-datascience/method-ledger.md`; the remaining gap is method-by-method population and thesis-use judgment, not file creation.
- Follow-up packet landed: fixed-`F` endpoint and random-baseline cache rows now persist bounded `orbit_scalars`; rows without persisted `orbit_scalars` now stay missing rather than triggering an on-the-fly KKT fallback.
- Follow-up packet landed: bounded `feature_face_geometry` now evaluates edge-length and facet-3-volume summaries in the `vol(K)=1` convention; it helps within-random (`R^2=0.3847` ridge, `0.7009` RF) and still adds only a small endpoint-only signal (`0.1030` ridge, `0.1218` RF), with transfer still strongly negative.
- Follow-up packet landed: the geometric magnitude packets now use the `vol(K)=1` convention, with bounded `feature_face_symplectic` using ridge-polygon symplectic-area summaries normalized by `vol(K)^(1/2)`; it is strong within random and becomes the strongest non-metadata endpoint-side block so far, but it still fails transfer in both directions. Record that other symmetry-aware normalizations remain possible future variants, and keep the summary's symmetry-status table explicit about which blocks are not translation-invariant or not `Sp(4)`-invariant.
- Result: cheap geometry helps within the random regime, richer orbit/KKT scalars improve the random `orbit` block further, sigma-local orbit features still help the endpoint regime more than the other non-metadata blocks, trajectory aggregates from fixed-`F` step logs stay near-null, metadata still beats every non-metadata block on endpoints, and the transfer surfaces stay strongly negative or become even more negative once the random packet carries search-level orbit scalars.
- Closure: record the negative result in `RESULTS.md` as evidence for the hostile-landscape interpretation; do not open a novel method-development line here without a separate thesis-scope decision.
- Canonical method-ledger target for the next clarification pass: `research/sys-landscape-toolbox-audit.md`.
- Next blocked direction for a future LICCA session: row count, not local code scaffolding, is now the main bottleneck if this line is reopened.
- Highest-value LICCA follow-up: generate many more endpoint rows for `gradient-ascent-general` and `gradient-ascent-products`, then `variable-f-ascent`; random baselines already show within-regime signal and have lower marginal value than endpoint packets.
- Current posterior from the bounded pass: more rows alone are unlikely to create a clean transferable random-to-endpoint heuristic, but LICCA-scale endpoint data is still plausibly high-value for revealing stronger endpoint-only structure or for making the current negative transfer result much more decisive.
- Re-entry point: reuse the current datascience pipeline exactly as-is, refresh the canonical produce JSONLs / caches from LICCA outputs, rerun `sys-dataset`, then rerun `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py` before inventing richer models.

### [open] [group:landscape-ds] Post-scaffold hostile-landscape datascience split
- Settled:
  - The maintained datascience path is now `raw/* -> sys-dataset-core-tables -> sys-dataset-features -> methods/*.py`.
  - The first bounded method pass is thesis-usable negative evidence: within-regime signal exists, transfer from random to ascent endpoints is strongly negative, metadata still dominates endpoints, and trajectory summaries are near-null.
  - The current evidence-bearing method packets are the random/product baselines, fixed-`F` ascent, variable-`F` continuation, and the `feature-pattern-search` regression/classification/residual scripts.
- Polish or redo:
  - `research/sys-landscape.md` still describes the superseded `normalized-dataset` / per-feature-bin surface and should be rewritten around the merged `raw -> datasets -> methods` architecture.
  - `research/sys-landscape-toolbox-audit.md` is still a scaffold; it has the right columns and buckets but not the row-by-row hostile-landscape verdicts yet.
  - `research/sys-landscape-datascience/method-ledger.md` exists, but `M012` / `M013` still have undecided thesis use and skipped standard-toolbox rows are still missing.
  - `RESULTS.md` / thesis wording should eventually cite the audit and avoid repeating packet-level detail once the audit is populated.
- Ready agent work:
  - populate the audit row by row from the committed packets already listed in the ledger
  - add explicit skipped / inapplicable standard-toolbox rows instead of leaving the unused-method surface implicit
  - tighten stale notes and README text to match the merged datascience pipeline
  - do not reopen scaffold design unless the current `raw -> datasets -> methods` split is no longer enough
- Jörn decisions still needed:
  - whether `M012` regime classification is thesis-facing evidence, supporting-only, or spike-only
  - whether `M013` residual analysis is thesis-facing evidence, supporting-only, or spike-only
  - which skipped methods count as part of the thesis-relevant "standard toolbox" and therefore must appear explicitly in the audit
  - whether any HKO-local control should enter the datascience write-up or stay outside the main hostile-landscape surface
- Blockers:
  - LICCA rows are only a blocker for stronger density wording or a reopened endpoint-only modeling pass; they are not a blocker for the current negative-transfer thesis wording
  - `variable-f-ascent` still has weaker orbit-feature provenance than the fixed-`F` packets because older rows fall back to one-best-sigma orbit recovery; this is a blocker only if a later method needs equal orbit-feature richness across all endpoint families
- Dependency chain:
  - canonical producer packets -> `sys-dataset-core-tables` -> `sys-dataset-features` -> method scripts -> audit / ledger -> thesis wording
  - richer endpoint-only methods depend more on endpoint row count than on more feature engineering
  - witness-guided continuation / reduced-model search is a separate successor line downstream of witness-oracle instrumentation, not part of the current datascience closure packet

### [open] [group:landscape-ds] Populate the hostile-landscape audit from current packets
- Turn `research/sys-landscape-toolbox-audit.md` from a scaffold into the canonical per-method verdict surface for the thesis.
- Use the current attempted rows in `research/sys-landscape-datascience/method-ledger.md` as the starting inventory, then add skipped / inapplicable rows for standard methods the thesis might be expected to mention.
- For each method, record:
  - question
  - search surface
  - concrete repo evidence
  - validity guard or caveat
  - observation
  - inference
  - thesis use
  - reopen condition
- Acceptance check: a later session can answer "did we do this method, what did it show, and may the thesis cite it?" by reading the audit without reconstructing chat history.

### [Jörn] [group:landscape-ds] Decide the thesis-facing status of the remaining method packets
- Make explicit thesis-use judgments for:
  - `M011` bounded regression pass
  - `M012` regime classification
  - `M013` residual analysis
- Also decide which absent methods must appear explicitly as skipped in the audit:
  - PCA / projection methods
  - clustering / manifold learning
  - SVM / boosting / nearest-neighbor methods
  - neural-network methods
  - permutation / bootstrap inference, if they are meant to count as part of the datascience toolbox here
- Acceptance check: `research/sys-landscape-toolbox-audit.md`, `research/sys-landscape-datascience/method-ledger.md`, and the hostile-landscape paragraph in `RESULTS.md` agree on what is main evidence, supporting-only, spike-only, redo-before-thesis, or omitted.

### [future] [group:landscape-ds] LICCA-returned endpoint refresh and datascience rerun
- Trigger: new canonical `gradient-ascent-general`, `gradient-ascent-products`, or `variable-f-ascent` JSONLs / caches land from LICCA.
- Refresh path:
  1. replace the canonical endpoint artifacts
  2. rerun `sys-dataset-core-tables`
  3. rerun `sys-dataset-features`
  4. rerun `experiments/sys-landscape/feature-pattern-search/analyze.py`
  5. rerun `analyze_regime_classification.py` and `analyze_residual.py` if the row-count jump is large enough to matter
- Goal: strengthen or weaken the endpoint-only / density wording with more rows, not invent new scaffold or feature families first.
- Acceptance check: updated audit / claim surfaces state only what the enlarged endpoint dataset supports.

### [future] Systematic landscape analysis
- Gradient flow convergence, local maxima below sys=1, random noise effects.
- Partial data in gradient-ascent experiments.
- Witness-search successor line: `research/sys-landscape.md`
- Local unblocked queue after the bounded hostile-landscape closure:
  1. only after richer columns: trajectory/state-graph methods beyond the current scalar `feature_trajectory` block
  2. optional local refinement: explain or normalize the new face-symplectic block if a cleaner endpoint-side interpretation is needed before LICCA rows arrive
- LICCA-blocked queue:
  1. more `gradient-ascent-general` endpoint rows
  2. more `gradient-ascent-products` endpoint rows
  3. more `variable-f-ascent` endpoint rows
- Restart rule for future sessions: if new LICCA rows exist, refresh the canonical produce JSONLs / caches and rerun `sys-dataset` and `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py` before inventing new model families.

## [open] Computing capacity

Instrument development. Results promote to `crates/symplectic/`.

### [done] [2026-03] 4a. Algorithm comparison (ablation, benchmark, profiling)
- A2 pruning: ~1078x speedup at F=10. Construction dominates for F<=10 (80-92%).
- `experiments/verification/algorithm-comparison/`

### [done] [2026-03] 4c. Capacity axiom validation
- All 6 axioms pass. The local-first post-refactor follow-up is now split into `all-minimum` (trusted minimum-set validation) and `orbit-recovery` (geometry-only recovery on trusted rows); both canonical packets pass on 2026-04-17.
- `experiments/verification/correctness/`

### [open] [group:data-refresh] Post-merge stale canonical dataset refresh after Rust cleanup merge
- Trigger: `rust-convention-cleanup` merged on 2026-04-22 after a pre-merge pass that found many experiment generators newer than their committed canonical `.jsonl` outputs.
- Default rule from `$pre-merge`: this was not a data-refresh branch, so the code merged first and the canonical dataset refresh is now a scheduled follow-up rather than an in-branch blocker.
- Refresh families flagged during pre-merge:
  - `experiments/combinatorial-cells/{boundary-characterization,cell-widths,convexity,multiple-crossings}`
  - `experiments/hko-local-maximum/{cut-and-ascent,gradient-analysis,lagrangian-boundary,perturbation-neighborhood,sage-validation,second-order}`
  - `experiments/numerics/{algebraic-exactness,sage-feasibility}`
  - `experiments/numerics/gradient/{numerics,numerics-edge-cases,numerics-subdifferential}`
  - `experiments/sys-landscape/{pentagon-rotation-formula,random-sample,random-product-sample,rejection-calibration,variable-f-ascent}`
  - `experiments/verification/{all-minimum,correctness,orbit-recovery}`
  - `experiments/verification/algorithm-comparison/{ablation,benchmark}`
- Execution rule:
  - refresh only the canonical tracked outputs for these existing filenames
  - keep smoke outputs untracked
  - if a family turns out to be intentionally frozen or superseded, record that explicitly instead of silently skipping it
- Acceptance check:
  - code-vs-data commit dates are no longer stale for the retained canonical outputs above
  - any intentionally skipped family has a written reason in `TASKS.md` or the relevant research note

### [open] [group:library] Capacity/orbit result API architecture
- Problem: the public library API should expose one clear algorithm family with nice names: `ehz_capacity` (auto), `ehz_capacity_pruned`, `ehz_capacity_unpruned`, and `ehz_capacity_billiard`. Non-default consumers still need richer algorithm control: all certified candidate orbits, all minimum-action simple orbits within tolerance, near-active witnesses, pruning/solver diagnostics, and recovered primal trajectories. That richer control should come from explicit building blocks, not from a second overlapping family of assembled algorithm entrypoints.
- Durable design note: `.codex/reference/repo-maintainability/design/hk2017-result-api-plan.md`.
- Architecture note from the first repo-level doc pass: the old `EhzResult` layering was accidental complexity. The root and explicit `ehz_capacity*` family now returns `OrbitSearchResult`; geometric-orbit recovery is still a separate pass; derivatives stay as low-level functions plus orbit-level helpers rather than a heavyweight report object.
- Execution status on `capacity-result-api-exec`: Packet 1 scaffold landed as `crates/symplectic/src/algorithms/orbit_search.rs` with shared enums/types for the new result layer. Packet 2 and the current follow-up refactor now treat the intended API split as: one router family `ehz_capacity`, `ehz_capacity_pruned`, `ehz_capacity_unpruned`, `ehz_capacity_billiard`, plus explicit building blocks for sigma traversal, one-sigma solve, and aggregation. Concretely, the library now has shared `solve_orbit_sigma(...)`, shared exact-fallback / guarantee-mode helpers, a public `aggregate_orbits(...)` building block for non-default consumers, and algorithm-specific sigma traversal helpers under `algorithms::hk2017` and `algorithms::billiard`. The root `ehz_capacity*` family returns `OrbitSearchResult`, `OrbitSearchResult` exposes scalar convenience accessors (`capacity()`, `best_sigma()`, `best_beta()`, `best_subset()`), and the legacy deeper HK2017 `EhzResult` family has been deleted. `orbit_recovery::recover_and_verify(...)` now consumes `OrbitKktData` rather than a root-shaped summary object. `crates/symplectic/src/derivatives.rs` exposes `OrbitGradientA`, `ClarkeSubdiffA`, `DerivativeError`, a `KktResult`-level derivative helper, an `OrbitKktData`-level derivative helper, and primitive Clarke directional-derivative helpers; the migrated buildable packages now include `exp-combinatorial-cells`, `exp-sys-landscape`, `exp-hko-local-maximum` (`hko-second-order`, `hko-cut-and-ascent`, `hko-gradient-analysis`, `hko-lagrangian-boundary`), the full `dev-gradient` package, `axioms-orbit-recovery`, `visualization`, `dev-algorithm-comparison` (`benchmark`, `profile`, `ablation`), `dev-capacity-validation` (`correctness`), and `dev-numerical-analysis` (`q-error`, `unknown-predicates`). `hko-gradient-analysis` now uses `OrbitKktData` and the orbit-level derivative helper while intentionally keeping its experiment-local stricter `beta > EPS_BETA_POSITIVE` valid-orbit policy; `hko-second-order` now also uses `OrbitKktData` and the orbit-level derivative helper, removing its per-orbit KKT re-solve for multiplier recovery; `omega-hypothesis` now uses the `KktResult`-level derivative helper instead of raw `beta/q/mu/sigma` glue; the repeated all-valid-orbit summary helper in `exp-combinatorial-cells` now lives once in `experiments/combinatorial-cells/src/lib.rs` instead of four binary-local copies; the repeated stricter-orbit collector in `exp-hko-local-maximum` now lives once in `experiments/hko-local-maximum/src/lib.rs` instead of two binary-local copies; and the `dev-gradient` package now centralizes its strict-orbit enumeration and safe-wrapper helpers in `experiments/numerics/gradient/src/lib.rs`. `unknown-predicates` now uses the new interval-rich result surface (`min_action`, `min_action_lower`, `min_action_upper`, `beta_min`, `has_unknown`) rather than the deleted `capacity_uncertain`/`numerical_gap()` output contract. The legacy billiard scalar adapter path (`billiard_capacity`, `BilliardResult`, `collect_legacy_capacity(...)`, `legacy_solution_from_orbit(...)`) and the dead `capacity_accumulator` module have now been deleted; the ablation experiment uses its own local result row type instead of borrowing library scalar-result structs. The root `ehz_capacity*` family deliberately stays on the unresolved f64 search surface. Non-default consumers that want stronger guarantee modes are expected to use the building blocks directly rather than calling a second overlapping assembled family. The current open limitation beyond that cleanup is still explicit projected-backend support: `OrbitSearchError::UnsupportedBackend` reports that `crates/symplectic/src/kkt/projection_solver.rs` does not yet expose the payload/error-bound contract needed by the shared result layer.
- Near-term follow-up worth doing without another design round: use the pre-merge review pass to trim any remaining stale migration wording and wrapper-shaped test names so the branch reads like the settled router/building-block architecture rather than the migration path that produced it.
- Pre-merge Phase 3 note: many touched experiment directories now have code newer than their committed `.jsonl` outputs because this branch migrated call sites and report schemas without rerunning every dataset. Treat those datasets as stale until a dedicated refresh packet reruns them; the immediate blocker found by the review pass was `unknown-predicates/analyze.py`, which now supports both the old committed schema and the new generator schema so the analyzer remains usable during that gap.
- Concrete post-merge cleanup found during pre-merge: rerun and recommit the canonical outputs for the touched experiment families that now have schema/code drift (`verification/correctness`, `verification/algorithm-comparison/{benchmark,ablation}`, `numerics/unknown-predicates`, and any product-reporting runs that persisted the deleted billiard scalar shape). Once `unknown-predicates.jsonl` is regenerated, delete the temporary old-schema fallback in `experiments/numerics/unknown-predicates/analyze.py`.
- Concrete smoke-fix follow-up found during pre-merge: keep adding local `create_dir_all(...)` / untracked-smoke-output guards when experiment binaries are touched. This review pass found one real missing directory-creation bug in `omega-hypothesis`, and the repo-wide smoke-output packet below should continue from that evidence rather than reopening the capacity-result design discussion.
- Explicit defer: projected-backend support is blocked on Jörn’s math time. Do not open an implementation packet for `OrbitSolveBackend::Projected` or try to “just wire it through” from the current projection solver until Jörn is available to help decide the payload/error-bound contract and the solver-to-result correspondence.
- Current root-policy direction: ordinary consumers call `ehz_capacity`. On Lagrangian products this routes to billiard by default; explicit HK2017-on-products is mainly for verification and cross-algorithm comparison.
- Remaining follow-up questions after the landed refactor: what exact tolerance the all-minimum validation packet should use when comparing interval-valued minima; whether the surviving deep expert paths (`hk2017::orbit_recovery`, `billiard::facet_classification`, low-level KKT assembly helpers) should stay experiment-facing through the thesis push; and whether the projected backend can later satisfy the shared `OrbitKktData` payload contract.
- To-discuss item from the split all-minimum packet review: what `passes_validation` should mean when the retained minimum set still has wide or one-sided action intervals. Current packet semantics treat those intervals as diagnostics while passing on scalar agreement + expected multiplicity; if the intended claim is stronger than “current result-layer output plus diagnostics,” tighten this into an explicit interval-resolution rule.
- Acceptance check for the design/execution session: the durable design note and execution packets name the public functions/types, migration path for copied experiment instrumentation, verification commands, explicit non-goals for the thesis push, and the required doc updates (`TASKS.md`, `ARCHITECTURE.md`) plus the rerun surface that will demonstrate the refactor actually worked. Current verification on `capacity-result-api-exec` includes the Packet 2 collector/building-block checks `cargo build -p symplectic --release`, `cargo test -p symplectic --release --lib`, plus the Packet 3 checks `cargo test -p symplectic --release derivatives::tests -- --nocapture`, `cargo build -p exp-combinatorial-cells --release`, `cargo build -p exp-sys-landscape --release`, `cargo build -p exp-hko-local-maximum --release`, `cargo build -p dev-gradient --release`, `cargo build -p dev-capacity-validation --release --bin axioms-orbit-recovery`, and `cargo build -p visualization --release`.
- Stop condition: if the design requires resolving tube correctness, changing solver semantics, or committing to a broad future-proof public API guarantee beyond the thesis-push surface, defer the implementation and keep validation experiment-local.

### [done] [2026-04-17] [group:verification] All-minimum simple-orbit validation (local-first)
- The local-first packet is now split into two experiments with shared target-pool logic in `experiments/verification/src/lib.rs`.
- `experiments/verification/all-minimum/` owns minimum-set validation: it recomputes minima from the polytope via pruned HK2017 sigma enumeration, `solve_orbit_sigma(...)`, and `aggregate_orbits(..., OrbitGuaranteeMode::MinimaSafe)`, then writes trusted minimum-orbit rows to `all-minimum-orbits.jsonl`.
- `experiments/verification/orbit-recovery/` now consumes those trusted rows, rebuilds one-sigma KKT data, and checks geometric recovery with `recover_and_verify(...)` only.
- Current outputs are `experiments/verification/all-minimum/{all-minimum,all-minimum-orbits}.jsonl` and `experiments/verification/orbit-recovery/{orbit-recovery,orbit-recovery-orbits}.jsonl`, plus matching `smoke-*` files. Current package reasoning, decisions, and next steps live in `research/verification.md`.
- Explicit tolerance choice landed in the experiment: use a tiny action-tie cutoff `1e-12` after the broader `MinimaSafe` candidate collector. Reason: `gap = 0.0` undercounted the known simplex multiplicity. The summary rows keep the broader candidate interval visible so loose lower bounds remain observable.
- Canonical local-first run on 2026-04-17: `all-minimum` passes 28/28 selected polytopes and writes 469 trusted minimum orbits total; `orbit-recovery` then rebuilds and recovers 469/469 trusted minimum orbits successfully. Expected multiplicities are hit on the documented symmetric witnesses (`simplex = 6`, `hypercube = 2`). The largest observed sigma-level minimum set in this packet is `hko_pentagon = 412`, with observed action spread `4.44e-15`.
- Discussion item left intentionally open after pre-merge review: some `all-minimum` rows still have broad `MinimaSafe` intervals or missing finite upper bounds on individual retained rows, while the current pass/fail rule treats those as diagnostics rather than failures. Keep that semantics visible in tracker/docs until Jörn decides whether the packet is “trusted output + diagnostics” or “interval-resolved certification.”
- Current dataset surface for diversity: 7 known polytopes (excluding crosspolytope), 8 random shared-cache facet-count strata, 10 lagrangian-product shared-cache pair strata, and 3 correctness-derived extras (`scaled`, `transformed`, `perturbed`). Smoke stays infrastructure-only: `simplex`, `hypercube`, `lagrangian_triangle_product`, one random cache row, and one transformed correctness row.
- Stress-test escalation remains separate: if Jörn later wants higher-F stress or theorem-bundle validation, widen this into a capacity-and-lemma packet rather than bloating the local-first validator in place.

### [done] [2026-03] 4b-partial. Q error and KKT inertia
- 1.13M nodes, worst E=2.9e-11. Empirically exact at f64.
- Eigenvalue inertia formula holds for 6/7 polytopes, 5 mismatches are threshold artifacts.
- `experiments/numerics/q-error/`, `experiments/numerics/kkt-inertia/`

### [open] [group:numerics] 4b. Numerical error bounds (verify-numerics)
- `formal/numerics/error-bounds.tex` Parts I+II complete. Proven Q error bound, eta bound for well-conditioned problems.
- 14 previously-failing tests now pass (329 pass, 0 fail).
- Rationale for current state: degenerate orbits are never capacity-achieving, so final capacity comes from well-conditioned orbits with proven low error. The remaining gap is publication/writeup polish rather than a blocker for the two main thesis results.
- Remaining open surface: Part III (f64 algorithm description), a thesis-facing treatment of the LP null-space search case (39 natural-data violations in cases with near-zero eigenvalues), and the GAP in `cor:taylor-structure` (needs Jörn).
- Finish-mode status: publication polish and thesis confidence, not a prerequisite for the two main thesis results. Leave explicit caveats or cut proof ambitions rather than opening new solver work unless Jörn says the thesis payoff is worth the time.
- `experiments/numerics/error-bounds/`, `research/numerics-error-bounds.md`

### [open] [group:numerics] Projection solver
- 5-step algorithm: (1) solve equality constraints → (m-5)-dim affine space, (2) project H → reduced Hessian, (3) eigendecompose → null directions, (4) beta>0 as LP on projected null space, (5) recover multipliers.
- Basic implementation in `kkt/projection_solver.rs`. The remaining asks here are mathematical rigor and ablation/comparison writeup; treat broad promotion or refactor as defer/future polish unless a bounded local-validation or stale-note cleanup slice directly supports retained thesis text.
- Post-Kai: do not let this row reopen a broad solver-development program during thesis closeout.
- `research/numerics-error-bounds.md`

### [open] [group:numerics] Beta-LP unification
- Replace `find_positive_beta_1d`/`find_positive_beta_nd` with single LP: maximize min_j beta_j subject to beta = beta_0 + V*alpha.
- Previous branch deleted (tip `7ca81b53` has salvageable design: unified function, Type A/B/C eigenvector classification).
- Thesis/code tension: thesis proves rank-deficient pairs are redundant (discard); code searches null space for beta>0 on *near*-singular systems (pseudoinverse beta_0 may have beta_i < 0 from noise; null-space shift recovers feasibility without changing Q). Not contradictory but needs explicit documentation.
- Current blocker for implementation work: Jörn mathematical judgment on whether filtering Type A directions is justified.
- Finish-mode status: documentation of the tension is useful polish; keep implementation unification deferable unless Jörn's judgment lands and a bounded session can finish and verify it without delaying thesis closure.

### [open] [group:numerics] Solver numerical formal writeup
- Per-module formal files for SVD, condition numbers, LU, eigendecomposition stability.
- Multiple modules use SVD without shared error analysis. For the thesis-close window, this row is only about the pieces that directly support thesis text or current validation.
- Finish-mode status: defer full per-module numerical formalization instead of treating it as required numerics closure work.

### [done] [2026-04] Crosspolytope capacity
- c_EHZ = 4.0 (same as hypercube), sys=0.75. Exhaustive search through m=13.
- `experiments/crosspolytope/`

### [future] Crosspolytope optimality proof
- Minimizing orbit has clean structure (uniform beta, max omega). Symmetry argument may avoid exhaustive enumeration.
- Defer for thesis push: this would strengthen the standalone crosspolytope result, but it does not support a main thesis claim and is lower priority than HKO, numerics, and writeup-closing work.
- Acceptance: replace the current "exhaustive through m=13" caveat with a proof that the symmetric m=4 orbit is globally minimizing, or explicitly keep it as future work if no short argument appears.

### [blocked] [group:tube] Tube vs HK2017 benchmark
- Blocked on: Jörn writing down the tube algorithm + rotation formula + correctness proof (first task in the tube subtree).
- Once formula + proof exist: one session wires the formula into `crates/symplectic/src/algorithms/tube/mod.rs` (the current file is a misleadingly-named wrong placeholder), then a sibling session runs a benchmark harness that reads the polytope database, runs tube + HK2017 per entry, compares c_EHZ values + wallclock + memory, produces a report.
- Goal: empirically decide "switch to tube if/where it beats HK2017" and cross-compare excessively for correctness verification.
- Re-plan trigger: Jörn's write-up lands → stage the wire-in and harness work as concrete items.

## [open] Thesis

thesis/ is stale (see `thesis/appendix-rewrite-notes.md` and `thesis/numerical-story.md`). Most work here is blocked on restructuring decisions.
tube-algorithm.tex and appendix-numerical.tex TODOs are about math correctness, independent of restructuring.

### [open] [group:pm] Writer-ready milestone
- Meaning: the remaining mainline work is writing `thesis/` from already-chosen thesis content.
- Thesis-external work may still happen after this milestone, but only when writing or thesis-driven reexamination makes the exact work concrete.
- Do not mark this done by corner-cutting. It is false if thesis-external work that is already foreseeable and worth doing before writing is still knowingly being postponed.
- Current status on 2026-04-20: false.
  - `Thesis restructuring` is still open, so the writing target is not yet stable enough to classify the thesis-external boundary cleanly.
  - The thesis-relevant packets outside `thesis/` have not yet been classified as `must finish before writer-ready`, `allowed after writer-ready only if writing makes it concrete`, or `defer/future`.
- Revisit this row only after `Thesis restructuring` and `Writer-ready boundary classification` update `TASKS.md` enough that the pre-writing / contingent / defer split is visible from repo state.
- Acceptance check: every still-relevant thesis-external packet has an explicit side of the writer-ready boundary, and anything left outside `thesis/` after the milestone is contingent work discovered during writing rather than work we already knew should happen first.

### [Jörn] [group:pm] Writer-ready boundary classification
- For each thesis-relevant packet outside `thesis/`, decide whether it is `must finish before writer-ready`, `allowed after writer-ready only if writing makes it concrete`, or `defer/future`.
- Use one question only: is this work already foreseeable and worth doing before writing, or does it become concrete only during writing or thesis-driven reexamination?
- This is classification, not execution. If the classification exposes missing pre-writing work, keep or create the corresponding task row instead of hiding it under the milestone.
- Likely inputs include retained `RESULTS.md` items, thesis-facing correctness or alignment passes, figure/table source preparation, and any open experiment/formal/code packet still on the thesis path.
- 2026-04-21 draft default if agents proceed before a full outline lands:
  - `must finish before writer-ready`: thesis-facing provenance cleanup (`S1`), HKO claim-strength freeze, hostile-landscape claim-strength freeze, numerical appendix route freeze, and explicit status for `lem:cap-derivative` / `lem:vol-derivative`.
  - `allowed after writer-ready only if writing makes it concrete`: HKO Packet 3 completion, figure inventory, AI/process reflection, and thesis-style experiment prose for surviving sections.
  - `defer/future`: new LICCA runs unless results are already about to land, tube implementation, pentagon rotation proof, and the full Appendix A redesign / combinatorics-theorem route.
- Acceptance: the relevant thesis-external packets are classified in `TASKS.md`, and the writer-ready row can then be checked locally from those classifications.

### [done] [2026-04-24] [group:pm] 2026-04-21 to 2026-04-23 pre-writing packet queue superseded
- Superseded by the finish-mode reset at the top of this file. The row no longer owns active integration.
- Durable outcome: the packet queue exposed the right current buckets, but the live queue dates and ownership are stale.
- Preserved follow-ups:
  - `HKO theorem/evidence/blocker compression`, `Hostile-landscape retained-claim compression`, and `Numerical appendix route freeze` remain as separate rows below.
  - `S1` / dataflow cleanup is now part of the broader reproducibility/artifact-truth bucket, not a live prerequisite hidden in this old queue.
- Stop condition preserved for future packets: if a packet opens a new theorem program, new experiment family, or a change to LICCA ownership, stop and re-plan instead of guessing.

### [open] [group:writeup] HKO theorem/evidence/blocker compression
- Goal: compress the HKO material outside `thesis/` into a thesis-safe theorem/evidence/blocker split that matches current exact and numerical artifacts.
- Scope: `research/hko-local-maximum.md`, `research/hko-local-maximum-exact-clarke.md`, and thesis-relevant `formal/hko-local-maximum/**` edits only if they are needed to reflect the retained claim honestly.
- Packet shape: one agent can own this top-to-bottom after the claim-strength boundary is fixed; a follow-up review pass is useful but not required to start.
- Verification: `cd formal && latexmk` if formal files move; if witness-facing docs or contracts change, also run `cd experiments/hko-local-maximum/exact-clarke && python3 build_widened_seed_witness.py && sage verify_widened_seed_witness.sage`.
- Stop condition: if the packet needs Packet 3 representative coverage, a new proof idea, or a stronger theorem statement than the current artifacts license, stop and weaken/caveat instead of extending the math program.

### [open] [group:writeup] Hostile-landscape retained-claim compression
- Goal: compress the hostile-landscape material outside `thesis/` into the retained thesis-safe claim set: bounded negative evidence, non-transferable heuristics, no strong density wording without new returned LICCA artifacts.
- Scope: `research/sys-landscape.md` and thesis-relevant `formal/sys-landscape/**` files touched by the retained claim set.
- Dependency: do this after `S1` so the data/provenance surface is stable while claims are being compressed.
- Packet shape: one agent can own this top-to-bottom after the retained claim boundary is fixed; reviewer value is mostly stale-claim checking.
- Verification: `cd formal && latexmk` if formal files move.
- Stop condition: if the packet wants a new density claim, new feature-search method, or a new LICCA obligation, stop and keep that work future/pending instead.

### [open] [group:numerics] Numerical appendix route freeze
- Goal: decide whether the numerical appendix stays on the current critical path as a minimal honest appendix or whether the larger Appendix A redesign/combinatorics-theorem route is explicitly deferred.
- Inputs: `thesis/appendix-rewrite-notes.md`, `thesis/numerical-story.md`, `thesis/appendix-numerical.tex`, and the retained thesis claim set in `RESULTS.md`.
- Agent scope: draft the route decision surface and the fallback wording that keeps the thesis truthful without opening a new theorem program.
- Jörn gate: if both routes remain plausible after the draft, Jörn chooses which route to keep on the mainline.
- Acceptance: `TASKS.md` records the current route, what is explicitly deferred, and what remaining appendix work is still load-bearing for the thesis.

### [Jörn] [group:writeup] Thesis restructuring
- Current content stale. Decisions needed: chapter structure, what content survives, what gets rewritten.
- a_i replaces (n,h). Sign conventions changed. Simplification theorem ordering changed.
- Blocks: S0, experiment writeups, experiments chapter, introduction, conclusion.
- See `thesis/appendix-rewrite-notes.md` and `thesis/numerical-story.md` for current narrative notes.

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
- `crates/symplectic/src/algorithms/tube/mod.rs`

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
- Outcome recorded in the finish-mode section: Kai agreed the two main `RESULTS.md` result blocks are sufficient for thesis completion; optional polish is not required unless it fixes thesis correctness, cited reproducibility, or submission blockers.

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

### [open] [group:submission] Final assembly (checklist-driven, target 2026-05-14)
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
- Closure note: the first pass merged `paranoia-numerics`, fixing 19 files across experiment logbooks + `experiments/numerics/error-bounds/tests.rs` + `experiments/numerics/unknown-predicates/main.rs`.
- This row is now a closure ledger for stale-claim reconciliation, not a broad open numerics program. The old top-level audit report has been retired after migration into current tracker/design-note ownership.
- Profiling closed as a live issue: `experiments/verification/algorithm-comparison/profiling/logbook.jsonl` keeps the bad 2026-04-04 zero-duration row as history, but the 2026-04-15 `f5d4ba18` row + `profile.jsonl` are the first usable post-fixture-removal baseline.
- 2026-04-16 to 2026-04-17 tracker reduction: the checkpoint cleanup closed the design-note-only rows for combinatorial-cells convexity, numerics error-bounds prose, crosspolytope timing wording, cut-and-ascent timing wording, and the local-first orbit-recovery packet for the current thesis-facing scope.
- Live follow-up packets:
  - `experiments/hko-local-maximum/perturbation-neighborhood/`: split historical `pentagon-perturb.jsonl` findings from current `data/{smoke,licca}-eps-*.jsonl` analyzer outputs; update stale task/design min/max wording.
- Closed in checkpoint `9d55e7f8`:
  - `experiments/combinatorial-cells/convexity/`: design note now matches committed JSONL (2800 rows, 2661 successful midpoint constructions, 1558/1558 product transition failures, 0/1103 random transition failures).
  - `research/numerics.md`: stale M1 wording marked historical; stale `make smoke` / `make full` commands removed from the live topic-note surface.
  - `research/crosspolytope.md`: elapsed-time wording reconciled to the committed `1095.1s` JSONL result vs historical `1112.8s` console/table total.
- `research/hko-local-maximum.md`: stale `~10s per trial` scale-up estimate removed from the live continuation-note surface.
- Stop condition for this closure bundle was reached. Keep any remaining work on dedicated packets instead of reopening this row into another broad numerics sweep.
- Finish-mode status: high-value polish because it protects thesis claims. Prefer shallow evidence repairs and stale-note fixes; otherwise weaken or qualify claims instead of rerunning broad experiments.

### [open] [group:paranoia] Data freshness and rerun matrix
- This row now records the finished rerun triage surface. The source packet (`/tmp/4.md`) originally asked for a prioritized table of evidence gaps with columns `claim`, `current data`, `missing data`, `local vs LICCA`, `estimated runtime`, `job/script readiness`, and `thesis impact`; the useful intermediate output was the LICCA script-readiness commit `fc7991e6`.
- Do not redo the LICCA script audit first: `fc7991e6` added `job-smoke.sh`, fixed `CARGO_TARGET_DIR` binary paths, ran the three smoke scripts, and updated the active LICCA handoff notes. Remaining LICCA-side check is external: current repo layout under `~/msc-math` on LICCA before `sbatch`.
- 2026-04-17 local refresh packet outcome:
  - in-scope canonical local datasets were rerun under current code and committed derived artifacts were regenerated from refreshed JSONL in the same pass;
  - smoke/default safety sweep landed for the touched binaries, so smoke runs now write untracked `smoke-*.jsonl` outputs unless the caller explicitly targets canonicals;
  - `bash scripts/dataflow.sh` now regenerates `DATAFLOW.md` with the current declared-entrypoint artifact DAG plus a worktree timestamp audit;
  - current audit result: no declared producer/input stale edge remains in that entrypoint/header audit; the remaining non-canonical tail is tracked smoke outputs plus detached historical tracked JSONL such as `experiments/verification/orbit-recovery/polytopes.jsonl`.
- Current matrix snapshot (2026-04-17):

| row | current data | missing data / blocker | local vs LICCA | runtime / readiness | thesis impact | recommendation | deadline bucket |
| --- | --- | --- | --- | --- | --- | --- | --- |
| all-minimum + orbit recovery | Split local-first packet landed: committed `all-minimum*.jsonl` and `orbit-recovery*.jsonl`, separate design notes, refreshed recovery plot, shared target-pool helper, and the old `orbit-recovery/polytopes.jsonl` mirror is now gone from the active pipeline | no blocker for the current local-first packet; only future question is whether thesis prose should quote any observed multiplicity counts beyond the documented simplex/hypercube witnesses | local-first | local binaries + analyzers exist; `axioms-all-minimum --full` passes 28/28 in ~10.3s and `axioms-orbit-recovery --full` passes 28/28 in ~1.5s; no LICCA script needed | verification/polish task for both minimum-set and geometry-only recovery work is now covered by a committed local packet | do not rerun unless the code changes or Jörn wants a larger stress packet | done for the current thesis-facing local-first scope |
| perturbation neighborhood | historical `pentagon-perturb.jsonl` kept; smoke/default outputs were moved to untracked `smoke-eps-*.jsonl`; LICCA production files still absent | production-scale 10k-per-bucket run | needs LICCA | `job.sh` + `job-smoke.sh` exist; smoke path now intentionally stays untracked; production script budgets 30 min and only needs the external LICCA repo-layout check | load-bearing only if the thesis keeps a large-N HKO neighborhood falsification claim | needs LICCA | finish-mode default is pending/future unless Jörn chooses the LICCA action or results already exist with low integration cost |
| convexity | committed `combinatorial-boundaries-convexity.jsonl` has 2800 rows and the design note matches it | none for current thesis wording | local if ever rerun | binary + analyzer exist; no current need to execute | supports the hostile-landscape / non-convex-cell interpretation, but current committed data already covers the claim | do not rerun because it would only move already-synced counts/figures | defer/future |
| numerics error-bounds note | current binaries/tests exist; stale note wording already fixed | none for the paranoia row; deeper solver work remains a separate numerics task | local if reopened | local commands documented; not a LICCA surface | prose clarity only; not a missing evidence artifact after the checkpoint cleanup | do not rerun because the stale issue was note-only | defer/future |
| crosspolytope timing | committed `crosspolytope.jsonl` records `1095.1s`; note already labels `1112.8s` as historical console/table output | none for current thesis wording | local if intentionally recomputed | binary exists, but rerun is a long recomputation of an already-established standalone result | standalone established result already in `RESULTS.md`; recomputation risks moving a non-essential timing number | do not rerun because it would move a non-load-bearing timing figure | defer/future |
| cut-and-ascent timing | committed `cut-and-ascent.jsonl` has 20 preliminary rows; stale estimate already removed from the note | no trusted current scale-up timing estimate; bigger run would become a new experiment scope | local if reopened | binary exists; per-trial budget in code is 180s, but no current tracker requirement to measure it | current thesis use is only the empirical `0/20 improved` evidence; larger-F validation is already future work | do not rerun now; keep the current preliminary claim and defer broader sampling | defer/future |

- Seed rows from the paranoia closure shortlist: orbit recovery, perturbation neighborhood, combinatorial-cells convexity, numerics error-bounds note, crosspolytope timing, and cut-and-ascent timing. Add non-paranoia stale evidence only when it affects `RESULTS.md` or an active thesis claim.
- Immediate PM consequence from the matrix: only one thesis-facing row still needs action here. Perturbation neighborhood needs the external LICCA submission step. The remaining rows should not absorb another rerun session unless a separate research reason appears.
- Repo-wide follow-up discovered during pre-merge on 2026-04-16: sweep other experiment binaries for default or `--smoke` code paths that still overwrite production `.jsonl` outputs or production-side cache overlays. Treat that as broader data-hygiene follow-up, not as part of the remaining thesis-facing numerics evidence surface.
- Stop condition: if the matrix recommends a new large experiment family rather than a rerun of an existing package, stop for Jörn's thesis-priority decision.
- Finish-mode rule: classify rows into `mainline thesis`, `weaken/reword`, or `future/follow-up` in addition to local vs LICCA. Do not let the matrix create a new required coverage obligation.

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
- Finish-mode status: fix items that would make the thesis wrong or unreproducible; leave code-side cleanup as future work.

### [open] [group:pm] Repo maintainability / architecture program
- Durable planning note: `.codex/reference/repo-maintainability/design/main.md`
- Purpose: gather repo facts first, then prepare the Jörn review surface for the broad maintainability refactor. Separate `observed facts`, `open architecture decisions`, and `candidate execution packets` before freezing a multi-session DAG.
- Seeded facts already recorded in the note: before this session there was no top-level `ARCHITECTURE.md`; experiments already depend on deep library paths; topic packages already have `src/lib.rs` helper crates; the 170-row polytope cache is mirrored in three identical files; `variable-f-ascent` cache is intentionally local.
- Discovery artifacts now written:
  - `.codex/reference/repo-maintainability/design/repo-facts.md`
  - `.codex/reference/repo-maintainability/design/import-surface-inventory.md`
  - `.codex/reference/repo-maintainability/design/shared-helper-inventory.md`
  - `.codex/reference/repo-maintainability/design/data-flow-inventory.md`
  - `.codex/reference/repo-maintainability/design/docs-navigation-inventory.md`
  - `.codex/reference/repo-maintainability/design/execution-constraints-inventory.md`
- Current documentation method: facts first, architecture prose second. The consolidated current-state fact base is `.codex/reference/repo-maintainability/design/repo-facts.md`; `ARCHITECTURE.md` should be derived from it instead of mixing discovery with policy.
- `ARCHITECTURE.md` now carries both the component/code architecture and the current persisted-data architecture. `scripts/dataflow.sh` regenerates `DATAFLOW.md`, which is the current declared artifact audit surface for producer/consumer and timestamp questions, while `AGENTS.md` remains the short repo map and `ARCHITECTURE.md` stays descriptive.
- Current phase: architecture-doc pass and capacity/orbit API decision surface reviewed. Next step is execution-packet planning and worktree setup for the approved shared result-layer direction (`hk2017`, `hk2017_unpruned`, `billiard` sharing one orbit/result layer with separate search frontends).
- Discussion order for the next phase: keep the design note as the source of approved API direction, then write and execute bounded packets incrementally instead of freezing the whole DAG upfront.
- Next PM action: commit the durable design notes, create a dedicated feature worktree, and start packetizing the refactor around shared core types/search frontends/consumer migration.
- Acceptance check: later sessions can resume from `TASKS.md` plus the note without chat history; the note names discovery packets, Jörn decision points, execution-packet template, and the current safe resume point.
- Stop condition: if the note starts implying API or data decisions that Jörn has not reviewed, keep them as options in the note instead of promoting them to tracker facts.

### [open] [group:library] Experiment-to-library algorithm surface audit
- Purpose: decide which experiment-grown algorithms are stable enough to promote, wrap, extract to a topic-local helper, or explicitly leave experiment-owned if that decision affects retained thesis text or cited reproducibility. This is a triage/design task, not a broad refactor permission.
- Evidence surface:
  - Rich HK2017 enumeration appears repeatedly as copied `ehz_capacity_instrumented` / `enumerate_all_orbits` code in HKO, combinatorial-cells, and gradient-validation experiments. This is the highest-value library API gap and is tracked directly by the "Capacity/orbit result API architecture" task.
  - `sys = c^2/(2 vol)` and `d_sys/da` are recomputed in several experiments using library `capacity_derivatives_a` and `volume_derivatives_a`. A small `systolic_ratio` / `sys_derivatives_a` helper could reduce drift, but only after Jörn is comfortable with the derivative lemma status.
  - Combinatorial-cell step-bound detection (`compute_step_bound_detailed`, including incidence, omega_0 sign, and dual-vertex degeneration events) is stable enough for shared experiment code; library promotion should wait until there is a public "combinatorial type/cell" API design.
  - Topic packages already have `src/lib.rs` entry points (`exp-combinatorial-cells`, `exp-hko-local-maximum`, `exp-numerics-gradient`), but some of those helper crates are still empty while shared routines remain copied across binaries. Extracting to `experiments/<topic>/src/lib.rs` is often cheaper and safer than immediate library promotion.
  - Projection-solver diagnostics and exact-QP validation are richer in `experiments/numerics/error-bounds/` than in the public library API. The library now has the fixed projection solver and exact KKT solver; stale experiment-local comments/copies should be pruned or relabeled before adding more solver APIs.
  - Experiments already import deep library paths such as `hk2017::permutations`, `hk2017::orbit_recovery`, and `kkt::saddle_point_solver`, so the audit should record which deep paths are intended expert surfaces versus accidental internals that later agents should avoid depending on.
  - Gradient ascent, wiggle/overshoot escape, add-facet/variable-F ascent, rotated-product sweeps, and crosspolytope symmetry reduction are publishable experimental methods or special computations, not general library algorithms for the thesis push.
- Finish-mode default: classify repeated helpers only when it unblocks validation, write-up, or agent navigation for retained thesis work. Defer broad migration of ascent/search heuristics, combinatorial-cell APIs, and crosspolytope-specific symmetry code.
- Acceptance check: a design note or short patch names each candidate as `promote now`, `extract to topic lib`, `experiment helper only`, `document stale copy`, or `future`, records the intended stable import path for anything shared, and gives one verification command per promoted or extracted API. If no code is promoted, close by linking this audit from the broad SWE polish bucket.
- Stop condition: if a candidate changes mathematical claims, proof obligations, or public solver semantics, stop for Jörn rather than promoting it as polish.

### [open] [group:data] Experiment data-flow audit and cache plan
- Purpose: map which experiments can reuse polytope/capacity/sigma datasets and which ones need experiment-owned intermediate data. This is the data-flow analogue of the algorithm-surface audit; do not start by moving `.jsonl` files.
- Current audit surface: `bash scripts/dataflow.sh` regenerates `DATAFLOW.md`
- Current cache evidence:
  - `crates/symplectic/src/database.rs` defines `PolytopeRecord` with rational dual vertices, rational vertices, optional volume/capacity, and optional `sigmas`; callers own path policy and there is no canonical mutable shared cache.
  - After the 2026-04-17 refresh packet, `experiments/combinatorial-cells/polytopes.jsonl` and `experiments/sys-landscape/datascience/produce/shared-cache.jsonl` are refreshed shared caches in active use. `experiments/verification/orbit-recovery/polytopes.jsonl` is no longer consumed by current orbit-recovery code and is now reported by the dataflow audit as a detached stale mirror rather than a canonical cache.
  - `experiments/sys-landscape/datascience/produce/continuation-cache.jsonl` is intentionally local and much larger than the shared cache: it stores many intermediate gradient-step polytopes so the shared family cache does not become a transient search log.
- Data-shape classes:
  - **Minimum action only:** random/product sampling, perturbation sweeps, rotated-product sweeps, correctness smoke data, and many landscape plots can reuse rows with dual vertices, volume, capacity, and sys.
  - **Best sigma at the minimum:** orbit recovery, omega-obstacle gradients, gradient-ascent derivative steps, and some combinatorial-cell experiments can use a cached best permutation plus a cheap single-perm KKT solve for beta.
  - **All tied or near-minimum sigmas:** HKO sensitivity/second-order work, subdifferential experiments, all-minimum-orbit validation, and witness-oracle work cannot be served by the current one-sigma cache; they need the richer capacity/orbit report API or an experiment-owned extension with a nonzero gap cutoff.
  - **Non-minimum intermediate nodes:** numerical error-bounds, Q-error/inertia, algorithm ablation, and solver benchmarks need per-`(S, sigma)` matrices, solver verdicts, or timing variants. These should stay in custom datasets rather than a shared polytope catalog.
  - **Path-dependent search traces:** gradient-ascent traces, variable-F paths, and LICCA shard outputs are analysis artifacts, not reusable polytope catalogs except for their final polytopes.
- Finish-mode default: extend header coverage or cache policy only when it affects cited artifact truth, retained thesis reproducibility, or final archive clarity.
- Acceptance check: regenerated `DATAFLOW.md` shows no real stale producer/input edge for declared producers and names any `.jsonl` files that should be future-only, mirrored, detached, or regenerated.
- Stop condition: if an optimization would change committed data values, alter thesis-facing figures, or merge transient search states into a shared cache, stop for Jörn's thesis-priority decision.

### [open] [group:docs] Agent-facing architecture and navigation guide
- Goal: give future agents a repo-level map for the common "where does this live?" questions across `crates/symplectic/`, `experiments/`, datasets, and verification commands. Current orientation is split across `AGENTS.md`, `TASKS.md`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/database.rs`, and per-package `src/lib.rs` headers; there is no top-level architecture guide today.
- Preferred output: top-level `ARCHITECTURE.md` that answers the frequent navigation questions first and links outward to module headers, formal files, and tracker packets instead of duplicating them. This is documentation and boundary-setting work, not a broad refactor task.
- Minimum contents:
  - workspace map (`crates/`, `experiments/`, `formal/`, `contracts/`, `.codex/reference/`, `thesis/`) with which surfaces are thesis-facing, experiment-owned, or infrastructure.
  - stable/simple crate entry points versus expert deep import paths versus intentionally experiment-local helpers.
  - data-flow map: canonical shared polytope catalog, mirrored caches, experiment-owned transient datasets, and JSONL/LFS rules.
  - code-placement rules: when to promote to `crates/symplectic/`, when to extract to `experiments/<topic>/src/lib.rs`, and when to keep logic per-binary.
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
- Finish-mode status: do not chase full formal coverage; make remaining unverified blocks visible to thesis writing or cut them from the publication path.

### [open] [group:docs] Geom formal file restructure
- Jörn partially reviewed Defs 1–13 of `formal/library/geom.tex` (`crates/symplectic/src/geom/review-notes.md`).
- Consolidate Defs 1-2 (symplectic form). Add Def for HKO2024 + Thm for false Viterbo's conjecture.
- Clarify H-representation irredundancy. Fix Defs 12-13 (area/volume are algorithms, not definitions).
- Consider splitting into `math_geometry.tex`, `math_symplectic.tex`, `math_reeb.tex`.
- Finish-mode status: do not perform a broad split unless it directly helps the retained thesis. Prefer narrow corrections to definitions and labels.

### [open] [group:pm] Pre-freeze thesis-facing simplification round
- Setup landed on 2026-04-20:
  - integration branch/worktree: `prefreeze-simplify-exec` at `.codex/worktrees/prefreeze-simplify-exec/`
  - live coordination surfaces: `scratch/prefreeze-2026-04-20-index.md`, `scratch/prefreeze-2026-04-20-context.md`, and packet notes `scratch/prefreeze-2026-04-20-s{1..7}-*.md`
- Packet sessions:
  - `S1` dataflow + generated-output cleanup
  - `S2` small reusable Rust `sys` surface
  - `S3` exact/algebraic boundary
  - `S4` formal library structure
  - `S5` HKO thesis-facing write-up
  - `S6` hostile-landscape thesis-facing write-up
  - `S7` merge + synthesis
- Hard dependencies:
  - `S1`/`S2`/`S3`/`S4` depend only on the setup state above.
  - `S5` starts only after merged `S3` and merged `S4`.
  - `S6` starts only after merged `S1`.
  - `S7` starts only after the accepted packet branches are merged into `prefreeze-simplify-exec`.
- Serialized ownership rule:
  - only setup / merge sessions edit `TASKS.md`
  - only `S7` edits `RESULTS.md` or does final architecture/result synthesis
  - each packet session updates only its own root-`scratch/` note while work is live
- 2026-04-21 PM reduction from the packet notes and current worktree state:
  - the original `S1`-`S7` dependency graph is now obsolete as a live execution graph; treat this row as a salvage/replan note, not as the current packet DAG.
  - `S1` is the only packet currently marked ready-to-merge with a direct thesis-facing payoff; its note reports a clean `bash scripts/dataflow.sh` verification and the worktree exists at `.codex/worktrees/prefreeze-s1-dataflow/`.
  - `S3` is marked `discarded locally`; do not merge it as a packet. Salvage only the research-note clarifications and stale `real_algebraic` naming fix if those are independently useful.
  - `S4` is marked `discarded locally`; do not retry it as a full packet.
  - `S5` and `S6` remain blocked in their packet notes, but should not stay blocked on the old graph; if they are revived, recut them as smaller claim-compression packets after re-planning.
- Immediate next action: integrate `S1`, then re-plan any remaining thesis-facing write-up work as smaller packets; leave `S2` optional and `S3` / `S4` discarded unless a smaller salvage task is explicitly opened.
- Acceptance check: `S7` merges accepted packets, refreshes `TASKS.md` / `RESULTS.md` / `ARCHITECTURE.md`, and reruns the round verification matrix from `scratch/prefreeze-2026-04-20-context.md`.

### [done] [2026-04-12] [group:docs] Library architecture docs audit
- Library docs audit: existing headers + per-module formal files cover architecture mostly held; 0 blockers, 7 gaps, 3 nits across `lib.rs`, `kkt/`, `algorithms` umbrella, `algorithms/tube`.
- 5 doc-only fixes applied and merged to main (no source/algorithm changes). Notable: `algorithms/mod.rs` tube description rewritten from "(placeholder)" to an explicit wrong-rotation-formula warning; `algorithms/` umbrella gained a "correctness invariant" paragraph (overlapping algorithms must agree).
- Skipped as marginal: `derivatives.rs` cross-directory lemma cite (findable via absolute path), `kkt/mod.rs` formula-location nit, umbrella missing utility math-label cross-refs.

### [future] [group:polish] SWE polish (post-thesis-draft-stability bucket)
- Covers: `dev-*/exp-*` stable code → `crates/symplectic/` promotion, test suite completion + perf, documentation gaps, code simplifications (adopt standard patterns, pull in overlooked libraries, abstract/unabstract as helpful).
- **Do not start broad polish during the thesis push**: rerunning experiments invalidates logbook numbers Jörn is about to write up, and abstraction changes break silent invariants nobody's testing.
- Finish-mode status: future work except for correctness, retained reproducibility, or submission blockers.
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
- First-pass cleanup covered `thesis/tube-algorithm-notes.md`, `thesis/appendix-rewrite-notes.md`, `crates/symplectic/src/geom/review-notes.md`, `research/numerics-error-bounds.md`, and formal headers.
- Follow-up experiment Rust pass removed old wave/subagent migration TODO scaffolding and stale pre-migration crate-path references from experiment comments; the experiment/library migration-wording scan returns no matches.
- Research-note migration coverage audit: `/tmp/1-answer.md` was the one-time pre-migration matrix for retiring the older `research/**/design/*.md` layout before the current flat `research/*.md` topic-note convention. Keep it only if a future coverage task needs the row-level audit evidence.
- Preserved copied-code provenance comments and historical deletion/audit notes by marking them historical or leaving already explicit provenance wording.
- Verification: stale-path scans now report only explicitly historical/provenance hits; changed live references point to existing `formal/`, `crates/`, `.codex/reference/`, `.devcontainer/`, or experiment paths.

### [done] [2026-04-15] [group:library] Remove HK2017 capacity fixture, keep profiling bench
- Decision: delete `crates/symplectic/tests/fixtures/capacity_dataset.json` and `crates/symplectic/src/algorithms/hk2017/generate_capacity_fixtures.rs`. Broad HK2017 validation belongs in `experiments/verification/correctness/`; library tests keep small live smoke/regression checks for literature values, conformality, symplectic invariance, pruning agreement, and billiard agreement.
- `crates/symplectic/benches/profiling.rs` remains the Criterion source cited by the verification benchmark write-up for phase profiling and micro-benchmarks; keep `crates/symplectic/Cargo.toml` bench metadata unchanged.
- Updated `experiments/verification/algorithm-comparison/profiling/analyze.py` and refreshed its generated profiling artifacts so the benchmark design notes no longer point at deleted fixture tests.
- Convention update: `AGENTS.md`, `$rust-conventions`, and `$experiment-conventions` now state the boundary between fast crate tests and slow validation experiments.
- Verification for the branch: `cargo test -p symplectic --release --lib`; `cargo clippy -p symplectic --lib -- -D warnings`; `cargo test -p dev-capacity-validation --bin axioms-correctness --release`; `cargo build --workspace --release`; `uv run analyze.py` in `experiments/verification/algorithm-comparison/profiling/`; `cd thesis/ && latexmk && ./check-build.sh`; `cd formal/ && latexmk`. No bench metadata changed, so `cargo bench -p symplectic --bench profiling --no-run` is not required.

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
- Never used organically by agents; agents read source directly. Removed `api-reference/`, the old API extract workspace member, and stale project-doc references. The current repo still carries a `.pre-commit-config.yaml` large-file guard for non-LFS files over 10 MB.

### [done] [2026-04] Polytope database
- `crates/symplectic/src/database.rs`, JSONL format, 1198 entries. 6 experiments migrated.

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
