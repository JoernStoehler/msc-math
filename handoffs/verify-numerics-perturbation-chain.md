# Task: Write the f64 per-σ solver algorithm and validate error bounds

## Context

The verify-numerics experiment develops a provably correct f64 algorithm for the HK2017 EHZ capacity computation. The pipeline (4 stages: collect → filter → diagnose → analyze) is implemented and working. math.tex has the perturbation chain (Links 1-5) with an explicit β certification bound (eq:eta-computable). The bound works for problems with well-separated eigenvalues but doesn't cover the null-eigenvalue case (LP search step). The f64 algorithm itself (Part III of math.tex) is not yet written.

## Scope

1. **Write the f64 algorithm** as `\begin{algorithm}` in math.tex Part III. Each step is a continuous threshold test, each threshold traced to a perturbation bound, proven TRUE/FALSE/INDET correctness. This is the main deliverable.

2. **Extend the η_k bound** to cover the null-eigenvalue case. When H' has near-zero eigenvalues, the solver uses an LP to search the null eigendirections. The bound currently doesn't account for this — the LP shift is O(1) but the bound computes η ≈ ε_mach. The error comes from O(‖δV‖ × ‖t‖) where t is the LP solution magnitude.

3. **Validate on natural data** using the pipeline. Current results: 86% certified on well-separated-eigenvalue problems, 39 violations on null-eigenvalue problems.

## Out of scope

- Library promotion (crates/src/kkt/ changes)
- Thesis chapter writing
- Merging to main

## Key files

- `experiments/verify-numerics/math.tex` — perturbation chain, η_k bound
- `experiments/verify-numerics/solvers.rs` — `solve_projected_with_diagnostics()`, `compute_eta_bound()`
- `experiments/verify-numerics/run.rs` — stage 3 (one input, one output)
- `experiments/verify-numerics/analyze.py` — stage 4 (multiple inputs)
- `experiments/verify-numerics/collect_poly.rs` — stage 1 natural
- `experiments/verify-numerics/collect_synth.rs` — stage 1 synthetic
- `experiments/verify-numerics/filter_poly_smoke.rs` — ~6 rows, pipeline smoke test
- `experiments/verify-numerics/filter_poly_diverse.rs` — ~1500 rows, per-polytope diverse
- `experiments/verify-numerics/filter_synth_all.rs` — pass-through
- `experiments/verify-numerics/collect_common.rs` — shared types (InputRow with β, λ)
- `experiments/verify-numerics/Makefile` — `make smoke` (seconds), `make full` (~3.5 min)

## Prior findings

### Pipeline
4-stage flow works end-to-end. `make smoke` runs in <1s (6 rows). `make full` runs in ~3.5 min (1535 poly + 4303 synth). Stage 1 output (collected_*.jsonl) is gitignored, regenerated via collect_poly/collect_synth. Stage 1 saves raw β, λ vectors.

### η_k bound (eq:eta-computable)
- **Valid** for problems where all retained eigenvalues of H' satisfy |γ̃_j| ≫ ‖ΔH'‖ (the first-order perturbation regime). Zero violations, c=m² safety factor.
- **Not valid** for null-eigenvalue cases (k=1, H' ≈ 0). 39/1192 natural problems. Root cause: the bound covers the critical-point perturbation α* = -(H')⁻¹g but not the LP search in the null eigenspace. The LP shift is O(1), changing β by O(0.1), while the bound predicts O(ε_mach).
- **Certification rate:** 86% on natural data (well-separated eigenvalue cases only).

### Session failures (feedback for next agent)
- Don't run long commands in foreground. Use `run_in_background` for all cargo run.
- Don't continue old work after a blocker is identified. Fix the blocker first.
- Respond to messages immediately. Don't queue them behind tool calls.
- Run natural data first when asked, not synthetic.
- A "smoke test" is ~10 rows and seconds, not 1500 rows and minutes.

## Branch state

Worktree at `.claude/worktrees/verify-numerics-math/`, branch `verify-numerics-q-accuracy`, 8 commits ahead of branch base. Clean working tree after this commit.

Regenerate stage 1 data:
```bash
cd experiments/verify-numerics
cat ../../experiments/correctness/correctness.jsonl ../../experiments/random-product-sweep/random-product-sweep.jsonl ../../experiments/benchmark/benchmark.jsonl ../../experiments/ablation/ablation.jsonl > /tmp/all_polytopes.jsonl
cd .. && cargo run --release --bin collect_poly -- --polytopes /tmp/all_polytopes.jsonl --max-facets 8
cargo run --release --bin collect_synth
```

## Success criteria

1. math.tex has the f64 algorithm as `\begin{algorithm}` in Part III
2. η_k bound extended to cover null-eigenvalue LP search
3. Zero bound violations on natural data
4. `make smoke` passes, `make full` reports results
