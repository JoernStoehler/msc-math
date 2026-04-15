# Task: Write up KKT solver algorithm specification + correctness proofs in math.tex

## Context

The verify-numerics experiment has proven Q error bounds and empirically validated β > 0 classification across 51K problems (458 polytopes + 15 synthetic families). A discussion on 2026-04-01 with Jörn produced a clean mathematical specification for what the KKT solver should do, separating concerns that the current saddle-point solver conflates. The next step is to formalize this specification in math.tex, prove correctness of the error bounds within this framework, and iterate on the algorithm in solvers.rs.

## Scope

1. **Update logbook.md** with the algorithm design discussion (the specification below, the vertex-enumeration dead end, the projection solver insight).

2. **Write math.tex** formalizing:
   - The solver specification (inputs, outputs, trinary classification)
   - The projection solver algorithm (SVD of C → eigendecompose H' → classify → check β → compute Q)
   - Direction-dependent β error bounds (1/|λ_i| amplification in eigenvector directions of H')
   - How Q error is insensitive to β uncertainty in low-eigenvalue directions (structural theorem)
   - The backward stability bound: ‖r‖ ≤ ‖P_D b‖ + O(ε_mach ‖M‖ ‖x̃‖) (cite Higham 2002 Ch 8, Golub & Van Loan 2013 §8.1)
   - The connection to the capacity algorithm (minimum-length minimum-action orbit passes adjacency, so false negatives on boundary cases don't affect capacity)

3. **Implement the projection solver algorithm** in solvers.rs, following the specification. Test against the 51K-problem dataset. Compare results with the saddle-point solver.

4. **Iterate** using the autonomous loop: change solver → regenerate → analyze → check violations → repeat.

5. **Use the generic `reviewer` subagent with `$review` formal-math checks** proactively after writing the formal source.

## Out of scope

- Library promotion (library/src/kkt/ changes) — do after the algorithm is settled.
- Thesis chapter writing — the experiment's math.tex is the source; thesis copies from it later.
- Merging to main — Jörn gates merges.
- Rational solver fallback for INDETERMINATE cases — design decision for later.

## Key files

- `experiments/numerics/error-bounds/` — all experiment files
  - `main.rs` — stage 2 binary (loads JSONL, exact + f64 solver, diagnostics)
  - `collect_inputs.rs` — stage 1 binary (generates artificial.jsonl + collected.jsonl)
  - `solvers.rs` — f64 solver copy (sign-fixed, no panics, LP-then-project fix)
  - `analyze.py` — stage 3 checks (propositions, bounds, β > 0 classification)
  - `math.tex` — proven bounds (3 lemmas, 2 corollaries, 1 remark, 1 GAP)
  - `logbook.md` — full findings and status
  - `artificial.jsonl` — 4303 synthetic problems (committed)
  - `collected.jsonl` — 1.66M polytope σ-nodes (gitignored, regenerate with collect_inputs.rs)
  - `results.jsonl` — 51K problems with exact ground truth (committed)
  - `checks.txt` — latest analysis output
- `library/src/kkt/projection_solver.rs` — reduced-gradient sign fixed in `e56cf161` (2026-04-12), with regression test `reduced_gradient_sign_distinguishes_fix`
- `library/src/kkt/qp_assembly.rs` — matrix assembly reference

## Prior findings

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

4. **Q with error bound:** |Q − Q*| ≤ ‖H‖·‖β‖·‖r‖/σ_min(C) (proven, B3). The bound is insensitive to β uncertainty in low-eigenvalue directions because Q varies as O(λ_i · |δβ_i|²) there — the first-order term vanishes by stationarity + null(C) orthogonality.

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

**Agent owns:** All experiment files (main.rs, solvers.rs, collect_inputs.rs, analyze.py, math.tex, logbook.md), JSONL data, TASKS.md updates, solver algorithm changes.

**Needs Jörn:** GAP in cor:taylor-structure proof, mathematical review of new bounds, merge to main, scope decisions.

**Iteration scope (things to consider changing):** Filter criteria, diagnostic fields, proposition thresholds, new propositions/bounds, polytope sources, analyze.py checks, algorithms in solvers.rs (new error correction, margin optimization, diagnostic variables).

**Iteration feedback:** checks.txt violations/ranges, diff from previous run, correlation hunting, coverage gaps, tightness of bounds approaching 1.0, natural vs artificial comparison, independent audit via the generic `reviewer` subagent with `$review` formal-math checks.

## Branch state

Worktree at `.claude/worktrees/verify-numerics-q-accuracy`, branch `verify-numerics-q-accuracy`, 12 commits ahead of main. Clean working tree (last commit: `c0b30f9`).

To regenerate data:
```bash
cargo run -p dev-numerical-analysis --release --bin collect_inputs -- artificial
cargo run -p dev-numerical-analysis --release --bin collect_inputs -- natural --polytopes /tmp/all_polytopes.jsonl --max-facets 8
cargo run -p dev-numerical-analysis --release --bin verify_numerics
uv run experiments/numerics/error-bounds/analyze.py
```

The `/tmp/all_polytopes.jsonl` is a concatenation of correctness + random-product-sample + benchmark + ablation data. Recreate with:
```bash
cat experiments/verification/correctness/correctness.jsonl experiments/sys-landscape/random-product-sample/random-product-sample.jsonl experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl experiments/verification/algorithm-comparison/ablation/ablation.jsonl > /tmp/all_polytopes.jsonl
```

## Success criteria

1. `math.tex` has the solver specification formalized as definitions + lemmas
2. `math.tex` has direction-dependent β error bound proven (or marked GAP with clear statement)
3. `solvers.rs` has projection solver implementation matching the specification
4. `analyze.py` reports zero violations on proven bounds with the new solver
5. `logbook.md` documents the algorithm design discussion and all findings
6. Generic `reviewer` subagent finds no blocking formal-math issues
7. `cargo build --release --bin verify_numerics --bin collect_inputs` succeeds
