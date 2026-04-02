# Gradient Search — Logbook

## Motivation

Existing gradient experiments (gradient-descent, sys-optimization) all hit the step-bound
barrier: gradient ascent converges within each combinatorial cell but cannot cross boundaries.
Best non-HKO sys: 0.9049 (gradient-descent, Lagrangian 5x5). This experiment adds boundary
crossing via overshooting the step bound, and wiggling (random height perturbation) to escape
local optima.

## Status: pipeline validated, computation not yet run at scale

## How to run

```bash
cd crates/exp-sys-optimization/gradient-search/
# 1. Generate seeds (appends, skips existing)
cargo run --bin opt-generate-seeds --release
# 2. Optimize (appends, skips completed seed_ids)
cargo run --bin opt-gradient-search --release
```

Interrupt with Ctrl+C at any time. Re-run to resume.
Expand by increasing PLAN counts in generate_seeds.rs and re-running both.

## Files

| File | Role |
|------|------|
| generate_seeds.rs | Binary: creates random polytopes, writes seeds.jsonl |
| run.rs | Binary: reads seeds, gradient ascent + overshoot + wiggle, writes results.jsonl |
| seeds.jsonl | Input data: polytopes to optimize |
| results.jsonl | Output data: optimization results per seed |
| logbook.md | This file |

## Architecture

Two-binary pipeline. `generate_seeds` produces polytopes, `gradient_search` consumes them.
Both are idempotent (skip existing work). Data files are the interface. No threading.
Embarrassingly parallel: different seed ranges can run on different machines.

## Algorithm

Per seed:
1. Gradient ascent (h-only) with step bounds (preserving combinatorial type)
2. When converged: try overshooting step bound at 1.5x, 2x, 3x t_max
3. If overshoot improves sys: back to gradient ascent (now in new combinatorial cell)
4. If not: wiggle (perturb heights +-5%), gradient ascent from perturbation
5. Repeat escape attempts up to 3 rounds

## Design decisions

- **H-only gradient**: simpler than (h,n). Boundary crossing provides escape that (h,n) normally gives within cells. Upgrade to new gradient API planned.
- **No facet manipulation in v1**: remove/split facets deferred. Gradient + overshoot + wiggle is the minimal viable approach.
- **Step-bound overshoot**: the key novelty. Existing experiments stop at t_max. We try going past it. The new polytope has different combinatorial type but is still valid (Polytope4D::from_f64 revalidates).
- **Random seeds only**: not seeding from gradient-descent's converged results or HKO perturbations. Simpler, independent exploration.

## Findings

### Smoke test (20 seeds, 3-5 per F=7..10)

- 100% improvement rate (all seeds improved by gradient ascent)
- Best sys: 0.835
- Mean gradient iterations: ~17 per seed
- F=10 seeds take ~10s each, F=7 seeds ~0.5s each

### Profiled per-eval cost (release, single thread)

| F | gen (ms) | vol (ms) | cap (ms) |
|---|----------|----------|----------|
| 7 | 1.4 | 1.6 | 0.3 |
| 8 | 1.8 | 2.3 | 1.6 |
| 10 | 3.4 | 2.9 | 20 |

## Future work

- Run at scale: 100+ seeds per F, production PLAN
- Upgrade to simplified gradient API (being developed by another agent)
- Add slurm orchestration for LICCA
- Analyze: sys distribution by F, effect of overshoot vs within-cell-only, convergence curves
- Possibly add (h,n) gradient, facet manipulation
