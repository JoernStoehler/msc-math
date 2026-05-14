<!--
Purpose: numerical appendix, solver, and method-claim roadmap.
Context: route-freeze surface for proof-vs-validation-vs-caveat decisions.
-->

# Numerics Roadmap

## Status

- State: active.
- Last updated: 2026-05-14.
- Source surfaces: `research/numerics.md`,
  `research/numerics-error-bounds.md`, `experiments/numerics/`,
  `formal/`, `tasks/verify-thesis-done.md`,
  `research/tube-algorithm-raw-jorn-2026-05-04.md`,
  `research/tube-algorithm.md`.
- Refresh when: numerical appendix route, solver story, or derivative/projection
  claims change.

## Steering Cache

- [accepted 2026-04-24] Full numerical formalization is not a default thesis
  obligation. Fix or caveat only what supports retained thesis text.
  Source: finish-mode reset.
  Why it matters: prevents broad solver-proof work from replacing writing.
- [accepted 2026-04-24] Projection solver / beta-LP implementation work should
  not reopen a broad solver-development program during thesis closeout.
  Source: legacy tracker.
  Why it matters: route freeze should decide wording, not launch refactors.
- [accepted 2026-04-24] Thesis/code alignment is live only where mismatch would
  make the thesis wrong or unreproducible.
  Source: finish-mode reset.
  Why it matters: code-side cleanup is future unless it changes retained claims.
- [accepted 2026-05-01] Generic-case-first numerics is now the preferred route:
  state explicit conditions on intermediate solver variables, prove/implement
  that exact case first, then measure f64 error and study how bounds blow up
  when those conditions fail in non-generic limits.
  Source: Jorn steering in numerics-strong-route session.
  Why it matters: agents should not try to certify every degenerate or
  near-threshold case before the generic theorem, code contract, and empirical
  loop align.
- [accepted 2026-05-04] The tube algorithm is being re-imported from Jörn's
  current head/paper formalization. The raw audited source is
  `research/tube-algorithm-raw-jorn-2026-05-04.md`; start from
  `research/tube-algorithm.md` for routing and accepted clarifications. Treat
  old thesis/formal/code tube surfaces as downstream or stale until the current
  mathematical source is written.
  Source: Jorn request to start importing the tube algorithm.
  Why it matters: prevents agents from treating the old agent-written tube draft
  or blocked Rust module as the specification.
- [accepted 2026-05-14] The immediate numerics target is not a fully trusted
  public numerical solver; it is trusted enough numerics to rerun the retained
  experiments properly.
  Source: Jorn writeup-first steering.
  Why it matters: exact paths, f64 fast paths, tests, and profiling should be
  selected by the writeup's retained claims and experiment rerun needs. Stronger
  proof obligations remain mainline only when the settled thesis text needs
  them.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Numerical appendix route freeze | `[map-input]` | mainline thesis | agent prep then Jorn | First state the exact/f64/indeterminate boundary needed to rerun retained experiments properly; then decide whether thesis prose also describes public f64 wrappers, the stronger exact/guaranteed verification layer, or both with an explicit boundary. | `research/numerics*.md`, `thesis/appendix-numerical.tex`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/algorithms/orbit_search.rs`, `thesis/planned-toc.md` |
| Generic-case solver contract | `[active]` | mainline thesis | agent prep then Jorn math review | Draft the exact generic theorem with conditions on `C`, reduced Hessian eigenvalues, beta margin, Q/action gap, and adjacency/pruning assumptions; then align experiment f64 diagnostics to those variables. | `research/numerics-error-bounds.md`, `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `experiments/numerics/error-bounds/` |
| Numerical error bounds | `[map-input]` | contingent during writing | retained wording | Treat as proved exact/Q pieces plus empirical eta checks plus named caveats; fix/caveat only the pieces the thesis cites. Under the generic route, record each caveat as a generic precondition, non-generic limit behavior, empirical formula, or Jorn review question. | `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `experiments/numerics/q-error/q_error_output.txt`, `thesis-stories-are-supported.md` |
| Projection solver | `[map-input]` | contingent during writing | Jorn math if retained | Use the projection/null-space solver as the candidate generic-route implementation story only if the thesis needs it. The old `OrbitSolveBackend::Projected` strategy surface was deleted; any revived route should be a flat solver with shared orbit payload and Q-bound contract. | `crates/symplectic/src/algorithms/orbit_search.rs`, `experiments/numerics/error-bounds/projection_solver.rs` |
| Beta-LP unification | `[future]` | future/follow-up by default | Jorn math if retained | Keep as future unless needed for retained solver explanation. | legacy beta-LP row |
| Solver formal writeup | `[map-input]` | contingent during writing | retained wording | Avoid full per-module formalization unless thesis text requires it. | `formal/`, `research/numerics-error-bounds.md` |
| Algorithm/numerics mismatch triage | `[map-input]` | contingent during writing | agents then Jorn for theorem/prose choices | Route `thesis/migration-findings.md` rows 3-11 before relying on existing algorithm boxes or numerical appendix prose: multiplier names, KKT sign convention, Q factor, beta/eigen thresholds, accumulator references, `|S| >= 2`, billiard adjacency pruning, and tube closing-edge status. | `thesis/migration-findings.md`, `thesis/algorithms.tex`, `thesis/appendix-numerical.tex` |
| Tube algorithm import | `[active]` | map input | Jorn proof, agents for routing | Write the current mathematical source from `research/tube-algorithm-raw-jorn-2026-05-04.md`, then write new thesis/formal/code surfaces from that source. Old tube thesis/formal/code surfaces were deleted from the active tree; use git history only if comparison is needed. | `research/tube-algorithm-raw-jorn-2026-05-04.md`, `research/tube-algorithm.md`, `thesis/migration-findings.md` rows 1 and 11-14 |

## Tube Algorithm Import Objective

Thesis project success means a defensible May 2026 thesis plus durable
supporting code and evidence. For the tube algorithm, that success condition is
not met by an empty implementation point, a raw note, or a stale agent-written
draft. It is met only if the algorithm becomes a thesis-usable
mathematical/computational route for computing capacity and the relevant simple
Reeb orbits, with evidence that the code matches the mathematics.

Therefore the active objective is to import the tube algorithm from Jörn's
current head/paper formalization into the repo as a current, traceable
thesis/code/evidence component. This objective is downstream of thesis success
and upstream of thesis prose, formalization, Rust implementation, and
validation: those surfaces should be driven from the current mathematical
source, not from the deleted stale tube files.

## Tube Algorithm Import Done State

The tube algorithm import is done when all of the following observable project
states hold:

1. A current mathematical tube source states the algorithm from
   `research/tube-algorithm-raw-jorn-2026-05-04.md`: `Tube(k,s,Acut)`,
   breakpoint order and locations, finite polygon-affine representation,
   primitive tubes, tube intersection, action restriction, closed-loop fixed
   points, exhaustive simple-word capacity search, and first-milestone
   exclusions such as no rotation pruning.
2. `thesis/` contains a correct tube-algorithm section or an explicit thesis
   decision to defer or cut it. If included, it matches the current
   mathematical source and does not claim empirical validation unless that
   evidence exists.
3. Rust implements the named objects and operations: primitive constructor,
   tube intersection, action restriction, closed-loop fixed-point solving,
   exhaustive simple-word search, and output of `capacity` plus simple Reeb
   orbits below `capacity + threshold`.
4. Current evidence shows that the implementation matches the source, covering
   primitive maps, polygon emptiness, intersection, action restriction, fixed
   points, and comparison to HK2017 on small eligible examples.
5. Old thesis/formal/Rust tube files are absent from the active tree, or else
   have been rewritten from the current mathematical source.
6. Task and index files point to the current mathematical source, thesis
   position, implementation, and evidence, with only concrete remaining
   follow-ups.

## Agent Cache

- [fresh 2026-04-24] Current code/thesis tension: thesis proves rank-deficient
  pairs redundant; code searches null space for beta>0 in near-singular systems.
  This is not automatically contradictory but needs wording if retained.
  Refresh by: reading `research/numerics-error-bounds.md` and projection solver
  code comments.
- [fresh 2026-04-24] Q-error-bound gap: `[lem:q-error-bound]` is too loose for
  near-singular KKT matrices. Code comments and ignored regression tests point
  here for the deferred replacement/tighter-bound work.
  Refresh by: reading `crates/symplectic/src/kkt/saddle_point_solver.rs`,
  `crates/symplectic/src/kkt/test_saddle_point_solver.rs`, and
  `formal/symplectic-polytope-geometry.tex`.
- [stale 2026-05-04] Pre-import tube algorithm status was blocked and not
  re-exported. The old thesis/formal/Rust tube surfaces were deleted from the
  active tree after `research/tube-algorithm-raw-jorn-2026-05-04.md` became the
  audited source. Refresh by reading the raw source and the current routing
  note, not the deleted files.
- [fresh 2026-05-04] Tube import status: use
  `research/tube-algorithm-raw-jorn-2026-05-04.md` as the raw audited source
  and `research/tube-algorithm.md` as the routing/clarification note. Old
  thesis/formal/Rust tube surfaces were deleted from the active tree because
  they were not trusted as the current algorithm specification. Refresh by
  reading the raw source, then the routing note, then git history only if
  comparison material is necessary.
- [fresh 2026-04-24] KKT notation decision: use the code's symmetric convention
  in thesis because eigenvalue decompositions are cleaner.
  Refresh by: checking thesis-code alignment notes.
- [fresh 2026-04-24] a_i replaces `(n,h)` for thesis notation; propagation is
  blocked on thesis restructuring.
  Refresh by: checking `tasks/writing.md` and current thesis notation.
- [fresh 2026-04-25] `thesis/appendix-numerical.tex` describes a
  certified/uncertain accumulator. The public `ehz_capacity*` wrappers now use
  `OrbitGuaranteeMode::MinimaSafe`; non-default guarantee controls exist behind
  the `aggregate_orbits_with_dual_vertices_exact` path. This must be made
  explicit if retained in thesis prose.
  Refresh by: reading `thesis/appendix-numerical.tex` around "Accumulator and
  Final Answer", `crates/symplectic/src/lib.rs`, and
  `crates/symplectic/src/algorithms/orbit_search.rs`.
- [fresh 2026-04-25] Current formal numerics state is not a fully proved
  numerical solver: `formal/hk2017-qp-core.tex` and
  `formal/hk2017-qp-precision.tex` contain exact per-sigma
  solver and trinary beta material, plus named gaps around near-threshold beta,
  empirical constants, and Taylor-cancellation algebra. Q-error experiments
  support known-polytopes/winner accuracy but do not remove those caveats.
  Refresh by: reading `formal/hk2017-qp-core.tex` and
  `formal/hk2017-qp-precision.tex` gap comments and
  `experiments/numerics/q-error/q_error_output.txt`.
- [fresh 2026-04-25] `thesis/migration-findings.md` rows 3-11 are the
  algorithm/numerics part of the thesis/code mismatch packet. Most are
  thesis-side exposition fixes or Jörn wording checks, not solver-development
  tasks.
  Refresh by: reading `thesis/migration-findings.md`.
- [fresh 2026-05-01; staleness caveat: source/output inspection, not rerun
  verification] A read-only audit of the strong numerics route returned verdict
  `WEAKENED`: the repo supports a truthful f64 diagnostic plus exact/empirical
  validation story, but not a claim that public `ehz_capacity*` wrappers are
  fully certified numerical solvers. The temporary report was removed with the
  repo-local temporary-file cleanup; refresh from source surfaces instead.
  Refresh by: rerunning the contract audit against `lib.rs`,
  `orbit_search.rs`, `formal/hk2017-qp-core.tex`,
  `formal/hk2017-qp-precision.tex`,
  `experiments/numerics/error-bounds/`, `q-error`, and `kkt-inertia` outputs.
- [fresh 2026-05-01] Current alignment snapshot: formal and experiment
  `error-bounds` surfaces are projection/null-space oriented, while public
  `ehz_capacity*` wrappers use saddle-point solving plus `MinimaSafe`
  aggregation by default. Exact rational certified output is available through
  `ehz_capacity_pruned_certified`; explicit non-default guarantee control is
  available through `aggregate_orbits_with_dual_vertices_exact`.
  Refresh by: checking `crates/symplectic/src/lib.rs` wrapper calls and
  `crates/symplectic/src/algorithms/orbit_search.rs` aggregation modes.
- [fresh 2026-05-01] Generic route resume point: first define exact generic
  conditions on intermediate variables, not input-polytope families. Candidate
  variables are full rank/condition of `C`, strict negative reduced Hessian on
  the retained tangent space, positive beta margin, positive Q/action gap from
  competitors, and adjacency/pruning assumptions. Then implement the exact
  generic case, mirror it in f64, run feedback loops that compare methods, and
  record limit behavior as conditions approach zero.
  Refresh by: reading the top "2026-05-01 generic-case pivot" section of
  `research/numerics-error-bounds.md`.

## Pruned / Stale

- [stale 2026-04-24] Pre-April cutoff solver polish plans are superseded.
  Thesis closeout needs route freeze and truthful wording.
