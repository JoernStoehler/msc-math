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

5. **Use review-proof and review-formalization agents** proactively after writing math.tex.

## Out of scope

- Library promotion (crates/src/kkt/ changes) — do after the algorithm is settled.
- Thesis chapter writing — the experiment's math.tex is the source; thesis copies from it later.
- Merging to main — Jörn gates merges.
- Rational solver fallback for INDETERMINATE cases — design decision for later.

## Key files

- `/workspaces/msc-math/.claude/worktrees/verify-numerics-q-accuracy/experiments/verify-numerics/` — all experiment files
  - `run.rs` — stage 2 binary (loads JSONL, exact + f64 solver, diagnostics)
  - `collect_inputs.rs` — stage 1 binary (generates artificial.jsonl + collected.jsonl)
  - `solvers.rs` — f64 solver copy (sign-fixed, no panics, LP-then-project fix)
  - `analyze.py` — stage 3 checks (propositions, bounds, β > 0 classification)
  - `math.tex` — proven bounds (3 lemmas, 2 corollaries, 1 remark, 1 GAP)
  - `logbook.md` — full findings and status
  - `artificial.jsonl` — 4303 synthetic problems (committed)
  - `collected.jsonl` — 1.66M polytope σ-nodes (gitignored, regenerate with collect_inputs.rs)
  - `results.jsonl` — 51K problems with exact ground truth (committed)
  - `checks.txt` — latest analysis output
- `/home/vscode/.claude/plans/peppy-finding-ritchie.md` — detailed plan with phases
- `/workspaces/msc-math/crates/src/kkt/projection_solver.rs:93` — library sign bug (unfixed)
- `/workspaces/msc-math/crates/src/kkt/qp_assembly.rs` — matrix assembly reference

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

## Branch state

Worktree at `.claude/worktrees/verify-numerics-q-accuracy`, branch `verify-numerics-q-accuracy`, 12 commits ahead of main. Clean working tree (last commit: `c0b30f9`).

To regenerate data:
```bash
cd experiments/
cargo run --release --bin collect_inputs -- artificial
cargo run --release --bin collect_inputs -- natural --polytopes /tmp/all_polytopes.jsonl --max-facets 8
cargo run --release --bin verify_numerics
python3 verify-numerics/analyze.py
```

The `/tmp/all_polytopes.jsonl` is a concatenation of `correctness.jsonl + random-product-sweep.jsonl + benchmark.jsonl + ablation.jsonl`. Recreate with:
```bash
cat correctness/correctness.jsonl random-product-sweep/random-product-sweep.jsonl benchmark/benchmark.jsonl ablation/ablation.jsonl > /tmp/all_polytopes.jsonl
```

## Success criteria

1. `math.tex` has the solver specification formalized as definitions + lemmas
2. `math.tex` has direction-dependent β error bound proven (or marked GAP with clear statement)
3. `solvers.rs` has projection solver implementation matching the specification
4. `analyze.py` reports zero violations on proven bounds with the new solver
5. `logbook.md` documents the algorithm design discussion and all findings
6. `review-proof` agent finds no issues in math.tex
7. `cargo build --release --bin verify_numerics --bin collect_inputs` succeeds
