<!--
Purpose: numerical appendix, solver, and method-claim roadmap.
Context: route-freeze surface for proof-vs-validation-vs-caveat decisions.
-->

# Numerics Roadmap

## Status

- State: active.
- Last updated: 2026-05-01.
- Source surfaces: `research/numerics.md`,
  `research/numerics-error-bounds.md`, `experiments/numerics/`,
  `formal/`, `tasks/verify-thesis-done.md`.
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

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Numerical appendix route freeze | `[map-input]` | mainline thesis | agent prep then Jorn | Decide whether thesis prose describes public f64 wrappers, the stronger exact/guaranteed verification layer, or both with an explicit boundary. | `research/numerics*.md`, `thesis/appendix-numerical.tex`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| Generic-case solver contract | `[active]` | mainline thesis | agent prep then Jorn math review | Draft the exact generic theorem with conditions on `C`, reduced Hessian eigenvalues, beta margin, Q/action gap, and adjacency/pruning assumptions; then align experiment f64 diagnostics to those variables. | `research/numerics-error-bounds.md`, `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `experiments/numerics/error-bounds/` |
| Numerical error bounds | `[map-input]` | contingent during writing | retained wording | Treat as proved exact/Q pieces plus empirical eta checks plus named caveats; fix/caveat only the pieces the thesis cites. Under the generic route, record each caveat as a generic precondition, non-generic limit behavior, empirical formula, or Jorn review question. | `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `experiments/numerics/q-error/q_error_output.txt`, `thesis-stories-are-supported.md` |
| Projection solver | `[map-input]` | contingent during writing | Jorn math if retained | Use the projection/null-space solver as the candidate generic-route implementation story. Do not claim it is the public capacity backend until `OrbitSolveBackend::Projected` returns the shared orbit payload and Q-bound contract. | `crates/symplectic/src/algorithms/orbit_search.rs`, `experiments/numerics/error-bounds/projection_solver.rs` |
| Beta-LP unification | `[future]` | future/follow-up by default | Jorn math if retained | Keep as future unless needed for retained solver explanation. | legacy beta-LP row |
| Solver formal writeup | `[map-input]` | contingent during writing | retained wording | Avoid full per-module formalization unless thesis text requires it. | `formal/`, `research/numerics-error-bounds.md` |
| Algorithm/numerics mismatch triage | `[map-input]` | contingent during writing | agents then Jorn for theorem/prose choices | Route `thesis/migration-findings.md` rows 3-11 before relying on existing algorithm boxes or numerical appendix prose: multiplier names, KKT sign convention, Q factor, beta/eigen thresholds, accumulator references, `|S| >= 2`, billiard adjacency pruning, and tube closing-edge status. | `thesis/migration-findings.md`, `thesis/algorithms.tex`, `thesis/appendix-numerical.tex` |
| Tube benchmark/formula | `[blocked]` | future/follow-up by default | Jorn proof | Do not unblock unless Jorn supplies formula and thesis payoff is worth delay. | `thesis/` TODOs, legacy tube rows |

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
- [fresh 2026-04-24] Tube algorithm status: blocked and not re-exported. The
  rotation-increment formula and thesis/formal TODOs need Jorn math before this
  can become thesis-relevant.
  Refresh by: reading `crates/symplectic/src/algorithms/tube/mod.rs` and the
  tube TODOs in `thesis/`.
- [fresh 2026-04-24] KKT notation decision: use the code's symmetric convention
  in thesis because eigenvalue decompositions are cleaner.
  Refresh by: checking thesis-code alignment notes.
- [fresh 2026-04-24] a_i replaces `(n,h)` for thesis notation; propagation is
  blocked on thesis restructuring.
  Refresh by: checking `tasks/writing.md` and current thesis notation.
- [fresh 2026-04-25] `thesis/appendix-numerical.tex` describes a
  certified/uncertain accumulator, but the public `ehz_capacity*` wrappers call
  f64-only aggregation by default; stronger guarantee modes exist behind the
  non-default `aggregate_orbits` path. This must be made explicit if retained
  in thesis prose.
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
  verification] `scratch/numerics-strong-route-state-report.md` records a
  read-only audit of the strong numerics route. Its verdict was `WEAKENED`:
  the repo supports a truthful f64 diagnostic plus exact/empirical validation
  story, but not a claim that public `ehz_capacity*` wrappers are fully
  certified numerical solvers.
  Refresh by: rerunning the contract audit against `lib.rs`,
  `orbit_search.rs`, `formal/hk2017-qp-core.tex`,
  `formal/hk2017-qp-precision.tex`,
  `experiments/numerics/error-bounds/`, `q-error`, and `kkt-inertia` outputs.
- [fresh 2026-05-01] Current alignment snapshot: formal and experiment
  `error-bounds` surfaces are projection/null-space oriented; public
  `ehz_capacity*` wrappers still use saddle-point solving plus f64-only
  aggregation by default; stronger exact/guaranteed aggregation is available
  only through explicit non-default `aggregate_orbits` modes.
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
