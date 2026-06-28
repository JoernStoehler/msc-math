# Capability Claim Map

## Role

This file is a non-authoritative cache of high-level repo-capability claims.
Source files, tests, data, owner-local notes, accepted Jörn/Kai decisions, and
thesis text overrule this file.

Use this file to answer "what can the repo currently rely on?" without
reconstructing the claim, scope, support, caveats, and refresh triggers from
scratch. This is not a component inventory, proof file, task tracker, or
complete claim graph.

Add a row only for a capability-level claim worth caching: repeated agent
question, thesis dependency, easy overclaim, expensive reconstruction, or
multi-place refresh risk. Do not add rows just because a folder, binary, API, or
artifact exists.

## Maintenance

Unmarked bullets are asserted true in the current map. Bracket markers record
current nonstandard state:

- `(partial)`: true only in the stated scope or with named gaps
- `(fail)`: tracked capability currently fails; the failure statement is true
- `(stale)`: do not rely on the row until refreshed
- `(blocked)`: current work or verification cannot proceed without a named
  blocker being resolved
- `(unchecked)`: plausible support, not checked in this map pass

Prefer the lowest-ceremony true statement that preserves verification:

- write the top claim plainly
- put technical boundaries under `Scope`
- cite source truth under `Files`
- write bare command bullets instead of labels like `Check:`
- use qualifiers only when they add information, e.g. focused, alternative, or
  artifact-refreshing commands
- keep nuanced subclaims shallow; promote them to their own row only when they
  become repeated capability questions
- point durable references to source files, tests, data, owner-local notes, or
  accepted decisions, not to internal map labels

When refreshing, start from source truth and update only rows affected by the
task unless doing an explicit whole-map audit. If a support command fails,
rewrite the row as a true `(fail)` state rather than leaving a false capability
claim.

## Capability Claims

- We can do exact arithmetic and exact dense linear algebra over statically
  chosen real algebraic fields.
  - Scope: one chosen field `Q[alpha]` at compile time; exact ordering; dense
    row reduction, rank, kernel basis, linear solve, and negative-definite
    checks.
  - Files:
    - `crates/algebraic-numbers/README.md`
    - `crates/algebraic-numbers/LINEAR_ALGEBRA_FEATURES.md`
    - `crates/algebraic-numbers/src/`
    - `crates/algebraic-numbers/tests/`
  - Tests cover scalar arithmetic, exact ordering, nalgebra container use,
    exact scalar trait behavior, and exact linear algebra.
    - `cargo test -p algebraic-numbers --release`
  - This is not a general computer algebra system.
  - Nuance:
    - no runtime field construction
    - no public determinant or inverse API
    - no full inertia, eigenvalue, or diagonalization layer
    - no f64 exact scalar implementation
  - Refresh when:
    - exact scalar public API changes
    - linear-algebra API or test coverage changes
    - exact validation or theorem-route claims start relying on a stronger
      algebraic capability

- We can do ordinary convex-polytope geometry needed by the current Rust and
  experiment workflows.
  - Scope: ambient `R^4`; point-set predicates; exact polar vertex enumeration;
    incidence-derived face helpers; known-incidence f64/exact 4-volume
    helpers; known-incidence f64 facet 3-volume helpers; f64 sign filters;
    random candidate dual-vertex sampling.
  - Files:
    - `crates/euclidean-polytopes/README.md`
    - `crates/euclidean-polytopes/DEVELOPMENT.md`
    - `crates/euclidean-polytopes/src/`
    - `crates/euclidean-polytopes/tests/`
  - Tests cover extreme points, faces, polar vertices, random sampling, and
    volume behavior.
    - `cargo test -p euclidean-polytopes`
  - This crate is not for EHZ capacity, Reeb dynamics, symplectic signs, KKT
    assembly, orbit search, or thesis experiment workflow code.
  - (partial) polar enumeration performance and follow-up work may still
    matter; route those through crate README/DEVELOPMENT files, owner-local
    notes, or implementation files.
  - Refresh when:
    - Euclidean public APIs change
    - polar, volume, incidence, or representation contracts change
    - performance claims become thesis- or workflow-bearing

- We can compute the volume of the polytopes in current workflows.
  - Scope: current workflows with full-dimensional `R^4` polytopes, vertices,
    and known vertex-facet incidence; both f64 and exact scalar paths exist;
    this is not EHZ capacity.
  - Files:
    - `crates/euclidean-polytopes/src/volume.rs`
    - `crates/euclidean-polytopes/src/lib.rs`
    - `crates/euclidean-polytopes/tests/volume.rs`
    - `formal/symplectic-polytope-geometry.tex`
    - `crates/euclidean-polytopes/DEVELOPMENT.md`
  - `volume.rs` implements the f64 and exact known-incidence volume helpers.
  - `lib.rs` exports the volume helpers.
  - `tests/volume.rs` checks known values, scaling, exact/f64 agreement, and
    rejection behavior.
    - `cargo test -p euclidean-polytopes`
    - Focused check for volume-named tests only:
      `cargo test -p euclidean-polytopes volume`
  - `formal/symplectic-polytope-geometry.tex` records the origin-star
    triangulation formula.
    - `rg -n "volume-star-triangulation|Volume of a 4D polytope" formal/symplectic-polytope-geometry.tex`
  - Nuance:
    - the formal volume definition and origin-star formula are currently in
      `unverified` blocks, so the formal note is a route/specification surface,
      not reviewed theorem evidence.
    - (partial) malformed or numerically untrusted f64 inputs are rejected
      rather than trusted; rejection is tested for known cases, not every
      malformed geometric configuration.
    - exact volume depends on the exact scalar layer.
    - this is ordinary geometry, not symplectic capacity.
  - Refresh when:
    - volume helper signatures change
    - incidence convention changes
    - exact scalar behavior changes
    - volume tests change or fail
    - thesis/formal text starts relying on a stronger volume claim

- We can run the main symplectic capacity and orbit-search machinery used by
  current experiments.
  - Scope: 4D convex-polytopes workflows in `crates/symplectic`; known
    fixtures, HK2017 and billiard algorithms, KKT/QP solve machinery, exact
    single-orbit kernels, derivatives, random sampling, and JSONL
    persistence/schema helpers.
  - Files:
    - `crates/symplectic/README.md`
    - `crates/MAP.md`
    - `crates/symplectic/src/lib.rs`
    - `crates/symplectic/src/algorithms/orbit_search.rs`
    - `crates/symplectic/src/algorithms/`
    - `crates/symplectic/src/kkt/`
    - `crates/symplectic/src/kkt/rational_solver.rs`
    - `crates/symplectic/src/exact/`
    - `crates/symplectic/src/derivatives.rs`
    - `crates/symplectic/src/random.rs`
    - `crates/symplectic/src/database.rs`
    - `crates/symplectic/src/dataset.rs`
    - `crates/symplectic/tests/public_capacity_api.rs`
  - Crate-local tests are smoke, unit, and regression checks.
    - `cargo test -p symplectic --release --lib`
    - Public API check: `cargo test -p symplectic --release --test public_capacity_api`
  - Broader correctness and orbit-recovery evidence belongs in
    `experiments/verification/`, not in this crate row.
  - Nuance:
    - some deep public paths are experiment-facing in practice but not settled
      as long-term public API.
    - f64 capacity behavior is not exact-real proof.
    - exact/certified capacity paths have a prior false exact-fallback
      regression; read `experiments/dev-quadratic-program/README.md` before
      relying on a stronger exact/certified claim.
  - Refresh when:
    - solver semantics change
    - capacity result semantics change
    - persistence schema changes
    - public API tier decisions change
    - retained thesis capacity claims change

- We have selected validation evidence for capacity algorithms and orbit
  recovery.
  - Scope: validation on selected local-first targets, not an exhaustive theorem
    over all polytopes.
  - Files:
    - `experiments/verification/README.md`
    - `experiments/verification/correctness/`
    - `experiments/verification/all-minimum/`
    - `experiments/verification/orbit-recovery/`
  - The verification package documents which commands refresh tracked evidence
    and which checks are safe smoke/compile checks.
  - `experiments/verification/README.md` records 28 selected polytopes, 469
    trusted minima, and full reconstruction success for all 469 minima.
  - Stored proposition rows can be checked without refreshing the producer.
    - `cargo test -p dev-capacity-validation --bin axioms-correctness --release`
  - Nuance:
    - tracked JSONL files are evidence artifacts; do not run full-output
      producers as smoke checks.
    - words like "diverse" or "representative" depend on target-pool criteria
      and should be refreshed if that pool changes.
  - Refresh when:
    - shared solver code changes
    - target-pool selection changes
    - all-minimum or orbit-recovery schema changes
    - tolerance assumptions change
    - thesis wording cites the validation more strongly

- We have an experiment command contract that protects tracked evidence files.
  - Scope: experiment README files document smoke-safe commands, full-output
    producers, and tracked-evidence refresh commands where known.
  - Files:
    - `experiments/verification/README.md`
    - `experiments/sys-landscape/README.md`
    - `experiments/sys-datascience/produce/README.md`
    - `experiments/hko-local-maximum/theorem/exact-witness/README.md`
    - `experiments/hko-local-maximum/README.md`
    - `experiments/dev-quadratic-program/numerics-audit/README.md`
    - `experiments/combinatorial-cells/README.md`
  - Agent rule: prefer `--help`, compile checks, or documented smoke modes for
    quick command validation.
  - (partial) command-contract normalization is uneven. Some packages have
    full-output producers without smoke modes.
  - (partial) The sys-landscape datascience `smoke-pipeline.sh` is temp-output
    safe but not cheap; use it as integration smoke, not as a quick command
    check.
  - Refresh when:
    - binary CLI behavior changes
    - tracked output paths change
    - package README command sections change
    - smoke/full conventions are normalized

- We have a sys-landscape datascience pipeline and method ledger for the
  hostile-search story.
  - Scope: data tables, method reports, and idea/method ledgers that support
    bounded negative/search-usefulness claims.
  - Files:
    - `experiments/sys-datascience/README.md`
    - `experiments/sys-datascience/prepare/README.md`
    - `experiments/sys-datascience/prepare/main.rs`
    - `experiments/sys-datascience/methods/README.md`
    - `experiments/sys-datascience/produce/README.md`
    - `experiments/sys-datascience/smoke-pipeline.sh`
  - The datascience prepare pipeline writes flat retained table files under
    `experiments/sys-datascience/prepare/`.
  - Method packets consume retained tables and build method-local rectangular
    inputs when needed.
  - Method reports separate observation, inference, and thesis use.
  - Nuance:
    - this supports bounded negative/search-usefulness evidence, not a density
      theorem, impossibility theorem, or general ML benchmark claim.
    - some ledger rows remain future, skipped, or source-truth repair work.
  - Refresh when:
    - table schemas or feature columns change
    - method reports are added or reclassified
    - positive or conjectured-positive search-rule statuses change
    - thesis-use decisions change

- We have a current HKO local-maximum theorem route with a verified exact
  feasible-section certificate, while final theorem wording remains gated by
  Kai review.
  - Scope: HKO2024 `M_10` local-maximality story modulo natural `sys`
    symmetries; broad HKO local maximality remains conjectural in current repo
    classification.
  - Files:
    - `experiments/hko-local-maximum/README.md`
    - `experiments/hko-local-maximum/theorem/README.md`
    - `experiments/hko-local-maximum/smooth-only-rank-defect/README.md`
    - `formal/hko-feasible-section-upper-branches.tex`
    - `thesis/hko-local-maximum-content.md`
    - `experiments/hko-local-maximum/theorem/exact-witness/`
    - `experiments/hko-local-maximum/theorem/feasible-section-certificate/`
  - The current theorem-facing certificate is the feasible-section certificate:
    26 exact rows, exact row rank `25`, exact symmetry tangent rank `15`,
    positive lambdas, and exact lambda-weighted row sum `0`.
  - The exact Sage certificate verifies. The formal implication from that
    finite predicate to quotient-local maximality has been agent-line-checked;
    Jörn quick-reviewed the rebuilt PDF on 2026-06-05 and spotted no gaps.
    The remaining gate is final theorem wording/Kai review.
  - The older widened representative-row witness remains a partial historical
    surface, not the theorem-facing certificate.
  - First-order numerical support, second-order evidence, neighborhood
    sampling, and `M_11` ascent experiments are supporting evidence.
  - Nuance:
    - supporting experiments do not replace the exact feasible-section
      certificate or the formal implication for theorem-strength wording.
    - current theorem-facing field wording uses the quartic
      `Q(tan(pi/5))`; older `Q(sqrt(5))` certificate wording is stale.
    - old `44`-orbit / `10`-gradient prose has been caveated against the
      current `150` exact action orbits / `20` visited subsets / `28`
      gradient-pattern bookkeeping; theorem-facing prose should use the
      26-row feasible-section certificate interface.
  - Refresh when:
    - exact-witness coverage changes
    - HKO theorem wording is frozen or weakened
    - field wording or exact-minimum bookkeeping changes
    - new LICCA/HKO evidence is promoted
    - thesis HKO chapter starts relying on a stronger claim

- We have a current route for writing first-order `sys` behavior, but the full
  arbitrary-polytope evaluator is too heavy for first exposition.
  - Scope: generic row-chart thesis story first; non-generic active-germ or
    semialgebraic evaluator material belongs in boundary discussion or
    future/follow-up unless HKO proof wording needs it.
  - Files:
    - `formal/sys-first-order-local-behavior.md`
    - `thesis/06-first-order-perturbations.tex`
    - `thesis/legacy/sys-first-order-regular-case.tex`
    - `formal/capacity-smoothness-classification.tex`
    - `formal/capacity-boundary-subdifferential.tex`
  - The current formal note records the broad theorem classification as
    `ONLY-HEAVY`.
  - Generic smooth-branch or Danskin-envelope statements are not substitutes
    for the compute-once arbitrary-polytope evaluator.
  - Exact-real claims and f64 diagnostics are separate.
  - Refresh when:
    - a proof, counterexample, or accepted weakening changes the
      `ONLY-HEAVY` classification
    - thesis wording starts relying on arbitrary first-order behavior
    - implementation claims exact first-order behavior beyond generic smooth
      branches
    - HKO proof wording starts depending on this theorem route

- We have a QP/KKT numerics audit experiment that supports solver diagnostics
  and caveated numerical trust.
  - Scope: structured f64-vs-oracle observations for retained KKT variables and
    predicates, with explicit input-pair and oracle provenance. This audit is
    coupled to QP route development and now lives under the dev-QP packet.
  - Files:
    - `experiments/dev-quadratic-program/numerics-audit/README.md`
    - `experiments/dev-quadratic-program/numerics-audit/`
    - `formal/hk2017-qp-core.tex`
    - `formal/hk2017-qp-precision.tex`
  - The current route is QP/KKT diagnostics plus exact/empirical caveats; route
    contract decisions live in `experiments/dev-quadratic-program/`.
  - This does not claim all public f64 capacity wrappers are fully certified
    numerical solvers.
  - Nuance:
    - near-threshold beta behavior, singular/rank-deficient cases, and some
      Taylor/error-bound details remain caveated unless final thesis text
      narrows the claim or later proof closes them.
    - full-output numerics commands should not be run as smoke checks unless
      the task intentionally refreshes artifacts.
  - Refresh when:
    - solver contract changes
    - error-bound theorem changes
    - f64/exact boundary changes
    - public wrapper guarantee wording changes
    - retained numerical appendix claims change

- We have bounded combinatorial-cell boundary exploration evidence.
  - Scope: boundary events, step bounds, cell widths, convexity, multiple
    crossings, omega hypotheses, and gradient-discontinuity analysis artifacts
    under `experiments/combinatorial-cells`.
  - Files:
    - `experiments/combinatorial-cells/README.md`
    - `experiments/combinatorial-cells/src/lib.rs`
    - `experiments/combinatorial-cells/boundary-characterization/`
    - `experiments/combinatorial-cells/cell-widths/`
    - `experiments/combinatorial-cells/convexity/`
    - `experiments/combinatorial-cells/multiple-crossings/`
    - `experiments/combinatorial-cells/omega-hypothesis/`
    - `experiments/combinatorial-cells/gradient-discontinuity/`
  - The package README documents that full producer binaries update tracked
    evidence and do not currently have smoke modes.
  - Gradient-discontinuity is analysis/artifact-side in this package, not a
    currently declared Rust binary.
  - Compile-only checks are available.
    - `cargo test -p exp-combinatorial-cells --all-targets`
    - Alternative: `cargo clippy -p exp-combinatorial-cells --all-targets`
  - Nuance:
    - this evidence does not license global convexity in dual-vertex space.
    - this evidence does not license continuity-from-sampling for
      first-boundary `sys`.
    - this evidence does not license a single-boundary or monotonicity model
      for repeated `sys` improvements.
  - Refresh when:
    - package cache scale changes
    - deterministic runtime controls change
    - tracked artifact counts change
    - formal boundary claims change
    - thesis text promotes this evidence

- We have a specialized crosspolytope capacity computation.
  - Scope: one-off 4D crosspolytope computation in
    `experiments/crosspolytope`, with symmetry reduction and checkpointed
    backtracking.
  - Files:
    - `experiments/crosspolytope/README.md`
    - `experiments/crosspolytope/main/main.rs`
    - `experiments/crosspolytope/main/crosspolytope.jsonl`
    - `crates/symplectic/src/geom/known_polytopes.rs`
    - `crates/symplectic/src/algorithms/hk2017/tests_literature.rs`
  - The current artifact records capacity near `4.0`, `sys` near `0.75`, and
    search complete through subset size 13.
  - Crate-level known-polytopes code records the crosspolytope capacity as
    computed, with no literature value.
  - The fast crate regression is an upper-bound certificate for the best known
    orbit.
  - Nuance:
    - global optimality still depends on the experiment's search evidence or a
      further exclusion/complete-run argument.
    - current artifacts do not enumerate `m = 14..16`.
  - Refresh when:
    - the candidate capacity changes
    - `MAX_SUBSET_SIZE` is raised and rerun
    - `crosspolytope.jsonl` is regenerated
    - thesis text cites this computation as more than a caveated computed value

- We can generate visualization figures and browser-rendered assets for 4D
  polytope geometry and recovered Reeb trajectories.
  - Scope: experiment-local presentation pipeline, not reusable library API and
    not proof surface.
  - Files:
    - `experiments/visualization/README.md`
    - `thesis/visualization-3d-content.md`
    - `experiments/visualization/main/main.rs`
    - `experiments/visualization/main/models.rs`
    - `experiments/visualization/main/orbit_collection.rs`
    - `experiments/visualization/main/trajectories.rs`
    - `experiments/visualization/main/viz/`
    - `experiments/visualization/main/viz/data/`
    - tracked PNG assets under `experiments/visualization/main/`
    - `thesis/visualization-3d.tex` if retained
  - The Rust generator writes geometry, combinatorics, trajectory payloads, and
    summary values.
  - The browser pipeline embeds generated JSON and uses screenshot automation
    for static figures.
  - Nuance:
    - projection distortion, pole tuning, and clipping are presentation
      controls.
    - high-facet orbit recovery is intentionally bounded and filtered.
    - do not restore the removed `docs/viz` deployment path as active workflow.
  - Refresh when:
    - geometry payload schema changes
    - projection or rendering pipeline changes
    - figure filenames or retained thesis figure set changes
    - thesis figure traceability checks change

- We have a formal proof-note layer for developer-facing mathematical
  documentation.
  - Scope: geometry, capacity algorithms, derivatives, regularity, numerical
    certification, special families, and search correctness under `formal/`.
  - Files:
    - `formal/main.tex`
    - `formal/preamble.tex`
    - topic files input by `formal/main.tex`
    - linked task and research files for current status
  - The formal tree is a proof-route and documentation surface, not
    automatically publication-ready theorem truth.
  - Many sections are explicitly unverified or contain Jörn-review TODOs.
    - `rg -n -e 'begin\{unverified\}' -e 'TODO: JÖRN' -e 'Status:' formal -g '*.tex'`
  - The current formal build passed on 2026-05-31.
    - `cd formal/ && latexmk`
    - Evidence: `scripts/repo-status/repo-status-smoke-and-core-2026-05-31.md`
  - Use local status comments and `unverified` environments before citing a
    formal note as support for thesis or code claims.
  - Refresh when:
    - a theorem is promoted to thesis text
    - a proof gap is closed or reopened
    - old tube, HKO, or numerics material is superseded
    - code comments start citing a formal label

- We have an active thesis scaffold and publication surface.
  - Scope: `thesis/main.tex` inputs the active scaffold; `thesis/legacy/` is
    source material only.
  - Files:
    - `thesis/main.tex`
    - `thesis/MAP.md`
    - `thesis/DEVELOPMENT.md`
    - `README.md`
    - `FACTSHEET.md`
    - `submit/README.md`
  - Thesis correctness must not depend on runtime links into `experiments/`,
    `formal/`, or `crates/`.
  - Publication assets and final wording must be copied, cited, or summarized
    deliberately inside `thesis/`.
  - The current thesis build passed on 2026-05-31.
    - `cd thesis/ && latexmk && ./check-build.sh`
    - Evidence: `scripts/repo-status/repo-status-smoke-and-core-2026-05-31.md`
  - Refresh when:
    - `thesis/main.tex` input structure changes
    - retained claim set changes
    - figure or publication assets change
    - build contract changes
    - submission requirements change

## Cross-Claim Refresh Clusters

- Capacity solver semantics change.
  - Recheck:
    - symplectic capacity and orbit-search machinery
    - capacity verification evidence
    - numerics caveats
    - HKO evidence if affected outputs are load-bearing
    - thesis capacity/HKO/numerics wording if retained

- Verification target-pool definition changes.
  - Recheck:
    - capacity verification evidence
    - any wording that says "diverse", "selected", "representative", or
      "tested on"
    - thesis/reproducibility wording if retained

- HKO theorem wording changes.
  - Recheck:
    - HKO evidence route
    - first-order `sys` dependency
    - numerics caveats
    - formal HKO notes
    - thesis HKO chapter and final gates

- Datascience table or feature-column schema changes.
  - Recheck:
    - sys-landscape datascience pipeline
    - method ledger
    - visualization only if figures use changed data
    - hostile-landscape thesis caveats

- Formal note promotion.
  - Recheck:
    - local `formal/*.tex` status comments and `unverified` environments
    - topic owner-local progress note
    - thesis/code comments that cite the promoted statement

## Refresh Recipe

- Check current `HEAD` against the latest dated verification cache:
  - `scripts/repo-status-summary.sh`
- List claim headings and marked rows:
  - `rg -n "^(- We|  - \()" CAPABILITY_CLAIM_MAP.md`
- Check navigation/source surfaces:
  - `rg --files | rg '(^|/)(README|DEVELOPMENT|MAP|INDEX)\.md$|Cargo\.toml$'`
- Check experiment entrypoints:
  - `rg --files experiments | rg '(^|/)(README\.md|Cargo\.toml|[^/]+\.(rs|py))$'`
- Check durable crate implementation and tests:
  - `rg --files crates | rg '(^|/)(README\.md|DEVELOPMENT\.md|.*FEATURES.*\.md|[^/]+\.rs)$'`
- Compare thesis story routing:
  - `sed -n '1,220p' thesis/central-claim-control.md`
- Search targeted status surfaces for drift:
  - `rg -n "Epistemic status|Status:|\[blocked\]|\[active\]|\[done\]" thesis formal experiments crates -g '*.md'`
  - `rg -n -e 'begin\{unverified\}' -e 'TODO: JÖRN' -e 'Status:' formal -g '*.tex'`
  - `rg -n "full-output|tracked evidence|smoke|canonical|--full" experiments -g README.md`
