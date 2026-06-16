# Current State

Cache only. Source files, tests, data, research notes, official forms, and
accepted Jörn/Kai decisions overrule it.

Keep only state that helps choose near-term thesis work, prevents a false thesis
claim, or routes a high-risk area. Keep inventories in their owning maps and
source files.

## Thesis

- Active thesis scaffold files live under `thesis/`; `thesis/main.tex` inputs
  the active scaffold.
  Refresh by: read `thesis/main.tex`, `thesis/MAP.md`, and the relevant
  scaffold file.

- Legacy thesis prose is source material only.
  Refresh by: read `thesis/legacy/README.md` before using legacy prose.
  Use: revalidate legacy algorithm, numerics, and tube prose before it affects
  current thesis wording.

- Current closeout is writeup-first.
  Evidence: migrated cache from the old task files, especially historical
  writing-task notes.
  Refresh when: advisor feedback, retained claim set, or chapter drafting
  changes.
  Meaning: settle thesis-facing method/claim wording first; route code, proof,
  experiment, and reproducibility maintenance from settled wording.

- Current accepted scope frame: HKO local maximality and hostile `sys`-search
  landscape are the main thesis story blocks treated as sufficient.
  Evidence: migrated cache from the old task files, especially historical
  writing-task notes, plus `research/INDEX.md`.
  Refresh when: Jörn/Kai changes scope or retained thesis claims change.

- Jörn stated on 2026-06-01 that the following thesis content areas are all
  must-have and should not be re-asked/reclassified in the next weeks:
  HKO local result; pentagon product result; search/data-science result;
  generalized Reeb orbit and HK2019 finite-computation foundation;
  first-order perturbation method; numerics/exactness story;
  code/data/reproducibility story; use-of-AI disclosure; visualization as
  exploration; CH2021/flow-graph/tube algorithm story; and preliminaries needed
  to make those readable. See `FACTSHEET.md` item 8 for clarified descriptions.
  Refresh when: Jörn explicitly changes the thesis content scope.

## Research Stories

- `research/INDEX.md` is the thesis story index.
  Authority: topic research notes, proof-bearing sources, and accepted Jörn/Kai
  decisions overrule it.

- HKO local maximality is thesis spine.
  Source truth: `experiments/hko-local-maximum/README.md`, the current
  `experiments/hko-local-maximum/theorem/` packet,
  `experiments/hko-local-maximum/smooth-only-rank-defect/`, and related
  formal/thesis files, especially `formal/hko-feasible-section-upper-branches.tex`
  and `thesis/hko-local-maximum.tex`.
  Current state: the exact feasible-section theorem certificate is merged and
  verifies in Sage; the formal implication has been agent-line-checked; the
  integrated thesis chapter `thesis/hko-local-maximum.tex` is on `main` as a
  theorem-strength draft. Jörn quick-reviewed the rebuilt formal PDF on
  2026-06-05 and spotted no gaps; he judged any remaining mistakes likely
  closeable before thesis release. The 2026-06-05 clarity repairs made the
  local chart hypotheses, the volume-row arithmetic, and the local symmetry
  group/dimension explicit. A 2026-06-16 status pass ported the thesis chapter,
  corrected the HKO2024 citation to Proposition 1.3 where needed, added the
  formal Hausdorff-domain bridge remark, normalized fixed-`F=10`/`F=11`
  wording, and repaired the malformed trailing line in the `F=11`
  neighborhood-splitting JSONL artifact. The remaining theorem-strength gate is
  final theorem wording/Kai review.
  High-risk facts:
  - Current exact field is quartic `Q(tan(pi/5))`, not `Q(sqrt(5))`.
  - Current theorem-facing certificate uses 26 feasible-section rows, exact
    row rank `25`, exact symmetry tangent rank `15`, and a positive exact
    convex relation summing the rows to `0`.
  - The proof route is the feasible-section upper-branch route. The smooth-only
    nonsingular positive-beta route is a recorded failed diagnostic, not
    theorem evidence: it has `44` nonsingular rows of projected rank `23` in
    the `25`-dimensional quotient, while the feasible-section route reaches
    rank `25`.
  - Old `44`-orbit / `10`-gradient prose is stale against the current
    `150`-active-row diagnostic and its `44` nonsingular / `106` singular row
    split.
  Refresh when: HKO theorem wording, thesis chapter status, or LICCA evidence
  changes.

- Hostile `sys`-search landscape is thesis spine.
  Data-science source truth:
  `experiments/sys-datascience/README.md`,
  `experiments/sys-datascience/methods/README.md`, and current
  `experiments/sys-datascience/methods/<method>/README.md` packets.
  The method-coverage checklist and research toolbox audit are recall/context
  only, not source truth.
  Current state: the retained LICCA datascience tables are in
  `experiments/sys-datascience/tables/`; method rows that still
  matter need current evidence packets or explicit abandonment.
  High-risk fact: tried thesis-affecting results need repo-owned evidence,
  checks appropriate to their verdict, explicit caveats, and thesis-use
  disposition.
  Current state: older pre-LICCA data-science reports were removed or replaced
  by status markers. See the methods README for current packet conventions and
  current method packets.
  Refresh when: retained hostile-landscape wording or endpoint datasets change.

- Sys first-order generic row-chart writing is active.
  Source truth: `research/sys-first-order-local-behavior.md`,
  `thesis/first-order-perturbations.tex`, formal notes.
  Current state: broad compute-once evaluator is classified as `ONLY-HEAVY`;
  generic smooth case is the readable thesis route.
  High-risk guard: accepted statuses are `PROVED`, `ONLY-HEAVY`, `BLOCKED`, and
  `NO-GO`; a theorem-ready route must not hide smooth-branch, Hadamard-only,
  ray-limit, or per-direction optimization substitutes.
  Refresh when: a later source gives a readable exact active-germ theorem,
  counterexample, or changed thesis dependency.

- Numerics supports retained thesis text.
  Source truth: `experiments/numerics/README.md`, `formal/hk2017-qp-*.tex`,
  current thesis numerics files.
  Current state: exact/f64/indeterminate story is not a public certified solver
  claim; the active experiment is a structured error audit with JSONL raw
  observations, processed summaries, and a generated report. Generic-case-first
  proof work drives any stronger retained theorem wording.
  Generic-route handles: full rank/condition of `C`, negative reduced Hessian
  on the retained tangent space, positive beta margin, positive `Q`/action gap
  from competitors, and adjacency/pruning assumptions.
  Flow-graph/tube cache: current live control surface is
  `crates/symplectic/src/algorithms/flow_graph/README.md`. The older
  `research/tube-algorithm-raw-jorn-2026-05-04.md` and
  `research/tube-algorithm.md` files are legacy/imported source material.
  Old thesis/formal/Rust tube surfaces are stale unless rewritten from the live
  flow-graph surface and source truth.
  Refresh when: numerical appendix route, solver story, derivative/projection
  claim, or tube inclusion changes.

## Code, Experiments, Verification

- `crates/MAP.md` is the crate navigation cache.
  Authority: crate source, crate READMEs, tests, and formal labels cited by code
  overrule it.

- `CAPABILITY_CLAIM_MAP.md` is the high-level capability cache.
  Use it for "what can the repo rely on?" questions. Refresh affected rows from
  source truth before depending on a stronger claim.

- Exact arithmetic replacement is merged.
  Source: `tasks/references/exact-arithmetic-replacement-2026-05-10.md`,
  `crates/algebraic-numbers/`.
  Refresh when: exact scalar API, linear algebra API, or exact validation claims
  change.

- Core smoke and selected verification passed on 2026-05-31.
  Source: `tasks/references/repo-status-smoke-and-core-2026-05-31.md`.
  Quick check: run `scripts/repo-status-summary.sh` to compare current `HEAD`
  and working-tree cleanliness against that dated reference.
  Scope: formatting, workspace check, algebraic-numbers tests/clippy/example,
  euclidean-polytopes tests/clippy, symplectic release lib tests, public
  capacity API tests, selected validation correctness test, and compile/no-run
  checks listed in the reference. Also includes workspace release build,
  `exp-sys-landscape` check, combinatorial-cells tests/clippy, and current
  `formal/` and `thesis/` LaTeX builds. This does not refresh tracked
  experiment datasets or prove thesis claims.
  Refresh when: core code changes, crate APIs change, validation evidence is
  promoted, or a future agent needs stronger status than the listed checks.

- Exact/certified validation has a known audit risk.
  Source truth: `crates/symplectic/src/kkt/rational_solver.rs`,
  `crates/symplectic/src/algorithms/orbit_search.rs`, current tests and
  research notes.
  Current state: a previous weak audit missed a high-severity exact/certified
  mismatch. False exact fallback was caused by floating relative pivot threshold
  in exact KKT solving. Boundary revalidation checks include beta length,
  `beta_i > 0`, `Q > 0`, exact normalization, and exact closure.
  Refresh by: code-first audit of exact/certified/ground-truth paths.

- Selected validation evidence exists for capacity algorithms and orbit
  recovery.
  Source truth: `research/verification.md`, `experiments/verification/`.
  Current state: `research/verification.md` records `28` selected polytopes,
  `469` trusted minima, and full reconstruction success for all `469` minima.
  Refresh when: solver code, target pool, schema, tolerances, or thesis wording
  changes.

- LFS-tracked `.jsonl` artifacts may be preserved artifacts, historical records,
  or future/follow-up material; they must not silently support thesis claims.
  Refresh by: checking retained thesis claim usage and the owning experiment or
  research note.

- `euclidean-polytopes` is merged and owns ordinary `R^4` convex geometry.
  Source truth: `crates/euclidean-polytopes/README.md`,
  `crates/euclidean-polytopes/DEVELOPMENT.md`, source, tests.
  Current state: exact origin-interior, exact extreme points, exact polar,
  incidence faces, known-incidence exact/f64 volume, facet volume, random dual
  vertex sampling exist. Polar/e2e performance has a merged first optimization
  but is not approved complete. Jörn-level API decisions live in the crate
  README/DEVELOPMENT files, not in this cache.
  Refresh when: Euclidean public API, polar/volume/incidence contracts, or
  performance claims change.

- Experiment command contracts are partially normalized.
  Source truth: experiment READMEs and current binaries.
  Current state: prefer `--help`, documented smoke modes, and README-declared
  commands for quick checks; full-output producers can overwrite tracked
  evidence and need explicit intent.
  Refresh when: binary CLI behavior, tracked output paths, or package README
  command sections change.

- Deleted draft skills and low-quality verification helpers are historical
  evidence only.
  Use: rebuild helpers only from current source truth or repeated current
  failures.

## Submission/Admin

- `tasks/submit-thesis/` stores downloaded MNTF forms and markdown conversions
  from 2026-04-24.
  Refresh by: read `tasks/submit-thesis/README.md` and recheck the official
  MNTF page before final handin.

- Registration form was recorded as filled and signed by Kai. Jörn confirmed
  that Elizabeth approved it; the pending action is to hand in the note to the
  `Prüfungsamt`.
  Refresh by: checking official forms and current Jörn/Kai/Elizabeth decisions.

- Zenodo is the leading non-GitHub preservation candidate because Kai named it.
  arXiv/outreach are post-Kai-review candidates unless promoted.
  Refresh by: checking `tasks/submit-thesis/README.md` and current advisor
  context.
