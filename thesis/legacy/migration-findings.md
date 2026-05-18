# Thesis ↔ Code Mismatches Found During Migration

Compiled from 4 review agents. Each item needs Jörn's decision on which side to fix.

## Must fix (code is wrong)

1. **`tube/mod.rs` `compute_rotation_increment`**: Doc comment claims it implements `[def:rotation-increment]` (CH2021 Lem. 2.21). It doesn't — it's a heuristic (angle between Reeb vectors, clamped). The function itself acknowledges this in a 300-line comment. Fix: change doc comment to mark as approximate.

2. **Label mismatch**: Code says `[thm:billiard-characterization]` and `[thm:bounce-bound]`, thesis has `\label{thm:billiard-completeness}`. One side needs renaming.

## Notation inconsistencies (thesis-side fix likely better)

3. **KKT multiplier names**: `(λ, ν)` in main algorithm text vs `(μ, ξ)` in appendix and code. No mapping stated.

4. **KKT matrix signs**: `[alg:ehz]` Step 2 shows `-N`, `-η`. Code and appendix use `+N`, `+η` (symmetric form). Equivalence proven in `[lem:kkt]` but no cross-reference from the algorithm box.

5. **Q definition factor-of-2**: Thesis `[alg:ehz]` Step 4 records `β^T H β`; code uses `Q = (1/2) β^T H β`. Thesis `c_EHZ = (max β^T H β)^{-1}` = code `capacity = 0.5 / q_max`. No pointer between them.

6. **Two epsilon_beta thresholds**: Thesis appendix uses one `ε_β`. Code has `EPS_BETA_POSITIVE = 1e-12` (eigensolver noise) and `EPS_MARGIN_TRUE/FALSE = 1e-9` (verdict boundary) — two distinct thresholds at different magnitudes.

7. **Two-tier eigenvalue threshold**: Thesis appendix `[alg:q-error-bound]` describes single threshold `τ`. Code has `EPS_EIGEN_FLOOR = 1e-12` (absolute) + `EIGEN_CONDITION_TAU = 1e-3` (relative). Appendix is out of date.

## Algorithm description gaps (thesis missing steps)

8. **Accumulator not in algorithm boxes**: The two-tier certified/uncertain tracking and gap invariant are described in appendix A.3–A.4 but `[alg:ehz]` and `[alg:billiard]` don't reference them. A reader implementing from the algorithm box alone will miss this.

9. **|S| ≥ 2 not stated**: `[alg:ehz]` says "nonempty subset" but singletons have no admissible β. Code correctly starts at m=2.

10. **Billiard missing adjacency pruning**: Code calls `is_adjacent_cycle` before KKT solve. `[alg:billiard]` has no such step.

11. **Tube missing closing edge check**: Code checks `has_close_edge_1 && has_close_edge_2`. Thesis has this as open TODO `[JÖRN Q4]` — code has resolved it.

## Code ↔ thesis structural divergences (Jörn decides which to align)

12. **Tube data in R^4 vs local 2D**: Thesis `[def:tube-data]` says 2×2 matrices in local coords. Code uses 4×4 matrices in R^4 with `try_inverse()`. Thesis doesn't discuss singularity handling.

13. **Tube step maps computed on-the-fly**: Thesis says precompute Φ_{ijl} for all valid triples. Code computes during DFS.

14. **Tube action function**: Thesis gives exact affine formula. Code uses affine fit via least squares. Approximation error affects `[lem:prune-action]` correctness.

15. **18 cross-ref labels invented by agents**: See `/tmp/review-target-completeness.md` for the full list. Most are plausible definitions/lemmas that should exist but don't have `\label{}` in the thesis yet.

## Convention gap that caused these issues

`rust-conventions/SKILL.md` lines 72-74 say "Definitions, lemmas, and proofs live as doc comments" and "Rust crates are self-contained mathematically." But line 83 says "Never duplicate proofs inline — comment says *what*, thesis says *why*." These contradict — agents interpreted the first rule and invented labels/math that doesn't exist in the thesis.
