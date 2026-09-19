# Variation interface closure

2026-09-18. Assignment V, isolated branch `research/variation-interface-check`
from `research/technical-closure-plan` at `60f4fb0b`. No active thesis edited.

## Outcome and remaining integration

The recovered Chapter 06 correctly distinguishes a persistent smooth branch
envelope, finite-step numerical search, and HKO feasible-section upper functions.
Its row/KKT/volume conventions agree with the implemented formulas. All 22
focused tests run here passed, including the normally ignored derivative tests.
This is bounded formula/implementation evidence, not a generic branch-coverage
certificate or full validation of optimizer trajectories.

Two required integration changes are identified:

1. The twelve-start illustration remains accurate but is not the later
   64-start/seven-policy study. Refer to the latter in its own optimizer account,
   with the historical evaluator and atomic budget rule; do not change `12` to
   `64` in place.
2. The statement that the branch-window model has no reproducible finite-distance
   validation is stale as an inventory statement. A completed, outcome-selected
   three-case diagnostic gives *negative* evidence at practical step sizes and
   checks small-step derivative convergence. It does not validate a trust policy.
   A small optional prose patch is supplied in `variation-integration.patch`.

The chapter inspected is
`thesis/candidate/recovered/thesis-candidate/06-variation.tex`, SHA256
`d84a787432466062e127df0e0a1193001ec2b1d64c7fb193936c6d30d8485261`.
The corresponding interrupted production-checkout file has the same hash.

## Checked formula contract

- Rows are `a_i=n_i/h_i` and coordinates `(q1,q2,p1,p2)`. Both code and
  `formal/capacity-derivatives.tex` use `J(q,p)=(-p,q)` and
  `omega(u,v)=<Ju,v>`. Chapter 06 refers to the QP objective
  `Q=sum_(j<k) beta_j beta_k omega(a_j,a_k)` and uses `c=1/(2Q)`.
- The chapter writes `D_beta Q=C^T lambda`. The runtime saddle system is
  `H beta+N mu+1 xi=0`, so `lambda=-(mu,xi)`; the apparently different
  minus/plus multiplier signs agree. At closure, with
  `P_i=sum_(j<i) beta_j a_j`, direct differentiation of Q gives
  `D_(a_k)Q=beta_i J(2P_i+beta_i a_k)`. Hence the optimized derivative is
  `beta_i [J(2P_i+beta_i a_k)+mu]`, exactly the f64 and exact code formulas.
- Multiplication by `-1/(2Q^2)` gives the branch action gradient. The exact
  helper asserts positive Q and a valid partial permutation, but it does not
  establish nondegeneracy, optimality or a smooth local family. Exact arithmetic
  alone cannot provide those hypotheses.
- The code's volume chain rule simplifies to `-S_i centroid_i/||a_i||`, matching
  the chapter's boundary-velocity argument. Runtime facets with volume below
  `1e-30` receive zero contribution; the thesis formula is mathematical, not a
  claim that this approximation is exact. The code's old module-header formula
  had the wrong sign on its tangential term; its actual body was correct.
- `systolic_ratio_gradient_a` computes `c/V * Dc - sys/V * DV`, equivalent to
  the chapter's `DU/U=-2Dq/q-DV/V` on a positive branch.
- Chapter 06 requires full rank, positive weights, negative constrained Hessian,
  complete local branch coverage and persistent discarded-word gaps before
  asserting the minimum of active branch slopes. Neither a tolerance action
  window nor scalar support reduction implies those hypotheses. Its explicit
  zero-weight warning correctly prevents transferring reduced support values
  to all nearby slopes.
- HKO only uses smooth positive feasible sections touching the global value at
  the base point. Its 26 upper bounds and transverse-slice argument do not need
  a continuation of all optimizing branches. No HKO re-audit or new nongeneric
  theory was performed or is implied by this check.

## Tests actually executed

Commands used `CARGO_TARGET_DIR=/workspaces/msc-math/target` for the shared build
cache, with this isolated checkout as cwd. Compilation took 23.76s. No producer
or generated evidence was replaced.

| Command after `cargo test -p symplectic --release --lib` | Result | What it checks |
| --- | --- | --- |
| `derivatives:: -- --nocapture` | 14 passed, 1 ignored; 0.69s | Hypercube volume FD, rational simplex exact/f64 agreement, conversion/wrapper boundaries, quotient rule, malformed-input guards, min-slope operation |
| `derivatives::tests::capacity_derivatives_a_on_hypercube -- --ignored --exact` | 1 passed; 0.14s | Analytical fixed-word action derivatives versus central differences on hypercube |
| `tests_capacity_derivative -- --include-ignored` | 7 passed; 11.08s | Height monotonicity/finite values, volume and capacity Euler diagnostics, zero-degree ratio check on listed fixtures |

The fixed-word FD test skips inactive/near-zero analytic components and directions
where either perturbed solve is infeasible; it does not certify every component
of an unrestricted derivative. Exact/f64 agreement uses one rational simplex.
The HK2017 finite-difference tests recompute the minimum-action envelope; at ties
they are symmetric difference diagnostics, not generally one-sided derivatives.
Their Euler checks are fixture checks, not a theorem of differentiability at
all named fixtures. No new tests were added to mirror implementation.

## Small source corrections in this branch

Only comments changed in `crates/symplectic/src/derivatives.rs` and
`crates/symplectic/src/algorithms/hk2017/tests_capacity_derivative.rs`:

- correct the volume-gradient header;
- describe supplied branch gradients without asserting complete subdifferentials;
- document that historical `clarke_directional_derivative_a` computes a minimum
  supplied slope, not the Clarke generalized directional derivative;
- correct the central-difference interpretation at ties.

The helper's name and behavior are unchanged because it has existing experiment
callers. The recovered chapter already uses the correct one-sided envelope
terminology. For `f(x)=-|x|`, minimum active slope at zero in direction 1 is -1,
whereas the Clarke generalized directional derivative is +1: names are not
interchangeable. This mismatch was already recorded in
`experiments/dev-gradient-ascent/endpoint-model-audit/REVIEW.md`; the present
patch removes the misleading public documentation without undertaking migration.

## Empirical ownership and coherent integration

| Evidence | Owner | Retained meaning |
| --- | --- | --- |
| Twelve F10 starts, eight accepted iterations each; every endpoint scan improves | `experiments/sys-landscape/gradient-ascent-observed-general/{README.md,artifacts/summary.json}` | Schema-v1 legacy evaluator illustration; not evidence for current schema-v2 exact-window implementation or endpoint maximality |
| Seven policies × 64 matched starts = 448 runs | `experiments/dev-gradient-ascent/optimizer-comparison/README.md`, `artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/final-summary.csv` | Historical heuristic target; four-anchor history median 0.984783. Same nominal 1000ms start-new-work cutoff and 128-call cap, not equal realized runtime. Manifest holdout label does not establish complete tuning separation |
| Three selected proposal decompositions, 39 geometry states | `experiments/dev-gradient-ascent/endpoint-model-audit/README.md`, `artifacts/directional-decomposition-20260729/analysis/REPORT.md` | Two practical-step failures and one success control. Small-step convergence; finite-distance KKT nonlinearities, not established globally reliable predictor |
| Reader-facing fuller optimizer account | `thesis/08-black-box-datascience-finite-budget-optimization.tex` | Already explains historical objective and comparison. Integration must select and reference it coherently; this check does not select chapter order |

The three-case audit reports KKT perturbation/base-eigenvalue ratios 218 and 67
at radius 1e-5 for the failures, with eigenvalue sign crossings. At 1e-8,
named-action relative FD errors fall to 9.92e-3 and 7.83e-5; success control
4.16e-7. It attributes the selected failures to finite-distance linearization,
not volume or missing target words. These are retained source-reported results,
not newly recomputed here. They support explaining why fixed Euclidean step
sizes can fail; they do not establish a universal trust-radius rule.

## Completion boundary

Assignment V's formula and evidence interface is checked. Remaining work is
integration of the bounded study descriptions and ordinary prose review. No
new generic/nongeneric theorem, expensive optimizer run or capacity recertification
is required by any finding here. Existing DS provenance/revalidation ownership
is unchanged. This active handoff worktree is retained for review/cherry-pick;
there are no task-created disposable scratch artifacts to preserve.
