# Semantic Ascent Plan — 2026-04-17

## Goal

Reduce duplicated ascent machinery by sharing mathematically meaningful units of
work, without stabilizing the full heuristic ascent policy into `library/`.

## Design stance

- Keep heuristic search policy in experiments:
  overshoot schedules, wiggle escape, Armijo/backtracking, budgets, trace rows.
- Move stable mathematical objects into `library/` when they represent a real
  reusable stage rather than one experiment's glue.
- In experiment packages, share stage-shaped helpers such as direction
  construction, masking, and dual-step application before considering larger
  ascent-kernel unification.

## Implement now

### 1. Library ascent primitives
- Worktree: `improvement-ascent-lib-primitives`
- Scope:
  - `library/src/derivatives.rs`
  - `library/src/algorithms/billiard/facet_classification.rs`
- Why now:
  - experiments repeatedly rebuild the systolic-ratio gradient from
    `(cap * dc - sys * dv) / vol`
  - LP-preserving direction masking is a mathematical transform tied to facet
    classification, not search glue
- Expected shape:
  - add `sys_gradient_a_from_kkt_result(...)`
  - add `sys_gradient_a_from_orbit(...)`
  - add a facet-classification method to mask a dual-vertex direction in place
- Verification:
  - `cargo build -p symplectic --release --lib`
- Risk:
  - do not add optimizer policy, line-search logic, or experiment reporting
  - keep the APIs on mathematical objects, not on "ascent steps"

### 2. Sys-landscape shared ascent stages
- Worktree: `improvement-ascent-sys-kernel`
- Scope:
  - `experiments/sys-landscape/src/lib.rs`
  - `experiments/sys-landscape/gradient-ascent-general/main.rs`
  - `experiments/sys-landscape/gradient-ascent-products/main.rs`
- Why now:
  - the two binaries share the ascent kernel and differ mainly in direction
    policy and seed-generation glue
- Expected shape:
  - introduce shared experiment-local helpers for:
    - computing the `sys` ascent direction
    - applying LP masking conditionally via a mode enum or equivalent explicit
      branch
    - applying a dual step `a + t d`
  - keep overshoot/wiggle schedules and binary-specific generation/reporting
    local unless the shared boundary stays obviously cleaner
- Verification:
  - `cargo build -p exp-sys-landscape --release --bin sys-gradient-ascent-general --bin sys-gradient-ascent-products`
- Risk:
  - avoid hooks/callbacks
  - avoid widening into `variable-f-ascent` in the same packet
  - stop if the shared kernel starts swallowing binary-specific glue

## Next assessments to discuss while workers run

- whether `variable-f-ascent` should consume the same shared ascent stages or
  stay separate because of its trial orchestration and cache logic
- whether HKO orbit-analysis should get a shared experiment-local layer:
  near-optimal orbit filtering, systolic-ratio gradient construction, and
  subdifferential-derived direction objects
- whether the next library cleanup packet should be inline-test extraction from
  dense production files (`geom/polytope.rs`, `kkt/saddle_point_solver.rs`)

## Non-goals

- no promotion of heuristic ascent algorithms into `library/`
- no `.jsonl` refresh
- no redesign of HKO Armijo ascent in this packet
