<!--
Purpose: numerical appendix, solver, and method-claim roadmap.
Context: route-freeze surface for proof-vs-validation-vs-caveat decisions.
-->

# Numerics Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-25.
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

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Numerical appendix route freeze | `[map-input]` | mainline thesis | agent prep then Jorn | Decide whether thesis prose describes public f64 wrappers, the stronger exact/guaranteed verification layer, or both with an explicit boundary. | `research/numerics*.md`, `thesis/appendix-numerical.tex`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| Numerical error bounds | `[map-input]` | contingent during writing | retained wording | Treat as proved exact/Q pieces plus empirical eta checks plus named caveats; fix/caveat only the pieces the thesis cites. | `formal/numerics/error-bounds.tex`, `experiments/numerics/q-error/q_error_output.txt`, `thesis-stories-are-supported.md` |
| Projection solver | `[future]` | future/follow-up by default | Jorn math if retained | Record tension between code and thesis; defer broad unification. The projected backend is scaffold-only on the shared orbit payload because it lacks the Q-bound contract. | `crates/symplectic/src/algorithms/orbit_search.rs`, legacy projection row |
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
  `formal/library/geom.tex`.
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
  numerical solver: `formal/numerics/error-bounds.tex` contains exact per-sigma
  solver and trinary beta material, plus named gaps around near-threshold beta,
  empirical constants, and Taylor-cancellation algebra. Q-error experiments
  support known-polytopes/winner accuracy but do not remove those caveats.
  Refresh by: reading `formal/numerics/error-bounds.tex` gap comments and
  `experiments/numerics/q-error/q_error_output.txt`.
- [fresh 2026-04-25] `thesis/migration-findings.md` rows 3-11 are the
  algorithm/numerics part of the thesis/code mismatch packet. Most are
  thesis-side exposition fixes or Jörn wording checks, not solver-development
  tasks.
  Refresh by: reading `thesis/migration-findings.md`.

## Pruned / Stale

- [stale 2026-04-24] Pre-April cutoff solver polish plans are superseded.
  Thesis closeout needs route freeze and truthful wording.
