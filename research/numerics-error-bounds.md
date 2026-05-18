# Numerics Error-Bounds Research Note

## 2026-05-01 generic-case pivot

Staleness note: this section preserves the 2026-04-30/2026-05-01
numerics-strong-route audit and Jorn's follow-up steering. It is a resume cache,
not proof closure. Refresh it before thesis-facing wording by rereading
`formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `experiments/numerics/error-bounds/`,
`crates/symplectic/src/lib.rs`, and
`crates/symplectic/src/algorithms/orbit_search.rs`.

Current verdict from the read-only route audit: `WEAKENED`. The repo supports a
truthful story that public capacity wrappers are f64 diagnostics with recorded
residual/Q information, known-polytope validation, and stronger exact/guaranteed
aggregation available through non-default APIs. It does not support a claim that
the public `ehz_capacity*` wrappers are fully certified numerical solvers.

The route should now be generic-case first:

1. Formalize the exact generic per-sigma problem under conditions on
   intermediate variables: full-rank/well-conditioned `C`, separated reduced
   Hessian eigenvalues, strict beta margin, a non-small Q/action gap, and the
   boundary/adjacency assumptions needed by the capacity search.
2. Implement or isolate the exact generic case so the mathematical contract is
   visible before f64 thresholds enter.
3. Implement the f64 version against the same variables and return continuous
   diagnostics, not hidden binary claims: eigenvalues, singular values, beta
   margin, residual norms, Q/action intervals, and the precondition margins.
4. Use the experiment loop to compare f64 methods, fit candidate bound formulas,
   and reject methods whose error blows up faster than the diagnostics predict.
5. Treat non-generic polytopes by limits of generic instances that violate one
   or more conditions. The thesis-safe question is then whether the bound
   explodes as the condition margin goes to zero, and whether finite precision
   leaves the local generic neighborhood.

The main alignment issue is unchanged: `formal/hk2017-qp-core.tex`,
`formal/hk2017-qp-precision.tex`, and
the experiment harness are projection/null-space oriented, but the public
`ehz_capacity*` wrappers use saddle-point solving plus f64-only aggregation by
default. A thesis claim can say "certified under the stated generic
preconditions" only for a route whose code/analyzer checks those preconditions.
Otherwise the safe wording remains "f64 diagnostic with exact/empirical
validation and named caveats."

## Current task surface

## Context

The verify-numerics experiment produced Q-error evidence and β > 0
classification evidence across 51K problems (458 polytopes + 15 synthetic
families). A discussion on 2026-04-01 with Jörn produced a mathematical
specification for what the KKT solver should do, separating concerns that the
current saddle-point solver conflates. The 2026-05-01 steering narrows the
target: first prove and implement the exact generic case, then use f64
experiments to find a provable numerical route and to describe the blow-up near
non-generic limits.

## Scope

1. **Track the algorithm design discussion** (the specification below, the vertex-enumeration dead end, the projection solver insight) in this file.

2. **Write `formal/hk2017-qp-core.tex` and `formal/hk2017-qp-precision.tex`** formalizing:
   - The solver specification (inputs, outputs, trinary classification)
   - The projection solver algorithm (SVD of C → eigendecompose H' → classify → check β → compute Q)
   - Direction-dependent β error bounds (1/|λ_i| amplification in eigenvector directions of H')
   - How Q error is insensitive to β uncertainty in low-eigenvalue directions (structural theorem)
   - The backward stability bound: ‖r‖ ≤ ‖P_D b‖ + O(ε_mach ‖M‖ ‖x̃‖) (cite Higham 2002 Ch 8, Golub & Van Loan 2013 §8.1)
   - The connection to the capacity algorithm (minimum-length minimum-action orbit passes adjacency, so false negatives on boundary cases don't affect capacity)

3. **Implement the projection solver algorithm** in `projection_solver.rs`, following the specification. Test against the 51K-problem dataset. Compare results with the saddle-point solver when that comparison answers a current contract question.

4. **Iterate** using the autonomous loop: change solver → regenerate → analyze → check violations → repeat.

5. **Use the generic `reviewer` subagent with `$review` formal-math checks** proactively after writing the formal source.

## Out of scope

- Library promotion (library/src/kkt/ changes) — do after the algorithm is settled.
- Thesis chapter writing — `formal/hk2017-qp-core.tex` and
  `formal/hk2017-qp-precision.tex` are the source; thesis copies from them
  later.
- Merging to main — Jörn gates merges.
- Rational solver fallback for INDETERMINATE cases — design decision for later.

## Key files

- `experiments/numerics/error-bounds/` — all experiment files
  - `main.rs` — stage 2 binary (loads JSONL, exact + f64 solver, diagnostics)
  - `collect_poly.rs` — stage 1 binary for polytope σ-node collection
  - `projection_solver.rs` and `saddle_point_solver.rs` — f64 solver copies
  - `analyze.py` — stage 3 checks (propositions, bounds, β > 0 classification)
  - `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex` — solver specification, proved pieces,
    preconditions, and named gaps
  - `research/numerics-error-bounds.md` — full findings and status
  - `testdata/*.jsonl` — committed regression fixtures
- `library/src/kkt/projection_solver.rs` — reduced-gradient sign fixed in `e56cf161` (2026-04-12), with regression test `reduced_gradient_sign_distinguishes_fix`
- `library/src/kkt/qp_assembly.rs` — matrix assembly reference

## Prior findings

### Strong-route audit snapshot (2026-04-30)

The audit answered the solver-contract questions as follows:

- The formal/projection route solves, for fixed sigma,
  `max Q(beta) = 1/2 beta^T H beta` subject to `C beta = d` and
  `beta >= 0`. Boundary optima are meant to be covered by shorter sigma data,
  not by computing every boundary face in the same solve.
- The experiment route solves the projection/null-space problem: SVD for
  `C beta = d`, reduced Hessian `H' = V^T H V`, eigensolve on `H'`, and
  null-direction margin search.
- The public route still solves the augmented saddle-point KKT matrix for
  ordinary `ehz_capacity*` wrappers and aggregates with the f64-only path.
- Trinary predicates include beta positivity/admissibility. Second-order sign,
  near-null eigendirections, Q/action gaps, and admissibility should be treated
  as trinary contract surfaces when they affect retained claims.
- Continuous diagnostics include singular/eigenvalue values, beta margin,
  residual norms, `sigma_min(C)`, `||H||`, eta values, Q error, and action
  interval endpoints.

Proof-gap triage from that audit:

- Closed or nearly closed for the projection story: exact per-sigma setup,
  affine restriction, critical-point structure, boundary/interior split, and
  the Q first-order residual bound under full-row-rank/interior conditions.
- Preconditions: separated retained negative eigenvalues, non-small
  `sigma_min(C)`, interior beta margin, adjacency/boundary-drop link, and
  explicit use of guarantee aggregation for public certified output.
- Empirical: eta constants, eigendirection scaling, finite q-error coverage,
  and kkt-inertia threshold behavior.
- Must weaken unless repaired: thesis claims that the public wrappers are fully
  certified, that the projection solver is the public backend, or that the
  saddle-point residual-correction proof is publication-ready without the
  named code-side gap.
- Jorn review questions: boundary-drop/adjacency sufficiency, eta constants,
  saddle-point residual-correction acceptability, and final thesis wording.

### Algorithm specification (from 2026-04-01 discussion with Jörn)

The solver takes f64 (H, C, d) and returns:

1. **Affine set check:** Is {β : Cβ = d} non-empty? SVD of C determines this. If empty, return error.

2. **Second-order classification (trinary):** Project H onto null(C): H' = V^T H V. Eigendecompose H'.
   - TRUE (all eigenvalues < −ε): strict local max of Q on affine set.
   - FALSE (some eigenvalue > ε): saddle point. Max is on β = 0 boundary. Done for this σ-node — the capacity algorithm finds boundary optima via shorter permutations.
   - INDETERMINATE (eigenvalue in [−ε, ε]): can't tell. Proceed as if TRUE.

3. **β > 0 certification (trinary):** Does the critical point/set have β > 0?
   - Direction-dependent error bound: uncertainty in β* is largest along eigenvectors of H' with small |eigenvalue|. The "tube" of possible β* is elongated in these directions.
   - TRUE: min(β̃_j) minus the direction-dependent bound > 0.
   - FALSE: max possible β*_j < 0 for some j.
   - INDETERMINATE: can't tell.

4. **Q with error bound:** Under the stated full-rank/interior assumptions,
   |Q − Q*| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C). The intended structural explanation is
   that Q is insensitive to beta uncertainty in low-eigenvalue directions
   because Q varies as O(λ_i · |δβ_i|²) there; the first-order term vanishes by
   stationarity + null(C) orthogonality. The Taylor-cancellation algebra remains
   a named proof gap where `formal/hk2017-qp-core.tex` and
   `formal/hk2017-qp-precision.tex` mark it.

### Key insight: "continuity of variables"

Every numerical test/assertion should be a continuous function of the input. Rank, eigenvalue signs, β > 0 are discontinuous. Instead: track eigenvalue VALUES (continuous), margin = min(β_j) (continuous), residual norms (continuous). The trinary classification puts a continuous buffer zone (INDETERMINATE) around the discontinuity.

### Dead ends

- **Vertex enumeration as a third algorithm:** The claim "max of quadratic on polytope is at vertex" is true for convex (positive definite H) objectives but FALSE for our indefinite H. The max can be in the interior of a face. Vertex enumeration is not a valid algorithm for the EHZ problem.

- **LP-then-project fix for false negatives:** Projecting the LP result back onto {Cβ = d} via C's SVD helped 2/26 false negatives (stress-test) but NOT the 9 natural polytope false negatives. The approximate null-space direction (discarded eigenvector of M) isn't in null(C), so projecting back to Cβ = d pulls β back toward the boundary.

### Root cause of false negatives

All 9 natural false negatives: m=6, rank 10/11, margin_exact = 0.17 (well inside β > 0). The saddle-point solver's eigenvalue threshold (τ = 1e-3 · max|λ|) discards an eigenvalue whose eigenvector mixes H and C directions. The LP shifts β along this direction (shift ~2) but violates Cβ = d (residual ~0.6). The solver falls back to β₀ with margin ≈ 0. The projection solver would avoid this because it handles C and H separately.

### Validated results

- B3 (Q error bound): zero violations on 45,476 feasible problems. Max ratio 0.217.
- β > 0: zero false positives on 44,808 natural polytope problems.
- P5 (‖H‖/σ_min(C) ≤ 100): FALSIFIED on natural data (max 1310). Removed.
- P6 (‖r_β‖ < 1e-3): zero violations when gated on full-rank M.
- Structural theorem (first/second = 2): empirically verified on all full-rank cases.

### Capacity algorithm connection

The capacity algorithm iterates over all subsets S and cyclic permutations σ of S, with adjacency pruning. For the capacity-achieving orbit: it has minimum action, is simple, and has minimum combinatorial length. This orbit has all β* > 0 strictly (otherwise a shorter σ exists). Its permutation passes adjacency (it IS a Reeb orbit). So the solver only needs to correctly handle interior optima for permutations that pass adjacency. False negatives on boundary cases don't affect capacity.

However: this argument requires that the minimum-length minimum-action orbit passes adjacency pruning. This is Jörn's claim from the mathematical structure, not proven in the experiment.

### Subagent research results (2026-04-01)

Three opus subagents explored research directions. Key findings:

**QP algorithm analysis:** Vertex enumeration (enumerate all C(m,5) subsets of 5 nonzero β components, solve 5×5 systems) was proposed as a third algorithm. **It's wrong for our problem:** the claim "max of quadratic on polytope is at a vertex" holds for convex (H positive semidefinite) objectives, NOT for our indefinite H. For indefinite H, the max can be in the interior of a face. The subagent caught and corrected this. For concave objectives (H negative semidefinite), the max CAN be interior — and for our ω₀-derived H which is typically indefinite, neither vertex-max nor interior-max is guaranteed. Active-set enumeration over all faces would work but is more expensive.

**Backward stability:** The residual bound is ‖r‖ ≤ ‖P̂_D b‖ + c·(m+5)·ε_mach·‖M‖·‖x̃‖. Two terms: thresholding (discarded eigenspace projection of b, zero when full-rank) and rounding (O(ε_mach)). References: Higham 2002 Theorem 8.5 (backward stability of symmetric eigendecomposition), Golub & Van Loan 2013 Theorem 8.1.5 (Weyl eigenvalue perturbation). **Theorem numbers need physical verification** — they're from the subagent's knowledge, not read from the books.

**β > 0 certification:** The null-space component of δβ **cannot be bounded from the constraint residual alone** (proven impossibility: two exact solutions with different null components have identical Cβ). The operational approach: **reframe as feasibility**, not perturbation. Project β̃ onto {Cβ = d}, check positivity of the projection. The certification condition becomes: min_j β̃_j > ‖C^T(CC^T)^{-1}‖_{∞←2} · ‖Cβ̃ − d‖₂. This sidesteps the null-space bound entirely — you're free to pick any feasible point near β̃.

### Correlation findings

| Comparison | Finding |
|-----------|---------|
| Q error: full-rank vs rank-deficient M | max 8e-16 vs 382 — 10^18× gap. Rank deficiency dominates. |
| Q error: Q > 0 vs Q ≤ 0 | Q sign NOT the driver — rank deficiency is |
| Correction effectiveness by rank | Helps >10× in 28% full-rank, 48% rank-deficient |
| Margin vs σ_min(C) on natural | σ_min(C) < 0.01 → median margin 2e-12 (boundary) |
| β error vs margin | β accuracy excellent (1e-15) even for small margins |
| p_discard_b_norm vs Q error | Only 2 cases with p_discard_b > 1e-6 |

### Capacity algorithm and boundary optima

The capacity algorithm iterates over all subsets S ⊆ {1,...,F} and all cyclic permutations σ of S, with adjacency pruning per permutation (`is_feasible_cycle`). A subset S is enumerated but may produce zero solved permutations after all its orderings fail adjacency.

**Jörn's argument for why false negatives on boundary cases don't matter:** The capacity-achieving orbit has minimum action, is simple, and has minimum combinatorial length. This orbit has all β* > 0 strictly (if β_k = 0, a shorter σ exists, contradicting length minimality). Its permutation passes adjacency (it IS a Reeb orbit — every transition physically occurs). So the solver only needs correct interior-β > 0 classification for permutations that pass adjacency. The adjacency pruning doesn't kick out minimum-length minimum-action simple Reeb orbits because it only removes (i,j) pairs where no locally simple Reeb trajectory exists.

**However:** This argument is from the mathematical structure (Jörn's domain knowledge), not proven in the experiment. And false negatives on non-capacity-achieving σ-nodes could still affect other uses of the solver.

### Design principles from session

- **"Continuity of variables":** Every numerical test should be a continuous function of input. Rank, eigenvalue signs, β > 0 are discontinuous → use trinary TRUE/FALSE/INDETERMINATE with continuous buffer zones.
- **Seeds are fragile:** Don't use seeded RNG for reproducibility. Generate once, store as JSONL, commit. The artificial.jsonl is the source of truth, not the generation code.
- **SingularMatrix represents a real case:** All eigenvalues ≈ 0 is a valid mathematical outcome (not garbage input). Stays as a KktOutcome variant. Callers handle it as non-feasible. (Overruled by Jörn 2026-04-07; previously said "garbage input, now a panic.")
- **Rank is not numerically testable:** For matrices H, H* with ‖H − H*‖ < δ, rank(H) and rank(H*) can differ. No finite-precision computation distinguishes eigenvalue 0 from eigenvalue 1e-15.

### Scope and iteration guidance

**Agent owns:** All experiment files (`main.rs`, `projection_solver.rs`, `saddle_point_solver.rs`, `collect_poly.rs`, `analyze.py`), `formal/hk2017-qp-core.tex`, `formal/hk2017-qp-precision.tex`, `research/numerics-error-bounds.md`, JSONL data, `tasks/numerics.md` updates, solver algorithm changes.

**Needs Jörn:** GAP in cor:taylor-structure proof, mathematical review of new bounds, merge to main, scope decisions.

**Iteration scope (things to consider changing):** Filter criteria, diagnostic fields, proposition thresholds, new propositions/bounds, polytope sources, analyze.py checks, algorithms in solvers.rs (new error correction, margin optimization, diagnostic variables).

**Iteration feedback:** checks.txt violations/ranges, diff from previous run, correlation hunting, coverage gaps, tightness of bounds approaching 1.0, natural vs artificial comparison, independent audit via the generic `reviewer` subagent with `$review` formal-math checks.

## Current execution state

This note is being refreshed from the assigned worktree
`/workspaces/msc-math/.worktrees/numerics-strong-route` on branch
`numerics-strong-route`. Older branch details from
`.claude/worktrees/verify-numerics-q-accuracy` are historical and should not be
used as the current cwd or merge state.

To regenerate data:
```bash
cargo run -p dev-numerical-analysis --release --bin num-collect-poly -- --polytopes /tmp/all_polytopes.jsonl --max-facets 8
cargo run -p dev-numerical-analysis --release --bin num-error-bounds -- <input.jsonl> <output.jsonl>
uv run experiments/numerics/error-bounds/analyze.py
```

The `/tmp/all_polytopes.jsonl` is a concatenation of correctness + random-product-sample + benchmark + ablation data. Recreate with:
```bash
cat experiments/verification/correctness/correctness.jsonl experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl experiments/verification/algorithm-comparison/ablation/ablation.jsonl > /tmp/all_polytopes.jsonl
```

## Success criteria

1. `formal/hk2017-qp-core.tex` and `formal/hk2017-qp-precision.tex` state the exact generic solver
   specification as definitions and lemmas, with each non-generic case either
   excluded by a precondition or routed to a limit/indeterminate discussion.
2. `formal/hk2017-qp-core.tex` and `formal/hk2017-qp-precision.tex` prove the direction-dependent beta error
   bound under those generic preconditions, or marks the remaining missing
   lemma with a clear GAP/Jorn question.
3. `experiments/numerics/error-bounds/projection_solver.rs` matches the generic
   specification and returns the diagnostics used by the theorem.
4. `analyze.py` and/or Rust tests check the same precondition margins and bound
   formulas that the theorem states.
5. `research/numerics-error-bounds.md` records the algorithm design discussion,
   method comparisons, empirical failures, and non-generic limit behavior.
6. Generic `reviewer` subagent finds no blocking formal-math issues.
7. `cargo build -p dev-numerical-analysis --release --bin num-error-bounds --bin num-collect-poly` succeeds.
