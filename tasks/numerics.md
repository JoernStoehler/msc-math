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
  `formal/`, `FINAL-VERIFICATION.md:T2.4`.
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
| Numerical appendix route freeze | `[map-input]` | mainline thesis | agent prep then Jorn | Summarize proof-vs-validation-vs-caveat options for retained numerical claims. | `research/numerics*.md` |
| Numerical error bounds | `[map-input]` | contingent during writing | retained wording | Fix/caveat only the pieces the thesis cites. | `FINAL-VERIFICATION.md:T2.4.3` |
| Projection solver | `[future]` | future/follow-up by default | Jorn math if retained | Record tension between code and thesis; defer broad unification. | legacy projection row |
| Beta-LP unification | `[future]` | future/follow-up by default | Jorn math if retained | Keep as future unless needed for retained solver explanation. | legacy beta-LP row |
| Solver formal writeup | `[map-input]` | contingent during writing | retained wording | Avoid full per-module formalization unless thesis text requires it. | `formal/`, `research/numerics-error-bounds.md` |
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

## Pruned / Stale

- [stale 2026-04-24] Pre-April cutoff solver polish plans are superseded.
  Thesis closeout needs route freeze and truthful wording.
